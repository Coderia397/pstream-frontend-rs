//! Static language options, ISO mappings, and display datasets ported from `pstream-frontend/data/languages.ts`.
//! Zero-cost compile-time static slices and pattern matching for WebAssembly CSR runtime.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguageOption {
    pub code: &'static str,
    pub label: &'static str,
}

/// Type alias aligning with `PROJECT.md` interface contracts.
pub type Language = LanguageOption;
pub type DisplayLanguage = LanguageOption;

pub static DISPLAY_LANGUAGES: &[LanguageOption] = &[
    LanguageOption { code: "en-US", label: "English (US)" },
    LanguageOption { code: "en-GB", label: "English (UK)" },
    LanguageOption { code: "es-ES", label: "Español (España)" },
    LanguageOption { code: "es-MX", label: "Español (México)" },
    LanguageOption { code: "fr-FR", label: "Français" },
    LanguageOption { code: "de-DE", label: "Deutsch" },
    LanguageOption { code: "it-IT", label: "Italiano" },
    LanguageOption { code: "pt-BR", label: "Português (Brasil)" },
    LanguageOption { code: "pt-PT", label: "Português (Portugal)" },
    LanguageOption { code: "ja-JP", label: "日本語" },
    LanguageOption { code: "ko-KR", label: "한국어" },
    LanguageOption { code: "zh-CN", label: "中文 (简体)" },
    LanguageOption { code: "zh-TW", label: "中文 (繁體)" },
    LanguageOption { code: "ar-SA", label: "العربية" },
    LanguageOption { code: "hi-IN", label: "हिन्दी" },
    LanguageOption { code: "ru-RU", label: "Русский" },
    LanguageOption { code: "tr-TR", label: "Türkçe" },
    LanguageOption { code: "pl-PL", label: "Polski" },
    LanguageOption { code: "nl-NL", label: "Nederlands" },
    LanguageOption { code: "sv-SE", label: "Svenska" },
];

/// Central ISO 639-1 → Display Label map.
/// 61 entries covering standard and extended languages.
pub static LANG_LABELS: &[(&str, &str)] = &[
    ("en", "English"), ("es", "Spanish"), ("fr", "French"), ("de", "German"), ("it", "Italian"),
    ("pt", "Portuguese"), ("ru", "Russian"), ("ja", "Japanese"), ("ko", "Korean"), ("zh", "Chinese"),
    ("ar", "Arabic"), ("tr", "Turkish"), ("nl", "Dutch"), ("pl", "Polish"), ("sv", "Swedish"),
    ("da", "Danish"), ("fi", "Finnish"), ("no", "Norwegian"), ("hu", "Hungarian"), ("el", "Greek"),
    ("he", "Hebrew"), ("cs", "Czech"), ("ro", "Romanian"), ("th", "Thai"), ("vi", "Vietnamese"),
    ("id", "Indonesian"), ("uk", "Ukrainian"), ("hr", "Croatian"), ("sk", "Slovak"), ("bg", "Bulgarian"),
    ("sr", "Serbian"), ("hi", "Hindi"), ("bn", "Bengali"), ("fa", "Persian"), ("ms", "Malay"),
    ("ca", "Catalan"), ("lt", "Lithuanian"), ("lv", "Latvian"), ("et", "Estonian"), ("sl", "Slovenian"),
    ("ta", "Tamil"), ("ml", "Malayalam"), ("te", "Telugu"), ("mr", "Marathi"), ("pa", "Punjabi"),
    ("ka", "Georgian"), ("sq", "Albanian"), ("mk", "Macedonian"), ("bs", "Bosnian"), ("is", "Icelandic"),
    ("si", "Sinhala"), ("ur", "Urdu"), ("af", "Afrikaans"), ("sw", "Swahili"), ("eu", "Basque"),
    ("gl", "Galician"), ("az", "Azerbaijani"), ("kk", "Kazakh"), ("uz", "Uzbek"), ("hy", "Armenian"),
    ("be", "Belarusian"),
];

/// OpenSubtitles Legacy Language IDs (ISO 639-2/3 equivalents).
/// 32 entries.
pub static LANG_TO_OS: &[(&str, &str)] = &[
    ("en", "eng"), ("es", "spa"), ("fr", "fre"), ("de", "ger"), ("it", "ita"), ("pt", "por"),
    ("ru", "rus"), ("ja", "jpn"), ("ko", "kor"), ("zh", "chi"), ("ar", "ara"), ("tr", "tur"),
    ("nl", "dut"), ("pl", "pol"), ("sv", "swe"), ("da", "dan"), ("fi", "fin"), ("no", "nor"),
    ("hu", "hun"), ("el", "ell"), ("he", "heb"), ("cs", "cze"), ("ro", "rum"), ("th", "tha"),
    ("vi", "vie"), ("id", "ind"), ("uk", "ukr"), ("hr", "hrv"), ("sk", "slo"), ("bg", "bul"),
    ("sr", "srp"), ("hi", "hin"),
];

