//! TMDB Genre Mappings, Adjacent Genres, and Micro-Genre Catalog.
//!
//! Zero-cost compile-time static slices (`&'static [T]`) and `const fn` lookups.
//! Ported directly from `pstream-frontend/data/genres.ts` and `pstream-frontend/data/themes.ts`.

use serde::{Deserialize, Serialize};

/// Target media type for queries and micro-genres.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "tv")]
    Tv,
}

impl MediaType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Movie => "movie",
            Self::Tv => "tv",
        }
    }

    pub const fn is_movie(&self) -> bool {
        matches!(self, Self::Movie)
    }

    pub const fn is_tv(&self) -> bool {
        matches!(self, Self::Tv)
    }
}

/// Core TMDB Genre entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Genre {
    pub id: u32,
    pub name: &'static str,
}

/// Standard TMDB Genre ID -> Name map (27 entries).
pub static GENRES: &[Genre] = &[
    Genre { id: 12, name: "Adventure" },
    Genre { id: 14, name: "Fantasy" },
    Genre { id: 16, name: "Animation" },
    Genre { id: 18, name: "Drama" },
    Genre { id: 27, name: "Horror" },
    Genre { id: 28, name: "Action" },
    Genre { id: 35, name: "Comedy" },
    Genre { id: 36, name: "History" },
    Genre { id: 37, name: "Western" },
    Genre { id: 53, name: "Thriller" },
    Genre { id: 80, name: "Crime" },
    Genre { id: 99, name: "Documentary" },
    Genre { id: 878, name: "Science Fiction" },
    Genre { id: 9648, name: "Mystery" },
    Genre { id: 10402, name: "Music" },
    Genre { id: 10749, name: "Romance" },
    Genre { id: 10751, name: "Family" },
    Genre { id: 10752, name: "War" },
    Genre { id: 10759, name: "Action & Adventure" },
    Genre { id: 10762, name: "Family" },
    Genre { id: 10763, name: "News" },
    Genre { id: 10764, name: "Reality" },
    Genre { id: 10765, name: "Sci-Fi & Fantasy" },
    Genre { id: 10766, name: "Soap" },
    Genre { id: 10767, name: "Talk" },
    Genre { id: 10768, name: "War & Politics" },
    Genre { id: 10770, name: "TV Movie" },
];

/// Constant-time genre name lookup by TMDB genre ID.
pub const fn get_genre_name(id: u32) -> Option<&'static str> {
    match id {
        12 => Some("Adventure"),
        14 => Some("Fantasy"),
        16 => Some("Animation"),
        18 => Some("Drama"),
        27 => Some("Horror"),
        28 => Some("Action"),
        35 => Some("Comedy"),
        36 => Some("History"),
        37 => Some("Western"),
        53 => Some("Thriller"),
        80 => Some("Crime"),
        99 => Some("Documentary"),
        878 => Some("Science Fiction"),
        9648 => Some("Mystery"),
        10402 => Some("Music"),
        10749 => Some("Romance"),
        10751 => Some("Family"),
        10752 => Some("War"),
        10759 => Some("Action & Adventure"),
        10762 => Some("Family"),
        10763 => Some("News"),
        10764 => Some("Reality"),
        10765 => Some("Sci-Fi & Fantasy"),
        10766 => Some("Soap"),
        10767 => Some("Talk"),
        10768 => Some("War & Politics"),
        10770 => Some("TV Movie"),
        _ => None,
    }
}

/// Adjacent genre mapping container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjacentGenre {
    pub id: u32,
    pub adjacent_ids: &'static [u32],
}

/// Adjacent Genre Map: used when filtering micro-genres by selected genre.
pub static ADJACENT_GENRES: &[AdjacentGenre] = &[
    AdjacentGenre { id: 12, adjacent_ids: &[28, 14, 878, 10759, 35] },
    AdjacentGenre { id: 14, adjacent_ids: &[12, 10765, 878, 10759, 36] },
    AdjacentGenre { id: 16, adjacent_ids: &[10751, 35, 12, 10762] },
    AdjacentGenre { id: 18, adjacent_ids: &[80, 53, 9648, 10749, 36, 10768] },
    AdjacentGenre { id: 27, adjacent_ids: &[53, 9648, 35, 10765] },
    AdjacentGenre { id: 28, adjacent_ids: &[12, 53, 878, 10759, 10752] },
    AdjacentGenre { id: 35, adjacent_ids: &[10749, 18, 16, 80, 27] },
    AdjacentGenre { id: 36, adjacent_ids: &[18, 99, 10752, 80] },
    AdjacentGenre { id: 37, adjacent_ids: &[12, 28, 18, 36] },
    AdjacentGenre { id: 53, adjacent_ids: &[80, 9648, 27, 878, 18, 28] },
    AdjacentGenre { id: 80, adjacent_ids: &[53, 18, 9648, 10768, 28] },
    AdjacentGenre { id: 99, adjacent_ids: &[36, 18, 10768, 10752] },
    AdjacentGenre { id: 878, adjacent_ids: &[12, 14, 53, 10765, 9648] },
    AdjacentGenre { id: 9648, adjacent_ids: &[53, 80, 27, 18, 878] },
    AdjacentGenre { id: 10402, adjacent_ids: &[18, 10749, 99, 35] },
    AdjacentGenre { id: 10749, adjacent_ids: &[35, 18, 10402, 10751] },
    AdjacentGenre { id: 10751, adjacent_ids: &[16, 35, 12, 10762] },
    AdjacentGenre { id: 10752, adjacent_ids: &[28, 36, 18, 99, 10768] },
    AdjacentGenre { id: 10759, adjacent_ids: &[28, 12, 53, 878] },
    AdjacentGenre { id: 10762, adjacent_ids: &[16, 35, 12, 10751] },
    AdjacentGenre { id: 10764, adjacent_ids: &[35, 99, 18] },
    AdjacentGenre { id: 10765, adjacent_ids: &[878, 14, 27, 53, 12] },
    AdjacentGenre { id: 10768, adjacent_ids: &[18, 53, 36, 99, 10752] },
];

/// Constant-time adjacent genre lookup.
pub const fn get_adjacent_genres(genre_id: u32) -> &'static [u32] {
    match genre_id {
        12 => &[28, 14, 878, 10759, 35],
        14 => &[12, 10765, 878, 10759, 36],
        16 => &[10751, 35, 12, 10762],
        18 => &[80, 53, 9648, 10749, 36, 10768],
        27 => &[53, 9648, 35, 10765],
        28 => &[12, 53, 878, 10759, 10752],
        35 => &[10749, 18, 16, 80, 27],
        36 => &[18, 99, 10752, 80],
        37 => &[12, 28, 18, 36],
        53 => &[80, 9648, 27, 878, 18, 28],
        80 => &[53, 18, 9648, 10768, 28],
        99 => &[36, 18, 10768, 10752],
        878 => &[12, 14, 53, 10765, 9648],
        9648 => &[53, 80, 27, 18, 878],
        10402 => &[18, 10749, 99, 35],
        10749 => &[35, 18, 10402, 10751],
        10751 => &[16, 35, 12, 10762],
        10752 => &[28, 36, 18, 99, 10768],
        10759 => &[28, 12, 53, 878],
        10762 => &[16, 35, 12, 10751],
        10764 => &[35, 99, 18],
        10765 => &[878, 14, 27, 53, 12],
        10768 => &[18, 53, 36, 99, 10752],
        _ => &[],
    }
}

/// Micro-genre catalog entry for curated dynamic rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MicroGenreEntry {
    pub name: &'static str,
    pub genres: &'static str,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub extra: Option<&'static str>,
}

