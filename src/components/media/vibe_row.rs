use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::tmdb::MediaItem;
use super::movie_card::MovieCard;

fn set_timeout_ms<F: FnOnce() + 'static>(cb: F, ms: i32) {
    if let Some(window) = web_sys::window() {
        let closure = Closure::once_into_js(cb);
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms);
    }
}

#[component]
pub fn VibeRow(
    #[prop(into)] title: String,
    #[prop(default = None)] tagline: Option<String>,
    #[prop(optional, default = Vec::new())] mood_pills: Vec<String>,
    #[prop(into)] items: Vec<MediaItem>,
) -> impl IntoView {
    let _ = (&tagline, &mood_pills);
    let scroll_ref = NodeRef::<leptos::html::Div>::new();
    let (can_scroll_left, set_can_scroll_left) = signal(false);
    let (can_scroll_right, set_can_scroll_right) = signal(true);
    let (page_index, set_page_index) = signal(0);

    let items_stored = StoredValue::new(items);

    let update_scroll_state = Callback::new(move |_: ()| {
        if let Some(el) = scroll_ref.get() {
            let cur = el.scroll_left() as f64;
            let client_w = el.client_width() as f64;
            let scroll_w = el.scroll_width() as f64;
            set_can_scroll_left.set(cur > 10.0);
            set_can_scroll_right.set(scroll_w > client_w + 10.0 && scroll_w - (cur + client_w) > 10.0);

            if client_w > 0.0 {
                let p = (cur / (client_w * 0.85)).round() as usize;
                set_page_index.set(p);
            }
        }
    });

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

    let on_scroll = {
        let update = update_scroll_state.clone();
        move |_| update.run(())
    };

    let scroll = {
        let update = update_scroll_state.clone();
        move |direction: &'static str| {
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

                let update_c = update.clone();
                set_timeout_ms(move || {
                    update_c.run(());
                }, 600);
            }
        }
    };

    let card_width_class = "netflix-card-width";

    view! {
        <div class="group relative my-3 md:my-5 space-y-1.5 z-10">
            // Row Header: Title + Pagination Indicators
            <div class="flex items-center justify-between px-[var(--app-x,56px)] mb-1">
                <h2 class="text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit tracking-wide">
                    {title.clone()}
                    <span class="text-xs text-cyan-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                        "Explore Vibe ›"
                    </span>
                </h2>

                // Netflix Pagination indicators (dashes) on hover
                {move || {
                    let count = items_stored.with_value(|it| it.len());
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

            // Horizontal Carousel Strip
            <div class="relative group/row row-scroll-outer" style="container-type: inline-size;">
                {
                    let m = items_stored.get_value();
                    if m.is_empty() {
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
                                        let title_item = item.display_title().to_string();
                                        let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                        let poster = item.poster_url("w342").unwrap_or_default();
                                        let vote_avg = item.vote_average;
                                        let match_pct = item.match_percentage;
                                        let vibe_pills = item.vibe_pills.clone();
                                        let overview = item.overview.clone();
                                        let genre_ids = item.genre_ids.clone();
                                        
                                        view! {
                                            <div 
                                                class=format!("movie-card-container relative flex-none pointer-events-auto overflow-visible rounded-[4px] md:rounded-[8px] {} aspect-video", card_width_class)
                                                style="z-index: auto;"
                                            >
                                                <MovieCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title_item
                                                    backdrop_path=backdrop
                                                    poster_path=poster
                                                    vote_average=vote_avg
                                                    overview=overview
                                                    genre_ids=genre_ids
                                                    match_percentage=match_pct
                                                    vibe_pills=vibe_pills
                                                />
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}

                                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                                </div>

                                // Left Chevron Navigation Button
                                <button
                                    on:click={
                                        let sl = scroll_left.clone();
                                        move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            sl("left");
                                        }
                                    }
                                    class=move || format!(
                                        "absolute left-0 top-2 bottom-6 w-[var(--app-x,56px)] z-30 flex items-center justify-center bg-black/40 hover:bg-black/70 text-white transition-all duration-300 pointer-events-auto rounded-r-[4px] backdrop-blur-[2px] cursor-pointer group/btn {}",
                                        if can_scroll_left.get() { "opacity-0 group-hover/row:opacity-100" } else { "opacity-0 pointer-events-none" }
                                    )
                                    aria-label="Scroll left"
                                >
                                    <svg class="w-6 h-6 md:w-8 md:h-8 transition-transform duration-200 group-hover/btn:scale-125" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7" />
                                    </svg>
                                </button>

                                // Right Chevron Navigation Button
                                <button
                                    on:click={
                                        let sr = scroll_right.clone();
                                        move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            sr("right");
                                        }
                                    }
                                    class=move || format!(
                                        "absolute right-0 top-2 bottom-6 w-[var(--app-x,56px)] z-30 flex items-center justify-center bg-black/40 hover:bg-black/70 text-white transition-all duration-300 pointer-events-auto rounded-l-[4px] backdrop-blur-[2px] cursor-pointer group/btn {}",
                                        if can_scroll_right.get() { "opacity-0 group-hover/row:opacity-100" } else { "opacity-0 pointer-events-none" }
                                    )
                                    aria-label="Scroll right"
                                >
                                    <svg class="w-6 h-6 md:w-8 md:h-8 transition-transform duration-200 group-hover/btn:scale-125" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M9 5l7 7-7 7" />
                                    </svg>
                                </button>
                            </>
                        }.into_any()
                    }
                }
            </div>
        </div>
    }
}
