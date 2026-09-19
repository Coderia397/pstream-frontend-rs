use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::tmdb::{fetch_trending, MediaItem};
use crate::models::movie::Movie;
use crate::store::use_ui_store;
use crate::components::media::movie_card_popup::MovieCardPopup;

fn set_timeout_ms<F: FnOnce() + 'static>(cb: F, ms: i32) -> Option<i32> {
    let window = web_sys::window()?;
    let closure = Closure::once_into_js(cb);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms)
        .ok()
}

fn clear_timeout_id(id: Option<i32>) {
    if let Some(id) = id {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(id);
        }
    }
}

#[component]
fn RankNumber(index: usize) -> impl IntoView {
    let is_ten = index == 9;
    let is_one = index == 0;

    let w_class = if is_ten {
        "w-[130px] sm:w-[140px] md:w-[150px] lg:w-[185px]"
    } else if is_one {
        "w-[105px] sm:w-[115px] md:w-[125px] lg:w-[155px]"
    } else {
        "w-[95px] sm:w-[105px] md:w-[115px] lg:w-[145px]"
    };

    let text_class = "font-black text-[150px] sm:text-[170px] md:text-[190px] lg:text-[245px]";
    let letter_spacing = if is_ten { "-15px" } else { "-5px" };
    let stroke_style = format!(
        "line-height: 0.75; letter-spacing: {}; -webkit-text-stroke: 8px #595959; font-family: 'PStream Sans', sans-serif;",
        letter_spacing
    );
    let fill_style = format!(
        "line-height: 0.75; letter-spacing: {}; font-family: 'PStream Sans', sans-serif;",
        letter_spacing
    );
    let num_str = (index + 1).to_string();

    view! {
        <div class=format!("absolute left-0 top-0 bottom-0 h-full {} flex justify-end items-center pointer-events-none z-0", w_class)>
            <div class="relative flex items-center justify-center">
                // Background thick stroke
                <span
                    class=format!("{} text-transparent absolute select-none", text_class)
                    style=stroke_style
                >
                    {num_str.clone()}
                </span>
                // Foreground sharp fill
                <span
                    class=format!("{} text-[#000000] relative select-none", text_class)
                    style=fill_style
                >
                    {num_str}
                </span>
            </div>
        </div>
    }
}

