//! Cookie and storage consent. Everything the app keeps in the browser is listed in the Privacy Preference Center
//! (components/consent.rs). Essential items (profiles, progress, language, this choice) are always on. Personalisation, which
//! sends the title you started and a random session number to the recommendation service, is off until the visitor accepts.
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub personalization: bool,
    pub at: u64,
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

/// The saved choice, or None when the visitor has not decided yet.
pub fn stored() -> Option<Choice> {
    storage().and_then(|s| s.get_item(crate::keys::CONSENT).ok().flatten()).and_then(|raw| serde_json::from_str(&raw).ok())
}

pub fn personalization_allowed() -> bool {
    stored().is_some_and(|c| c.personalization)
}

pub fn save(personalization: bool) {
    if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(&Choice { personalization, at: js_sys::Date::now() as u64 })) {
        let _ = s.set_item(crate::keys::CONSENT, &raw);
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
    /// The top banner is showing (no choice saved and not dismissed this visit).
    pub banner: RwSignal<bool>,
    pub center: RwSignal<bool>,
    pub personalization: RwSignal<bool>,
    /// Bumped on every save so views that show the current choice refresh.
    pub version: RwSignal<u32>,
}

pub fn provide_ui() {
    provide_context(ConsentUi {
        banner: RwSignal::new(stored().is_none()),
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

    pub fn open_center(&self) {
        self.personalization.set(personalization_allowed());
        self.center.set(true);
    }
}
