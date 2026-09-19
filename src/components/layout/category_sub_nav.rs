use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::JsCast;
use crate::components::layout::category_sub_nav_mobile::CategorySubNavMobile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubNavGenre {
    pub id: u32,
    pub name: String,
}

#[component]
pub fn CategorySubNav(
    #[prop(into)] title: String,
    genres: Vec<SubNavGenre>,
    selected_genre: ReadSignal<Option<SubNavGenre>>,
    on_genre_select: Callback<Option<SubNavGenre>>,
    #[prop(optional, default = false)] hide_genres_on_desktop: bool,
    #[prop(optional)] dropdown_label: Option<String>,
) -> impl IntoView {
    if hide_genres_on_desktop {
        return view! { <div class="hidden"></div> }.into_any();
    }

    let mobile_title = title.clone();
    let mobile_genres = genres.clone();
    let mobile_dropdown_label = dropdown_label.clone();

    let dt_title = title.clone();
    let dt_genres = genres.clone();
    let dt_dropdown_label = dropdown_label.clone();

    view! {
        // Mobile layout: render in-flow
        <div class="block sm:hidden">
            {if let Some(lbl) = mobile_dropdown_label {
                view! {
                    <CategorySubNavMobile
                        _title=mobile_title
                        genres=mobile_genres
                        selected_genre=selected_genre
                        on_genre_select=on_genre_select
                        dropdown_label=lbl
                    />
                }.into_any()
            } else {
                view! {
                    <CategorySubNavMobile
                        _title=mobile_title
                        genres=mobile_genres
                        selected_genre=selected_genre
                        on_genre_select=on_genre_select
                    />
                }.into_any()
            }}
        </div>

        // Desktop layout: fixed sub-navigation directly below navbar (no portal desync)
        <div class="hidden sm:block">
            <CategorySubNavDesktop
                title=dt_title
                genres=dt_genres
                selected_genre=selected_genre
                on_genre_select=on_genre_select
                dropdown_label=dt_dropdown_label
            />
        </div>
    }.into_any()
}

use leptos_router::hooks::use_query_map;