/// Subtitle languages pre-sorted alphabetically by English label.
/// 61 entries with zero runtime sorting overhead.
pub static SUBTITLE_LANGUAGES: &[LanguageOption] = &[
    LanguageOption { code: "af", label: "Afrikaans" },
    LanguageOption { code: "sq", label: "Albanian" },
    LanguageOption { code: "ar", label: "Arabic" },
    LanguageOption { code: "hy", label: "Armenian" },
    LanguageOption { code: "az", label: "Azerbaijani" },
    LanguageOption { code: "eu", label: "Basque" },
    LanguageOption { code: "be", label: "Belarusian" },
    LanguageOption { code: "bn", label: "Bengali" },
    LanguageOption { code: "bs", label: "Bosnian" },
    LanguageOption { code: "bg", label: "Bulgarian" },
    LanguageOption { code: "ca", label: "Catalan" },
    LanguageOption { code: "zh", label: "Chinese" },
    LanguageOption { code: "hr", label: "Croatian" },
    LanguageOption { code: "cs", label: "Czech" },
    LanguageOption { code: "da", label: "Danish" },
    LanguageOption { code: "nl", label: "Dutch" },
    LanguageOption { code: "en", label: "English" },
    LanguageOption { code: "et", label: "Estonian" },
    LanguageOption { code: "fi", label: "Finnish" },
    LanguageOption { code: "fr", label: "French" },
    LanguageOption { code: "gl", label: "Galician" },
    LanguageOption { code: "ka", label: "Georgian" },
    LanguageOption { code: "de", label: "German" },
    LanguageOption { code: "el", label: "Greek" },
    LanguageOption { code: "he", label: "Hebrew" },
    LanguageOption { code: "hi", label: "Hindi" },
    LanguageOption { code: "hu", label: "Hungarian" },
    LanguageOption { code: "is", label: "Icelandic" },
    LanguageOption { code: "id", label: "Indonesian" },
    LanguageOption { code: "it", label: "Italian" },
    LanguageOption { code: "ja", label: "Japanese" },
    LanguageOption { code: "kk", label: "Kazakh" },
    LanguageOption { code: "ko", label: "Korean" },
    LanguageOption { code: "lv", label: "Latvian" },
    LanguageOption { code: "lt", label: "Lithuanian" },
    LanguageOption { code: "mk", label: "Macedonian" },
    LanguageOption { code: "ms", label: "Malay" },
    LanguageOption { code: "ml", label: "Malayalam" },
    LanguageOption { code: "mr", label: "Marathi" },
    LanguageOption { code: "no", label: "Norwegian" },
    LanguageOption { code: "fa", label: "Persian" },
    LanguageOption { code: "pl", label: "Polish" },
    LanguageOption { code: "pt", label: "Portuguese" },
    LanguageOption { code: "pa", label: "Punjabi" },
    LanguageOption { code: "ro", label: "Romanian" },
    LanguageOption { code: "ru", label: "Russian" },
    LanguageOption { code: "sr", label: "Serbian" },
    LanguageOption { code: "si", label: "Sinhala" },
    LanguageOption { code: "sk", label: "Slovak" },
    LanguageOption { code: "sl", label: "Slovenian" },
    LanguageOption { code: "es", label: "Spanish" },
    LanguageOption { code: "sw", label: "Swahili" },
    LanguageOption { code: "sv", label: "Swedish" },
    LanguageOption { code: "ta", label: "Tamil" },
    LanguageOption { code: "te", label: "Telugu" },
    LanguageOption { code: "th", label: "Thai" },
    LanguageOption { code: "tr", label: "Turkish" },
    LanguageOption { code: "uk", label: "Ukrainian" },
    LanguageOption { code: "ur", label: "Urdu" },
    LanguageOption { code: "uz", label: "Uzbek" },
    LanguageOption { code: "vi", label: "Vietnamese" },
];

/// Fast zero-allocation lookup from ISO 639-1 code to English display name.
pub fn get_lang_label(code: &str) -> Option<&'static str> {
    match code {
        "en" => Some("English"), "es" => Some("Spanish"), "fr" => Some("French"),
        "de" => Some("German"), "it" => Some("Italian"), "pt" => Some("Portuguese"),
        "ru" => Some("Russian"), "ja" => Some("Japanese"), "ko" => Some("Korean"),
        "zh" => Some("Chinese"), "ar" => Some("Arabic"), "tr" => Some("Turkish"),
        "nl" => Some("Dutch"), "pl" => Some("Polish"), "sv" => Some("Swedish"),
        "da" => Some("Danish"), "fi" => Some("Finnish"), "no" => Some("Norwegian"),
        "hu" => Some("Hungarian"), "el" => Some("Greek"), "he" => Some("Hebrew"),
        "cs" => Some("Czech"), "ro" => Some("Romanian"), "th" => Some("Thai"),
        "vi" => Some("Vietnamese"), "id" => Some("Indonesian"), "uk" => Some("Ukrainian"),
        "hr" => Some("Croatian"), "sk" => Some("Slovak"), "bg" => Some("Bulgarian"),
        "sr" => Some("Serbian"), "hi" => Some("Hindi"), "bn" => Some("Bengali"),
        "fa" => Some("Persian"), "ms" => Some("Malay"), "ca" => Some("Catalan"),
        "lt" => Some("Lithuanian"), "lv" => Some("Latvian"), "et" => Some("Estonian"),
        "sl" => Some("Slovenian"), "ta" => Some("Tamil"), "ml" => Some("Malayalam"),
        "te" => Some("Telugu"), "mr" => Some("Marathi"), "pa" => Some("Punjabi"),
        "ka" => Some("Georgian"), "sq" => Some("Albanian"), "mk" => Some("Macedonian"),
        "bs" => Some("Bosnian"), "is" => Some("Icelandic"), "si" => Some("Sinhala"),
        "ur" => Some("Urdu"), "af" => Some("Afrikaans"), "sw" => Some("Swahili"),
        "eu" => Some("Basque"), "gl" => Some("Galician"), "az" => Some("Azerbaijani"),
        "kk" => Some("Kazakh"), "uz" => Some("Uzbek"), "hy" => Some("Armenian"),
        "be" => Some("Belarusian"),
        _ => None,
    }
}

