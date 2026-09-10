use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Movie {
    pub id: serde_json::Value,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub original_title: Option<String>,
    #[serde(default)]
    pub original_name: Option<String>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub first_air_date: Option<String>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub genre_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub adult: Option<bool>,
    #[serde(default)]
    pub original_language: Option<String>,
    #[serde(default)]
    pub runtime: Option<i64>,
    #[serde(default)]
    pub number_of_seasons: Option<i64>,
    #[serde(default)]
    pub imdb_id: Option<String>,
    #[serde(default)]
    pub vote_count: Option<i64>,
    #[serde(default)]
    pub popularity: Option<f64>,
    #[serde(default)]
    pub certification: Option<String>,
}

impl Movie {
    pub fn display_title(&self) -> String {
        self.title
            .as_deref()
            .or(self.name.as_deref())
            .or(self.original_title.as_deref())
            .or(self.original_name.as_deref())
            .unwrap_or("Untitled")
            .to_string()
    }

    pub fn is_tv(&self) -> bool {
        self.media_type.as_deref() == Some("tv")
            || (self.media_type.is_none() && self.title.is_none() && self.name.is_some())
    }

    pub fn id_u32(&self) -> u32 {
        if let Some(n) = self.id.as_u64() {
            n as u32
        } else if let Some(s) = self.id.as_str() {
            s.parse::<u32>().unwrap_or(0)
        } else {
            0
        }
    }
}

impl From<crate::services::tmdb::MediaItem> for Movie {
    fn from(m: crate::services::tmdb::MediaItem) -> Self {
        Movie {
            id: serde_json::Value::from(m.id),
            title: m.title,
            name: m.name,
            overview: m.overview,
            poster_path: m.poster_path,
            backdrop_path: m.backdrop_path,
            vote_average: m.vote_average,
            release_date: m.release_date,
            first_air_date: m.first_air_date,
            media_type: m.media_type,
            ..Default::default()
        }
    }
}

impl From<crate::services::tmdb::DetailedMediaItem> for Movie {
    fn from(d: crate::services::tmdb::DetailedMediaItem) -> Self {
        Movie {
            id: serde_json::Value::from(d.id),
            title: d.title,
            name: d.name,
            overview: d.overview,
            poster_path: d.poster_path,
            backdrop_path: d.backdrop_path,
            vote_average: d.vote_average,
            release_date: d.release_date,
            first_air_date: d.first_air_date,
            runtime: d.runtime.map(|r| r as i64),
            number_of_seasons: d.number_of_seasons.map(|s| s as i64),
            ..Default::default()
        }
    }
}

