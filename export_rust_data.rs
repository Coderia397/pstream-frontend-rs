use std::collections::HashMap;
use serde::Serialize;
use pstream_frontend_rs::data::*;

#[derive(Serialize)]
struct DumpAdjacentGenre {
    id: u32,
    adjacent_ids: Vec<u32>,
}

#[derive(Serialize)]
struct DumpDayStream {
    day: &'static str,
    name: &'static str,
    genres: &'static str,
    media_type: &'static str,
    extra: Option<&'static str>,
}

#[derive(Serialize)]
struct DumpMicroGenre {
    name: &'static str,
    genres: &'static str,
    media_type: &'static str,
    extra: Option<&'static str>,
}

fn convert_micro(m: &genres::MicroGenreEntry) -> DumpMicroGenre {
    DumpMicroGenre {
        name: m.name,
        genres: m.genres,
        media_type: m.media_type.as_str(),
        extra: m.extra,
    }
}

#[derive(Serialize)]
struct FullRustDump {
    // Avatars
    avatar_categories: Vec<avatars::AvatarCategory>,
    all_avatars: Vec<&'static str>,
    default_avatar: &'static str,

    // Languages
    display_languages: Vec<languages::LanguageOption>,
    lang_labels: Vec<(&'static str, &'static str)>,
    lang_to_os: Vec<(&'static str, &'static str)>,
    subtitle_languages: Vec<languages::LanguageOption>,

    // Genres
    genres: Vec<genres::Genre>,
    adjacent_genres: Vec<DumpAdjacentGenre>,
    all_themed_streams: Vec<DumpMicroGenre>,
    micro_genres: Vec<DumpMicroGenre>,
    day_streams: Vec<DumpDayStream>,
    time_streams: HashMap<String, Vec<DumpMicroGenre>>,
    season_streams: HashMap<String, Vec<DumpMicroGenre>>,
    holiday_streams: HashMap<String, Vec<DumpMicroGenre>>,

    // Page Genres
    movie_genres: Vec<page_genres::PageGenre>,
    tv_genres: Vec<page_genres::PageGenre>,
    home_genre_id_map: Vec<(u32, u32, u32)>, // source, movie, tv
    tv_only_ids: Vec<u32>,
    movie_only_ids: Vec<u32>,
    home_mobile_genres: Vec<page_genres::PageGenre>,
    universal_genres: Vec<page_genres::PageGenre>,
    kids_tv_genres: Vec<page_genres::PageGenre>,
    kids_movie_genres: Vec<page_genres::PageGenre>,
}

fn main() {
    let mut adjacent = Vec::new();
    for ag in genres::ADJACENT_GENRES {
        adjacent.push(DumpAdjacentGenre {
            id: ag.id,
            adjacent_ids: ag.adjacent_ids.to_vec(),
        });
    }

    let mut days = Vec::new();
    for d in genres::DAY_STREAMS {
        days.push(DumpDayStream {
            day: d.day,
            name: d.entry.name,
            genres: d.entry.genres,
            media_type: d.entry.media_type.as_str(),
            extra: d.entry.extra,
        });
    }

    let mut times: HashMap<String, Vec<DumpMicroGenre>> = HashMap::new();
    for slot in ["morning", "afternoon", "evening", "late_night", "night_owl"] {
        let entries = genres::get_time_streams(slot).iter().map(convert_micro).collect();
        times.insert(slot.to_string(), entries);
    }

    let mut seasons: HashMap<String, Vec<DumpMicroGenre>> = HashMap::new();
    for s in ["spring", "summer", "autumn", "winter"] {
        let entries = genres::get_season_streams(s).iter().map(convert_micro).collect();
        seasons.insert(s.to_string(), entries);
    }

    let mut holidays: HashMap<String, Vec<DumpMicroGenre>> = HashMap::new();
    for h in ["christmas", "halloween", "valentines", "new_year", "easter", "summer_break"] {
        let entries = genres::get_holiday_streams(h).iter().map(convert_micro).collect();
        holidays.insert(h.to_string(), entries);
    }

    let mut home_map = Vec::new();
    for m in page_genres::HOME_GENRE_ID_MAP {
        home_map.push((m.source_id, m.movie, m.tv));
    }

    let dump = FullRustDump {
        avatar_categories: avatars::AVATAR_CATEGORIES.to_vec(),
        all_avatars: avatars::ALL_AVATARS.to_vec(),
        default_avatar: avatars::DEFAULT_AVATAR,

        display_languages: languages::DISPLAY_LANGUAGES.to_vec(),
        lang_labels: languages::LANG_LABELS.to_vec(),
        lang_to_os: languages::LANG_TO_OS.to_vec(),
        subtitle_languages: languages::SUBTITLE_LANGUAGES.to_vec(),

        genres: genres::GENRES.to_vec(),
        adjacent_genres: adjacent,
        all_themed_streams: genres::ALL_THEMED_STREAMS.iter().map(convert_micro).collect(),
        micro_genres: genres::MICRO_GENRES.iter().map(convert_micro).collect(),
        day_streams: days,
        time_streams: times,
        season_streams: seasons,
        holiday_streams: holidays,

        movie_genres: page_genres::MOVIE_GENRES.to_vec(),
        tv_genres: page_genres::TV_GENRES.to_vec(),
        home_genre_id_map: home_map,
        tv_only_ids: page_genres::TV_ONLY_GENRE_IDS.to_vec(),
        movie_only_ids: page_genres::MOVIE_ONLY_GENRE_IDS.to_vec(),
        home_mobile_genres: page_genres::HOME_MOBILE_GENRES.to_vec(),
        universal_genres: page_genres::UNIVERSAL_GENRES.to_vec(),
        kids_tv_genres: page_genres::KIDS_TV_GENRES.to_vec(),
        kids_movie_genres: page_genres::KIDS_MOVIE_GENRES.to_vec(),
    };

    let json = serde_json::to_string_pretty(&dump).expect("Failed to serialize Rust data");
    println!("{}", json);
}
