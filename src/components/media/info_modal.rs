use leptos::prelude::*;
use leptos::html::Div;
use crate::store::{use_ui_store, use_library_store, LibraryEntry};
use crate::services::tmdb::{fetch_details, fetch_movie_logo, fetch_videos};
use crate::models::movie::Movie;
use crate::components::media::movie_card_badges::MaturityBadge;
use crate::components::media::info_modal_episodes::InfoModalEpisodes;
use crate::components::media::info_modal_recommendations::InfoModalRecommendations;
use crate::components::media::tooltip_wrapper::TooltipWrapper;

#[component]
pub fn InfoModal() -> impl IntoView {
    let ui_store = use_ui_store();
    let library_store = use_library_store();
    let is_open = ui_store.info_modal_open;
    let movie_id = ui_store.info_modal_movie_id;
    let is_tv = ui_store.info_modal_is_tv;

    let backdrop_ref = NodeRef::<Div>::new();
    let selected_season = RwSignal::new(1u32);

    // Video trailer state
    let (is_playing_trailer, set_is_playing_trailer) = signal(false);
    let (is_muted, set_is_muted) = signal(true);
    let (video_key, set_video_key) = signal(None::<String>);
    let (has_ended, set_has_ended) = signal(false);

    let on_close = move |_| {
        is_open.set(false);
        set_is_playing_trailer.set(false);
        set_video_key.set(None);
        set_has_ended.set(false);
    };

    // Task 083: Escape key dismiss
    let _ = leptos::prelude::window_event_listener(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
        if is_open.get() && ev.key() == "Escape" {
            is_open.set(false);
            set_is_playing_trailer.set(false);
            set_video_key.set(None);
            set_has_ended.set(false);
        }
    });

    // Task 084: Lock body scroll during modal presentation + scrollbar width compensation
    Effect::new(move |_| {
        let open = is_open.get();
        if let Some(w) = web_sys::window() {
            if let Some(doc) = w.document() {
                if let Some(body) = doc.body() {
                    if open {
                        let inner_w = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let client_w = doc.document_element().map(|el| el.client_width() as f64).unwrap_or(0.0);
                        let scrollbar_w = (inner_w - client_w).max(0.0);
                        let _ = body.style().set_property("overflow", "hidden");
                        if scrollbar_w > 0.0 {
                            let _ = body.style().set_property("padding-right", &format!("{}px", scrollbar_w));
                        }
                    } else {
                        let _ = body.style().set_property("overflow", "");
                        let _ = body.style().set_property("padding-right", "");
                    }
                }
            }
        }
    });

    on_cleanup(move || {
        if let Some(w) = web_sys::window() {
            if let Some(doc) = w.document() {
                if let Some(body) = doc.body() {
                    let _ = body.style().set_property("overflow", "");
                    let _ = body.style().set_property("padding-right", "");
                }
            }
        }
    });

    // Reset state when movie_id or is_tv changes
    Effect::new(move |_| {
        let _ = movie_id.get();
        let _ = is_tv.get();
        selected_season.set(1);
        set_is_playing_trailer.set(false);
        set_video_key.set(None);
        set_has_ended.set(false);
    });

    let details_resource = LocalResource::new(move || {
        let id_opt = movie_id.get();
        let is_tv_val = is_tv.get();
        async move {
            if let Some(id) = id_opt {
                fetch_details(id, is_tv_val).await.inspect_err(|e| leptos::logging::error!("Fetch details failed: {:?}", e)).ok()
            } else {
                None
            }
        }
    });

    let logo_resource = LocalResource::new(move || {
        let id_opt = movie_id.get();
        let is_tv_val = is_tv.get();
        async move {
            if let Some(id) = id_opt {
                fetch_movie_logo(id, is_tv_val).await.ok().flatten()
            } else {
                None
            }
        }
    });

    // Trailer fetch and auto-preview
    Effect::new(move |_| {
        let id_opt = movie_id.get();
        let is_tv_val = is_tv.get();
        let open = is_open.get();

        if open {
            if let Some(id) = id_opt {
                leptos::task::spawn_local(async move {
                    if let Ok(videos) = fetch_videos(id, is_tv_val).await {
                        let trailer = videos.into_iter().find(|v| {
                            v.site.to_lowercase() == "youtube"
                                && (v.r#type == "Trailer" || v.r#type == "Teaser" || v.r#type == "Clip")
                        });
                        if let Some(t) = trailer {
                            set_video_key.set(Some(t.key));
                            // Auto-play after 2s delay
                            let _ = leptos::leptos_dom::helpers::set_timeout_with_handle(
                                move || {
                                    set_is_playing_trailer.set(true);
                                },
                                std::time::Duration::from_millis(2000),
                            );
                        }
                    }
                });
            }
        }
    });

    let on_recommendation_click = Callback::new(move |rec: Movie| {
        let rec_id = rec.id_u32();
        let rec_is_tv = rec.is_tv();
        movie_id.set(Some(rec_id));
        is_tv.set(rec_is_tv);
        selected_season.set(1);
        set_is_playing_trailer.set(false);
        set_video_key.set(None);
        set_has_ended.set(false);

        if let Some(backdrop_el) = backdrop_ref.get() {
            backdrop_el.set_scroll_top(0);
        }
    });

    view! {
        // Modal Backdrop (Task 083: backdrop click dismiss)
        <div
            node_ref=backdrop_ref
            class="fixed inset-0 z-[10000] bg-black/70 flex justify-center overflow-y-auto scrollbar-hide cursor-default transition-opacity duration-300"
            class=("opacity-100", move || is_open.get())
            class=("pointer-events-auto", move || is_open.get())
            class=("opacity-0", move || !is_open.get())
            class=("pointer-events-none", move || !is_open.get())
            on:click=on_close
        >
            // Modal Container
            <div
                class="relative w-full max-w-[850px] bg-[#181818] rounded-xl shadow-2xl mt-6 md:mt-8 mb-8 overflow-hidden h-fit mx-4 ring-1 ring-white/10 transition-transform duration-300 origin-center"
                class=("scale-100", move || is_open.get())
                class=("scale-95", move || !is_open.get())
                on:click=|e| e.stop_propagation()
            >
                // Floating Close Button
                <button
                    type="button"
                    class="absolute top-4 right-4 w-10 h-10 rounded-full border border-white/40 bg-zinc-800/80 flex items-center justify-center transition-colors duration-150 hover:bg-white/15 hover:border-white z-50 cursor-pointer shadow-lg"
                    on:click=on_close
                    title="Close"
                >
                    <span class="text-white text-xl">"✕"</span>
                </button>

                <Suspense fallback=move || view! { <div class="w-full h-[50vh] bg-[#181818] animate-pulse"></div> }>
                    {move || details_resource.get().map(|res| match res {
                        Some(details) => {
                            let title = details.display_title().to_string();
                            let fallback_title = title.clone();
                            let backdrop = details.backdrop_path.as_ref().map(|p| format!("https://image.tmdb.org/t/p/original{}", p)).unwrap_or_default();
                            let overview = details.overview;
                            let release_year = details.release_date.or(details.first_air_date)
                                .map(|d| d.chars().take(4).collect::<String>()).unwrap_or_default();
                            let runtime = details.runtime.map(|r| format!("{}h {}m", r / 60, r % 60));
                            let seasons_count = details.number_of_seasons.unwrap_or(0);
                            let seasons_label = if seasons_count > 0 {
                                Some(format!("{} {}", seasons_count, if seasons_count == 1 { "Season" } else { "Seasons" }))
                            } else {
                                None
                            };
                            let duration = runtime.or(seasons_label).unwrap_or_default();
                            let cast_items = details.credits.map(|c| c.cast.into_iter().take(4).map(|cm| cm.name).collect::<Vec<_>>()).unwrap_or_default();
                            let genres_items = details.genres.map(|g| g.into_iter().map(|gx| (gx.id, gx.name)).collect::<Vec<_>>()).unwrap_or_default();
                            let match_score = format!("{:.0}% Match", (details.vote_average * 10.0).max(60.0).min(99.0));

                            let current_movie_id = details.id;
                            let is_tv_val = details.number_of_seasons.is_some();
                            let certification_str = if details.vote_average >= 8.0 { "18" } else if details.vote_average >= 6.5 { "15" } else { "12" }.to_string();

                            let is_in_list = Signal::derive(move || {
                                library_store.my_list.get().contains_key(&current_movie_id)
                            });

                            let toggle_my_list = {
                                let title_c = title.clone();
                                let backdrop_c = details.backdrop_path.clone();
                                let overview_c = overview.clone();
                                move |e: leptos::ev::MouseEvent| {
                                    e.stop_propagation();
                                    let mut list = library_store.my_list.get();
                                    if list.contains_key(&current_movie_id) {
                                        list.remove(&current_movie_id);
                                    } else {
                                        list.insert(current_movie_id, LibraryEntry {
                                            media: crate::services::tmdb::MediaItem {
                                                id: current_movie_id,
                                                title: Some(title_c.clone()),
                                                name: Some(title_c.clone()),
                                                overview: overview_c.clone(),
                                                poster_path: backdrop_c.clone(),
                                                backdrop_path: backdrop_c.clone(),
                                                vote_average: details.vote_average,
                                                release_date: None,
                                                first_air_date: None,
                                                media_type: if is_tv_val { Some("tv".to_string()) } else { Some("movie".to_string()) },
                                            },
                                            added_at: js_sys::Date::now() as u64,
                                        });
                                    }
                                    library_store.my_list.set(list);
                                }
                            };

                            let watch_store = crate::store::use_watch_store();
                            let watch_record = watch_store.get_record(current_movie_id, is_tv_val);
                            let has_progress = watch_record.as_ref().map(|r| r.percentage > 0.0).unwrap_or(false);
                            let play_label = if has_progress { "Resume" } else { "Play" };

                            let watch_url = if is_tv_val {
                                if let Some(ref rec) = watch_record {
                                    if let (Some(s), Some(e)) = (rec.season, rec.episode) {
                                        format!("/watch/tv/{}?season={}&episode={}", current_movie_id, s, e)
                                    } else {
                                        format!("/watch/tv/{}?season={}&episode=1", current_movie_id, selected_season.get())
                                    }
                                } else {
                                    format!("/watch/tv/{}?season={}&episode=1", current_movie_id, selected_season.get())
                                }
                            } else {
                                format!("/watch/movie/{}", current_movie_id)
                            };

                            view! {
                                <>
                                    // Hero Banner Container
                                    <div class="relative aspect-[16/8] max-sm:aspect-video w-full bg-black group overflow-hidden">
                                        // Static Backdrop Image
                                        <img
                                            src=backdrop
                                            class="w-full h-full object-cover scale-[1.05] transition-opacity duration-700"
                                            class=("opacity-0", move || is_playing_trailer.get())
                                            class=("opacity-100", move || !is_playing_trailer.get())
                                            alt=fallback_title.clone()
                                        />

                                        // Video Trailer Layer (Iframe)
                                        {move || {
                                            if let Some(key) = video_key.get() {
                                                let muted_param = if is_muted.get() { 1 } else { 0 };
                                                let embed_url = format!(
                                                    "https://www.youtube-nocookie.com/embed/{}?autoplay=1&mute={}&controls=0&modestbranding=1&rel=0&iv_load_policy=3&enablejsapi=1&loop=1&playlist={}",
                                                    key, muted_param, key
                                                );
                                                view! {
                                                    <div
                                                        class="absolute inset-0 transition-opacity duration-700 pointer-events-none overflow-hidden"
                                                        class=("opacity-100", move || is_playing_trailer.get())
                                                        class=("opacity-0", move || !is_playing_trailer.get())
                                                    >
                                                        <iframe
                                                            src=embed_url
                                                            class="w-[150%] h-[150%] -top-[25%] -left-[25%] absolute pointer-events-none border-0"
                                                            allow="autoplay; encrypted-media"
                                                            title="trailer"
                                                        />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span /> }.into_any()
                                            }
                                        }}

                                        // Cinematic Gradient Overlay (bottom 40% blend)
                                        <div class="absolute inset-x-0 bottom-0 h-2/5 bg-gradient-to-t from-[#181818] via-[#181818]/40 to-transparent z-10 pointer-events-none" />

                                        // Watch Progress Bar along the bottom of the hero banner
                                        {if has_progress {
                                            if let Some(ref rec) = watch_record {
                                                let pct = rec.percentage.clamp(0.0, 100.0);
                                                view! {
                                                    <div class="absolute bottom-0 inset-x-0 h-1.5 bg-white/20 z-30 pointer-events-none">
                                                        <div class="h-full bg-[#e50914] transition-all duration-300" style=format!("width: {:.1}%;", pct) />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span /> }.into_any()
                                            }
                                        } else {
                                            view! { <span /> }.into_any()
                                        }}

                                        // Title / Logo & Action Buttons Overlay
                                        <div class="absolute bottom-6 sm:bottom-10 left-6 sm:left-10 w-[70%] z-20 pointer-events-auto">
                                            // Logo or Title
                                            <div class="relative flex items-end mb-4">
                                                <Suspense fallback=move || view! { <h2 class="text-white text-3xl sm:text-4xl font-bold mb-4">{fallback_title.clone()}</h2> }>
                                                    {
                                                        let tc = title.clone();
                                                        move || {
                                                            let t = tc.clone();
                                                            logo_resource.get().map(|res| match res {
                                                                Some(url) if !url.is_empty() => view! {
                                                                    <img
                                                                        src=url.clone()
                                                                        alt=t.clone()
                                                                        class="object-contain object-bottom drop-shadow-xl"
                                                                        style="max-height: clamp(65px, 15vw, 120px); max-width: 100%;"
                                                                    />
                                                                }.into_any(),
                                                                _ => view! { <h2 class="text-white text-3xl sm:text-4xl md:text-5xl font-black font-leaner drop-shadow-xl leading-none tracking-wide">{t.clone()}</h2> }.into_any()
                                                            })
                                                        }
                                                    }
                                                </Suspense>
                                            </div>

                                            // CTA Action Buttons
                                            <div class="flex items-center flex-wrap gap-2 sm:gap-3">
                                                <a
                                                    href=watch_url
                                                    class="bg-white text-black px-6 sm:px-8 py-2 rounded-[4px] font-bold text-base sm:text-lg flex items-center justify-center gap-2 hover:bg-white/80 transition-colors cursor-pointer"
                                                >
                                                    <i class="ph-fill ph-play text-xl"></i>
                                                    {play_label}
                                                </a>

                                                // MyList Button
                                                <TooltipWrapper label="Add to My List".to_string()>
                                                    <button
                                                        type="button"
                                                        on:click=toggle_my_list
                                                        title=move || if is_in_list.get() { "Remove from My List" } else { "Add to My List" }
                                                        class="border border-white/40 bg-zinc-800/80 rounded-full w-10 h-10 flex items-center justify-center text-white hover:bg-white/15 hover:border-white transition-colors duration-150 cursor-pointer"
                                                    >
                                                        <i class=move || if is_in_list.get() { "ph-bold ph-check text-xl" } else { "ph-bold ph-plus text-xl" }></i>
                                                    </button>
                                                </TooltipWrapper>

                                                // Thumbs Up / Rate Button
                                                <TooltipWrapper label="I like this".to_string()>
                                                    <button
                                                        type="button"
                                                        class="border border-white/40 bg-zinc-800/80 rounded-full w-10 h-10 flex items-center justify-center text-white hover:bg-white/15 hover:border-white transition-colors duration-150 cursor-pointer"
                                                    >
                                                        <i class="ph ph-thumbs-up text-xl"></i>
                                                    </button>
                                                </TooltipWrapper>
                                            </div>
                                        </div>

                                        // Mute / Replay Button
                                        {move || if video_key.get().is_some() {
                                            view! {
                                                <button
                                                    type="button"
                                                    on:click=move |e| {
                                                        e.stop_propagation();
                                                        if has_ended.get() {
                                                            set_has_ended.set(false);
                                                            set_is_playing_trailer.set(true);
                                                        } else {
                                                            set_is_muted.update(|m| *m = !*m);
                                                        }
                                                    }
                                                    class="absolute bottom-6 right-6 z-30 w-10 h-10 rounded-full border border-white/40 bg-zinc-800/80 flex items-center justify-center transition-colors duration-150 hover:bg-white/15 hover:border-white shadow-xl pointer-events-auto cursor-pointer"
                                                    title=move || if is_muted.get() { "Unmute" } else { "Mute" }
                                                >
                                                    <i class=move || {
                                                        if has_ended.get() {
                                                            "ph ph-arrow-counter-clockwise text-white text-lg"
                                                        } else if is_muted.get() {
                                                            "ph ph-speaker-slash text-white text-lg"
                                                        } else {
                                                            "ph ph-speaker-high text-white text-lg"
                                                        }
                                                    }></i>
                                                </button>
                                            }.into_any()
                                        } else {
                                            view! { <span /> }.into_any()
                                        }}
                                    </div>

                                    // Content Details Section
                                    <div class="px-6 md:px-12 pb-12 bg-[#181818]">
                                        <div class="grid grid-cols-1 md:grid-cols-[3fr_1fr] gap-x-10 gap-y-5 pt-4">
                                            // Left Column: Match, Year, HD, Duration, Maturity, Overview
                                            <div class="space-y-4">
                                                <div class="flex flex-wrap items-center gap-x-3 gap-y-2 text-white font-bold text-sm md:text-base font-netflix">
                                                    <span class="text-[#46d369] font-extrabold tracking-wide">{match_score}</span>
                                                    <span class="text-gray-300 tracking-wide">{release_year}</span>
                                                    <span class="border border-gray-500/70 px-1.5 text-gray-300 text-xs flex items-center rounded-sm">"HD"</span>
                                                    <span class="text-gray-300 tracking-wide">{duration}</span>
                                                </div>

                                                <div class="flex items-center gap-3">
                                                    <MaturityBadge
                                                        certification=certification_str
                                                        size="md".to_string()
                                                    />
                                                </div>

                                                <p class="text-white font-normal text-[14px] md:text-[15px] leading-[1.65] pt-1">
                                                    {overview}
                                                </p>
                                            </div>

                                            // Right Column: Cast, Genres, Tags
                                            <div class="space-y-4 pt-1 text-sm">
                                                {if !cast_items.is_empty() {
                                                    let cast_rendered = cast_items.iter().map(|actor| {
                                                        let a = actor.clone();
                                                            let query_str = String::from(js_sys::encode_uri_component(&a));
                                                            view! {
                                                                <a
                                                                    href=format!("/search?q={}", query_str)
                                                                    class="text-white font-semibold hover:underline cursor-pointer"
                                                                >
                                                                    {a}
                                                                </a>
                                                            }
                                                    }).collect_view();

                                                    view! {
                                                        <div class="flex flex-wrap gap-x-1">
                                                            <span class="text-[#777] font-semibold mr-1">"Cast: "</span>
                                                            {cast_rendered}
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span /> }.into_any()
                                                }}

                                                {if !genres_items.is_empty() {
                                                    let genres_rendered = genres_items.iter().map(|(gid, gname)| {
                                                        let gn = gname.clone();
                                                        let target_route = format!("/browse/genre-{}", gid);
                                                        view! {
                                                            <a
                                                                href=target_route
                                                                class="text-white font-semibold hover:underline cursor-pointer"
                                                            >
                                                                {gn}
                                                            </a>
                                                        }
                                                    }).collect_view();

                                                    view! {
                                                        <div class="flex flex-wrap gap-x-1">
                                                            <span class="text-[#777] font-semibold mr-1">"Genres: "</span>
                                                            {genres_rendered}
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span /> }.into_any()
                                                }}

                                                <div class="flex flex-wrap gap-x-1">
                                                    <span class="text-[#777] font-semibold mr-1">"This show is: "</span>
                                                    <span class="text-white font-semibold">"Exciting, Suspenseful"</span>
                                                </div>
                                            </div>
                                        </div>

                                        // TV Shows: Episodes Selector & Episode List
                                        {if is_tv_val && seasons_count > 0 {
                                            view! {
                                                <div class="mt-8 border-t border-white/10 pt-6">
                                                    <InfoModalEpisodes
                                                        series_id=current_movie_id
                                                        total_seasons=seasons_count
                                                        selected_season=selected_season
                                                    />
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span /> }.into_any()
                                        }}

                                        // Recommendations: More Like This Grid
                                        <div class="mt-8 border-t border-white/10 pt-6">
                                            <InfoModalRecommendations
                                                movie_id=current_movie_id
                                                is_tv=is_tv_val
                                                on_recommendation_click=on_recommendation_click
                                            />
                                        </div>
                                    </div>
                                </>
                            }.into_any()
                        },
                        None => view! {
                            <div class="w-full h-40 text-white p-10 flex items-center justify-center">
                                "Error loading details."
                            </div>
                        }.into_any()
                    })}
                </Suspense>
            </div>
        </div>
    }
}
