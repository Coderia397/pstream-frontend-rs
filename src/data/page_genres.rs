//! Page-level Genre Dictionaries and Cross-Media ID Resolution.
//!
//! Zero-cost compile-time static slices (`&'static [PageGenre]`) and `const fn` functions.
//! Ported directly from `pstream-frontend/data/pageGenres.ts` and `pstream-frontend/hooks/kidsManifestBuilder.ts`.

use super::genres::{Genre, MediaType};

/// Type alias aligning PageGenre with the core Genre definition.
pub type PageGenre = Genre;

/// TMDB Movie Genres following the Netflix naming convention (27 entries).
pub static MOVIE_GENRES: &[PageGenre] = &[
    PageGenre { id: 28, name: "Action" },
    PageGenre { id: 16, name: "Anime" },
    PageGenre { id: 10002, name: "Astrology" },
    PageGenre { id: 10003, name: "Black Stories" },
    PageGenre { id: 10004, name: "Book Adaptations" },
    PageGenre { id: 10005, name: "British" },
    PageGenre { id: 10009, name: "Classics" },
    PageGenre { id: 35, name: "Comedies" },
    PageGenre { id: 80, name: "Crime" },
    PageGenre { id: 10010, name: "Cult" },
    PageGenre { id: 99, name: "Documentaries" },
    PageGenre { id: 18, name: "Dramas" },
    PageGenre { id: 10006, name: "European" },
    PageGenre { id: 14, name: "Fantasy" },
    PageGenre { id: 10008, name: "Hollywood" },
    PageGenre { id: 27, name: "Horror" },
    PageGenre { id: 10011, name: "Independent" },
    PageGenre { id: 10012, name: "International" },
    PageGenre { id: 10751, name: "Kids & Family" },
    PageGenre { id: 10007, name: "Moods" },
    PageGenre { id: 10402, name: "Music & Musicals" },
    PageGenre { id: 10001, name: "Pride" },
    PageGenre { id: 10749, name: "Romance" },
    PageGenre { id: 878, name: "Sci-Fi" },
    PageGenre { id: 10013, name: "Shorts" },
    PageGenre { id: 10014, name: "Sport" },
    PageGenre { id: 53, name: "Thriller" },
];

/// TMDB TV Genres following the Netflix naming convention (25 entries).
pub static TV_GENRES: &[PageGenre] = &[
    PageGenre { id: 10759, name: "Action" },
    PageGenre { id: 16, name: "Anime" },
    PageGenre { id: 10002, name: "Astrology" },
    PageGenre { id: 10003, name: "Black Stories" },
    PageGenre { id: 10004, name: "Book Adaptations" },
    PageGenre { id: 10005, name: "British" },
    PageGenre { id: 35, name: "Comedies" },
    PageGenre { id: 80, name: "Crime" },
    PageGenre { id: 99, name: "Documentary Series" },
    PageGenre { id: 18, name: "Dramas" },
    PageGenre { id: 10006, name: "European" },
    PageGenre { id: 27, name: "Horror" },
    PageGenre { id: 10012, name: "International" },
    PageGenre { id: 10762, name: "Kids" },
    PageGenre { id: 10007, name: "Moods" },
    PageGenre { id: 9648, name: "Mysteries" },
    PageGenre { id: 10001, name: "Pride" },
    PageGenre { id: 10764, name: "Reality" },
    PageGenre { id: 10749, name: "Romance" },
    PageGenre { id: 10765, name: "Sci-Fi & Fantasy" },
    PageGenre { id: 99, name: "Science & Nature" },
    PageGenre { id: 10014, name: "Sport" },
    PageGenre { id: 10015, name: "Teen" },
    PageGenre { id: 53, name: "Thriller" },
    PageGenre { id: 10008, name: "US" },
];

/// Cross-media genre ID mapping for hybrid home discover (movie <-> TV TMDB ids).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HomeGenreMapping {
    pub source_id: u32,
    pub movie: u32,
    pub tv: u32,
}

pub static HOME_GENRE_ID_MAP: &[HomeGenreMapping] = &[
    HomeGenreMapping { source_id: 12, movie: 12, tv: 10759 },
    HomeGenreMapping { source_id: 14, movie: 14, tv: 10765 },
    HomeGenreMapping { source_id: 28, movie: 28, tv: 10759 },
    HomeGenreMapping { source_id: 53, movie: 53, tv: 9648 },
    HomeGenreMapping { source_id: 878, movie: 878, tv: 10765 },
    HomeGenreMapping { source_id: 9648, movie: 53, tv: 9648 },
    HomeGenreMapping { source_id: 10751, movie: 10751, tv: 10762 },
    HomeGenreMapping { source_id: 10759, movie: 28, tv: 10759 },
    HomeGenreMapping { source_id: 10762, movie: 10751, tv: 10762 },
    HomeGenreMapping { source_id: 10765, movie: 878, tv: 10765 },
];

