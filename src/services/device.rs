//! What this browser says about itself when it signs in, so the account can tell its devices apart. A device is recognised by an
//! id the browser creates and keeps for itself (not by where it connects from), plus the browser, system, kind of device,
//! language, time zone and screen size it reports. The country comes from our own edge (/cdn-cgi/trace on the site's address,
//! two letters, no connection address is read or kept). A VPN or travel changes the country only; the id stays, so it is still
//! the same device. Everything is sent to `register_device` (see streams-backend-rs/supabase/migrations/20261012200000_*).
use crate::data::settings::{BROWSERS, SYSTEMS};
use crate::services::account::{self, AuthError};
use gloo_net::http::Request;
use js_sys::Reflect;
use serde_json::json;
use wasm_bindgen::{JsCast, JsValue};

/// The device id: 32 hex digits made once per browser and kept in local storage. It is not a secret and not a login.
pub fn device_key() -> String {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    if let Some(existing) = storage.as_ref().and_then(|s| s.get_item(crate::keys::DEVICE_KEY).ok().flatten()).filter(|k| (16..=64).contains(&k.len())) {
        return existing;
    }
    let made = random_hex(32);
    if let Some(s) = storage {
        let _ = s.set_item(crate::keys::DEVICE_KEY, &made);
    }
    made
}

fn random_hex(len: usize) -> String {
    (0..len).map(|_| char::from_digit((js_sys::Math::random() * 16.0) as u32 % 16, 16).unwrap_or('0')).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
    pub browser: Option<&'static str>,
    pub browser_major: Option<u32>,
    pub os: Option<&'static str>,
    /// "desktop", "mobile", "tablet" or "other"
    pub kind: &'static str,
}

/// Reads a user agent text with the tables in data::settings. First match wins in each table.
pub fn parse_agent(ua: &str) -> Agent {
    let browser = BROWSERS.iter().find(|(needle, _)| ua.contains(needle)).copied();
    let browser_major = browser.and_then(|(needle, name)| {
        // Safari writes its version after "Version/", the others after their own name
        let marker = if name == "Safari" && ua.contains("Version/") { "Version/" } else { needle };
        let at = ua.find(marker)? + marker.len();
        ua[at..].chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok()
    });
    let os = SYSTEMS.iter().find(|(needle, _)| ua.contains(needle)).map(|(_, name)| *name);
    let kind = if ua.contains("iPad") || (ua.contains("Android") && !ua.contains("Mobile")) {
        "tablet"
    } else if ua.contains("iPhone") || ua.contains("Mobi") {
        "mobile"
    } else if os.is_some() {
        "desktop"
    } else {
        "other"
    };
    Agent { browser: browser.map(|(_, name)| name), browser_major, os, kind }
}

/// "loc=GB" from the text /cdn-cgi/trace returns, if present and two capital letters.
pub fn parse_trace(text: &str) -> Option<String> {
    text.lines().find_map(|l| l.strip_prefix("loc=")).map(|c| c.trim().to_string()).filter(|c| c.len() == 2 && c.chars().all(|ch| ch.is_ascii_uppercase()))
}

/// The country this connection comes from, from our own edge. None in development or when the edge does not answer.
pub async fn country() -> Option<String> {
    let resp = Request::get("/cdn-cgi/trace").send().await.ok().filter(|r| r.ok())?;
    parse_trace(&resp.text().await.ok()?)
}

fn global(name: &str) -> Option<JsValue> {
    web_sys::window().and_then(|w| Reflect::get(&w, &JsValue::from_str(name)).ok()).filter(|v| !v.is_undefined() && !v.is_null())
}

fn time_zone() -> Option<String> {
    let intl = global("Intl")?;
    let ctor: js_sys::Function = Reflect::get(&intl, &"DateTimeFormat".into()).ok()?.dyn_into().ok()?;
    let format = Reflect::construct(&ctor, &js_sys::Array::new()).ok()?;
    let options = Reflect::get(&format, &"resolvedOptions".into()).ok()?.dyn_into::<js_sys::Function>().ok()?.call0(&format).ok()?;
    Reflect::get(&options, &"timeZone".into()).ok()?.as_string()
}

