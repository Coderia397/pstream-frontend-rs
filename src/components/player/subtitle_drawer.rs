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
                class="fixed inset-0 z-40 bg-void/60 backdrop-blur-sm transition-opacity"
            />

            // Drawer
            <div class="fixed top-0 right-0 bottom-0 z-50 w-full sm:w-[380px] bg-page/95 backdrop-blur-xl border-l border-ink/10 shadow-2xl flex flex-col animate-in slide-in-from-right duration-(--motion-base)">
                // Header
                <div class="flex items-center justify-between px-6 py-5 border-b border-ink/10">
                    <h2 class="text-xl font-bold text-ink tracking-tight flex items-center gap-2">
                        <i class="ph ph-subtitles text-2xl text-brand"></i>
                        {crate::i18n::t("controls-overlay-audio-subtitles")}
                    </h2>
                    <button
                        on:click=move |_| on_close.run(())
                        class="w-8 h-8 rounded-full flex items-center justify-center text-ink/70 hover:text-ink hover:bg-ink/10 transition-colors"
                    >
                        <i class="ph ph-x text-lg"></i>
                    </button>
                </div>

                <div class="flex-1 overflow-y-auto p-6 space-y-6">
                    // Subtitles Section
                    <div>
                        <h3 class="text-xs uppercase tracking-wider font-bold text-ink/50 mb-3">
                            {crate::i18n::t("subtitle-drawer-subtitles")}
                        </h3>
                        <div class="space-y-1">
                            // Off Option
                            <button
                                on:click=move |_| on_select_sub.run(None)
                                class=move || format!(
                                    "w-full flex items-center justify-between px-4 py-3 rounded-card text-sm font-medium transition-colors {}",
                                    if selected_sub_url.get().is_none() {
                                        "bg-ink/15 text-ink"
                                    } else {
                                        "text-ink/70 hover:bg-ink/5 hover:text-ink"
                                    }
                                )
                            >
                                <span>{crate::i18n::t("player-subs-off")}</span>
                                <Show when=move || selected_sub_url.get().is_none()>
                                    <i class="ph ph-check text-base text-brand"></i>
                                </Show>
                            </button>

                            // Available tracks
                            {move || {
                                let subs = subtitles.get();
                                if subs.is_empty() {
                                    view! {
                                        <div class="text-xs text-ink/40 italic px-4 py-2">
                                            {crate::i18n::t("subtitle-drawer-no-subtitles-detected-for-this")}
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
                                                    "w-full flex items-center justify-between px-4 py-3 rounded-card text-sm font-medium transition-colors {}",
                                                    if is_active {
                                                        "bg-ink/15 text-ink"
                                                    } else {
                                                        "text-ink/70 hover:bg-ink/5 hover:text-ink"
                                                    }
                                                )
                                            >
                                                <span>{label}</span>
                                                <Show when=move || is_active>
                                                    <i class="ph ph-check text-base text-brand"></i>
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
                        <div class="pt-4 border-t border-ink/10">
                            <h3 class="text-xs uppercase tracking-wider font-bold text-ink/50 mb-3">
                                {crate::i18n::t("subtitle-drawer-subtitle-timing-offset")}
                            </h3>
                            <div class="flex items-center justify-between bg-ink/[0.04] p-3 rounded-card border border-ink/10">
                                <button
                                    on:click=move |_| on_change_delay.run(sub_delay.get() - 0.5)
                                    class="px-3 py-1.5 bg-ink/10 hover:bg-ink/20 text-ink rounded-control font-bold text-sm"
                                >
                                    "-0.5s"
                                </button>
                                <span class="text-sm font-mono text-ink/90">
                                    {move || format!("{:+.1}s", sub_delay.get())}
                                </span>
                                <button
                                    on:click=move |_| on_change_delay.run(sub_delay.get() + 0.5)
                                    class="px-3 py-1.5 bg-ink/10 hover:bg-ink/20 text-ink rounded-control font-bold text-sm"
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
