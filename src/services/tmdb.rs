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
    Ok(resp.results)
}

/// Fetch top rated items
pub async fn fetch_top_rated(kind: &str) -> Result<Vec<MediaItem>, gloo_net::Error> {
    let url = format!(
        "{}/{}/top_rated?api_key={}&language=en-US",
        TMDB_BASE_URL, kind, TMDB_API_KEY
    );
    let resp: TmdbResponse<MediaItem> = Request::get(&url).send().await?.json().await?;
    Ok(resp.results)
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
    match map_netflix_id_to_tmdb(netflix_id) {
        None => Ok(vec![]),
        Some(ctx) => match ctx.tmdb_genre_id {
            // Genre-specific query
            Some(gid) => {
                let kind = match ctx.kind {
                    MediaKind::Movie => "movie",
                    MediaKind::Tv => "tv",
                };
                fetch_by_genre(kind, gid).await
            }
            // Broad category with no genre filter — use trending
            None => match ctx.kind {
                MediaKind::Movie => fetch_trending("movie").await,
                MediaKind::Tv => {
                    if netflix_id == "52117" {
                        fetch_british_tv().await
                    } else {
                        fetch_trending("tv").await
                    }
                }
            },
        },
    }
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
    // Filter out people
    let filtered = resp.results.into_iter().filter(|i| i.media_type.as_deref() != Some("person")).collect();
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
    Ok(resp.results)
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
    let url = format!("{}/trending/{}/day?api_key={}&page={}", TMDB_BASE_URL, kind, TMDB_API_KEY, page);
    let resp = Request::get(&url).send().await?;
    let data: TmdbResponse<MediaItem> = resp.json().await?;
    Ok(data.results)
}

pub async fn fetch_for_genre_route_page(netflix_id: &str, page: u32) -> Result<Vec<MediaItem>, gloo_net::Error> {
    if let Some(ctx) = map_netflix_id_to_tmdb(netflix_id) {
        if let Some(tmdb_genre_id) = ctx.tmdb_genre_id {
            let kind_str = match ctx.kind {
                MediaKind::Tv => "tv",
                MediaKind::Movie => "movie",
            };
            let url = format!("{}/discover/{}?api_key={}&with_genres={}&sort_by=popularity.desc&page={}", TMDB_BASE_URL, kind_str, TMDB_API_KEY, tmdb_genre_id, page);
            let resp = Request::get(&url).send().await?;
            let data: TmdbResponse<MediaItem> = resp.json().await?;
            return Ok(data.results);
        }
    }
    fetch_trending_page("movie", page).await
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

