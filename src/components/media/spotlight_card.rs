use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::store::{use_library_store, use_ui_store, LibraryEntry};
use crate::services::tmdb::MediaItem;
use crate::components::media::movie_card_badges::MaturityBadge;

#[component]
pub fn SpotlightCard(
    movie_id: u32,
    is_tv: bool,
    title: String,
    backdrop_path: String,
    overview: String,
    #[prop(default = None)] rank: Option<usize>,
    #[prop(default = None)] release_date: Option<String>,
    #[prop(default = false)] is_coming_soon: bool,
    #[prop(default = false)] hide_play: bool,
) -> impl IntoView {
    let navigate = use_navigate();
    let library = use_library_store();
    let ui_store = use_ui_store();

    let is_in_my_list = move || library.my_list.with(|m| m.contains_key(&movie_id));

    let title_stored = title.clone();
    let backdrop_stored = backdrop_path.clone();
    let overview_stored = overview.clone();
    let rel_date_stored = release_date.clone();

    let on_toggle_list = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        if is_in_my_list() {
            library.my_list.update(|m| { m.remove(&movie_id); });
        } else {
            let item = MediaItem {
                id: movie_id,
                title: if is_tv { None } else { Some(title_stored.clone()) },
                name: if is_tv { Some(title_stored.clone()) } else { None },
                overview: overview_stored.clone(),
                poster_path: None,
                backdrop_path: Some(backdrop_stored.clone()),
                vote_average: 0.0,
                release_date: rel_date_stored.clone(),
                first_air_date: if is_tv { rel_date_stored.clone() } else { None },
                media_type: Some(if is_tv { "tv".to_string() } else { "movie".to_string() }),
                ..Default::default()
            };
            library.my_list.update(|m| {
                m.insert(movie_id, LibraryEntry {
                    media: item,
                    added_at: js_sys::Date::now() as u64,
                });
            });
        }
    };

    let on_play_click = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        let watch_store = crate::store::use_watch_store();
        let target = if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
            if is_tv && rec.season.is_some() && rec.episode.is_some() {
                format!("/watch/tv/{}?season={}&episode={}", movie_id, rec.season.unwrap(), rec.episode.unwrap())
            } else if is_tv {
                format!("/watch/tv/{}", movie_id)
            } else {
                format!("/watch/movie/{}", movie_id)
            }
        } else if is_tv {
            format!("/watch/tv/{}", movie_id)
        } else {
            format!("/watch/movie/{}", movie_id)
        };
        navigate(&target, Default::default());
    };

    let on_card_click = move |_| {
        ui_store.hero_paused_by_modal.set(false);
        ui_store.modal_initial_time.set(0.0);
        ui_store.modal_current_time.set(0.0);
        ui_store.info_modal_movie_id.set(Some(movie_id));
        ui_store.info_modal_is_tv.set(is_tv);
        ui_store.info_modal_open.set(true);
    };

    let formatted_rank = rank.map(|r| if r < 10 { format!("0{}", r) } else { format!("{}", r) });

    let coming_soon_label = release_date.clone().unwrap_or_default();

    let backdrop_src = if backdrop_path.starts_with("http") {
        backdrop_path.clone()
    } else if !backdrop_path.is_empty() {
        format!("https://image.tmdb.org/t/p/w780{}", backdrop_path)
    } else {
        String::new()
    };

    view! {
        <div class="relative overflow-visible py-3 group/spotlight z-10 w-full sm:max-w-[85vw] sm:mx-auto">
            // Optional Top 10 SVG rank display
            {formatted_rank.map(|r_text| {
                view! {
                    <div class="relative h-16 w-full flex items-end pl-2 -mb-2 pointer-events-none select-none">
                        <svg viewBox="0 0 280 210" class="h-24 w-auto overflow-visible" preserveAspectRatio="none">
                            <text
                                x="4"
                                y="180"
                                text-anchor="start"
                                fill="#0a0a0a"
                                stroke="rgba(255,255,255,0.9)"
                                stroke-width="6"
                                stroke-linejoin="round"
                                font-size="160"
                                font-weight="900"
                                font-family="'Inter', sans-serif"
                                letter-spacing="-8"
                            >
                                {r_text}
                            </text>
                        </svg>
                    </div>
                }
            })}

            // Spotlight card container
            <div
                on:click=on_card_click
                class="relative w-full min-h-[440px] flex flex-col bg-[#141414] border border-white/[0.09] rounded-xl overflow-hidden shadow-2xl cursor-pointer select-none"
            >
                // Media Area (16:9)
                <div class="relative w-full aspect-video overflow-hidden bg-black shrink-0">
                    {if !backdrop_src.is_empty() {
                        view! {
                            <img
                                src=backdrop_src
                                alt=title.clone()
                                class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 hover:scale-105"
                                loading="lazy"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-full h-full bg-[#222] flex items-center justify-center">
                                <span class="text-white/40 font-bold">{title.clone()}</span>
                            </div>
                        }.into_any()
                    }}

                    // Play hint overlay
                    <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                        <div class="w-[52px] h-[52px] rounded-full bg-black/55 border border-white/25 flex items-center justify-center shadow-2xl">
                            <i class="ph-fill ph-play text-white text-2xl ml-1"></i>
                        </div>
                    </div>

                    // Maturity disc
                    <div class="absolute top-3 right-3 z-20 pointer-events-none drop-shadow-md">
                        <MaturityBadge
                            certification="16+".to_string()
                            size="xs".to_string()
                        />
                    </div>
                </div>

                // Below-media content area
                <div class="px-5 pb-6 pt-4 flex flex-col flex-grow gap-3">
                    // Title
                    <h3 class="text-white font-bold text-[18px] md:text-[20px] leading-snug drop-shadow-sm">
                        {title.clone()}
                    </h3>

                    // Coming Soon label if applicable
                    {if is_coming_soon && !coming_soon_label.is_empty() {
                        view! {
                            <p class="text-[#E50914] font-bold text-xs md:text-sm tracking-wider uppercase">
                                {format!("Coming {}", coming_soon_label)}
                            </p>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }}

                    // Overview
                    <p class="text-[14px] md:text-[15px] text-[#e5e5e5] font-normal leading-relaxed line-clamp-3 flex-1">
                        {overview}
                    </p>

                    // CTA Buttons
                    <div class="mt-auto pt-3">
                        {if is_coming_soon || hide_play {
                            view! {
                                <button
                                    on:click=on_toggle_list
                                    class="flex items-center justify-center h-[48px] px-8 rounded-[4px] bg-[#6d6d6e]/40 hover:bg-[#6d6d6e]/60 text-white font-bold text-base gap-2.5 transition-all active:scale-95 w-full cursor-pointer"
                                >
                                    {move || if is_in_my_list() {
                                        view! {
                                            <i class="ph-bold ph-check text-xl text-white"></i>
                                            <span>"In My List"</span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <i class="ph-bold ph-plus text-xl text-white"></i>
                                            <span>"Add to My List"</span>
                                        }.into_any()
                                    }}
                                </button>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex items-center gap-3">
                                    <button
                                        on:click=on_play_click
                                        class="flex-1 flex items-center justify-center h-[48px] rounded-[4px] bg-white hover:bg-neutral-200 text-black font-bold text-base gap-2 transition-all active:scale-95 cursor-pointer shadow"
                                    >
                                        <i class="ph-fill ph-play text-xl text-black"></i>
                                        <span>{move || {
                                            let watch_store = crate::store::use_watch_store();
                                            if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                                                if rec.percentage > 0.0 {
                                                    return "Resume";
                                                }
                                            }
                                            "Play"
                                        }}</span>
                                    </button>
                                    <button
                                        on:click=on_toggle_list
                                        class="flex-1 flex items-center justify-center h-[48px] rounded-[4px] bg-[#6d6d6e]/40 hover:bg-[#6d6d6e]/60 text-white font-bold text-base gap-2 transition-all active:scale-95 cursor-pointer"
                                    >
                                        {move || if is_in_my_list() {
                                            view! {
                                                <i class="ph-bold ph-check text-xl text-white"></i>
                                                <span>"In My List"</span>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <i class="ph-bold ph-plus text-xl text-white"></i>
                                                <span>"My List"</span>
                                            }.into_any()
                                        }}
                                    </button>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