/// Direct lookup of cross-media TMDB ID pairing.
pub const fn get_home_genre_mapping(selected_genre_id: u32) -> Option<(u32, u32)> {
    match selected_genre_id {
        12 => Some((12, 10759)),
        14 => Some((14, 10765)),
        28 => Some((28, 10759)),
        53 => Some((53, 9648)),
        878 => Some((878, 10765)),
        9648 => Some((53, 9648)),
        10751 => Some((10751, 10762)),
        10759 => Some((28, 10759)),
        10762 => Some((10751, 10762)),
        10765 => Some((878, 10765)),
        _ => None,
    }
}

/// Resolves a selected genre ID to the appropriate TMDB ID for the requested media type.
pub const fn resolve_genre_id(media_type: MediaType, selected_genre_id: u32) -> u32 {
    if let Some((movie, tv)) = get_home_genre_mapping(selected_genre_id) {
        match media_type {
            MediaType::Movie => movie,
            MediaType::Tv => tv,
        }
    } else {
        selected_genre_id
    }
}

/// TV-only genre IDs (9 entries).
pub static TV_ONLY_GENRE_IDS: &[u32] = &[
    10759, 10762, 10763, 10764, 10765, 10766, 10768, 10015, 9648,
];

pub const fn is_tv_only_genre_id(genre_id: u32) -> bool {
    matches!(
        genre_id,
        10759 | 10762 | 10763 | 10764 | 10765 | 10766 | 10768 | 10015 | 9648
    )
}

/// Movie-only genre IDs (6 entries).
pub static MOVIE_ONLY_GENRE_IDS: &[u32] = &[
    12, 36, 53, 10402, 10751, 10013,
];

pub const fn is_movie_only_genre_id(genre_id: u32) -> bool {
    matches!(
        genre_id,
        12 | 36 | 53 | 10402 | 10751 | 10013
    )
}

/// Merged mobile Home genre picker: movie-forward base + TV-only / distinct variants (35 entries).
pub static HOME_MOBILE_GENRES: &[PageGenre] = &[
    PageGenre { id: 28, name: "Action" },
    PageGenre { id: 16, name: "Anime" },
    PageGenre { id: 10002, name: "Astrology" },
    PageGenre { id: 10003, name: "Black Stories" },
    PageGenre { id: 10004, name: "Book Adaptations" },
    PageGenre { id: 10005, name: "British" },
    PageGenre { id: 10009, name: "Classics" },
    PageGenre { id: 35, name: "Comedies" },
    PageGenre { id: 80, name: "Crime" },
    PageGenre { id: 10010, name: "Cult" },
    PageGenre { id: 99, name: "Documentaries" },
    PageGenre { id: 18, name: "Dramas" },
    PageGenre { id: 10006, name: "European" },
    PageGenre { id: 14, name: "Fantasy" },
    PageGenre { id: 10008, name: "Hollywood" },
    PageGenre { id: 27, name: "Horror" },
    PageGenre { id: 10011, name: "Independent" },
    PageGenre { id: 10012, name: "International" },
    PageGenre { id: 10751, name: "Kids & Family" },
    PageGenre { id: 10007, name: "Moods" },
    PageGenre { id: 10402, name: "Music & Musicals" },
    PageGenre { id: 10001, name: "Pride" },
    PageGenre { id: 10749, name: "Romance" },
    PageGenre { id: 878, name: "Sci-Fi" },
    PageGenre { id: 10013, name: "Shorts" },
    PageGenre { id: 10014, name: "Sport" },
    PageGenre { id: 53, name: "Thriller" },
    PageGenre { id: 10762, name: "Kids" },
    PageGenre { id: 10764, name: "Reality" },
    PageGenre { id: 10765, name: "Sci-Fi & Fantasy" },
    PageGenre { id: 99, name: "Science & Nature" },
    PageGenre { id: 99, name: "Documentary Series" },
    PageGenre { id: 10015, name: "Teen" },
    PageGenre { id: 10008, name: "US" },
    PageGenre { id: 9648, name: "Mysteries" },
];

/// Lookup movie genre name by TMDB ID.
pub fn get_movie_genre_name(id: u32) -> Option<&'static str> {
    let mut i = 0;
    while i < MOVIE_GENRES.len() {
        if MOVIE_GENRES[i].id == id {
            return Some(MOVIE_GENRES[i].name);
        }
        i += 1;
    }
    None
}

