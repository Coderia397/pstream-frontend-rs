//! Cookie and storage consent. Everything the app keeps in the browser is listed in the Privacy Preference Center
//! (components/consent.rs). Essential items (profiles, progress, language, this choice) are always on. Personalisation, which
//! sends the title you started and a random session number to the recommendation service, is off until the visitor accepts.
//!
//! When the banner is shown:
//!   * never once a choice is saved, until it is a year old or the policy version changes;
//!   * never when the browser sends Global Privacy Control, which is treated as a refusal;
//!   * not for a week after the visitor closed it with the cross;
//!   * not on the player, and not while the Who's watching screen is up;
//!   * only after a short pause, so it does not compete with the first paint.
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Bump when the categories or what they cover change, so everyone is asked again.
pub const POLICY_VERSION: u8 = 1;
const YEAR_MS: u64 = 365 * 24 * 60 * 60 * 1000;
const DISMISS_MS: u64 = 7 * 24 * 60 * 60 * 1000;
/// Pause before the banner appears.
pub const SHOW_DELAY_MS: i32 = 1500;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub personalization: bool,
    pub at: u64,
    #[serde(default)]
    pub v: u8,
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

/// True while a saved choice still counts: made under the current policy version and less than a year ago.
pub fn is_current(choice: &Choice, now: u64) -> bool {
    choice.v == POLICY_VERSION && now.saturating_sub(choice.at) < YEAR_MS
}

/// Pages where the banner stays away: the player.
pub fn suppressed_on(path: &str) -> bool {
    path.starts_with("/watch")
}

/// The saved choice, or None when the visitor has not decided yet or the choice is out of date.
pub fn stored() -> Option<Choice> {
    let choice: Choice = storage().and_then(|s| s.get_item(crate::keys::CONSENT).ok().flatten()).and_then(|raw| serde_json::from_str(&raw).ok())?;
    is_current(&choice, js_sys::Date::now() as u64).then_some(choice)
}

pub fn personalization_allowed() -> bool {
    stored().is_some_and(|c| c.personalization)
}

/// The browser asks sites not to sell or share data (Global Privacy Control).
fn global_privacy_control() -> bool {
    web_sys::window().map(|w| js_sys::Reflect::get(&w.navigator(), &"globalPrivacyControl".into()).ok().and_then(|v| v.as_bool()).unwrap_or(false)).unwrap_or(false)
}

fn dismissed_recently() -> bool {
    let at = storage().and_then(|s| s.get_item(crate::keys::CONSENT_DISMISSED).ok().flatten()).and_then(|v| v.parse::<u64>().ok());
    at.is_some_and(|t| (js_sys::Date::now() as u64).saturating_sub(t) < DISMISS_MS)
}

pub fn save(personalization: bool) {
    let choice = Choice { personalization, at: js_sys::Date::now() as u64, v: POLICY_VERSION };
    if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(&choice)) {
        let _ = s.set_item(crate::keys::CONSENT, &raw);
        let _ = s.remove_item(crate::keys::CONSENT_DISMISSED);
    }
    if !personalization {
        // withdrawing consent also removes what was kept for personalisation
        if let Some(s) = storage() {
            let _ = s.remove_item(crate::keys::SESSION_ID);
            let _ = s.remove_item(crate::keys::USER_AFFINITY);
        }
    }
}

/// Shared by the banner, the preference center and every "Cookie Preferences" link.
#[derive(Clone, Copy)]
pub struct ConsentUi {
    /// The banner may show (no valid choice, not dismissed lately, no privacy signal). The component adds the page rules.
    pub banner: RwSignal<bool>,
    pub center: RwSignal<bool>,
    pub personalization: RwSignal<bool>,
    /// Bumped on every save so views that show the current choice refresh.
    pub version: RwSignal<u32>,
}

pub fn provide_ui() {
    // a privacy signal from the browser counts as a refusal and needs no banner
    if stored().is_none() && global_privacy_control() {
        save(false);
    }
    provide_context(ConsentUi {
        banner: RwSignal::new(stored().is_none() && !dismissed_recently()),
        center: RwSignal::new(false),
        personalization: RwSignal::new(personalization_allowed()),
        version: RwSignal::new(0),
    });
}

pub fn use_ui() -> ConsentUi {
    use_context::<ConsentUi>().expect("ConsentUi provided by App")
}

impl ConsentUi {
    pub fn decide(&self, personalization: bool) {
        save(personalization);
        self.personalization.set(personalization);
        self.banner.set(false);
        self.center.set(false);
        self.version.update(|v| *v += 1);
    }

    /// The visitor closed the banner without choosing. It stays away for a week.
    pub fn dismiss(&self) {
        if let Some(s) = storage() {
            let _ = s.set_item(crate::keys::CONSENT_DISMISSED, &(js_sys::Date::now() as u64).to_string());
        }
        self.banner.set(false);
    }

    pub fn open_center(&self) {
        self.personalization.set(personalization_allowed());
        self.center.set(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_choice_expires_after_a_year_or_a_policy_change() {
        let now = 2 * YEAR_MS;
        assert!(is_current(&Choice { personalization: true, at: now - 1000, v: POLICY_VERSION }, now));
        assert!(!is_current(&Choice { personalization: true, at: now - YEAR_MS - 1, v: POLICY_VERSION }, now));
        assert!(!is_current(&Choice { personalization: true, at: now - 1000, v: POLICY_VERSION + 1 }, now));
        assert!(!is_current(&Choice { personalization: true, at: now - 1000, v: 0 }, now));
    }

    #[test]
    fn the_sheet_stays_off_the_player() {
        assert!(suppressed_on("/watch/603"));
        assert!(!suppressed_on("/browse"));
    }
}
