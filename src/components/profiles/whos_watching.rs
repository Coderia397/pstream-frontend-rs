use leptos::prelude::*;
use crate::models::profile::Profile;
use crate::store::use_profile_store;
use crate::components::profiles::pin_prompt::ProfilePinPrompt;
use crate::components::profiles::add_edit_modal::{AddEditProfileModal, ProfileFormData};
use crate::components::profiles::switch_overlay::ProfileSwitchOverlay;

fn save_profiles_to_storage(profiles: &[Profile], active_id: Option<&str>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(json) = serde_json::to_string(profiles) {
            let _ = storage.set_item("pstream_profiles", &json);
        }
        if let Some(id) = active_id {
            let _ = storage.set_item("pstream_active_profile", id);
        }
    }
}

#[component]
pub fn WhosWatchingGate() -> impl IntoView {
    let profile_store = use_profile_store();
    let profiles_signal = profile_store.profiles;
    let active_profile_id = profile_store.active_profile_id;

    let (edit_mode, set_edit_mode) = signal(false);
    let (pin_prompt_profile, set_pin_prompt_profile) = signal(None::<Profile>);
    let (modal_state, set_modal_state) = signal(None::<(String, Option<Profile>)>); // (mode, initial)
    let (switching_target, set_switching_target) = signal(None::<Profile>);
    let (unlocked_ids, set_unlocked_ids) = signal(Vec::<String>::new());

    // Perform profile switch with overlay animation (Task 103)
    let perform_switch = move |p: Profile| {
        let pid = p.id.clone();
        set_switching_target.set(Some(p));
        save_profiles_to_storage(&profiles_signal.get_untracked(), Some(&pid));
    };

    let on_select_tile = move |p: Profile| {
        if edit_mode.get() {
            // In edit mode, clicking a tile opens edit modal (Task 099)
            set_modal_state.set(Some(("edit".to_string(), Some(p))));
        } else {
            // Check if profile has PIN lock (Task 098)
            let is_unlocked = unlocked_ids.get().contains(&p.id);
            if p.pin.is_some() && !is_unlocked {
                set_pin_prompt_profile.set(Some(p));
            } else {
                perform_switch(p);
            }
        }
    };

    let on_unlock_pin = move || {
        if let Some(p) = pin_prompt_profile.get() {
            set_unlocked_ids.update(|ids| ids.push(p.id.clone()));
            set_pin_prompt_profile.set(None);
            perform_switch(p);
        }
    };

    let on_save_modal = move |form: ProfileFormData| {
        if let Some((mode, initial)) = modal_state.get() {
            if mode == "add" {
                let new_id = format!("p_{}", js_sys::Date::now() as u64);
                let new_profile = Profile {
                    id: new_id.clone(),
                    name: form.name,
                    avatar_url: form.avatar_url,
                    is_kids: form.is_kids,
                    is_default: Some(false),
                    pin: form.pin,
                    sort_order: profiles_signal.get().len() as i32,
                };
                profiles_signal.update(|list| list.push(new_profile));
            } else if mode == "edit" {
                if let Some(init) = initial {
                    profiles_signal.update(|list| {
                        if let Some(target) = list.iter_mut().find(|p| p.id == init.id) {
                            target.name = form.name;
                            target.avatar_url = form.avatar_url;
                            target.is_kids = form.is_kids;
                            target.pin = form.pin;
                        }
                    });
                }
            }
            save_profiles_to_storage(&profiles_signal.get(), active_profile_id.get().as_deref());
            set_modal_state.set(None);
        }
    };

    let on_delete_profile = move |target_id: String| {
        profiles_signal.update(|list| list.retain(|p| p.id != target_id));
        if active_profile_id.get().as_deref() == Some(&target_id) {
            let next_id = profiles_signal.get().first().map(|p| p.id.clone());
            active_profile_id.set(next_id.clone());
            save_profiles_to_storage(&profiles_signal.get(), next_id.as_deref());
        } else {
            save_profiles_to_storage(&profiles_signal.get(), active_profile_id.get().as_deref());
        }
        set_modal_state.set(None);
    };

    view! {
        <div class="fixed inset-0 z-[9999] bg-[#141414] text-white flex flex-col items-center justify-center animate-in fade-in duration-500 select-none">
            <h1 class="text-3xl md:text-5xl text-white font-medium mb-12 tracking-wide text-center">
                {move || if edit_mode.get() { "Manage Profiles:" } else { "Who's watching?" }}
            </h1>

            // Profile tiles grid (Task 095)
            <div class="flex flex-wrap items-center justify-center gap-[2vw] md:gap-8 max-w-[85vw] md:max-w-5xl mx-auto px-4">
                {move || profiles_signal.get().into_iter().map(|profile| {
                    let p_clone = profile.clone();
                    let name = profile.name.clone();
                    let is_kids = profile.is_kids;
                    let has_pin = profile.pin.is_some();
                    let avatar = profile.avatar_url.clone().unwrap_or_else(|| {
                        "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string()
                    });

                    view! {
                        <button
                            type="button"
                            on:click=move |_| on_select_tile(p_clone.clone())
                            class="flex flex-col items-center gap-3 group w-[110px] sm:w-[140px] cursor-pointer focus:outline-none"
                        >
                            // Tile with white hover outline (Task 095)
                            <div class="relative w-[110px] h-[110px] sm:w-[140px] sm:h-[140px] rounded-md overflow-hidden shadow-lg ring-white transition-all group-hover:ring-[3px] group-active:scale-[0.98]">
                                {if is_kids && p_clone.avatar_url.is_none() {
                                    view! { <crate::components::profiles::kids_avatar::KidsAvatar size=140.0 /> }.into_any()
                                } else {
                                    view! {
                                        <img
                                            src=avatar
                                            class="w-full h-full object-cover"
                                            alt=name.clone()
                                        />
                                        {if is_kids {
                                            view! {
                                                <div class="absolute bottom-1.5 right-1.5 bg-[#E50914] text-white text-[9px] font-black px-1 py-0.5 rounded shadow">
                                                    "KIDS"
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div /> }.into_any()
                                        }}
                                    }.into_any()
                                }}

                                // Edit mode pencil overlay
                                {move || if edit_mode.get() {
                                    view! {
                                        <div class="absolute inset-0 bg-black/55 flex items-center justify-center">
                                            <i class="ph-bold ph-pencil-simple text-white text-3xl"></i>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div /> }.into_any()
                                }}
                            </div>

                            // Profile name and lock icon (Task 096)
                            <span
                                class=move || if edit_mode.get() {
                                    "text-sm sm:text-base font-medium transition-colors text-white/40"
                                } else {
                                    "text-sm sm:text-base font-medium transition-colors text-[#808080] group-hover:text-white"
                                }
                            >
                                {name}
                            </span>
                            {if has_pin && !edit_mode.get() {
                                view! {
                                    <i class="ph-bold ph-lock-key text-xs text-[#808080] -mt-1.5"></i>
                                }.into_any()
                            } else {
                                view! { <div /> }.into_any()
                            }}
                        </button>
                    }
                }).collect::<Vec<_>>()}

                // Add Profile Tile (Task 097: rounded-full circle background #2b2b2b)
                {move || {
                    if profiles_signal.get().len() < 5 {
                        view! {
                            <button
                                type="button"
                                on:click=move |_| set_modal_state.set(Some(("add".to_string(), None)))
                                class="flex flex-col items-center gap-3 group w-[110px] sm:w-[140px] cursor-pointer focus:outline-none"
                            >
                                <div class="w-[110px] h-[110px] sm:w-[140px] sm:h-[140px] rounded-full bg-[#2b2b2b] flex items-center justify-center transition-all group-hover:bg-[#404040] group-active:scale-[0.98]">
                                    <i class="ph-bold ph-plus text-[#808080] group-hover:text-white text-4xl sm:text-5xl transition-colors"></i>
                                </div>
                                <span class="text-sm sm:text-base text-[#808080] group-hover:text-white transition-colors font-medium">
                                    "Add Profile"
                                </span>
                            </button>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }
                }}
            </div>

            // Manage Profiles / Done button (Task 099)
            <div class="mt-16 md:mt-24 mb-10">
                <button
                    type="button"
                    on:click=move |_| set_edit_mode.update(|e| *e = !*e)
                    class="border border-gray-500 text-gray-500 hover:text-white hover:border-white px-6 py-2 text-sm sm:text-base md:text-lg font-medium tracking-[2px] uppercase transition-colors cursor-pointer active:scale-95"
                    class=("bg-white", move || edit_mode.get())
                    class=("!text-black", move || edit_mode.get())
                >
                    {move || if edit_mode.get() { "Done" } else { "Manage Profiles" }}
                </button>
            </div>

            // PIN prompt modal (Task 098)
            <Show when=move || pin_prompt_profile.get().is_some()>
                {move || pin_prompt_profile.get().map(|p| view! {
                    <ProfilePinPrompt
                        profile=p
                        on_unlock=Callback::new(move |_| on_unlock_pin())
                        on_cancel=Callback::new(move |_| set_pin_prompt_profile.set(None))
                    />
                })}
            </Show>

            // Add/Edit Profile modal (Task 100)
            <Show when=move || modal_state.get().is_some()>
                {move || modal_state.get().map(|(mode, initial)| {
                    let target_id = initial.as_ref().map(|p| p.id.clone());
                    let on_delete_cb = target_id.map(|tid| Callback::new(move |_| on_delete_profile(tid.clone())));
                    view! {
                        <AddEditProfileModal
                            mode=mode
                            initial=initial
                            on_save=Callback::new(move |form| on_save_modal(form))
                            on_delete=on_delete_cb
                            on_cancel=Callback::new(move |_| set_modal_state.set(None))
                        />
                    }
                })}
            </Show>

            // Full-screen Switch Overlay Animation (Task 103)
            <Show when=move || switching_target.get().is_some()>
                {move || switching_target.get().map(|p| {
                    let pid = p.id.clone();
                    view! {
                        <ProfileSwitchOverlay
                            profile=p
                            on_finish=Callback::new(move |_| {
                                active_profile_id.set(Some(pid.clone()));
                                set_switching_target.set(None);
                            })
                        />
                    }
                })}
            </Show>
        </div>
    }
}
