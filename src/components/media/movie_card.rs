use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use crate::models::movie::Movie;
use crate::services::tmdb::fetch_movie_logo;
use crate::store::use_ui_store;

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
pub fn MovieCard(
    movie_id: u32,
    is_tv: bool,
    title: String,
    backdrop_path: String,
    #[prop(optional)] poster_path: Option<String>,
    vote_average: f64,
    #[prop(optional, default = false)] is_grid: bool,
) -> impl IntoView {
    let ui_store = use_ui_store();
    let my_card_id = format!("{}-{}", if is_tv { "tv" } else { "movie" }, movie_id);

    let (is_hovered, set_is_hovered) = signal(false);
    let (rect, set_rect) = signal::<Option<web_sys::DomRect>>(None);
    let (scroll_pos, set_scroll_pos) = signal((0.0, 0.0));
    let (hover_pos, set_hover_pos) = signal("center".to_string());

    let show_timer = StoredValue::new(None::<i32>);
    let close_timer = StoredValue::new(None::<i32>);

    let open_modal = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        ui_store.info_modal_movie_id.set(Some(movie_id));
        ui_store.info_modal_is_tv.set(is_tv);
        ui_store.info_modal_open.set(true);
    };

    let img_src = if !backdrop_path.is_empty() {
        backdrop_path.clone()
    } else {
        poster_path.clone().unwrap_or_default()
    };

    let title_stored = StoredValue::new(title);
    let backdrop_stored = StoredValue::new(backdrop_path);
    
    let logo_resource = LocalResource::new(move || async move {
        fetch_movie_logo(movie_id, is_tv).await.ok().flatten()
    });

    let card_ref = NodeRef::<leptos::html::Div>::new();

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

                let edge_buffer = 120.0;
                let pos = if r.left() < edge_buffer {
                    "left"
                } else if win_w - r.right() < edge_buffer {
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

    // Close when another card popup opens
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

    let card_class = if is_grid {
        "relative group group/card select-none w-full aspect-video cursor-pointer"
    } else {
        "relative group group/card select-none flex-none h-[128px] aspect-video cursor-pointer"
    };

    view! {
        <div 
            node_ref=card_ref
            data-card="true"
            data-card-id=my_card_id.clone()
            class=card_class
            class=("z-[9]", move || is_hovered.get())
            class=("z-10", move || !is_hovered.get())
            on:mouseenter=move |_| mouse_enter_cb.run(())
            on:mouseleave=move |_| mouse_leave_cb.run(())
            on:click=open_modal
        >
            <div class="w-full h-full relative rounded-sm overflow-hidden movie-card-glow">
                <img
                    src=img_src
                    alt=title_stored.with_value(|t| t.clone())
                    class="w-full h-full object-cover object-center rounded-sm backdrop-pop"
                    loading="lazy"
                    decoding="async"
                    draggable="false"
                />

                // Watch Progress Bar if user has watched this title
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

                {move || {
                    if !is_hovered.get() {
                        let _t = title_stored.with_value(|t| t.clone());
                        view! {
                            <div class="absolute inset-x-0 bottom-0 flex items-end justify-center pb-2 px-2 transition-opacity duration-200 pointer-events-none">
                                <Suspense fallback=move || {
                                    let t = title_stored.with_value(|t| t.clone());
                                    view! {
                                        <h3 class="text-white font-bold text-center tracking-wide leading-tight drop-shadow-[0_2px_8px_rgba(0,0,0,1)] line-clamp-2 w-full px-1 text-[15px]">
                                            {t}
                                        </h3>
                                    }
                                }>
                                    {move || {
                                        let t = title_stored.with_value(|t| t.clone());
                                        logo_resource.get().map(|res| match res {
                                            Some(url) if !url.is_empty() => view! {
                                                <img
                                                    src=url
                                                    alt=t
                                                    class="w-auto max-w-[80%] max-h-[44px] object-contain origin-bottom drop-shadow-[0_2px_6px_rgba(0,0,0,0.6)]"
                                                    decoding="async"
                                                    draggable="false"
                                                />
                                            }.into_any(),
                                            _ => view! {
                                                <h3 class="text-white font-bold text-center tracking-wide leading-tight drop-shadow-[0_2px_8px_rgba(0,0,0,1)] line-clamp-2 w-full px-1 text-[15px]">
                                                    {t}
                                                </h3>
                                            }.into_any(),
                                        })
                                    }}
                                </Suspense>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span /> }.into_any()
                    }
                }}
            </div>

            {move || {
                if is_hovered.get() {
                    if let Some(r) = rect.get() {
                        let m = Movie {
                            id: serde_json::json!(movie_id),
                            title: if !is_tv { Some(title_stored.with_value(|t| t.clone())) } else { None },
                            name: if is_tv { Some(title_stored.with_value(|t| t.clone())) } else { None },
                            backdrop_path: Some(backdrop_stored.with_value(|b| b.clone())),
                            poster_path: poster_path.clone(),
                            vote_average,
                            media_type: Some(if is_tv { "tv".to_string() } else { "movie".to_string() }),
                            ..Default::default()
                        };
                        let on_play_cb = Callback::new(move |_| {
                            let navigate = leptos_router::hooks::use_navigate();
                            let watch_store = crate::store::use_watch_store();
                            let target = if let Some(rec) = watch_store.get_record(movie_id, is_tv) {
                                if is_tv && rec.season.is_some() && rec.episode.is_some() {
                                    format!("/watch/{}?season={}&episode={}", movie_id, rec.season.unwrap(), rec.episode.unwrap())
                                } else {
                                    format!("/watch/{}", movie_id)
                                }
                            } else {
                                format!("/watch/{}", movie_id)
                            };
                            navigate(&target, Default::default());
                        });
                        let on_modal_cb = Callback::new(move |_| {
                            ui_store.info_modal_movie_id.set(Some(movie_id));
                            ui_store.info_modal_is_tv.set(is_tv);
                            ui_store.info_modal_open.set(true);
                        });
                        view! {
                            <crate::components::media::movie_card_popup::MovieCardPopup
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
