use leptos::prelude::*;
use crate::components::player::timeline_scrubber::TimelineScrubber;

fn format_time_remaining(curr: f64, dur: f64) -> String {
    if dur <= 0.0 || curr.is_nan() || dur.is_nan() {
        return "00:00".to_string();
    }
    let rem = (dur - curr).max(0.0).floor() as u64;
    let hours = rem / 3600;
    let mins = (rem % 3600) / 60;
    let secs = rem % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}

#[component]
pub fn ControlsOverlay(
    title: String,
    subtitle: Option<String>,
    is_tv: bool,
    is_playing: ReadSignal<bool>,
    is_buffering: ReadSignal<bool>,
    is_muted: ReadSignal<bool>,
    volume: ReadSignal<f64>,
    current_time: ReadSignal<f64>,
    duration: ReadSignal<f64>,
    buffered_fraction: ReadSignal<f64>,
    playback_speed: ReadSignal<f64>,
    is_idle: ReadSignal<bool>,
    on_toggle_play: Callback<()>,
    on_seek: Callback<f64>,
    on_skip_by: Callback<f64>,
    on_toggle_mute: Callback<()>,
    on_set_volume: Callback<f64>,
    on_set_speed: Callback<f64>,
    on_toggle_episodes: Callback<()>,
    on_toggle_subtitles: Callback<()>,
    on_next_episode: Option<Callback<()>>,
    on_toggle_fullscreen: Callback<()>,
    on_back: Callback<()>,
) -> impl IntoView {
    let (show_speed_menu, set_show_speed_menu) = signal(false);

    let play_icon_class = move || {
        if is_playing.get() {
            "ph-fill ph-pause text-2xl md:text-3xl"
        } else {
            "ph-fill ph-play text-2xl md:text-3xl"
        }
    };

    let speaker_icon_class = move || {
        if is_muted.get() || volume.get() <= 0.0 {
            "ph ph-speaker-slash text-2xl md:text-3xl"
        } else if volume.get() > 0.5 {
            "ph ph-speaker-high text-2xl md:text-3xl"
        } else {
            "ph ph-speaker-low text-2xl md:text-3xl"
        }
    };

    let overlay_class = move || {
        format!(
            "absolute inset-0 z-20 flex flex-col justify-between p-6 md:p-8 bg-gradient-to-t from-black/85 via-transparent to-black/75 transition-opacity duration-300 select-none {}",
            if is_idle.get() && is_playing.get() {
                "opacity-0 pointer-events-none cursor-none"
            } else {
                "opacity-100 pointer-events-auto cursor-default"
            }
        )
    };

    view! {
        <div class=overlay_class>
            // Top Bar
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center gap-4">
                    <button
                        on:click=move |_| on_back.run(())
                        class="w-10 h-10 rounded-full flex items-center justify-center text-white/90 hover:text-white hover:bg-white/10 active:scale-95 transition-all duration-150"
                        title="Back to Browse"
                    >
                        <i class="ph ph-arrow-left text-2xl"></i>
                    </button>

                    <div class="flex flex-col">
                        <h1 class="text-base sm:text-lg md:text-xl font-bold text-white tracking-tight leading-tight">
                            {title.clone()}
                        </h1>
                        {subtitle.clone().map(|sub| view! {
                            <p class="text-xs sm:text-sm text-white/70 font-medium">
                                {sub}
                            </p>
                        })}
                    </div>
                </div>
            </div>

            // Center Area (Buffering or Play indicator)
            <div class="flex-1 flex items-center justify-center pointer-events-none">
                <Show when=move || is_buffering.get()>
                    <div class="w-16 h-16 rounded-full border-4 border-[#e50914] border-t-transparent animate-spin shadow-2xl"></div>
                </Show>
            </div>

            // Bottom Bar
            <div class="flex flex-col gap-2 w-full">
                // Timeline Scrubber
                <TimelineScrubber
                    current_time=current_time
                    duration=duration
                    buffered_fraction=buffered_fraction
                    on_seek=on_seek
                />

                // Controls Row
                <div class="flex items-center justify-between mt-1">
                    // Left: Play, Rewind 10, FastForward 10, Volume, Time
                    <div class="flex items-center gap-3 md:gap-5">
                        // Play / Pause
                        <button
                            on:click=move |_| on_toggle_play.run(())
                            class="text-white hover:text-white/80 active:scale-90 transition-transform duration-100"
                            title=move || if is_playing.get() { "Pause (Space)" } else { "Play (Space)" }
                        >
                            <i class=play_icon_class></i>
                        </button>

                        // Rewind 10s
                        <button
                            on:click=move |_| on_skip_by.run(-10.0)
                            class="relative text-white hover:text-white/80 active:scale-90 transition-transform duration-100 flex items-center justify-center"
                            title="Rewind 10 seconds (Left Arrow)"
                        >
                            <i class="ph ph-arrow-counter-clockwise text-2xl md:text-3xl"></i>
                            <span class="absolute text-[9px] font-black top-1/2 -translate-y-1/2 pt-0.5 pointer-events-none">"10"</span>
                        </button>

                        // Forward 10s
                        <button
                            on:click=move |_| on_skip_by.run(10.0)
                            class="relative text-white hover:text-white/80 active:scale-90 transition-transform duration-100 flex items-center justify-center"
                            title="Forward 10 seconds (Right Arrow)"
                        >
                            <i class="ph ph-arrow-clockwise text-2xl md:text-3xl"></i>
                            <span class="absolute text-[9px] font-black top-1/2 -translate-y-1/2 pt-0.5 pointer-events-none">"10"</span>
                        </button>

                        // Volume Control (Hover to open slider)
                        <div class="flex items-center gap-2 group/vol">
                            <button
                                on:click=move |_| on_toggle_mute.run(())
                                class="text-white hover:text-white/80 transition-colors"
                                title=move || if is_muted.get() { "Unmute (M)" } else { "Mute (M)" }
                            >
                                <i class=speaker_icon_class></i>
                            </button>

                            <input
                                type="range"
                                min="0"
                                max="1"
                                step="0.05"
                                prop:value=move || if is_muted.get() { 0.0 } else { volume.get() }
                                on:input=move |e| {
                                    if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                        on_set_volume.run(v);
                                    }
                                }
                                class="w-0 group-hover/vol:w-20 transition-all duration-200 accent-[#e50914] cursor-pointer h-1.5 rounded-lg overflow-hidden"
                            />
                        </div>

                        // Time Remaining
                        <div class="text-xs md:text-sm font-medium text-white/80 select-none">
                            {move || format!("-{}", format_time_remaining(current_time.get(), duration.get()))}
                        </div>
                    </div>

                    // Right: Next Episode, Episodes Drawer, Subtitles, Speed, Fullscreen
                    <div class="flex items-center gap-4 md:gap-6">
                        // Next Episode Button (TV only)
                        <Show when=move || is_tv && on_next_episode.is_some()>
                            <button
                                on:click=move |_| {
                                    if let Some(cb) = on_next_episode {
                                        cb.run(());
                                    }
                                }
                                class="text-white hover:text-white/80 active:scale-95 transition-transform"
                                title="Next Episode"
                            >
                                <i class="ph ph-skip-forward text-2xl"></i>
                            </button>
                        </Show>

                        // Episodes Drawer Toggle (TV only)
                        <Show when=move || is_tv>
                            <button
                                on:click=move |_| on_toggle_episodes.run(())
                                class="text-white hover:text-white/80 active:scale-95 transition-transform"
                                title="Episode Selector"
                            >
                                <i class="ph ph-squares-four text-2xl"></i>
                            </button>
                        </Show>

                        // Audio & Subtitles
                        <button
                            on:click=move |_| on_toggle_subtitles.run(())
                            class="text-white hover:text-white/80 active:scale-95 transition-transform"
                            title="Audio & Subtitles"
                        >
                            <i class="ph ph-subtitles text-2xl"></i>
                        </button>

                        // Playback Speed Toggle
                        <div class="relative">
                            <button
                                on:click=move |_| set_show_speed_menu.update(|s| *s = !*s)
                                class="text-white hover:text-white/80 font-bold text-xs md:text-sm px-2 py-1 rounded bg-white/10 hover:bg-white/20 transition-colors"
                                title="Playback Speed"
                            >
                                {move || format!("{:.2}x", playback_speed.get())}
                            </button>

                            <Show when=move || show_speed_menu.get()>
                                <div class="absolute bottom-8 right-0 bg-[#181818] border border-white/15 rounded-lg py-1 shadow-2xl flex flex-col z-30 min-w-[80px]">
                                    {[0.5, 0.75, 1.0, 1.25, 1.5].into_iter().map(|spd| {
                                        let is_active = (playback_speed.get() - spd).abs() < 0.01;
                                        let set_spd_cb = on_set_speed;
                                        view! {
                                            <button
                                                on:click=move |_| {
                                                    set_spd_cb.run(spd);
                                                    set_show_speed_menu.set(false);
                                                }
                                                class=move || format!(
                                                    "px-3 py-1.5 text-xs text-left transition-colors flex items-center justify-between {}",
                                                    if is_active { "bg-[#e50914] text-white font-bold" } else { "text-white/80 hover:bg-white/10" }
                                                )
                                            >
                                                <span>{format!("{:.2}x", spd)}</span>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </Show>
                        </div>

                        // Fullscreen Toggle
                        <button
                            on:click=move |_| on_toggle_fullscreen.run(())
                            class="text-white hover:text-white/80 active:scale-95 transition-transform"
                            title="Fullscreen (F)"
                        >
                            <i class="ph ph-corners-out text-2xl"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
