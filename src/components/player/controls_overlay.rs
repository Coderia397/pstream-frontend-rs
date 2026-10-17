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
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}

#[component]
pub fn ControlsOverlay(
    title: String,
    subtitle: Signal<String>,
    is_tv: bool,
    is_playing: ReadSignal<bool>,
    is_buffering: ReadSignal<bool>,
    is_muted: ReadSignal<bool>,
    volume: ReadSignal<f64>,
    current_time: ReadSignal<f64>,
    duration: ReadSignal<f64>,
    buffered_fraction: ReadSignal<f64>,
    is_idle: ReadSignal<bool>,
    on_toggle_play: Callback<()>,
    on_seek: Callback<f64>,
    on_skip_by: Callback<f64>,
    on_toggle_mute: Callback<()>,
    on_set_volume: Callback<f64>,
    on_toggle_episodes: Callback<()>,
    on_toggle_subtitles: Callback<()>,
    on_next_episode: Option<Callback<()>>,
    on_toggle_fullscreen: Callback<()>,
    on_back: Callback<()>,
) -> impl IntoView {
    // 36 px icons inside 44 px buttons, as in the app
    let play_icon_class = move || if is_playing.get() { "ph-fill ph-pause text-[36px]" } else { "ph-fill ph-play text-[36px]" };
    let speaker_icon_class = move || {
        if is_muted.get() || volume.get() <= 0.0 { "ph ph-speaker-x text-[36px]" } else if volume.get() > 0.5 { "ph ph-speaker-high text-[36px]" } else { "ph ph-speaker-low text-[36px]" }
    };
    let overlay_class = move || {
        format!(
            "absolute inset-0 z-20 transition-opacity duration-(--motion-slow) select-none {}",
            if is_idle.get() && is_playing.get() { "opacity-0 pointer-events-none cursor-none" } else { "opacity-100 pointer-events-auto cursor-default" }
        )
    };
    // every button: 44 x 44, no plate
    let btn = "flex h-11 relative w-11 shrink-0 items-center justify-center rounded-[2px] text-ink transition-transform duration-(--motion-quick) hover:scale-110 active:scale-95";

    view! {
        <div class=overlay_class>
            // the top shade and the back and report buttons (30 px from the corner)
            <div class="pointer-events-none absolute inset-x-0 top-0 h-[85%]" style="background: linear-gradient(rgba(0,0,0,0.3) 0, rgba(0,0,0,0) 20%);" />
            <button on:click=move |_| on_back.run(()) class=format!("{} absolute left-[30px] top-[30px] z-10", btn.replace(" relative", "")) title=crate::i18n::t("controls-overlay-back-to-browse") aria-label=crate::i18n::t("controls-overlay-back-to-browse")>
                <i class="ph ph-arrow-left text-[36px]"></i>
            </button>
            <a href="/help/contact" class=format!("{} absolute right-[30px] top-[30px] z-10", btn.replace(" relative", "")) title=crate::i18n::t("controls-overlay-report-problem") aria-label=crate::i18n::t("controls-overlay-report-problem")>
                <i class="ph ph-flag text-[34px]"></i>
            </a>

            // the buffering ring in the middle
            <div class="pointer-events-none absolute inset-0 flex items-center justify-center">
                <Show when=move || is_buffering.get()>
                    <div class="h-16 w-16 animate-spin rounded-full border-4 border-brand border-t-transparent"></div>
                </Show>
            </div>

            // the bottom: a 135 px band with a soft shade, the timeline on top, the buttons 52 px below its top
            <div class="absolute inset-x-0 bottom-0 h-[135px]" style="background: linear-gradient(0deg, rgba(0,0,0,0.3) 0, rgba(0,0,0,0));">
                <div class="absolute left-[20px] right-[90px] top-0">
                    <TimelineScrubber current_time=current_time duration=duration buffered_fraction=buffered_fraction on_seek=on_seek />
                </div>
                <span class="absolute right-[28px] top-[1px] w-11 text-right text-[16px] leading-5 text-ink">
                    {move || format_time_remaining(current_time.get(), duration.get())}
                </span>

                <div class="absolute left-[20px] right-[28px] top-[52px] flex h-11 items-center">
                    // left: play or pause, back 10, forward 10, volume
                    <div class="flex items-center gap-3 md:gap-[38px]">
                        <button on:click=move |_| on_toggle_play.run(()) class=btn title=move || if is_playing.get() { crate::i18n::t("player-pause") } else { crate::i18n::t("player-play") }>
                            <i class=play_icon_class></i>
                        </button>
                        <button on:click=move |_| on_skip_by.run(-10.0) class=btn title=crate::i18n::t("controls-overlay-rewind-10-seconds-left-arrow")>
                            <i class="ph ph-arrow-counter-clockwise text-[36px]"></i>
                            <span class="pointer-events-none absolute pt-[2px] text-[11px] font-black">"10"</span>
                        </button>
                        <button on:click=move |_| on_skip_by.run(10.0) class=btn title=crate::i18n::t("controls-overlay-forward-10-seconds-right-arrow")>
                            <i class="ph ph-arrow-clockwise text-[36px]"></i>
                            <span class="pointer-events-none absolute pt-[2px] text-[11px] font-black">"10"</span>
                        </button>
                        // the volume slider opens while the pointer rests on the speaker
                        <div class="group/vol flex items-center">
                            <button on:click=move |_| on_toggle_mute.run(()) class=btn title=move || if is_muted.get() { crate::i18n::t("player-unmute") } else { crate::i18n::t("player-mute") }>
                                <i class=speaker_icon_class></i>
                            </button>
                            <input
                                type="range" min="0" max="1" step="0.05"
                                prop:value=move || if is_muted.get() { 0.0 } else { volume.get() }
                                on:input=move |e| { if let Ok(v) = event_target_value(&e).parse::<f64>() { on_set_volume.run(v); } }
                                class="h-1 w-0 cursor-pointer opacity-0 accent-brand transition-all duration-(--motion-base) group-hover/vol:ms-2 group-hover/vol:w-24 group-hover/vol:opacity-100"
                                aria-label=crate::i18n::t("player-volume")
                            />
                        </div>
                    </div>

                    // the title, centred
                    <div class="mx-6 flex min-w-0 flex-1 items-baseline justify-center gap-3 text-[20px] text-ink">
                        <span class="truncate font-bold">{title.clone()}</span>
                        {move || { let sub = subtitle.get(); (!sub.is_empty()).then(|| view! { <span class="truncate text-ink">{sub}</span> }) }}
                    </div>

                    // right: next, episodes, audio and subtitles, fullscreen
                    <div class="flex items-center gap-3 md:gap-[38px]">
                        <Show when=move || is_tv && on_next_episode.is_some()>
                            <button on:click=move |_| { if let Some(cb) = on_next_episode { cb.run(()); } } class=btn title=crate::i18n::t("controls-overlay-next-episode")>
                                <i class="ph ph-skip-forward text-[36px]"></i>
                            </button>
                        </Show>
                        <Show when=move || is_tv>
                            <button on:click=move |_| on_toggle_episodes.run(()) class=btn title=crate::i18n::t("controls-overlay-episode-selector")>
                                <i class="ph ph-stack text-[36px]"></i>
                            </button>
                        </Show>
                        <button on:click=move |_| on_toggle_subtitles.run(()) class=btn title=crate::i18n::t("controls-overlay-audio-subtitles")>
                            <i class="ph ph-subtitles text-[36px]"></i>
                        </button>
                        <button on:click=move |_| on_toggle_fullscreen.run(()) class=btn title=crate::i18n::t("controls-overlay-fullscreen-f")>
                            <i class="ph ph-corners-out text-[36px]"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
