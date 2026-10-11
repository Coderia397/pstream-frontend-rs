//! Small lists kept in this browser: the last searches and the last articles opened.
use std::collections::VecDeque;

fn storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

fn read(key: &str) -> VecDeque<String> {
    storage().and_then(|s| s.get_item(key).ok().flatten()).and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or_default()
}

fn write(key: &str, list: &VecDeque<String>) {
    if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(list)) {
        let _ = s.set_item(key, &raw);
    }
}

/// Puts `item` first, drops an older copy and keeps at most `max` entries.
fn push(key: &str, item: &str, max: usize) {
    let mut list = read(key);
    list.retain(|x| x != item);
    list.push_front(item.to_string());
    list.truncate(max);
    write(key, &list);
}

pub fn searches() -> Vec<String> {
    read(crate::keys::HELP_RECENT).into_iter().collect()
}

pub fn remember_search(query: &str) {
    let q = query.trim();
    if q.chars().count() >= 2 {
        push(crate::keys::HELP_RECENT, q, 6);
    }
}

pub fn clear_searches() {
    if let Some(s) = storage() {
        let _ = s.remove_item(crate::keys::HELP_RECENT);
    }
}

pub fn viewed() -> Vec<String> {
    read(crate::keys::HELP_VIEWED).into_iter().collect()
}

pub fn remember_viewed(slug: &str) {
    push(crate::keys::HELP_VIEWED, slug, 5);
}
