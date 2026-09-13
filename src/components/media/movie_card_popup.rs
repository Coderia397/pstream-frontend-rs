use leptos::prelude::*;
use leptos::portal::Portal;
use wasm_bindgen::prelude::*;
use crate::models::movie::Movie;
use crate::components::media::movie_card_badges::MaturityBadge;
use crate::components::media::movie_card_rating::{RatingIcon, MovieRating};

pub const POPUP_W: f64 = 341.0;
pub const TOP_OFFSET: f64 = -88.0;

pub fn calc_popup_position(
    rect: &web_sys::DomRect,
    pos: &str,
    initial_scroll: (f64, f64),
    window_w: f64,
) -> (f64, f64) {
    let mut left = if pos == "left" {
        rect.left()
    } else if pos == "right" {
        rect.right() - POPUP_W
    } else {
        rect.left() + rect.width() / 2.0 - POPUP_W / 2.0
    };

    let app_x = if window_w >= 1024.0 { 56.0 } else if window_w >= 768.0 { 48.0 } else { 16.0 };
    left = left.max(app_x).min(window_w - POPUP_W - app_x);

    let top = rect.top() + initial_scroll.1 + TOP_OFFSET;
    (top, left + initial_scroll.0)
}

#[component]
pub fn MovieCardPopup(
    movie: Movie,
    hovered_rect: web_sys::DomRect,
    initial_scroll: (f64, f64),
    #[prop(default = "center".to_string())] hover_position: String,
    on_mouse_enter: Callback<()>,
    on_mouse_leave: Callback<()>,
    on_play: Callback<()>,
    on_open_modal: Callback<()>,
) -> impl IntoView {
    let window_w = web_sys::window()
        .and_then(|w| w.inner_width().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(1200.0);

    let (top, left) = calc_popup_position(&hovered_rect, &hover_position, initial_scroll, window_w);

    let transform_origin = match hover_position.as_str() {
        "left" => "top left",
        "right" => "top right",
        _ => "top center",
    };

    let title = movie.display_title();
    let backdrop = movie.backdrop_path.clone().or(movie.poster_path.clone());
    let cert = movie.certification.clone();
    let adult = movie.adult;
    let vote_avg = (movie.vote_average * 10.0).round() / 10.0;
    let is_tv = movie.is_tv();

    let overview_stored = StoredValue::new(movie.overview.clone());
    let title_stored = StoredValue::new(title);
    let cert_stored = StoredValue::new(cert);
    let image_url = backdrop.map(|p| {
        if p.starts_with("http") { p } else { format!("https://image.tmdb.org/t/p/w780{}", p) }
    });
    let image_url_stored = StoredValue::new(image_url);

    let library_store = crate::store::use_library_store();
    let movie_stored = StoredValue::new(movie.clone());
    let movie_id = movie.id_u32();
    let is_in_list = Signal::derive(move || {
        library_store.my_list.get().contains_key(&movie_id)
    });

    let (rating, set_rating) = signal(None::<MovieRating>);

    // 1200ms debounce auto-preview trailer
    let (is_playing, set_is_playing) = signal(false);
    let (is_muted, set_is_muted) = signal(true);
    let (video_key, set_video_key) = signal(None::<String>);

    let movie_id = movie.id_u32();
    let is_tv_for_video = is_tv;

    Effect::new(move |_| {
        let mid = movie_id;
        let is_tv_val = is_tv_for_video;
        if mid > 0 {
            leptos::task::spawn_local(async move {
                if let Ok(videos) = crate::services::tmdb::fetch_videos(mid, is_tv_val).await {
                    let trailer = videos.into_iter().find(|v| {
                        v.site.to_lowercase() == "youtube" && (v.r#type == "Trailer" || v.r#type == "Teaser" || v.r#type == "Clip")
                    });
                    if let Some(t) = trailer {
                        set_video_key.set(Some(t.key));
                    }
                }
            });
        }

        if let Some(window) = web_sys::window() {
            let cb = Closure::once_into_js(move || {
                set_is_playing.set(true);
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 1200);
        }
    });

    view! {
        <Portal>
            <div
                data-popup="true"
                class="absolute z-[9999] pointer-events-auto rounded-md overflow-hidden bg-[#181818] text-white shadow-[0_4px_30px_rgba(0,0,0,0.9)] animate-in fade-in zoom-in-95 duration-200"
                style=format!(
                    "width: {}px; top: {}px; left: {}px; transform-origin: {};",
                    POPUP_W, top, left, transform_origin
                )
                on:mouseenter=move |_| on_mouse_enter.run(())
                on:mouseleave=move |_| on_mouse_leave.run(())
            >
                // Media / Thumbnail preview
                <div class="relative w-full aspect-video bg-[#202020] overflow-hidden">
                    {move || {
                        let key_opt = video_key.get();
                        let playing = is_playing.get();

                        if playing && key_opt.is_some() {
                            let key = key_opt.unwrap();
                            let muted_val = if is_muted.get() { 1 } else { 0 };
                            let embed_url = format!(
                                "https://www.youtube-nocookie.com/embed/{}?autoplay=1&mute={}&controls=0&modestbranding=1&rel=0&iv_load_policy=3&playsinline=1",
                                key, muted_val
                            );
                            view! {
                                <iframe
                                    src=embed_url
                                    class="w-full h-full object-cover scale-[1.35] pointer-events-none"
                                    allow="autoplay; encrypted-media"
                                    tabindex="-1"
                                />
                                // Mute toggle button
                                <button
                                    type="button"
                                    class="absolute bottom-3 right-3 w-7 h-7 rounded-full border border-white/40 bg-black/60 flex items-center justify-center hover:bg-white/20 hover:border-white/70 z-50 pointer-events-auto cursor-pointer"
                                    on:click=move |e| {
                                        e.stop_propagation();
                                        set_is_muted.update(|m| *m = !*m);
                                    }
                                >
                                    {move || if is_muted.get() {
                                        view! { <i class="ph-bold ph-speaker-slash text-xs text-white"></i> }.into_any()
                                    } else {
                                        view! { <i class="ph-bold ph-speaker-high text-xs text-white"></i> }.into_any()
                                    }}
                                </button>
                            }.into_any()
                        } else {
                            let src_opt = image_url_stored.get_value();
                            let t = title_stored.get_value();
                            view! {
                                {if let Some(src) = src_opt {
                                    view! {
                                        <img
                                            src=src
                                            alt=t.clone()
                                            class="w-full h-full object-cover"
                                        />
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="w-full h-full flex items-center justify-center text-white/30 text-xs">
                                            "No Image"
                                        </div>
                                    }.into_any()
                                }}
                                <div class="absolute inset-0 bg-gradient-to-t from-[#181818] via-transparent to-transparent opacity-80 pointer-events-none" />
                                <span class="absolute bottom-2 left-3 font-black text-sm tracking-wide drop-shadow-md line-clamp-1 pointer-events-none">
                                    {t}
                                </span>
                            }.into_any()
                        }
                    }}
                </div>

                // Watch Progress Bar if watched
                {move || {
                    let watch_store = crate::store::use_watch_store();
                    if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                        if rec.percentage > 0.0 {
                            let pct = rec.percentage.clamp(0.0, 100.0);
                            view! {
                                <div class="h-1 w-full bg-white/20 relative">
                                    <div class="h-full bg-[#e50914]" style=format!("width: {:.1}%;", pct) />
                                </div>
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }
                    } else {
                        view! { <span /> }.into_any()
                    }
                }}

                // Metadata & Action Controls
                <div class="p-4 space-y-3">
                    // Action Buttons Row
                    <div class="flex items-center justify-between">
                        <div class="flex items-center gap-2">
                            // Play button
                            <button
                                type="button"
                                on:click=move |_| on_play.run(())
                                class="w-9 h-9 rounded-full bg-white text-black flex items-center justify-center hover:bg-white/80 active:scale-95 transition-all shadow-md cursor-pointer"
                                title=move || {
                                    let watch_store = crate::store::use_watch_store();
                                    if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                                        if rec.percentage > 0.0 {
                                            return "Resume";
                                        }
                                    }
                                    "Play"
                                }
                            >
                                <i class="ph-fill ph-play text-lg translate-x-0.5"></i>
                            </button>

                            // Add to My List button
                            <button
                                type="button"
                                on:click=move |_| {
                                    let m = movie_stored.get_value();
                                    library_store.my_list.update(|map| {
                                        if map.contains_key(&movie_id) {
                                            map.remove(&movie_id);
                                        } else {
                                            let entry = crate::store::LibraryEntry {
                                                media: crate::services::tmdb::MediaItem {
                                                    id: movie_id,
                                                    title: m.title.clone(),
                                                    name: m.name.clone(),
                                                    overview: m.overview.clone(),
                                                    poster_path: m.poster_path.clone(),
                                                    backdrop_path: m.backdrop_path.clone(),
                                                    vote_average: m.vote_average,
                                                    release_date: m.release_date.clone(),
                                                    first_air_date: m.first_air_date.clone(),
                                                    media_type: m.media_type.clone(),
                                                    ..Default::default()
                                                },
                                                added_at: js_sys::Date::now() as u64,
                                            };
                                            map.insert(movie_id, entry);
                                        }
                                    });
                                }
                                class="w-9 h-9 rounded-full border-2 border-white/40 hover:border-white text-white flex items-center justify-center hover:bg-white/10 active:scale-95 transition-all"
                                title="Add to My List"
                            >
                                {move || if is_in_list.get() {
                                    view! { <i class="ph-bold ph-check text-base text-green-400"></i> }.into_any()
                                } else {
                                    view! { <i class="ph-bold ph-plus text-base"></i> }.into_any()
                                }}
                            </button>

                            // Rating Pill button
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_rating.update(|r| {
                                        *r = match *r {
                                            None => Some(MovieRating::Like),
                                            Some(MovieRating::Like) => Some(MovieRating::Love),
                                            Some(MovieRating::Love) => Some(MovieRating::Dislike),
                                            Some(MovieRating::Dislike) => None,
                                        };
                                    });
                                }
                                class="w-9 h-9 rounded-full border-2 border-white/40 hover:border-white text-white flex items-center justify-center hover:bg-white/10 active:scale-95 transition-all"
                                title="Rate"
                            >
                                <RatingIcon rating=rating.get() size=16 />
                            </button>
                        </div>

                        // Expand Info Modal caret
                        <button
                            type="button"
                            on:click=move |_| on_open_modal.run(())
                            class="w-9 h-9 rounded-full border-2 border-white/40 hover:border-white text-white flex items-center justify-center hover:bg-white/10 active:scale-95 transition-all"
                            title="More Info"
                        >
                            <i class="ph-bold ph-caret-down text-base"></i>
                        </button>
                    </div>

                    // Badges & Details Row
                    <div class="flex items-center gap-2 text-xs font-semibold text-white/80 flex-wrap">
                        <span class="text-green-400 font-bold">
                            {format!("{}% Match", (vote_avg * 10.0) as u32)}
                        </span>
                        <MaturityBadge certification=cert_stored.get_value().unwrap_or_default() adult=adult.unwrap_or(false) size="xs".to_string() />
                        <span class="text-white/60">
                            {if is_tv { "TV Series" } else { "Movie" }}
                        </span>
                        <span class="border border-white/40 rounded px-1 text-[10px] text-white/70">
                            "HD"
                        </span>
                    </div>

                    // Genres overview
                    <p class="text-xs text-white/60 line-clamp-2 leading-relaxed">
                        {overview_stored.get_value()}
                    </p>
                </div>
            </div>
        </Portal>
    }
}
