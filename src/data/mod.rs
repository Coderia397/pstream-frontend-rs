//! Static Data Dictionaries for PStream Frontend.
//!
//! Zero-cost compile-time static slices for avatars, languages, genres, page genres, and themes.

pub mod avatars;
pub mod genres;
pub mod languages;
pub mod page_genres;

// Re-exports for convenient downstream consumption
pub use avatars::{
    get_avatar_by_url, get_avatar_category, Avatar, AvatarCategory, ALL_AVATARS, AVATAR_CATEGORIES,
    DEFAULT_AVATAR,
};
pub use genres::{
    get_adjacent_genres, get_day_stream, get_genre_name, get_holiday_streams, get_season_streams,
    get_time_streams, AdjacentGenre, DayStreamEntry, Genre, HolidayStreamGroup, MediaType,
    MicroGenreEntry, SeasonStreamGroup, TimeStreamGroup, ADJACENT_GENRES, AFTERNOON_STREAMS,
    ALL_THEMED_STREAMS, AUTUMN_STREAMS, CHRISTMAS_STREAMS, DAY_STREAMS, EASTER_STREAMS,
    EVENING_STREAMS, GENRES, HALLOWEEN_STREAMS, HOLIDAY_STREAMS, LATE_NIGHT_STREAMS, MICRO_GENRES,
    MORNING_STREAMS, NEW_YEAR_STREAMS, NIGHT_OWL_STREAMS, SEASON_STREAMS, SPRING_STREAMS,
    SUMMER_BREAK_STREAMS, SUMMER_STREAMS, TIME_STREAMS, VALENTINES_STREAMS, WINTER_STREAMS,
};
pub use languages::{
    format_lang_label, get_lang_label, get_lang_to_os, get_os_to_lang, DisplayLanguage, Language,
    LanguageOption, DISPLAY_LANGUAGES, LANG_LABELS, LANG_TO_OS, SUBTITLE_LANGUAGES,
};
pub use page_genres::{
    get_home_genre_display_name, get_home_genre_mapping, get_movie_genre_name,
    is_movie_only_genre_id, is_tv_only_genre_id, resolve_genre_id, HomeGenreMapping, PageGenre,
    HOME_GENRE_ID_MAP, HOME_MOBILE_GENRES, KIDS_MOVIE_GENRES, KIDS_TV_GENRES, MOVIE_GENRES,
    MOVIE_ONLY_GENRE_IDS, TV_GENRES, TV_ONLY_GENRE_IDS, UNIVERSAL_GENRES,
};