#[component]
fn TopTenCard(
    item: MediaItem,
    index: usize,
    is_tv: bool,
) -> impl IntoView {
    let ui_store = use_ui_store();
    let movie_id = item.id;
    let my_card_id = format!("topten-{}-{}", if is_tv { "tv" } else { "movie" }, movie_id);

    let (is_hovered, set_is_hovered) = signal(false);
    let (rect, set_rect) = signal::<Option<web_sys::DomRect>>(None);
    let (scroll_pos, set_scroll_pos) = signal((0.0, 0.0));
    let (hover_pos, set_hover_pos) = signal("center".to_string());

    let show_timer = StoredValue::new(None::<i32>);
    let close_timer = StoredValue::new(None::<i32>);

    let card_ref = NodeRef::<leptos::html::Div>::new();

    let is_ten = index == 9;
    let pl_class = if is_ten {
        "pl-[110px] sm:pl-[120px] md:pl-[130px] lg:pl-[165px]"
    } else {
        "pl-[75px] sm:pl-[85px] md:pl-[95px] lg:pl-[125px]"
    };

    let title_stored = StoredValue::new(item.display_title().to_string());
    let poster_url_stored = StoredValue::new(item.poster_url("w342").unwrap_or_default());
    let backdrop_url_stored = StoredValue::new(item.backdrop_url("w780").unwrap_or_default());
    let overview_stored = StoredValue::new(item.overview.clone());
    let vote_avg = item.vote_average;
    let match_percentage = item.match_percentage;
    let vibe_pills = item.vibe_pills.clone();
    let genre_ids = item.genre_ids.clone();

    let cancel_close_cb = Callback::new(move |_| {
        clear_timeout_id(close_timer.get_value());
        close_timer.set_value(None);
    });

    let my_card_id_for_leave = my_card_id.clone();
    let mouse_leave_cb = Callback::new(move |_| {
        clear_timeout_id(show_timer.get_value());
        show_timer.set_value(None);

        clear_timeout_id(close_timer.get_value());
        let card_id = my_card_id_for_leave.clone();
        let timer_id = set_timeout_ms(move || {
            close_timer.set_value(None);
            set_is_hovered.set(false);
            set_rect.set(None);
            if ui_store.active_popup_id.get_untracked().as_deref() == Some(&card_id) {
                ui_store.active_popup_id.set(None);
            }
        }, 150);
        close_timer.set_value(timer_id);
    });

    let my_card_id_for_enter = my_card_id.clone();
    let mouse_enter_cb = Callback::new(move |_| {
        cancel_close_cb.run(());

        if is_hovered.get_untracked() {
            return;
        }
        if show_timer.get_value().is_some() {
            return;
        }

        let card_id = my_card_id_for_enter.clone();
        let timer_id = set_timeout_ms(move || {
            show_timer.set_value(None);
            if let Some(el) = card_ref.get() {
                let r = el.get_bounding_client_rect();
                let win = web_sys::window().unwrap();
                let win_w = win.inner_width().unwrap().as_f64().unwrap_or(1200.0);
                let sx = win.scroll_x().unwrap_or(0.0);
                let sy = win.scroll_y().unwrap_or(0.0);

                let app_x = if win_w >= 1024.0 { 56.0 } else if win_w >= 768.0 { 48.0 } else { 16.0 };
                let half_popup = crate::components::media::movie_card_popup::POPUP_W / 2.0;
                let card_center = r.left() + r.width() / 2.0;

                let number_pl = if win_w >= 1024.0 {
                    if is_ten { 165.0 } else { 125.0 }
                } else if win_w >= 768.0 {
                    if is_ten { 130.0 } else { 95.0 }
                } else {
                    if is_ten { 110.0 } else { 75.0 }
                };
                let item_left = r.left() - number_pl;

                let pos = if item_left <= app_x + 30.0 || card_center - half_popup < app_x {
                    "left"
                } else if card_center + half_popup > win_w - app_x - 15.0 {
                    "right"
                } else {
                    "center"
                };

                set_hover_pos.set(pos.to_string());
                set_rect.set(Some(r));
                set_scroll_pos.set((sx, sy));
                set_is_hovered.set(true);
                ui_store.active_popup_id.set(Some(card_id));
            }
        }, 20);
        show_timer.set_value(timer_id);
    });

    let my_card_id_for_effect = my_card_id.clone();
    Effect::new(move |_| {
        let active = ui_store.active_popup_id.get();
        if is_hovered.get() {
            if let Some(ref id) = active {
                if id != &my_card_id_for_effect {
                    clear_timeout_id(show_timer.get_value());
                    clear_timeout_id(close_timer.get_value());
                    show_timer.set_value(None);
                    close_timer.set_value(None);
                    set_is_hovered.set(false);
                    set_rect.set(None);
                }
            }
        }
    });

    on_cleanup(move || {
        clear_timeout_id(show_timer.get_value());
        clear_timeout_id(close_timer.get_value());
    });

    let open_modal = move |_| {
        set_is_hovered.set(false);
        set_rect.set(None);
        ui_store.active_popup_id.set(None);
        ui_store.hero_paused_by_modal.set(false);
        ui_store.modal_initial_time.set(0.0);
        ui_store.modal_current_time.set(0.0);
        ui_store.info_modal_movie_id.set(Some(movie_id));
        ui_store.info_modal_is_tv.set(is_tv);
        ui_store.info_modal_open.set(true);
    };

    let title_for_img = title_stored.with_value(|t| t.clone());
    let poster_src = poster_url_stored.with_value(|p| p.clone());

    view! {
        <div
            data-card="true"
            data-card-id=my_card_id
            class=format!("relative flex-none flex items-end {} pr-1 md:pr-1.5 lg:pr-2 cursor-pointer", pl_class)
            on:mouseenter=move |_| mouse_enter_cb.run(())
            on:mouseleave=move |_| mouse_leave_cb.run(())
            on:click=open_modal
        >
            <RankNumber index=index />

            // Poster card — card_ref is HERE so popup positions relative to the poster,
            // not the outer div (which includes the wide number padding-left).
            <div
                node_ref=card_ref
                class="relative flex-none h-[128px] w-[89px] sm:h-[138px] sm:w-[96px] md:h-[148px] md:w-[103px] lg:h-[195px] lg:w-[135px] z-10 rounded-[4px] md:rounded-[8px] overflow-hidden mb-1 sm:mb-1.5 md:mb-2 shadow-[0_0_15px_rgba(0,0,0,0.5)] movie-card-glow">
                <img
                    src=poster_src
                    alt=title_for_img.clone()
                    class="w-full h-full object-cover object-top rounded-[4px] md:rounded-[8px]"
                    loading="lazy"
                    draggable="false"
                />

                // Watch Progress Bar
                {move || {
                    let watch_store = crate::store::use_watch_store();
                    if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                        if rec.percentage > 0.0 {
                            let pct = rec.percentage.clamp(0.0, 100.0);
                            view! {
                                <div class="absolute bottom-0 inset-x-0 h-1 bg-white/20 z-10 pointer-events-none">
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
            </div>

            {move || {
                if is_hovered.get() {
                    if let Some(r) = rect.get() {
                        let t = title_stored.with_value(|t| t.clone());
                        let b = backdrop_url_stored.with_value(|b| b.clone());
                        let p = poster_url_stored.with_value(|p| p.clone());
                        let ov = overview_stored.with_value(|ov| ov.clone());
                        let m = Movie {
                            id: serde_json::json!(movie_id),
                            title: if !is_tv { Some(t.clone()) } else { None },
                            name: if is_tv { Some(t) } else { None },
                            backdrop_path: if !b.is_empty() { Some(b) } else { None },
                            poster_path: if !p.is_empty() { Some(p) } else { None },
                            overview: ov,
                            vote_average: vote_avg,
                            media_type: Some(if is_tv { "tv".to_string() } else { "movie".to_string() }),
                            match_percentage,
                            genre_ids: if genre_ids.is_empty() { None } else { Some(genre_ids.iter().map(|&g| g as i64).collect()) },
                            vibe_pills: if vibe_pills.is_empty() { None } else { Some(vibe_pills.clone()) },
                            ..Default::default()
                        };
                        let on_play_cb = Callback::new(move |_| {
                            let navigate = leptos_router::hooks::use_navigate();
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
                        });
                        let on_modal_cb = Callback::new(move |_| {
                            set_is_hovered.set(false);
                            set_rect.set(None);
                            ui_store.active_popup_id.set(None);
                            ui_store.info_modal_movie_id.set(Some(movie_id));
                            ui_store.info_modal_is_tv.set(is_tv);
                            ui_store.info_modal_open.set(true);
                        });
                        view! {
                            <MovieCardPopup
                                movie=m
                                hovered_rect=r
                                initial_scroll=scroll_pos.get()
                                hover_position=hover_pos.get()
                                on_mouse_enter=cancel_close_cb
                                on_mouse_leave=mouse_leave_cb
                                on_play=on_play_cb
                                on_open_modal=on_modal_cb
                            />
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }
                } else {
                    view! { <span /> }.into_any()
                }
            }}
        </div>
    }
}

#[component]
pub fn TopTenRow(
    #[prop(into, optional, default = "Top 10 in the UK Today".to_string())] title: String,
    #[prop(into, optional, default = "movie".to_string())] kind: String,
    #[prop(default = None)] items: Option<Vec<MediaItem>>,
) -> impl IntoView {
    let is_tv = kind == "tv";
    let items_prop = items.clone();
    let kind_c = kind.clone();
    let items_res = LocalResource::new(move || {
        let items_opt = items_prop.clone();
        let kind_str = kind_c.clone();
        async move {
            if let Some(it) = items_opt {
                if !it.is_empty() {
                    return it.into_iter().take(10).collect::<Vec<_>>();
                }
            }
            let results = fetch_trending(&kind_str).await.unwrap_or_default();
            results.into_iter().take(10).collect::<Vec<_>>()
        }
    });

    let scroll_ref = NodeRef::<leptos::html::Div>::new();
    let (can_scroll_left, set_can_scroll_left) = signal(false);
    let (can_scroll_right, set_can_scroll_right) = signal(true);

    let update_scroll_state = Callback::new(move |_: ()| {
        if let Some(el) = scroll_ref.get() {
            let cur = el.scroll_left() as f64;
            let client_w = el.client_width() as f64;
            let scroll_w = el.scroll_width() as f64;
            set_can_scroll_left.set(cur > 10.0);
            set_can_scroll_right.set(scroll_w > client_w + 10.0 && scroll_w - (cur + client_w) > 10.0);
        }
    });

    // Window resize listener
    Effect::new({
        let update = update_scroll_state.clone();
        move |_| {
            if let Some(win) = web_sys::window() {
                let update_c = update.clone();
                let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                    update_c.run(());
                }));
                let _ = win.add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    });

    let on_scroll = {
        let update = update_scroll_state.clone();
        move |_| update.run(())
    };

    let scroll = {
        let update = update_scroll_state.clone();
        move |direction: &str| {
            if let Some(el) = scroll_ref.get() {
                let client_width = el.client_width() as f64;
                let current_scroll = el.scroll_left() as f64;
                let step = client_width * 0.9;
                let target = if direction == "left" {
                    (current_scroll - step).max(0.0)
                } else {
                    current_scroll + step
                };
                el.scroll_to_with_x_and_y(target, 0.0);

                let update_c = update.clone();
                let _ = set_timeout_ms(move || {
                    update_c.run(());
                }, 600);
            }
        }
    };

    view! {
        <div class="group relative my-4 md:my-6 space-y-2 z-10">
            <h2 class="px-[var(--app-x,56px)] text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit">
                {title}
                <span class="text-xs text-cyan-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                    "Explore All ›"
                </span>
            </h2>

            <div class="relative group/row" style="container-type: inline-size;">
                <div
                    node_ref=scroll_ref
                    on:scroll=on_scroll
                    class="flex overflow-x-scroll [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden pt-2 pb-10 -mb-10 w-full items-end pointer-events-auto relative z-10"
                    style="scroll-behavior: smooth;"
                >
                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />

                    <Suspense fallback=move || view! {
                        { (0..6).map(|_| view! {
                            <div class="relative flex-none flex items-end pl-[75px] sm:pl-[85px] md:pl-[95px] lg:pl-[125px] pr-1 md:pr-1.5 lg:pr-2">
                                <div class="absolute left-0 top-0 bottom-0 h-full w-[95px] sm:w-[105px] md:w-[115px] lg:w-[145px] flex justify-end items-center pointer-events-none z-0">
                                    <div class="h-[85%] w-[80%] bg-[#222] rounded-sm opacity-40 skew-x-[-6deg]" />
                                </div>
                                <div class="relative flex-none h-[128px] w-[89px] sm:h-[138px] sm:w-[96px] md:h-[148px] md:w-[103px] lg:h-[195px] lg:w-[135px] z-10 bg-[#222] rounded-[4px] md:rounded-[8px] border border-white/5 overflow-hidden mb-1 sm:mb-1.5 md:mb-2">
                                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/5 to-transparent -translate-x-full animate-[shimmer_1.5s_infinite]" />
                                </div>
                            </div>
                        }).collect::<Vec<_>>() }
                    }>
                        {move || items_res.get().map(|media| {
                            media.into_iter().enumerate().map(|(idx, item)| {
                                view! {
                                    <TopTenCard item=item index=idx is_tv=is_tv />
                                }
                            }).collect::<Vec<_>>()
                        })}
                    </Suspense>

                    <div class="flex-none pointer-events-none" style="width: var(--app-x, 56px);" />
                </div>

                // Left arrow — hidden at position 0, same style as normal Row
                <button
                    type="button"
                    class=move || if can_scroll_left.get() {
                        "hidden md:flex absolute top-2 bottom-0 left-0 z-30 items-center justify-center cursor-pointer bg-transparent hover:bg-black/70 transition-all duration-200 opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto select-none border-none outline-none"
                    } else {
                        "hidden !pointer-events-none !opacity-0"
                    }
                    style="width: var(--app-x, 56px);"
                    on:click={
                        let scroll_left = scroll.clone();
                        move |_| scroll_left("left")
                    }
                    aria-label="Scroll Left"
                >
                    <i class="ph-bold ph-caret-left text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                </button>

                // Right arrow — hidden when at end
                <button
                    type="button"
                    class=move || if can_scroll_right.get() {
                        "hidden md:flex absolute top-2 bottom-0 right-0 z-30 items-center justify-center cursor-pointer bg-transparent hover:bg-black/70 transition-all duration-200 opacity-0 group-hover/row:opacity-100 group-hover/row:pointer-events-auto select-none border-none outline-none"
                    } else {
                        "hidden !pointer-events-none !opacity-0"
                    }
                    style="width: var(--app-x, 56px);"
                    on:click={
                        let scroll_right = scroll.clone();
                        move |_| scroll_right("right")
                    }
                    aria-label="Scroll Right"
                >
                    <i class="ph-bold ph-caret-right text-white text-3xl sm:text-4xl drop-shadow-lg transition-transform hover:scale-125"></i>
                </button>
            </div>
        </div>
    }
}
