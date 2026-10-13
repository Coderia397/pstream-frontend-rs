use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::store::{use_library_store, use_ui_store, LibraryEntry};
use crate::services::tmdb::MediaItem;
use crate::components::media::movie_card_badges::RatingBadge;

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

    let coming_soon_label = release_date.as_deref().map(crate::i18n::short_date).unwrap_or_default();

    let backdrop_src = if backdrop_path.starts_with("http") {
        backdrop_path.clone()
    } else if !backdrop_path.is_empty() {
        format!("{}/w780{}", crate::config::TMDB_IMAGE_URL, backdrop_path)
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
                class="relative w-full min-h-[440px] flex flex-col bg-page border border-ink/[0.09] rounded-modal overflow-hidden shadow-2xl cursor-pointer select-none"
            >
                // Media Area (16:9)
                <div class="relative w-full aspect-video overflow-hidden bg-void shrink-0">
                    {if !backdrop_src.is_empty() {
                        view! {
                            <img
                                src=backdrop_src
                                alt=title.clone()
                                class="absolute inset-0 w-full h-full object-cover transition-transform duration-(--motion-slower) hover:scale-105"
                                loading="lazy"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-full h-full bg-placeholder flex items-center justify-center">
                                <span class="text-ink/40 font-bold">{title.clone()}</span>
                            </div>
                        }.into_any()
                    }}

                    // Play hint overlay
                    <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                        <div class="w-[52px] h-[52px] rounded-full bg-void/55 border border-ink/25 flex items-center justify-center shadow-2xl">
                            <i class="ph-fill ph-play text-ink text-2xl ml-1"></i>
                        </div>
                    </div>

                    // Maturity disc
                    <div class="absolute top-3 right-3 z-20 pointer-events-none drop-shadow-md">
                        <RatingBadge id=movie_id is_tv=is_tv size="xs".to_string() />
                    </div>
                </div>

                // Below-media content area
                <div class="px-5 pb-6 pt-4 flex flex-col flex-grow gap-3">
                    // Title
                    <h3 class="text-ink font-bold text-title-sm md:text-title leading-snug drop-shadow-sm">
                        {title.clone()}
                    </h3>

                    // Coming Soon label if applicable
                    {if is_coming_soon && !coming_soon_label.is_empty() {
                        view! {
                            <p class="text-brand font-bold text-xs md:text-sm tracking-wider uppercase">
                                {crate::i18n::t_args("media-coming", &[("when", fluent_bundle::FluentValue::from(coming_soon_label.clone()))])}
                            </p>
                        }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    }}

                    // Overview
                    <p class="text-body-sm md:text-body text-ink-soft font-normal leading-relaxed line-clamp-3 flex-1">
                        {overview}
                    </p>

                    // CTA Buttons
                    <div class="mt-auto pt-3">
                        {if is_coming_soon || hide_play {
                            view! {
                                <button
                                    on:click=on_toggle_list
                                    class="btn-glass flex items-center justify-center h-[48px] px-8 rounded-control text-base gap-2.5 transition-all active:scale-95 w-full"
                                >
                                    {move || if is_in_my_list() {
                                        view! {
                                            <i class="ph-bold ph-check text-xl text-ink"></i>
                                            <span>{crate::i18n::t("media-in-list")}</span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <i class="ph-bold ph-plus text-xl text-ink"></i>
                                            <span>{crate::i18n::t("media-add-to-list")}</span>
                                        }.into_any()
                                    }}
                                </button>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex items-center gap-3">
                                    <button
                                        on:click=on_play_click
                                        class="btn-ink flex-1 flex items-center justify-center h-[48px] rounded-control text-base gap-2 transition-all active:scale-95 shadow"
                                    >
                                        <i class="ph-fill ph-play text-xl text-void"></i>
                                        <span>{move || {
                                            let watch_store = crate::store::use_watch_store();
                                            if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                                                if rec.percentage > 0.0 {
                                                    return crate::i18n::t("media-resume");
                                                }
                                            }
                                            crate::i18n::t("media-play")
                                        }}</span>
                                    </button>
                                    <button
                                        on:click=on_toggle_list
                                        class="btn-glass flex-1 flex items-center justify-center h-[48px] rounded-control text-base gap-2 transition-all active:scale-95"
                                    >
                                        {move || if is_in_my_list() {
                                            view! {
                                                <i class="ph-bold ph-check text-xl text-ink"></i>
                                                <span>{crate::i18n::t("media-in-list")}</span>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <i class="ph-bold ph-plus text-xl text-ink"></i>
                                                <span>{crate::i18n::t("nav-my-list")}</span>
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
