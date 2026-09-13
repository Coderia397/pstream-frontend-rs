use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::models::mapping::{map_netflix_id_to_tmdb, MediaKind};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

// NOTE: Replace with your actual TMDB API key.
// This key lives in the client and is freely visible in browser devtools.
// TMDB keys are free and easily cycled — the tradeoff is zero server cost.
const TMDB_API_KEY: &str = "fc5fec3b73d8605daaeb1eb3b91157eb";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TmdbResponse<T> {
    pub page: u32,
    pub results: Vec<T>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MediaItem {
    pub id: u32,
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: Option<u32>,
    #[serde(default)]
    pub adult: Option<bool>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub media_type: Option<String>,
}

impl MediaItem {
    pub fn display_title(&self) -> &str {
        self.title
            .as_deref()
            .or(self.name.as_deref())
            .unwrap_or("Unknown Title")
    }

    pub fn is_nsfw(&self) -> bool {
        if self.adult.unwrap_or(false) {
            return true;
        }
        let t = self.display_title().to_lowercase();
        const BLOCKED: &[&str] = &[
            "overflow", "おーばーふろぉ", "じ〜くれっとみっしょん", "secret mission",
            "sweet agony", "hentai", "ecchi", "erotic", "porn", "xxx",
        ];
        for &term in BLOCKED {
            if t.contains(term) {
                return true;
            }
        }
        false
    }

    /// Full poster URL at a given size (e.g. "w500", "original")
    pub fn poster_url(&self, size: &str) -> Option<String> {
        self.poster_path
            .as_ref()
            .map(|p| format!("{}/{}{}", TMDB_IMAGE_BASE, size, p))
    }

    /// Full backdrop URL at a given size
    pub fn backdrop_url(&self, size: &str) -> Option<String> {
        self.backdrop_path
            .as_ref()
            .map(|p| format!("{}/{}{}", TMDB_IMAGE_BASE, size, p))
    }

    pub fn is_tv(&self) -> bool {
        self.media_type.as_deref() == Some("tv")
            || self.name.is_some() && self.title.is_none()
    }
}

/// Fetch trending items (used for the hero carousel and home page rows).
pub async fn fetch_trending(kind: &str) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url = format!(
        "{}/trending/{}/week?api_key={}&language=en-US",
        TMDB_BASE_URL, kind, TMDB_API_KEY
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    let items = resp.results.into_iter().filter(|i| !i.is_nsfw() && (i.backdrop_path.is_some() || i.poster_path.is_some())).collect();
    Ok(items)
}

/// Fetch top rated items
pub async fn fetch_top_rated(kind: &str) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url = format!(
        "{}/{}/top_rated?api_key={}&language=en-US",
        TMDB_BASE_URL, kind, TMDB_API_KEY
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    let items = resp.results.into_iter().filter(|i| !i.is_nsfw() && (i.backdrop_path.is_some() || i.poster_path.is_some())).collect();
    Ok(items)
}

/// Fetch items for a specific TMDB genre ID.
pub async fn fetch_by_genre(kind: &str, genre_id: u32) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url = format!(
        "{}/discover/{}?api_key={}&with_genres={}&language=en-US&sort_by=popularity.desc",
        TMDB_BASE_URL, kind, TMDB_API_KEY, genre_id
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    Ok(resp.results)
}

/// Fetch items for a British region filter (used by the 'British TV' category).
pub async fn fetch_british_tv() -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url = format!(
        "{}/discover/tv?api_key={}&with_origin_country=GB&language=en-US&sort_by=popularity.desc",
        TMDB_BASE_URL, TMDB_API_KEY
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    Ok(resp.results)
}

/// Primary entry point: given a Netflix-style genre ID from the URL,
/// resolve the correct TMDB query and return its results.
pub async fn fetch_for_genre_route(netflix_id: &str) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let kind = if let Some(ctx) = map_netflix_id_to_tmdb(netflix_id) {
        match ctx.kind {
            MediaKind::Tv => "tv",
            MediaKind::Movie => "movie",
        }
    } else if netflix_id == "british" || netflix_id == "binge" || netflix_id == "52117" || netflix_id == "1191605" {
        "tv"
    } else {
        "movie"
    };
    fetch_row_content(kind, Some(netflix_id), None, None, None, 1).await
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TmdbImages {
    pub logos: Vec<TmdbImage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TmdbImage {
    pub file_path: String,
    pub iso_639_1: Option<String>,
}

pub async fn fetch_movie_logo(id: u32, is_tv: bool) -> Result<Option<String>, gloo_net::Error> {
    let kind = if is_tv { "tv" } else { "movie" };
    let url = format!(
        "{}/{}/{}/images?api_key={}",
        TMDB_BASE_URL, kind, id, TMDB_API_KEY
    );
    let resp: TmdbImages = Request::get(&url).send().await?.json().await?;
    
    // Find english logo or first available
    let logo = resp.logos.iter().find(|l| l.iso_639_1.as_deref() == Some("en"))
        .or_else(|| resp.logos.first());
        
    Ok(logo.map(|l| format!("{}/w300{}", TMDB_IMAGE_BASE, l.file_path)))
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DetailedMediaItem {
    pub id: u32,
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: f64,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub runtime: Option<u32>,
    pub number_of_seasons: Option<u32>,
    pub credits: Option<Credits>,
    pub genres: Option<Vec<Genre>>,
}

impl DetailedMediaItem {
    pub fn display_title(&self) -> &str {
        self.title.as_deref().or(self.name.as_deref()).unwrap_or("Unknown")
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CastMember {
    pub name: String,
}

pub async fn fetch_details(id: u32, is_tv: bool) -> Result<DetailedMediaItem, gloo_net::Error> {
    let kind = if is_tv { "tv" } else { "movie" };
    let url = format!(
        "{}/{}/{}?api_key={}&append_to_response=credits",
        TMDB_BASE_URL, kind, id, TMDB_API_KEY
    );
    Request::get(&url).send().await?.json().await
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

/// Search across TV and Movies
pub async fn search_media(query: &str) -> Result<Vec<MediaItem>, gloo_net::Error> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }
    let url = format!(
        "{}/search/multi?api_key={}&language=en-US&query={}&page=1&include_adult=false",
        TMDB_BASE_URL, TMDB_API_KEY, query
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    // Filter out people, nsfw content, and items without artwork
    let filtered = resp.results.into_iter().filter(|i| {
        i.media_type.as_deref() != Some("person")
            && !i.is_nsfw()
            && (i.backdrop_path.is_some() || i.poster_path.is_some())
    }).collect();
    Ok(filtered)
}

/// Fetch recommendations (More Like This)
pub async fn fetch_recommendations(id: u32, is_tv: bool) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let kind = if is_tv { "tv" } else { "movie" };
    let url = format!(
        "{}/{}/{}/recommendations?api_key={}&language=en-US&page=1",
        TMDB_BASE_URL, kind, id, TMDB_API_KEY
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    let filtered = resp.results.into_iter().filter(|i| {
        !i.is_nsfw() && (i.backdrop_path.is_some() || i.poster_path.is_some())
    }).collect();
    Ok(filtered)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoResult {
    pub key: String,
    pub site: String,
    pub r#type: String, // "Trailer", "Clip", etc
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TmdbVideos {
    pub results: Vec<VideoResult>,
}

/// Fetch YouTube trailers/clips
pub async fn fetch_videos(id: u32, is_tv: bool) -> Result<Vec<VideoResult>, gloo_net::Error> {
    let kind = if is_tv { "tv" } else { "movie" };
    let url = format!(
        "{}/{}/{}/videos?api_key={}&language=en-US",
        TMDB_BASE_URL, kind, id, TMDB_API_KEY
    );
    let resp: TmdbVideos = Request::get(&url).send().await?.json().await?;
    Ok(resp.results)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SeasonDetails {
    pub _id: String,
    pub episodes: Vec<Episode>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub episode_number: u32,
    pub still_path: Option<String>,
    pub runtime: Option<u32>,
}

impl Episode {
    pub fn still_url(&self, size: &str) -> Option<String> {
        self.still_path
            .as_ref()
            .map(|p| format!("{}/{}{}", TMDB_IMAGE_BASE, size, p))
    }
}

/// Fetch TV season episodes
pub async fn fetch_season_details(series_id: u32, season_number: u32) -> Result<SeasonDetails, gloo_net::Error> {
    let url = format!(
        "{}/tv/{}/season/{}?api_key={}&language=en-US",
        TMDB_BASE_URL, series_id, season_number, TMDB_API_KEY
    );
    Request::get(&url).send().await?.json().await
}

pub async fn fetch_trending_page(kind: &str, page: u32) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let safe_kind = if kind == "tv" { "tv" } else { "movie" };
    let url = format!("{}/trending/{}/day?api_key={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, page);
    let resp = Request::get(&url).send().await?;
    let data: TmdbResponse<MediaItem> = resp.json().await?;
    let items = data.results.into_iter().filter_map(|mut item| {
        if item.is_nsfw() {
            return None;
        }
        if item.backdrop_path.is_none() && item.poster_path.is_none() {
            return None;
        }
        if item.media_type.is_none() {
            item.media_type = Some(safe_kind.to_string());
        }
        Some(item)
    }).collect();
    Ok(items)
}

/// Robust TMDB Query Engine for Rows, Feeds, and Category Pages.
/// Dispatches to distinct endpoints and queries with correct media_type separation,
/// quality floors, content moderation, and anti-leakage filters.
pub async fn fetch_row_content(
    kind: &str,
    genre_id: Option<&str>,
    sort_by: Option<&str>,
    extra_params: Option<&str>,
    endpoint: Option<&str>,
    page: u32,
) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let safe_kind = if kind == "tv" { "tv" } else { "movie" };

    // 1. Explicit Endpoint (e.g. "/movie/now_playing", "/movie/upcoming", "/trending/tv/week")
    if let Some(ep) = endpoint {
        let ep_clean = ep.trim_start_matches('/');
        let sep = if ep_clean.contains('?') { "&" } else { "?" };
        let mut url = format!("{}/{}{}api_key={}&include_adult=false&page={}", TMDB_BASE_URL, ep_clean, sep, TMDB_API_KEY, page);
        if let Some(extra) = extra_params {
            if !extra.is_empty() {
                let extra_clean = extra.trim_start_matches('&');
                url.push_str(&format!("&{}", extra_clean));
            }
        }
        if let Ok(resp) = Request::get(&url).send().await {
            if let Ok(data) = resp.json::<TmdbResponse<MediaItem>>().await {
                let items: Vec<MediaItem> = data.results.into_iter().filter_map(|mut item| {
                    if item.is_nsfw() {
                        return None;
                    }
                    if item.backdrop_path.is_none() && item.poster_path.is_none() {
                        return None;
                    }
                    if item.media_type.is_none() {
                        item.media_type = Some(safe_kind.to_string());
                    }
                    Some(item)
                }).collect();
                if !items.is_empty() {
                    return Ok(items);
                }
            }
        }
        return fetch_trending_page(safe_kind, page).await;
    }

    // 2. Genre or Category ID
    if let Some(gid) = genre_id {
        let gid_lower = gid.to_lowercase();
        let default_sort = sort_by.unwrap_or("popularity.desc");
        let has_custom_votes = extra_params.map(|e| e.contains("vote_count.gte")).unwrap_or(false);

        let mut url = match gid_lower.as_str() {
            // Binge-worthy / Boredom Busters
            "binge" | "1191605" | "boredom" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=40" };
                if safe_kind == "tv" {
                    format!("{}/discover/tv?api_key={}&with_genres=10759,80,10765&without_genres=16,10764{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                } else {
                    format!("{}/discover/movie?api_key={}&with_genres=28,12,53,878&without_genres=16{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                }
            }
            // British origin
            "british" | "52117" | "10005" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=30" };
                format!("{}/discover/{}?api_key={}&with_origin_country=GB{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, vote_filter, default_sort, page)
            }
            // US origin
            "us" | "10008" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=40" };
                format!("{}/discover/{}?api_key={}&with_origin_country=US{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, vote_filter, default_sort, page)
            }
            // European origin
            "european" | "10006" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=25" };
                format!("{}/discover/{}?api_key={}&with_origin_country=FR|DE|IT|ES|NL|DK|SE|NO|FI|PL{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, vote_filter, default_sort, page)
            }
            // International (non-English)
            "international" | "10012" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=30" };
                format!("{}/discover/{}?api_key={}&without_original_language=en{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, vote_filter, default_sort, page)
            }
            // Anime (Strict vote floor + adult keyword exclusion wipes out all low-vote hentai/ecchi shorts)
            "anime" | "7424" => {
                let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=60" };
                format!("{}/discover/{}?api_key={}&with_genres=16&with_original_language=ja{}&without_keywords=190370,222243,267498&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, safe_kind, TMDB_API_KEY, vote_filter, default_sort, page)
            }
            // Numeric IDs or Netflix codes
            _ => {
                let resolved_genre_id = match gid_lower.as_str() {
                    "1365" => if safe_kind == "tv" { "10759" } else { "28" },
                    "6548" | "10375" => "35",
                    "1492" | "1372" => if safe_kind == "tv" { "10765" } else { "878" },
                    "8933" => if safe_kind == "tv" { "9648" } else { "53" },
                    "5763" | "11714" => "18",
                    "8711" => if safe_kind == "tv" { "9648" } else { "27" },
                    "8883" => "10749",
                    "6839" => "99",
                    "9875" | "26146" => "80",
                    "783" => if safe_kind == "tv" { "10762" } else { "10751" },
                    "10673" => "10759",
                    // TMDB cross-media resolution
                    "28" | "12" => if safe_kind == "tv" { "10759" } else { gid },
                    "878" => if safe_kind == "tv" { "10765" } else { "878" },
                    "10751" => if safe_kind == "tv" { "10762" } else { "10751" },
                    "10759" => if safe_kind == "movie" { "28" } else { "10759" },
                    "10765" => if safe_kind == "movie" { "878" } else { "10765" },
                    "10762" => if safe_kind == "movie" { "10751" } else { "10762" },
                    _ => gid,
                };

                if safe_kind == "tv" {
                    match resolved_genre_id {
                        // Docuseries: require vote threshold and modern era to avoid 1-vote local foreign broadcasts
                        "99" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=25" };
                            format!("{}/discover/tv?api_key={}&with_genres=99{}&first_air_date.gte=2000-01-01&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        // Action & Adventure TV: exclude animation & kids cartoons (Doraemon, etc.)
                        "10759" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=35" };
                            format!("{}/discover/tv?api_key={}&with_genres=10759&without_genres=16,10762{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        // Kids & Family TV: modern era threshold to avoid 1960s relics like Capitão Furacão
                        "10762" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=20" };
                            format!("{}/discover/tv?api_key={}&with_genres=10762&first_air_date.gte=1990-01-01{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        // Sci-Fi & Fantasy TV: exclude pure animation to keep live-action series prominent
                        "10765" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=30" };
                            format!("{}/discover/tv?api_key={}&with_genres=10765&without_genres=16{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        // Comedies & Dramas: exclude reality/talk shows
                        "35" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=30" };
                            format!("{}/discover/tv?api_key={}&with_genres=35&without_genres=10764,10767{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        "18" => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=30" };
                            format!("{}/discover/tv?api_key={}&with_genres=18&without_genres=10764,10767{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, vote_filter, default_sort, page)
                        }
                        _ => {
                            let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=25" };
                            format!("{}/discover/tv?api_key={}&with_genres={}{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, resolved_genre_id, vote_filter, default_sort, page)
                        }
                    }
                } else {
                    let vote_filter = if has_custom_votes { "" } else { "&vote_count.gte=35" };
                    format!("{}/discover/movie?api_key={}&with_genres={}{}&include_adult=false&sort_by={}&page={}", TMDB_BASE_URL, TMDB_API_KEY, resolved_genre_id, vote_filter, default_sort, page)
                }
            }
        };

        if let Some(extra) = extra_params {
            if !extra.is_empty() {
                let extra_clean = extra.trim_start_matches('&');
                url.push_str(&format!("&{}", extra_clean));
            }
        }

        if let Ok(resp) = Request::get(&url).send().await {
            if let Ok(data) = resp.json::<TmdbResponse<MediaItem>>().await {
                let items: Vec<MediaItem> = data.results.into_iter().filter_map(|mut item| {
                    if item.is_nsfw() {
                        return None;
                    }
                    if item.backdrop_path.is_none() && item.poster_path.is_none() {
                        return None;
                    }
                    if item.media_type.is_none() {
                        item.media_type = Some(safe_kind.to_string());
                    }
                    Some(item)
                }).collect();
                if !items.is_empty() {
                    return Ok(items);
                }
            }
        }
    }

    // 3. Fallback: ALWAYS respect safe_kind so TV series never show movies
    fetch_trending_page(safe_kind, page).await
}

pub async fn fetch_for_genre_route_page(netflix_id: &str, page: u32) -> Result<Vec<MediaItem>, gloo_net::Error> {
    fetch_row_content("movie", Some(netflix_id), None, None, None, page).await
}

pub async fn fetch_by_language(lang: &str, page: u32) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url_movie = format!(
        "{}/discover/movie?api_key={}&with_original_language={}&sort_by=popularity.desc&page={}",
        TMDB_BASE_URL, TMDB_API_KEY, lang, page
    );
    let resp_movie: Result<TmdbResponse<MediaItem>, _> = Request::get(&url_movie).send().await?.json().await;

    let url_tv = format!(
        "{}/discover/tv?api_key={}&with_original_language={}&sort_by=popularity.desc&page={}",
        TMDB_BASE_URL, TMDB_API_KEY, lang, page
    );
    let resp_tv: Result<TmdbResponse<MediaItem>, _> = Request::get(&url_tv).send().await?.json().await;

    let mut combined = resp_movie.map(|r| r.results).unwrap_or_default();
    if let Ok(tv_resp) = resp_tv {
        for mut item in tv_resp.results {
            item.media_type = Some("tv".to_string());
            combined.push(item);
        }
    }
    Ok(combined)
}

