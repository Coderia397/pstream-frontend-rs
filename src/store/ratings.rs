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

fn read() -> BTreeMap<String, String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(crate::keys::TITLE_RATINGS).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
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
    let mut map = read();
    match rating {
        Some(r) => map.insert(key(id, is_tv), word(r).to_string()),
        None => map.remove(&key(id, is_tv)),
    };
    if let (Some(storage), Ok(raw)) = (web_sys::window().and_then(|w| w.local_storage().ok().flatten()), serde_json::to_string(&map)) {
        let _ = storage.set_item(crate::keys::TITLE_RATINGS, &raw);
    }
}

/// Keys (`movie:550`) of the titles liked or loved, at most `limit`.
pub fn liked_keys(limit: usize) -> Vec<String> {
    read().into_iter().filter(|(_, v)| v == "like" || v == "love").map(|(k, _)| k).take(limit).collect()
}