#[component]
fn CategorySubNavDesktop(
    title: String,
    genres: Vec<SubNavGenre>,
    selected_genre: ReadSignal<Option<SubNavGenre>>,
    on_genre_select: Callback<Option<SubNavGenre>>,
    dropdown_label: Option<String>,
) -> impl IntoView {
    let query_map = use_query_map();
    let initial_open = query_map.with_untracked(|q| q.get("open_genres").is_some());
    let (genre_menu_open, set_genre_menu_open) = signal(initial_open);
    let container_ref = NodeRef::<Div>::new();

    // Close on outside click
    Effect::new(move |_| {
        if genre_menu_open.get() {
            let window = web_sys::window();
            if let Some(win) = window {
                let document = win.document();
                if let Some(doc) = document {
                    let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        if let Some(container) = container_ref.get() {
                            let target = e.target();
                            if let Some(target_node) = target.and_then(|t| t.dyn_into::<web_sys::Node>().ok()) {
                                let container_node: &web_sys::Node = container.as_ref();
                                if !container_node.contains(Some(&target_node)) {
                                    set_genre_menu_open.set(false);
                                }
                            }
                        }
                    }));
                    let _ = doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
                    cb.forget();
                }
            }
        }
    });

    // 3-column transposed layout matching Netflix
    let mut sorted_genres = genres.clone();
    sorted_genres.sort_by(|a, b| a.name.cmp(&b.name));
    let num_cols = 3;
    let num_rows = (sorted_genres.len() + num_cols - 1) / num_cols;
    let mut transposed_genres: Vec<Option<SubNavGenre>> = Vec::new();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let idx = c * num_rows + r;
            if idx < sorted_genres.len() {
                transposed_genres.push(Some(sorted_genres[idx].clone()));
            } else {
                transposed_genres.push(None);
            }
        }
    }

    let (scroll_y, set_scroll_y) = signal(0.0);
    Effect::new(move |_| {
        if let Some(win) = web_sys::window() {
            let win_c = win.clone();
            let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                set_scroll_y.set(win_c.scroll_y().unwrap_or(0.0));
            }));
            let _ = win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    let scroll_bg_style = move || {
        let y = scroll_y.get();
        let op = if y <= 40.0 {
            y / 80.0
        } else {
            (0.5 + (y - 40.0) / 160.0).min(1.0)
        };
        format!("background-color: rgba(20, 20, 20, {:.3});", op)
    };

    let toggle_menu = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_genre_menu_open.update(|o| *o = !*o);
    };

    let title_clone = title.clone();
    let on_genre_click_reset = on_genre_select;

    view! {
        <div
            class="fixed inset-x-0 top-16 z-[79] pointer-events-none transition-colors duration-200"
            style=scroll_bg_style
        >
            <div class="pointer-events-none relative z-30 flex items-center justify-between pl-8 md:pl-[72px] pr-6 md:pr-14 py-5 select-none w-full">
                <div node_ref=container_ref class="pointer-events-auto flex items-center gap-4">
                <h1 class="text-[28px] md:text-[38px] font-bold tracking-[-0.5px] text-white leading-none flex items-center">
                    {
                        let t_c = title_clone.clone();
                        move || {
                            if let Some(genre) = selected_genre.get() {
                                let g_name = genre.name.clone();
                                view! {
                                    <>
                                        <span
                                            on:click=move |_| on_genre_click_reset.run(None)
                                            class="cursor-pointer hover:underline text-white/40 hover:text-white/80 transition-colors whitespace-nowrap text-[15px] md:text-[21px] font-normal"
                                        >
                                            {t_c.clone()}
                                        </span>
                                        <span class="text-white/20 font-normal text-xs md:text-[16px] mx-1.5 md:mx-2.5 whitespace-nowrap">
                                            ">"
                                        </span>
                                        <span class="whitespace-nowrap">
                                            {g_name}
                                        </span>
                                    </>
                                }.into_any()
                            } else {
                                let t = t_c.clone();
                                view! {
                                    <span>{t}</span>
                                }.into_any()
                            }
                        }
                    }
                </h1>

                {
                    let s_g = sorted_genres.clone();
                    let t_g = transposed_genres.clone();
                    let d_l = dropdown_label.clone();
                    move || {
                        if selected_genre.get().is_none() && !s_g.is_empty() {
                            let label = d_l.clone().unwrap_or_else(|| "Genres".to_string());
                            let transposed = t_g.clone();
                            let on_select = on_genre_select;

                            view! {
                                <div class="relative ml-3 md:ml-5">
                                    <button
                                        on:click=toggle_menu
                                        class="flex items-center justify-between min-w-[100px] md:min-w-[125px] px-4 py-2 leading-none text-[13px] md:text-[14px] font-semibold tracking-[-0.2px] text-white bg-white/10 hover:bg-white/20 border border-white/20 backdrop-blur-md transition-all rounded-full active:scale-95 gap-x-2.5 cursor-pointer shadow-sm"
                                        class=("!bg-white/25 !border-white/40", move || genre_menu_open.get())
                                        aria-haspopup="listbox"
                                        aria-expanded=move || genre_menu_open.get()
                                    >
                                        <span>{label}</span>
                                        <i
                                            class="ph-fill ph-caret-down text-[12px] text-white/80 transition-transform duration-200 shrink-0"
                                            class=("rotate-180", move || genre_menu_open.get())
                                        ></i>
                                    </button>

                                    // Dropdown popup
                                    <div
                                        class=move || if genre_menu_open.get() {
                                            "absolute top-[calc(100%+8px)] left-0 z-50 transition-all duration-200 translate-x-0 opacity-100 translate-y-0 pointer-events-auto"
                                        } else {
                                            "absolute top-[calc(100%+8px)] left-0 z-50 transition-all duration-200 translate-x-0 opacity-0 -translate-y-2 pointer-events-none"
                                        }
                                        role="listbox"
                                    >
                                        <div class="w-max max-w-[90vw] md:max-w-none max-h-[65vh] md:max-h-[460px] overflow-y-auto bg-[#141414]/95 backdrop-blur-2xl border border-white/15 rounded-2xl p-4 md:p-6 scrollbar-hide shadow-2xl">
                                            <div class="grid grid-cols-[repeat(3,max-content)] gap-x-6 md:gap-x-10 gap-y-1">
                                                {transposed.into_iter().enumerate().map(|(_idx, opt)| {
                                                    if let Some(genre) = opt {
                                                        let g_clone = genre.clone();
                                                        view! {
                                                            <button
                                                                on:click=move |_| {
                                                                    set_genre_menu_open.set(false);
                                                                    on_select.run(Some(g_clone.clone()));
                                                                }
                                                                role="option"
                                                                aria-selected="false"
                                                                class="text-left text-[14px] md:text-[15px] font-medium transition-all px-3 py-1.5 rounded-lg hover:bg-white/15 whitespace-nowrap text-white/80 hover:text-white cursor-pointer"
                                                            >
                                                                {genre.name}
                                                            </button>
                                                        }.into_any()
                                                    } else {
                                                        view! { <div class="h-6 pointer-events-none"></div> }.into_any()
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }
                }
            </div>
        </div>
    </div>
    }
}
