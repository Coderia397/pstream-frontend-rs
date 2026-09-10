//! Standalone Empirical Stress Test Probe for Milestone 1 Static Data Modules.
//!
//! Exhaustive boundary tests, fuzzing, and invariant checks against:
//! - Languages: `get_lang_label`, `get_lang_to_os`, `get_os_to_lang`, `format_lang_label`
//! - Genres: `get_genre_name`, `get_adjacent_genres`, `resolve_genre_id`
//! - Avatars: `get_avatar_by_url`, `get_avatar_category`
//! - Themed / Micro-genre streams & page genres

use std::collections::HashSet;
use pstream_frontend_rs::data::*;

// Simple Xorshift64 PRNG for deterministic, reproducible fuzz testing
struct Xorshift64(u64);

impl Xorshift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0xdeadbeefcafebabe } else { seed })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

// -----------------------------------------------------------------------------
// 1. LANGUAGES EXHAUSTIVE AND BOUNDARY TESTS
// -----------------------------------------------------------------------------

#[test]
fn test_languages_exhaustive_valid_mappings() {
    assert_eq!(LANG_LABELS.len(), 61, "LANG_LABELS must have exactly 61 entries");
    assert_eq!(LANG_TO_OS.len(), 32, "LANG_TO_OS must have exactly 32 entries");
    assert_eq!(SUBTITLE_LANGUAGES.len(), 61, "SUBTITLE_LANGUAGES must have exactly 61 entries");
    assert_eq!(DISPLAY_LANGUAGES.len(), 20, "DISPLAY_LANGUAGES must have exactly 20 entries");

    // Check no duplicate language codes in LANG_LABELS
    let mut seen_lang_codes = HashSet::new();
    for (code, label) in LANG_LABELS {
        assert!(seen_lang_codes.insert(*code), "Duplicate language code in LANG_LABELS: {}", code);
        assert!(!label.is_empty(), "Language label cannot be empty for code: {}", code);

        // Direct lookup verification
        let lookup = get_lang_label(code);
        assert_eq!(lookup, Some(*label), "get_lang_label failed for valid code: {}", code);

        // format_lang_label must return the display label for known codes
        assert_eq!(format_lang_label(code), *label);
    }

    // Check forward mapping in LANG_TO_OS
    let mut seen_os_sources = HashSet::new();
    let mut seen_os_targets = HashSet::new();
    for (lang, os) in LANG_TO_OS {
        assert!(seen_os_sources.insert(*lang), "Duplicate source in LANG_TO_OS: {}", lang);
        assert!(seen_os_targets.insert(*os), "Duplicate target in LANG_TO_OS: {}", os);

        // Forward lookup
        assert_eq!(get_lang_to_os(lang), Some(*os), "get_lang_to_os failed for: {}", lang);
    }

    // Verify SUBTITLE_LANGUAGES order and contents
    let mut seen_subtitle_codes = HashSet::new();
    for i in 0..SUBTITLE_LANGUAGES.len() {
        let opt = &SUBTITLE_LANGUAGES[i];
        assert!(seen_subtitle_codes.insert(opt.code), "Duplicate subtitle code: {}", opt.code);
        assert_eq!(get_lang_label(opt.code), Some(opt.label));

        if i > 0 {
            assert!(
                SUBTITLE_LANGUAGES[i - 1].label <= opt.label,
                "SUBTITLE_LANGUAGES must be pre-sorted alphabetically: {} > {}",
                SUBTITLE_LANGUAGES[i - 1].label,
                opt.label
            );
        }
    }
    assert_eq!(seen_subtitle_codes.len(), 61);

    // Verify DISPLAY_LANGUAGES
    let mut seen_display_codes = HashSet::new();
    for disp in DISPLAY_LANGUAGES {
        assert!(seen_display_codes.insert(disp.code), "Duplicate code in DISPLAY_LANGUAGES: {}", disp.code);
        assert!(!disp.label.is_empty(), "Display label cannot be empty for code: {}", disp.code);
    }
}

