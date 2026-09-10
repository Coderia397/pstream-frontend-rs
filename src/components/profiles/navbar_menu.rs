use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::store::use_profile_store;

#[component]
pub fn NavbarProfileMenu() -> impl IntoView {
    let profile_store = use_profile_store();
    let profiles = profile_store.profiles;
    let active_profile_id = profile_store.active_profile_id;
    let (is_open, set_is_open) = signal(false);
    let navigate = use_navigate();

    let active_profile = move || {
        let id = active_profile_id.get()?;
        profiles.get().into_iter().find(|p| p.id == id)
    };

    let other_profiles = move || {
        let id = active_profile_id.get().unwrap_or_default();
        profiles.get().into_iter().filter(|p| p.id != id).collect::<Vec<_>>()
    };

    let logout = move |_| {
        active_profile_id.set(None);
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.remove_item("pstream_active_profile");
        }
        set_is_open.set(false);
    };

    let switch_profile = move |id: String| {
        active_profile_id.set(Some(id));
        set_is_open.set(false);
    };

    let manage_profiles = move |_| {
        active_profile_id.set(None);
        set_is_open.set(false);
    };

    let nav_to_account = navigate.clone();
    let nav_to_help = navigate.clone();

    view! {
        <div 
            class="relative flex items-center group/menu cursor-pointer select-none"
            on:mouseenter=move |_| set_is_open.set(true)
            on:mouseleave=move |_| set_is_open.set(false)
        >
            <div class="flex items-center gap-2">
                <div class="w-8 h-8 rounded-sm overflow-hidden ring-1 ring-white/20">
                    {move || {
                        let p = active_profile();
                        if p.as_ref().map(|p| p.is_kids && p.avatar_url.is_none()).unwrap_or(false) {
                            view! { <crate::components::profiles::kids_avatar::KidsAvatar size=32.0 /> }.into_any()
                        } else {
                            let url = p.and_then(|p| p.avatar_url).unwrap_or_else(|| "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string());
                            view! { <img src=url class="w-full h-full object-cover" alt="Current Profile" /> }.into_any()
                        }
                    }}
                </div>
                <i class="ph-fill ph-caret-down text-white text-xs transition-transform duration-200"
                   class=("rotate-180", move || is_open.get())></i>
            </div>

            // Dropdown menu (Task 104)
            <div 
                class=move || if is_open.get() {
                    "absolute top-[120%] right-0 w-[220px] bg-black/95 border border-[#333] rounded-sm py-2 shadow-2xl transition-all duration-200 origin-top-right z-50 divide-y divide-[#333] opacity-100 scale-100 pointer-events-auto"
                } else {
                    "absolute top-[120%] right-0 w-[220px] bg-black/95 border border-[#333] rounded-sm py-2 shadow-2xl transition-all duration-200 origin-top-right z-50 divide-y divide-[#333] opacity-0 scale-95 pointer-events-none"
                }
            >
                <div class="flex flex-col gap-3 px-3 pb-3">
                    // Other profiles list
                    {move || other_profiles().into_iter().map(|p| {
                        let id = p.id.clone();
                        let name = p.name.clone();
                        let is_kids = p.is_kids;
                        let has_custom_avatar = p.avatar_url.is_some();
                        let avatar = p.avatar_url.clone().unwrap_or_else(|| "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string());
                        
                        view! {
                            <div 
                                class="flex items-center gap-3 hover:underline cursor-pointer group"
                                on:click=move |_| switch_profile(id.clone())
                            >
                                <div class="w-8 h-8 rounded-sm overflow-hidden shrink-0">
                                    {if is_kids && !has_custom_avatar {
                                        view! { <crate::components::profiles::kids_avatar::KidsAvatar size=32.0 /> }.into_any()
                                    } else {
                                        view! { <img src=avatar class="w-full h-full object-cover" alt=name.clone() /> }.into_any()
                                    }}
                                </div>
                                <span class="text-white text-sm font-medium group-hover:underline truncate">{name}</span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}

                    // Manage Profiles button
                    <div
                        on:click=manage_profiles
                        class="flex items-center gap-3 hover:underline cursor-pointer mt-1 text-white/80 hover:text-white"
                    >
                        <i class="ph-bold ph-pencil-simple text-gray-400 text-xl w-8 text-center"></i>
                        <span class="text-sm font-medium">"Manage Profiles"</span>
                    </div>

                    // Transfer Profile
                    <div class="flex items-center gap-3 hover:underline cursor-pointer text-white/80 hover:text-white">
                        <i class="ph-bold ph-user-focus text-gray-400 text-xl w-8 text-center"></i>
                        <span class="text-sm font-medium">"Transfer Profile"</span>
                    </div>

                    // Account
                    <div
                        on:click=move |_| {
                            set_is_open.set(false);
                            nav_to_account("/browse/my-list", Default::default());
                        }
                        class="flex items-center gap-3 hover:underline cursor-pointer text-white/80 hover:text-white"
                    >
                        <i class="ph-bold ph-user text-gray-400 text-xl w-8 text-center"></i>
                        <span class="text-sm font-medium">"Account"</span>
                    </div>

                    // Help Center
                    <div
                        on:click=move |_| {
                            set_is_open.set(false);
                            nav_to_help("/browse", Default::default());
                        }
                        class="flex items-center gap-3 hover:underline cursor-pointer text-white/80 hover:text-white"
                    >
                        <i class="ph-bold ph-question text-gray-400 text-xl w-8 text-center"></i>
                        <span class="text-sm font-medium">"Help Center"</span>
                    </div>
                </div>
                
                // Sign out button
                <div class="pt-3 px-3 text-center">
                    <button 
                        on:click=logout
                        class="text-white text-sm font-bold hover:underline cursor-pointer"
                    >
                        "Sign out of Pstream"
                    </button>
                </div>
            </div>
        </div>
    }
}
