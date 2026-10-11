use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::store::use_profile_store;
use crate::data::{AVATAR_CATEGORIES, DISPLAY_LANGUAGES, SUBTITLE_LANGUAGES, DEFAULT_AVATAR};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SettingsView {
    Overview,
    ProfileEdit,
    AvatarPicker,
    Language,
    Subtitle,
    Playback,
    ViewingActivity,
    Privacy,
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let navigate = leptos::prelude::StoredValue::new(use_navigate());
    let profile_store = use_profile_store();

    let (current_view, set_current_view) = signal(SettingsView::Overview);

    // Profile state
    let active_profile = move || {
        let current_id = profile_store.active_profile_id.get();
        profile_store.profiles.get().into_iter().find(|p| Some(p.id.clone()) == current_id)
    };

    let profile_name = move || {
        active_profile().map(|p| p.name).unwrap_or_else(|| "User".to_string())
    };

    let avatar_url = move || {
        active_profile().and_then(|p| p.avatar_url).unwrap_or_else(|| DEFAULT_AVATAR.to_string())
    };

    // Subtitle appearance settings
    let (sub_size, set_sub_size) = signal("medium".to_string());
    let (sub_color, set_sub_color) = signal("white".to_string());
    let (sub_opacity, set_sub_opacity) = signal(50);

    // Playback settings
    let (autoplay_next, set_autoplay_next) = signal(true);
    let (autoplay_previews, set_autoplay_previews) = signal(true);
    let (autoplay_video, set_autoplay_video) = signal(true);

    // Language settings
    let (display_lang, set_display_lang) = signal("en".to_string());
    let (sub_lang, set_sub_lang) = signal("en".to_string());

    // Edit name temporary input
    let (edit_name, set_edit_name) = signal(profile_name());

    let handle_back = move |_| {
        match current_view.get() {
            SettingsView::Overview => {
                navigate.with_value(|n| n("/browse", Default::default()));
            }
            SettingsView::AvatarPicker => {
                set_current_view.set(SettingsView::ProfileEdit);
            }
            _ => {
                set_current_view.set(SettingsView::Overview);
            }
        }
    };

    let title = move || match current_view.get() {
        SettingsView::Overview => "Account & Preferences",
        SettingsView::ProfileEdit => "Edit Profile",
        SettingsView::AvatarPicker => "Choose Profile Icon",
        SettingsView::Language => "Languages",
        SettingsView::Subtitle => "Subtitle Appearance",
        SettingsView::Playback => "Playback Settings",
        SettingsView::ViewingActivity => "Viewing Activity",
        SettingsView::Privacy => "Privacy & Data Settings",
    };

    view! {
        <div class="relative min-h-screen bg-void text-ink font-sans pt-20 pb-16">
            <main class="w-full max-w-[1100px] mx-auto px-6 lg:px-12 py-6 flex flex-col lg:flex-row gap-8 lg:gap-16">
                // Back button column
                <div class="flex-shrink-0 flex items-center lg:items-start">
                    <button
                        on:click=handle_back
                        class="p-2 lg:p-0 text-ink cursor-pointer hover:scale-110 active:scale-95 transition-transform flex items-center gap-2"
                    >
                        <i class="ph-bold ph-arrow-left text-2xl"></i>
                    </button>
                    <h2 class="lg:hidden ml-3 text-xl font-bold truncate">
                        {title}
                    </h2>
                </div>

                // Main content
                <div class="flex-1 w-full lg:max-w-[750px]">
                    <h1 class="hidden lg:block text-3xl font-black text-ink tracking-tight mb-8">
                        {title}
                    </h1>

                    {move || match current_view.get() {
                        SettingsView::Overview => view! {
                            <div class="space-y-8 animate-fadeIn">
                                // Profile card
                                <div class="border border-ink/10 rounded-card bg-ink/5 overflow-hidden">
                                    <div
                                        on:click=move |_| {
                                            set_edit_name.set(profile_name());
                                            set_current_view.set(SettingsView::ProfileEdit);
                                        }
                                        class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                    >
                                        <div class="w-12 h-12 rounded-control overflow-hidden mr-4 bg-zinc-800 shrink-0">
                                            <img src=avatar_url alt="" class="w-full h-full object-cover" />
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <div class="text-body-lg font-bold text-ink truncate">{profile_name}</div>
                                            <div class="text-label text-ink/40 mt-0.5">"Edit personal information and icon"</div>
                                        </div>
                                        <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                    </div>
                                </div>

                                // Preferences section
                                <div class="space-y-3">
                                    <h2 class="text-xs font-bold text-ink/40 uppercase tracking-wider ml-1">
                                        {crate::i18n::t("settings-preferences")}
                                    </h2>
                                    <div class="border border-ink/10 rounded-card bg-ink/5 divide-y divide-ink/10 overflow-hidden">
                                        // Language row
                                        <div
                                            on:click=move |_| set_current_view.set(SettingsView::Language)
                                            class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                        >
                                            <i class="ph-bold ph-translate text-2xl text-ink/70 mr-4"></i>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-body-md font-bold text-ink">"Languages"</div>
                                                <div class="text-label text-ink/40 mt-0.5">"Set languages for display and audio"</div>
                                            </div>
                                            <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                        </div>

                                        // Subtitle appearance row
                                        <div
                                            on:click=move |_| set_current_view.set(SettingsView::Subtitle)
                                            class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                        >
                                            <i class="ph-bold ph-subtitles text-2xl text-ink/70 mr-4"></i>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-body-md font-bold text-ink">"Subtitle Appearance"</div>
                                                <div class="text-label text-ink/40 mt-0.5">"Customize the way subtitles look"</div>
                                            </div>
                                            <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                        </div>

                                        // Playback row
                                        <div
                                            on:click=move |_| set_current_view.set(SettingsView::Playback)
                                            class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                        >
                                            <i class="ph-bold ph-play-circle text-2xl text-ink/70 mr-4"></i>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-body-md font-bold text-ink">"Playback Settings"</div>
                                                <div class="text-label text-ink/40 mt-0.5">"Configure autoplay, audio and video quality"</div>
                                            </div>
                                            <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                        </div>

                                        // Viewing activity row
                                        <div
                                            on:click=move |_| set_current_view.set(SettingsView::ViewingActivity)
                                            class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                        >
                                            <i class="ph-bold ph-clock text-2xl text-ink/70 mr-4"></i>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-body-md font-bold text-ink">"Viewing Activity"</div>
                                                <div class="text-label text-ink/40 mt-0.5">"Manage viewing history and ratings"</div>
                                            </div>
                                            <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                        </div>

                                        // Privacy row
                                        <div
                                            on:click=move |_| set_current_view.set(SettingsView::Privacy)
                                            class="flex items-center p-5 cursor-pointer hover:bg-ink/10 transition-colors group"
                                        >
                                            <i class="ph-bold ph-shield-check text-2xl text-ink/70 mr-4"></i>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-body-md font-bold text-ink">"Privacy & Data Settings"</div>
                                                <div class="text-label text-ink/40 mt-0.5">"Manage usage of personal information"</div>
                                            </div>
                                            <i class="ph-bold ph-caret-right text-xl text-ink/40 group-hover:text-ink transition-colors"></i>
                                        </div>
                                    </div>
                                </div>

                                // Sign out
                                <div class="border border-ink/10 rounded-card bg-ink/5 overflow-hidden">
                                    <div
                                        on:click=move |_| {
                                            profile_store.active_profile_id.set(None);
                                            navigate.with_value(|n| n("/browse", Default::default()));
                                        }
                                        class="flex items-center p-5 cursor-pointer hover:bg-danger/10 transition-colors group"
                                    >
                                        <i class="ph-bold ph-sign-out text-2xl text-danger mr-4"></i>
                                        <div class="flex-1 text-body-md font-bold text-danger">
                                            {crate::i18n::t("settings-sign-out")}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any(),

                        SettingsView::ProfileEdit => view! {
                            <div class="max-w-[500px] space-y-8 animate-fadeIn">
                                <div class="flex items-center gap-6">
                                    <div
                                        on:click=move |_| set_current_view.set(SettingsView::AvatarPicker)
                                        class="relative w-24 h-24 rounded-card overflow-hidden cursor-pointer group bg-zinc-800"
                                    >
                                        <img src=avatar_url alt="" class="w-full h-full object-cover" />
                                        <div class="absolute inset-0 bg-void/50 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
                                            <i class="ph-bold ph-pencil-simple text-ink text-xl"></i>
                                        </div>
                                    </div>
                                    <div class="flex-1">
                                        <label class="block text-xs text-ink/40 font-semibold mb-2">"PROFILE NAME"</label>
                                        <input
                                            type="text"
                                            prop:value=edit_name
                                            on:input=move |e| set_edit_name.set(event_target_value(&e))
                                            class="w-full bg-ink/10 border border-ink/20 rounded-control px-4 py-2.5 text-ink font-medium focus:outline-none focus:border-ink transition-colors"
                                        />
                                    </div>
                                </div>

                                <div class="flex gap-4 pt-4">
                                    <button
                                        on:click=move |_| {
                                            let current_id = profile_store.active_profile_id.get();
                                            let name = edit_name.get().trim().to_string();
                                            if !name.is_empty() {
                                                if let Some(id) = current_id {
                                                    profile_store.profiles.update(|list| {
                                                        if let Some(p) = list.iter_mut().find(|p| p.id == id) {
                                                            p.name = name;
                                                        }
                                                    });
                                                }
                                            }
                                            set_current_view.set(SettingsView::Overview);
                                        }
                                        class="btn-ink px-8 py-2.5 rounded-control"
                                    >
                                        {crate::i18n::t("add-edit-modal-save")}
                                    </button>
                                    <button
                                        on:click=move |_| set_current_view.set(SettingsView::Overview)
                                        class="px-8 py-2.5 bg-transparent border border-ink/20 text-ink font-bold rounded-control hover:bg-ink/10 transition-colors"
                                    >
                                        {crate::i18n::t("continue-watching-options-sheet-cancel")}
                                    </button>
                                </div>
                            </div>
                        }.into_any(),

                        SettingsView::AvatarPicker => view! {
                            <div class="space-y-8 animate-fadeIn pb-12">
                                {AVATAR_CATEGORIES.iter().map(|cat| {
                                    let cat_name = cat.name;
                                    let cat_avatars = cat.avatars;
                                    view! {
                                        <div class="space-y-3">
                                            <h3 class="text-lg font-bold text-ink">{cat_name}</h3>
                                            <div class="flex gap-3 overflow-x-auto pb-2 scrollbar-hide">
                                                {cat_avatars.iter().map(|av| {
                                                    let url = av.url.to_string();
                                                    let url_select = url.clone();
                                                    view! {
                                                        <div
                                                            on:click=move |_| {
                                                                let current_id = profile_store.active_profile_id.get();
                                                                if let Some(id) = current_id {
                                                                    let u = url_select.clone();
                                                                    profile_store.profiles.update(|list| {
                                                                        if let Some(p) = list.iter_mut().find(|p| p.id == id) {
                                                                            p.avatar_url = Some(u);
                                                                        }
                                                                    });
                                                                }
                                                                set_current_view.set(SettingsView::ProfileEdit);
                                                            }
                                                            class="w-20 h-20 rounded-field overflow-hidden shrink-0 cursor-pointer border-2 border-transparent hover:border-ink transition-all hover:scale-105"
                                                        >
                                                            <img src=url alt=av.name class="w-full h-full object-cover" />
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any(),

                        SettingsView::Language => view! {
                            <div class="space-y-8 animate-fadeIn max-w-[600px]">
                                <div>
                                    <h3 class="text-lg font-bold text-ink mb-2">"Display Language"</h3>
                                    <p class="text-sm text-ink/50 mb-4">"Choose the language used for navigation, titles, and menus."</p>
                                    <select
                                        on:change=move |e| set_display_lang.set(event_target_value(&e))
                                        prop:value=display_lang
                                        class="w-full bg-ink/10 border border-ink/20 rounded-control p-3 text-ink focus:outline-none focus:border-ink"
                                    >
                                        {DISPLAY_LANGUAGES.iter().map(|l| {
                                            view! { <option value=l.code class="bg-page text-ink">{l.label}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>

                                <div class="pt-4 border-t border-ink/10">
                                    <h3 class="text-lg font-bold text-ink mb-2">"Preferred Subtitle Language"</h3>
                                    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
                                        {SUBTITLE_LANGUAGES.iter().map(|l| {
                                            let code = l.code;
                                            let is_sel = move || sub_lang.get() == code;
                                            view! {
                                                <button
                                                    on:click=move |_| set_sub_lang.set(code.to_string())
                                                    class=move || format!(
                                                        "p-3 rounded-control border text-left text-sm font-semibold transition-colors {}",
                                                        if is_sel() { "bg-ink text-void border-ink" } else { "bg-ink/5 text-ink/70 border-ink/10 hover:border-ink/30" }
                                                    )
                                                >
                                                    {l.label}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                <div class="pt-4">
                                    <button
                                        on:click=move |_| set_current_view.set(SettingsView::Overview)
                                        class="btn-ink px-8 py-2.5 rounded-control"
                                    >
                                        {crate::i18n::t("settings-save-preferences")}
                                    </button>
                                </div>
                            </div>
                        }.into_any(),

                        SettingsView::Subtitle => view! {
                            <div class="space-y-8 animate-fadeIn max-w-[650px]">
                                // Live preview
                                <div class="relative h-44 bg-zinc-900 rounded-card overflow-hidden border border-ink/10 flex items-center justify-center">
                                    <div class="absolute inset-0 bg-gradient-to-t from-void/80 via-void/30 to-void/60 pointer-events-none" />
                                    <span
                                        style=move || {
                                            let size_px = match sub_size.get().as_str() {
                                                "tiny" => "14px",
                                                "small" => "18px",
                                                "large" => "26px",
                                                "huge" => "32px",
                                                _ => "22px",
                                            };
                                            let color = match sub_color.get().as_str() {
                                                "yellow" => "#fef08a",
                                                "cyan" => "#a5f3fc",
                                                "green" => "#86efac",
                                                "red" => "#fca5a5",
                                                _ => "#ffffff",
                                            };
                                            let op = sub_opacity.get() as f64 / 100.0;
                                            format!(
                                                "font-size: {}; color: {}; background-color: rgba(0, 0, 0, {}); padding: 4px 12px; border-radius: 4px; text-shadow: 0 2px 4px rgba(0,0,0,0.8);",
                                                size_px, color, op
                                            )
                                        }
                                        class="relative z-10 font-bold text-center"
                                    >
                                        {crate::i18n::t("settings-this-is-a-preview-of")}
                                    </span>
                                </div>

                                // Font size
                                <div>
                                    <h4 class="text-sm font-bold text-ink/60 mb-2">"TEXT SIZE"</h4>
                                    <div class="flex gap-2">
                                        {["tiny", "small", "medium", "large", "huge"].iter().map(|sz| {
                                            let s = sz.to_string();
                                            let s_btn = s.clone();
                                            let s_label = s.clone();
                                            let is_sel = move || sub_size.get() == s;
                                            view! {
                                                <button
                                                    on:click=move |_| set_sub_size.set(s_btn.clone())
                                                    class=move || format!(
                                                        "px-4 py-2 rounded-control text-xs uppercase font-bold border transition-colors {}",
                                                        if is_sel() { "bg-ink text-void border-ink" } else { "bg-ink/5 text-ink/70 border-ink/10 hover:border-ink/30" }
                                                    )
                                                >
                                                    {s_label.clone()}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                // Color
                                <div>
                                    <h4 class="text-sm font-bold text-ink/60 mb-2">"COLOR"</h4>
                                    <div class="flex gap-2">
                                        {["white", "yellow", "cyan", "green", "red"].iter().map(|c| {
                                            let col = c.to_string();
                                            let col_btn = col.clone();
                                            let col_label = col.clone();
                                            let is_sel = move || sub_color.get() == col;
                                            view! {
                                                <button
                                                    on:click=move |_| set_sub_color.set(col_btn.clone())
                                                    class=move || format!(
                                                        "px-4 py-2 rounded-control text-xs uppercase font-bold border transition-colors {}",
                                                        if is_sel() { "bg-ink text-void border-ink" } else { "bg-ink/5 text-ink/70 border-ink/10 hover:border-ink/30" }
                                                    )
                                                >
                                                    {col_label.clone()}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>

                                // Opacity
                                <div>
                                    <h4 class="text-sm font-bold text-ink/60 mb-2">"BACKGROUND OPACITY"</h4>
                                    <input
                                        type="range"
                                        min="0"
                                        max="100"
                                        prop:value=sub_opacity
                                        on:input=move |e| {
                                            if let Ok(v) = event_target_value(&e).parse::<i32>() {
                                                set_sub_opacity.set(v);
                                            }
                                        }
                                        class="w-full accent-brand"
                                    />
                                </div>

                                <div class="pt-4">
                                    <button
                                        on:click=move |_| set_current_view.set(SettingsView::Overview)
                                        class="btn-ink px-8 py-2.5 rounded-control"
                                    >
                                        {crate::i18n::t("settings-done")}
                                    </button>
                                </div>
                            </div>
                        }.into_any(),

                        SettingsView::Playback => view! {
                            <div class="space-y-6 animate-fadeIn max-w-[600px]">
                                <div class="border border-ink/10 rounded-card bg-ink/5 divide-y divide-ink/10">
                                    // Autoplay next
                                    <div class="flex items-center justify-between p-5">
                                        <div>
                                            <div class="font-bold text-ink">"Autoplay next episode"</div>
                                            <div class="text-xs text-ink/40 mt-0.5">"Automatically start the next episode in a series."</div>
                                        </div>
                                        <button
                                            on:click=move |_| set_autoplay_next.update(|v| *v = !*v)
                                            class=move || format!(
                                                "relative inline-flex h-6 w-11 items-center rounded-full transition-colors {}",
                                                if autoplay_next.get() { "bg-brand" } else { "bg-ink/20" }
                                            )
                                        >
                                            <span class=move || format!(
                                                "inline-block h-4 w-4 transform rounded-full bg-ink transition-transform {}",
                                                if autoplay_next.get() { "translate-x-6" } else { "translate-x-1" }
                                            ) />
                                        </button>
                                    </div>

                                    // Autoplay previews
                                    <div class="flex items-center justify-between p-5">
                                        <div>
                                            <div class="font-bold text-ink">"Autoplay previews"</div>
                                            <div class="text-xs text-ink/40 mt-0.5">"Play trailers while browsing titles."</div>
                                        </div>
                                        <button
                                            on:click=move |_| set_autoplay_previews.update(|v| *v = !*v)
                                            class=move || format!(
                                                "relative inline-flex h-6 w-11 items-center rounded-full transition-colors {}",
                                                if autoplay_previews.get() { "bg-brand" } else { "bg-ink/20" }
                                            )
                                        >
                                            <span class=move || format!(
                                                "inline-block h-4 w-4 transform rounded-full bg-ink transition-transform {}",
                                                if autoplay_previews.get() { "translate-x-6" } else { "translate-x-1" }
                                            ) />
                                        </button>
                                    </div>

                                    // Autoplay video
                                    <div class="flex items-center justify-between p-5">
                                        <div>
                                            <div class="font-bold text-ink">"High Definition Streaming"</div>
                                            <div class="text-xs text-ink/40 mt-0.5">"Prefer 1080p and 4K streams when available."</div>
                                        </div>
                                        <button
                                            on:click=move |_| set_autoplay_video.update(|v| *v = !*v)
                                            class=move || format!(
                                                "relative inline-flex h-6 w-11 items-center rounded-full transition-colors {}",
                                                if autoplay_video.get() { "bg-brand" } else { "bg-ink/20" }
                                            )
                                        >
                                            <span class=move || format!(
                                                "inline-block h-4 w-4 transform rounded-full bg-ink transition-transform {}",
                                                if autoplay_video.get() { "translate-x-6" } else { "translate-x-1" }
                                            ) />
                                        </button>
                                    </div>
                                </div>

                                <button
                                    on:click=move |_| set_current_view.set(SettingsView::Overview)
                                    class="btn-ink px-8 py-2.5 rounded-control"
                                >
                                    {crate::i18n::t("add-edit-modal-save")}
                                </button>
                            </div>
                        }.into_any(),

                        SettingsView::ViewingActivity => view! {
                            <div class="space-y-6 animate-fadeIn max-w-[600px]">
                                <p class="text-sm text-ink/60">
                                    {crate::i18n::t("settings-viewing-history-is-saved-automatically")}
                                </p>
                                <div class="p-6 bg-ink/5 border border-ink/10 rounded-card text-center">
                                    <i class="ph-bold ph-film-slate text-3xl text-ink/30 mb-2"></i>
                                    <div class="text-ink/40 text-sm">"No recent history to display."</div>
                                </div>
                            </div>
                        }.into_any(),

                        SettingsView::Privacy => view! {
                            <div class="space-y-6 animate-fadeIn max-w-[600px]">
                                <p class="text-sm text-ink/60 leading-relaxed">
                                    {crate::i18n::t("settings-your-personal-information-viewing-habits")}
                                </p>
                                <div class="p-4 bg-ink/5 border border-ink/10 rounded-card flex items-center justify-between">
                                    <div>
                                        <div class="font-bold text-ink text-sm">"Diagnostic Reporting"</div>
                                        <div class="text-xs text-ink/40 mt-0.5">"Help improve client performance."</div>
                                    </div>
                                    <span class="text-xs font-semibold text-green-400">"Enabled"</span>
                                </div>
                            </div>
                        }.into_any(),
                    }}
                </div>
            </main>
        </div>
    }
}
