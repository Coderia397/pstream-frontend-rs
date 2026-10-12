//! Storage keys and DOM ids shared between Rust and the TypeScript bridge. The values are frozen: existing browsers hold
//! data under these names, so renaming one needs a migration that copies the old value to the new key first.

pub const ACTIVE_PROFILE: &str = "pstream_active_profile";
pub const PROFILES: &str = "pstream_profiles";
pub const PREVIEW_MUTED: &str = "pstream_preview_muted";
pub const SESSION_ID: &str = "pstream_session_id";
pub const USER_AFFINITY: &str = "pstream_user_affinity";
pub const LANGUAGE: &str = "pstream_language";
pub const NOTIFICATIONS_READ: &str = "pstream-notifications-read:default";
pub const AMBIENT_LAST: &str = "pstream-ambient:last";
pub const SESSION: &str = "pstream_session";
pub const CONSENT: &str = "pstream_consent";
pub const CONSENT_DISMISSED: &str = "pstream_consent_dismissed";
pub const HELP_RECENT: &str = "pstream_help_recent";
pub const HELP_VIEWED: &str = "pstream_help_viewed";
pub const HELP_VOTES: &str = "pstream_help_votes";
pub const PROFILE_PREFS: &str = "pstream_profile_prefs";
pub const APP_PREFS: &str = "pstream_app_prefs";
pub const ACCOUNT_PREFS: &str = "pstream_account_prefs";
/// When the app last started in this browser (milliseconds); index.html reads it to decide whether to show the start animation.
/// Random id this browser keeps so an account can recognise it again (see services/device.rs).
pub const DEVICE_KEY: &str = "pstream_device_key";
pub const BOOT_SEEN: &str = "pstream_boot_seen";

pub const VIDEO_ELEMENT_ID: &str = "pstream-video";
pub const PLAYER_CONTAINER_ID: &str = "pstream-main-player";

pub fn preview_muted_for(profile_id: &str) -> String {
    format!("{PREVIEW_MUTED}_{profile_id}")
}
