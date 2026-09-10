use leptos::prelude::*;
use wasm_bindgen::prelude::*;
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
        "w-[130px] sm:w-[140px] md:w-[150px] lg:w-[160px]"
    } else if is_one {
        "w-[105px] sm:w-[115px] md:w-[125px] lg:w-[135px]"
    } else {
        "w-[95px] sm:w-[105px] md:w-[115px] lg:w-[125px]"
    };

    let text_class = "font-black text-[150px] sm:text-[170px] md:text-[190px] lg:text-[210px]";
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
        "pl-[110px] sm:pl-[120px] md:pl-[130px] lg:pl-[140px]"
    } else {
        "pl-[75px] sm:pl-[85px] md:pl-[95px] lg:pl-[105px]"
    };

    let title_stored = StoredValue::new(item.display_title().to_string());
    let poster_url_stored = StoredValue::new(item.poster_url("w342").unwrap_or_default());
    let backdrop_url_stored = StoredValue::new(item.backdrop_url("w780").unwrap_or_default());
    let overview_stored = StoredValue::new(item.overview.clone());
    let vote_avg = item.vote_average;

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
        ui_store.info_modal_movie_id.set(Some(movie_id));
        ui_store.info_modal_is_tv.set(is_tv);
        ui_store.info_modal_open.set(true);
    };

    let title_for_img = title_stored.with_value(|t| t.clone());
    let poster_src = poster_url_stored.with_value(|p| p.clone());

    view! {
        <div
            node_ref=card_ref
            data-card="true"
            data-card-id=my_card_id
            class=format!("relative flex-none flex items-end {} pr-1 md:pr-1.5 lg:pr-2 cursor-pointer", pl_class)
            on:mouseenter=move |_| mouse_enter_cb.run(())
            on:mouseleave=move |_| mouse_leave_cb.run(())
            on:click=open_modal
        >
            <RankNumber index=index />

            <div class="relative flex-none h-[128px] w-[89px] sm:h-[138px] sm:w-[96px] md:h-[148px] md:w-[103px] lg:h-[163px] lg:w-[114px] z-10 rounded-sm overflow-hidden mb-1 sm:mb-1.5 md:mb-2 shadow-[0_0_15px_rgba(0,0,0,0.5)]">
                <img
                    src=poster_src
                    alt=title_for_img
                    class="w-full h-full object-cover object-top"
                    loading="lazy"
                    draggable="false"
                />
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
                            ..Default::default()
                        };
                        let on_play_cb = Callback::new(move |_| {
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate(&format!("/watch/{}", movie_id), Default::default());
                        });
                        let on_modal_cb = Callback::new(move |_| {
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
    title: &'static str,
    #[prop(optional, default = "movie")] kind: &'static str,
) -> impl IntoView {
    let is_tv = kind == "tv";
    let items = LocalResource::new(move || async move {
        let results = fetch_trending(kind).await.unwrap_or_default();
        results.into_iter().take(10).collect::<Vec<_>>()
    });

    let scroll_ref = NodeRef::<leptos::html::Div>::new();

    let scroll = move |direction: &str| {
        if let Some(el) = scroll_ref.get() {
            let client_width = el.client_width() as f64;
            let current_scroll = el.scroll_left() as f64;
            let step = client_width * 0.9;
            let target = if direction == "left" { current_scroll - step } else { current_scroll + step };
            el.scroll_to_with_x_and_y(target, 0.0);
        }
    };

    let btn_base = "absolute top-1/2 -translate-y-1/2 mb-0.5 sm:mb-[3px] md:mb-1 z-50 h-[128px] sm:h-[138px] md:h-[148px] lg:h-[163px] w-12 md:w-16 lg:w-20 bg-black/50 hover:bg-black/70 cursor-pointer flex items-center justify-center transition-[opacity,background-color] duration-200 opacity-0 pointer-events-none";

    view! {
        <div class="group relative my-4 md:my-6 space-y-2 z-10">
            <h2 class="px-[var(--app-x,56px)] text-sm sm:text-base md:text-lg font-bold text-[#e5e5e5] hover:text-white transition cursor-pointer flex items-center group/title w-fit">
                {title}
                <span class="text-xs text-cyan-500 ml-2 opacity-0 group-hover/title:opacity-100 transition-opacity duration-300 flex items-center font-semibold">
                    "Explore All"
                    <span class="ml-1 text-sm">"›"</span>
                </span>
            </h2>

            <div class="relative group/row">
                {
                    let scroll_left = scroll.clone();
                    view! {
                        <button
                            class=format!("{} left-0 rounded-r-md group-hover/row:opacity-100 group-hover/row:pointer-events-auto", btn_base)
                            on:click=move |_| scroll_left("left")
                            aria-label="Scroll Left"
                        >
                            <span class="text-white text-3xl font-bold">"‹"</span>
                        </button>
                    }
                }

                <div
                    node_ref=scroll_ref
                    class="flex overflow-x-scroll [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden py-10 -my-10 w-full items-center pointer-events-auto relative z-10"
                >
                    <div class="flex-none h-full pointer-events-none" style="width: var(--app-x, 56px);" />

                    <Suspense fallback=move || view! {
                        { (0..6).map(|_| view! {
                            <div class="relative flex-none flex items-end pl-[75px] sm:pl-[85px] md:pl-[95px] lg:pl-[105px] pr-1 md:pr-1.5 lg:pr-2">
                                <div class="absolute left-0 top-0 bottom-0 h-full w-[95px] sm:w-[105px] md:w-[115px] lg:w-[125px] flex justify-end items-center pointer-events-none z-0">
                                    <div class="h-[85%] w-[80%] bg-[#222] rounded-sm opacity-40 skew-x-[-6deg]" />
                                </div>
                                <div class="relative flex-none h-[128px] w-[89px] sm:h-[138px] sm:w-[96px] md:h-[148px] md:w-[103px] lg:h-[163px] lg:w-[114px] z-10 bg-[#222] rounded-sm border border-white/5 overflow-hidden mb-1 sm:mb-1.5 md:mb-2">
                                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/5 to-transparent -translate-x-full animate-[shimmer_1.5s_infinite]" />
                                </div>
                            </div>
                        }).collect::<Vec<_>>() }
                    }>
                        {move || items.get().map(|media| {
                            media.into_iter().enumerate().map(|(idx, item)| {
                                view! {
                                    <TopTenCard item=item index=idx is_tv=is_tv />
                                }
                            }).collect::<Vec<_>>()
                        })}
                    </Suspense>

                    <div class="flex-none pointer-events-none" style="width: var(--app-x, 56px);" />
                </div>

                {
                    let scroll_right = scroll.clone();
                    view! {
                        <button
                            class=format!("{} right-0 rounded-l-md group-hover/row:opacity-100 group-hover/row:pointer-events-auto", btn_base)
                            on:click=move |_| scroll_right("right")
                            aria-label="Scroll Right"
                        >
                            <span class="text-white text-3xl font-bold">"›"</span>
                        </button>
                    }
                }
            </div>
        </div>
    }
}

