use leptos::prelude::*;
use std::collections::HashMap;
use crate::services::tmdb::MediaItem;

#[derive(Clone, Copy)]
pub struct UIStore {
    pub info_modal_open: RwSignal<bool>,
    pub info_modal_movie_id: RwSignal<Option<u32>>,
    pub info_modal_is_tv: RwSignal<bool>,
    pub active_popup_id: RwSignal<Option<String>>,
    pub ambient_color: RwSignal<(u8, u8, u8)>,
    pub preview_muted: RwSignal<bool>,
    pub modal_initial_time: RwSignal<f64>,
    pub modal_current_time: RwSignal<f64>,
    pub modal_closing_time: RwSignal<Option<(u32, f64)>>,
    pub hero_paused_by_modal: RwSignal<bool>,
    pub modal_video_key: RwSignal<Option<String>>,
    pub modal_is_teaser: RwSignal<bool>,
    /// One-shot seek signal: set when modal closes so hero TrailerPlayer jumps to exact timestamp.
    /// Cleared immediately after being consumed by the seek Effect.
    pub hero_resume_seek: RwSignal<Option<f64>>,
}

pub fn get_session_preview_muted(profile_id: Option<&str>, is_kids: bool) -> bool {
    if is_kids {
        return true;
    }
    let storage = web_sys::window().and_then(|w| w.session_storage().ok().flatten());
    if let Some(ref s) = storage {
        if let Some(pid) = profile_id {
            if let Ok(Some(val)) = s.get_item(&format!("pstream_preview_muted_{}", pid)) {
                return val == "1" || val == "true";
            }
        }
        if let Ok(Some(val)) = s.get_item("pstream_preview_muted") {
            return val == "1" || val == "true";
        }
    }
    true // default to muted on fresh session
}

pub fn save_session_preview_muted(muted: bool, profile_id: Option<&str>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
        let val = if muted { "1" } else { "0" };
        let _ = storage.set_item("pstream_preview_muted", val);
        if let Some(pid) = profile_id {
            let _ = storage.set_item(&format!("pstream_preview_muted_{}", pid), val);
        }
    }
}

impl UIStore {
    pub fn set_preview_muted(&self, muted: bool) {
        self.preview_muted.set(muted);
        save_session_preview_muted(muted, None);
    }

    pub fn set_preview_muted_with_profile(&self, muted: bool, profile_id: Option<&str>) {
        self.preview_muted.set(muted);
        save_session_preview_muted(muted, profile_id);
    }
}

pub fn provide_ui_store() {
    let initial_ambient = crate::utils::ambient::get_last_ambient_color().unwrap_or((16, 21, 25));
    let initial_muted = get_session_preview_muted(None, false);
    provide_context(UIStore {
        info_modal_open: RwSignal::new(false),
        info_modal_movie_id: RwSignal::new(None),
        info_modal_is_tv: RwSignal::new(false),
        active_popup_id: RwSignal::new(None),
        ambient_color: RwSignal::new(initial_ambient),
        preview_muted: RwSignal::new(initial_muted),
        modal_initial_time: RwSignal::new(0.0),
        modal_current_time: RwSignal::new(0.0),
        modal_closing_time: RwSignal::new(None),
        hero_paused_by_modal: RwSignal::new(false),
        modal_video_key: RwSignal::new(None),
        modal_is_teaser: RwSignal::new(false),
        hero_resume_seek: RwSignal::new(None),
    });
}

pub fn use_ui_store() -> UIStore {
    use_context::<UIStore>().expect("UIStore not provided")
}

#[derive(Clone)]
pub struct ProfileStore {
    pub active_profile_id: RwSignal<Option<String>>,
    pub profiles: RwSignal<Vec<crate::models::profile::Profile>>,
    pub is_authenticated: RwSignal<bool>,
    pub is_kids_mode: RwSignal<bool>,
}

pub fn provide_profile_store() {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    let initial_profile = storage
        .as_ref()
        .and_then(|s| s.get_item("pstream_active_profile").ok().flatten())
        .or_else(|| Some("p1".to_string()));

    let initial_profiles = storage
        .as_ref()
        .and_then(|s| s.get_item("pstream_profiles").ok().flatten())
        .and_then(|raw| serde_json::from_str::<Vec<crate::models::profile::Profile>>(&raw).ok())
        .unwrap_or_else(|| vec![
            crate::models::profile::Profile {
                id: "p1".to_string(),
                name: "Main".to_string(),
                avatar_url: Some("https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string()),
                is_kids: false,
                is_default: Some(true),
                pin: None,
                sort_order: 0,
            },
            crate::models::profile::Profile {
                id: "p2".to_string(),
                name: "Kids".to_string(),
                avatar_url: None,
                is_kids: true,
                is_default: Some(false),
                pin: None,
                sort_order: 1,
            },
        ]);

    provide_context(ProfileStore {
        active_profile_id: RwSignal::new(initial_profile),
        profiles: RwSignal::new(initial_profiles),
        is_authenticated: RwSignal::new(true),
        is_kids_mode: RwSignal::new(false),
    });
}

pub fn use_profile_store() -> ProfileStore {
    use_context::<ProfileStore>().expect("ProfileStore not provided")
}

#[derive(Clone, Debug)]
pub struct LibraryEntry {
    pub media: MediaItem,
    pub added_at: u64,
}

#[derive(Clone)]
pub struct LibraryStore {
    pub my_list: RwSignal<HashMap<u32, LibraryEntry>>,
}

pub fn provide_library_store() {
    provide_context(LibraryStore {
        my_list: RwSignal::new(HashMap::new()),
    });
}

pub fn use_library_store() -> LibraryStore {
    use_context::<LibraryStore>().expect("LibraryStore not provided")
}

pub mod watch_store;
pub use watch_store::{provide_watch_store, use_watch_store, WatchRecord, WatchStore};
