use leptos::prelude::*;
use crate::services::tmdb::{fetch_season_details, Episode};

#[component]
pub fn EpisodeDrawer(
    series_id: u32,
    current_season: u32,
    current_episode: u32,
    is_open: ReadSignal<bool>,
    on_close: Callback<()>,
    on_select_episode: Callback<(u32, u32)>,
) -> impl IntoView {
    let (selected_season, set_selected_season) = signal(current_season);

    // Season episodes resource
    let episodes_resource = LocalResource::new(move || {
        let season_num = selected_season.get();
        async move {
            if series_id > 0 {
                fetch_season_details(series_id, season_num).await.ok().map(|d| d.episodes)
            } else {
                None
            }
        }
    });

    view! {
        <Show when=move || is_open.get()>
            // Backdrop scrim
            <div
                on:click=move |_| on_close.run(())
                class="fixed inset-0 z-40 bg-void/60 backdrop-blur-sm transition-opacity"
            />

            // Slide-Over Drawer
            <div class="fixed top-0 right-0 bottom-0 z-50 w-full sm:w-[420px] md:w-[480px] bg-page/95 backdrop-blur-xl border-l border-ink/10 shadow-2xl flex flex-col animate-in slide-in-from-right duration-(--motion-base)">
                // Header
                <div class="flex items-center justify-between px-6 py-5 border-b border-ink/10">
                    <div class="flex items-center gap-3">
                        <h2 class="text-xl font-bold text-ink tracking-tight">"Episodes"</h2>
                        // Season Selector
                        <div class="relative">
                            <select
                                on:change=move |e| {
                                    if let Ok(val) = event_target_value(&e).parse::<u32>() {
                                        set_selected_season.set(val);
                                    }
                                }
                                class="bg-ink/10 text-ink text-sm font-semibold rounded-card px-3 py-1.5 pr-8 border border-ink/15 outline-none appearance-none cursor-pointer hover:bg-ink/20 transition-colors"
                            >
                                {(1..=8).map(|s| {
                                    let is_curr = s == selected_season.get();
                                    view! {
                                        <option value=s selected=is_curr class="bg-raised text-ink">
                                            {format!("Season {}", s)}
                                        </option>
                                    }
                                }).collect::<Vec<_>>()}
                            </select>
                            <i class="ph ph-caret-down text-xs text-ink/70 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none"></i>
                        </div>
                    </div>

                    <button
                        on:click=move |_| on_close.run(())
                        class="w-8 h-8 rounded-full flex items-center justify-center text-ink/70 hover:text-ink hover:bg-ink/10 transition-colors"
                    >
                        <i class="ph ph-x text-lg"></i>
                    </button>
                </div>

                // Episode List
                <div class="flex-1 overflow-y-auto p-4 space-y-3">
                    <Suspense fallback=move || view! {
                        <div class="flex items-center justify-center py-20 text-ink/50">
                            <div class="w-8 h-8 rounded-full border-2 border-brand border-t-transparent animate-spin"></div>
                        </div>
                    }>
                        {move || {
                            episodes_resource.get().map(|res| match res {
                                Some(episodes) if !episodes.is_empty() => {
                                    episodes.into_iter().map(|ep: Episode| {
                                        let ep_num = ep.episode_number;
                                        let is_active = selected_season.get() == current_season && ep_num == current_episode;
                                        let still_url = ep.still_url("w300");
                                        let ep_name = ep.name.clone();
                                        let ep_overview = ep.overview.clone();
                                        let ep_runtime = ep.runtime.map(|r| format!("{}m", r)).unwrap_or_default();

                                        let on_card_click = {
                                            let on_select = on_select_episode;
                                            let s_num = selected_season.get();
                                            move |_| {
                                                on_select.run((s_num, ep_num));
                                                on_close.run(());
                                            }
                                        };

                                        view! {
                                            <div
                                                on:click=on_card_click
                                                class=move || format!(
                                                    "group flex gap-3 p-3 rounded-modal cursor-pointer transition-all duration-(--motion-fast) border {}",
                                                    if is_active {
                                                        "bg-ink/15 border-ink/30 shadow-md"
                                                    } else {
                                                        "bg-ink/[0.04] border-transparent hover:bg-ink/[0.09]"
                                                    }
                                                )
                                            >
                                                // Thumbnail
                                                <div class="relative w-28 aspect-video flex-shrink-0 rounded-card overflow-hidden bg-placeholder">
                                                    {if let Some(url) = still_url {
                                                        view! {
                                                            <img
                                                                src=url
                                                                alt=ep_name.clone()
                                                                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-(--motion-base)"
                                                                loading="lazy"
                                                            />
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="w-full h-full flex items-center justify-center text-ink/30 text-xs">
                                                                <i class="ph ph-film-strip text-2xl"></i>
                                                            </div>
                                                        }.into_any()
                                                    }}
                                                    <Show when=move || is_active>
                                                        <div class="absolute inset-0 bg-void/40 flex items-center justify-center">
                                                            <i class="ph-fill ph-play text-xl text-brand"></i>
                                                        </div>
                                                    </Show>
                                                </div>

                                                // Details
                                                <div class="flex-1 min-w-0 flex flex-col justify-center">
                                                    <div class="flex items-center justify-between gap-2">
                                                        <h4 class=move || format!(
                                                            "text-sm font-semibold truncate {}",
                                                            if is_active { "text-brand" } else { "text-ink group-hover:text-ink/90" }
                                                        )>
                                                            {format!("{}. {}", ep_num, ep_name)}
                                                        </h4>
                                                        <span class="text-xs text-ink/50 flex-shrink-0">{ep_runtime}</span>
                                                    </div>
                                                    <p class="text-xs text-ink/60 line-clamp-2 mt-1 leading-relaxed">
                                                        {ep_overview}
                                                    </p>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }
                                _ => view! {
                                    <div class="text-center py-16 text-ink/50 text-sm">
                                        {crate::i18n::t("episode-drawer-no-episodes-available-for-this")}
                                    </div>
                                }.into_any(),
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </Show>
    }
}
