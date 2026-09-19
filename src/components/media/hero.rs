use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::services::tmdb::{fetch_movie_logo, fetch_videos, fetch_details, MediaItem};
use crate::store::use_ui_store;
use crate::components::media::mobile_hero::MobileHero;
use crate::components::media::movie_card_badges::MaturityBadge;
use crate::components::media::action_buttons::{PlayPillButton, MoreInfoPillButton, MuteReplayButton};
use crate::components::trailer_player::TrailerPlayer;
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

    let (show_video, set_show_video) = signal(false);
    let (is_video_ready, set_is_video_ready) = signal(false);
    let (has_video_ended, set_has_video_ended) = signal(false);
    let (replay_count, set_replay_count) = signal(0u32);
    let (is_out_of_view, set_is_out_of_view) = signal(false);

    let show_timer = StoredValue::new(None::<i32>);
    let scroll_inactivity_timer = StoredValue::new(None::<i32>);
    let (is_unmounted_by_scroll, set_is_unmounted_by_scroll) = signal(false);

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

    let best_trailer = move || {
        videos_resource.get().and_then(|videos| {
            crate::services::tmdb::select_best_tmdb_trailer(&videos)
        })
    };

    let trailer_key = move || {
        best_trailer().map(|(k, _)| k)
    };

    let is_teaser = move || {
        best_trailer().map(|(_, t)| t).unwrap_or(false)
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
        let details_opt = details_resource.get().flatten();
        let genres_list = details_opt.as_ref().and_then(|d| d.genres.clone()).unwrap_or_default();
        let is_family_or_anim = genres_list.iter().any(|g| {
            let n = g.name.to_lowercase();
            n.contains("anim") || n.contains("family") || n.contains("child")
        }) || {
            let g = genre_label().to_lowercase();
            g.contains("anim") || g.contains("family") || g.contains("child")
        };
        let is_horror = genres_list.iter().any(|g| g.name.to_lowercase().contains("horror"))
            || genre_label().to_lowercase().contains("horror");

        if is_family_or_anim {
            if item.vote_average < 7.0 {
                "U".to_string()
            } else {
                "PG".to_string()
            }
        } else if is_horror {
            "18".to_string()
        } else if genres_list.iter().any(|g| {
            let n = g.name.to_lowercase();
            n.contains("crime") || n.contains("thriller")
        }) || genre_label().to_lowercase().contains("crime") || genre_label().to_lowercase().contains("thriller") {
            if item.vote_average >= 7.8 {
                "18".to_string()
            } else {
                "15".to_string()
            }
        } else if item.vote_average >= 8.0 {
            "18".to_string()
        } else if item.vote_average >= 6.5 {
            "15".to_string()
        } else {
            "12".to_string()
        }
    };

    // Intelligent Hero Highlights from local recommendation engine & creator vectors
    let highlight_resource = LocalResource::new(move || {
        let item = current_item();
        let id = item.id;
        let title = item.display_title().to_string();
        async move {
            crate::services::ai_engine::fetch_hero_highlight(id, Some(&title)).await
        }
    });

    // Right Badge: Creator Hook (e.g. "Directed by Denis Villeneuve", "From Creator Kane Parsons") or Marquee Stars
    let badge_right = move || {
        if let Some(opt_highlight) = highlight_resource.get() {
            if let Some(highlight) = opt_highlight {
                if let Some(hook) = highlight.primary_hook {
                    let icon = highlight.primary_icon.unwrap_or_else(|| "clapper".to_string());
                    return Some((icon, hook));
                }
                // Evaluated by AI engine with no notable creator/star hook
                return None;
            }
        }

        // Offline fallback only: check TMDb credits crew for Director/Creator
        if let Some(Some(details)) = details_resource.get() {
            if let Some(credits) = details.credits {
                if let Some(director) = credits.crew.iter().find(|c| c.job.as_deref() == Some("Director") || c.job.as_deref() == Some("Creator")) {
                    let prefix = if is_tv { "Created by" } else { "Directed by" };
                    return Some(("clapper".to_string(), format!("{} {}", prefix, director.name)));
                }
            }
        }
        None
    };

    // Left Badge: Distinction / Cinematic Hook (e.g. "Visual Masterpiece", "Viral Sci-Fi Phenomenon")
    let badge_left = move || {
        if let Some(opt_highlight) = highlight_resource.get() {
            if let Some(highlight) = opt_highlight {
                if let Some(hook) = highlight.secondary_hook {
                    let icon = highlight.secondary_icon.unwrap_or_else(|| "clapper".to_string());
                    return Some((icon, hook));
                }
                // Evaluated by AI engine with no notable distinction hook
                return None;
            }
        }

        // Offline fallback only: high critical acclaim or genuine recent release
        let yr = year_label().parse::<u32>().unwrap_or(2024);
        let vote = current_item().vote_average;
        if vote >= 8.2 {
            Some(("clapper".to_string(), "Critically Acclaimed Masterpiece".to_string()))
        } else if vote >= 7.8 {
            Some(("clapper".to_string(), "Critically Acclaimed".to_string()))
        } else if yr >= 2024 && vote >= 7.2 {
            Some(("megaphone".to_string(), "Trending Now".to_string()))
        } else {
            None
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
        }, 2500);
        show_timer.set_value(t);
    };

    // Trigger trailer timer on mount or when active item changes
    Effect::new(move |_| {
        let _ = active_index.get();
        let _ = replay_count.get();
        start_trailer_timer();
    });

    // Scroll observer: immediate pause when scrolled out of view + 15-second RAM unmount
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
                    let was_out = is_out_of_view.get_untracked();

                    if out != was_out {
                        set_is_out_of_view.set(out);

                        if out {
                            // Scrolled past hero: immediate pause
                            send_yt_command("hero-trailer-iframe", "pauseVideo");

                            // Start 15-second timer to unmount iframe & free system RAM / GPU
                            clear_timeout_id(scroll_inactivity_timer.get_value());
                            let t = set_timeout_ms(move || {
                                set_is_unmounted_by_scroll.set(true);
                                set_show_video.set(false);
                                set_is_video_ready.set(false);
                            }, 15_000);
                            scroll_inactivity_timer.set_value(t);
                        } else {
                            // Scrolled back into view
                            clear_timeout_id(scroll_inactivity_timer.get_value());
                            scroll_inactivity_timer.set_value(None);

                            if is_unmounted_by_scroll.get_untracked() {
                                // Was away > 15s: re-mount player cleanly
                                set_is_unmounted_by_scroll.set(false);
                                set_show_video.set(true);
                            } else {
                                // Was away < 15s: iframe was kept warm in DOM, resume immediately!
                                if show_video.get_untracked() && !has_video_ended.get_untracked() && !ui_store.info_modal_open.get_untracked() && ui_store.active_popup_id.get_untracked().is_none() {
                                    send_yt_command("hero-trailer-iframe", "playVideo");
                                }
                            }
                        }
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
        clear_timeout_id(scroll_inactivity_timer.get_value());
    });

    fn send_yt_command(iframe_id: &str, func: &str) {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.get_element_by_id(iframe_id) {
                    if let Ok(iframe) = el.dyn_into::<web_sys::HtmlIFrameElement>() {
                        if let Some(cw) = iframe.content_window() {
                            let cmd = format!(r#"{{"event":"command","func":"{}","args":[]}}"#, func);
                            let _ = cw.post_message(&wasm_bindgen::JsValue::from_str(&cmd), "*");
                        }
                    }
                }
            }
        }
    }


    let (hero_trailer_time, set_hero_trailer_time) = signal(0.0);

    // When video is verified playing, apply user mute preference if they had unmuted
    Effect::new(move |_| {
        if is_video_ready.get() && !ui_store.preview_muted.get() && !ui_store.info_modal_open.get() {
            send_yt_command("hero-trailer-iframe", "unMute");
        }
    });

    // Seamless Video Hand-Off: Pause on modal open, resume & sync timestamp on modal close
    Effect::new(move |_| {
        let modal_open = ui_store.info_modal_open.get();
        let paused_by_modal = ui_store.hero_paused_by_modal.get_untracked();

        if modal_open {
            // Modal just opened → immediately pause hero iframe
            send_yt_command("hero-trailer-iframe", "pauseVideo");
        } else if paused_by_modal {
            // Modal just closed → resume hero from exact timestamp
            ui_store.hero_paused_by_modal.set(false);
            let my_id = current_id();

            let mut resume_t: Option<f64> = None;

            // Use the timestamp the modal recorded when it closed
            if let Some((closed_id, final_t)) = ui_store.modal_closing_time.get_untracked() {
                ui_store.modal_closing_time.set(None);
                if closed_id == my_id && final_t > 0.0 {
                    set_hero_trailer_time.set(final_t);
                    set_has_video_ended.set(false);
                    set_show_video.set(true);
                    resume_t = Some(final_t);
                }
            }

            // Tell TrailerPlayer to seek to exact timestamp (updates internal anchor too)
            if let Some(t) = resume_t {
                ui_store.hero_resume_seek.set(Some(t));
            }

            // Resume playback smoothly in hero
            if show_video.get_untracked() && !has_video_ended.get_untracked() && !is_out_of_view.get_untracked() {
                send_yt_command("hero-trailer-iframe", "playVideo");
                let _ = set_timeout_ms(move || {
                    send_yt_command("hero-trailer-iframe", "playVideo");
                }, 80);
            }
        }
    });

    // Open modal handler with instant timestamp hand-off
    let trigger_open_modal = move || {
        let id = current_id();
        let tv = current_is_tv();
        let cur_t = hero_trailer_time.get();

        // Pause hero video while modal is active
        send_yt_command("hero-trailer-iframe", "pauseVideo");
        ui_store.hero_paused_by_modal.set(true);

        ui_store.modal_video_key.set(trailer_key());
        ui_store.modal_is_teaser.set(is_teaser());
        ui_store.modal_initial_time.set(cur_t);
        ui_store.modal_current_time.set(cur_t);
        ui_store.info_modal_movie_id.set(Some(id));
        ui_store.info_modal_is_tv.set(tv);
        ui_store.info_modal_open.set(true);
    };

    let should_play = move || {
        show_video.get()
            && trailer_key().is_some()
            && !has_video_ended.get()
            && !is_out_of_view.get()
            && !is_unmounted_by_scroll.get()
            && !ui_store.info_modal_open.get()
            && ui_store.active_popup_id.get().is_none()
    };

    let is_video_visible = move || should_play() && is_video_ready.get();

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
                <TrailerPlayer
                    video_key=Signal::derive(move || trailer_key())
                    is_teaser=Signal::derive(move || is_teaser())
                    is_playing=Signal::derive(move || should_play())
                    is_muted=ui_store.preview_muted
                    // NOTE: initial_seek_time is read ONCE at playback start (get_untracked inside TrailerPlayer).
                    // We pass hero_trailer_time as a frozen snapshot — TrailerPlayer only reads it untracked
                    // at the moment playback first starts, so this does NOT create a reactive loop.
                    // The `seek_time` prop handles all subsequent seeks (modal close hand-off).
                    initial_seek_time=Signal::derive(move || hero_trailer_time.get_untracked())
                    seek_time=Signal::derive(move || ui_store.hero_resume_seek.get())
                    variant="hero".to_string()
                    iframe_id="hero-trailer-iframe".to_string()
                    on_ready=Callback::new(move |_| {
                        set_is_video_ready.set(true);
                    })
                    on_ended=Callback::new(move |_| {
                        set_has_video_ended.set(true);
                    })
                    on_time_update=Callback::new(move |t| {
                        set_hero_trailer_time.set(t);
                        // Consume the one-shot seek signal once we confirm playback is updating
                        if ui_store.hero_resume_seek.get_untracked().is_some() {
                            ui_store.hero_resume_seek.set(None);
                        }
                    })
                />

                // ── Backdrop image with smooth cross-fade ────────────────────────
                <img
                    src=move || current_backdrop()
                    fetchpriority="high"
                    loading="eager"
                    alt=move || current_title()
                    class="absolute inset-0 w-full h-full object-cover object-[50%_20%] transition-opacity duration-700 ease-in-out z-0"
                    class=("opacity-0", move || is_video_visible())
                    class=("opacity-100", move || !is_video_visible())
                />

                // ── Vignettes (Tuned down so backdrop image is clear & vibrant) ──
                <div
                    class="absolute inset-0 z-10 pointer-events-none bg-gradient-to-r from-black/70 via-black/25 to-transparent"
                />
                <div
                    class="absolute inset-x-0 bottom-0 h-44 z-10 pointer-events-none bg-gradient-to-t from-black/80 via-black/25 to-transparent"
                />
                <div
                    class="absolute inset-x-0 top-0 h-20 z-10 pointer-events-none bg-gradient-to-b from-black/35 to-transparent"
                />

                // ── Top-right reload/replay button ──────────────────────────────
                <MuteReplayButton
                    is_muted=ui_store.preview_muted
                    is_ended=Signal::derive(move || has_video_ended.get())
                    on_toggle=Callback::new(move |_| {
                        if has_video_ended.get() {
                            set_has_video_ended.set(false);
                            set_replay_count.update(|c| *c += 1);
                        } else {
                            let new_muted = !ui_store.preview_muted.get_untracked();
                            ui_store.set_preview_muted(new_muted);
                            send_yt_command("hero-trailer-iframe", if new_muted { "mute" } else { "unMute" });
                        }
                    })
                    on_force_replay=Callback::new(move |_| {
                        set_has_video_ended.set(false);
                        set_replay_count.update(|c| *c += 1);
                        send_yt_command("hero-trailer-iframe", "seekTo");
                        send_yt_command("hero-trailer-iframe", "playVideo");
                    })
                    size="md".to_string()
                    class="absolute top-5 right-5 z-30 pointer-events-auto".to_string()
                />

                // ── Bottom Content Layer ─────────────────────────────────────────
                <div class="absolute inset-x-0 bottom-0 z-20 p-6 md:p-10 lg:p-12 flex flex-col md:flex-row md:items-end md:justify-between gap-6 pointer-events-none">

                    // Left Column: Logo/Title, Meta, Synopsis, CTA Buttons
                    <div class="max-w-2xl flex flex-col items-start gap-2 md:gap-2.5 pointer-events-auto">

                        // Logo / Title with scale transition
                        <div
                            class=move || if is_video_visible() {
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

                        // Metadata line: Series • Action • 2017 • 5 Seasons • [15]
                        <div class="flex items-center flex-wrap gap-2 text-[13px] md:text-[14px] text-white/90 font-medium select-none">
                            <span>{media_type_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{genre_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{year_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <span>{duration_or_seasons_label}</span>
                            <span class="text-white/40 text-xs">"•"</span>
                            <div class="inline-flex items-center">
                                {move || {
                                    let cert = maturity_rating_label();
                                    view! {
                                        <MaturityBadge
                                            certification=cert
                                            size="xs".to_string()
                                        />
                                    }
                                }}
                            </div>
                        </div>

                        // Overview / Synopsis
                        <div class="overflow-hidden transition-opacity duration-300">
                            <p class="text-white/90 text-[13px] md:text-[14.5px] font-normal leading-relaxed max-w-xl line-clamp-2 md:line-clamp-3 mb-1 select-none drop-shadow-[0_2px_6px_rgba(0,0,0,0.9)]">
                                {move || current_overview()}
                            </p>
                        </div>

                        // Action Buttons: Play (pill) & More Info (pill)
                        <div class="flex items-center gap-3 mt-1.5 pointer-events-auto">
                            <PlayPillButton
                                href=Signal::derive(move || {
                                    let id = current_id();
                                    let is_tv = current_is_tv();
                                    let kind = if is_tv { "tv" } else { "movie" };
                                    let watch_store = crate::store::use_watch_store();
                                    if let Some(rec) = watch_store.get_record(id, is_tv) {
                                        if is_tv && rec.season.is_some() && rec.episode.is_some() {
                                            return format!("/watch/tv/{}?season={}&episode={}", id, rec.season.unwrap(), rec.episode.unwrap());
                                        }
                                    }
                                    format!("/watch/{}/{}", kind, id)
                                })
                                is_resume=Signal::derive(move || {
                                    let id = current_id();
                                    let is_tv = current_is_tv();
                                    let watch_store = crate::store::use_watch_store();
                                    if let Some(rec) = watch_store.get_record(id, is_tv) {
                                        if rec.percentage > 0.0 {
                                            return true;
                                        }
                                    }
                                    false
                                })
                                size="lg".to_string()
                            />

                            <MoreInfoPillButton
                                on_click=Callback::new(move |_| trigger_open_modal())
                                size="lg".to_string()
                            />
                        </div>
                    </div>

                    // Right Column: Feature Badges (Intelligent creator & distinction hooks)
                    <div class="hidden lg:flex items-center gap-3 pb-2 flex-shrink-0 pointer-events-auto select-none">
                        {move || {
                            badge_left().map(|(icon_type, text)| view! {
                                <div class="flex items-center gap-2 text-[12px] font-medium text-white/95 bg-black/50 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-white/15 shadow-sm">
                                    {match icon_type.as_str() {
                                        "megaphone" => view! { <MegaphoneBadgeIcon /> }.into_any(),
                                        _ => view! { <ClapperboardBadgeIcon /> }.into_any(),
                                    }}
                                    <span>{text}</span>
                                </div>
                            })
                        }}

                        {move || {
                            badge_right().map(|(icon_type, text)| view! {
                                <div class="flex items-center gap-2 text-[12px] font-medium text-white/95 bg-black/50 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-white/15 shadow-sm">
                                    {match icon_type.as_str() {
                                        "megaphone" => view! { <MegaphoneBadgeIcon /> }.into_any(),
                                        _ => view! { <ClapperboardBadgeIcon /> }.into_any(),
                                    }}
                                    <span>{text}</span>
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
