use leptos::prelude::*;
use crate::services::resolver::SubtitleTrack;

#[component]
pub fn SubtitleDrawer(
    is_open: ReadSignal<bool>,
    subtitles: Signal<Vec<SubtitleTrack>>,
    selected_sub_url: ReadSignal<Option<String>>,
    sub_delay: ReadSignal<f64>,
    on_select_sub: Callback<Option<String>>,
    on_change_delay: Callback<f64>,
    on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || is_open.get()>
            // Backdrop scrim
            <div
                on:click=move |_| on_close.run(())
                class="fixed inset-0 z-40 bg-black/60 backdrop-blur-sm transition-opacity"
            />

            // Drawer
            <div class="fixed top-0 right-0 bottom-0 z-50 w-full sm:w-[380px] bg-[#141414]/95 backdrop-blur-xl border-l border-white/10 shadow-2xl flex flex-col animate-in slide-in-from-right duration-200">
                // Header
                <div class="flex items-center justify-between px-6 py-5 border-b border-white/10">
                    <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
                        <i class="ph ph-subtitles text-2xl text-[#e50914]"></i>
                        "Audio & Subtitles"
                    </h2>
                    <button
                        on:click=move |_| on_close.run(())
                        class="w-8 h-8 rounded-full flex items-center justify-center text-white/70 hover:text-white hover:bg-white/10 transition-colors"
                    >
                        <i class="ph ph-x text-lg"></i>
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto p-6 space-y-6">
                    // Subtitles Section
                    <div>
                        <h3 class="text-xs uppercase tracking-wider font-bold text-white/50 mb-3">
                            "Subtitles"
                        </h3>
                        <div class="space-y-1">
                            // Off Option
                            <button
                                on:click=move |_| on_select_sub.run(None)
                                class=move || format!(
                                    "w-full flex items-center justify-between px-4 py-3 rounded-lg text-sm font-medium transition-colors {}",
                                    if selected_sub_url.get().is_none() {
                                        "bg-white/15 text-white"
                                    } else {
                                        "text-white/70 hover:bg-white/5 hover:text-white"
                                    }
                                )
                            >
                                <span>"Off"</span>
                                <Show when=move || selected_sub_url.get().is_none()>
                                    <i class="ph ph-check text-base text-[#e50914]"></i>
                                </Show>
                            </button>

                            // Available tracks
                            {move || {
                                let subs = subtitles.get();
                                if subs.is_empty() {
                                    view! {
                                        <div class="text-xs text-white/40 italic px-4 py-2">
                                            "No subtitles detected for this stream."
                                        </div>
                                    }.into_any()
                                } else {
                                    subs.into_iter().map(|s| {
                                        let url = s.url.clone();
                                        let label = s.label.clone();
                                        let is_active = selected_sub_url.get().as_deref() == Some(&url);
                                        let on_click = {
                                            let u = url.clone();
                                            let select_cb = on_select_sub;
                                            move |_| select_cb.run(Some(u.clone()))
                                        };

                                        view! {
                                            <button
                                                on:click=on_click
                                                class=move || format!(
                                                    "w-full flex items-center justify-between px-4 py-3 rounded-lg text-sm font-medium transition-colors {}",
                                                    if is_active {
                                                        "bg-white/15 text-white"
                                                    } else {
                                                        "text-white/70 hover:bg-white/5 hover:text-white"
                                                    }
                                                )
                                            >
                                                <span>{label}</span>
                                                <Show when=move || is_active>
                                                    <i class="ph ph-check text-base text-[#e50914]"></i>
                                                </Show>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }
                            }}
                        </div>
                    </div>

                    // Subtitle Sync Offset
                    <Show when=move || selected_sub_url.get().is_some()>
                        <div class="pt-4 border-t border-white/10">
                            <h3 class="text-xs uppercase tracking-wider font-bold text-white/50 mb-3">
                                "Subtitle Timing Offset"
                            </h3>
                            <div class="flex items-center justify-between bg-white/[0.04] p-3 rounded-lg border border-white/10">
                                <button
                                    on:click=move |_| on_change_delay.run(sub_delay.get() - 0.5)
                                    class="px-3 py-1.5 bg-white/10 hover:bg-white/20 text-white rounded font-bold text-sm"
                                >
                                    "-0.5s"
                                </button>
                                <span class="text-sm font-mono text-white/90">
                                    {move || format!("{:+.1}s", sub_delay.get())}
                                </span>
                                <button
                                    on:click=move |_| on_change_delay.run(sub_delay.get() + 0.5)
                                    class="px-3 py-1.5 bg-white/10 hover:bg-white/20 text-white rounded font-bold text-sm"
                                >
                                    "+0.5s"
                                </button>
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
