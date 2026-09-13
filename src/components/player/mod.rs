pub mod timeline_scrubber;
pub mod controls_overlay;
pub mod episode_drawer;
pub mod subtitle_drawer;
pub mod skip_button;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::components::hls_adapter::HlsPlayer;
use crate::services::resolver::{resolve_stream, PlayableKind, StreamResolveResult};
use crate::store::use_watch_store;

use controls_overlay::ControlsOverlay;
use episode_drawer::EpisodeDrawer;
use subtitle_drawer::SubtitleDrawer;
use skip_button::SkipButton;

#[derive(Clone, Debug, PartialEq)]
enum PlayerState {
    Resolving,
    Playing {
        result: StreamResolveResult,
        active_source_idx: usize,
    },
    Error(String),
}

fn set_timeout_handle<F: FnOnce() + 'static>(cb: F, ms: i32) -> Option<i32> {
    let window = web_sys::window()?;
    let closure = Closure::once_into_js(cb);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms)
        .ok()
}

fn clear_timeout_handle(id: Option<i32>) {
    if let Some(id) = id {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(id);
        }
    }
}

#[component]
pub fn NetflixVideoPlayer(
    media_id: u32,
    title: String,
    is_tv: bool,
    season: Option<u32>,
    episode: Option<u32>,
    episode_title: Option<String>,
    year: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    on_close: Callback<()>,
    on_episode_change: Option<Callback<(u32, u32)>>,
) -> impl IntoView {
    let watch_store = use_watch_store();
    let player_container_ref = NodeRef::<leptos::html::Div>::new();
    let video_ref = NodeRef::<leptos::html::Video>::new();

    let (player_state, set_player_state) = signal(PlayerState::Resolving);
    let (is_playing, set_is_playing) = signal(true);
    let (is_buffering, set_is_buffering) = signal(false);
    let (is_muted, set_is_muted) = signal(false);
    let (volume, set_volume) = signal(1.0);
    let (current_time, set_current_time) = signal(0.0);
    let (duration, set_duration) = signal(0.0);
    let (buffered_fraction, set_buffered_fraction) = signal(0.0);
    let (playback_speed, set_playback_speed) = signal(1.0);
    let (is_idle, set_is_idle) = signal(false);

    // Drawers
    let (is_episodes_open, set_is_episodes_open) = signal(false);
    let (is_subtitles_open, set_is_subtitles_open) = signal(false);
    let (selected_sub_url, set_selected_sub_url) = signal(None::<String>);
    let (sub_delay, set_sub_delay) = signal(0.0);

    let hls_player = RwSignal::new_local(None::<std::rc::Rc<HlsPlayer>>);
    let idle_timer = StoredValue::new(None::<i32>);
    let progress_timer = StoredValue::new(None::<i32>);

    let title_stored = StoredValue::new(title.clone());

    // Resolve Stream
    let do_resolve = Callback::new({
        let title_c = title.clone();
        let year_c = year.clone();
        move |force: bool| {
            set_player_state.set(PlayerState::Resolving);
            let t = title_c.clone();
            let y = year_c.clone();
            leptos::task::spawn_local(async move {
                match resolve_stream(media_id, is_tv, &t, y.as_deref(), season, episode, force).await {
                    Ok(result) => {
                        set_player_state.set(PlayerState::Playing {
                            result,
                            active_source_idx: 0,
                        });
                    }
                    Err(e) => {
                        set_player_state.set(PlayerState::Error(e));
                    }
                }
            });
        }
    });

    // Initial resolve
    Effect::new(move |_| {
        do_resolve.run(false);
    });

    // Idle timer reset
    let reset_idle_timer = move || {
        set_is_idle.set(false);
        clear_timeout_handle(idle_timer.get_value());
        let t = set_timeout_handle(move || {
            set_is_idle.set(true);
        }, 2500);
        idle_timer.set_value(t);
    };

    let on_mouse_move = move |_| {
        reset_idle_timer();
    };

    // Attach stream when video element and player_state ready
    Effect::new(move |_| {
        if let PlayerState::Playing { ref result, active_source_idx } = player_state.get() {
                if let Some(source) = result.sources.get(active_source_idx) {
                    if source.kind != PlayableKind::Embed {
                        if let Some(video) = video_ref.get() {
                            let stream_url = &source.url;
                            let player = HlsPlayer::attach(video.clone(), stream_url);
                            hls_player.set(Some(std::rc::Rc::new(player)));

                            // Restore resume time
                            let resume_t = watch_store.get_resume_time(media_id, is_tv, season, episode);
                            if resume_t > 0.0 {
                                video.set_current_time(resume_t);
                                set_current_time.set(resume_t);
                            }

                            let _ = video.play();
                            set_is_playing.set(true);
                        }
                    }
                }
            }
        }
    );

    // Progress persistence loop
    {
        let title_c = title.clone();
        let poster_c = poster_path.clone();
        let backdrop_c = backdrop_path.clone();
        let ep_title_c = episode_title.clone();

        Effect::new(move |_| {
            if is_playing.get() {
                clear_timeout_handle(progress_timer.get_value());
                let t_title = title_c.clone();
                let t_poster = poster_c.clone();
                let t_backdrop = backdrop_c.clone();
                let t_ep_title = ep_title_c.clone();

                let handle = set_timeout_handle(move || {
                    let cur = current_time.get_untracked();
                    let dur = duration.get_untracked();
                    if cur > 5.0 && dur > 0.0 {
                        watch_store.save_progress(
                            media_id,
                            is_tv,
                            t_title,
                            t_poster,
                            t_backdrop,
                            season,
                            episode,
                            t_ep_title,
                            cur,
                            dur,
                        );
                    }
                }, 4000);
                progress_timer.set_value(handle);
            }
        });
    }

    // Play/Pause Action
    let do_toggle_play = move || {
        reset_idle_timer();
        if let Some(video) = video_ref.get() {
            if video.paused() {
                let _ = video.play();
                set_is_playing.set(true);
            } else {
                let _ = video.pause();
                set_is_playing.set(false);
            }
        }
    };

    // Seek Action
    let do_seek = move |target_secs: f64| {
        reset_idle_timer();
        if let Some(video) = video_ref.get() {
            let clamped = target_secs.clamp(0.0, duration.get());
            video.set_current_time(clamped);
            set_current_time.set(clamped);
        }
    };

    // Skip Relative Action
    let do_skip_by = move |delta_secs: f64| {
        let current = current_time.get();
        do_seek(current + delta_secs);
    };

    // Mute Toggle
    let do_toggle_mute = move || {
        reset_idle_timer();
        if let Some(video) = video_ref.get() {
            let new_mute = !video.muted();
            video.set_muted(new_mute);
            set_is_muted.set(new_mute);
        }
    };

    // Set Volume
    let do_set_volume = move |vol: f64| {
        reset_idle_timer();
        let clamped = vol.clamp(0.0, 1.0);
        if let Some(video) = video_ref.get() {
            video.set_volume(clamped);
            if clamped == 0.0 {
                video.set_muted(true);
                set_is_muted.set(true);
            } else if video.muted() {
                video.set_muted(false);
                set_is_muted.set(false);
            }
            set_volume.set(clamped);
        }
    };

    // Set Playback Speed
    let do_set_speed = move |spd: f64| {
        reset_idle_timer();
        if let Some(video) = video_ref.get() {
            video.set_playback_rate(spd);
            set_playback_speed.set(spd);
        }
    };

    // Fullscreen Toggle
    let do_toggle_fullscreen = move || {
        reset_idle_timer();
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if doc.fullscreen_element().is_some() {
                    let _ = doc.exit_fullscreen();
                } else if let Some(el) = player_container_ref.get() {
                    let _ = el.request_fullscreen();
                }
            }
        }
    };

    // Video Element Event Handlers
    let on_time_update = move |_| {
        if let Some(video) = video_ref.get() {
            let cur = video.current_time();
            set_current_time.set(cur);
            let dur = video.duration();
            if !dur.is_nan() && dur > 0.0 {
                set_duration.set(dur);
                let frac = ((cur + 60.0) / dur).clamp(0.0, 1.0);
                set_buffered_fraction.set(frac);
            }
        }
    };

    let on_waiting = move |_| set_is_buffering.set(true);
    let on_playing = move |_| {
        set_is_buffering.set(false);
        set_is_playing.set(true);
    };
    let on_pause = move |_| set_is_playing.set(false);

    // Keyboard shortcuts
    Effect::new(move |_| {
        let on_keydown = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let key = e.key();
            match key.as_str() {
                " " | "k" | "K" => {
                    e.prevent_default();
                    do_toggle_play();
                }
                "ArrowLeft" | "j" | "J" => {
                    e.prevent_default();
                    do_skip_by(-10.0);
                }
                "ArrowRight" | "l" | "L" => {
                    e.prevent_default();
                    do_skip_by(10.0);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    do_set_volume(volume.get_untracked() + 0.10);
                }
                "ArrowDown" => {
                    e.prevent_default();
                    do_set_volume(volume.get_untracked() - 0.10);
                }
                "m" | "M" => {
                    e.prevent_default();
                    do_toggle_mute();
                }
                "f" | "F" => {
                    e.prevent_default();
                    do_toggle_fullscreen();
                }
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref());
        }
        on_keydown.forget();
    });

    on_cleanup(move || {
        clear_timeout_handle(idle_timer.get_value());
        clear_timeout_handle(progress_timer.get_value());
    });

    // Subtitle Display Logic
    let current_subtitle_tracks = move || {
        if let PlayerState::Playing { ref result, .. } = player_state.get() {
            result.subtitles.clone()
        } else {
            vec![]
        }
    };

    let subtitle_header_text = move || {
        if is_tv {
            format!(
                "S{}:E{} \"{}\"",
                season.unwrap_or(1),
                episode.unwrap_or(1),
                episode_title.as_deref().unwrap_or("Episode")
            )
        } else {
            year.clone().unwrap_or_default()
        }
    };

    view! {
        <div
            node_ref=player_container_ref
            on:mousemove=on_mouse_move
            class="relative w-full h-screen bg-black overflow-hidden select-none"
        >
            {move || match player_state.get() {
                PlayerState::Resolving => view! {
                    <div class="absolute inset-0 flex flex-col items-center justify-center bg-black z-30">
                        <div class="w-16 h-16 rounded-full border-4 border-[#e50914] border-t-transparent animate-spin mb-4 shadow-2xl"></div>
                        <p class="text-white/90 text-sm font-semibold tracking-wide animate-pulse">
                            "Finding optimal stream..."
                        </p>
                    </div>
                }.into_any(),

                PlayerState::Error(err_msg) => view! {
                    <div class="absolute inset-0 flex flex-col items-center justify-center bg-black z-30 text-center px-6">
                        <div class="w-16 h-16 rounded-2xl bg-white/10 flex items-center justify-center mb-4 text-[#e50914] text-3xl">
                            <i class="ph ph-warning-circle"></i>
                        </div>
                        <h3 class="text-xl font-bold text-white mb-2">"Playback Unavailable"</h3>
                        <p class="text-white/60 text-sm max-w-md mb-6">{err_msg}</p>
                        <div class="flex items-center gap-4">
                            <button
                                on:click=move |_| do_resolve.run(true)
                                class="px-6 py-2.5 bg-[#e50914] hover:bg-[#b80710] text-white font-bold rounded-lg transition-colors text-sm"
                            >
                                "Retry"
                            </button>
                            <button
                                on:click=move |_| on_close.run(())
                                class="px-6 py-2.5 bg-white/15 hover:bg-white/25 text-white font-semibold rounded-lg transition-colors text-sm"
                            >
                                "Back to Browse"
                            </button>
                        </div>
                    </div>
                }.into_any(),

                PlayerState::Playing { result, active_source_idx } => {
                    let source = result.sources.get(active_source_idx).cloned();
                    let is_embed = source.as_ref().map_or(false, |s| s.kind == PlayableKind::Embed);

                    view! {
                        <>
                            {if is_embed {
                                let embed_url = source.as_ref().map(|s| s.url.clone()).unwrap_or_default();
                                view! {
                                    <div class="w-full h-full relative">
                                        <iframe
                                            src=embed_url
                                            class="w-full h-full border-0"
                                            allow="autoplay; fullscreen; encrypted-media"
                                            allowfullscreen=true
                                        />
                                        // Top back bar overlay for embed fallback
                                        <div class="absolute top-4 left-4 z-30">
                                            <button
                                                on:click=move |_| on_close.run(())
                                                class="flex items-center gap-2 px-4 py-2 rounded-full bg-black/80 hover:bg-black text-white text-sm font-semibold backdrop-blur-md shadow-lg"
                                            >
                                                <i class="ph ph-arrow-left text-lg"></i>
                                                <span>"Back"</span>
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="relative w-full h-full flex items-center justify-center bg-black">
                                        <video
                                            node_ref=video_ref
                                            id="pstream-video"
                                            on:timeupdate=on_time_update
                                            on:waiting=on_waiting
                                            on:playing=on_playing
                                            on:pause=on_pause
                                            on:click=move |_| do_toggle_play()
                                            class="w-full h-full object-contain cursor-pointer"
                                            playsinline=true
                                        />

                                        // Controls Overlay
                                        <ControlsOverlay
                                            title=title_stored.get_value()
                                            subtitle=Some(subtitle_header_text())
                                            is_tv=is_tv
                                            is_playing=is_playing
                                            is_buffering=is_buffering
                                            is_muted=is_muted
                                            volume=volume
                                            current_time=current_time
                                            duration=duration
                                            buffered_fraction=buffered_fraction
                                            playback_speed=playback_speed
                                            is_idle=is_idle
                                            on_toggle_play=Callback::new(move |_| do_toggle_play())
                                            on_seek=Callback::new(move |t| do_seek(t))
                                            on_skip_by=Callback::new(move |d| do_skip_by(d))
                                            on_toggle_mute=Callback::new(move |_| do_toggle_mute())
                                            on_set_volume=Callback::new(move |v| do_set_volume(v))
                                            on_set_speed=Callback::new(move |s| do_set_speed(s))
                                            on_toggle_episodes=Callback::new(move |_| set_is_episodes_open.update(|o| *o = !*o))
                                            on_toggle_subtitles=Callback::new(move |_| set_is_subtitles_open.update(|o| *o = !*o))
                                            on_next_episode=on_episode_change.map(|cb| Callback::new(move |_| {
                                                let next_ep = episode.unwrap_or(1) + 1;
                                                let curr_s = season.unwrap_or(1);
                                                cb.run((curr_s, next_ep));
                                            }))
                                            on_toggle_fullscreen=Callback::new(move |_| do_toggle_fullscreen())
                                            on_back=on_close
                                        />

                                        // Skip Intro Button
                                        <SkipButton
                                            current_time=current_time
                                            duration=duration
                                            is_tv=is_tv
                                            on_skip_intro=Callback::new(move |_| do_skip_by(85.0))
                                            on_next_episode=Callback::new(move |_| {
                                                if let Some(cb) = on_episode_change {
                                                    let next_ep = episode.unwrap_or(1) + 1;
                                                    let curr_s = season.unwrap_or(1);
                                                    cb.run((curr_s, next_ep));
                                                }
                                            })
                                        />

                                        // Episode Drawer
                                        <EpisodeDrawer
                                            series_id=media_id
                                            current_season=season.unwrap_or(1)
                                            current_episode=episode.unwrap_or(1)
                                            is_open=is_episodes_open
                                            on_close=Callback::new(move |_| set_is_episodes_open.set(false))
                                            on_select_episode=Callback::new(move |(s, e)| {
                                                if let Some(cb) = on_episode_change {
                                                    cb.run((s, e));
                                                }
                                            })
                                        />

                                        // Subtitle Drawer
                                        <SubtitleDrawer
                                            is_open=is_subtitles_open
                                            subtitles=Signal::derive(current_subtitle_tracks)
                                            selected_sub_url=selected_sub_url
                                            sub_delay=sub_delay
                                            on_select_sub=Callback::new(move |u| set_selected_sub_url.set(u))
                                            on_change_delay=Callback::new(move |d| set_sub_delay.set(d))
                                            on_close=Callback::new(move |_| set_is_subtitles_open.set(false))
                                        />
                                    </div>
                                }.into_any()
                            }}
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}
