use leptos::prelude::*;
use leptos::portal::Portal;
use wasm_bindgen::prelude::*;
use crate::models::movie::Movie;
use crate::components::media::movie_card_badges::MaturityBadge;
use crate::components::media::movie_card_rating::MovieRating;
use crate::components::media::action_buttons::{
    PlayCircleButton, MyListButton, RatingButton, MoreInfoCaretButton, MuteReplayButton,
};
use crate::components::trailer_player::TrailerPlayer;

pub const POPUP_W: f64 = 341.0;
pub const TOP_OFFSET: f64 = -88.0;

pub fn calc_popup_position(
    rect: &web_sys::DomRect,
    pos: &str,
    initial_scroll: (f64, f64),
    window_w: f64,
    _window_h: f64,
) -> (f64, f64) {
    let app_x = if window_w >= 1024.0 { 56.0 } else if window_w >= 768.0 { 48.0 } else { 16.0 };

    let mut left = if pos == "left" {
        app_x
    } else if pos == "right" {
        window_w - app_x - POPUP_W
    } else {
        rect.left() + rect.width() / 2.0 - POPUP_W / 2.0
    };

    left = left.max(app_x).min(window_w - POPUP_W - app_x);

    let (scroll_x, scroll_y) = initial_scroll;
    // For taller cards (e.g. Top 10 portrait posters which are ~195px tall),
    // adjust top offset so the popup vertically centers naturally over the card
    let top_offset = if rect.height() > 160.0 { -60.0 } else { TOP_OFFSET };
    let page_top = (scroll_y + rect.top() + top_offset).max(scroll_y + 68.0);
    let page_left = scroll_x + left;

    (page_top, page_left)
}

