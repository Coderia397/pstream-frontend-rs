use leptos::prelude::*;
use super::movie_card::MovieCard;

#[component]
pub fn Row(
    #[prop(into)] title: String,
    #[prop(into, optional)] genre_id: Option<String>,
    #[prop(into, optional, default = "movie".to_string())] kind: String,
) -> impl IntoView {
    let (movies, set_movies) = signal::<Vec<crate::services::tmdb::MediaItem>>(Vec::new());
    let (page, set_page) = signal(1);
    let (has_more, set_has_more) = signal(true);
    let (is_fetching, set_is_fetching) = signal(false);

    let genre_id_stored = StoredValue::new(genre_id);
    let kind_stored = StoredValue::new(kind);

    let load_more = move || {
        if is_fetching.get_untracked() || !has_more.get_untracked() {
            return;
        }
        set_is_fetching.set(true);
        let current_page = page.get_untracked();
        let genre_id = genre_id_stored.get_value();
        let kind = kind_stored.get_value();
        
        leptos::task::spawn_local(async move {
            let res = match genre_id {
                Some(ref id) => crate::services::tmdb::fetch_for_genre_route_page(id, current_page).await,
                None => crate::services::tmdb::fetch_trending_page(&kind, current_page).await,
            };
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

    let scroll = move |direction: &str| {
        if let Some(el) = scroll_ref.get() {
            let client_width = el.client_width() as f64;
            let current_scroll = el.scroll_left() as f64;
            let scroll_width = el.scroll_width() as f64;
            
            let step = client_width * 0.9;
            
            let target = if direction == "left" {
                current_scroll - step
            } else {
                current_scroll + step
            };

            el.scroll_to_with_x_and_y(target, 0.0);

            // Proactively load next page if we're getting close to the end
            if direction == "right" && (current_scroll + client_width * 2.0) >= scroll_width {
                load_more();
            }
        }
    };

    view! {
        <div class="group relative my-3 md:my-4 space-y-1 z-10">
            <div class="flex items-center justify-between px-[var(--app-x,56px)]">
                <h2 class="text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit tracking-wide">
                    {title}
                    <span class="text-xs text-cyan-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                        "Explore All ›"
                    </span>
                </h2>
            </div>

            <div class="relative group/row row-scroll-outer">
                {move || {
                    let m = movies.get();
                    if m.is_empty() && is_fetching.get() {
                        view! {
                            <div class="flex overflow-x-scroll scrollbar-hide w-full pointer-events-auto relative z-10 py-2 pb-6">
                                <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                                {(0..6).map(|_| view! {
                                    <div
                                        class="movie-card-container relative flex-none w-[calc((100vw-3rem)/2.3)] sm:w-[calc((100vw-3rem)/3.3)] md:w-[calc((100vw-3.5rem)/4.3)] lg:w-[calc((100vw-4rem)/6.6)] aspect-[7/4.20] bg-[#1e1e1e] rounded-sm overflow-hidden border border-white/[0.04] pointer-events-auto mr-0.5 md:mr-1 lg:mr-1.5"
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
                                    class="row-scroll-strip flex overflow-x-scroll scrollbar-hide w-full pointer-events-auto relative z-10 py-2 pb-6"
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
                                                class="movie-card-container relative pointer-events-auto mr-0.5 md:mr-1 lg:mr-1.5 overflow-visible"
                                                style="z-index: auto;"
                                            >
                                                <MovieCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    poster_path=poster
                                                    vote_average=item.vote_average
                                                />
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                    
                                    {move || if is_fetching.get() {
                                        view! {
                                            <div class="movie-card-container relative flex-none h-[128px] aspect-video flex items-center justify-center mr-0.5 md:mr-1 lg:mr-1.5">
                                                <div class="w-8 h-8 rounded-full border-2 border-transparent border-t-white/60 animate-spin" />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div /> }.into_any()
                                    }}

                                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                                </div>

                                // Left Chevron
                                <div
                                    class="absolute top-0 bottom-6 left-0 z-30 w-6 md:w-14 lg:w-16 items-center justify-center cursor-pointer bg-transparent hover:bg-[#141414]/70 flex group/arrow-left transition-[opacity,background-color] duration-200 rounded-r-sm opacity-0 pointer-events-none group-hover/row:opacity-100 group-hover/row:pointer-events-auto"
                                    on:click=move |_| scroll_left("left")
                                >
                                    <i class="ph-bold ph-caret-left text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                                </div>

                                // Right Chevron
                                <div
                                    class="absolute top-0 bottom-6 right-0 z-30 w-6 md:w-14 lg:w-16 items-center justify-center cursor-pointer bg-transparent hover:bg-[#141414]/70 flex group/arrow-right transition-[opacity,background-color] duration-200 pointer-events-none rounded-l-sm opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto"
                                    on:click=move |_| scroll_right("right")
                                >
                                    <i class="ph-bold ph-caret-right text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                                </div>
                            </>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
