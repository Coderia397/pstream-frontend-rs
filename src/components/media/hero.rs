use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::services::tmdb::{fetch_movie_logo, fetch_videos, MediaItem};
use crate::store::use_ui_store;
use crate::components::media::movie_card_badges::MaturityBadge;
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
        if let Some(ref bp) = item.backdrop_path {
            if bp.starts_with("http") {
                bp.clone()
            } else {
                format!("https://image.tmdb.org/t/p/original{}", bp)
            }
        } else {
            String::new()
        }
    };
    let current_overview = move || current_item().overview;

    // TMDB Logo resource
    let logo_resource = LocalResource::new(move || {
        let id = current_id();
        let tv = current_is_tv();
        async move {
            fetch_movie_logo(id, tv).await.ok().flatten()
        }
    });

    // TMDB Trailer video resource
    let videos_resource = LocalResource::new(move || {
        let id = current_id();
        let tv = current_is_tv();
        async move {
            fetch_videos(id, tv).await.unwrap_or_default()
        }
    });

    let trailer_key = move || {
        videos_resource.get().and_then(|videos| {
            videos.into_iter()
                .find(|v| v.site == "YouTube" && (v.r#type == "Trailer" || v.r#type == "Teaser"))
                .map(|v| v.key)
        })
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
            class="hidden md:block relative w-full aspect-video max-h-[85vh] min-h-[500px] overflow-hidden group bg-black"
        >
            // ── Background Video Layer ──────────────────────────────────────
            <div
                id="hero-video-layer"
                class="absolute inset-0 z-0 transition-opacity duration-700 overflow-hidden"
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
                                        class="pointer-events-none scale-[1.35] lg:scale-[1.15]"
                                        style="width: 100vw; height: 56.25vw; min-height: 100%; min-width: 177.77vh; border: none;"
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
                class="absolute inset-0 w-full h-full object-cover object-[50%_15%] transition-opacity duration-700 ease-in-out z-0"
                class=("opacity-0", move || is_playing())
                class=("opacity-100", move || !is_playing())
            />

            // ── Vignettes (Netflix 77deg left gradient + bottom fade) ────────
            <div
                class="absolute inset-y-0 left-0 z-10 pointer-events-none"
                style="right: 26%; background: linear-gradient(77deg, rgba(0,0,0,0.72) 0%, transparent 85%);"
            />
            <div
                class="absolute inset-0 z-10 pointer-events-none"
                style="background: linear-gradient(to top, #141414 0%, rgba(20,20,20,0.6) 14%, rgba(20,20,20,0.2) 26%, transparent 40%);"
            />

            // ── Content layer ───────────────────────────────────────────────
            <div class="absolute inset-0 z-20 pointer-events-none flex flex-col justify-end pb-[7%]">
                <div class="flex items-end justify-between px-[var(--app-x,56px)]">

                    // ── Left: Logo / Title, Synopsis, CTA Buttons ────────────
                    <div class="max-w-[60%] md:max-w-[52%] lg:max-w-[620px] flex flex-col justify-end gap-3 md:gap-4 pointer-events-auto">

                        // Logo / Title with shrink transition
                        <div
                            class=move || if is_playing() {
                                "relative flex items-end transition-transform duration-700 origin-bottom-left scale-[0.65] sm:scale-[0.7]"
                            } else {
                                "relative flex items-end transition-transform duration-700 origin-bottom-left"
                            }
                        >
                            <Suspense fallback=move || {
                                let t = current_title();
                                view! {
                                    <h1 class="text-3xl sm:text-5xl md:text-6xl font-black drop-shadow-xl leading-none text-white tracking-wide uppercase">
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
                                                class="object-contain object-bottom drop-shadow-xl"
                                                style="max-height: clamp(85px, 20vw, 210px); max-width: 100%;"
                                            />
                                        }.into_any(),
                                        _ => view! {
                                            <h1 class="text-3xl sm:text-5xl md:text-6xl font-black drop-shadow-xl leading-none text-white tracking-wide uppercase">
                                                {t}
                                            </h1>
                                        }.into_any(),
                                    })
                                }}
                            </Suspense>
                        </div>

                        // Description with collapse transition
                        <div
                            class=move || if is_playing() {
                                "transition-[opacity,max-height] duration-500 ease-in-out overflow-hidden opacity-0 max-h-0"
                            } else {
                                "transition-[opacity,max-height] duration-500 ease-in-out overflow-hidden opacity-100 max-h-40"
                            }
                        >
                            <p class="max-w-sm md:max-w-md text-[12px] sm:text-[13px] md:text-[15px] font-medium text-white/90 line-clamp-2 md:line-clamp-3 leading-relaxed">
                                {move || current_overview()}
                            </p>
                        </div>

                        // CTA buttons: Play and More Info (Task 066 frosted glass)
                        <div class="flex items-center flex-wrap gap-2 md:gap-3 mt-1">
                            <a
                                href=move || format!("/watch/{}", current_id())
                                class="flex items-center justify-center bg-white text-black px-5 sm:px-8 h-[35px] md:h-[45px] rounded-[4px] font-bold hover:bg-white/80 transition-colors duration-150 active:scale-95 text-[15px] md:text-[18px] gap-2 pointer-events-auto"
                            >
                                <i class="ph-fill ph-play text-[18px] md:text-[24px]"></i>
                                <span class="whitespace-nowrap">"Play"</span>
                            </a>

                            <button
                                on:click=move |_| trigger_open_modal()
                                class="flex items-center justify-center bg-[#6d6d6e]/50 text-white px-5 sm:px-8 h-[35px] md:h-[45px] rounded-[4px] font-bold hover:bg-[#6d6d6e]/35 backdrop-blur-sm transition-colors duration-150 active:scale-95 text-[15px] md:text-[18px] gap-2 pointer-events-auto"
                            >
                                <i class="ph-bold ph-info text-[22px] md:text-[30px]"></i>
                                <span class="whitespace-nowrap">"More Info"</span>
                            </button>
                        </div>
                    </div>

                    // ── Right: Replay / Mute, Maturity Badge ─
                    <div class="flex items-center pointer-events-auto flex-shrink-0 gap-3">

                        // Replay button / Mute toggle button (Task 071)
                        <button
                            on:click=on_mute_or_replay
                            class="w-9 h-9 border-[1.8px] border-white/70 rounded-full flex items-center justify-center transition-colors duration-200 hover:bg-white/15"
                            aria-label=move || if has_video_ended.get() { "Replay" } else if is_muted.get() { "Unmute" } else { "Mute" }
                        >
                            {move || {
                                if has_video_ended.get() {
                                    view! { <i class="ph-bold ph-arrow-counter-clockwise text-white text-lg"></i> }.into_any()
                                } else if is_muted.get() {
                                    view! { <i class="ph ph-speaker-slash text-white text-lg"></i> }.into_any()
                                } else {
                                    view! { <i class="ph ph-speaker-high text-white text-lg"></i> }.into_any()
                                }
                            }}
                        </button>

                        // Maturity badge
                        <div class="flex items-center bg-[#2e2e2e]/40 h-10 pl-4 -mr-14 lg:-mr-16 pr-14 lg:pr-16 ml-2 border-l-[3px] border-white/40">
                            <MaturityBadge
                                certification="TV-MA".to_string()
                                size="md".to_string()
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
        </>
    }
}