/// Fast zero-allocation lookup from ISO 639-1 code to OpenSubtitles 3-letter code.
pub fn get_lang_to_os(code: &str) -> Option<&'static str> {
    match code {
        "en" => Some("eng"), "es" => Some("spa"), "fr" => Some("fre"),
        "de" => Some("ger"), "it" => Some("ita"), "pt" => Some("por"),
        "ru" => Some("rus"), "ja" => Some("jpn"), "ko" => Some("kor"),
        "zh" => Some("chi"), "ar" => Some("ara"), "tr" => Some("tur"),
        "nl" => Some("dut"), "pl" => Some("pol"), "sv" => Some("swe"),
        "da" => Some("dan"), "fi" => Some("fin"), "no" => Some("nor"),
        "hu" => Some("hun"), "el" => Some("ell"), "he" => Some("heb"),
        "cs" => Some("cze"), "ro" => Some("rum"), "th" => Some("tha"),
        "vi" => Some("vie"), "id" => Some("ind"), "uk" => Some("ukr"),
        "hr" => Some("hrv"), "sk" => Some("slo"), "bg" => Some("bul"),
        "sr" => Some("srp"), "hi" => Some("hin"),
        _ => None,
    }
}

/// Fast zero-allocation lookup from OpenSubtitles 3-letter code to ISO 639-1 code.
pub fn get_os_to_lang(os_code: &str) -> Option<&'static str> {
    match os_code {
        "eng" => Some("en"), "spa" => Some("es"), "fre" => Some("fr"),
        "ger" => Some("de"), "ita" => Some("it"), "por" => Some("pt"),
        "ru" => Some("ru"), "jpn" => Some("ja"), "kor" => Some("ko"),
        "chi" => Some("zh"), "ara" => Some("ar"), "tur" => Some("tr"),
        "dut" => Some("nl"), "pol" => Some("pl"), "swe" => Some("sv"),
        "dan" => Some("da"), "fin" => Some("fi"), "nor" => Some("no"),
        "hun" => Some("hu"), "ell" => Some("el"), "heb" => Some("he"),
        "cze" => Some("cs"), "rum" => Some("ro"), "tha" => Some("th"),
        "vie" => Some("vi"), "ind" => Some("id"), "ukr" => Some("uk"),
        "hrv" => Some("hr"), "slo" => Some("sk"), "bul" => Some("bg"),
        "srp" => Some("sr"), "hin" => Some("hi"),
        _ => None,
    }
}

/// Format language code into display label, falling back to uppercase code.
pub fn format_lang_label(code: &str) -> String {
    get_lang_label(code)
        .map(|s| s.to_string())
        .unwrap_or_else(|| code.to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_counts() {
        assert_eq!(DISPLAY_LANGUAGES.len(), 20);
        assert_eq!(LANG_LABELS.len(), 61);
        assert_eq!(LANG_TO_OS.len(), 32);
        assert_eq!(SUBTITLE_LANGUAGES.len(), 61);
    }

    #[test]
    fn test_subtitles_sorted() {
        for window in SUBTITLE_LANGUAGES.windows(2) {
            assert!(window[0].label <= window[1].label);
        }
        assert_eq!(SUBTITLE_LANGUAGES.first().unwrap().code, "af");
        assert_eq!(SUBTITLE_LANGUAGES.last().unwrap().code, "vi");
    }

    #[test]
    fn test_language_lookups() {
        assert_eq!(get_lang_label("en"), Some("English"));
        assert_eq!(get_lang_label("fr"), Some("French"));
        assert_eq!(get_lang_label("unknown"), None);

        assert_eq!(get_lang_to_os("en"), Some("eng"));
        assert_eq!(get_lang_to_os("ja"), Some("jpn"));
        assert_eq!(get_lang_to_os("ta"), None);

        assert_eq!(get_os_to_lang("eng"), Some("en"));
        assert_eq!(get_os_to_lang("fra"), None); // OpenSubtitles uses 'fre' for French
        assert_eq!(get_os_to_lang("fre"), Some("fr"));

        assert_eq!(format_lang_label("en"), "English");
        assert_eq!(format_lang_label("xyz"), "XYZ");
    }
}