/// Adversarial empirical probe detecting the reverse lookup bug in `get_os_to_lang`.
/// Bug description: `languages.rs:185` matches `"ru" => Some("ru")` instead of `"rus" => Some("ru")`.
/// As a consequence, `get_os_to_lang("rus")` returns `None` despite `("ru", "rus")` being in `LANG_TO_OS`.
#[test]
fn test_get_os_to_lang_bidirectional_parity_and_russian_defect() {
    let mut mismatching_os_codes = Vec::new();
    for (lang, os) in LANG_TO_OS {
        let reverse = get_os_to_lang(os);
        if reverse != Some(*lang) {
            mismatching_os_codes.push((*os, *lang, reverse));
        }
    }

    // We empirically assert that this defect exists to document it accurately:
    assert_eq!(
        mismatching_os_codes,
        vec![("rus", "ru", None)],
        "Defect detected: get_os_to_lang failed on 'rus'. Expected Some('ru'), got None."
    );

    // And verify that "ru" (an invalid 2-letter OS code) mistakenly returns Some("ru"):
    assert_eq!(
        get_os_to_lang("ru"),
        Some("ru"),
        "Defect detected: get_os_to_lang mistakenly matches 2-letter 'ru' instead of 3-letter 'rus'"
    );
}

#[test]
fn test_languages_boundary_and_edge_cases() {
    // Exact empty string
    assert_eq!(get_lang_label(""), None);
    assert_eq!(get_lang_to_os(""), None);
    assert_eq!(get_os_to_lang(""), None);
    assert_eq!(format_lang_label(""), "");

    // Single character
    assert_eq!(get_lang_label("e"), None);
    assert_eq!(get_lang_to_os("e"), None);
    assert_eq!(get_os_to_lang("e"), None);
    assert_eq!(format_lang_label("e"), "E");

    // Case sensitivity checks (all ISO codes are lowercase)
    assert_eq!(get_lang_label("EN"), None);
    assert_eq!(get_lang_label("En"), None);
    assert_eq!(get_lang_label("eN"), None);
    assert_eq!(get_lang_to_os("EN"), None);
    assert_eq!(get_os_to_lang("ENG"), None);
    assert_eq!(get_os_to_lang("Eng"), None);

    // Whitespace / special characters
    let edge_strings = [
        " ", "   ", "\t", "\n", "\r\n", "en ", " en", "en\0", "\0en",
        "!@#$%^&*()", "123", "en-US", "en_US", "fra", "und", "mul",
        "../../etc/passwd", "<script>alert(1)</script>", "русский", "日本語", "🔥"
    ];

    for s in edge_strings {
        assert_eq!(get_lang_label(s), None, "Expected None for input: {:?}", s);
        assert_eq!(get_lang_to_os(s), None, "Expected None for input: {:?}", s);
        assert_eq!(get_os_to_lang(s), None, "Expected None for input: {:?}", s);

        // format_lang_label should format safely without panic
        let formatted = format_lang_label(s);
        assert_eq!(formatted, s.to_ascii_uppercase());
    }

    // Long strings
    let large_string = "a".repeat(65536);
    assert_eq!(get_lang_label(&large_string), None);
    assert_eq!(get_lang_to_os(&large_string), None);
    assert_eq!(get_os_to_lang(&large_string), None);
    let formatted_large = format_lang_label(&large_string);
    assert_eq!(formatted_large.len(), 65536);
}

// -----------------------------------------------------------------------------
// 2. GENRES EXHAUSTIVE AND BOUNDARY TESTS
// -----------------------------------------------------------------------------

#[test]
fn test_genres_exhaustive_valid_mappings() {
    assert_eq!(GENRES.len(), 27, "GENRES must have exactly 27 entries");
    assert_eq!(ADJACENT_GENRES.len(), 23, "ADJACENT_GENRES must have exactly 23 entries");

    let mut seen_genre_ids = HashSet::new();
    for genre in GENRES {
        assert!(seen_genre_ids.insert(genre.id), "Duplicate genre ID: {}", genre.id);
        assert!(!genre.name.is_empty(), "Genre name cannot be empty for ID: {}", genre.id);

        let name = get_genre_name(genre.id);
        assert_eq!(name, Some(genre.name), "Lookup failed for genre ID: {}", genre.id);
    }

    let mut seen_adj_ids = HashSet::new();
    for adj in ADJACENT_GENRES {
        assert!(seen_adj_ids.insert(adj.id), "Duplicate ID in ADJACENT_GENRES: {}", adj.id);
        assert!(!adj.adjacent_ids.is_empty(), "Adjacent IDs list empty for: {}", adj.id);

        let queried = get_adjacent_genres(adj.id);
        assert_eq!(queried, adj.adjacent_ids, "get_adjacent_genres failed for: {}", adj.id);

        // All adjacent IDs should be legitimate genre IDs
        for target_id in adj.adjacent_ids {
            assert!(
                get_genre_name(*target_id).is_some(),
                "Adjacent genre {} references unknown target genre {}",
                adj.id,
                target_id
            );
        }
    }
}

