use leptos::prelude::*;

#[component]
pub fn SkipButton(
    current_time: ReadSignal<f64>,
    duration: ReadSignal<f64>,
    is_tv: bool,
    on_skip_intro: Callback<()>,
    on_next_episode: Callback<()>,
) -> impl IntoView {
    // Show Skip Intro between 10s and 90s if video duration > 300s
    let show_skip_intro = move || {
        let t = current_time.get();
        let d = duration.get();
        d > 300.0 && t >= 10.0 && t <= 90.0
    };

    // Show Next Episode when within last 45s of episode
    let show_next_episode = move || {
        if !is_tv {
            return false;
        }
        let t = current_time.get();
        let d = duration.get();
        d > 120.0 && t >= d - 45.0 && t < d - 2.0
    };

    // "Watch credits" hides the end of episode offer for the rest of this play
    let (credits_dismissed, set_credits_dismissed) = signal(false);
    // laid out as read from the end of episode screen on Netflix (1666 wide page): buttons 2.36vw high, 0.75vw apart, 3vw from the right edge and 3.54vw from the bottom, text 1vw, radius 4px
    let base = "flex h-[2.36vw] items-center justify-center gap-[0.6vw] rounded-[4px] px-[1.4vw] text-[1vw] font-medium select-none transition-colors duration-(--motion-fast)";

    view! {
        <div class="fixed bottom-[3.54vw] right-[3vw] z-30 flex items-center gap-[0.75vw] pointer-events-auto">
            <Show when=show_skip_intro>
                <button on:click=move |_| on_skip_intro.run(()) class=format!("{base} bg-[rgba(109,109,110,0.7)] text-ink hover:bg-[rgba(109,109,110,0.4)]")>
                    <i class="ph-fill ph-fast-forward text-[1.4vw]"></i>
                    <span>{crate::i18n::t("player-skip-intro")}</span>
                </button>
            </Show>

            <Show when=move || show_next_episode() && !credits_dismissed.get()>
                <button on:click=move |_| set_credits_dismissed.set(true) class=format!("{base} bg-[rgba(109,109,110,0.7)] text-ink hover:bg-[rgba(109,109,110,0.4)]")>
                    <span>{crate::i18n::t("player-watch-credits")}</span>
                </button>
                <button on:click=move |_| on_next_episode.run(()) class=format!("{base} bg-ink text-void hover:bg-ink/75")>
                    <i class="ph-fill ph-play text-[1.6vw]"></i>
                    <span>{crate::i18n::t("player-next-episode")}</span>
                </button>
            </Show>
        </div>
    }
}
