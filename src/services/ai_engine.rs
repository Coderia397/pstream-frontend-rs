use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use crate::services::tmdb::MediaItem;

const AI_API_BASE_URL: &str = "http://127.0.0.1:8088";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct VibeItem {
    pub id: u32,
    pub media_type: Option<String>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub release_year: Option<String>,
    pub director: Option<String>,
    pub cast: Option<String>,
    pub tagline: Option<String>,
    pub genres: Option<String>,
    pub keywords: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f64>,
    pub match_percentage: Option<u32>,
    pub cosine_similarity: Option<f64>,
    #[serde(default)]
    pub vibe_pills: Vec<String>,
}

impl VibeItem {
    pub fn display_title(&self) -> &str {
        self.title
            .as_deref()
            .or(self.name.as_deref())
            .unwrap_or("Untitled")
    }

    pub fn is_tv(&self) -> bool {
        self.media_type.as_deref() == Some("tv")
    }

    pub fn to_media_item(&self) -> MediaItem {
        MediaItem {
            id: self.id,
            title: self.title.clone(),
            name: self.name.clone(),
            overview: self.overview.clone().unwrap_or_default(),
            poster_path: self.poster_path.clone(),
            backdrop_path: self.backdrop_path.clone(),
            vote_average: self.vote_average.unwrap_or(0.0),
            vote_count: None,
            adult: Some(false),
            release_date: self.release_year.clone(),
            first_air_date: self.release_year.clone(),
            media_type: self.media_type.clone(),
            match_percentage: self.match_percentage,
            vibe_pills: self.vibe_pills.clone(),
            genre_ids: Vec::new(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FeedRow {
    pub id: String,
    #[serde(default = "default_row_type")]
    pub row_type: String, // "vibe", "top_ten", "auteur"
    pub title: String,
    pub tagline: Option<String>,
    #[serde(default)]
    pub mood_pills: Vec<String>,
    #[serde(default)]
    pub items: Vec<VibeItem>,
}

fn default_row_type() -> String {
    "vibe".to_string()
}

impl FeedRow {
    pub fn is_top_ten(&self) -> bool {
        self.row_type == "top_ten"
    }

    pub fn to_media_items(&self) -> Vec<MediaItem> {
        self.items.iter().map(|item| item.to_media_item()).collect()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FeedResponse {
    pub surface: String,
    pub genre: Option<String>,
    #[serde(default)]
    pub total_rows: usize,
    #[serde(default)]
    pub rows: Vec<FeedRow>,
}

fn get_or_create_session_id() -> String {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(sid)) = storage.get_item("pstream_session_id") {
            if !sid.is_empty() {
                return sid;
            }
        }
        let new_sid = format!("sess_{}_{}", (js_sys::Math::random() * 1_000_000.0) as u32, js_sys::Date::now() as u64);
        let _ = storage.set_item("pstream_session_id", &new_sid);
        return new_sid;
    }
    format!("sess_anon_{}", js_sys::Date::now() as u64)
}

fn get_user_taste_affinity() -> Option<String> {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten())?;
    storage.get_item("pstream_user_affinity").ok().flatten()
}

/// Fetches dynamic feed rows composed by the backend intelligence server
pub async fn fetch_dynamic_feed(surface: &str, genre: Option<&str>, limit: Option<usize>) -> Option<Vec<FeedRow>> {
    let lim = limit.unwrap_or(18);
    let session_id = get_or_create_session_id();
    let mut url = format!("{}/api/feed?surface={}&limit={}&session_id={}", AI_API_BASE_URL, surface, lim, session_id);
    
    if let Some(aff) = get_user_taste_affinity() {
        if !aff.is_empty() {
            let encoded = js_sys::encode_uri_component(&aff);
            let encoded_str: String = encoded.into();
            url.push_str(&format!("&affinity={}", encoded_str));
        }
    }

    if let Some(g) = genre {
        if !g.is_empty() {
            let encoded = js_sys::encode_uri_component(g);
            let encoded_str: String = encoded.into();
            url.push_str(&format!("&genre={}", encoded_str));
        }
    }
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: FeedResponse = resp.json().await.ok()?;
    if data.rows.is_empty() {
        None
    } else {
        Some(data.rows)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct VibeRow {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub mood_pills: Vec<String>,
    #[serde(default)]
    pub items: Vec<VibeItem>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct VibeRowsResponse {
    pub rows: Vec<VibeRow>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct SemanticSearchResponse {
    pub query: String,
    #[serde(default)]
    pub latency_ms: f64,
    #[serde(default)]
    pub total_matches: usize,
    #[serde(default)]
    pub results: Vec<VibeItem>,
}

/// Fetches dynamic AI-curated vibe rows from local AI Recommendation API
pub async fn fetch_vibe_rows() -> Option<Vec<VibeRow>> {
    let url = format!("{}/api/vibe-rows", AI_API_BASE_URL);
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: VibeRowsResponse = resp.json().await.ok()?;
    Some(data.rows)
}

/// Performs semantic vector search on local catalog via Qwen3-Embedding-8B
pub async fn search_semantic(query: &str) -> Option<Vec<VibeItem>> {
    let encoded = js_sys::encode_uri_component(query);
    let url = format!("{}/api/search/semantic?q={}", AI_API_BASE_URL, encoded);
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: SemanticSearchResponse = resp.json().await.ok()?;
    Some(data.results)
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct HeroHighlight {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub primary_hook: Option<String>,
    pub primary_icon: Option<String>,
    pub secondary_hook: Option<String>,
    pub secondary_icon: Option<String>,
}

/// Fetches dynamic creator-aware hero hook and distinction badge from local recommendation engine
pub async fn fetch_hero_highlight(id: u32, title: Option<&str>) -> Option<HeroHighlight> {
    let mut url = format!("{}/api/hero-highlight?id={}", AI_API_BASE_URL, id);
    if let Some(t) = title {
        let encoded = js_sys::encode_uri_component(t);
        let encoded_str: String = encoded.into();
        url.push_str(&format!("&title={}", encoded_str));
    }
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    resp.json().await.ok()
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct HeroFeedResponse {
    pub surface: String,
    pub count: usize,
    #[serde(default)]
    pub items: Vec<VibeItem>,
}

/// Fetches dynamic surface-differentiated hero feed from local recommendation engine
pub async fn fetch_hero_feed(surface: &str) -> Option<Vec<MediaItem>> {
    let url = format!("{}/api/hero-feed?surface={}", AI_API_BASE_URL, surface);
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: HeroFeedResponse = resp.json().await.ok()?;
    if data.items.is_empty() {
        return None;
    }
    Some(data.items.into_iter().map(|item| item.to_media_item()).collect())
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RecommendationsResponse {
    pub target_id: u32,
    pub target_title: Option<String>,
    #[serde(default)]
    pub recommendations: Vec<VibeItem>,
}

/// Fetches latent vector cosine recommendations with Netflix match percentage and vibe tags
pub async fn fetch_ai_recommendations(id: u32) -> Option<Vec<VibeItem>> {
    let url = format!("{}/api/recommendations?id={}", AI_API_BASE_URL, id);
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: RecommendationsResponse = resp.json().await.ok()?;
    if data.recommendations.is_empty() {
        return None;
    }
    Some(data.recommendations)
}
