use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::store::use_watch_store;
use crate::store::use_ui_store;

#[component]
pub fn ContinueWatchingRow() -> impl IntoView {
    let watch_store = use_watch_store();
    let ui_store = use_ui_store();
    let navigate = use_navigate();
    let scroll_ref = NodeRef::<leptos::html::Div>::new();

    let scroll = move |direction: &str| {
        if let Some(el) = scroll_ref.get() {
            let client_width = el.client_width() as f64;
            let current_scroll = el.scroll_left() as f64;
            let step = client_width * 0.85;
            let target = if direction == "left" {
                current_scroll - step
            } else {
                current_scroll + step
            };
            el.scroll_to_with_x_and_y(target, 0.0);
        }
    };

    view! {
        {move || {
            let items = watch_store.get_continue_watching_list();
            if items.is_empty() {
                view! { <span /> }.into_any()
            } else {
                let scroll_left = scroll.clone();
                let scroll_right = scroll.clone();

                view! {
                    <div class="group relative my-3 md:my-5 space-y-2 z-10">
                        <div class="flex items-center justify-between px-[var(--app-x,56px)]">
                            <h2 class="text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit tracking-wide">
                                "Continue Watching"
                                <span class="text-xs text-red-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                                    "Jump Back In ›"
                                </span>
                            </h2>
                        </div>

                        <div class="relative group/row row-scroll-outer">
                            <div
                                node_ref=scroll_ref
                                class="row-scroll-strip flex overflow-x-scroll scrollbar-hide w-full pointer-events-auto relative z-10 py-2 pb-4 gap-2 md:gap-3"
                                style="scroll-behavior: smooth;"
                            >
                                <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />

                                {items.into_iter().map(|rec| {
                                    let nav_play = navigate.clone();
                                    let id = rec.media_id;
                                    let is_tv = rec.is_tv;
                                    let season = rec.season;
                                    let episode = rec.episode;
                                    let title = rec.title.clone();
                                    let episode_title = rec.episode_title.clone();
                                    let percentage = rec.percentage.clamp(0.0, 100.0);
                                    let backdrop = rec.backdrop_url("w780").or_else(|| rec.poster_url("w500")).unwrap_or_default();

                                    let target_url = if is_tv && season.is_some() && episode.is_some() {
                                        format!("/watch/{}?season={}&episode={}", id, season.unwrap(), episode.unwrap())
                                    } else {
                                        format!("/watch/{}", id)
                                    };

                                    let target_url_for_play = target_url.clone();
                                    let handle_play = move |e: leptos::ev::MouseEvent| {
                                        e.stop_propagation();
                                        nav_play(&target_url_for_play, Default::default());
                                    };

                                    let handle_open_info = move |e: leptos::ev::MouseEvent| {
                                        e.stop_propagation();
                                        ui_store.info_modal_movie_id.set(Some(id));
                                        ui_store.info_modal_is_tv.set(is_tv);
                                        ui_store.info_modal_open.set(true);
                                    };

                                    let handle_remove = move |e: leptos::ev::MouseEvent| {
                                        e.stop_propagation();
                                        watch_store.remove_record(id, is_tv);
                                    };

                                    let ep_badge = if is_tv {
                                        match (season, episode) {
                                            (Some(s), Some(e)) => Some(format!("S{}:E{}", s, e)),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    };

                                    view! {
                                        <div
                                            class="relative flex-none w-[220px] sm:w-[260px] md:w-[290px] lg:w-[320px] bg-[#181818] rounded-md overflow-hidden border border-white/10 hover:border-white/30 transition-all duration-300 shadow-lg hover:shadow-2xl group/card cursor-pointer"
                                            on:click=handle_play
                                        >
                                            // Thumbnail image with play overlay
                                            <div class="relative w-full aspect-video bg-[#202020] overflow-hidden">
                                                <img
                                                    src=backdrop
                                                    alt=title.clone()
                                                    class="w-full h-full object-cover object-center group-hover/card:scale-105 transition-transform duration-500"
                                                    loading="lazy"
                                                />
                                                <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-black/20" />

                                                // Centered play button icon
                                                <div class="absolute inset-0 flex items-center justify-center opacity-80 group-hover/card:opacity-100 group-hover/card:scale-110 transition-all duration-300">
                                                    <div class="w-11 h-11 sm:w-12 sm:h-12 rounded-full bg-white/20 backdrop-blur-md border border-white/40 flex items-center justify-center text-white shadow-xl hover:bg-white/40">
                                                        <i class="ph-fill ph-play text-xl translate-x-0.5"></i>
                                                    </div>
                                                </div>

                                                // Top Right Dismiss Button (hover only)
                                                <button
                                                    type="button"
                                                    on:click=handle_remove
                                                    class="absolute top-2 right-2 w-7 h-7 rounded-full bg-black/60 hover:bg-black/90 text-white/70 hover:text-white flex items-center justify-center opacity-0 group-hover/card:opacity-100 transition-opacity duration-200 z-20 backdrop-blur-sm"
                                                    title="Remove from Continue Watching"
                                                >
                                                    <i class="ph-bold ph-x text-xs"></i>
                                                </button>

                                                // Top Left Episode Badge (if TV)
                                                {ep_badge.map(|badge| view! {
                                                    <div class="absolute top-2 left-2 px-2 py-0.5 rounded bg-black/70 backdrop-blur-md text-white/90 text-[11px] font-bold tracking-wider uppercase border border-white/10">
                                                        {badge}
                                                    </div>
                                                })}
                                            </div>

                                            // Progress Bar
                                            <div class="h-1.5 w-full bg-white/20 relative">
                                                <div
                                                    class="h-full bg-[#e50914] transition-all duration-300"
                                                    style=format!("width: {:.1}%;", percentage)
                                                />
                                            </div>

                                            // Card Metadata & Actions footer
                                            <div class="p-3 flex items-center justify-between bg-[#141414]">
                                                <div class="min-w-0 flex-1 pr-2">
                                                    <p class="text-white text-xs sm:text-sm font-semibold truncate leading-tight">
                                                        {title}
                                                    </p>
                                                    {if let Some(ep_t) = episode_title {
                                                        view! {
                                                            <p class="text-white/50 text-[11px] truncate mt-0.5">
                                                                {ep_t}
                                                            </p>
                                                        }.into_any()
                                                    } else {
                                                        let pct_str = format!("{:.0}% watched", percentage);
                                                        view! {
                                                            <p class="text-white/40 text-[11px] truncate mt-0.5">
                                                                {pct_str}
                                                            </p>
                                                        }.into_any()
                                                    }}
                                                </div>

                                                <div class="flex items-center gap-1.5 shrink-0">
                                                    // Info button
                                                    <button
                                                        type="button"
                                                        on:click=handle_open_info
                                                        class="w-7 h-7 rounded-full border border-white/30 hover:border-white text-white flex items-center justify-center hover:bg-white/10 transition-all"
                                                        title="More Info"
                                                    >
                                                        <i class="ph-bold ph-info text-xs"></i>
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}

                                <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />
                            </div>

                            // Left Chevron
                            <div
                                class="absolute top-0 bottom-4 left-0 z-30 w-6 md:w-14 lg:w-16 items-center justify-center cursor-pointer bg-transparent hover:bg-[#141414]/70 flex group/arrow-left transition-[opacity,background-color] duration-200 rounded-r-sm opacity-0 pointer-events-none group-hover/row:opacity-100 group-hover/row:pointer-events-auto"
                                on:click=move |_| scroll_left("left")
                            >
                                <i class="ph-bold ph-caret-left text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                            </div>

                            // Right Chevron
                            <div
                                class="absolute top-0 bottom-4 right-0 z-30 w-6 md:w-14 lg:w-16 items-center justify-center cursor-pointer bg-transparent hover:bg-[#141414]/70 flex group/arrow-right transition-[opacity,background-color] duration-200 pointer-events-none rounded-l-sm opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto"
                                on:click=move |_| scroll_right("right")
                            >
                                <i class="ph-bold ph-caret-right text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                            </div>
                        </div>
                    </div>
                }.into_any()
            }
        }}
    }
}
