use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use crate::services::tmdb::MediaItem;

/// Base URL of the recommendation engine, from crate::config.
const AI_API_BASE_URL: &str = crate::config::ENGINE_URL;

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
    #[serde(default)]
    pub rating_gb: Option<String>,
    #[serde(default)]
    pub rating_us: Option<String>,
    /// Full release date (YYYY-MM-DD) when the engine knows it; New & Popular rows carry it
    #[serde(default)]
    pub release_date: Option<String>,
    /// Up to three words for how the title feels (then its genres), shown as the tags of the hover card
    #[serde(default)]
    pub tags: Vec<String>,
    /// Labels the engine puts on a title: `top10` with a rank in a chart row, `new` for a recent release
    #[serde(default)]
    pub badges: Vec<Badge>,
    /// Minutes, 0 when unknown
    #[serde(default)]
    pub runtime: Option<u32>,
    /// The lines of the hero banner (hero feed only)
    #[serde(default)]
    pub hooks: Vec<crate::services::tmdb::Hook>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Badge {
    pub kind: String,
    #[serde(default)]
    pub rank: Option<u32>,
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
            release_date: self.release_date.clone().or_else(|| self.release_year.clone()),
            first_air_date: self.release_date.clone().or_else(|| self.release_year.clone()),
            media_type: self.media_type.clone(),
            match_percentage: self.match_percentage,
            vibe_pills: if self.vibe_pills.is_empty() { self.tags.clone() } else { self.vibe_pills.clone() },
            badges: self.badges.iter().map(|b| b.kind.clone()).collect(),
            hooks: self.hooks.clone(),
            genre_ids: Vec::new(),
            rating_gb: self.rating_gb.clone(),
            rating_us: self.rating_us.clone(),
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
    /// What the engine built the row from: personal, new, collection, vibe, mood, calendar, attribute, related
    #[serde(default, rename = "type")]
    pub kind: String,
    /// Translation key of the row title (`row.because_you_watched`) and its arguments; the plain `title` is the English fallback
    #[serde(default)]
    pub title_key: Option<String>,
    #[serde(default)]
    pub title_args: Option<serde_json::Value>,
    /// Why the row is shown (`{ kind, title, id, media_type }`)
    #[serde(default)]
    pub reason: Option<serde_json::Value>,
    /// Banner data of a collection row
    #[serde(default)]
    pub collection: Option<Collection>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Collection {
    pub id: String,
    pub label_key: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub target: serde_json::Value,
    /// Backdrop paths of the lead titles, for the background of the banner
    #[serde(default)]
    pub images: Vec<String>,
}

/// A chip for the top of the home page: an event that is in season.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Chip {
    pub id: String,
    pub label_key: String,
    pub label: String,
    #[serde(default)]
    pub target: serde_json::Value,
}

