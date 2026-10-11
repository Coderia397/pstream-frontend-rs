use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_location};
use crate::components::layout::category_sub_nav::SubNavGenre;

#[component]
pub fn CategorySubNavMobile(
    #[prop(optional)] _title: Option<String>,
    genres: Vec<SubNavGenre>,
    selected_genre: ReadSignal<Option<SubNavGenre>>,
    on_genre_select: Callback<Option<SubNavGenre>>,
    #[prop(optional)] dropdown_label: Option<String>,
) -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let (genre_menu_open, set_genre_menu_open) = signal(false);
    let stored_genres = StoredValue::new(genres);

    let is_tv_active = move || location.pathname.get() == "/browse/series";
    let is_movie_active = move || location.pathname.get() == "/browse/films";
    let is_new_active = move || location.pathname.get() == "/latest";

    let nav_series = navigate.clone();
    let on_series = move |_| {
        if is_tv_active() {
            nav_series("/browse", Default::default());
        } else {
            nav_series("/browse/series", Default::default());
        }
    };

    let nav_films = navigate.clone();
    let on_films = move |_| {
        if is_movie_active() {
            nav_films("/browse", Default::default());
        } else {
            nav_films("/browse/films", Default::default());
        }
    };

    let nav_latest = navigate.clone();
    let on_latest = move |_| {
        if is_new_active() {
            nav_latest("/browse", Default::default());
        } else {
            nav_latest("/latest", Default::default());
        }
    };

    let active_genre_label = move || {
        if let Some(g) = selected_genre.get() {
            crate::i18n::genre_label(&g.name)
        } else {
            dropdown_label.clone().unwrap_or_else(|| crate::i18n::t("nav-categories"))
        }
    };

    let on_genre_select_stored = on_genre_select;

    view! {
        <div class="relative w-full overflow-hidden select-none">
            // Horizontal scrolling pills container (Task 092)
            <div class="pt-2 pb-4 px-4 flex items-center justify-start overflow-x-auto scrollbar-hide max-w-full">
                <div class="flex items-center gap-[6px] shrink-0">
                    // Series pill
                    <button
                        on:click=on_series
                        class=move || if is_tv_active() {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink text-void border border-ink"
                        } else {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink/[0.08] text-ink-dim border border-ink/15"
                        }
                    >
                        {crate::i18n::t("category-sub-nav-mobile-series")}
                    </button>

                    // Movies pill
                    <button
                        on:click=on_films
                        class=move || if is_movie_active() {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink text-void border border-ink"
                        } else {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink/[0.08] text-ink-dim border border-ink/15"
                        }
                    >
                        {crate::i18n::t("category-sub-nav-mobile-films")}
                    </button>

                    // New & Hot pill
                    <button
                        on:click=on_latest
                        class=move || if is_new_active() {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink text-void border border-ink"
                        } else {
                            "flex items-center justify-center h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer bg-ink/[0.08] text-ink-dim border border-ink/15"
                        }
                    >
                        {crate::i18n::t("category-sub-nav-mobile-new-hot")}
                    </button>

                    // Categories / Genre Pill
                    <button
                        on:click=move |_| {
                            if selected_genre.get().is_some() {
                                on_genre_select_stored.run(None);
                            } else {
                                set_genre_menu_open.update(|o| *o = !*o);
                            }
                        }
                        class=move || if selected_genre.get().is_some() {
                            "flex items-center justify-center space-x-1 h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 leading-none shrink-0 cursor-pointer bg-ink text-void border border-ink"
                        } else {
                            "flex items-center justify-center space-x-1 h-12 px-4 rounded-modal text-body-lg font-medium whitespace-nowrap active:scale-95 leading-none shrink-0 cursor-pointer bg-ink/[0.08] text-ink-dim border border-ink/15"
                        }
                    >
                        <span>{active_genre_label}</span>
                        {move || {
                            if selected_genre.get().is_some() {
                                view! {
                                    <i class="ph-bold ph-x text-xs ml-1 opacity-80"></i>
                                }.into_any()
                            } else {
                                view! {
                                    <i class="ph-fill ph-caret-down text-xs ml-1 opacity-70"></i>
                                }.into_any()
                            }
                        }}
                    </button>
                </div>
            </div>

            // Mobile Genre Sheet Modal
            <Show when=move || genre_menu_open.get()>
                <div class="fixed inset-0 z-(--z-drawer)">
                    // Backdrop
                    <div
                        on:click=move |_| set_genre_menu_open.set(false)
                        class="absolute inset-0 bg-void/70 backdrop-blur-sm"
                    ></div>

                    // Sheet content
                    <div class="absolute bottom-0 inset-x-0 bg-page border-t border-ink/10 rounded-t-sheet max-h-[75vh] flex flex-col z-10 shadow-2xl animate-in slide-in-from-bottom duration-(--motion-base)">
                        <div class="flex items-center justify-between p-4 border-b border-ink/10">
                            <span class="text-body-lg font-bold text-ink">{crate::i18n::t("nav-categories")}</span>
                            <button
                                on:click=move |_| set_genre_menu_open.set(false)
                                class="w-8 h-8 rounded-full bg-ink/10 flex items-center justify-center text-ink/70 hover:text-ink"
                            >
                                <i class="ph-bold ph-x text-base"></i>
                            </button>
                        </div>
                        <div class="overflow-y-auto p-4 flex flex-col gap-2 scrollbar-hide">
                            {move || stored_genres.with_value(|g_list| {
                                g_list.iter().map(|genre| {
                                    let g_clone = genre.clone();
                                    let on_sel = on_genre_select_stored;
                                    view! {
                                        <button
                                            on:click=move |_| {
                                                set_genre_menu_open.set(false);
                                                on_sel.run(Some(g_clone.clone()));
                                            }
                                            class="text-left text-body-md font-medium py-2.5 px-3 rounded-control hover:bg-ink/10 text-ink transition-colors cursor-pointer"
                                        >
                                            {crate::i18n::genre_label(&genre.name)}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()
                            })}
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