#[test]
fn test_genres_boundary_and_edge_cases() {
    let boundary_ids: &[u32] = &[
        0,
        1,
        2,
        9,
        10,
        11, // 12 is Adventure
        13, // 14 is Fantasy
        15, // 16 is Animation
        17, // 18 is Drama
        9999,
        10000,
        10771,
        65535,
        1_000_000,
        i32::MAX as u32,
        u32::MAX - 1,
        u32::MAX,
    ];

    for &id in boundary_ids {
        assert_eq!(get_genre_name(id), None, "Expected None for unmapped ID: {}", id);
        assert_eq!(get_adjacent_genres(id), &[] as &[u32], "Expected empty slice for unmapped ID: {}", id);
    }

    // Test genres that exist in GENRES but have no adjacent mapping (e.g. 10763 News, 10766 Soap, 10767 Talk, 10770 TV Movie)
    let genres_without_adj = [10763, 10766, 10767, 10770];
    for &id in &genres_without_adj {
        assert!(get_genre_name(id).is_some(), "Genre {} should exist", id);
        assert_eq!(get_adjacent_genres(id), &[] as &[u32], "Genre {} should have no adjacent genres", id);
    }
}

// -----------------------------------------------------------------------------
// 3. RESOLVE GENRE ID BOUNDARY AND CROSS-TYPE TESTS
// -----------------------------------------------------------------------------

#[test]
fn test_resolve_genre_id_exhaustive_and_boundaries() {
    // All 10 defined mappings in HOME_GENRE_ID_MAP:
    // 12: Adventure (Movie: 12, Tv: 10759)
    assert_eq!(resolve_genre_id(MediaType::Movie, 12), 12);
    assert_eq!(resolve_genre_id(MediaType::Tv, 12), 10759);

    // 14: Fantasy (Movie: 14, Tv: 10765)
    assert_eq!(resolve_genre_id(MediaType::Movie, 14), 14);
    assert_eq!(resolve_genre_id(MediaType::Tv, 14), 10765);

    // 28: Action (Movie: 28, Tv: 10759)
    assert_eq!(resolve_genre_id(MediaType::Movie, 28), 28);
    assert_eq!(resolve_genre_id(MediaType::Tv, 28), 10759);

    // 53: Thriller (Movie: 53, Tv: 9648)
    assert_eq!(resolve_genre_id(MediaType::Movie, 53), 53);
    assert_eq!(resolve_genre_id(MediaType::Tv, 53), 9648);

    // 878: Sci-Fi (Movie: 878, Tv: 10765)
    assert_eq!(resolve_genre_id(MediaType::Movie, 878), 878);
    assert_eq!(resolve_genre_id(MediaType::Tv, 878), 10765);

    // 9648: Mystery (Movie: 53, Tv: 9648)
    assert_eq!(resolve_genre_id(MediaType::Movie, 9648), 53);
    assert_eq!(resolve_genre_id(MediaType::Tv, 9648), 9648);

    // 10751: Family (Movie: 10751, Tv: 10762)
    assert_eq!(resolve_genre_id(MediaType::Movie, 10751), 10751);
    assert_eq!(resolve_genre_id(MediaType::Tv, 10751), 10762);

    // 10759: Action & Adventure (Movie: 28, Tv: 10759)
    assert_eq!(resolve_genre_id(MediaType::Movie, 10759), 28);
    assert_eq!(resolve_genre_id(MediaType::Tv, 10759), 10759);

    // 10762: Family/Kids (Movie: 10751, Tv: 10762)
    assert_eq!(resolve_genre_id(MediaType::Movie, 10762), 10751);
    assert_eq!(resolve_genre_id(MediaType::Tv, 10762), 10762);

    // 10765: Sci-Fi & Fantasy (Movie: 878, Tv: 10765)
    assert_eq!(resolve_genre_id(MediaType::Movie, 10765), 878);
    assert_eq!(resolve_genre_id(MediaType::Tv, 10765), 10765);

    // Truly unmapped IDs must pass through completely unchanged for both Movie and Tv:
    let unmapped_ids = [0, 1, 16, 18, 27, 35, 36, 37, 80, 99, 9999, i32::MAX as u32, u32::MAX];
    for &id in &unmapped_ids {
        assert_eq!(resolve_genre_id(MediaType::Movie, id), id, "Failed for Movie ID: {}", id);
        assert_eq!(resolve_genre_id(MediaType::Tv, id), id, "Failed for Tv ID: {}", id);
    }

    // Stress loop over 0..10,000 to verify zero panics
    for id in 0..10_000 {
        let movie_res = resolve_genre_id(MediaType::Movie, id);
        let tv_res = resolve_genre_id(MediaType::Tv, id);
        assert!(movie_res > 0 || id == 0);
        assert!(tv_res > 0 || id == 0);
    }
}

