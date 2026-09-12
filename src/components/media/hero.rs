use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::services::tmdb::{fetch_movie_logo, fetch_videos, fetch_details, MediaItem};
use crate::store::use_ui_store;
use crate::components::media::mobile_hero::MobileHero;
use crate::models::movie::Movie;

fn set_timeout_ms<F: FnOnce() + 'static>(cb: F, ms: i32) -> Option<i32> {
    let window = web_sys::window()?;
    let closure = Closure::once_into_js(cb);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms)
        .ok()
}

fn clear_timeout_id(id: Option<i32>) {
    if let Some(id) = id {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(id);
        }
    }
}

#[component]
fn MegaphoneBadgeIcon() -> impl IntoView {
    view! {
        <svg
            class="w-[15px] h-[15px] flex-shrink-0 drop-shadow-[0_1px_3px_rgba(255,0,100,0.5)]"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <defs>
                <linearGradient id="megaGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#ff007f" />
                    <stop offset="60%" stop-color="#ff1a40" />
                    <stop offset="100%" stop-color="#ff6b00" />
                </linearGradient>
            </defs>
            <path
                d="M4 11.5V8.5C4 7.95 4.45 7.5 5 7.5H7L13.2 3.4C13.8 3.0 14.6 3.4 14.6 4.1V15.9C14.6 16.6 13.8 17.0 13.2 16.6L7 12.5H5C4.45 12.5 4 12.05 4 11.5Z"
                fill="url(#megaGrad)"
            />
            <path
                d="M6 12.5V15.5C6 16.05 6.45 16.5 7 16.5H7.5C8.05 16.5 8.5 16.05 8.5 15.5V12.5"
                fill="url(#megaGrad)"
            />
            <path
                d="M16 7.5C16.8 8.8 16.8 11.2 16 12.5"
                stroke="url(#megaGrad)"
                stroke-width="1.8"
                stroke-linecap="round"
            />
        </svg>
    }
}

#[component]
fn ClapperboardBadgeIcon() -> impl IntoView {
    view! {
        <svg
            class="w-[15px] h-[15px] flex-shrink-0 drop-shadow-[0_1px_3px_rgba(155,93,229,0.5)]"
            viewBox="0 0 20 20"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <defs>
                <linearGradient id="clapGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="#ff007f" />
                    <stop offset="50%" stop-color="#9b5de5" />
                    <stop offset="100%" stop-color="#7209b7" />
                </linearGradient>
            </defs>
            <rect x="2.5" y="4" width="15" height="3.5" rx="1" fill="url(#clapGrad)" />
            <path d="M5.5 4L4 7.5M9.5 4L8 7.5M13.5 4L12 7.5" stroke="#ffffff" stroke-width="1.2" stroke-linecap="round" />
            <rect x="2.5" y="8" width="15" height="8" rx="1.2" fill="url(#clapGrad)" />
            <line x1="5" y1="11" x2="11" y2="11" stroke="#ffffff" stroke-opacity="0.8" stroke-width="1.2" stroke-linecap="round" />
            <line x1="5" y1="13.5" x2="9" y2="13.5" stroke="#ffffff" stroke-opacity="0.5" stroke-width="1.2" stroke-linecap="round" />
            <circle cx="14" cy="12.2" r="1.2" fill="#ffffff" fill-opacity="0.8" />
        </svg>
    }
}