/// Display name for manifest row titles when needed.
pub fn get_home_genre_display_name(id: u32, selected_name: Option<&'static str>) -> &'static str {
    if let Some(name) = selected_name {
        name
    } else if let Some(name) = get_movie_genre_name(id) {
        name
    } else {
        "Content"
    }
}

/// Universal filter set (My List, Search filters, etc., 18 entries).
pub static UNIVERSAL_GENRES: &[PageGenre] = &[
    PageGenre { id: 28, name: "Action" },
    PageGenre { id: 12, name: "Adventure" },
    PageGenre { id: 16, name: "Anime" },
    PageGenre { id: 35, name: "Comedies" },
    PageGenre { id: 80, name: "Crime" },
    PageGenre { id: 99, name: "Documentaries" },
    PageGenre { id: 18, name: "Dramas" },
    PageGenre { id: 10751, name: "Children & Family" },
    PageGenre { id: 14, name: "Fantasy" },
    PageGenre { id: 36, name: "History" },
    PageGenre { id: 27, name: "Horror" },
    PageGenre { id: 10402, name: "Music & Musicals" },
    PageGenre { id: 9648, name: "Mysteries" },
    PageGenre { id: 10749, name: "Romance" },
    PageGenre { id: 878, name: "Sci-Fi" },
    PageGenre { id: 53, name: "Thriller" },
    PageGenre { id: 10752, name: "War" },
    PageGenre { id: 37, name: "Western" },
];

/// Kids TV Genres catalog (synthetic 21000+ series IDs, 16 entries).
pub static KIDS_TV_GENRES: &[PageGenre] = &[
    PageGenre { id: 21001, name: "Action" },
    PageGenre { id: 21002, name: "Animal Time" },
    PageGenre { id: 21003, name: "Cars, Trucks & Trains" },
    PageGenre { id: 21004, name: "Dinosaurs" },
    PageGenre { id: 21005, name: "Fantasy" },
    PageGenre { id: 21006, name: "Feel-good" },
    PageGenre { id: 21007, name: "For Little Ones" },
    PageGenre { id: 21008, name: "Friends" },
    PageGenre { id: 21009, name: "Funny" },
    PageGenre { id: 21010, name: "Girls Take the Lead" },
    PageGenre { id: 21011, name: "Reality" },
    PageGenre { id: 21012, name: "Sci-Fi" },
    PageGenre { id: 21013, name: "Science & Nature" },
    PageGenre { id: 21014, name: "Singing & Dancing" },
    PageGenre { id: 21015, name: "Spooky Stuff" },
    PageGenre { id: 21016, name: "Watch with the Family" },
];

/// Kids Movie Genres catalog (synthetic 21000+ movie IDs, 17 entries).
pub static KIDS_MOVIE_GENRES: &[PageGenre] = &[
    PageGenre { id: 21001, name: "Action" },
    PageGenre { id: 21017, name: "Adventure" },
    PageGenre { id: 21002, name: "Animals" },
    PageGenre { id: 21018, name: "Animated" },
    PageGenre { id: 21019, name: "Documentaries" },
    PageGenre { id: 21005, name: "Fantasy" },
    PageGenre { id: 21006, name: "Feel-good" },
    PageGenre { id: 21007, name: "For Little Ones" },
    PageGenre { id: 21008, name: "Friends" },
    PageGenre { id: 21009, name: "Funny" },
    PageGenre { id: 21010, name: "Girls Take the Lead" },
    PageGenre { id: 21020, name: "Princess Tales" },
    PageGenre { id: 21012, name: "Sci-Fi" },
    PageGenre { id: 21013, name: "Science & Nature" },
    PageGenre { id: 21014, name: "Singing & Dancing" },
    PageGenre { id: 21015, name: "Spooky Stuff" },
    PageGenre { id: 21016, name: "Watch with the Family" },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        assert_eq!(MOVIE_GENRES.len(), 27);
        assert_eq!(TV_GENRES.len(), 25);
        assert_eq!(HOME_GENRE_ID_MAP.len(), 10);
        assert_eq!(HOME_MOBILE_GENRES.len(), 35);
        assert_eq!(UNIVERSAL_GENRES.len(), 18);
        assert_eq!(KIDS_TV_GENRES.len(), 16);
        assert_eq!(KIDS_MOVIE_GENRES.len(), 17);
    }

    #[test]
    fn test_resolve_genre_id() {
        assert_eq!(resolve_genre_id(MediaType::Movie, 28), 28);
        assert_eq!(resolve_genre_id(MediaType::Tv, 28), 10759);
        assert_eq!(resolve_genre_id(MediaType::Movie, 10759), 28);
        assert_eq!(resolve_genre_id(MediaType::Tv, 10759), 10759);
        assert_eq!(resolve_genre_id(MediaType::Movie, 14), 14);
        assert_eq!(resolve_genre_id(MediaType::Tv, 14), 10765);
        assert_eq!(resolve_genre_id(MediaType::Movie, 53), 53);
        assert_eq!(resolve_genre_id(MediaType::Tv, 53), 9648);
        assert_eq!(resolve_genre_id(MediaType::Movie, 35), 35);
        assert_eq!(resolve_genre_id(MediaType::Movie, 9999), 9999);
    }

    #[test]
    fn test_tv_movie_only_filters() {
        assert!(is_tv_only_genre_id(10759));
        assert!(is_tv_only_genre_id(9648));
        assert!(!is_tv_only_genre_id(28));

        assert!(is_movie_only_genre_id(12));
        assert!(is_movie_only_genre_id(53));
        assert!(!is_movie_only_genre_id(28));
    }

    #[test]
    fn test_display_names() {
        assert_eq!(get_home_genre_display_name(28, None), "Action");
        assert_eq!(get_home_genre_display_name(28, Some("Custom")), "Custom");
        assert_eq!(get_home_genre_display_name(9999, None), "Content");
    }
}