// -----------------------------------------------------------------------------
// 4. AVATARS EXHAUSTIVE AND BOUNDARY TESTS
// -----------------------------------------------------------------------------

#[test]
fn test_avatars_exhaustive_and_edge_cases() {
    assert_eq!(AVATAR_CATEGORIES.len(), 5, "Must have 5 avatar categories");
    assert_eq!(ALL_AVATARS.len(), 41, "Must have 41 flattened avatars");

    let total_in_categories: usize = AVATAR_CATEGORIES.iter().map(|c| c.avatars.len()).sum();
    assert_eq!(total_in_categories, 41, "Total avatars in categories must match 41");

    // Category lookups
    let expected_cats = ["bridgerton", "one-piece", "peaky-blinders", "lucifer", "classics"];
    for cat_id in &expected_cats {
        let cat = get_avatar_category(cat_id);
        assert!(cat.is_some(), "Category lookup failed for: {}", cat_id);
        let c = cat.unwrap();
        assert_eq!(c.id, *cat_id);
        assert!(!c.name.is_empty());
        assert!(!c.avatars.is_empty());
    }

    // Every avatar in ALL_AVATARS must be found
    let mut seen_urls = HashSet::new();
    for (i, url) in ALL_AVATARS.iter().enumerate() {
        assert!(seen_urls.insert(*url), "Duplicate avatar URL in ALL_AVATARS: {}", url);
        let av = get_avatar_by_url(url);
        assert!(av.is_some(), "Avatar URL lookup failed for index {}: {}", i, url);
        let a = av.unwrap();
        assert_eq!(a.url, *url);
        assert!(!a.name.is_empty());
    }

    // Default avatar
    assert_eq!(DEFAULT_AVATAR, ALL_AVATARS[0]);
    let default_av = get_avatar_by_url(DEFAULT_AVATAR).expect("DEFAULT_AVATAR must exist");
    assert_eq!(default_av.name, "Anthony Bridgerton");

    // Boundary & negative avatar lookups
    let invalid_urls = [
        "",
        " ",
        "https://",
        "https://lh3.googleusercontent.com/d/does_not_exist",
        "http://insecure.example.com/avatar.png",
        "\0",
        "\r\n",
        "../../etc/passwd",
        "<img src=x onerror=alert(1)>",
        "https://lh3.googleusercontent.com/d/1KzRtMnyHwlJYwjr09S3UjhLtEg51W-Lr ", // trailing space
    ];
    for url in &invalid_urls {
        assert_eq!(get_avatar_by_url(url), None, "Expected None for invalid URL: {:?}", url);
    }

    // Boundary & negative category lookups
    let invalid_cats = [
        "",
        " ",
        "Bridgerton", // Case sensitive
        "BRIDGERTON",
        "one_piece",
        "classics ",
        "anime",
        "nonexistent",
        "\0",
        "../../etc/passwd",
    ];
    for cat_id in &invalid_cats {
        assert_eq!(get_avatar_category(cat_id), None, "Expected None for invalid category: {:?}", cat_id);
    }
}

// -----------------------------------------------------------------------------
// 5. THEMED STREAMS & PAGE GENRE VALIDATION
// -----------------------------------------------------------------------------

