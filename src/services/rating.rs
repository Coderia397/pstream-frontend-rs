//! The age rating of a title, from the engine (`/api/rating`), which holds the real British or US certificate. A title with no certificate is
//! "NR" (not rated). Nothing here guesses a rating from genres or scores. Answers are kept for the session.
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static CACHE: RefCell<HashMap<(u32, bool), String>> = RefCell::new(HashMap::new());
}

/// "U", "PG", "12", "15", "18" or "NR". An unreachable engine also gives "NR" (and is not remembered).
pub async fn fetch(id: u32, is_tv: bool) -> String {
    if let Some(hit) = CACHE.with(|c| c.borrow().get(&(id, is_tv)).cloned()) {
        return hit;
    }
    let url = format!("{}/api/rating?id={}&media_type={}", crate::config::ENGINE_URL, id, if is_tv { "tv" } else { "movie" });
    let label = match gloo_net::http::Request::get(&url).send().await {
        Ok(r) if r.ok() => r.json::<serde_json::Value>().await.ok().and_then(|v| v.get("rating").and_then(|x| x.as_str()).map(str::to_string)),
        _ => None,
    };
    match label {
        Some(l) => {
            CACHE.with(|c| c.borrow_mut().insert((id, is_tv), l.clone()));
            l
        }
        None => "NR".to_string(),
    }
}

thread_local! {
    /// (age limit, title key) -> may the profile see it
    static ALLOWED: RefCell<HashMap<(String, String), bool>> = RefCell::new(HashMap::new());
}

fn key_of(item: &crate::services::tmdb::MediaItem) -> String {
    format!("{}:{}", if item.is_tv() { "tv" } else { "movie" }, item.id)
}

/// Keeps the titles the profile in use may see. A profile limited to U, PG, 12 or 15 asks the engine (one call per list, answers kept);
/// with no limit (18) or no profile nothing is asked. If the engine cannot be reached a limited profile gets nothing rather than
/// unchecked titles.
pub async fn filter_allowed(items: Vec<crate::services::tmdb::MediaItem>) -> Vec<crate::services::tmdb::MediaItem> {
    let Some(limit) = crate::pages::settings::prefs::active_rating().filter(|l| *l != "18") else { return items };
    let limit = limit.to_string();
    let unknown: Vec<String> = ALLOWED.with(|c| {
        let c = c.borrow();
        items.iter().map(key_of).filter(|k| !c.contains_key(&(limit.clone(), k.clone()))).collect()
    });
    if !unknown.is_empty() {
        let url = format!("{}/api/ratings?rating={}&ids={}", crate::config::ENGINE_URL, limit, unknown.iter().take(100).cloned().collect::<Vec<_>>().join(","));
        let answer = match gloo_net::http::Request::get(&url).send().await {
            Ok(r) if r.ok() => r.json::<serde_json::Value>().await.ok(),
            _ => None,
        };
        let Some(answer) = answer else { return Vec::new() };
        let allowed: Vec<String> = answer.get("allowed").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
        ALLOWED.with(|c| {
            let mut c = c.borrow_mut();
            for k in unknown.iter().take(100) {
                c.insert((limit.clone(), k.clone()), allowed.contains(k));
            }
        });
    }
    ALLOWED.with(|c| {
        let c = c.borrow();
        items.into_iter().filter(|i| c.get(&(limit.clone(), key_of(i))).copied().unwrap_or(false)).collect()
    })
}