pub fn derive_vibe_tags(
    vibe_pills: Option<&[String]>,
    genre_ids: Option<&[i64]>,
    overview: &str,
) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    // 1. Existing vibe pills if present
    if let Some(pills) = vibe_pills {
        for pill in pills {
            let p = pill.trim().to_string();
            if !p.is_empty() && !tags.iter().any(|t| t.eq_ignore_ascii_case(&p)) {
                tags.push(p);
                if tags.len() >= 3 {
                    return tags;
                }
            }
        }
    }

    let ov_lower = overview.to_lowercase();

    // 2. Thematic cues from overview text
    let thematic_cues = [
        (&["silo", "dystop", "post-apoc", "apocalyp", "bunker", "toxic future", "ruined future"][..], "Dystopian"),
        (&["cyberpunk", "artificial intelligence", "cyborg", "robot", "virtual reality", "simulation"][..], "Cyberpunk"),
        (&["space", "galaxy", "alien", "interstellar", "starship", "planet", "extraterrestrial"][..], "Mind-Bending"),
        (&["time travel", "multiverse", "timeline", "paradox"][..], "Mind-Bending"),
        (&["conspiracy", "secrets", "lies", "deception", "whistleblower", "cover-up"][..], "Suspenseful"),
        (&["detective", "murder", "investigat", "serial killer", "crime syndicate", "underworld"][..], "Gritty"),
        (&["psychological", "paranoia", "sanity", "hallucination", "nightmare"][..], "Psychological"),
        (&["survival", "survive", "stranded", "hostile", "deadly"][..], "Edge-of-Your-Seat"),
        (&["heist", "thief", "steal", "con artist", "casino"][..], "Slick"),
        (&["supernatural", "ghost", "haunt", "demon", "curse", "witch"][..], "Ominous"),
        (&["romantic", "falls in love", "heartbreak", "wedding", "passion"][..], "Heartfelt"),
        (&["satire", "parody", "hilarious", "misadventures", "absurd"][..], "Irreverent"),
        (&["coming of age", "adolescen", "high school", "childhood", "growing up"][..], "Coming-of-Age"),
        (&["epic battle", "empire", "kingdom", "dynasty", "warrior", "rebellion"][..], "Epic"),
    ];

    for (keywords, label) in thematic_cues {
        if keywords.iter().any(|&kw| ov_lower.contains(kw)) {
            if !tags.iter().any(|t| t.eq_ignore_ascii_case(label)) {
                tags.push(label.to_string());
                if tags.len() >= 3 {
                    return tags;
                }
            }
        }
    }

    // 3. Genre ID mapping
    if let Some(genres) = genre_ids {
        for &gid in genres {
            let genre_tags: &[&str] = match gid {
                28 => &["High-Octane", "Action"],
                12 => &["Adventurous", "Thrilling"],
                16 => &["Imaginative", "Charming"],
                35 => &["Witty", "Feel-Good"],
                80 => &["Gritty", "Dark"],
                99 => &["Provocative", "Insightful"],
                18 => &["Compelling", "Emotional"],
                10751 => &["Heartwarming", "Family"],
                14 => &["Enchanting", "Mythical"],
                36 => &["Sweeping", "Historical"],
                27 => &["Chilling", "Terrifying"],
                10402 => &["Soulful", "Rhythmic"],
                9648 => &["Mysterious", "Intriguing"],
                10749 => &["Romantic", "Passionate"],
                878 => &["Sci-Fi", "Futuristic"],
                53 => &["Suspenseful", "Gripping"],
                10752 => &["Visceral", "Intense"],
                37 => &["Atmospheric", "Raw"],
                10759 => &["Thrilling", "Action"],
                10762 => &["Playful", "Wholesome"],
                10764 => &["Unscripted", "Dramatic"],
                10765 => &["Sci-Fi", "Imaginative"],
                10768 => &["Political", "Intense"],
                _ => &[],
            };

            for &gt in genre_tags {
                if !tags.iter().any(|t| t.eq_ignore_ascii_case(gt)) {
                    tags.push(gt.to_string());
                    if tags.len() >= 3 {
                        return tags;
                    }
                }
            }
        }
    }

    // 4. Fallback pool if fewer than 3
    let fallback_pool = ["Compelling", "Suspenseful", "Exciting", "Atmospheric", "Intriguing"];
    for &fb in &fallback_pool {
        if !tags.iter().any(|t| t.eq_ignore_ascii_case(fb)) {
            tags.push(fb.to_string());
            if tags.len() >= 3 {
                break;
            }
        }
    }

    tags.truncate(3);
    tags
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
    let (window_w, window_h) = web_sys::window()
        .map(|w| {
            let w_val = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1200.0);
            let h_val = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            (w_val, h_val)
        })
        .unwrap_or((1200.0, 800.0));

    let (top, left) = calc_popup_position(&hovered_rect, &hover_position, initial_scroll, window_w, window_h);

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
    let ui_store = crate::store::use_ui_store();
    let movie_stored = StoredValue::new(movie.clone());
    let movie_id = movie.id_u32();
    let is_in_list = Signal::derive(move || {
        library_store.my_list.get().contains_key(&movie_id)
    });

    let (rating, set_rating) = signal(None::<MovieRating>);

    // 1200ms debounce auto-preview trailer
    let (is_playing, set_is_playing) = signal(false);
    let (is_frame_ready, set_is_frame_ready) = signal(false);
    let (card_trailer_time, set_card_trailer_time) = signal(0.0);
    let is_muted = ui_store.preview_muted;
    let (video_key, set_video_key) = signal(None::<String>);
    let (is_teaser, set_is_teaser) = signal(false);

    let movie_id = movie.id_u32();
    let is_tv_for_video = is_tv;

    Effect::new(move |_| {
        let mid = movie_id;
        let is_tv_val = is_tv_for_video;
        if mid > 0 {
            leptos::task::spawn_local(async move {
                if let Ok(videos) = crate::services::tmdb::fetch_videos(mid, is_tv_val).await {
                    if let Some((key, teaser)) = crate::services::tmdb::select_best_tmdb_trailer(&videos) {
                        set_video_key.set(Some(key));
                        set_is_teaser.set(teaser);
                    }
                }
            });
        }

        if let Some(window) = web_sys::window() {
            let cb = Closure::once_into_js(move || {
                set_is_playing.set(true);
                if let Some(w) = web_sys::window() {
                    let ready_cb = Closure::once_into_js(move || {
                        set_is_frame_ready.set(true);
                    });
                    let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(ready_cb.as_ref().unchecked_ref(), 1200);
                }
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 1200);
        }
    });

    Effect::new(move |_| {
        if is_frame_ready.get() && !ui_store.preview_muted.get() {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(el) = doc.get_element_by_id("card-trailer-iframe") {
                        if let Ok(iframe) = el.dyn_into::<web_sys::HtmlIFrameElement>() {
                            if let Some(cw) = iframe.content_window() {
                                let cmd = r#"{"event":"command","func":"unMute","args":[]}"#;
                                let _ = cw.post_message(&wasm_bindgen::JsValue::from_str(cmd), "*");
                            }
                        }
                    }
                }
            }
        }
    });

    view! {
        <Portal>
            <div
                data-popup="true"
                class="absolute z-[9999] pointer-events-auto rounded-[12px] overflow-hidden bg-[#181818] text-white shadow-[0_4px_30px_rgba(0,0,0,0.9)] animate-in fade-in zoom-in-95 duration-200"
                style=format!(
                    "width: {}px; top: {}px; left: {}px; transform-origin: {};",
                    POPUP_W, top, left, transform_origin
                )
                on:mouseenter=move |_| on_mouse_enter.run(())
                on:mouseleave=move |_| on_mouse_leave.run(())
            >
                // Media / Thumbnail preview
                <div class="relative w-full aspect-video bg-[#202020] overflow-hidden">
                    // Base Image layer (always present so no flash of red play button or black box)
                    {
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
                        }
                    }

                    // Video layer
                    <TrailerPlayer
                        video_key=Signal::derive(move || video_key.get())
                        is_teaser=Signal::derive(move || is_teaser.get())
                        is_playing=Signal::derive(move || is_playing.get())
                        is_muted=ui_store.preview_muted
                        variant="card".to_string()
                        iframe_id="card-trailer-iframe".to_string()
                        on_ready=Callback::new(move |_| {
                            set_is_frame_ready.set(true);
                        })
                        on_ended=Callback::new(move |_| {
                            set_is_playing.set(false);
                        })
                        on_time_update=Callback::new(move |t| {
                            set_card_trailer_time.set(t);
                        })
                    />

                    // Mute toggle button
                    {move || {
                        if video_key.get().is_some() && is_playing.get() {
                            view! {
                                <MuteReplayButton
                                    is_muted=is_muted
                                    is_ended=Signal::derive(|| false)
                                    on_toggle=Callback::new(move |_| {
                                        let new_muted = !ui_store.preview_muted.get_untracked();
                                        ui_store.set_preview_muted(new_muted);
                                    })
                                    size="md".to_string()
                                    class="absolute bottom-3 right-3 z-50 pointer-events-auto".to_string()
                                />
                            }.into_any()
                        } else {
                            view! { <span /> }.into_any()
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
                            <PlayCircleButton
                                on_click=on_play
                                is_resume=Signal::derive(move || {
                                    let watch_store = crate::store::use_watch_store();
                                    if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                                        rec.percentage > 0.0
                                    } else {
                                        false
                                    }
                                })
                                size="md".to_string()
                            />

                            // Add to My List button
                            <MyListButton
                                is_in_list=is_in_list
                                on_toggle=Callback::new(move |_| {
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
                                })
                                size="md".to_string()
                            />

                            // Rating Pill button
                            <RatingButton
                                rating=rating
                                on_rate=Callback::new(move |_| {
                                    set_rating.update(|r| {
                                        *r = match *r {
                                            None => Some(MovieRating::Like),
                                            Some(MovieRating::Like) => Some(MovieRating::Love),
                                            Some(MovieRating::Love) => Some(MovieRating::Dislike),
                                            Some(MovieRating::Dislike) => None,
                                        };
                                    });
                                })
                                size="md".to_string()
                            />
                        </div>

                        // Expand Info Modal caret
                        <MoreInfoCaretButton
                            on_open=Callback::new(move |_| {
                                let cur_t = card_trailer_time.get_untracked();
                                if cur_t > 0.0 {
                                    ui_store.modal_initial_time.set(cur_t);
                                    ui_store.modal_current_time.set(cur_t);
                                } else {
                                    ui_store.modal_initial_time.set(0.0);
                                    ui_store.modal_current_time.set(0.0);
                                }
                                on_open_modal.run(());
                            })
                            size="md".to_string()
                        />
                    </div>

                    // Badges & Details Row
                    <div class="flex items-center gap-2 text-xs font-semibold text-white/80 flex-wrap">
                        <span class="text-[#46d369] font-bold">
                            {format!("{}% Match", movie_stored.get_value().match_score())}
                        </span>
                        <MaturityBadge
                            certification=cert_stored.get_value().unwrap_or_default()
                            adult=adult.unwrap_or(false)
                            vote_average=vote_avg
                            size="xs".to_string()
                        />
                        <span class="text-white/60">
                            {if is_tv { "TV Series" } else { "Movie" }}
                        </span>
                        <span class="border border-white/40 rounded px-1 text-[10px] text-white/70">
                            "HD"
                        </span>
                    </div>

                    // 3-word vibe descriptors (replacing overview paragraph)
                    {
                        let m = movie_stored.get_value();
                        let vibe_tags = derive_vibe_tags(
                            m.vibe_pills.as_deref(),
                            m.genre_ids.as_deref(),
                            &overview_stored.get_value(),
                        );
                        view! {
                            <div class="flex items-center gap-1.5 text-[11px] font-medium text-white/80 flex-wrap pt-0.5">
                                {vibe_tags.into_iter().enumerate().map(|(idx, tag)| {
                                    view! {
                                        {if idx > 0 {
                                            view! { <span class="text-white/40 text-[9px]">"•"</span> }.into_any()
                                        } else {
                                            view! { <span /> }.into_any()
                                        }}
                                        <span>{tag}</span>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }
                </div>
            </div>
        </Portal>
    }
}