#[test]
fn test_themed_streams_and_page_genres() {
    // Days
    let valid_days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    for day in &valid_days {
        let stream = get_day_stream(day);
        assert!(stream.is_some(), "Day stream missing for: {}", day);
        let s = stream.unwrap();
        assert!(!s.name.is_empty());
        assert!(!s.genres.is_empty());
    }
    assert_eq!(get_day_stream("funday"), None);
    assert_eq!(get_day_stream("monday"), None); // case-sensitive
    assert_eq!(get_day_stream(""), None);

    // Time slots
    let time_slots = ["morning", "afternoon", "evening", "late_night", "night_owl"];
    for slot in &time_slots {
        let streams = get_time_streams(slot);
        assert!(!streams.is_empty(), "Time stream slot empty for: {}", slot);
    }
    assert!(get_time_streams("midnight").is_empty());
    assert!(get_time_streams("").is_empty());

    // Seasons
    let seasons = ["spring", "summer", "autumn", "winter"];
    for season in &seasons {
        let streams = get_season_streams(season);
        assert!(!streams.is_empty(), "Season streams empty for: {}", season);
    }
    assert!(get_season_streams("monsoon").is_empty());
    assert!(get_season_streams("").is_empty());

    // Holidays
    let holidays = ["halloween", "christmas", "new_year", "valentines", "easter", "summer_break"];
    for holiday in &holidays {
        let streams = get_holiday_streams(holiday);
        assert!(!streams.is_empty(), "Holiday streams empty for: {}", holiday);
    }
    assert!(get_holiday_streams("thanksgiving").is_empty());
    assert!(get_holiday_streams("").is_empty());

    // Micro-genres
    assert_eq!(MICRO_GENRES.len(), 327, "MICRO_GENRES must have 327 entries");
    assert_eq!(ALL_THEMED_STREAMS.len(), 102, "ALL_THEMED_STREAMS must have 102 entries");

    // Page genres & movie/tv only filters
    assert_eq!(MOVIE_GENRES.len(), 27);
    assert_eq!(TV_GENRES.len(), 25);
    assert_eq!(HOME_MOBILE_GENRES.len(), 35);
    assert_eq!(UNIVERSAL_GENRES.len(), 18);

    for &id in MOVIE_ONLY_GENRE_IDS {
        assert!(is_movie_only_genre_id(id));
        assert!(!is_tv_only_genre_id(id));
    }

    for &id in TV_ONLY_GENRE_IDS {
        assert!(is_tv_only_genre_id(id));
        assert!(!is_movie_only_genre_id(id));
    }

    assert!(!is_movie_only_genre_id(0));
    assert!(!is_movie_only_genre_id(u32::MAX));
    assert!(!is_tv_only_genre_id(0));
    assert!(!is_tv_only_genre_id(u32::MAX));
}

// -----------------------------------------------------------------------------
// 6. ADVERSARIAL PSEUDO-RANDOM FUZZ HARNESS
// -----------------------------------------------------------------------------

#[test]
fn test_adversarial_fuzz_probes() {
    let mut rng = Xorshift64::new(0x1337c0d3_deadbeef);
    const FUZZ_ITERATIONS: usize = 50_000;

    for _ in 0..FUZZ_ITERATIONS {
        // Fuzz u32 integers for genre lookups
        let rand_id = rng.next_u32();
        let _ = get_genre_name(rand_id);
        let _ = get_adjacent_genres(rand_id);
        let _ = resolve_genre_id(MediaType::Movie, rand_id);
        let _ = resolve_genre_id(MediaType::Tv, rand_id);
        let _ = is_movie_only_genre_id(rand_id);
        let _ = is_tv_only_genre_id(rand_id);

        // Fuzz strings of length 0..16
        let len = (rng.next_u64() % 17) as usize;
        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            // Mix of printable ASCII, non-ASCII UTF-8 fragments, and null bytes
            let b = (rng.next_u64() & 0xFF) as u8;
            buf.push(b);
        }

        // Test with lossless or lossy UTF-8
        if let Ok(s) = std::str::from_utf8(&buf) {
            let _ = get_lang_label(s);
            let _ = get_lang_to_os(s);
            let _ = get_os_to_lang(s);
            let _ = format_lang_label(s);
            let _ = get_avatar_by_url(s);
            let _ = get_avatar_category(s);
            let _ = get_day_stream(s);
            let _ = get_time_streams(s);
            let _ = get_season_streams(s);
            let _ = get_holiday_streams(s);
        }
    }
}
