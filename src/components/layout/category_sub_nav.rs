use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::JsCast;

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
    #[prop(optional)] view_mode: Option<ReadSignal<String>>,
    #[prop(optional)] on_view_mode_change: Option<Callback<String>>,
    #[prop(optional, default = false)] hide_genres_on_desktop: bool,
    #[prop(optional)] dropdown_label: Option<String>,
) -> impl IntoView {
    if hide_genres_on_desktop {
        return view! { <div class="hidden"></div> }.into_any();
    }

    let (genre_menu_open, set_genre_menu_open) = signal(false);
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

    let toggle_menu = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        set_genre_menu_open.update(|o| *o = !*o);
    };

    let title_clone = title.clone();
    let on_genre_click_reset = on_genre_select;

    view! {
        <div class="relative z-30 flex items-center justify-between px-6 md:px-14 py-3 select-none w-full">
            <div node_ref=container_ref class="flex items-center gap-4">
                <h1 class="text-[28px] md:text-[38px] font-bold tracking-[-0.5px] text-white leading-none flex items-center">
                    {move || {
                        if let Some(genre) = selected_genre.get() {
                            let g_name = genre.name.clone();
                            view! {
                                <>
                                    <span
                                        on:click=move |_| on_genre_click_reset.run(None)
                                        class="cursor-pointer hover:underline text-white/40 hover:text-white/80 transition-colors whitespace-nowrap text-[15px] md:text-[21px] font-normal"
                                    >
                                        {title_clone.clone()}
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
                            view! {
                                <span>{title.clone()}</span>
                            }.into_any()
                        }
                    }}
                </h1>

                {move || {
                    if selected_genre.get().is_none() && !sorted_genres.is_empty() {
                        let label = dropdown_label.clone().unwrap_or_else(|| "Genres".to_string());
                        let transposed = transposed_genres.clone();
                        let on_select = on_genre_select;

                        view! {
                            <div class="relative ml-3 md:ml-5">
                                <button
                                    on:click=toggle_menu
                                    class="flex items-center justify-between min-w-[95px] md:min-w-[115px] px-3 py-[5px] leading-none text-[13px] md:text-[14px] font-bold tracking-[-0.2px] text-white bg-black hover:bg-white/5 border border-white/80 transition-colors rounded-none active:scale-95 gap-x-2 cursor-pointer"
                                    class=("bg-white/5", move || genre_menu_open.get())
                                    aria-haspopup="listbox"
                                    aria-expanded=move || genre_menu_open.get()
                                >
                                    <span>{label}</span>
                                    <i
                                        class="ph-fill ph-caret-down text-[12px] text-white transition-transform duration-200 shrink-0"
                                        class=("rotate-180", move || genre_menu_open.get())
                                    ></i>
                                </button>

                                // Dropdown popup
                                <div
                                    class=move || if genre_menu_open.get() {
                                        "absolute top-full left-0 z-50 transition-all duration-200 translate-x-0 opacity-100 translate-y-0 pointer-events-auto"
                                    } else {
                                        "absolute top-full left-0 z-50 transition-all duration-200 translate-x-0 opacity-0 -translate-y-1 pointer-events-none"
                                    }
                                    role="listbox"
                                >
                                    <div class="w-max max-w-[90vw] md:max-w-none max-h-[60vh] md:max-h-[400px] overflow-y-auto bg-[rgba(0,0,0,0.95)] border border-white/10 rounded-none pt-2 pb-2 pl-3 pr-6 md:pt-3 md:pb-3 md:pl-4 md:pr-8 scrollbar-hide shadow-2xl">
                                        <div class="grid grid-cols-[repeat(3,max-content)] gap-x-6 md:gap-x-8 gap-y-1.5">
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
                                                            class="text-left text-[14px] md:text-[15px] font-normal transition-colors hover:underline whitespace-nowrap text-white cursor-pointer"
                                                        >
                                                            {genre.name}
                                                        </button>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div class="h-4 pointer-events-none"></div>
                                                    }.into_any()
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
                }}
            </div>

            // View Mode Switcher: row vs grid (Task 091)
            {move || {
                if let (Some(mode_sig), Some(on_mode_change)) = (view_mode, on_view_mode_change) {
                    let on_row = move |_| on_mode_change.run("row".to_string());
                    let on_grid = move |_| on_mode_change.run("grid".to_string());
                    view! {
                        <div class="flex items-center bg-[#1a1a1a] rounded-full border border-white/10 p-0.5">
                            <button
                                on:click=on_row
                                class=move || if mode_sig.get() == "row" {
                                    "p-1.5 rounded-full transition-all duration-200 cursor-pointer bg-white/10 text-white"
                                } else {
                                    "p-1.5 rounded-full transition-all duration-200 cursor-pointer text-white/40 hover:text-white/70"
                                }
                                title="Row view"
                                aria-label="Row view"
                            >
                                <i class="ph-bold ph-rows text-[16px]"></i>
                            </button>
                            <button
                                on:click=on_grid
                                class=move || if mode_sig.get() == "grid" {
                                    "p-1.5 rounded-full transition-all duration-200 cursor-pointer bg-white/10 text-white"
                                } else {
                                    "p-1.5 rounded-full transition-all duration-200 cursor-pointer text-white/40 hover:text-white/70"
                                }
                                title="Grid view"
                                aria-label="Grid view"
                            >
                                <i class="ph-bold ph-squares-four text-[16px]"></i>
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }.into_any()
}
