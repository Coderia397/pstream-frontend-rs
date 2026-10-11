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
        <div class="fixed inset-0 z-(--z-modal-top) bg-page overflow-y-auto animate-in fade-in duration-(--motion-base)">
            // ── Sticky Header ───────────────────────────────────────────────
            <header class="sticky top-0 z-20 bg-page/90 backdrop-blur-md border-b border-ink/10 px-6 sm:px-12 py-4 flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <button
                        on:click=move |_| on_cancel.run(())
                        class="p-2 rounded-full hover:bg-ink/10 text-ink/80 hover:text-ink transition-colors cursor-pointer"
                        aria-label=crate::i18n::t("choose-icon-back")
                    >
                        <i class="ph-bold ph-arrow-left text-2xl"></i>
                    </button>
                    <div>
                        <h1 class="text-xl sm:text-2xl font-bold text-ink">{crate::i18n::t("profile-choose-icon")}</h1>
                        <p class="text-xs sm:text-sm text-ink/50">{crate::i18n::t("profile-choose-icon-hint")}</p>
                    </div>
                </div>

                <button
                    on:click=move |_| on_cancel.run(())
                    class="px-5 py-1.5 border border-ink/30 rounded-control text-ink text-sm font-semibold hover:border-ink transition-colors cursor-pointer"
                >
                    {crate::i18n::t("continue-watching-options-sheet-cancel")}
                </button>
            </header>

            // ── Categories & Avatars ────────────────────────────────────────
            <div class="max-w-6xl mx-auto px-6 sm:px-12 py-8 space-y-10">
                {AVATAR_CATEGORIES.iter().map(|cat| {
                    let cat_name = cat.name;
                    let avatars = cat.avatars;

                    view! {
                        <div class="space-y-4">
                            <h2 class="text-lg sm:text-xl font-bold text-ink tracking-wide">
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
                                                    "relative aspect-square w-full rounded-field overflow-hidden bg-field transition-all duration-(--motion-base) group-hover:scale-105 group-active:scale-95 ring-4 ring-ink shadow-xl scale-105"
                                                } else {
                                                    "relative aspect-square w-full rounded-field overflow-hidden bg-field transition-all duration-(--motion-base) group-hover:scale-105 group-active:scale-95 shadow-md hover:ring-2 hover:ring-ink/60"
                                                }
                                            >
                                                <img
                                                    src=url.clone()
                                                    alt=name
                                                    class="w-full h-full object-cover"
                                                    loading="lazy"
                                                />
                                            </div>
                                            <span class="text-caption sm:text-xs text-ink/60 group-hover:text-ink transition-colors text-center truncate w-full">
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
