//! What the settings screens list: maturity levels, subtitle choices, data use choices, legal links, codec probes, licences.
//! Each entry names its translation key, so adding or removing a choice never touches a page and every label is translated.

use crate::config::routes;

/// A viewing level for a profile. The key is what the database stores in profiles.maturity.
pub struct Maturity {
    pub key: &'static str,
    pub label_key: &'static str,
    pub hint_key: &'static str,
    /// Offered on Kids profiles too.
    pub kids: bool,
}

pub const MATURITY_LEVELS: &[Maturity] = &[
    Maturity { key: "little_kids", label_key: "set-maturity-little-kids", hint_key: "set-maturity-little-kids-hint", kids: true },
    Maturity { key: "older_kids", label_key: "set-maturity-older-kids", hint_key: "set-maturity-older-kids-hint", kids: true },
    Maturity { key: "teens", label_key: "set-maturity-teens", hint_key: "set-maturity-teens-hint", kids: false },
    Maturity { key: "adult", label_key: "set-maturity-adult", hint_key: "set-maturity-adult-hint", kids: false },
];
pub const DEFAULT_MATURITY: &str = "adult";
pub const DEFAULT_KIDS_MATURITY: &str = "older_kids";

pub fn maturity(key: &str) -> &'static Maturity {
    MATURITY_LEVELS.iter().find(|m| m.key == key).unwrap_or(&MATURITY_LEVELS[MATURITY_LEVELS.len() - 1])
}

/// Subtitle colours: id (also the end of the translation key set-color-<id>) and the colour shown.
pub const SUBTITLE_COLORS: &[(&str, &str)] = &[("white", "#ffffff"), ("yellow", "#ffe600"), ("green", "#2fd45a"), ("cyan", "#22d3ee"), ("blue", "#3b82f6"), ("magenta", "#e879f9"), ("red", "#ef4444"), ("black", "#000000")];
/// Text edge styles; the key is set-edge-<id>.
pub const SUBTITLE_EDGES: &[&str] = &["none", "raised", "depressed", "uniform", "shadow"];
/// Text sizes: value, translation key and the size in pixels used by the preview.
pub const SUBTITLE_SIZES: &[(u8, &str, u32)] = &[(0, "set-sub-size-small", 16), (1, "set-sub-size-medium", 22), (2, "set-sub-size-large", 30)];

/// Data use choices: value saved and translation key.
pub const DATA_USAGE: &[(&str, &str)] = &[("auto", "set-data-auto"), ("save", "set-data-save"), ("max", "set-data-max")];

/// Data use per profile (the website's "Data usage per screen"): value saved, label and hint keys.
pub const PROFILE_DATA_USAGE: &[(&str, &str, &str)] = &[
    ("auto", "set-usage-auto", "set-usage-auto-hint"),
    ("low", "set-usage-low", "set-usage-low-hint"),
    ("medium", "set-usage-medium", "set-usage-medium-hint"),
    ("high", "set-usage-high", "set-usage-high-hint"),
];

/// What a profile can be told about, and by which channel. The saved key is "<channel>.<topic>"; the channel alone ("email",
/// "push") is its master switch. Label and hint keys are set-topic-<topic> and set-topic-<topic>-hint.
pub const NOTIFICATION_CHANNELS: &[(&str, &str, &str)] = &[("email", "set-notify-email", "set-notify-email-hint"), ("push", "set-notify-push", "set-notify-push-hint")];
pub const NOTIFICATION_TOPICS: &[(&str, &[&str])] = &[("new_arrivals", &["email", "push"]), ("recommendations", &["email", "push"]), ("reminders", &["email", "push"]), ("news", &["email"])];

/// Words a suggested game handle is put together from, with a number at the end.
pub const HANDLE_ADJECTIVES: &[&str] = &["Bold", "Swift", "Quiet", "Lucky", "Brave", "Clever", "Mellow", "Bright", "Wild", "Steady", "Cosmic", "Gentle"];
pub const HANDLE_NOUNS: &[&str] = &["Fox", "Crane", "Otter", "Falcon", "Panda", "Heron", "Lynx", "Raven", "Tiger", "Whale", "Comet", "Maple"];

