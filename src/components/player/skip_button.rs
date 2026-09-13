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

    view! {
        <div class="fixed bottom-24 right-8 z-30 flex flex-col gap-3 pointer-events-auto">
            <Show when=show_skip_intro>
                <button
                    on:click=move |_| on_skip_intro.run(())
                    class="flex items-center gap-2.5 px-6 py-3 bg-black/75 hover:bg-black/90 backdrop-blur-md text-white font-bold text-sm rounded-lg border border-white/20 shadow-2xl hover:scale-105 active:scale-95 transition-all duration-150 select-none group"
                >
                    <i class="ph ph-fast-forward text-lg text-white group-hover:text-[#e50914] transition-colors"></i>
                    <span>"Skip Intro"</span>
                </button>
            </Show>

            <Show when=show_next_episode>
                <button
                    on:click=move |_| on_next_episode.run(())
                    class="flex items-center gap-2.5 px-6 py-3 bg-[#e50914] hover:bg-[#b80710] text-white font-bold text-sm rounded-lg shadow-2xl hover:scale-105 active:scale-95 transition-all duration-150 select-none"
                >
                    <i class="ph ph-skip-forward text-lg"></i>
                    <span>"Next Episode"</span>
                </button>
            </Show>
        </div>
    }
}
