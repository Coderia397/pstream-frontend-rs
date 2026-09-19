pub enum MediaKind {
    Movie,
    Tv,
}

pub struct CategoryContext {
    pub kind: MediaKind,
    pub tmdb_genre_id: Option<u32>,
    pub title: &'static str,
}

/// Maps Netflix's internal UI routing IDs to TMDB API parameters.
pub fn map_netflix_id_to_tmdb(netflix_id: &str) -> Option<CategoryContext> {
    match netflix_id {
        // Core Types
        "34" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Films",
        }),
        "83" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "Series",
        }),

        // Movie Genres
        "1365" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(28),
            title: "Action & Adventure",
        }),
        "6548" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(35),
            title: "Comedies",
        }),
        "8711" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(27),
            title: "Horror Films",
        }),
        "8933" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(53),
            title: "Thrillers",
        }),
        "5763" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(18),
            title: "Dramas",
        }),
        "1492" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(878),
            title: "Sci-Fi & Fantasy",
        }),
        "6839" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(99),
            title: "Documentaries",
        }),
        "8883" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(10749),
            title: "Romantic Films",
        }),
        "7424" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(16),
            title: "Anime & Animation",
        }),
        "783" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(10751),
            title: "Children & Family",
        }),
        "9875" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(80),
            title: "Crime Films",
        }),

        // TV Genres
        "10673" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(10759),
            title: "Action & Adventure (TV)",
        }),
        "10375" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(35),
            title: "TV Comedies",
        }),
        "11714" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(18),
            title: "TV Dramas",
        }),
        "26146" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(80),
            title: "Crime TV Shows",
        }),
        "1372" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(10765),
            title: "TV Sci-Fi & Fantasy",
        }),
        // Common and Synthetic Categories
        "10001" | "lgbtq" | "pride" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "LGBTQ",
        }),
        "10003" | "black stories" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Black Stories",
        }),
        "10005" | "52117" | "british" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "British",
        }),
        "10009" | "classics" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Classics",
        }),
        "10010" | "cult" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Cult",
        }),
        "10006" | "european" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "European",
        }),
        "10016" | "halloween" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Halloween",
        }),
        "10008" | "hollywood" | "us" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Hollywood",
        }),
        "10011" | "independent" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Independent",
        }),
        "10012" | "international" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "International",
        }),
        "10013" | "shorts" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Shorts",
        }),
        "10014" | "sports" | "sport" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Sports",
        }),
        "10017" | "stand-up" | "stand-up comedy" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: None,
            title: "Stand-up Comedy",
        }),
        "10018" | "emmy" | "emmys" | "emmy or emmys" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "Emmys",
        }),
        "10019" | "science & nature" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "Science & Nature",
        }),
        "10020" | "stand-up & chat shows" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "Stand-up & Chat Shows",
        }),
        "10015" | "teen" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: None,
            title: "Teen",
        }),
        "10402" => Some(CategoryContext {
            kind: MediaKind::Movie,
            tmdb_genre_id: Some(10402),
            title: "Music & Musicals",
        }),
        "10764" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(10764),
            title: "Reality",
        }),
        "9648" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(9648),
            title: "Mystery",
        }),
        "10762" => Some(CategoryContext {
            kind: MediaKind::Tv,
            tmdb_genre_id: Some(10762),
            title: "Kids",
        }),

        _ => None,
    }
}