/// The interface text for a key the engine sends (`row.my_list` is `row-my-list` in the language files), None when no language has it.
pub fn engine_text(key: &str, args: Option<&serde_json::Value>) -> Option<String> {
    let id = key.replace(['.', '_'], "-");
    if !crate::i18n::has(&id) {
        return None;
    }
    let pairs: Vec<(String, fluent_bundle::FluentValue<'static>)> = args
        .and_then(|a| a.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| match (v.as_str(), v.as_i64()) {
                    (Some(s), _) => Some((k.clone(), fluent_bundle::FluentValue::from(s.to_string()))),
                    (_, Some(n)) => Some((k.clone(), fluent_bundle::FluentValue::from(n))),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default();
    let refs: Vec<(&str, fluent_bundle::FluentValue)> = pairs.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
    Some(crate::i18n::t_args(&id, &refs))
}

fn default_row_type() -> String {
    "vibe".to_string()
}

impl FeedRow {
    pub fn is_top_ten(&self) -> bool {
        self.row_type == "top_ten"
    }

    /// The title in the language of the page: the translation of the engine's key when there is one, the engine's own words otherwise.
    pub fn display_title(&self) -> String {
        self.title_key.as_deref().and_then(|k| engine_text(k, self.title_args.as_ref())).unwrap_or_else(|| self.title.clone())
    }

    pub fn to_media_items(&self) -> Vec<MediaItem> {
        self.items.iter().map(|item| item.to_media_item()).collect()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FeedResponse {
    #[serde(default)]
    pub surface: String,
    pub genre: Option<String>,
    #[serde(default)]
    pub total_rows: usize,
    #[serde(default)]
    pub rows: Vec<FeedRow>,
    /// Events in season (home page only)
    #[serde(default)]
    pub chips: Vec<Chip>,
}

thread_local! {
    /// Session number used while personalisation is off: it lives for this page load only and is never stored.
    static VOLATILE_SESSION: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// Where the recommendation engine lives. Used by the help center to check that it answers.
pub fn engine_base_url() -> &'static str {
    AI_API_BASE_URL
}

fn get_or_create_session_id() -> String {
    if !crate::consent::personalization_allowed() {
        return VOLATILE_SESSION.with(|v| v.borrow_mut().get_or_insert_with(|| format!("sess_tmp_{}_{}", (js_sys::Math::random() * 1_000_000.0) as u32, js_sys::Date::now() as u64)).clone());
    }
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(sid)) = storage.get_item(crate::keys::SESSION_ID) {
            if !sid.is_empty() {
                return sid;
            }
        }
        let new_sid = format!("sess_{}_{}", (js_sys::Math::random() * 1_000_000.0) as u32, js_sys::Date::now() as u64);
        let _ = storage.set_item(crate::keys::SESSION_ID, &new_sid);
        return new_sid;
    }
    format!("sess_anon_{}", js_sys::Date::now() as u64)
}

fn get_user_taste_affinity() -> Option<String> {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten())?;
    storage.get_item(crate::keys::USER_AFFINITY).ok().flatten()
}

/// `&rating=PG` for the profile in use, so the engine only returns titles that profile may see (empty when there is no profile).
fn rating_query() -> String {
    crate::pages::settings::prefs::active_rating().map(|r| format!("&rating={r}")).unwrap_or_default()
}

thread_local! {
    /// (route, age limit, surface) -> names of pages the profile cannot fill
    static HIDDEN_PAGES: std::cell::RefCell<std::collections::HashMap<(String, String, String), std::collections::HashSet<String>>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Asks the engine which of `names` a profile has too few titles for (`route` is `genre-support` or `language-support`, `param` the
/// query field that carries the names, `field` the key of the answer). An engine that cannot be reached hides nothing. Answers are kept per limit, route and surface.
async fn hidden_pages(route: &str, param: &str, field: &str, surface: &str, names: &[String]) -> std::collections::HashSet<String> {
    // a profile without a limit is asked as an adult: a page with too little of its own is not offered to anyone
    let limit = crate::pages::settings::prefs::active_rating().unwrap_or("18");
    let key = (route.to_string(), limit.to_string(), surface.to_string());
    if let Some(hit) = HIDDEN_PAGES.with(|c| c.borrow().get(&key).cloned()) {
        return hit;
    }
    let joined: String = js_sys::encode_uri_component(&names.join(",")).into();
    let url = format!("{}/api/{}?surface={}&{}={}&rating={}", AI_API_BASE_URL, route, surface, param, joined, limit);
    let Some(v) = (match Request::get(&url).send().await {
        Ok(r) if r.ok() => r.json::<serde_json::Value>().await.ok(),
        _ => None,
    }) else { return Default::default() };
    let hidden: std::collections::HashSet<String> = v
        .get(field)
        .and_then(|g| g.as_object())
        .map(|m| m.iter().filter(|(_, e)| e.get("supported").and_then(|s| s.as_bool()) == Some(false)).map(|(n, _)| n.clone()).collect())
        .unwrap_or_default();
    HIDDEN_PAGES.with(|c| c.borrow_mut().insert(key, hidden.clone()));
    hidden
}

/// The genre pages a limited profile should not be offered (see `hidden_pages`).
pub async fn hidden_genres(surface: &str, names: &[String]) -> std::collections::HashSet<String> {
    hidden_pages("genre-support", "names", "genres", surface, names).await
}

/// The language pages a limited profile should not be offered.
pub async fn hidden_languages(codes: &[String]) -> std::collections::HashSet<String> {
    hidden_pages("language-support", "codes", "languages", "both", codes).await
}

/// Search from the engine: titles by words (the last one may be unfinished), by director or cast, and near misses for typos, with the real
/// certificates and the profile's age limit applied. None when the engine cannot be reached.
pub async fn search_titles(query: &str) -> Option<Vec<MediaItem>> {
    let encoded: String = js_sys::encode_uri_component(query.trim()).into();
    let url = format!("{}/api/search?q={}&limit=40{}", AI_API_BASE_URL, encoded, rating_query());
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let v: serde_json::Value = resp.json().await.ok()?;
    let text = |r: &serde_json::Value, k: &str| r.get(k).and_then(|x| x.as_str()).map(str::to_string);
    let items = v
        .get("results")?
        .as_array()?
        .iter()
        .filter_map(|r| {
            let media_type = text(r, "media_type")?;
            let year = r.get("release_year").and_then(|y| y.as_i64()).map(|y| y.to_string());
            let title = text(r, "title");
            Some(MediaItem {
                id: r.get("id")?.as_u64()? as u32,
                title: if media_type == "tv" { None } else { title.clone() },
                name: if media_type == "tv" { title } else { None },
                overview: text(r, "overview").unwrap_or_default(),
                poster_path: text(r, "poster_path"),
                backdrop_path: text(r, "backdrop_path"),
                vote_average: r.get("vote_average").and_then(|x| x.as_f64()).unwrap_or(0.0),
                release_date: year.clone(),
                first_air_date: year,
                media_type: Some(media_type),
                rating_gb: text(r, "rating_gb"),
                rating_us: text(r, "rating_us"),
                ..Default::default()
            })
        })
        .filter(|m| m.poster_path.is_some() || m.backdrop_path.is_some())
        .collect();
    Some(items)
}

/// The New & Popular page from the engine: the desktop rows, or with a `tab` (watching, top10_movies, top10_series, just_landed,
/// coming_soon) the single row of that mobile tab. None when the engine cannot be reached or has nothing.
pub async fn fetch_new_popular(tab: Option<&str>) -> Option<Vec<FeedRow>> {
    fetch_new_popular_n(tab, None).await
}

/// As `fetch_new_popular`, asking for up to `limit` titles per row (the engine allows 30).
pub async fn fetch_new_popular_n(tab: Option<&str>, limit: Option<usize>) -> Option<Vec<FeedRow>> {
    let mut url = format!("{}/api/new-popular?session_id={}{}", AI_API_BASE_URL, get_or_create_session_id(), rating_query());
    if let Some(t) = tab {
        url.push_str(&format!("&tab={t}"));
    }
    if let Some(n) = limit {
        url.push_str(&format!("&limit={n}"));
    }
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: FeedResponse = resp.json().await.ok()?;
    (!data.rows.is_empty()).then_some(data.rows)
}

/// The titles of a language page (original language) as rows from the engine, the Top 10 rows left out so the page reads as one ranked grid.
pub async fn fetch_language_titles(lang: &str) -> Option<Vec<MediaItem>> {
    let url = format!("{}/api/feed?language={}&limit=12&session_id={}{}", AI_API_BASE_URL, lang, get_or_create_session_id(), rating_query());
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data: FeedResponse = resp.json().await.ok()?;
    let mut seen = std::collections::HashSet::new();
    let items: Vec<MediaItem> = data
        .rows
        .iter()
        .filter(|r| !r.is_top_ten())
        .flat_map(|r| r.to_media_items())
        .filter(|m| seen.insert((m.media_type.clone(), m.id)))
        .collect();
    (!items.is_empty()).then_some(items)
}

/// The profile's own titles for the personal rows, as query text: `&recent=` (what was watched, newest first), `&saved=` (My List) and the hour
/// of the day for the late night rows. Empty when personalisation is off. Call it where the page is built (it reads the stores of the page).
pub fn personal_signals() -> String {
    use leptos::prelude::WithUntracked;
    let hour = format!("&hour={}", js_sys::Date::new_0().get_hours());
    if !crate::consent::personalization_allowed() {
        return hour;
    }
    let key = |id: u32, is_tv: bool| format!("{}:{}", if is_tv { "tv" } else { "movie" }, id);
    let mut out = hour;
    if let Some(watch) = leptos::prelude::use_context::<crate::store::WatchStore>() {
        let recent: Vec<String> = watch.records.with_untracked(|m| {
            let mut list: Vec<&crate::store::WatchRecord> = m.values().collect();
            list.sort_by_key(|r| std::cmp::Reverse(r.updated_at));
            list.into_iter().take(12).map(|r| key(r.media_id, r.is_tv)).collect()
        });
        if !recent.is_empty() {
            out.push_str(&format!("&recent={}", recent.join(",")));
        }
    }
    let liked = crate::store::ratings::liked_keys(12);
    if !liked.is_empty() {
        out.push_str(&format!("&liked={}", liked.join(",")));
    }
    if let Some(lib) = leptos::prelude::use_context::<crate::store::LibraryStore>() {
        let saved: Vec<String> = lib.my_list.with_untracked(|m| {
            let mut list: Vec<&crate::store::LibraryEntry> = m.values().collect();
            list.sort_by_key(|e| std::cmp::Reverse(e.added_at));
            list.into_iter().take(12).map(|e| key(e.media.id, e.media.media_type.as_deref() == Some("tv"))).collect()
        });
        if !saved.is_empty() {
            out.push_str(&format!("&saved={}", saved.join(",")));
        }
    }
    out
}

/// Fetches dynamic feed rows composed by the backend intelligence server
pub async fn fetch_dynamic_feed(surface: &str, genre: Option<&str>, limit: Option<usize>) -> Option<Vec<FeedRow>> {
    fetch_feed_page(surface, genre, limit, "").await.map(|d| d.rows)
}

/// As `fetch_dynamic_feed`, with the answer as a whole (rows and chips) and the profile's own titles (`signals`, from `personal_signals`).
pub async fn fetch_feed_page(surface: &str, genre: Option<&str>, limit: Option<usize>, signals: &str) -> Option<FeedResponse> {
    let lim = limit.unwrap_or(18);
    let session_id = get_or_create_session_id();
    let mut url = format!("{}/api/feed?surface={}&limit={}&session_id={}{}{}", AI_API_BASE_URL, surface, lim, session_id, rating_query(), signals);
    
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
        Some(data)
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
pub async fn fetch_hero_feed(surface: &str, genre: Option<&str>) -> Option<Vec<MediaItem>> {
    let session_id = get_or_create_session_id();
    let mut url = format!("{}/api/hero-feed?surface={}&session_id={}{}&hour={}", AI_API_BASE_URL, surface, session_id, rating_query(), js_sys::Date::new_0().get_hours());
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
pub async fn fetch_ai_recommendations(id: u32, is_tv: bool) -> Option<Vec<VibeItem>> {
    // TMDB ids repeat across movies and series, so the media type is part of the identity
    let url = format!("{}/api/recommendations?id={}&media_type={}{}", AI_API_BASE_URL, id, if is_tv { "tv" } else { "movie" }, rating_query());
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

#[derive(Serialize)]
struct FeedbackBody<'a> {
    session_id: &'a str,
    item_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<&'a str>,
    event: &'a str,
}

/// Reports an implicit signal (click, start, complete, like, skip) so the engine
/// can adapt the session's feed and hero. Fire and forget: failures are ignored.
pub async fn send_feedback(item_id: u32, media_type: Option<&str>, event: &str) {
    if !crate::consent::personalization_allowed() {
        return;
    }
    let session_id = get_or_create_session_id();
    let body = FeedbackBody { session_id: &session_id, item_id, media_type, event };
    let url = format!("{}/api/feedback", AI_API_BASE_URL);
    if let Ok(req) = Request::post(&url).json(&body) {
        let _ = req.send().await;
    }
}
