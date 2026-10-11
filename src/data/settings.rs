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

/// What the "Download my data" file is called and its media type.
pub const EXPORT_MIME: &str = "application/json";
pub fn export_file_name() -> String {
    format!("{}-data.json", crate::brand::NAME.to_lowercase())
}

/// Subtitle preview: corner radius, padding and window glow in pixels (the preview box only, not the player).
pub const PREVIEW_RADIUS_PX: u32 = 4;
pub const PREVIEW_PAD_Y_PX: u32 = 2;
pub const PREVIEW_PAD_X_PX: u32 = 8;
pub const PREVIEW_WINDOW_PX: u32 = 8;

/// Software shipped in the app: name and licence. Names are proper nouns, so they are not translated.
pub const LICENCES: &[(&str, &str)] = &[
    ("Leptos", "MIT"),
    ("Tailwind CSS", "MIT"),
    ("hls.js", "Apache 2.0"),
    ("Phosphor Icons", "MIT"),
    ("Fluent (fluent-bundle)", "Apache 2.0"),
    ("wasm-bindgen and web-sys", "MIT or Apache 2.0"),
    ("serde and serde_json", "MIT or Apache 2.0"),
    ("gloo-net", "MIT or Apache 2.0"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maturity_keys_match_the_database_and_kids_levels_come_first() {
        let keys: Vec<_> = MATURITY_LEVELS.iter().map(|m| m.key).collect();
        assert_eq!(keys, ["little_kids", "older_kids", "teens", "adult"]);
        assert!(MATURITY_LEVELS.iter().filter(|m| m.kids).count() == 2);
        assert_eq!(maturity("unknown").key, DEFAULT_MATURITY);
    }
}