#[component]
pub fn HeroSection(
    #[prop(optional)] movies: Option<Vec<MediaItem>>,
    #[prop(optional, default = 0)] movie_id: u32,
    #[prop(optional, default = false)] is_tv: bool,
    #[prop(optional, default = String::new())] title: String,
    #[prop(optional, default = String::new())] backdrop: String,
    #[prop(optional, default = String::new())] overview: String,
) -> impl IntoView {
    let ui_store = use_ui_store();

    // Prepare items list: either passed movies or synthetic single-item list
    let initial_items = if let Some(ref m) = movies {
        if !m.is_empty() {
            m.clone()
        } else {
            vec![MediaItem {
                id: movie_id,
                title: if !is_tv { Some(title.clone()) } else { None },
                name: if is_tv { Some(title.clone()) } else { None },
                overview: overview.clone(),
                backdrop_path: Some(backdrop.clone()),
                media_type: Some(if is_tv { "tv".to_string() } else { "movie".to_string() }),
                ..Default::default()
            }]
        }
    } else {
        vec![MediaItem {
            id: movie_id,
            title: if !is_tv { Some(title.clone()) } else { None },
            name: if is_tv { Some(title.clone()) } else { None },
            overview: overview.clone(),
            backdrop_path: Some(backdrop.clone()),
            media_type: Some(if is_tv { "tv".to_string() } else { "movie".to_string() }),
            ..Default::default()
        }]
    };

    let items_stored = StoredValue::new(initial_items);
    let active_index = RwSignal::new(0usize);

    let (is_muted, set_is_muted) = signal(true);
    let (show_video, set_show_video) = signal(false);
    let (is_video_ready, set_is_video_ready) = signal(false);
    let (has_video_ended, set_has_video_ended) = signal(false);
    let (replay_count, set_replay_count) = signal(0u32);
    let (is_out_of_view, set_is_out_of_view) = signal(false);

    let show_timer = StoredValue::new(None::<i32>);
    let visibility_timer = StoredValue::new(None::<i32>);

    // Get current item
    let current_item = move || {
        let idx = active_index.get_untracked();
        items_stored.with_value(|items| {
            items.get(idx).cloned().unwrap_or_default()
        })
    };

    let current_id = move || current_item().id;
    let current_is_tv = move || current_item().is_tv();
    let current_title = move || current_item().display_title().to_string();
    let current_backdrop = move || {
        let item = current_item();
        if let Some(bp) = item.backdrop_path.as_ref().or(item.poster_path.as_ref()) {
            if bp.starts_with("http") {
                bp.to_string()
            } else {
                format!("https://image.tmdb.org/t/p/original{}", bp)
            }
        } else {
            String::new()
        }
    };
    let current_overview = move || current_item().overview;

    // Ambient background color synchronization with 0ms cache & fast w92 extraction
    Effect::new(move |_| {
        let bp = current_backdrop();
        if bp.is_empty() {
            return;
        }
        // 1. Synchronous cache check (0ms)
        if let Some(rgb) = crate::utils::ambient::get_cached_ambient_color(&bp) {
            ui_store.ambient_color.set(rgb);
            crate::utils::ambient::apply_ambient_color(rgb.0, rgb.1, rgb.2);
            return;
        }

        // 2. Ultra-fast thumbnail canvas sampling (~15ms)
        crate::utils::ambient::extract_ambient_color(&bp, move |maybe_rgb| {
            if let Some(rgb) = maybe_rgb {
                ui_store.ambient_color.set(rgb);
                crate::utils::ambient::apply_ambient_color(rgb.0, rgb.1, rgb.2);
            }
        });
    });

    // TMDB Logo resource
    let logo_resource = LocalResource::new(move || {
        let id = current_id();
        let tv = current_is_tv();
        async move {
            if id > 0 {
                fetch_movie_logo(id, tv).await.ok().flatten()
            } else {
                None
            }
        }
    });

    // TMDB Trailer video resource
    let videos_resource = LocalResource::new(move || {
        let id = current_id();
        let tv = current_is_tv();
        async move {
            if id > 0 {
                fetch_videos(id, tv).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    });

    let trailer_key = move || {
        videos_resource.get().and_then(|videos| {
            videos.into_iter()
                .find(|v| v.site == "YouTube" && (v.r#type == "Trailer" || v.r#type == "Teaser"))
                .map(|v| v.key)
        })
    };

    // TMDB Details resource (genres, runtime/seasons, cast, release year)
    let details_resource = LocalResource::new(move || {
        let id = current_id();
        let tv = current_is_tv();
        async move {
            if id > 0 {
                fetch_details(id, tv).await.ok()
            } else {
                None
            }
        }
    });

    let media_type_label = move || {
        if current_is_tv() { "Series" } else { "Film" }
    };

    let genre_label = move || {
        details_resource.get().and_then(|d| {
            d.and_then(|item| item.genres.and_then(|g| g.first().map(|x| x.name.clone())))
        }).unwrap_or_else(|| "Action".to_string())
    };

    let year_label = move || {
        if let Some(Some(ref d)) = details_resource.get() {
            if let Some(ref date) = d.release_date.as_ref().or(d.first_air_date.as_ref()) {
                if date.len() >= 4 {
                    return date.chars().take(4).collect::<String>();
                }
            }
        }
        let item = current_item();
        if let Some(ref date) = item.release_date.as_ref().or(item.first_air_date.as_ref()) {
            if date.len() >= 4 {
                return date.chars().take(4).collect::<String>();
            }
        }
        "2024".to_string()
    };

    let duration_or_seasons_label = move || {
        if current_is_tv() {
            let seasons = details_resource.get()
                .and_then(|d| d.and_then(|x| x.number_of_seasons))
                .unwrap_or(1);
            format!("{} Season{}", seasons, if seasons == 1 { "" } else { "s" })
        } else {
            let runtime = details_resource.get()
                .and_then(|d| d.and_then(|x| x.runtime))
                .unwrap_or(105);
            if runtime >= 60 {
                format!("{}h {}m", runtime / 60, runtime % 60)
            } else {
                format!("{}m", runtime)
            }
        }
    };

    let maturity_rating_label = move || {
        let item = current_item();
        if item.vote_average >= 8.0 {
            "18"
        } else if item.vote_average >= 6.0 {
            "15"
        } else {
            "12"
        }
    };

    let lead_actor = move || {
        details_resource.get().and_then(|d| {
            d.and_then(|item| item.credits.and_then(|c| c.cast.first().map(|a| a.name.clone())))
        })
    };

    let primary_badge = move || {
        let yr = year_label().parse::<u32>().unwrap_or(2024);
        let vote = current_item().vote_average;
        if yr >= 2024 {
            Some(("megaphone", "Recently added".to_string()))
        } else if vote >= 7.8 {
            Some(("clapper", "Critically Acclaimed".to_string()))
        } else if vote >= 7.0 {
            Some(("megaphone", "Trending Now".to_string()))
        } else {
            let g = genre_label();
            if !g.is_empty() && g != "Action" {
                Some(("clapper", format!("Top Pick in {}", g)))
            } else {
                Some(("megaphone", "Top Pick".to_string()))
            }
        }
    };

    // 2.5-second idle trailer playback trigger (Task 069)
    let start_trailer_timer = move || {
        clear_timeout_id(show_timer.get_value());
        set_show_video.set(false);
        set_is_video_ready.set(false);
        set_has_video_ended.set(false);

        let t = set_timeout_ms(move || {
            set_show_video.set(true);
            // Brief buffer before fading in video over backdrop
            let ready_t = set_timeout_ms(move || {
                set_is_video_ready.set(true);
            }, 600);
            let _ = ready_t;
        }, 2500);
        show_timer.set_value(t);
    };

    // Trigger trailer timer on mount or when active item changes
    Effect::new(move |_| {
        let _ = active_index.get();
        let _ = replay_count.get();
        start_trailer_timer();
    });

    // 15-second tab visibility inactivity timer to conserve RAM (Task 070)
    Effect::new(move |_| {
        let on_vis_change = Closure::wrap(Box::new(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    let is_visible = js_sys::Reflect::get(&doc, &JsValue::from_str("visibilityState"))
                        .ok()
                        .and_then(|v| v.as_string())
                        .map(|s| s == "visible")
                        .unwrap_or(true);

                    if !is_visible {
                        // Tab hidden: pause/unmount video after 15 seconds to free system memory
                        let t = set_timeout_ms(move || {
                            set_show_video.set(false);
                            set_is_video_ready.set(false);
                        }, 15_000);
                        visibility_timer.set_value(t);
                    } else {
                        // Tab visible again: cancel timer and resume trailer
                        clear_timeout_id(visibility_timer.get_value());
                        visibility_timer.set_value(None);
                        if !show_video.get_untracked() {
                            let resume_t = set_timeout_ms(move || {
                                set_show_video.set(true);
                                set_is_video_ready.set(true);
                            }, 500);
                            let _ = resume_t;
                        }
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                let _ = doc.add_event_listener_with_callback(
                    "visibilitychange",
                    on_vis_change.as_ref().unchecked_ref(),
                );
            }
        }
        on_vis_change.forget();
    });

    // Scroll observer to pause when scrolled out of view (conserve RAM)
    let hero_container_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let on_scroll = Closure::wrap(Box::new(move || {
            if let Some(el) = hero_container_ref.get() {
                let rect = el.get_bounding_client_rect();
                let height = rect.height();
                if height > 0.0 {
                    let visible_px = rect.bottom().min(height) - rect.top().max(0.0);
                    let visible_fraction = (visible_px / height).max(0.0);
                    let out = visible_fraction < 0.45;
                    set_is_out_of_view.set(out);
                    if out {
                        set_show_video.set(false);
                        set_is_video_ready.set(false);
                    }
                }
            }
        }) as Box<dyn FnMut()>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback(
                "scroll",
                on_scroll.as_ref().unchecked_ref(),
            );
        }
        on_scroll.forget();
    });

    on_cleanup(move || {
        clear_timeout_id(show_timer.get_value());
        clear_timeout_id(visibility_timer.get_value());
    });

    // Replay / Mute toggle handler (Task 071)
    let on_mute_or_replay = move |_| {
        if has_video_ended.get() {
            set_has_video_ended.set(false);
            set_replay_count.update(|c| *c += 1);
        } else {
            set_is_muted.update(|m| *m = !*m);
        }
    };

    // Open modal handler
    let trigger_open_modal = move || {
        let id = current_id();
        let tv = current_is_tv();
        ui_store.info_modal_movie_id.set(Some(id));
        ui_store.info_modal_is_tv.set(tv);
        ui_store.info_modal_open.set(true);
    };

    let is_playing = move || {
        show_video.get() && is_video_ready.get() && trailer_key().is_some() && !has_video_ended.get() && !is_out_of_view.get()
    };

    view! {
        <>
            // Mobile Hero (< 768px)
            <div class="block md:hidden">
                {move || {
                    let item = current_item();
                    let movie = Movie::from(item);
                    let on_select_cb = Callback::new(move |_| {
                        trigger_open_modal();
                    });
                    view! {
                        <MobileHero
                            movie=movie
                            on_select=on_select_cb
                        />
                    }
                }}
            </div>

        // Desktop Hero (>= 768px)
        <div
            id="hero-container"
            node_ref=hero_container_ref
            class="hidden md:block w-full px-6 md:px-14 pt-16 md:pt-18 pb-2"
        >
            <div class="relative w-full aspect-[16/9] md:aspect-[1.95/1] min-h-[500px] md:min-h-[510px] max-h-[74vh] rounded-xl md:rounded-2xl overflow-hidden bg-[#181818] border border-white/[0.05] backdrop-blur-sm shadow-[inset_0_1px_0_0_rgba(255,255,255,0.12)] group">
                // ── Background Video Layer ──────────────────────────────────────
                <div
                    id="hero-video-layer"
                    class="absolute inset-0 z-0 transition-opacity duration-700 overflow-hidden pointer-events-none"
                    class=("opacity-100", move || is_playing())
                    class=("opacity-0", move || !is_playing())
                >
                    {move || {
                        if show_video.get() && !is_out_of_view.get() {
                            if let Some(key) = trailer_key() {
                                let mute_val = if is_muted.get() { "1" } else { "0" };
                                let embed_url = format!(
                                    "https://www.youtube-nocookie.com/embed/{}?autoplay=1&mute={}&controls=0&modestbranding=1&rel=0&playsinline=1&enablejsapi=1&playlist={}",
                                    key, mute_val, key
                                );
                                view! {
                                    <div class="w-full h-full pointer-events-none flex items-center justify-center overflow-hidden">
                                        <iframe
                                            class="pointer-events-none w-[115%] h-[115%] object-cover scale-[1.2]"
                                            style="border: none;"
                                            src=embed_url
                                            allow="autoplay; encrypted-media"
                                            tabindex="-1"
                                        />
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div /> }.into_any()
                            }
                        } else {
                            view! { <div /> }.into_any()
                        }
                    }}
                </div>

                // ── Backdrop image with smooth cross-fade ────────────────────────
                <img
                    src=move || current_backdrop()
                    fetchpriority="high"
                    loading="eager"
                    alt=move || current_title()
                    class="absolute inset-0 w-full h-full object-cover object-[50%_20%] transition-opacity duration-700 ease-in-out z-0"
                    class=("opacity-0", move || is_playing())
                    class=("opacity-100", move || !is_playing())
                />

                // ── Vignettes ───────────────────────────────────────────────────
                <div
                    class="absolute inset-0 z-10 pointer-events-none bg-gradient-to-r from-black/85 via-black/40 to-transparent"
                />
                <div
                    class="absolute inset-x-0 bottom-0 h-52 z-10 pointer-events-none bg-gradient-to-t from-black/95 via-black/40 to-transparent"
                />
                <div
                    class="absolute inset-x-0 top-0 h-24 z-10 pointer-events-none bg-gradient-to-b from-black/50 to-transparent"
                />

                // ── Top-right reload/replay button ──────────────────────────────
                <button
                    on:click=on_mute_or_replay
                    class="absolute top-5 right-5 z-20 w-9 h-9 md:w-10 md:h-10 rounded-full bg-black/40 hover:bg-black/60 border border-white/20 backdrop-blur-md flex items-center justify-center text-white/90 hover:text-white transition-all cursor-pointer select-none"
                    aria-label="Replay or mute"
                >
                    <i class="ph ph-arrow-counter-clockwise text-lg"></i>
                </button>

                // ── Bottom Content Layer ─────────────────────────────────────────
                <div class="absolute inset-x-0 bottom-0 z-20 p-6 md:p-10 lg:p-12 flex flex-col md:flex-row md:items-end md:justify-between gap-6 pointer-events-none">

                    // Left Column: Logo/Title, Meta, Synopsis, CTA Buttons
                    <div class="max-w-2xl flex flex-col items-start gap-2 md:gap-2.5 pointer-events-auto">

                        // Logo / Title with scale transition
                        <div
                            class=move || if is_playing() {
                                "relative flex items-end transition-transform duration-700 origin-bottom-left scale-[0.93] sm:scale-[0.96]"
                            } else {
                                "relative flex items-end transition-transform duration-700 origin-bottom-left"
                            }
                        >
                            <Suspense fallback=move || {
                                let t = current_title();
                                view! {
                                    <h1 class="text-4xl sm:text-6xl md:text-7xl lg:text-8xl font-black drop-shadow-[0_4px_14px_rgba(0,0,0,0.95)] leading-none text-white tracking-tight uppercase mb-1.5">
                                        {t}
                                    </h1>
                                }
                            }>
                                {move || {
                                    let t = current_title();
                                    logo_resource.get().map(|res| match res {
                                        Some(url) if !url.is_empty() => view! {
                                            <img
                                                src=url
                                                alt=t.clone()
                                                class="object-contain object-bottom drop-shadow-[0_4px_14px_rgba(0,0,0,0.95)] mb-1.5"
                                                style="max-height: clamp(130px, 28vw, 260px); max-width: min(100%, 620px);"
                                            />
                                        }.into_any(),
                                        _ => view! {
                                            <h1 class="text-4xl sm:text-6xl md:text-7xl lg:text-8xl font-black drop-shadow-[0_4px_14px_rgba(0,0,0,0.95)] leading-none text-white tracking-tight uppercase mb-1.5">
                                                {t}
                                            </h1>
                                        }.into_any(),
                                    })
                                }}
                            </Suspense>
                        </div>

                        // Metadata line: Series • Action • 2017 • 5 Seasons [15]
                        <div class="flex items-center flex-wrap gap-2 text-[13px] md:text-[14px] text-white/90 font-medium select-none">
                            <span>{media_type_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{genre_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{year_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{duration_or_seasons_label}</span>
                            <span class="w-5 h-5 rounded-full bg-[#E50914] text-white text-[10px] font-bold flex items-center justify-center leading-none ml-1">
                                {maturity_rating_label}
                            </span>
                        </div>

                        // Overview / Synopsis
                        <div class="overflow-hidden transition-opacity duration-300">
                            <p class="text-white/90 text-[13px] md:text-[14.5px] font-normal leading-relaxed max-w-xl line-clamp-2 md:line-clamp-3 mb-1 select-none drop-shadow-[0_2px_6px_rgba(0,0,0,0.9)]">
                                {move || current_overview()}
                            </p>
                        </div>

                        // Action Buttons: Play (pill) & More Info (pill)
                        <div class="flex items-center gap-3 mt-1.5 pointer-events-auto">
                            <a
                                href=move || {
                                    let id = current_id();
                                    let kind = if current_is_tv() { "tv" } else { "movie" };
                                    format!("/watch/{}/{}", kind, id)
                                }
                                class="flex items-center justify-center bg-white text-black px-7 py-2.5 rounded-full font-bold hover:bg-white/90 transition-all duration-150 active:scale-95 text-[15px] md:text-[16px] gap-2 shadow-lg select-none"
                            >
                                <i class="ph-fill ph-play text-xl"></i>
                                <span>"Play"</span>
                            </a>

                            <button
                                on:click=move |_| trigger_open_modal()
                                class="flex items-center justify-center bg-white/20 hover:bg-white/30 backdrop-blur-md text-white px-7 py-2.5 rounded-full font-semibold transition-all duration-150 active:scale-95 text-[15px] md:text-[16px] shadow-lg select-none cursor-pointer"
                            >
                                <span>"More Info"</span>
                            </button>
                        </div>
                    </div>

                    // Right Column: Feature Badges (Recently added / Trending, Starring ...)
                    <div class="hidden lg:flex items-center gap-3 pb-2 flex-shrink-0 pointer-events-auto select-none">
                        {move || {
                            primary_badge().map(|(icon_type, text)| view! {
                                <div class="flex items-center gap-2 text-[12px] font-medium text-white/95 bg-black/50 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-white/15 shadow-sm">
                                    {match icon_type {
                                        "megaphone" => view! { <MegaphoneBadgeIcon /> }.into_any(),
                                        _ => view! { <ClapperboardBadgeIcon /> }.into_any(),
                                    }}
                                    <span>{text}</span>
                                </div>
                            })
                        }}

                        {move || {
                            lead_actor().map(|actor| view! {
                                <div class="flex items-center gap-2 text-[12px] font-medium text-white/95 bg-black/50 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-white/15 shadow-sm">
                                    <ClapperboardBadgeIcon />
                                    <span>{format!("Starring {}", actor)}</span>
                                </div>
                            })
                        }}
                    </div>
                </div>
            </div>
        </div>
        </>
    }
}
