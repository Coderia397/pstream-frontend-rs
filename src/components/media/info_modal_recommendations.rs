use leptos::prelude::*;
use crate::models::movie::Movie;
use crate::services::tmdb::{fetch_recommendations, fetch_movie_logo};
use crate::components::media::movie_card_badges::MaturityBadge;
use crate::store::{use_library_store, LibraryEntry};

const BATCH_REC: usize = 9;

#[component]
pub fn RecCard(
    rec: Movie,
    on_play: Option<Callback<Movie>>,
    on_open_modal: Callback<Movie>,
) -> impl IntoView {
    let library_store = use_library_store();
    let rec_id = rec.id_u32();
    let is_tv = rec.is_tv();
    let title = rec.display_title();
    let year = rec.release_date.as_ref().or(rec.first_air_date.as_ref())
        .map(|d| d.chars().take(4).collect::<String>()).unwrap_or_default();
    let overview = if rec.overview.is_empty() {
        "No description available.".to_string()
    } else {
        rec.overview.clone()
    };

    let is_added = Signal::derive(move || {
        library_store.my_list.get().contains_key(&rec_id)
    });

    let toggle_my_list = {
        let rec_clone = rec.clone();
        move |e: leptos::ev::MouseEvent| {
            e.stop_propagation();
            let mut list = library_store.my_list.get();
            if list.contains_key(&rec_id) {
                list.remove(&rec_id);
            } else {
                list.insert(rec_id, LibraryEntry {
                    media: crate::services::tmdb::MediaItem {
                        id: rec_id,
                        title: rec_clone.title.clone(),
                        name: rec_clone.name.clone(),
                        overview: rec_clone.overview.clone(),
                        poster_path: rec_clone.poster_path.clone(),
                        backdrop_path: rec_clone.backdrop_path.clone(),
                        vote_average: rec_clone.vote_average,
                        release_date: rec_clone.release_date.clone(),
                        first_air_date: rec_clone.first_air_date.clone(),
                        media_type: rec_clone.media_type.clone(),
                    },
                    added_at: js_sys::Date::now() as u64,
                });
            }
            library_store.my_list.set(list);
        }
    };

    let logo_resource = LocalResource::new(move || {
        let mid = rec_id;
        let is_tv_val = is_tv;
        async move {
            if mid > 0 {
                fetch_movie_logo(mid, is_tv_val).await.ok().flatten()
            } else {
                None
            }
        }
    });

    let backdrop = rec.backdrop_path.as_ref().or(rec.poster_path.as_ref())
        .map(|p| {
            if p.starts_with("http") {
                p.clone()
            } else {
                format!("https://image.tmdb.org/t/p/w342{}", p)
            }
        });

    let on_image_click = {
        let rec_c = rec.clone();
        move |_| {
            if let Some(cb) = on_play {
                cb.run(rec_c.clone());
            } else if let Some(w) = web_sys::window() {
                let kind = if is_tv { "tv" } else { "movie" };
                let _ = w.location().set_href(&format!("/watch/{}/{}", kind, rec_id));
            }
        }
    };

    let on_body_click = {
        let rec_c = rec.clone();
        move |_| {
            on_open_modal.run(rec_c.clone());
        }
    };

    view! {
        <div class="bg-[#2f2f2f] rounded-sm overflow-hidden shadow-lg group">
            // Image area - click to play
            <div
                class="relative aspect-video bg-[#1a1a1a] overflow-hidden cursor-pointer"
                on:click=on_image_click
            >
                {if let Some(src) = backdrop {
                    view! {
                        <img
                            src=src
                            alt=title.clone()
                            class="w-full h-full object-cover opacity-90 group-hover:opacity-100 transition-opacity duration-200"
                            loading="lazy"
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="w-full h-full bg-[#222]" />
                    }.into_any()
                }}

                // Play icon overlay on hover
                <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none">
                    <div class="w-11 h-11 rounded-full bg-black/50 border border-white/50 flex items-center justify-center">
                        <i class="ph-fill ph-play text-white ml-0.5 text-lg"></i>
                    </div>
                </div>

                // Bottom-left title/logo
                <div class="absolute bottom-2 left-2 right-2 max-w-[78%]">
                    <Suspense fallback=move || view! { <div class="h-6 w-20 rounded bg-white/10 animate-pulse" /> }>
                        {
                            let tc = title.clone();
                            move || {
                                let t = tc.clone();
                                logo_resource.get().map(|res| match res {
                                    Some(url) if !url.is_empty() => view! {
                                        <img
                                            src=url
                                            alt=t.clone()
                                            class="max-h-12 max-w-full object-contain object-left-bottom drop-shadow-lg"
                                        />
                                    }.into_any(),
                                    _ => view! {
                                        <p class="text-white font-bold text-sm leading-tight drop-shadow-md line-clamp-2">
                                            {t.clone()}
                                        </p>
                                    }.into_any()
                                })
                            }
                        }
                    </Suspense>
                </div>
            </div>

            // Card body - click opens InfoModal
            <div
                class="p-3 cursor-pointer hover:bg-[#3a3a3a] transition-colors duration-150"
                on:click=on_body_click
            >
                <div class="flex items-center justify-between gap-2 mb-2.5">
                    <div class="flex items-center gap-2 flex-wrap">
                        <MaturityBadge
                            adult=rec.adult.unwrap_or(false)
                            certification=rec.certification.clone().unwrap_or_default()
                            size="sm".to_string()
                        />
                        {if !year.is_empty() {
                            view! {
                                <span class="text-gray-300 text-xs font-medium px-0.5">{year}</span>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }}
                    </div>

                    // Add/remove from list button
                    <button
                        type="button"
                        on:click=toggle_my_list
                        title=move || if is_added.get() { "Remove from My List" } else { "Add to My List" }
                        class="shrink-0 w-8 h-8 rounded-full border-2 flex items-center justify-center transition-all duration-150 hover:scale-110 active:scale-95 cursor-pointer"
                        class=("border-white", move || is_added.get())
                        class=("bg-white/10", move || is_added.get())
                        class=("text-white", move || is_added.get())
                        class=("border-gray-500", move || !is_added.get())
                        class=("text-gray-400", move || !is_added.get())
                        class=("hover:border-white", move || !is_added.get())
                        class=("hover:text-white", move || !is_added.get())
                    >
                        <i class=move || if is_added.get() { "ph-bold ph-check text-xs" } else { "ph-bold ph-plus text-xs" }></i>
                    </button>
                </div>

                <p class="text-white/80 text-[12px] leading-relaxed line-clamp-5 min-h-[72px]">
                    {overview}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn InfoModalRecommendations(
    movie_id: u32,
    is_tv: bool,
    on_recommendation_click: Callback<Movie>,
    #[prop(optional)] on_play: Option<Callback<Movie>>,
) -> impl IntoView {
    let visible_count = RwSignal::new(BATCH_REC);

    let rec_resource = LocalResource::new(move || {
        let mid = movie_id;
        let tv = is_tv;
        async move {
            if mid > 0 {
                fetch_recommendations(mid, tv).await.ok()
            } else {
                None
            }
        }
    });

    view! {
        <Suspense fallback=move || view! {
            <div class="mt-10">
                <h3 class="text-xl md:text-2xl font-bold text-white mb-5">"More Like This"</h3>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-4">
                    {(0..6).map(|_| view! {
                        <div class="bg-[#2f2f2f] rounded-sm aspect-[4/5] animate-pulse" />
                    }).collect_view()}
                </div>
            </div>
        }>
            {move || rec_resource.get().map(|res| match res {
                Some(items) if !items.is_empty() => {
                    let total = items.len();
                    let count = visible_count.get();
                    let visible_items: Vec<_> = items.into_iter().take(count).collect();
                    let has_more = count < total;

                    view! {
                        <div class="mt-10">
                            <h3 class="text-xl md:text-2xl font-bold text-white mb-5">"More Like This"</h3>
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-4">
                                {visible_items.into_iter().map(|item| {
                                    let movie = Movie::from(item);
                                    let on_rec_click = on_recommendation_click;
                                    let on_play_cb = on_play;

                                    view! {
                                        <RecCard
                                            rec=movie
                                            on_play=on_play_cb
                                            on_open_modal=on_rec_click
                                        />
                                    }
                                }).collect_view()}
                            </div>

                            {if has_more || count > BATCH_REC {
                                view! {
                                    <div class="flex justify-center mt-5">
                                        <button
                                            type="button"
                                            on:click=move |e| {
                                                e.stop_propagation();
                                                if has_more {
                                                    visible_count.update(|c| *c += BATCH_REC);
                                                } else {
                                                    visible_count.set(BATCH_REC);
                                                }
                                            }
                                            class="w-10 h-10 rounded-full border border-white/20 bg-[#2a2a2a] hover:border-white/50 hover:bg-[#3a3a3a] flex items-center justify-center transition-all duration-200 hover:scale-110 active:scale-95 group/btn cursor-pointer"
                                        >
                                            <i class=if has_more {
                                                "ph-bold ph-caret-down text-white/60 group-hover/btn:text-white transition-colors"
                                            } else {
                                                "ph-bold ph-caret-up text-white/60 group-hover/btn:text-white transition-colors"
                                            }></i>
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span /> }.into_any()
                            }}
                        </div>
                    }.into_any()
                },
                _ => view! { <span /> }.into_any()
            })}
        </Suspense>
    }
}
