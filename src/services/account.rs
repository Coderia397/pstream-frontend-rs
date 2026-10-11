//! Sign up, sign in, password reset and sign out against Supabase Auth over its HTTP interface. The session is kept in the
//! browser (local storage when the person asks to stay signed in, session storage otherwise) and is counted as essential in the
//! cookie manager. Profiles still live in this browser; the database side is in pstream-backend-rs/supabase.
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    /// Unix seconds when the access token stops working.
    pub expires_at: u64,
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthError {
    /// Wrong email or password. The same answer is given for an unknown email.
    Invalid,
    /// The address has to be confirmed first.
    Unconfirmed,
    /// The password does not meet the rules; the text says why.
    Weak(String),
    /// Signing up failed in a way that must not reveal whether the address is already registered.
    CouldNotCreate,
    RateLimited,
    Network,
    Other(String),
}

/// What signing up leads to.
#[derive(Clone, Debug, PartialEq)]
pub enum SignUp {
    /// The account is ready and signed in.
    SignedIn(Session),
    /// A confirmation email was sent.
    ConfirmEmail,
}

fn url(path: &str) -> String {
    format!("{}{path}", crate::config::SUPABASE_URL)
}

fn storage(persistent: bool) -> Option<web_sys::Storage> {
    let w = web_sys::window()?;
    if persistent { w.local_storage().ok().flatten() } else { w.session_storage().ok().flatten() }
}

fn now_secs() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

fn session_from(v: &Value) -> Option<Session> {
    let meta = &v["user"]["user_metadata"];
    let text = |x: &Value| x.as_str().unwrap_or_default().to_string();
    Some(Session {
        access_token: v["access_token"].as_str()?.to_string(),
        refresh_token: v["refresh_token"].as_str().unwrap_or_default().to_string(),
        expires_at: v["expires_at"].as_u64().unwrap_or_else(|| now_secs() + v["expires_in"].as_u64().unwrap_or(3600)),
        user_id: v["user"]["id"].as_str()?.to_string(),
        email: v["user"]["email"].as_str().unwrap_or_default().to_string(),
        first_name: text(&meta["first_name"]),
        last_name: text(&meta["last_name"]),
    })
}

/// Maps a Supabase error body to a case the pages can speak about.
pub fn classify(status: u16, body: &Value) -> AuthError {
    let code = body["error_code"].as_str().or_else(|| body["code"].as_str()).unwrap_or_default();
    let msg = body["msg"].as_str().or_else(|| body["message"].as_str()).or_else(|| body["error_description"].as_str()).unwrap_or_default();
    match code {
        "invalid_credentials" | "invalid_grant" => AuthError::Invalid,
        "email_not_confirmed" => AuthError::Unconfirmed,
        "weak_password" => AuthError::Weak(msg.to_string()),
        "user_already_exists" | "email_exists" | "signup_disabled" | "validation_failed" | "email_address_invalid" => AuthError::CouldNotCreate,
        "over_request_rate_limit" | "over_email_send_rate_limit" => AuthError::RateLimited,
        _ if status == 429 => AuthError::RateLimited,
        _ if status == 400 && msg.to_lowercase().contains("invalid login") => AuthError::Invalid,
        _ => AuthError::Other(msg.to_string()),
    }
}

async fn post(path: &str, body: &Value, token: Option<&str>) -> Result<(u16, Value), AuthError> {
    let mut req = Request::post(&url(path)).header("apikey", crate::config::SUPABASE_KEY).header("Content-Type", "application/json");
    if let Some(t) = token {
        req = req.header("Authorization", &format!("Bearer {t}"));
    }
    let resp = req.body(body.to_string()).map_err(|_| AuthError::Network)?.send().await.map_err(|_| AuthError::Network)?;
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    Ok((status, serde_json::from_str(&text).unwrap_or(Value::Null)))
}

fn save(session: &Session, remember: bool) {
    // the other storage must not keep an older copy
    if let Some(other) = storage(!remember) {
        let _ = other.remove_item(crate::keys::SESSION);
    }
    if let (Some(s), Ok(raw)) = (storage(remember), serde_json::to_string(session)) {
        let _ = s.set_item(crate::keys::SESSION, &raw);
    }
}

/// The saved session, if there is one that has not expired.
pub fn session() -> Option<Session> {
    [true, false]
        .into_iter()
        .filter_map(|p| storage(p)?.get_item(crate::keys::SESSION).ok().flatten())
        .filter_map(|raw| serde_json::from_str::<Session>(&raw).ok())
        .find(|s| s.expires_at > now_secs())
}

