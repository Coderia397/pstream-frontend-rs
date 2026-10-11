//! What the settings screens save. Per profile: languages, subtitle look, autoplay, age limit, blocked titles. Per device: data use
//! and notifications. Everything is kept in this browser first; the account tables take over after the database setup is run.
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subtitle look, the same six choices Netflix offers on phones and tablets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubtitleStyle {
    /// 0 small, 1 medium, 2 large
    pub size: u8,
    pub color: String,
    pub edge: String,
    pub edge_color: String,
    pub background: String,
    pub background_clear: bool,
    pub window: String,
    pub window_clear: bool,
}

impl Default for SubtitleStyle {
    fn default() -> Self {
        Self {
            size: 1,
            color: "white".into(),
            edge: "shadow".into(),
            edge_color: "black".into(),
            background: "none".into(),
            background_clear: false,
            window: "none".into(),
            window_clear: false,
        }
    }
}

/// Settings of one profile. The field names follow the columns of public.profile_settings (autoplay_next, autoplay_previews,
/// audio_language, subtitle_language, subtitle_style, ui_language) and profiles.maturity, so syncing them is a copy, not a
/// translation. Blocked titles have no column yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfilePrefs {
    pub autoplay_next: bool,
    pub autoplay_previews: bool,
    /// "email", "push" and "<channel>.<topic>" switches; a missing key means on.
    pub notifications: HashMap<String, bool>,
    /// "auto", "low", "medium" or "high" (profile_settings.data_usage).
    pub data_usage: String,
    /// Name in Netflix style games (profile_settings.game_handle).
    pub game_handle: Option<String>,
    pub audio_language: Option<String>,
    pub subtitle_language: Option<String>,
    pub ui_language: Option<String>,
    /// Key from data::settings::MATURITY_LEVELS; None means the default for the kind of profile.
    pub maturity: Option<String>,
    pub blocked_titles: Vec<String>,
    pub subtitle_style: SubtitleStyle,
}

impl Default for ProfilePrefs {
    fn default() -> Self {
        Self { autoplay_next: true, autoplay_previews: true, notifications: HashMap::new(), data_usage: "auto".into(), game_handle: None, audio_language: None, subtitle_language: None, ui_language: None, maturity: None, blocked_titles: Vec::new(), subtitle_style: SubtitleStyle::default() }
    }
}

impl ProfilePrefs {
    /// A notification switch; anything never set counts as on.
    pub fn notify(&self, key: &str) -> bool {
        self.notifications.get(key).copied().unwrap_or(true)
    }

    /// The level in force: the saved one if it is allowed for this kind of profile, otherwise the default for it.
    pub fn maturity_for(&self, kids: bool) -> &'static str {
        use crate::data::settings::{maturity, DEFAULT_KIDS_MATURITY, DEFAULT_MATURITY};
        let default = if kids { DEFAULT_KIDS_MATURITY } else { DEFAULT_MATURITY };
        let level = maturity(self.maturity.as_deref().unwrap_or(default));
        if kids && !level.kids { maturity(DEFAULT_KIDS_MATURITY).key } else { level.key }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppPrefs {
    /// "auto", "save" or "max"
    pub data_usage: String,
    pub notifications: bool,
}

impl Default for AppPrefs {
    fn default() -> Self {
        Self { data_usage: "auto".into(), notifications: true }
    }
}

/// Switches of the whole account (account_details.test_participation and profile_transfer).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountPrefs {
    pub test_participation: bool,
    pub profile_transfer: bool,
}

impl Default for AccountPrefs {
    fn default() -> Self {
        Self { test_participation: true, profile_transfer: true }
    }
}

#[derive(Clone, Copy)]
pub struct Prefs {
    profiles: RwSignal<HashMap<String, ProfilePrefs>>,
    pub app: RwSignal<AppPrefs>,
    pub account: RwSignal<AccountPrefs>,
    /// Unix seconds until which the password check counts. Kept in memory only, a reload asks again.
    pub verified_until: RwSignal<u64>,
}

fn read<T: for<'de> Deserialize<'de>>(key: &str) -> Option<T> {
    let raw = web_sys::window()?.local_storage().ok().flatten()?.get_item(key).ok().flatten()?;
    serde_json::from_str(&raw).ok()
}

fn write<T: Serialize>(key: &str, value: &T) {
    if let (Some(storage), Ok(json)) = (web_sys::window().and_then(|w| w.local_storage().ok().flatten()), serde_json::to_string(value)) {
        let _ = storage.set_item(key, &json);
    }
}

pub fn now_secs() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

pub fn provide_prefs() {
    let prefs = Prefs {
        profiles: RwSignal::new(read(crate::keys::PROFILE_PREFS).unwrap_or_default()),
        app: RwSignal::new(read(crate::keys::APP_PREFS).unwrap_or_default()),
        account: RwSignal::new(read(crate::keys::ACCOUNT_PREFS).unwrap_or_default()),
        verified_until: RwSignal::new(0),
    };
    Effect::new(move |_| prefs.profiles.with(|p| write(crate::keys::PROFILE_PREFS, p)));
    Effect::new(move |_| prefs.app.with(|a| write(crate::keys::APP_PREFS, a)));
    Effect::new(move |_| prefs.account.with(|a| write(crate::keys::ACCOUNT_PREFS, a)));
    provide_context(prefs);
}

pub fn use_prefs() -> Prefs {
    use_context::<Prefs>().expect("Prefs not provided")
}

impl Prefs {
    pub fn profile(&self, id: &str) -> ProfilePrefs {
        self.profiles.with(|m| m.get(id).cloned().unwrap_or_default())
    }

    pub fn update_profile(&self, id: &str, change: impl FnOnce(&mut ProfilePrefs)) {
        self.profiles.update(|m| change(m.entry(id.to_string()).or_default()));
    }

    pub fn forget_profile(&self, id: &str) {
        self.profiles.update(|m| {
            m.remove(id);
        });
    }

    pub fn forget_all(&self) {
        self.profiles.set(HashMap::new());
        self.app.set(AppPrefs::default());
        self.account.set(AccountPrefs::default());
    }

    pub fn is_verified(&self) -> bool {
        self.verified_until.get_untracked() > now_secs()
    }

    pub fn mark_verified(&self) {
        self.verified_until.set(now_secs() + crate::config::RECENT_SIGN_IN_SECONDS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_saved_data_still_loads() {
        let p: ProfilePrefs = serde_json::from_str(r#"{"autoplay_next":false}"#).unwrap();
        assert!(!p.autoplay_next);
        assert!(p.autoplay_previews);
        assert_eq!(p.subtitle_style, SubtitleStyle::default());
    }

    #[test]
    fn a_kids_profile_never_gets_an_adult_level() {
        let mut p = ProfilePrefs::default();
        assert_eq!(p.maturity_for(false), "adult");
        assert_eq!(p.maturity_for(true), "older_kids");
        p.maturity = Some("adult".into());
        assert_eq!(p.maturity_for(true), "older_kids");
        p.maturity = Some("teens".into());
        assert_eq!(p.maturity_for(false), "teens");
    }
}
