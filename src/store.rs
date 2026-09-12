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
}

pub fn provide_ui_store() {
    let initial_ambient = crate::utils::ambient::get_last_ambient_color().unwrap_or((29, 42, 50));
    provide_context(UIStore {
        info_modal_open: RwSignal::new(false),
        info_modal_movie_id: RwSignal::new(None),
        info_modal_is_tv: RwSignal::new(false),
        active_popup_id: RwSignal::new(None),
        ambient_color: RwSignal::new(initial_ambient),
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