/// The name of a country in the reader's language ("GB" becomes "United Kingdom"), or the code when the browser cannot say.
pub fn region_name(code: &str) -> String {
    (|| {
        let intl = global("Intl")?;
        let ctor: js_sys::Function = Reflect::get(&intl, &"DisplayNames".into()).ok()?.dyn_into().ok()?;
        let options = js_sys::Object::new();
        Reflect::set(&options, &"type".into(), &"region".into()).ok()?;
        let args = js_sys::Array::of2(&js_sys::Array::of1(&crate::i18n::language().into()), &options);
        let names = Reflect::construct(&ctor, &args).ok()?;
        let of: js_sys::Function = Reflect::get(&names, &"of".into()).ok()?.dyn_into().ok()?;
        of.call1(&names, &code.into()).ok()?.as_string()
    })()
    .unwrap_or_else(|| code.to_string())
}

/// Everything sent to the database about this browser.
pub fn collect(country: Option<String>) -> serde_json::Value {
    let window = web_sys::window();
    let ua = window.as_ref().and_then(|w| w.navigator().user_agent().ok()).unwrap_or_default();
    let agent = parse_agent(&ua);
    let language = window.as_ref().and_then(|w| w.navigator().language());
    let screen = window.as_ref().and_then(|w| {
        let s = w.screen().ok()?;
        Some(format!("{}x{}", s.width().ok()?, s.height().ok()?))
    });
    json!({
        "browser": agent.browser,
        "browser_major": agent.browser_major,
        "os": agent.os,
        "device_type": agent.kind,
        "language": language,
        "timezone": time_zone(),
        "screen": screen,
        "user_agent": ua,
        "country": country,
    })
}

/// Tells the account about this browser. Called after every sign in and when the app starts signed in.
pub async fn register() -> Result<(), AuthError> {
    if account::session().is_none() {
        return Ok(());
    }
    let country = country().await;
    if let Some(c) = &country {
        store(crate::keys::COUNTRY, c);
    }
    let info = collect(country);
    let registered = account::rpc("register_device", &json!({ "p_device_key": device_key(), "p_info": info })).await.map(|_| ());
    refresh_rating_system().await;
    registered
}

fn store(key: &str, value: &str) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(key, value);
    }
}

fn stored(key: &str) -> Option<String> {
    web_sys::window()?.local_storage().ok().flatten()?.get_item(key).ok().flatten().filter(|v| !v.is_empty())
}

/// The country the connection came from at the last registration, if there was one.
pub fn cached_country() -> Option<String> {
    stored(crate::keys::COUNTRY)
}

/// The rating system the database resolved for this session ("uk" or "us"), if it has answered yet.
pub fn cached_rating_system() -> Option<String> {
    stored(crate::keys::RATING_SYSTEM_AUTO)
}

/// Asks the database which rating labels this session reads (the account choice, else the country of this session's device) and keeps the
/// answer. A failure changes nothing.
pub async fn refresh_rating_system() {
    if let Ok(answer) = account::rpc("my_rating_system", &json!({})).await {
        if let Some(system) = answer.as_str().filter(|s| matches!(*s, "uk" | "us")) {
            store(crate::keys::RATING_SYSTEM_AUTO, system);
        }
    }
}

/// `register` without waiting for it; a failure (not set up yet, offline) changes nothing for the person.
pub fn register_in_background() {
    leptos::task::spawn_local(async {
        let _ = register().await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAC_CHROME: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36";
    const IPHONE_SAFARI: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1";
    const ANDROID_TABLET: &str = "Mozilla/5.0 (Linux; Android 16; Pixel Tablet) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36";
    const EDGE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0.0.0 Safari/537.36 Edg/150.0.0.0";

    #[test]
    fn agents_are_read_into_browser_system_and_kind() {
        assert_eq!(parse_agent(MAC_CHROME), Agent { browser: Some("Chrome"), browser_major: Some(156), os: Some("Mac"), kind: "desktop" });
        assert_eq!(parse_agent(IPHONE_SAFARI), Agent { browser: Some("Safari"), browser_major: Some(18), os: Some("iPhone"), kind: "mobile" });
        assert_eq!(parse_agent(ANDROID_TABLET).kind, "tablet");
        assert_eq!(parse_agent(EDGE), Agent { browser: Some("Edge"), browser_major: Some(150), os: Some("Windows"), kind: "desktop" });
        assert_eq!(parse_agent("curl/8").kind, "other");
    }

    #[test]
    fn the_country_is_read_from_the_edge_trace() {
        assert_eq!(parse_trace("fl=1\nip=1.2.3.4\nloc=GB\ncolo=LHR\n"), Some("GB".into()));
        assert_eq!(parse_trace("loc=XX1\n"), None);
        assert_eq!(parse_trace("nothing here"), None);
    }
}
