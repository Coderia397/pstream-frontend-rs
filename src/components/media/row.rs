use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::movie_card::MovieCard;

fn set_timeout_ms<F: FnOnce() + 'static>(cb: F, ms: i32) {
    if let Some(window) = web_sys::window() {
        let closure = Closure::once_into_js(cb);
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms);
    }
}

#[component]
pub fn Row(
    #[prop(into)] title: String,
    #[prop(into, optional)] genre_id: Option<String>,
    #[prop(into, optional, default = "movie".to_string())] kind: String,
    #[prop(into, optional)] sort_by: Option<String>,
    #[prop(into, optional)] extra_params: Option<String>,
    #[prop(into, optional)] endpoint: Option<String>,
) -> impl IntoView {
    let (movies, set_movies) = signal::<Vec<crate::services::tmdb::MediaItem>>(Vec::new());
    let (page, set_page) = signal(1);
    let (has_more, set_has_more) = signal(true);
    let (is_fetching, set_is_fetching) = signal(false);

    let genre_id_stored = StoredValue::new(genre_id);
    let kind_stored = StoredValue::new(kind);
    let sort_by_stored = StoredValue::new(sort_by);
    let extra_params_stored = StoredValue::new(extra_params);
    let endpoint_stored = StoredValue::new(endpoint);

    let load_more = move || {
        if is_fetching.get_untracked() || !has_more.get_untracked() {
            return;
        }
        set_is_fetching.set(true);
        let current_page = page.get_untracked();
        let genre_id = genre_id_stored.get_value();
        let kind = kind_stored.get_value();
        let sort_by = sort_by_stored.get_value();
        let extra_params = extra_params_stored.get_value();
        let endpoint = endpoint_stored.get_value();
        
        leptos::task::spawn_local(async move {
            let res = crate::services::tmdb::fetch_row_content(
                &kind,
                genre_id.as_deref(),
                sort_by.as_deref(),
                extra_params.as_deref(),
                endpoint.as_deref(),
                current_page,
            ).await;
            let new_movies = res.unwrap_or_default();
            if new_movies.is_empty() {
                set_has_more.set(false);
            } else {
                // Filter out duplicates (if any) just in case
                set_movies.update(|m| {
                    for movie in new_movies {
                        if !m.iter().any(|existing| existing.id == movie.id) {
                            m.push(movie);
                        }
                    }
                });
                set_page.update(|p| *p += 1);
            }
            set_is_fetching.set(false);
        });
    };

    // Initial load - run only once
    let initial_load_done = StoredValue::new(false);
    Effect::new(move |_| {
        if !initial_load_done.get_value() {
            initial_load_done.set_value(true);
            load_more();
        }
    });

    let scroll_ref = NodeRef::<leptos::html::Div>::new();
    let (can_scroll_left, set_can_scroll_left) = signal(false);
    let (can_scroll_right, set_can_scroll_right) = signal(true);
    let (page_index, set_page_index) = signal(0usize);

    let update_scroll_state = Callback::new(move |_: ()| {
        if let Some(el) = scroll_ref.get() {
            let cur = el.scroll_left() as f64;
            let client_w = el.client_width() as f64;
            let scroll_w = el.scroll_width() as f64;

            set_can_scroll_left.set(cur > 10.0);
            set_can_scroll_right.set(scroll_w > client_w + 10.0 && scroll_w - (cur + client_w) > 10.0);

            let margin = if client_w >= 1750.0 { 60.0 } else if client_w >= 1350.0 { 56.0 } else if client_w >= 800.0 { 48.0 } else { 16.0 };
            let page_w = (client_w - 2.0 * margin + 6.0).max(100.0);
            let idx = ((cur + page_w * 0.3) / page_w).floor() as usize;
            set_page_index.set(idx);
        }
    });

    let on_scroll = {
        let update = update_scroll_state.clone();
        move |_| update.run(())
    };

    // Window resize listener
    Effect::new({
        let update = update_scroll_state.clone();
        move |_| {
            if let Some(win) = web_sys::window() {
                let update_c = update.clone();
                let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                    update_c.run(());
                }));
                let _ = win.add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    });

    // Content change / initial render listener
    Effect::new({
        let update = update_scroll_state.clone();
        move |_| {
            let _ = movies.get();
            let update_c = update.clone();
            set_timeout_ms(move || {
                update_c.run(());
            }, 100);
        }
    });

    let scroll = {
        let update = update_scroll_state.clone();
        move |direction: &str| {
            if let Some(el) = scroll_ref.get() {
                let client_width = el.client_width() as f64;
                let current_scroll = el.scroll_left() as f64;
                let scroll_width = el.scroll_width() as f64;
                
                let margin = if client_width >= 1750.0 { 60.0 } else if client_width >= 1350.0 { 56.0 } else if client_width >= 800.0 { 48.0 } else { 16.0 };
                let step = (client_width - 2.0 * margin + 6.0).max(100.0);
                
                let target = if direction == "left" {
                    (current_scroll - step).max(0.0)
                } else {
                    (current_scroll + step).min(scroll_width - client_width)
                };

                el.scroll_to_with_x_and_y(target, 0.0);

                // Proactively load next page if we're getting close to the end
                if direction == "right" && (current_scroll + client_width * 2.0) >= scroll_width {
                    load_more();
                }

                let update_c = update.clone();
                set_timeout_ms(move || {
                    update_c.run(());
                }, 600);
            }
        }
    };

    let card_width_class = "netflix-card-width";

    view! {
        <div class="group relative my-3 md:my-4 space-y-1.5 z-10">
            <div class="flex items-center justify-between px-[var(--app-x,56px)] mb-1">
                <h2 class="text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit tracking-wide">
                    {title}
                    <span class="text-xs text-cyan-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                        "Explore All ›"
                    </span>
                </h2>

                // Netflix Pagination indicators (dashes) on hover
                {move || {
                    let count = movies.get().len();
                    if count > 4 {
                        let pages = (count / 4).clamp(2, 6);
                        let current = page_index.get();
                        view! {
                            <div class="hidden md:flex items-center gap-[3px] opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                                {(0..pages).map(|i| {
                                    let is_active = i == current;
                                    view! {
                                        <div
                                            class=if is_active {
                                                "h-[2px] w-3 rounded-full bg-white transition-colors duration-200"
                                            } else {
                                                "h-[2px] w-3 rounded-full bg-white/20 transition-colors duration-200"
                                            }
                                        />
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }
                }}
            </div>

            <div class="relative group/row row-scroll-outer" style="container-type: inline-size;">
                {move || {
                    let m = movies.get();
                    if m.is_empty() && is_fetching.get() {
                        view! {
                            <div class="flex overflow-x-scroll scrollbar-hide w-full pointer-events-auto relative z-10 py-2 pb-6 gap-[6px]">
                                <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                                {(0..6).map(|_| view! {
                                    <div
                                        class=format!("movie-card-container relative flex-none {} aspect-video bg-[#1e1e1e] rounded-[4px] md:rounded-[8px] overflow-hidden border border-white/[0.04] pointer-events-auto", card_width_class)
                                    >
                                        <div class="absolute inset-0 -translate-x-full animate-[shimmer_1.8s_ease-in-out_infinite] bg-gradient-to-r from-transparent via-white/[0.05] to-transparent" />
                                        <div class="absolute inset-0 bg-gradient-to-b from-[#252525] via-[#1e1e1e] to-[#181818]" />
                                        <div class="absolute bottom-4 left-3 space-y-2">
                                            <div class="h-2.5 bg-white/[0.08] rounded-full w-16" />
                                            <div class="h-1.5 bg-white/[0.05] rounded-full w-10" />
                                        </div>
                                    </div>
                                }).collect::<Vec<_>>()}
                                <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                            </div>
                        }.into_any()
                    } else if m.is_empty() {
                        view! { <div /> }.into_any()
                    } else {
                        let scroll_left = scroll.clone();
                        let scroll_right = scroll.clone();
                        
                        view! {
                            <>
                                <div
                                    node_ref=scroll_ref
                                    on:scroll=on_scroll
                                    class="row-scroll-strip flex overflow-x-scroll scrollbar-hide w-full pointer-events-auto relative z-10 py-2 pb-6 gap-[6px]"
                                    style="scroll-behavior: smooth;"
                                >
                                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />

                                    {m.into_iter().map(|item| {
                                        let movie_id = item.id;
                                        let is_tv = item.is_tv();
                                        let title = item.display_title().to_string();
                                        let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                        let poster = item.poster_url("w342").unwrap_or_default();
                                        
                                        view! {
                                            <div 
                                                class=format!("movie-card-container relative flex-none pointer-events-auto overflow-visible rounded-[4px] md:rounded-[8px] {} aspect-video", card_width_class)
                                                style="z-index: auto;"
                                            >
                                                <MovieCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    poster_path=poster
                                                    vote_average=item.vote_average
                                                    overview=item.overview.clone()
                                                    genre_ids=item.genre_ids.clone()
                                                    vibe_pills=item.vibe_pills.clone()
                                                />
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                    
                                    {move || if is_fetching.get() {
                                        view! {
                                            <div class=format!("movie-card-container relative flex-none {} aspect-video flex items-center justify-center", card_width_class)>
                                                <div class="w-8 h-8 rounded-full border-2 border-transparent border-t-white/60 animate-spin" />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div /> }.into_any()
                                    }}

                                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                                </div>

                                // Left Chevron Handle: sits inside left margin gutter, exactly var(--app-x) wide
                                <button
                                    type="button"
                                    class=move || if can_scroll_left.get() {
                                        "hidden md:flex absolute top-2 bottom-6 left-0 z-30 items-center justify-center cursor-pointer bg-transparent hover:bg-black/70 transition-all duration-200 opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto select-none border-none outline-none"
                                    } else {
                                        "hidden !pointer-events-none !opacity-0"
                                    }
                                    style="width: var(--app-x, 56px);"
                                    on:click=move |_| scroll_left("left")
                                    aria-label="Scroll Left"
                                >
                                    <i class="ph-bold ph-caret-left text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                                </button>

                                // Right Chevron Handle: sits inside right margin gutter, exactly var(--app-x) wide
                                <button
                                    type="button"
                                    class=move || if can_scroll_right.get() {
                                        "hidden md:flex absolute top-2 bottom-6 right-0 z-30 items-center justify-center cursor-pointer bg-transparent hover:bg-black/70 transition-all duration-200 opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto select-none border-none outline-none"
                                    } else {
                                        "hidden !pointer-events-none !opacity-0"
                                    }
                                    style="width: var(--app-x, 56px);"
                                    on:click=move |_| scroll_right("right")
                                    aria-label="Scroll Right"
                                >
                                    <i class="ph-bold ph-caret-right text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                                </button>
                            </>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
