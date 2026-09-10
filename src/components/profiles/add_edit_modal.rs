use leptos::prelude::*;
use crate::models::profile::Profile;
use crate::components::profiles::choose_icon::ChooseIconModal;

#[derive(Clone, Debug)]
pub struct ProfileFormData {
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_kids: bool,
    pub pin: Option<String>,
}

#[component]
pub fn AddEditProfileModal(
    #[prop(into)] mode: String, // "add" or "edit"
    initial: Option<Profile>,
    on_save: Callback<ProfileFormData>,
    on_delete: Option<Callback<()>>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let initial_name = initial.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let initial_avatar = initial.as_ref().and_then(|p| p.avatar_url.clone()).unwrap_or_else(|| {
        "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string()
    });
    let initial_kids = initial.as_ref().map(|p| p.is_kids).unwrap_or(false);
    let initial_pin = initial.as_ref().and_then(|p| p.pin.clone());
    let has_pin = initial_pin.is_some();

    let (name, set_name) = signal(initial_name);
    let (avatar_url, set_avatar_url) = signal(initial_avatar);
    let (is_kids, set_kids) = signal(initial_kids);
    let (pin_enabled, set_pin_enabled) = signal(has_pin);
    let (pin_val, set_pin_val) = signal(initial_pin.unwrap_or_default());

    let (picking_avatar, set_picking_avatar) = signal(false);
    let (confirm_delete, set_confirm_delete) = signal(false);

    let is_pin_valid = move || {
        if !pin_enabled.get() {
            true
        } else {
            let p = pin_val.get();
            p.len() == 4 && p.chars().all(|c| c.is_ascii_digit())
        }
    };

    let can_save = move || {
        !name.get().trim().is_empty() && is_pin_valid()
    };

    let is_edit = mode == "edit";
    let modal_title = if is_edit { "Edit Profile" } else { "Add Profile" };

    let handle_save = move |_| {
        if !can_save() { return; }
        let pin = if pin_enabled.get() { Some(pin_val.get()) } else { None };
        on_save.run(ProfileFormData {
            name: name.get().trim().to_string(),
            avatar_url: Some(avatar_url.get()),
            is_kids: is_kids.get(),
            pin,
        });
    };

    view! {
        <div class="fixed inset-0 z-[200] flex items-center justify-center bg-black/80 backdrop-blur-sm px-4 animate-in fade-in duration-200">
            <div
                class="relative w-full max-w-[560px] max-h-[90vh] overflow-y-auto bg-[#181818] rounded-md shadow-2xl border border-white/10 p-6 sm:p-10 text-white"
            >
                // Close button
                <button
                    on:click=move |_| on_cancel.run(())
                    class="absolute top-5 right-5 w-8 h-8 rounded-full flex items-center justify-center text-white/50 hover:text-white hover:bg-white/10 transition-colors cursor-pointer"
                    aria-label="Close"
                >
                    <i class="ph-bold ph-x text-xl"></i>
                </button>

                <h2 class="text-2xl sm:text-3xl font-bold text-white text-center mb-1">
                    {modal_title}
                </h2>
                <p class="text-white/40 text-sm text-center mb-8">
                    {if is_edit { "Update this profile's name, icon, and lock." } else { "Add a profile for another person watching Pstream." }}
                </p>

                // Avatar + Name Input (Task 100)
                <div class="flex flex-col sm:flex-row items-center sm:items-start gap-6 mb-8">
                    // Avatar tile with pencil overlay
                    <button
                        type="button"
                        on:click=move |_| set_picking_avatar.set(true)
                        class="relative w-24 h-24 sm:w-28 sm:h-28 rounded-md overflow-hidden bg-white/10 shrink-0 shadow-md group hover:ring-4 hover:ring-white/20 transition-all active:scale-95 cursor-pointer"
                    >
                        <img
                            src=avatar_url
                            alt="Avatar"
                            class="w-full h-full object-cover block"
                        />
                        <div class="absolute inset-0 bg-black/50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                            <i class="ph-bold ph-pencil-simple text-white text-2xl"></i>
                        </div>
                    </button>

                    // Name input
                    <div class="flex-1 w-full space-y-2">
                        <input
                            type="text"
                            prop:value=move || name.get()
                            on:input=move |e| set_name.set(event_target_value(&e))
                            placeholder="Name"
                            maxlength="30"
                            class="w-full bg-[#262626] border border-white/20 rounded px-4 py-3 text-white text-base placeholder:text-white/30 outline-none focus:border-white transition-colors"
                        />
                    </div>
                </div>

                // Kids Mode Toggle
                <div class="border-t border-white/10 py-5">
                    <label class="flex items-center justify-between gap-4 cursor-pointer">
                        <div>
                            <span class="text-white font-semibold text-[15px]">"Kids Profile"</span>
                            <p class="text-white/40 text-[13px] mt-0.5">
                                "Only see child-friendly series and films"
                            </p>
                        </div>
                        <input
                            type="checkbox"
                            prop:checked=move || is_kids.get()
                            on:change=move |e| set_kids.set(event_target_checked(&e))
                            class="w-5 h-5 accent-[#E50914] rounded cursor-pointer"
                        />
                    </label>
                </div>

                // Profile Lock PIN Toggle
                <div class="border-t border-white/10 py-5 space-y-3">
                    <label class="flex items-center justify-between gap-4 cursor-pointer">
                        <div>
                            <span class="text-white font-semibold text-[15px]">"Profile Lock"</span>
                            <p class="text-white/40 text-[13px] mt-0.5">
                                "Require a 4-digit PIN to access this profile"
                            </p>
                        </div>
                        <input
                            type="checkbox"
                            prop:checked=move || pin_enabled.get()
                            on:change=move |e| set_pin_enabled.set(event_target_checked(&e))
                            class="w-5 h-5 accent-[#E50914] rounded cursor-pointer"
                        />
                    </label>

                    {move || {
                        if pin_enabled.get() {
                            view! {
                                <div class="pt-2">
                                    <input
                                        type="password"
                                        maxlength="4"
                                        placeholder="Enter 4-digit PIN"
                                        prop:value=move || pin_val.get()
                                        on:input=move |e| {
                                            let v = event_target_value(&e);
                                            let clean: String = v.chars().filter(|c| c.is_ascii_digit()).take(4).collect();
                                            set_pin_val.set(clean);
                                        }
                                        class="w-48 bg-[#262626] border border-white/20 rounded px-4 py-2 text-white text-base tracking-widest placeholder:tracking-normal placeholder:text-white/30 outline-none focus:border-white transition-colors"
                                    />
                                    <Show when=move || !is_pin_valid()>
                                        <p class="text-red-400 text-xs mt-1">"PIN must be exactly 4 numeric digits."</p>
                                    </Show>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div /> }.into_any()
                        }
                    }}
                </div>

                // Delete profile section (Task 099)
                {move || {
                    if is_edit && on_delete.is_some() {
                        let on_del = on_delete.unwrap();
                        view! {
                            <div class="border-t border-white/10 py-5">
                                {if confirm_delete.get() {
                                    view! {
                                        <div class="bg-red-950/40 border border-red-500/30 rounded-lg p-4 space-y-3">
                                            <p class="text-sm text-red-200">
                                                "Delete this profile? All viewing history and My List items will be permanently removed."
                                            </p>
                                            <div class="flex items-center gap-3">
                                                <button
                                                    type="button"
                                                    on:click=move |_| on_del.run(())
                                                    class="px-4 py-1.5 bg-[#E50914] hover:bg-[#f40612] text-white text-xs font-bold rounded transition-colors cursor-pointer active:scale-95"
                                                >
                                                    "Yes, Delete"
                                                </button>
                                                <button
                                                    type="button"
                                                    on:click=move |_| set_confirm_delete.set(false)
                                                    class="px-4 py-1.5 border border-white/30 text-white/70 hover:text-white rounded text-xs transition-colors cursor-pointer"
                                                >
                                                    "Cancel"
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button
                                            type="button"
                                            on:click=move |_| set_confirm_delete.set(true)
                                            class="border border-white/20 hover:border-red-500 text-white/60 hover:text-red-400 px-4 py-2 rounded text-sm font-semibold transition-colors cursor-pointer"
                                        >
                                            "Delete Profile"
                                        </button>
                                    }.into_any()
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }
                }}

                // Actions: Save & Cancel
                <div class="flex items-center justify-end gap-3 pt-6 border-t border-white/10">
                    <button
                        type="button"
                        on:click=move |_| on_cancel.run(())
                        class="px-6 py-2 border border-white/30 text-white/70 hover:text-white hover:border-white rounded text-sm font-semibold transition-colors cursor-pointer active:scale-95"
                    >
                        "Cancel"
                    </button>
                    <button
                        type="button"
                        on:click=handle_save
                        class=move || if can_save() {
                            "px-8 py-2 rounded text-sm font-bold transition-colors cursor-pointer active:scale-95 bg-[#E50914] text-white hover:bg-[#f40612]"
                        } else {
                            "px-8 py-2 rounded text-sm font-bold transition-colors cursor-pointer active:scale-95 bg-white/20 text-white/40 pointer-events-none"
                        }
                    >
                        "Save"
                    </button>
                </div>
            </div>

            // Avatar picker gallery overlay (Task 101)
            <Show when=move || picking_avatar.get()>
                <ChooseIconModal
                    current_avatar_url=Some(avatar_url.get())
                    on_select=Callback::new(move |url| {
                        set_avatar_url.set(url);
                        set_picking_avatar.set(false);
                    })
                    on_cancel=Callback::new(move |_| {
                        set_picking_avatar.set(false);
                    })
                />
            </Show>
        </div>
    }
}