pub async fn sign_in(email: &str, password: &str, remember: bool) -> Result<Session, AuthError> {
    let (status, body) = post("/auth/v1/token?grant_type=password", &json!({ "email": email.trim(), "password": password }), None).await?;
    if status == 200 {
        let session = session_from(&body).ok_or_else(|| AuthError::Other("unexpected answer".into()))?;
        save(&session, remember);
        return Ok(session);
    }
    Err(classify(status, &body))
}

pub async fn sign_up(email: &str, password: &str, first: &str, last: &str) -> Result<SignUp, AuthError> {
    let data = json!({ "first_name": first.trim(), "last_name": last.trim(), "full_name": format!("{} {}", first.trim(), last.trim()).trim() });
    let (status, body) = post("/auth/v1/signup", &json!({ "email": email.trim(), "password": password, "data": data }), None).await?;
    if status == 200 {
        return Ok(match session_from(&body) {
            Some(session) => {
                save(&session, true);
                SignUp::SignedIn(session)
            }
            None => SignUp::ConfirmEmail,
        });
    }
    Err(classify(status, &body))
}

/// Asks for a reset link. The answer is the same whether or not the address has an account.
pub async fn recover(email: &str) -> Result<(), AuthError> {
    let (status, body) = post("/auth/v1/recover", &json!({ "email": email.trim() }), None).await?;
    if status == 200 || status == 400 && classify(status, &body) == AuthError::Invalid { Ok(()) } else { Err(classify(status, &body)) }
}

/// Resends the confirmation email for an address that signed up.
pub async fn resend_confirmation(email: &str) -> Result<(), AuthError> {
    let (status, body) = post("/auth/v1/resend", &json!({ "type": "signup", "email": email.trim() }), None).await?;
    if status == 200 { Ok(()) } else { Err(classify(status, &body)) }
}

pub fn sign_out() {
    if let Some(s) = session() {
        leptos::task::spawn_local(async move {
            let _ = post("/auth/v1/logout", &json!({}), Some(&s.access_token)).await;
        });
    }
    for persistent in [true, false] {
        if let Some(st) = storage(persistent) {
            let _ = st.remove_item(crate::keys::SESSION);
        }
    }
}

/// True for someone who has been here before: signed in, or a profile was chosen on this browser. They go straight to the app;
/// everyone else sees the landing page at the root address.
pub fn is_returning() -> bool {
    session().is_some() || web_sys::window().and_then(|w| w.local_storage().ok().flatten()).and_then(|s| s.get_item(crate::keys::ACTIVE_PROFILE).ok().flatten()).is_some()
}

pub fn valid_email(email: &str) -> bool {
    let e = email.trim();
    match e.split_once('@') {
        Some((local, domain)) => !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.') && !e.contains(' '),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_shape() {
        assert!(valid_email("a@b.co"));
        assert!(valid_email("  first.last@mail.example.org "));
        assert!(!valid_email("a@b"));
        assert!(!valid_email("@b.co"));
        assert!(!valid_email("a b@c.de"));
        assert!(!valid_email("plain"));
    }

    #[test]
    fn supabase_errors_map_to_cases() {
        assert_eq!(classify(400, &json!({"error_code": "invalid_credentials", "msg": "Invalid login credentials"})), AuthError::Invalid);
        assert_eq!(classify(400, &json!({"error_code": "email_not_confirmed"})), AuthError::Unconfirmed);
        assert_eq!(classify(422, &json!({"error_code": "user_already_exists"})), AuthError::CouldNotCreate);
        assert_eq!(classify(422, &json!({"error_code": "weak_password", "msg": "Password should be at least 8 characters"})), AuthError::Weak("Password should be at least 8 characters".into()));
        assert_eq!(classify(429, &Value::Null), AuthError::RateLimited);
    }

    #[test]
    fn a_session_is_read_from_the_token_answer() {
        let body = json!({"access_token": "a", "refresh_token": "r", "expires_at": 4102444800u64, "user": {"id": "u1", "email": "x@y.zz", "user_metadata": {"first_name": "Ada", "last_name": "Lovelace"}}});
        let s = session_from(&body).unwrap();
        assert_eq!((s.first_name.as_str(), s.last_name.as_str(), s.user_id.as_str()), ("Ada", "Lovelace", "u1"));
    }
}
