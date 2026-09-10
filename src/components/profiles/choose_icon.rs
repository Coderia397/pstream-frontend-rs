use leptos::prelude::*;
use crate::data::avatars::AVATAR_CATEGORIES;

#[component]
pub fn ChooseIconModal(
    current_avatar_url: Option<String>,
    on_select: Callback<String>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let current_url = current_avatar_url.unwrap_or_default();

    view! {
        <div class="fixed inset-0 z-[250] bg-[#141414] overflow-y-auto animate-in fade-in duration-200">
            // ── Sticky Header ───────────────────────────────────────────────
            <header class="sticky top-0 z-20 bg-[#141414]/90 backdrop-blur-md border-b border-white/10 px-6 sm:px-12 py-4 flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <button
                        on:click=move |_| on_cancel.run(())
                        class="p-2 rounded-full hover:bg-white/10 text-white/80 hover:text-white transition-colors cursor-pointer"
                        aria-label="Back"
                    >
                        <i class="ph-bold ph-arrow-left text-2xl"></i>
                    </button>
                    <div>
                        <h1 class="text-xl sm:text-2xl font-bold text-white">"Choose an Icon"</h1>
                        <p class="text-xs sm:text-sm text-white/50">"Select an avatar for your profile"</p>
                    </div>
                </div>

                <button
                    on:click=move |_| on_cancel.run(())
                    class="px-5 py-1.5 border border-white/30 rounded text-white text-sm font-semibold hover:border-white transition-colors cursor-pointer"
                >
                    "Cancel"
                </button>
            </header>

            // ── Categories & Avatars ────────────────────────────────────────
            <div class="max-w-6xl mx-auto px-6 sm:px-12 py-8 space-y-10">
                {AVATAR_CATEGORIES.iter().map(|cat| {
                    let cat_name = cat.name;
                    let avatars = cat.avatars;

                    view! {
                        <div class="space-y-4">
                            <h2 class="text-lg sm:text-xl font-bold text-white tracking-wide">
                                {cat_name}
                            </h2>
                            <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-7 gap-3 sm:gap-4">
                                {avatars.iter().map(|av| {
                                    let url = av.url.to_string();
                                    let name = av.name;
                                    let is_current = url == current_url;
                                    let on_choose = on_select;
                                    let url_select = url.clone();

                                    view! {
                                        <button
                                            type="button"
                                            on:click=move |_| on_choose.run(url_select.clone())
                                            class="flex flex-col items-center gap-2 group cursor-pointer focus:outline-none"
                                        >
                                            <div
                                                class=if is_current {
                                                    "relative aspect-square w-full rounded-md overflow-hidden bg-[#262626] transition-all duration-200 group-hover:scale-105 group-active:scale-95 ring-4 ring-white shadow-xl scale-105"
                                                } else {
                                                    "relative aspect-square w-full rounded-md overflow-hidden bg-[#262626] transition-all duration-200 group-hover:scale-105 group-active:scale-95 shadow-md hover:ring-2 hover:ring-white/60"
                                                }
                                            >
                                                <img
                                                    src=url.clone()
                                                    alt=name
                                                    class="w-full h-full object-cover"
                                                    loading="lazy"
                                                />
                                            </div>
                                            <span class="text-[11px] sm:text-xs text-white/60 group-hover:text-white transition-colors text-center truncate w-full">
                                                {name}
                                            </span>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
