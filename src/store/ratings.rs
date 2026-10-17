//! The ratings a person gave titles (like, love, dislike), kept in this browser and sent to the engine, with the watched and saved titles,
//! so the rows made from taste (More Like, We Think You'll Love These, Familiar Favourites) can be built.
use crate::components::media::movie_card_rating::MovieRating;
use std::collections::BTreeMap;

fn key(id: u32, is_tv: bool) -> String {
    format!("{}:{}", if is_tv { "tv" } else { "movie" }, id)
}

fn word(r: MovieRating) -> &'static str {
    match r {
        MovieRating::Like => "like",
        MovieRating::Love => "love",
        MovieRating::Dislike => "dislike",
    }
}

const RATINGS_KEY: &str = "streams_title_ratings_v1";
const STAMPS_KEY: &str = "streams_title_ratings_ts_v1";

fn read_key(base: &str) -> BTreeMap<String, String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(&crate::store::scoped::key(base)).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn write_key(base: &str, map: &BTreeMap<String, String>) {
    if let (Some(storage), Ok(raw)) = (web_sys::window().and_then(|w| w.local_storage().ok().flatten()), serde_json::to_string(map)) {
        let _ = storage.set_item(&crate::store::scoped::key(base), &raw);
    }
}

fn read() -> BTreeMap<String, String> {
    read_key(RATINGS_KEY)
}

/// When a title's rating (or its removal) last changed, in milliseconds; 0 when never.
pub fn stamp(key: &str) -> u64 {
    read_key(STAMPS_KEY).get(key).and_then(|t| t.parse().ok()).unwrap_or(0)
}

/// Every rating of the profile in use with the time it was given: (title key, (word, time)).
pub fn all() -> BTreeMap<String, (String, u64)> {
    let stamps = read_key(STAMPS_KEY);
    read().into_iter().map(|(k, v)| { let t = stamps.get(&k).and_then(|t| t.parse().ok()).unwrap_or(0); (k, (v, t)) }).collect()
}

fn store(id: u32, is_tv: bool, rating: Option<MovieRating>, at: u64) {
    let k = key(id, is_tv);
    let mut map = read();
    match rating {
        Some(r) => map.insert(k.clone(), word(r).to_string()),
        None => map.remove(&k),
    };
    write_key(RATINGS_KEY, &map);
    let mut stamps = read_key(STAMPS_KEY);
    stamps.insert(k, at.to_string());
    write_key(STAMPS_KEY, &stamps);
}

/// A rating that came from the account: kept here, not sent back.
pub fn apply_remote(id: u32, is_tv: bool, rating: Option<MovieRating>, at: u64) {
    store(id, is_tv, rating, at);
}

pub fn get(id: u32, is_tv: bool) -> Option<MovieRating> {
    match read().get(&key(id, is_tv)).map(String::as_str) {
        Some("like") => Some(MovieRating::Like),
        Some("love") => Some(MovieRating::Love),
        Some("dislike") => Some(MovieRating::Dislike),
        _ => None,
    }
}

pub fn set(id: u32, is_tv: bool, rating: Option<MovieRating>) {
    store(id, is_tv, rating, js_sys::Date::now() as u64);
    crate::services::sync::schedule_push();
}

/// Keys (`movie:550`) of the titles liked or loved, at most `limit`.
pub fn liked_keys(limit: usize) -> Vec<String> {
    read().into_iter().filter(|(_, v)| v == "like" || v == "love").map(|(k, _)| k).take(limit).collect()
}
