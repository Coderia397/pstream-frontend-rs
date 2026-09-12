use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::JsCast;
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

pub const PREF_TYPES: &[&'static str] = &[
    "Original Language",
    "Dubbing",
    "Subtitles",
];

#[component]
pub fn BrowseLanguagePage() -> impl IntoView {
    let (selected_pref_type, set_selected_pref_type) = signal("Original Language");
    let (pref_dropdown_open, set_pref_dropdown_open) = signal(false);

    let (selected_lang, set_selected_lang) = signal("en");
    let (lang_dropdown_open, set_lang_dropdown_open) = signal(false);

    let pref_container_ref = NodeRef::<Div>::new();
    let lang_container_ref = NodeRef::<Div>::new();

    // Close dropdowns on outside click
    Effect::new(move |_| {
        if pref_dropdown_open.get() || lang_dropdown_open.get() {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        let target = e.target();
                        if let Some(target_node) = target.and_then(|t| t.dyn_into::<web_sys::Node>().ok()) {
                            if let Some(pref_el) = pref_container_ref.get() {
                                let pref_node: &web_sys::Node = pref_el.as_ref();
                                if !pref_node.contains(Some(&target_node)) {
                                    set_pref_dropdown_open.set(false);
                                }
                            }
                            if let Some(lang_el) = lang_container_ref.get() {
                                let lang_node: &web_sys::Node = lang_el.as_ref();
                                if !lang_node.contains(Some(&target_node)) {
                                    set_lang_dropdown_open.set(false);
                                }
                            }
                        }
                    }));
                    let _ = doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
                    cb.forget();
                }
            }
        }
    });

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
            let mut all = fetch_by_language(lang, 1).await.unwrap_or_default();
            if let Ok(mut more) = fetch_by_language(lang, 2).await {
                all.append(&mut more);
            }
            all
        }
    });

    let toggle_pref_dropdown = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        set_lang_dropdown_open.set(false);
        set_pref_dropdown_open.update(|o| *o = !*o);
    };

    let toggle_lang_dropdown = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        set_pref_dropdown_open.set(false);
        set_lang_dropdown_open.update(|o| *o = !*o);
    };

    view! {
        <Layout>
            <div class="bg-black md:bg-[#141414] min-h-screen pb-20 pt-[calc(4rem+env(safe-area-inset-top))] md:pt-28">
                <div class="px-6 md:px-14 pt-2 md:pt-4">
                    // Header with title and Netflix-matching preference dropdowns
                    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
                        <h1 class="text-white font-bold text-[28px] md:text-[32px] tracking-tight leading-tight select-none">
                            "Browse by Language"
                        </h1>

                        <div class="flex items-center gap-3">
                            <span class="text-white/80 text-[14px] hidden md:block select-none whitespace-nowrap">
                                "Select your preferences"
                            </span>

                            // Dropdown 1: Preference Type (Original Language / Dubbing / Subtitles)
                            <div node_ref=pref_container_ref class="relative">
                                <button
                                    on:click=toggle_pref_dropdown
                                    class="flex items-center justify-between min-w-[160px] md:min-w-[170px] px-3.5 py-1.5 text-[13px] md:text-[14px] font-bold tracking-tight text-white bg-black border border-[#4d4d4d] hover:border-white transition-colors gap-x-2.5 cursor-pointer select-none rounded-[2px]"
                                >
                                    <span>{move || selected_pref_type.get()}</span>
                                    <i class="ph-fill ph-caret-down text-white text-[10px] transition-transform duration-200"
                                       class=("rotate-180", move || pref_dropdown_open.get())></i>
                                </button>

                                {move || {
                                    if pref_dropdown_open.get() {
                                        view! {
                                            <div class="absolute top-[calc(100%+4px)] left-0 w-full min-w-[170px] bg-black/95 border border-[#333] shadow-2xl z-50 py-1 scrollbar-hide rounded-[2px]">
                                                {PREF_TYPES.iter().map(|&t| {
                                                    let is_selected = selected_pref_type.get() == t;
                                                    view! {
                                                        <button
                                                            on:click=move |_| {
                                                                set_selected_pref_type.set(t);
                                                                set_pref_dropdown_open.set(false);
                                                            }
                                                            class=if is_selected {
                                                                "w-full text-left px-3.5 py-1.5 text-[13px] md:text-[14px] text-white font-bold bg-white/10 cursor-pointer"
                                                            } else {
                                                                "w-full text-left px-3.5 py-1.5 text-[13px] md:text-[14px] text-[#e5e5e5] hover:text-white hover:bg-white/5 transition-colors cursor-pointer"
                                                            }
                                                        >
                                                            {t}
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

                            // Dropdown 2: Language Selector
                            <div node_ref=lang_container_ref class="relative">
                                <button
                                    on:click=toggle_lang_dropdown
                                    class="flex items-center justify-between min-w-[130px] md:min-w-[150px] px-3.5 py-1.5 text-[13px] md:text-[14px] font-bold tracking-tight text-white bg-black border border-[#4d4d4d] hover:border-white transition-colors gap-x-2.5 cursor-pointer select-none rounded-[2px]"
                                >
                                    <span>{active_label}</span>
                                    <i class="ph-fill ph-caret-down text-white text-[10px] transition-transform duration-200"
                                       class=("rotate-180", move || lang_dropdown_open.get())></i>
                                </button>

                                {move || {
                                    if lang_dropdown_open.get() {
                                        view! {
                                            <div class="absolute top-[calc(100%+4px)] left-0 w-full min-w-[150px] max-h-64 overflow-y-auto bg-black/95 border border-[#333] shadow-2xl z-50 py-1 scrollbar-hide rounded-[2px]">
                                                {BROWSE_LANGUAGES.iter().map(|lang| {
                                                    let code = lang.code;
                                                    let label = lang.label;
                                                    let is_selected = selected_lang.get() == code;
                                                    view! {
                                                        <button
                                                            on:click=move |_| {
                                                                set_selected_lang.set(code);
                                                                set_lang_dropdown_open.set(false);
                                                            }
                                                            class=if is_selected {
                                                                "w-full text-left px-3.5 py-1.5 text-[13px] md:text-[14px] text-white font-bold bg-white/10 cursor-pointer"
                                                            } else {
                                                                "w-full text-left px-3.5 py-1.5 text-[13px] md:text-[14px] text-[#e5e5e5] hover:text-white hover:bg-white/5 transition-colors cursor-pointer"
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

                    // Content grid — exact 6-column Netflix desktop grid with tight horizontal gap and full aspect-video cards
                    <Suspense fallback=move || view! {
                        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-x-1.5 md:gap-x-2 gap-y-10 md:gap-y-12 animate-pulse">
                            {(0..18).map(|_| view! {
                                <div class="aspect-video bg-[#1e1e1e] rounded-[4px] border border-white/[0.04]"></div>
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
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-x-1.5 md:gap-x-2 gap-y-10 md:gap-y-12">
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
                                                    is_grid=true
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