/// Master Consolidated Themed Library (102 entries from themes.ts).
pub static ALL_THEMED_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Christmas Cozy Traditions", genres: "10751,35", media_type: MediaType::Movie, extra: Some("&with_keywords=9672&vote_average.gte=6.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Christmas Action & Thrills", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_keywords=9672&vote_count.gte=1000") },
    MicroGenreEntry { name: "Nightmare Before Christmas", genres: "14,16", media_type: MediaType::Movie, extra: Some("&with_keywords=9672|3335&vote_count.gte=1000") },
    MicroGenreEntry { name: "Halloween Night Terrors", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=3335&vote_count.gte=2000") },
    MicroGenreEntry { name: "Paranormal Creepshow", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=10185&vote_average.gte=6.8&vote_count.gte=1000") },
    MicroGenreEntry { name: "Gothic Vampire Chronicles", genres: "27,18", media_type: MediaType::Movie, extra: Some("&with_keywords=3133&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Valentine's Tearjerkers", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=1000&with_keywords=9799") },
    MicroGenreEntry { name: "Quirky Rom-Com Matches", genres: "10749,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=6.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Thanksgiving Feuds & Food", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=10306&vote_count.gte=200") },
    MicroGenreEntry { name: "New Year's Resolutions", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=9673|158485&vote_average.gte=6.5") },
    MicroGenreEntry { name: "St. Patrick's Emerald Isle", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=IE&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Easter Family Adventure", genres: "10751,16", media_type: MediaType::Movie, extra: Some("&with_keywords=11370&vote_average.gte=6.5") },
    MicroGenreEntry { name: "Mother's Day Maternal Bonds", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=158566|10527&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Father's Day Dad Energies", genres: "28,18", media_type: MediaType::Movie, extra: Some("&with_keywords=158565|10526&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Fourth of July Fireworks", genres: "28,36", media_type: MediaType::Movie, extra: Some("&with_keywords=351|173163&vote_count.gte=1000") },
    MicroGenreEntry { name: "Sunday Morning Cozy Cartoons", genres: "16,10751", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&with_runtime.lte=30") },
    MicroGenreEntry { name: "Monday Coffee & Brain Food", genres: "99", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=500") },
    MicroGenreEntry { name: "Midweek Office Sitcom Laughs", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=1701&vote_count.gte=1000") },
    MicroGenreEntry { name: "Late Night High-Octane Thrills", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=110&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Evening Escape: Epic Fantasy", genres: "12,14", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=130&vote_count.gte=5000") },
    MicroGenreEntry { name: "After-School Teen Drama", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_keywords=4565&vote_average.gte=7.2") },
    MicroGenreEntry { name: "Midnight Mind-Benders", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Rainy Day Comfort Blanket", genres: "35,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.2&vote_count.gte=2500") },
    MicroGenreEntry { name: "Lunch Hour Quick Bites", genres: "35", media_type: MediaType::Tv, extra: Some("&with_runtime.lte=25&vote_average.gte=7.8&vote_count.gte=1500") },
    MicroGenreEntry { name: "Friday Night Popcorn Hits", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_average.gte=6.5&vote_count.gte=15000") },
    MicroGenreEntry { name: "Saturday Night Date Night", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.0&vote_count.gte=4000") },
    MicroGenreEntry { name: "Thursday Throwback Retro", genres: "18,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=2000-01-01&vote_average.gte=7.8&vote_count.gte=5000") },
    MicroGenreEntry { name: "Sunset Road-Trips", genres: "12,35", media_type: MediaType::Movie, extra: Some("&with_keywords=11930&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Deep Sleep Ambient Space", genres: "878,14", media_type: MediaType::Movie, extra: Some("&with_keywords=3801&vote_average.gte=7.5&vote_count.gte=3000") },
    MicroGenreEntry { name: "Dawn Patrol Motivation", genres: "18,10402", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&with_keywords=6075") },
    MicroGenreEntry { name: "Aries' Adrenaline Junkies", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_keywords=1706|549&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Taurus' Slow & Savored Comforts", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&with_runtime.gte=125&vote_count.gte=2000") },
    MicroGenreEntry { name: "Gemini's Spill the Tea", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_keywords=10224&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Cancer's Emotional Safe Haven", genres: "18,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Leo's Glitz & Center Stage", genres: "10402,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "Virgo's Perfect Puzzle Boxes", genres: "9648,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=2500") },
    MicroGenreEntry { name: "Libra's Harmonious Masterpieces", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Scorpio's Erotic & Dark Dread", genres: "27,53", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.0&vote_count.gte=2000") },
    MicroGenreEntry { name: "Sagittarius' Epic Wanderlust", genres: "12,14", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.2&vote_count.gte=4000") },
    MicroGenreEntry { name: "Capricorn's Corporate Intrigues", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Aquarius' Quirky & Offbeat Worlds", genres: "878,35", media_type: MediaType::Movie, extra: Some("&with_keywords=180370&vote_average.gte=7.2&vote_count.gte=1500") },
    MicroGenreEntry { name: "Pisces' Surreal Dreamscapes", genres: "14,16", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=2000") },
    MicroGenreEntry { name: "Cyberpunk & AI Rebellion", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=180370|310&vote_average.gte=7.2&vote_count.gte=3000") },
    MicroGenreEntry { name: "The Perfect Heist Sagas", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=10214&vote_average.gte=7.4&vote_count.gte=4000") },
    MicroGenreEntry { name: "Lethal Martial Arts Spectacles", genres: "28", media_type: MediaType::Movie, extra: Some("&with_keywords=3671&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "Time-Loop Paradoxes", genres: "878,9648", media_type: MediaType::Movie, extra: Some("&with_keywords=4379|207436&vote_average.gte=7.2&vote_count.gte=2000") },
    MicroGenreEntry { name: "Cozy British Village Mysteries", genres: "9648,80", media_type: MediaType::Tv, extra: Some("&with_origin_country=GB&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Post-Apocalyptic Sand & Dust", genres: "878,28", media_type: MediaType::Movie, extra: Some("&with_keywords=4565&vote_average.gte=6.8&vote_count.gte=4000") },
    MicroGenreEntry { name: "Gothic Victorian Dread", genres: "27,18", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.0&vote_count.gte=1500") },
    MicroGenreEntry { name: "High-Seas Pirate Swashbucklers", genres: "12,28", media_type: MediaType::Movie, extra: Some("&with_keywords=2081|1801&vote_count.gte=3000") },
    MicroGenreEntry { name: "Great Prison Escapes", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=378&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Masterchef Culinary Dramas", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=159491|10461&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Underdog Sports & Glory", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=6075&vote_average.gte=7.4&vote_count.gte=2000") },
    MicroGenreEntry { name: "Assassins & Hitmen", genres: "28,80", media_type: MediaType::Movie, extra: Some("&with_keywords=10103|12317&vote_count.gte=3000") },
    MicroGenreEntry { name: "Cops, Detectives & Procedurals", genres: "80,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=1701&vote_average.gte=7.6&vote_count.gte=1500") },
    MicroGenreEntry { name: "Dark Comedy & Black Humor", genres: "35,80", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "CGI & Pixar-Style Magic", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&with_keywords=12542&vote_average.gte=7.6&vote_count.gte=4000") },
    MicroGenreEntry { name: "Anime Shonen Battles", genres: "16,28", media_type: MediaType::Tv, extra: Some("&with_keywords=210024&vote_average.gte=8.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "Period Romance & Swoons", genres: "10749,36", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "True Crime Investigative Docs", genres: "99,80", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.6&vote_count.gte=1000") },
    MicroGenreEntry { name: "Ancient Historical Epics", genres: "36,28", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.2&vote_count.gte=3000") },
    MicroGenreEntry { name: "Stand-Up Comedy Specials", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=9716&vote_average.gte=7.2&vote_count.gte=500") },
    MicroGenreEntry { name: "Reality TV Matches & Dates", genres: "10764", media_type: MediaType::Tv, extra: Some("&with_keywords=9743&vote_average.gte=6.0") },
    MicroGenreEntry { name: "Superheroes & Comic Books", genres: "28,878", media_type: MediaType::Movie, extra: Some("&with_keywords=9715&vote_count.gte=12000") },
    MicroGenreEntry { name: "Nature & Earth Wildlife", genres: "99", media_type: MediaType::Tv, extra: Some("&with_keywords=196884&vote_average.gte=8.2&vote_count.gte=1000") },
    MicroGenreEntry { name: "Survival in the Wilderness", genres: "53", media_type: MediaType::Movie, extra: Some("&with_keywords=549|9663&vote_average.gte=6.8&vote_count.gte=2000") },
    MicroGenreEntry { name: "Witchcraft & Dark Spells", genres: "14,27", media_type: MediaType::Movie, extra: Some("&with_keywords=6158&vote_average.gte=6.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "Space Operas & Galaxies", genres: "878,12", media_type: MediaType::Movie, extra: Some("&with_keywords=3801|161244&vote_average.gte=7.0&vote_count.gte=8000") },
    MicroGenreEntry { name: "Courtroom Legal Chess", genres: "18,80", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "High-School Rivalries", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=4565&vote_average.gte=6.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Haunted Houses & Mansions", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=10185&vote_average.gte=6.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Smart Hackers & Cyber Wars", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=180370&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "90s Grunge & Street Crime", genres: "80,53", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&vote_average.gte=7.4&vote_count.gte=3000") },
    MicroGenreEntry { name: "Golden Age Hollywood Glamour", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=1960-01-01&vote_average.gte=7.8&vote_count.gte=1000") },
    MicroGenreEntry { name: "70s Soul, Funk & Musicals", genres: "10402,18", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1970-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.2") },
    MicroGenreEntry { name: "80s Synthwave & Cyber Neon", genres: "878,28", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31&vote_count.gte=2500") },
    MicroGenreEntry { name: "Silent Film Pioneers", genres: "18,35", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=1930-01-01&vote_average.gte=7.6") },
    MicroGenreEntry { name: "50s B-Movie Creature Features", genres: "27,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1950-01-01&primary_release_date.lte=1959-12-31&vote_average.gte=6.0") },
    MicroGenreEntry { name: "60s Cold War Espionage", genres: "53,36", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1969-12-31&with_keywords=470") },
    MicroGenreEntry { name: "Medieval Sword & Shield Epics", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.0&vote_count.gte=4000") },
    MicroGenreEntry { name: "Peak 2000s Teen Pop Culture", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2000-01-01&primary_release_date.lte=2009-12-31&vote_count.gte=4000") },
    MicroGenreEntry { name: "Defining 2010s Blockbusters", genres: "28,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2010-01-01&primary_release_date.lte=2019-12-31&vote_count.gte=15000") },
    MicroGenreEntry { name: "Retro 80s Slacker Comedies", genres: "35", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31&vote_count.gte=2000") },
    MicroGenreEntry { name: "Post-WWII Cinema Noir", genres: "80,9648", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1945-01-01&primary_release_date.lte=1959-12-31&vote_average.gte=7.2") },
    MicroGenreEntry { name: "90s Cyber Sci-Fi Tech", genres: "878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&with_keywords=180370") },
    MicroGenreEntry { name: "Swinging 60s Musical Romance", genres: "10749,10402", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1969-12-31&vote_average.gte=7.0") },
    MicroGenreEntry { name: "New Hollywood Renaissance", genres: "18,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1967-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Victorian Era Aristocracy", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.4") },
    MicroGenreEntry { name: "Samurai Mastery: Jidaigeki", genres: "28,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=JP&vote_average.gte=7.6&vote_count.gte=500") },
    MicroGenreEntry { name: "Spaghetti Western Legends", genres: "37", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Early CGI Revolution", genres: "12,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=2005-12-31&vote_count.gte=8000") },
    MicroGenreEntry { name: "Modern Classics: 2020s Prestige", genres: "18", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2020-01-01&vote_average.gte=8.0&vote_count.gte=2000") },
    MicroGenreEntry { name: "French New Wave & Romance", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_origin_country=FR&vote_average.gte=7.2&vote_count.gte=1000") },
    MicroGenreEntry { name: "Korean Edge-of-Seat Thrillers", genres: "53,80", media_type: MediaType::Movie, extra: Some("&with_origin_country=KR&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Spanish High-Tension Crime", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_original_language=es&vote_average.gte=7.2&vote_count.gte=1500") },
    MicroGenreEntry { name: "Japanese Anime Masterpieces", genres: "16,14", media_type: MediaType::Movie, extra: Some("&with_origin_country=JP&vote_average.gte=7.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Bollywood Colors & Romance", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_origin_country=IN&vote_average.gte=7.0&vote_count.gte=500") },
    MicroGenreEntry { name: "Nordic Noir & Snowy Secrets", genres: "80,9648", media_type: MediaType::Tv, extra: Some("&with_origin_country=SE|NO|DK|FI&vote_average.gte=7.5") },
    MicroGenreEntry { name: "German Prestige & Historiography", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=DE&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Latin American Vivid Realism", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=MX|AR|BR|CO&vote_average.gte=7.4") },
    MicroGenreEntry { name: "British Wit & Dry Humor", genres: "35", media_type: MediaType::Movie, extra: Some("&with_origin_country=GB&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Italian Cinema & Neo-Realism", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=IT&vote_average.gte=7.5&vote_count.gte=800") },
];

/// Complete Micro-Genre Library (327 entries: 102 themed + 225 genre-specific).
pub static MICRO_GENRES: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Christmas Cozy Traditions", genres: "10751,35", media_type: MediaType::Movie, extra: Some("&with_keywords=9672&vote_average.gte=6.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Christmas Action & Thrills", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_keywords=9672&vote_count.gte=1000") },
    MicroGenreEntry { name: "Nightmare Before Christmas", genres: "14,16", media_type: MediaType::Movie, extra: Some("&with_keywords=9672|3335&vote_count.gte=1000") },
    MicroGenreEntry { name: "Halloween Night Terrors", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=3335&vote_count.gte=2000") },
    MicroGenreEntry { name: "Paranormal Creepshow", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=10185&vote_average.gte=6.8&vote_count.gte=1000") },
    MicroGenreEntry { name: "Gothic Vampire Chronicles", genres: "27,18", media_type: MediaType::Movie, extra: Some("&with_keywords=3133&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Valentine's Tearjerkers", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=1000&with_keywords=9799") },
    MicroGenreEntry { name: "Quirky Rom-Com Matches", genres: "10749,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=6.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Thanksgiving Feuds & Food", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=10306&vote_count.gte=200") },
    MicroGenreEntry { name: "New Year's Resolutions", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=9673|158485&vote_average.gte=6.5") },
    MicroGenreEntry { name: "St. Patrick's Emerald Isle", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=IE&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Easter Family Adventure", genres: "10751,16", media_type: MediaType::Movie, extra: Some("&with_keywords=11370&vote_average.gte=6.5") },
    MicroGenreEntry { name: "Mother's Day Maternal Bonds", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=158566|10527&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Father's Day Dad Energies", genres: "28,18", media_type: MediaType::Movie, extra: Some("&with_keywords=158565|10526&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Fourth of July Fireworks", genres: "28,36", media_type: MediaType::Movie, extra: Some("&with_keywords=351|173163&vote_count.gte=1000") },
    MicroGenreEntry { name: "Sunday Morning Cozy Cartoons", genres: "16,10751", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&with_runtime.lte=30") },
    MicroGenreEntry { name: "Monday Coffee & Brain Food", genres: "99", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=500") },
    MicroGenreEntry { name: "Midweek Office Sitcom Laughs", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=1701&vote_count.gte=1000") },
    MicroGenreEntry { name: "Late Night High-Octane Thrills", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=110&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Evening Escape: Epic Fantasy", genres: "12,14", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=130&vote_count.gte=5000") },
    MicroGenreEntry { name: "After-School Teen Drama", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_keywords=4565&vote_average.gte=7.2") },
    MicroGenreEntry { name: "Midnight Mind-Benders", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Rainy Day Comfort Blanket", genres: "35,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.2&vote_count.gte=2500") },
    MicroGenreEntry { name: "Lunch Hour Quick Bites", genres: "35", media_type: MediaType::Tv, extra: Some("&with_runtime.lte=25&vote_average.gte=7.8&vote_count.gte=1500") },
    MicroGenreEntry { name: "Friday Night Popcorn Hits", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_average.gte=6.5&vote_count.gte=15000") },
    MicroGenreEntry { name: "Saturday Night Date Night", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.0&vote_count.gte=4000") },
    MicroGenreEntry { name: "Thursday Throwback Retro", genres: "18,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=2000-01-01&vote_average.gte=7.8&vote_count.gte=5000") },
    MicroGenreEntry { name: "Sunset Road-Trips", genres: "12,35", media_type: MediaType::Movie, extra: Some("&with_keywords=11930&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Deep Sleep Ambient Space", genres: "878,14", media_type: MediaType::Movie, extra: Some("&with_keywords=3801&vote_average.gte=7.5&vote_count.gte=3000") },
    MicroGenreEntry { name: "Dawn Patrol Motivation", genres: "18,10402", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&with_keywords=6075") },
    MicroGenreEntry { name: "Aries' Adrenaline Junkies", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_keywords=1706|549&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Taurus' Slow & Savored Comforts", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&with_runtime.gte=125&vote_count.gte=2000") },
    MicroGenreEntry { name: "Gemini's Spill the Tea", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_keywords=10224&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Cancer's Emotional Safe Haven", genres: "18,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Leo's Glitz & Center Stage", genres: "10402,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "Virgo's Perfect Puzzle Boxes", genres: "9648,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=2500") },
    MicroGenreEntry { name: "Libra's Harmonious Masterpieces", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Scorpio's Erotic & Dark Dread", genres: "27,53", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.0&vote_count.gte=2000") },
    MicroGenreEntry { name: "Sagittarius' Epic Wanderlust", genres: "12,14", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.2&vote_count.gte=4000") },
    MicroGenreEntry { name: "Capricorn's Corporate Intrigues", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Aquarius' Quirky & Offbeat Worlds", genres: "878,35", media_type: MediaType::Movie, extra: Some("&with_keywords=180370&vote_average.gte=7.2&vote_count.gte=1500") },
    MicroGenreEntry { name: "Pisces' Surreal Dreamscapes", genres: "14,16", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=2000") },
    MicroGenreEntry { name: "Cyberpunk & AI Rebellion", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=180370|310&vote_average.gte=7.2&vote_count.gte=3000") },
    MicroGenreEntry { name: "The Perfect Heist Sagas", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=10214&vote_average.gte=7.4&vote_count.gte=4000") },
    MicroGenreEntry { name: "Lethal Martial Arts Spectacles", genres: "28", media_type: MediaType::Movie, extra: Some("&with_keywords=3671&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "Time-Loop Paradoxes", genres: "878,9648", media_type: MediaType::Movie, extra: Some("&with_keywords=4379|207436&vote_average.gte=7.2&vote_count.gte=2000") },
    MicroGenreEntry { name: "Cozy British Village Mysteries", genres: "9648,80", media_type: MediaType::Tv, extra: Some("&with_origin_country=GB&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Post-Apocalyptic Sand & Dust", genres: "878,28", media_type: MediaType::Movie, extra: Some("&with_keywords=4565&vote_average.gte=6.8&vote_count.gte=4000") },
    MicroGenreEntry { name: "Gothic Victorian Dread", genres: "27,18", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.0&vote_count.gte=1500") },
    MicroGenreEntry { name: "High-Seas Pirate Swashbucklers", genres: "12,28", media_type: MediaType::Movie, extra: Some("&with_keywords=2081|1801&vote_count.gte=3000") },
    MicroGenreEntry { name: "Great Prison Escapes", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=378&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Masterchef Culinary Dramas", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=159491|10461&vote_average.gte=7.0") },
    MicroGenreEntry { name: "Underdog Sports & Glory", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=6075&vote_average.gte=7.4&vote_count.gte=2000") },
    MicroGenreEntry { name: "Assassins & Hitmen", genres: "28,80", media_type: MediaType::Movie, extra: Some("&with_keywords=10103|12317&vote_count.gte=3000") },
    MicroGenreEntry { name: "Cops, Detectives & Procedurals", genres: "80,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=1701&vote_average.gte=7.6&vote_count.gte=1500") },
    MicroGenreEntry { name: "Dark Comedy & Black Humor", genres: "35,80", media_type: MediaType::Movie, extra: Some("&with_keywords=10224&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "CGI & Pixar-Style Magic", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&with_keywords=12542&vote_average.gte=7.6&vote_count.gte=4000") },
    MicroGenreEntry { name: "Anime Shonen Battles", genres: "16,28", media_type: MediaType::Tv, extra: Some("&with_keywords=210024&vote_average.gte=8.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "Period Romance & Swoons", genres: "10749,36", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "True Crime Investigative Docs", genres: "99,80", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.6&vote_count.gte=1000") },
    MicroGenreEntry { name: "Ancient Historical Epics", genres: "36,28", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.2&vote_count.gte=3000") },
    MicroGenreEntry { name: "Stand-Up Comedy Specials", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=9716&vote_average.gte=7.2&vote_count.gte=500") },
    MicroGenreEntry { name: "Reality TV Matches & Dates", genres: "10764", media_type: MediaType::Tv, extra: Some("&with_keywords=9743&vote_average.gte=6.0") },
    MicroGenreEntry { name: "Superheroes & Comic Books", genres: "28,878", media_type: MediaType::Movie, extra: Some("&with_keywords=9715&vote_count.gte=12000") },
    MicroGenreEntry { name: "Nature & Earth Wildlife", genres: "99", media_type: MediaType::Tv, extra: Some("&with_keywords=196884&vote_average.gte=8.2&vote_count.gte=1000") },
    MicroGenreEntry { name: "Survival in the Wilderness", genres: "53", media_type: MediaType::Movie, extra: Some("&with_keywords=549|9663&vote_average.gte=6.8&vote_count.gte=2000") },
    MicroGenreEntry { name: "Witchcraft & Dark Spells", genres: "14,27", media_type: MediaType::Movie, extra: Some("&with_keywords=6158&vote_average.gte=6.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "Space Operas & Galaxies", genres: "878,12", media_type: MediaType::Movie, extra: Some("&with_keywords=3801|161244&vote_average.gte=7.0&vote_count.gte=8000") },
    MicroGenreEntry { name: "Courtroom Legal Chess", genres: "18,80", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.5&vote_count.gte=1500") },
    MicroGenreEntry { name: "High-School Rivalries", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=4565&vote_average.gte=6.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Haunted Houses & Mansions", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=10185&vote_average.gte=6.5&vote_count.gte=2000") },
    MicroGenreEntry { name: "Smart Hackers & Cyber Wars", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=180370&vote_average.gte=7.0&vote_count.gte=1000") },
    MicroGenreEntry { name: "90s Grunge & Street Crime", genres: "80,53", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&vote_average.gte=7.4&vote_count.gte=3000") },
    MicroGenreEntry { name: "Golden Age Hollywood Glamour", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=1960-01-01&vote_average.gte=7.8&vote_count.gte=1000") },
    MicroGenreEntry { name: "70s Soul, Funk & Musicals", genres: "10402,18", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1970-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.2") },
    MicroGenreEntry { name: "80s Synthwave & Cyber Neon", genres: "878,28", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31&vote_count.gte=2500") },
    MicroGenreEntry { name: "Silent Film Pioneers", genres: "18,35", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=1930-01-01&vote_average.gte=7.6") },
    MicroGenreEntry { name: "50s B-Movie Creature Features", genres: "27,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1950-01-01&primary_release_date.lte=1959-12-31&vote_average.gte=6.0") },
    MicroGenreEntry { name: "60s Cold War Espionage", genres: "53,36", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1969-12-31&with_keywords=470") },
    MicroGenreEntry { name: "Medieval Sword & Shield Epics", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.0&vote_count.gte=4000") },
    MicroGenreEntry { name: "Peak 2000s Teen Pop Culture", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2000-01-01&primary_release_date.lte=2009-12-31&vote_count.gte=4000") },
    MicroGenreEntry { name: "Defining 2010s Blockbusters", genres: "28,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2010-01-01&primary_release_date.lte=2019-12-31&vote_count.gte=15000") },
    MicroGenreEntry { name: "Retro 80s Slacker Comedies", genres: "35", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31&vote_count.gte=2000") },
    MicroGenreEntry { name: "Post-WWII Cinema Noir", genres: "80,9648", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1945-01-01&primary_release_date.lte=1959-12-31&vote_average.gte=7.2") },
    MicroGenreEntry { name: "90s Cyber Sci-Fi Tech", genres: "878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&with_keywords=180370") },
    MicroGenreEntry { name: "Swinging 60s Musical Romance", genres: "10749,10402", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1969-12-31&vote_average.gte=7.0") },
    MicroGenreEntry { name: "New Hollywood Renaissance", genres: "18,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1967-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Victorian Era Aristocracy", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_keywords=5691&vote_average.gte=7.4") },
    MicroGenreEntry { name: "Samurai Mastery: Jidaigeki", genres: "28,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=JP&vote_average.gte=7.6&vote_count.gte=500") },
    MicroGenreEntry { name: "Spaghetti Western Legends", genres: "37", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1960-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Early CGI Revolution", genres: "12,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=2005-12-31&vote_count.gte=8000") },
    MicroGenreEntry { name: "Modern Classics: 2020s Prestige", genres: "18", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2020-01-01&vote_average.gte=8.0&vote_count.gte=2000") },
    MicroGenreEntry { name: "French New Wave & Romance", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_origin_country=FR&vote_average.gte=7.2&vote_count.gte=1000") },
    MicroGenreEntry { name: "Korean Edge-of-Seat Thrillers", genres: "53,80", media_type: MediaType::Movie, extra: Some("&with_origin_country=KR&vote_average.gte=7.6&vote_count.gte=2000") },
    MicroGenreEntry { name: "Spanish High-Tension Crime", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_original_language=es&vote_average.gte=7.2&vote_count.gte=1500") },
    MicroGenreEntry { name: "Japanese Anime Masterpieces", genres: "16,14", media_type: MediaType::Movie, extra: Some("&with_origin_country=JP&vote_average.gte=7.8&vote_count.gte=3000") },
    MicroGenreEntry { name: "Bollywood Colors & Romance", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_origin_country=IN&vote_average.gte=7.0&vote_count.gte=500") },
    MicroGenreEntry { name: "Nordic Noir & Snowy Secrets", genres: "80,9648", media_type: MediaType::Tv, extra: Some("&with_origin_country=SE|NO|DK|FI&vote_average.gte=7.5") },
    MicroGenreEntry { name: "German Prestige & Historiography", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_origin_country=DE&vote_average.gte=7.5&vote_count.gte=1000") },
    MicroGenreEntry { name: "Latin American Vivid Realism", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=MX|AR|BR|CO&vote_average.gte=7.4") },
    MicroGenreEntry { name: "British Wit & Dry Humor", genres: "35", media_type: MediaType::Movie, extra: Some("&with_origin_country=GB&vote_average.gte=7.0&vote_count.gte=3000") },
    MicroGenreEntry { name: "Italian Cinema & Neo-Realism", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=IT&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Hollywood's Biggest Spectacles", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=5000") },
    MicroGenreEntry { name: "High-Octane Action Thrillers", genres: "28,53", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1500&with_runtime.lte=130") },
    MicroGenreEntry { name: "Heists That Actually Worked", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=heist|bank robbery") },
    MicroGenreEntry { name: "Edge-of-Your-Seat Thrillers", genres: "53", media_type: MediaType::Movie, extra: Some("&vote_count.gte=3000") },
    MicroGenreEntry { name: "Assassins, Hitmen & Contract Killers", genres: "28,80", media_type: MediaType::Movie, extra: Some("&with_keywords=assassin|hitman") },
    MicroGenreEntry { name: "Crime Sagas Worth Every Minute", genres: "80,18", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1500&with_runtime.gte=140") },
    MicroGenreEntry { name: "Spy Films With Real Bite", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_keywords=spy|espionage") },
    MicroGenreEntry { name: "One Last Job", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=heist|robbery|one last job") },
    MicroGenreEntry { name: "Police Procedurals, Film Edition", genres: "80,18", media_type: MediaType::Movie, extra: Some("&with_keywords=detective|police investigation") },
    MicroGenreEntry { name: "Car Chases & Chaos", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_keywords=car chase|pursuit") },
    MicroGenreEntry { name: "Psychological Thrillers That Mess With You", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1500") },
    MicroGenreEntry { name: "Revenge Stories Done Right", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_keywords=revenge|vengeance") },
    MicroGenreEntry { name: "Mob Stories & Gangster Epics", genres: "80,18", media_type: MediaType::Movie, extra: Some("&with_keywords=mafia|gangster|mob") },
    MicroGenreEntry { name: "Fast-Paced Films Under 100 Minutes", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=100&vote_count.gte=800") },
    MicroGenreEntry { name: "Tense Survival Scenarios", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_keywords=survival|stranded|trapped") },
    MicroGenreEntry { name: "The Kind of Drama That Stays With You", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=800") },
    MicroGenreEntry { name: "Award Season's Best-Kept Secrets", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&vote_count.gte=250&vote_count.lte=1200") },
    MicroGenreEntry { name: "Films That Broke Your Heart (in a Good Way)", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Stories Ripped From the Headlines", genres: "18,99", media_type: MediaType::Movie, extra: Some("&with_keywords=true story|based on true events") },
    MicroGenreEntry { name: "Character Studies Worth Your Evening", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.8&with_runtime.gte=100") },
    MicroGenreEntry { name: "Films That Made Adults Cry", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_keywords=emotional|tearjerker") },
    MicroGenreEntry { name: "Underdog Stories That Deliver", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=underdog|redemption|triumph") },
    MicroGenreEntry { name: "Biopics Worth Your Time", genres: "18,36", media_type: MediaType::Movie, extra: Some("&with_keywords=biopic|biography") },
    MicroGenreEntry { name: "Social Issue Films With Real Impact", genres: "18,99", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=150") },
    MicroGenreEntry { name: "Coming-of-Age Stories That Hit Different", genres: "18", media_type: MediaType::Movie, extra: Some("&with_keywords=coming of age|growing up") },
    MicroGenreEntry { name: "Slow Burns Worth Every Minute", genres: "18,53", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=120&vote_average.gte=7.8") },
    MicroGenreEntry { name: "Films You'll Watch Twice to Catch Everything", genres: "53,18", media_type: MediaType::Movie, extra: Some("&with_keywords=twist|multiple storylines|nonlinear") },
    MicroGenreEntry { name: "Career-Best Performances", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8.2&vote_count.gte=800") },
    MicroGenreEntry { name: "Quiet Films That Hit Loud", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=150&vote_count.lte=500") },
    MicroGenreEntry { name: "Laugh-Out-Loud Comedies", genres: "35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500") },
    MicroGenreEntry { name: "The Comedy That Sneaks Up on You", genres: "35,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Late Night Comedy Picks", genres: "35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&with_runtime.lte=100") },
    MicroGenreEntry { name: "Buddy Comedies for a Lazy Afternoon", genres: "35,28", media_type: MediaType::Movie, extra: Some("&with_keywords=buddy|duo|friends") },
    MicroGenreEntry { name: "Dark Comedies That Go There", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=dark comedy|satire|black comedy") },
    MicroGenreEntry { name: "Cult Comedies You Quote for Years", genres: "35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.lte=7.5&popularity.gte=20") },
    MicroGenreEntry { name: "The Rom-Coms You Actually Want to Watch", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.2") },
    MicroGenreEntry { name: "Dinner Table Comedies", genres: "35,10751", media_type: MediaType::Movie, extra: Some("&with_keywords=family|reunion|holiday") },
    MicroGenreEntry { name: "Hollywood Comedy at Its Peak", genres: "35", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2000-01-01&primary_release_date.lte=2015-01-01&vote_count.gte=2500") },
    MicroGenreEntry { name: "Absurdist Comedies for When You Need It", genres: "35", media_type: MediaType::Movie, extra: Some("&with_keywords=absurd|surreal|quirky") },
    MicroGenreEntry { name: "Horror That Actually Scared People", genres: "27", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500") },
    MicroGenreEntry { name: "Psychological Horror Worth Losing Sleep", genres: "27,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7&vote_count.gte=500") },
    MicroGenreEntry { name: "Horror Classics From Every Decade", genres: "27", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Supernatural Horror: Demons, Ghosts & More", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=ghost|demon|supernatural") },
    MicroGenreEntry { name: "Slasher Favourites", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=slasher|killer") },
    MicroGenreEntry { name: "Horror Comedies That Work", genres: "27,35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=250") },
    MicroGenreEntry { name: "Elevated Horror: Arthouse Scares", genres: "27,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=250") },
    MicroGenreEntry { name: "Body Horror & The Grotesque", genres: "27,878", media_type: MediaType::Movie, extra: Some("&with_keywords=body horror|transformation") },
    MicroGenreEntry { name: "The Monsters You Can't Outrun", genres: "27,28", media_type: MediaType::Movie, extra: Some("&with_keywords=monster|creature") },
    MicroGenreEntry { name: "Sci-Fi That Actually Makes You Think", genres: "878", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Space Operas Worth the Runtime", genres: "878,12", media_type: MediaType::Movie, extra: Some("&with_keywords=space|galaxy|planet") },
    MicroGenreEntry { name: "Dystopian Futures That Felt Too Real", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=dystopia|post-apocalyptic") },
    MicroGenreEntry { name: "Time Travel Done Right", genres: "878,12", media_type: MediaType::Movie, extra: Some("&with_keywords=time travel") },
    MicroGenreEntry { name: "Artificial Intelligence Nightmares", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=artificial intelligence|robot|android") },
    MicroGenreEntry { name: "Cyberpunk Worlds", genres: "878", media_type: MediaType::Movie, extra: Some("&with_keywords=cyberpunk|hacker|neon") },
    MicroGenreEntry { name: "Fantasy Epics That Transported You", genres: "14,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Mythological Adventures", genres: "14,12", media_type: MediaType::Movie, extra: Some("&with_keywords=mythology|myth|legend") },
    MicroGenreEntry { name: "Superhero Films That Redefined the Genre", genres: "28,878", media_type: MediaType::Movie, extra: Some("&with_keywords=superhero|comic book&vote_count.gte=2500") },
    MicroGenreEntry { name: "Close Encounters & First Contact", genres: "878", media_type: MediaType::Movie, extra: Some("&with_keywords=alien|extraterrestrial|first contact") },
    MicroGenreEntry { name: "Mind-Bending Realities", genres: "878,53", media_type: MediaType::Movie, extra: Some("&with_keywords=mind bending|reality|simulation") },
    MicroGenreEntry { name: "French Cinema: Cool, Sharp, Undeniable", genres: "18,80", media_type: MediaType::Movie, extra: Some("&with_origin_country=FR&vote_average.gte=7") },
    MicroGenreEntry { name: "Korean Cinema Rising", genres: "18,53", media_type: MediaType::Movie, extra: Some("&with_origin_country=KR&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Spanish Language Thrillers", genres: "53,80", media_type: MediaType::Movie, extra: Some("&with_original_language=es&vote_average.gte=7") },
    MicroGenreEntry { name: "Italian Genre Classics", genres: "28,53", media_type: MediaType::Movie, extra: Some("&with_origin_country=IT&vote_average.gte=7") },
    MicroGenreEntry { name: "Japanese Genre Mastery", genres: "28,18", media_type: MediaType::Movie, extra: Some("&with_origin_country=JP&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Bollywood Goes Global", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_origin_country=IN&vote_average.gte=7") },
    MicroGenreEntry { name: "Scandinavian Slow Burns", genres: "53,18", media_type: MediaType::Movie, extra: Some("&with_origin_country=SE,NO,DK,FI&vote_average.gte=7") },
    MicroGenreEntry { name: "German Prestige Cinema", genres: "18,53", media_type: MediaType::Movie, extra: Some("&with_origin_country=DE&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Latin American Stories That Shine", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=MX,AR,CO,BR&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Asian Action Legends", genres: "28,12", media_type: MediaType::Movie, extra: Some("&with_origin_country=HK,KR,JP,CN,TH&vote_count.gte=250") },
    MicroGenreEntry { name: "Middle Eastern & North African Cinema", genres: "18", media_type: MediaType::Movie, extra: Some("&with_origin_country=IR,EG,MA,SA&vote_average.gte=7") },
    MicroGenreEntry { name: "World Cinema: The Hidden Library", genres: "18", media_type: MediaType::Movie, extra: Some("&without_original_language=en&vote_average.gte=7.8&page=2") },
    MicroGenreEntry { name: "British Films Worth Every Minute", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_origin_country=GB&vote_count.gte=500") },
    MicroGenreEntry { name: "Films That Defined the 70s", genres: "18,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1970-01-01&primary_release_date.lte=1979-12-31&vote_average.gte=7.5") },
    MicroGenreEntry { name: "80s Action Heroes You Grew Up With", genres: "28,12", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31&vote_count.gte=800") },
    MicroGenreEntry { name: "80s Sci-Fi: The Golden Age", genres: "878,12", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1980-01-01&primary_release_date.lte=1989-12-31") },
    MicroGenreEntry { name: "The Films That Built the 90s", genres: "28,18,35", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&vote_count.gte=1200") },
    MicroGenreEntry { name: "90s Thrillers You Still Quote", genres: "53,80", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=1990-01-01&primary_release_date.lte=1999-12-31&vote_count.gte=800") },
    MicroGenreEntry { name: "Peak 2000s Cinema", genres: "28,18", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2000-01-01&primary_release_date.lte=2009-12-31&vote_count.gte=2500") },
    MicroGenreEntry { name: "The 2010s Decade in Blockbusters", genres: "28,878", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2010-01-01&primary_release_date.lte=2019-12-31&vote_count.gte=4000") },
    MicroGenreEntry { name: "This Decade's Best: 2020s Hits", genres: "18,28", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2020-01-01&vote_count.gte=800") },
    MicroGenreEntry { name: "Before CG: When Stunts Were Real", genres: "28,12", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=1999-12-31&with_keywords=stunt|practical effects&vote_count.gte=800") },
    MicroGenreEntry { name: "Films Under 90 Minutes: No Filler", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=90&vote_average.gte=7&vote_count.gte=500") },
    MicroGenreEntry { name: "Films That Reward Patience", genres: "18,53", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=150&vote_average.gte=8") },
    MicroGenreEntry { name: "Friday Night Blockbusters", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=4000") },
    MicroGenreEntry { name: "Saturday Night Crowd-Pleasers", genres: "28,35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500&vote_average.gte=7") },
    MicroGenreEntry { name: "Rainy Day Comfort Films", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7&vote_count.gte=800") },
    MicroGenreEntry { name: "Date Night Picks That Actually Work", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Watch Alone at Midnight", genres: "53,27", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "The Films You Never Finished — Until Now", genres: "18,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=1200&page=3") },
    MicroGenreEntry { name: "Two-Hour Escapes From Everything", genres: "12,28", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=105&with_runtime.lte=125&vote_average.gte=7") },
    MicroGenreEntry { name: "Films With Soundtracks You'll Download", genres: "18,10402", media_type: MediaType::Movie, extra: Some("&with_keywords=music|soundtrack|musician") },
    MicroGenreEntry { name: "Animated Films the Whole Family Loves", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7&vote_count.gte=800") },
    MicroGenreEntry { name: "Animated Adventures That Hit Differently as an Adult", genres: "16,12", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=1200") },
    MicroGenreEntry { name: "Family Films That Don't Talk Down to Kids", genres: "10751,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Disney & Studio Classics (Era-Defining)", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Fantasy Adventures for the Whole Family", genres: "14,10751", media_type: MediaType::Movie, extra: Some("&vote_count.gte=500") },
    MicroGenreEntry { name: "Animated Sci-Fi That Expands Young Minds", genres: "16,878", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7") },
    MicroGenreEntry { name: "Love Stories That Actually Feel Real", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "The Rom-Com Revival", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2018-01-01&vote_count.gte=250") },
    MicroGenreEntry { name: "Classic Hollywood Romance", genres: "10749", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=2000-01-01&vote_count.gte=1200") },
    MicroGenreEntry { name: "Films That Made You Believe in Love", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Feel-Good Films for a Rough Week", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=feel good|uplifting|heartwarming") },
    MicroGenreEntry { name: "Stories of Friendship That Feel True", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=friendship|best friends|loyalty") },
    MicroGenreEntry { name: "Documentaries That Changed How People Thought", genres: "99", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=250") },
    MicroGenreEntry { name: "True Crime Docs That Gripped the World", genres: "99,80", media_type: MediaType::Movie, extra: Some("&with_keywords=true crime|murder|crime investigation") },
    MicroGenreEntry { name: "Nature Documentaries That Floor You", genres: "99", media_type: MediaType::Movie, extra: Some("&with_keywords=nature|wildlife|animals|ocean") },
    MicroGenreEntry { name: "Music Documentaries Worth Your Headphones", genres: "99,10402", media_type: MediaType::Movie, extra: Some("&with_keywords=music|band|concert") },
    MicroGenreEntry { name: "War History That Doesn't Flinch", genres: "10752,36", media_type: MediaType::Movie, extra: Some("&vote_count.gte=800") },
    MicroGenreEntry { name: "The Films That Built the 20th Century", genres: "36,18", media_type: MediaType::Movie, extra: Some("&primary_release_date.lte=2000-01-01&vote_average.gte=8") },
    MicroGenreEntry { name: "The Series That Proved TV Can Be Art", genres: "18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Slow Burns That Reward Your Patience", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
    MicroGenreEntry { name: "Prestige Drama You've Been Putting Off", genres: "18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=1200&page=2") },
    MicroGenreEntry { name: "One More Episode Situations", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800&vote_average.gte=7.8") },
    MicroGenreEntry { name: "Shows That Got Better Every Season", genres: "18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=1200") },
    MicroGenreEntry { name: "Limited Series That Hit Different", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_keywords=limited series|miniseries") },
    MicroGenreEntry { name: "Series With Endings Worth Getting To", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&status=Ended") },
    MicroGenreEntry { name: "Character Studies That Take Their Time", genres: "18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.2&vote_count.gte=250") },
    MicroGenreEntry { name: "Ensemble Casts That Deliver Every Time", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "Anthology Series Worth Starting", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_keywords=anthology") },
    MicroGenreEntry { name: "Prestige Drama Set in a Single Season", genres: "18", media_type: MediaType::Tv, extra: Some("&with_keywords=limited series&vote_average.gte=8") },
    MicroGenreEntry { name: "The Watercooler Shows of the Year", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_count.gte=2500&first_air_date.gte=2020-01-01") },
    MicroGenreEntry { name: "Crime Sagas You Can't Leave Unfinished", genres: "80,18", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "Gritty Crime Procedurals", genres: "80", media_type: MediaType::Tv, extra: Some("&with_keywords=police|investigation|detective&vote_count.gte=800") },
    MicroGenreEntry { name: "Binge-Worthy Crime Dramas", genres: "80,18", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800") },
    MicroGenreEntry { name: "True Crime Series That Gripped the Internet", genres: "80,99", media_type: MediaType::Tv, extra: Some("&with_keywords=true crime|real events") },
    MicroGenreEntry { name: "Cop Shows With Actual Depth", genres: "80,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.8&vote_count.gte=500") },
    MicroGenreEntry { name: "Legal Dramas Worth the Courtroom", genres: "18,80", media_type: MediaType::Tv, extra: Some("&with_keywords=law|lawyer|trial|courtroom") },
    MicroGenreEntry { name: "Criminal Minds: Profiler & Forensic Series", genres: "80,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=forensic|profiler|serial killer") },
    MicroGenreEntry { name: "Heist & Con Artist Series", genres: "80,53", media_type: MediaType::Tv, extra: Some("&with_keywords=heist|con artist|swindle") },
    MicroGenreEntry { name: "Organized Crime Empires", genres: "80,18", media_type: MediaType::Tv, extra: Some("&with_keywords=mafia|cartel|gang|organized crime") },
    MicroGenreEntry { name: "Suspense Thrillers Where No One Is Safe", genres: "53,9648", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Mysteries With Endings That Earned It", genres: "9648,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=250") },
    MicroGenreEntry { name: "Small Town, Big Secrets", genres: "9648,18", media_type: MediaType::Tv, extra: Some("&with_keywords=small town|secrets|mystery") },
    MicroGenreEntry { name: "Sitcoms You'll Watch Start to Finish", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "The New Generation of Sitcoms", genres: "35", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=2015-01-01&vote_count.gte=500") },
    MicroGenreEntry { name: "Wit-Filled British Comedies", genres: "35", media_type: MediaType::Tv, extra: Some("&with_origin_country=GB&vote_count.gte=250") },
    MicroGenreEntry { name: "Offbeat Comedies That Find Their Audience", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.8&vote_count.gte=150&vote_count.lte=800") },
    MicroGenreEntry { name: "Workplace Comedies That Get It Right", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=workplace|office|coworkers") },
    MicroGenreEntry { name: "Sharp Satirical Series", genres: "35,18", media_type: MediaType::Tv, extra: Some("&with_keywords=satire|political comedy|social commentary") },
    MicroGenreEntry { name: "Comedies That Sneak in the Drama", genres: "35,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
    MicroGenreEntry { name: "Quick-Bite Comedies Under 30 Minutes", genres: "35", media_type: MediaType::Tv, extra: Some("&with_runtime.lte=30&vote_count.gte=500") },
    MicroGenreEntry { name: "Comedy That Made the Critics Sit Up", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=250") },
    MicroGenreEntry { name: "Dark Comedies With Real Edge", genres: "35,18", media_type: MediaType::Tv, extra: Some("&with_keywords=dark comedy|black comedy") },
    MicroGenreEntry { name: "Cringe Comedy That Works", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=cringe comedy|awkward|embarrassment") },
    MicroGenreEntry { name: "Stand-Up Specials Worth Your Evening", genres: "35", media_type: MediaType::Tv, extra: Some("&with_keywords=stand-up comedy|comedian") },
    MicroGenreEntry { name: "Sci-Fi Series That Predicted the Future", genres: "10765,878", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=800") },
    MicroGenreEntry { name: "Epic Fantasy Worlds Worth Living In", genres: "14,10765", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800&vote_average.gte=7.8") },
    MicroGenreEntry { name: "Dystopian TV That Felt Too Real", genres: "10765,18", media_type: MediaType::Tv, extra: Some("&with_keywords=dystopia|oppressive|authoritarian") },
    MicroGenreEntry { name: "Supernatural Series That Defined the Genre", genres: "10765,27", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Thought-Provoking Sci-Fi Anthologies", genres: "10765,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=anthology|technology|future") },
    MicroGenreEntry { name: "Fantasy With Real Emotional Stakes", genres: "14,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
    MicroGenreEntry { name: "Superhero Universes Worth Starting", genres: "10759,10765", media_type: MediaType::Tv, extra: Some("&with_keywords=superhero|powers") },
    MicroGenreEntry { name: "Alien Encounters & Conspiracy Arcs", genres: "10765,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=alien|extraterrestrial|conspiracy") },
    MicroGenreEntry { name: "Horror-Adjacent Sci-Fi", genres: "10765,27", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&vote_count.gte=250") },
    MicroGenreEntry { name: "Post-Apocalyptic Survival Series", genres: "10765,18", media_type: MediaType::Tv, extra: Some("&with_keywords=post-apocalyptic|survival|apocalypse") },
    MicroGenreEntry { name: "Epic Sagas: Magic Systems & World-Building", genres: "14,12", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800&vote_average.gte=8") },
    MicroGenreEntry { name: "Time Travel & Parallel Worlds", genres: "10765,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=time travel|parallel universe|alternate reality") },
    MicroGenreEntry { name: "Horror Series That Ruined Your Sleep", genres: "27,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
    MicroGenreEntry { name: "Haunted House Series: Atmosphere Unmatched", genres: "27", media_type: MediaType::Tv, extra: Some("&with_keywords=haunted house|ghost|haunting") },
    MicroGenreEntry { name: "Horror Anthologies Worth Every Episode", genres: "27,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=anthology|horror stories") },
    MicroGenreEntry { name: "Dread, Paranoia & Psychological Scares", genres: "27,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Vampires, Werewolves & Mythic Horror", genres: "27,10765", media_type: MediaType::Tv, extra: Some("&with_keywords=vampire|werewolf|supernatural") },
    MicroGenreEntry { name: "Slow-Dread Series for Brave Binge-Watchers", genres: "27,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=150") },
    MicroGenreEntry { name: "K-Drama: Where to Begin", genres: "18,10749", media_type: MediaType::Tv, extra: Some("&with_origin_country=KR&vote_average.gte=8&vote_count.gte=150") },
    MicroGenreEntry { name: "K-Drama Thrillers That Grip and Don't Let Go", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=KR&vote_average.gte=8") },
    MicroGenreEntry { name: "Spanish Thrillers: Edge-of-Seat Television", genres: "53,80", media_type: MediaType::Tv, extra: Some("&with_original_language=es&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Scandinavian Noir: Cold, Dark, Gripping", genres: "80,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=SE,NO,DK,FI&vote_average.gte=7.5") },
    MicroGenreEntry { name: "British Dramas Worth Every Accented Minute", genres: "18", media_type: MediaType::Tv, extra: Some("&with_origin_country=GB&vote_count.gte=500&with_runtime.gte=45") },
    MicroGenreEntry { name: "French Series: Subtle, Sharp, Addictive", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=FR&vote_average.gte=7.5") },
    MicroGenreEntry { name: "German Series: Dark, Precise, Unmissable", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=DE&vote_average.gte=7.5") },
    MicroGenreEntry { name: "International Crime That Travels Well", genres: "80,53", media_type: MediaType::Tv, extra: Some("&without_origin_country=US&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Turkish Drama: Epic in Every Sense", genres: "18,10749", media_type: MediaType::Tv, extra: Some("&with_origin_country=TR&vote_average.gte=7") },
    MicroGenreEntry { name: "Israeli Series the World Discovered", genres: "18,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=IL&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Japanese Series With a Vision All Their Own", genres: "18", media_type: MediaType::Tv, extra: Some("&with_origin_country=JP&vote_average.gte=7") },
    MicroGenreEntry { name: "Passport Not Required: International Hits", genres: "18", media_type: MediaType::Tv, extra: Some("&without_origin_country=US&vote_average.gte=8&page=2") },
    MicroGenreEntry { name: "Teen Dramas With Real Emotional Intelligence", genres: "18,10765", media_type: MediaType::Tv, extra: Some("&with_keywords=teen|high school&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Young Adult Series That Adults Secretly Love", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_keywords=young adult|coming of age") },
    MicroGenreEntry { name: "High School Chaos: Series Edition", genres: "35,18", media_type: MediaType::Tv, extra: Some("&with_keywords=high school|teenagers|adolescence") },
    MicroGenreEntry { name: "UK Teen Series: Brutally Honest", genres: "18,35", media_type: MediaType::Tv, extra: Some("&with_origin_country=GB&with_keywords=teen|youth") },
    MicroGenreEntry { name: "Reality Competition Worth Your Saturday Night", genres: "10764", media_type: MediaType::Tv, extra: Some("&vote_count.gte=250") },
    MicroGenreEntry { name: "Talent Shows That Became Cultural Moments", genres: "10764,10402", media_type: MediaType::Tv, extra: Some("&with_keywords=talent show|singing competition") },
    MicroGenreEntry { name: "Dating Shows You Can't Look Away From", genres: "10764,10749", media_type: MediaType::Tv, extra: Some("&with_keywords=dating|love|romantic competition") },
    MicroGenreEntry { name: "Cooking Competitions That Actually Entertain", genres: "10764", media_type: MediaType::Tv, extra: Some("&with_keywords=cooking competition|baking|culinary") },
    MicroGenreEntry { name: "US Drama Series With Staying Power", genres: "18", media_type: MediaType::Tv, extra: Some("&with_origin_country=US&vote_count.gte=1200&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Military & Combat Series Done Properly", genres: "10759,18", media_type: MediaType::Tv, extra: Some("&with_keywords=military|war|combat|soldier") },
    MicroGenreEntry { name: "Spy Thrillers: Long-Running Tension", genres: "10759,53", media_type: MediaType::Tv, extra: Some("&with_keywords=spy|espionage|intelligence") },
    MicroGenreEntry { name: "Action-Packed Series You Fly Through", genres: "10759", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&with_runtime.lte=50") },
    MicroGenreEntry { name: "Political Power Games", genres: "10768,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "War & Politics: The Human Cost", genres: "10768,18", media_type: MediaType::Tv, extra: Some("&vote_count.gte=500&vote_average.gte=8") },
    MicroGenreEntry { name: "The 90s Series That Defined a Generation", genres: "18,35", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=1990-01-01&first_air_date.lte=1999-12-31&vote_count.gte=800") },
    MicroGenreEntry { name: "Vault: The Best of Early 2000s TV", genres: "18,35", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=2000-01-01&first_air_date.lte=2007-12-31&vote_count.gte=1200") },
    MicroGenreEntry { name: "The Golden Age of Television: 2008-2015", genres: "18,80", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=2008-01-01&first_air_date.lte=2015-12-31&vote_average.gte=8") },
    MicroGenreEntry { name: "90s Animated Favourites", genres: "16", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=1990-01-01&first_air_date.lte=1999-12-31&vote_count.gte=250") },
    MicroGenreEntry { name: "Long-Running Series With Iconic Runs", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_count.gte=2500&vote_average.gte=8") },
    MicroGenreEntry { name: "The Series That Ate Your Weekend", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_count.gte=2500&vote_average.gte=8") },
    MicroGenreEntry { name: "Comfort TV for a Rough Week", genres: "35,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Binge-List: Not Starting Unless You're Ready", genres: "18,80", media_type: MediaType::Tv, extra: Some("&vote_average.gte=9&vote_count.gte=1200") },
    MicroGenreEntry { name: "Casual Viewing That Doesn't Demand Too Much", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800&with_runtime.lte=30") },
    MicroGenreEntry { name: "Background TV That Secretly Grabs You", genres: "35,10764", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800") },
    MicroGenreEntry { name: "Sleep-Preventers: Series Banned After 10pm", genres: "53,80", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Drama That Hit the Internet Like a Freight Train", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_count.gte=4000&first_air_date.gte=2019-01-01") },
    MicroGenreEntry { name: "The Cult Favourites Everyone Finds Eventually", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.lte=1200") },
    MicroGenreEntry { name: "Boredom Busters: Variety Pack", genres: "35,28", media_type: MediaType::Tv, extra: Some("&vote_count.gte=800&page=2") },
    MicroGenreEntry { name: "The Kind of Series You Watch Twice", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=9&vote_count.gte=800") },
    MicroGenreEntry { name: "Go Back and Relax: Easy Evening Series", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&with_runtime.lte=30") },
    MicroGenreEntry { name: "Stories That Spark the Conversation", genres: "18,99", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&page=2") },
    MicroGenreEntry { name: "Shows That Demand to Be Discussed", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_count.gte=2000&vote_average.gte=8.2") },
    MicroGenreEntry { name: "Animated Series for Adults (Genuinely)", genres: "16,35", media_type: MediaType::Tv, extra: Some("&with_keywords=adult animation&vote_count.gte=800") },
    MicroGenreEntry { name: "Anime That Changed the Game", genres: "16,28", media_type: MediaType::Tv, extra: Some("&with_keywords=anime&vote_average.gte=8") },
    MicroGenreEntry { name: "Anime Drama & Emotional Storytelling", genres: "16,18", media_type: MediaType::Tv, extra: Some("&with_keywords=anime&vote_average.gte=8.5") },
    MicroGenreEntry { name: "Animated Action Series Worth Your Time", genres: "16,10759", media_type: MediaType::Tv, extra: Some("&vote_count.gte=500") },
    MicroGenreEntry { name: "Feel-Good Family Animations", genres: "16,10751", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&page=2") },
    MicroGenreEntry { name: "Medical Dramas With Actual Weight", genres: "18", media_type: MediaType::Tv, extra: Some("&with_keywords=medical|hospital|doctor&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Political Dramas That Mirror the Real World", genres: "10768,18", media_type: MediaType::Tv, extra: Some("&with_keywords=politics|government|election") },
    MicroGenreEntry { name: "Procedurals Worth Returning To Each Week", genres: "80,18", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=7.8") },
    MicroGenreEntry { name: "Hidden Gems: The Algorithm Never Shows You These", genres: "18,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=150&vote_count.lte=800") },
    MicroGenreEntry { name: "The Critics Loved These. So Will You.", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Films That Made History at the Box Office", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=6000") },
    MicroGenreEntry { name: "New Releases: Not to Sleep On", genres: "18,53", media_type: MediaType::Movie, extra: Some("&primary_release_date.gte=2024-01-01&vote_count.gte=150") },
    MicroGenreEntry { name: "Series: Not to Sleep On", genres: "18,53", media_type: MediaType::Tv, extra: Some("&first_air_date.gte=2024-01-01&vote_count.gte=150") },
    MicroGenreEntry { name: "Cult Films That Finally Found Their Audience", genres: "35,27", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=250&vote_count.lte=1200") },
    MicroGenreEntry { name: "Films You Can't Explain Why You Love", genres: "35,28", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500&page=3") },
    MicroGenreEntry { name: "Period Dramas Worth the Costume Budget", genres: "18,36", media_type: MediaType::Tv, extra: Some("&vote_count.gte=500&vote_average.gte=8") },
    MicroGenreEntry { name: "Period Romances: Swoon-Worthy Drama", genres: "18,10749", media_type: MediaType::Tv, extra: Some("&with_keywords=historical romance|period drama") },
    MicroGenreEntry { name: "Stories That Expand How You See the World", genres: "18,99", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=250") },
];

/// Day-of-the-week themed row entry with mixture syntax.
pub struct DayStreamEntry {
    pub day: &'static str,
    pub entry: MicroGenreEntry,
}

pub static DAY_STREAMS: &[DayStreamEntry] = &[
    DayStreamEntry {
        day: "Monday",
        entry: MicroGenreEntry {
            name: "Monday Motivation|Start the Week Strong|Beat the Monday Feeling",
            genres: "18|28,12|35,18",
            media_type: MediaType::Movie,
            extra: Some("&vote_average.gte=7.5"),
        },
    },
    DayStreamEntry {
        day: "Tuesday",
        entry: MicroGenreEntry {
            name: "Tuesday Discoveries|International Breakout Hits|Films the World Loved First",
            genres: "18|18|18",
            media_type: MediaType::Movie,
            extra: Some("&without_original_language=en&vote_average.gte=7.8"),
        },
    },
    DayStreamEntry {
        day: "Wednesday",
        entry: MicroGenreEntry {
            name: "Hump Day Thrillers|Wednesday Night Tension|Mind-Bending Wednesday Watch",
            genres: "53|53,9648|878,53",
            media_type: MediaType::Movie,
            extra: Some("&vote_count.gte=1200"),
        },
    },
    DayStreamEntry {
        day: "Thursday",
        entry: MicroGenreEntry {
            name: "Throwback Thursday|Before They Were Blockbusters|90s Night: The Essential Cut",
            genres: "18,35|28,12|18,35",
            media_type: MediaType::Movie,
            extra: Some("&primary_release_date.lte=2000-01-01&vote_count.gte=1200"),
        },
    },
    DayStreamEntry {
        day: "Friday",
        entry: MicroGenreEntry {
            name: "Friday Night Hits|Start the Weekend Right|Big Friday Energy",
            genres: "28,12|28,12,35|28,12",
            media_type: MediaType::Movie,
            extra: Some("&vote_count.gte=2500"),
        },
    },
    DayStreamEntry {
        day: "Saturday",
        entry: MicroGenreEntry {
            name: "Saturday Night Spectacles|Family Film Night|Epic Saturday Watch",
            genres: "28,12,10751|16,10751|28,14,878",
            media_type: MediaType::Movie,
            extra: Some("&vote_count.gte=1200"),
        },
    },
    DayStreamEntry {
        day: "Sunday",
        entry: MicroGenreEntry {
            name: "Sunday Prestige Binge|The Long Sunday Series|Sunday Night Drama",
            genres: "18,80|18,53|18",
            media_type: MediaType::Tv,
            extra: Some("&vote_count.gte=1200&vote_average.gte=8"),
        },
    },
];

pub fn get_day_stream(day: &str) -> Option<&'static MicroGenreEntry> {
    match day {
        "Monday" => Some(&DAY_STREAMS[0].entry),
        "Tuesday" => Some(&DAY_STREAMS[1].entry),
        "Wednesday" => Some(&DAY_STREAMS[2].entry),
        "Thursday" => Some(&DAY_STREAMS[3].entry),
        "Friday" => Some(&DAY_STREAMS[4].entry),
        "Saturday" => Some(&DAY_STREAMS[5].entry),
        "Sunday" => Some(&DAY_STREAMS[6].entry),
        _ => None,
    }
}

/// Time-of-day stream grouping.
pub struct TimeStreamGroup {
    pub time_slot: &'static str,
    pub entries: &'static [MicroGenreEntry],
}
pub static MORNING_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Feel-Good Starts: Morning Pick-Me-Up Films", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=uplifting|feel good&vote_average.gte=7") },
    MicroGenreEntry { name: "Animated Mornings for the Whole Household", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Light Documentaries Over Your First Coffee", genres: "99", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=90&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Inspiring True Stories to Kick Off the Day", genres: "18,99", media_type: MediaType::Movie, extra: Some("&with_keywords=true story|inspiring") },
    MicroGenreEntry { name: "Easy Morning Viewing: Nature Docs", genres: "99", media_type: MediaType::Movie, extra: Some("&with_keywords=nature|wildlife") },
    MicroGenreEntry { name: "Short & Sharp Morning Comedies", genres: "35", media_type: MediaType::Tv, extra: Some("&with_runtime.lte=30&vote_count.gte=800") },
];

pub static AFTERNOON_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Afternoon Blockbusters: No Commitment Needed", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500") },
    MicroGenreEntry { name: "Adventure Films for a Slow Afternoon", genres: "12,14", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Family Matinee Picks", genres: "10751,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7&vote_count.gte=800") },
    MicroGenreEntry { name: "Crowd-Pleasing Comedies: Afternoon Edition", genres: "35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500") },
    MicroGenreEntry { name: "Sports Dramas & Underdog Stories", genres: "18", media_type: MediaType::Movie, extra: Some("&with_keywords=sport|football|basketball|boxing") },
    MicroGenreEntry { name: "Afternoon Sci-Fi: Big Ideas, Easy Watch", genres: "878,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
];

pub static EVENING_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Tonight's Prestige Pick", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=1200") },
    MicroGenreEntry { name: "Evening Thrillers Worth Silencing Your Phone For", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Tonight's Premium Drama", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Perfect Evening: Crime Drama Picks", genres: "80,18", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "Sophisticated Evening Cinema", genres: "18", media_type: MediaType::Movie, extra: Some("&without_original_language=en&vote_average.gte=8") },
    MicroGenreEntry { name: "International Cinema for Your Evening", genres: "18,53", media_type: MediaType::Movie, extra: Some("&without_original_language=en&vote_average.gte=7.5") },
];

pub static LATE_NIGHT_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Late Night Thrillers: Just One More", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Psychological Horror for the Dark Hours", genres: "27,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Mind-Bending Late Night Sci-Fi", genres: "878,9648", media_type: MediaType::Movie, extra: Some("&vote_count.gte=800&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Dark Crime Sagas: After Midnight", genres: "80,18", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "The Series That Stole Your Sleep Last Time", genres: "53,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=9&vote_count.gte=800") },
    MicroGenreEntry { name: "Neo-Noir: Late Night City Thrillers", genres: "80,53", media_type: MediaType::Movie, extra: Some("&with_keywords=neo-noir|noir|detective") },
];

pub static NIGHT_OWL_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Night Owl Comfort: Long-Form TV to Fall Asleep To", genres: "35,18", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=1200") },
    MicroGenreEntry { name: "Quiet Late-Night Documentaries", genres: "99", media_type: MediaType::Movie, extra: Some("&with_runtime.lte=100&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Gentle Comedy Reruns for 3am", genres: "35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&with_runtime.lte=30") },
    MicroGenreEntry { name: "Ambient Cinema: Visually Stunning Films", genres: "14,878", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
];

pub static TIME_STREAMS: &[TimeStreamGroup] = &[
    TimeStreamGroup { time_slot: "morning", entries: MORNING_STREAMS },
    TimeStreamGroup { time_slot: "afternoon", entries: AFTERNOON_STREAMS },
    TimeStreamGroup { time_slot: "evening", entries: EVENING_STREAMS },
    TimeStreamGroup { time_slot: "late_night", entries: LATE_NIGHT_STREAMS },
    TimeStreamGroup { time_slot: "night_owl", entries: NIGHT_OWL_STREAMS },
];

pub fn get_time_streams(slot: &str) -> &'static [MicroGenreEntry] {
    match slot {
        "morning" => MORNING_STREAMS,
        "afternoon" => AFTERNOON_STREAMS,
        "evening" => EVENING_STREAMS,
        "late_night" => LATE_NIGHT_STREAMS,
        "night_owl" => NIGHT_OWL_STREAMS,
        _ => &[],
    }
}

/// Seasonal stream grouping.
pub struct SeasonStreamGroup {
    pub season: &'static str,
    pub entries: &'static [MicroGenreEntry],
}
pub static SPRING_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Spring Awakening: Feel-Good Films", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7&vote_count.gte=800") },
    MicroGenreEntry { name: "New Beginnings: Stories of Fresh Starts", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=new start|new life|fresh start") },
    MicroGenreEntry { name: "Light & Breezy Comedies for Longer Days", genres: "35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&with_runtime.lte=100") },
    MicroGenreEntry { name: "Romance Blooms: Spring Love Stories", genres: "10749,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Spring Series: Light Drama for Lighter Nights", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
];

pub static SUMMER_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Summer Blockbusters: The Essential Cut", genres: "28,12", media_type: MediaType::Movie, extra: Some("&vote_count.gte=4000") },
    MicroGenreEntry { name: "Beach Day Comedies & Holiday Films", genres: "35", media_type: MediaType::Movie, extra: Some("&with_keywords=beach|holiday|vacation|summer") },
    MicroGenreEntry { name: "Sun-Soaked Adventure Films", genres: "12,28", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Summer Family Films for the Long Evenings", genres: "10751,16", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7") },
    MicroGenreEntry { name: "Road Trip Films: Hit Play and Drive", genres: "35,12", media_type: MediaType::Movie, extra: Some("&with_keywords=road trip") },
    MicroGenreEntry { name: "Binge Before Autumn: The Summer Watchlist", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
];

pub static AUTUMN_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Autumn Evenings: Cosy Mystery Series", genres: "9648,80", media_type: MediaType::Tv, extra: Some("&vote_average.gte=7.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Dark Drama for the Darker Nights", genres: "18,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=800") },
    MicroGenreEntry { name: "Gothic & Atmospheric Series", genres: "27,18", media_type: MediaType::Tv, extra: Some("&with_keywords=gothic|atmospheric|moody") },
    MicroGenreEntry { name: "Prestige Autumn Cinema", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8.5&vote_count.gte=500") },
    MicroGenreEntry { name: "Cold-Night Thrillers: Wrap Up Tight", genres: "53,9648", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Scandinavian Noir: Perfect Autumn Viewing", genres: "80,53", media_type: MediaType::Tv, extra: Some("&with_origin_country=SE,NO,DK,FI") },
];

pub static WINTER_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Cosy Winter Nights: Comfort Drama", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=800") },
    MicroGenreEntry { name: "Epic Winter Binges: Saga Series", genres: "14,10765", media_type: MediaType::Tv, extra: Some("&vote_count.gte=1200&vote_average.gte=8") },
    MicroGenreEntry { name: "Long, Cold Nights: Long, Great Films", genres: "18,28", media_type: MediaType::Movie, extra: Some("&with_runtime.gte=140&vote_average.gte=8") },
    MicroGenreEntry { name: "Winter Family Films: All Together Now", genres: "10751,16", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Dark & Gripping: What Winters Were Made For", genres: "80,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Fireplace Films: Slow, Warm, Meaningful", genres: "18", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8.5&vote_count.gte=250") },
];

pub static SEASON_STREAMS: &[SeasonStreamGroup] = &[
    SeasonStreamGroup { season: "spring", entries: SPRING_STREAMS },
    SeasonStreamGroup { season: "summer", entries: SUMMER_STREAMS },
    SeasonStreamGroup { season: "autumn", entries: AUTUMN_STREAMS },
    SeasonStreamGroup { season: "winter", entries: WINTER_STREAMS },
];

pub fn get_season_streams(season: &str) -> &'static [MicroGenreEntry] {
    match season {
        "spring" => SPRING_STREAMS,
        "summer" => SUMMER_STREAMS,
        "autumn" => AUTUMN_STREAMS,
        "winter" => WINTER_STREAMS,
        _ => &[],
    }
}

/// Holiday stream grouping.
pub struct HolidayStreamGroup {
    pub holiday: &'static str,
    pub entries: &'static [MicroGenreEntry],
}
pub static CHRISTMAS_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Christmas Films That Became Family Tradition", genres: "10751,35", media_type: MediaType::Movie, extra: Some("&with_keywords=christmas|xmas|holiday season") },
    MicroGenreEntry { name: "Festive Feel-Good Films for Christmas Eve", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&with_keywords=christmas|festive|holiday") },
    MicroGenreEntry { name: "Christmas Drama: The Emotional Ones", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&with_keywords=christmas|holiday|winter") },
    MicroGenreEntry { name: "Animated Christmas Classics", genres: "16,10751", media_type: MediaType::Movie, extra: Some("&with_keywords=christmas|santa|holiday") },
    MicroGenreEntry { name: "Christmas Series: Cosy and Warming", genres: "35,18", media_type: MediaType::Tv, extra: Some("&with_keywords=christmas|festive") },
];

pub static HALLOWEEN_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Halloween Night Horrors: The Full Menu", genres: "27", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Psychological Terror for the Spooky Season", genres: "27,53", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5") },
    MicroGenreEntry { name: "Halloween Horror Anthologies", genres: "27,9648", media_type: MediaType::Tv, extra: Some("&with_keywords=anthology|horror stories") },
    MicroGenreEntry { name: "Haunted House Films: Best in Class", genres: "27", media_type: MediaType::Movie, extra: Some("&with_keywords=haunted house|ghost|haunting") },
    MicroGenreEntry { name: "Horror Comedies: Scary Enough, Still Fun", genres: "27,35", media_type: MediaType::Movie, extra: Some("&vote_count.gte=250") },
    MicroGenreEntry { name: "Creepy Series to Watch With the Lights Off", genres: "27,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8") },
];

pub static VALENTINES_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Valentine's Night: The Films That Deliver", genres: "10749,35", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Love Stories That Stand the Test of Time", genres: "10749,18", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200&vote_average.gte=7.8") },
    MicroGenreEntry { name: "Date Night Picks: Crowd-Tested & Approved", genres: "35,10749", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Romantic Dramas That Actually Have Depth", genres: "18,10749", media_type: MediaType::Movie, extra: Some("&vote_average.gte=8&vote_count.gte=500") },
    MicroGenreEntry { name: "Romance Series Worth Starting Together", genres: "18,10749", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=250") },
];

pub static NEW_YEAR_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "New Year, New Series: The Watchlist Starters", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8.5&vote_count.gte=800") },
    MicroGenreEntry { name: "Reset Button Films: New Beginnings", genres: "18,35", media_type: MediaType::Movie, extra: Some("&with_keywords=new start|new year|resolution") },
    MicroGenreEntry { name: "New Year's Eve Crowd-Pleasers", genres: "35,28", media_type: MediaType::Movie, extra: Some("&vote_count.gte=2500") },
    MicroGenreEntry { name: "Goal-Setting: Inspiring True Stories", genres: "18,99", media_type: MediaType::Movie, extra: Some("&with_keywords=true story|inspiring|triumph") },
];

pub static EASTER_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "Easter Weekend Family Films", genres: "10751,16", media_type: MediaType::Movie, extra: Some("&vote_average.gte=7") },
    MicroGenreEntry { name: "Bank Holiday Adventure Films", genres: "12,28", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Long Weekend Binge Series", genres: "18,53", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=1200") },
];

pub static SUMMER_BREAK_STREAMS: &[MicroGenreEntry] = &[
    MicroGenreEntry { name: "School's Out: Big Summer Films", genres: "28,12,10751", media_type: MediaType::Movie, extra: Some("&vote_count.gte=1200") },
    MicroGenreEntry { name: "Summer Playlist: Binge-Worthy Series", genres: "18,35", media_type: MediaType::Tv, extra: Some("&vote_average.gte=8&vote_count.gte=800") },
    MicroGenreEntry { name: "Teen Summer Favourites", genres: "35,18", media_type: MediaType::Movie, extra: Some("&with_keywords=summer|teen|high school&vote_count.gte=500") },
];

pub static HOLIDAY_STREAMS: &[HolidayStreamGroup] = &[
    HolidayStreamGroup { holiday: "christmas", entries: CHRISTMAS_STREAMS },
    HolidayStreamGroup { holiday: "halloween", entries: HALLOWEEN_STREAMS },
    HolidayStreamGroup { holiday: "valentines", entries: VALENTINES_STREAMS },
    HolidayStreamGroup { holiday: "new_year", entries: NEW_YEAR_STREAMS },
    HolidayStreamGroup { holiday: "easter", entries: EASTER_STREAMS },
    HolidayStreamGroup { holiday: "summer_break", entries: SUMMER_BREAK_STREAMS },
];

pub fn get_holiday_streams(holiday: &str) -> &'static [MicroGenreEntry] {
    match holiday {
        "christmas" => CHRISTMAS_STREAMS,
        "halloween" => HALLOWEEN_STREAMS,
        "valentines" => VALENTINES_STREAMS,
        "new_year" => NEW_YEAR_STREAMS,
        "easter" => EASTER_STREAMS,
        "summer_break" => SUMMER_BREAK_STREAMS,
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genres_count() {
        assert_eq!(GENRES.len(), 27);
        assert_eq!(get_genre_name(28), Some("Action"));
        assert_eq!(get_genre_name(878), Some("Science Fiction"));
        assert_eq!(get_genre_name(9999), None);
    }

    #[test]
    fn test_adjacent_genres() {
        assert_eq!(ADJACENT_GENRES.len(), 23);
        assert_eq!(get_adjacent_genres(28), &[12, 53, 878, 10759, 10752]);
        assert_eq!(get_adjacent_genres(9999), &[] as &[u32]);
    }


    #[test]
    fn test_micro_genres_count() {
        assert_eq!(ALL_THEMED_STREAMS.len(), 102);
        assert_eq!(MICRO_GENRES.len(), 327);
    }

    #[test]
    fn test_stream_accessors() {
        assert!(get_day_stream("Monday").is_some());
        assert_eq!(get_time_streams("morning").len(), 6);
        assert_eq!(get_season_streams("spring").len(), 5);
        assert_eq!(get_holiday_streams("christmas").len(), 5);
    }
}
