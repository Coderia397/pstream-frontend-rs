use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::components::media::movie_card::MovieCard;
use crate::services::tmdb::fetch_by_language;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LangItem {
    pub code: &'static str,
    pub label: &'static str,
}

pub const BROWSE_LANGUAGES: &[LangItem] = &[
    LangItem { code: "en", label: "English" },
    LangItem { code: "es", label: "Spanish" },
    LangItem { code: "fr", label: "French" },
    LangItem { code: "de", label: "German" },
    LangItem { code: "it", label: "Italian" },
    LangItem { code: "pt", label: "Portuguese" },
    LangItem { code: "ru", label: "Russian" },
    LangItem { code: "ja", label: "Japanese" },
    LangItem { code: "ko", label: "Korean" },
    LangItem { code: "zh", label: "Chinese" },
    LangItem { code: "ar", label: "Arabic" },
    LangItem { code: "hi", label: "Hindi" },
    LangItem { code: "tr", label: "Turkish" },
    LangItem { code: "nl", label: "Dutch" },
    LangItem { code: "pl", label: "Polish" },
    LangItem { code: "sv", label: "Swedish" },
    LangItem { code: "da", label: "Danish" },
    LangItem { code: "no", label: "Norwegian" },
    LangItem { code: "th", label: "Thai" },
    LangItem { code: "id", label: "Indonesian" },
];

#[component]
pub fn BrowseLanguagePage() -> impl IntoView {
    let (selected_lang, set_selected_lang) = signal("en");
    let (dropdown_open, set_dropdown_open) = signal(false);

    let active_label = move || {
        let code = selected_lang.get();
        BROWSE_LANGUAGES
            .iter()
            .find(|l| l.code == code)
            .map(|l| l.label)
            .unwrap_or("English")
    };

    let items_resource = LocalResource::new(move || {
        let lang = selected_lang.get();
        async move {
            fetch_by_language(lang, 1).await.unwrap_or_default()
        }
    });

    let toggle_dropdown = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        set_dropdown_open.update(|o| *o = !*o);
    };

    view! {
        <Layout>
            <div class="bg-black md:bg-[#141414] min-h-screen pb-20 pt-[calc(4rem+env(safe-area-inset-top))] md:pt-28">
                <div class="px-4 md:px-10 lg:px-14 pt-0 md:pt-1">
                    // Header with language dropdown
                    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8 md:mb-12">
                        <h1 class="text-white font-normal text-[22px] md:text-[26px] tracking-tight leading-tight">
                            "Browse by Language"
                        </h1>

                        <div class="flex items-center gap-4">
                            <span class="text-white/90 text-[15px] hidden md:block select-none">
                                "Select your preferences"
                            </span>

                            <div class="relative">
                                <button
                                    on:click=toggle_dropdown
                                    class="flex items-center justify-between min-w-[180px] md:min-w-[210px] px-4 py-[7px] md:py-[8px] text-[13px] md:text-[14px] font-bold tracking-[-0.2px] text-white bg-black border border-white/80 hover:bg-white/5 transition-colors gap-x-3 cursor-pointer select-none"
                                >
                                    <span>{active_label}</span>
                                    <i class="ph-fill ph-caret-down text-white text-xs"></i>
                                </button>

                                // Dropdown menu
                                {move || {
                                    if dropdown_open.get() {
                                        view! {
                                            <div class="absolute top-full left-0 mt-1 w-full max-h-64 overflow-y-auto bg-black/95 border border-[#333] shadow-2xl z-50 scrollbar-hide py-1">
                                                {BROWSE_LANGUAGES.iter().map(|lang| {
                                                    let code = lang.code;
                                                    let label = lang.label;
                                                    let is_selected = selected_lang.get() == code;
                                                    view! {
                                                        <button
                                                            on:click=move |_| {
                                                                set_selected_lang.set(code);
                                                                set_dropdown_open.set(false);
                                                            }
                                                            class=if is_selected {
                                                                "w-full text-left px-4 py-2 text-sm text-white font-bold bg-white/10 cursor-pointer"
                                                            } else {
                                                                "w-full text-left px-4 py-2 text-sm text-[#e5e5e5] hover:text-white hover:bg-white/5 transition-colors cursor-pointer"
                                                            }
                                                        >
                                                            {label}
                                                        </button>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div /> }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>

                    // Content grid
                    <Suspense fallback=move || view! {
                        <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-6 animate-pulse">
                            {(0..15).map(|_| view! {
                                <div class="aspect-video bg-[#1e1e1e] rounded-sm border border-white/[0.04]"></div>
                            }).collect::<Vec<_>>()}
                        </div>
                    }>
                        {move || items_resource.get().map(|items| {
                            if items.is_empty() {
                                view! {
                                    <div class="flex flex-col items-center justify-center mt-24 text-center">
                                        <i class="ph ph-globe text-5xl text-white/30 mb-3"></i>
                                        <p class="text-white/50 text-lg">"No titles available for this language."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-6">
                                        {items.into_iter().map(|item| {
                                            let movie_id = item.id;
                                            let is_tv = item.is_tv();
                                            let title = item.display_title().to_string();
                                            let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                            let poster = item.poster_url("w342").unwrap_or_default();
                                            let vote_avg = item.vote_average;
                                            view! {
                                                <MovieCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    poster_path=poster
                                                    vote_average=vote_avg
                                                />
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