/// Browsers and systems recognised in a session's user agent, first match wins: text to find and the name shown.
pub const BROWSERS: &[(&str, &str)] = &[("Edg/", "Edge"), ("OPR/", "Opera"), ("Firefox/", "Firefox"), ("Chrome/", "Chrome"), ("CriOS/", "Chrome"), ("Safari/", "Safari")];
pub const SYSTEMS: &[(&str, &str)] = &[("iPhone", "iPhone"), ("iPad", "iPad"), ("Android", "Android"), ("Windows", "Windows"), ("Mac OS X", "Mac"), ("CrOS", "ChromeOS"), ("Linux", "Linux")];

/// Legal pages listed under App settings: icon, translation key and address. An empty address opens the cookie preferences.
pub const LEGAL_LINKS: &[(&str, &str, &str)] = &[
    ("file-text", "set-app-licences", "/settings/app/licences"),
    ("file-text", "help-ui-legal-privacy", routes::PRIVACY),
    ("cookie", "cookie-link", ""),
    ("file-text", "help-ui-legal-terms", routes::TERMS),
    ("file-text", "help-ui-legal-dmca", routes::DMCA),
    ("file-text", "help-ui-legal-disclaimer", routes::DISCLAIMER),
];

/// Codecs the playback specification checks: name and the media type asked of the browser.
pub const CODEC_PROBES: &[(&str, &str)] = &[
    ("AVC (H.264)", "video/mp4; codecs=\"avc1.640028\""),
    ("HEVC (H.265)", "video/mp4; codecs=\"hvc1.1.6.L120.90\""),
    ("AV1", "video/mp4; codecs=\"av01.0.08M.08\""),
    ("VP9", "video/webm; codecs=\"vp09.00.10.08\""),
];

/// Widevine probe for the playback specification: key system, the media type and init data type asked for, and the
/// robustness levels tried from best to worst with the level name shown.
pub const DRM_KEY_SYSTEM: &str = "com.widevine.alpha";
pub const DRM_PROBE_MIME: &str = "video/mp4; codecs=\"avc1.640028\"";
pub const DRM_INIT_DATA: &str = "cenc";
pub const DRM_LEVELS: &[(&str, &str)] = &[("HW_SECURE_ALL", "L1"), ("SW_SECURE_DECODE", "L3")];
pub const DRM_NAME: &str = "Widevine";

/// Largest screen side in pixels and the name of the playback quality it supports, best first.
pub const RESOLUTION_LABELS: &[(u32, &str)] = &[(3840, "4K"), (2560, "QHD"), (1920, "Full HD"), (1280, "HD")];

/// What the "Download my data" file is called and its media type.
pub const EXPORT_MIME: &str = "application/json";
pub fn export_file_name() -> String {
    export_file_name_for("data", "json")
}

/// "<brand>-<what>.<extension>", for example streams-viewing-activity.csv.
pub fn export_file_name_for(what: &str, extension: &str) -> String {
    format!("{}-{what}.{extension}", crate::brand::NAME.to_lowercase())
}

/// Subtitle preview: corner radius, padding and window glow in pixels (the preview box only, not the player).
pub const PREVIEW_RADIUS_PX: u32 = 4;
pub const PREVIEW_PAD_Y_PX: u32 = 2;
pub const PREVIEW_PAD_X_PX: u32 = 8;
pub const PREVIEW_WINDOW_PX: u32 = 8;

/// Every package the app is built from (Rust crates, JavaScript packages and build tools) with its licence, generated by
/// scripts/make_licences.py from Cargo and npm.
pub const LICENCES_JSON: &str = include_str!("licences.json");

#[derive(serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub license: String,
    pub kind: String,
}

pub fn licences() -> Vec<Package> {
    serde_json::from_str(LICENCES_JSON).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_licence_list_is_generated_and_complete() {
        let list = licences();
        assert!(list.len() > 100, "run scripts/make_licences.py");
        assert!(list.iter().any(|p| p.name == "leptos" && p.kind == "rust"));
        assert!(list.iter().any(|p| p.name == "hls.js" && p.kind == "js"));
        assert!(list.iter().all(|p| !p.license.is_empty()));
    }

    #[test]
    fn maturity_keys_match_the_database_and_kids_levels_come_first() {
        let keys: Vec<_> = MATURITY_LEVELS.iter().map(|m| m.key).collect();
        assert_eq!(keys, ["little_kids", "older_kids", "teens", "adult"]);
        assert!(MATURITY_LEVELS.iter().filter(|m| m.kids).count() == 2);
        assert_eq!(maturity("unknown").key, DEFAULT_MATURITY);
    }
}
