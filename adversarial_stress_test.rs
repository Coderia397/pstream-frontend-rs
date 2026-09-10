use pstream_frontend_rs::data::*;
use pstream_frontend_rs::data::genres::MediaType;

fn main() {
    println!("=== Running Adversarial Stress Test Suite ===");

    // 1. Avatars boundary tests
    assert!(avatars::get_avatar_by_url("").is_none());
    assert!(avatars::get_avatar_by_url("https://evil.com/xss.png").is_none());
    assert!(avatars::get_avatar_by_url("http://lh3.googleusercontent.com/d/1KzRtMnyHwlJYwjr09S3UjhLtEg51W-Lr").is_none()); // http vs https
    assert!(avatars::get_avatar_category("").is_none());
    assert!(avatars::get_avatar_category("BRIDGERTON").is_none()); // case sensitivity
    assert!(avatars::get_avatar_category("bridgerton-extra").is_none());

    // 2. Languages boundary tests
    assert!(languages::get_lang_label("").is_none());
    assert!(languages::get_lang_label("EN").is_none()); // case sensitivity
    assert!(languages::get_lang_label("en-US").is_none()); // full locale vs iso-639-1
    assert_eq!(languages::format_lang_label("en"), "English");
    assert_eq!(languages::format_lang_label("xyz"), "XYZ");
    assert_eq!(languages::format_lang_label(""), "");
    assert!(languages::get_lang_to_os("").is_none());
    assert!(languages::get_lang_to_os("ta").is_none()); // Tamil has label but no OS code
    assert!(languages::get_os_to_lang("").is_none());
    assert!(languages::get_os_to_lang("fra").is_none()); // Legacy code fre vs fra
    assert_eq!(languages::get_os_to_lang("fre"), Some("fr"));

    // 3. Genres boundary tests
    assert!(genres::get_genre_name(0).is_none());
    assert!(genres::get_genre_name(u32::MAX).is_none());
    assert!(genres::get_genre_name(10000).is_none());
    assert!(genres::get_adjacent_genres(0).is_empty());
    assert!(genres::get_adjacent_genres(u32::MAX).is_empty());
    assert!(genres::get_adjacent_genres(9999).is_empty());

    // Temporal streams boundaries
    assert!(genres::get_day_stream("").is_none());
    assert!(genres::get_day_stream("monday").is_none()); // case sensitivity
    assert!(genres::get_day_stream("Funday").is_none());
    assert!(genres::get_time_streams("").is_empty());
    assert!(genres::get_time_streams("midnight").is_empty());
    assert!(genres::get_season_streams("").is_empty());
    assert!(genres::get_season_streams("monsoon").is_empty());
    assert!(genres::get_holiday_streams("").is_empty());
    assert!(genres::get_holiday_streams("diwali").is_empty());

    // 4. Page genres boundaries
    assert_eq!(page_genres::resolve_genre_id(MediaType::Movie, 0), 0);
    assert_eq!(page_genres::resolve_genre_id(MediaType::Tv, 0), 0);
    assert_eq!(page_genres::resolve_genre_id(MediaType::Movie, u32::MAX), u32::MAX);
    assert_eq!(page_genres::resolve_genre_id(MediaType::Tv, u32::MAX), u32::MAX);

    assert!(!page_genres::is_tv_only_genre_id(0));
    assert!(!page_genres::is_tv_only_genre_id(28));
    assert!(!page_genres::is_tv_only_genre_id(u32::MAX));
    assert!(!page_genres::is_movie_only_genre_id(0));
    assert!(!page_genres::is_movie_only_genre_id(10759));
    assert!(!page_genres::is_movie_only_genre_id(u32::MAX));

    assert!(page_genres::get_movie_genre_name(0).is_none());
    assert!(page_genres::get_movie_genre_name(u32::MAX).is_none());
    assert_eq!(page_genres::get_home_genre_display_name(0, None), "Content");
    assert_eq!(page_genres::get_home_genre_display_name(28, None), "Action");
    assert_eq!(page_genres::get_home_genre_display_name(28, Some("Custom Action")), "Custom Action");
    assert_eq!(page_genres::get_home_genre_display_name(0, Some("Fallback")), "Fallback");

    println!("✓ All 35 boundary and adversarial assertions passed cleanly!");
}
