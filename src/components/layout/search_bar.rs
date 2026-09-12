use leptos::prelude::*;
use leptos::html::{Div, Input};
use leptos_router::hooks::{use_navigate, use_query_map};
use wasm_bindgen::JsCast;

#[component]
pub fn SearchBar() -> impl IntoView {
    let query_map = use_query_map();
    let query_val = move || query_map.read().get("q").unwrap_or_default();
    
    let (is_active, set_is_active) = signal(false);
    let (input_val, set_input_val) = signal(String::new());
    
    let input_ref = NodeRef::<Input>::new();
    let container_ref = NodeRef::<Div>::new();

    let location = leptos_router::hooks::use_location();
    let is_search_page = move || location.pathname.get().starts_with("/search");

    // Sync input val with URL on mount or URL change
    Effect::new(move |_| {
        let q = query_val();
        set_input_val.set(q.clone());
        if !q.is_empty() || is_search_page() {
            set_is_active.set(true);
        }
    });

    // Auto-focus input when opened
    Effect::new(move |_| {
        if is_active.get() || is_search_page() {
            if let Some(input_el) = input_ref.get() {
                let _ = input_el.focus();
            }
        }
    });

    // Handle outside clicks to collapse search if empty and not on /search
    Effect::new(move |_| {
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
                                if !is_search_page() && input_val.get().trim().is_empty() {
                                    set_is_active.set(false);
                                }
                            }
                        }
                    }
                }));

                let _ = doc.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    });

    let navigate = use_navigate();
    let timer_id = StoredValue::new(None::<i32>);
    
    // 300ms live debounced search query execution (Task 090)
    let navigate_debounced = navigate.clone();
    let on_input = move |e: leptos::ev::Event| {
        let val = event_target_value(&e);
        set_input_val.set(val.clone());
        
        // Cancel previous timer
        timer_id.update_value(|id| {
            if let Some(handle) = id.take() {
                if let Some(win) = web_sys::window() {
                    win.clear_timeout_with_handle(handle);
                }
            }
        });

        let nav = navigate_debounced.clone();
        let query = val.clone();
        let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
            let target_url = if query.trim().is_empty() {
                "/search".to_string()
            } else {
                let encoded = js_sys::encode_uri_component(&query).as_string().unwrap_or(query.clone());
                format!("/search?q={}", encoded)
            };
            nav(&target_url, Default::default());
        }));

        if let Some(win) = web_sys::window() {
            if let Ok(id) = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 300) {
                timer_id.set_value(Some(id));
            }
        }
        cb.forget();
    };

    let navigate_toggle = navigate.clone();
    let toggle_search = move |_| {
        let active = is_active.get();
        if !active {
            set_is_active.set(true);
            if let Some(input_el) = input_ref.get() {
                let _ = input_el.focus();
            }
            if input_val.get().is_empty() {
                navigate_toggle("/search", Default::default());
            }
        }
    };

    // Instant clear (X) button (Task 090)
    let stored_navigate = StoredValue::new(navigate.clone());
    let clear_search = move |_| {
        timer_id.update_value(|id| {
            if let Some(handle) = id.take() {
                if let Some(win) = web_sys::window() {
                    win.clear_timeout_with_handle(handle);
                }
            }
        });
        set_input_val.set("".to_string());
        stored_navigate.with_value(|n| n("/search", Default::default()));
        if let Some(input_el) = input_ref.get() {
            let _ = input_el.focus();
        }
    };

    let on_keydown = move |e: leptos::ev::KeyboardEvent| {
        if e.key() == "Escape" {
            timer_id.update_value(|id| {
                if let Some(handle) = id.take() {
                    if let Some(win) = web_sys::window() {
                        win.clear_timeout_with_handle(handle);
                    }
                }
            });
            set_input_val.set("".to_string());
            stored_navigate.with_value(|n| n("/browse", Default::default()));
            set_is_active.set(false);
        }
    };

    let effective_active = move || is_active.get() || is_search_page() || !input_val.get().is_empty();

    view! {
        <div node_ref=container_ref class="relative flex items-center">
            <div 
                class="relative flex items-center transition-all duration-300 ease-out p-1 overflow-hidden"
                class=("bg-black/80", effective_active)
                class=("border", effective_active)
                class=("border-white", effective_active)
                class=("w-[180px]", effective_active)
                class=("sm:w-[220px]", effective_active)
                class=("md:w-[280px]", effective_active)
                class=("bg-transparent", move || !effective_active())
                class=("border-transparent", move || !effective_active())
                class=("w-8", move || !effective_active())
            >
                <button
                    on:click=toggle_search
                    class="focus:outline-none flex items-center justify-center z-10 shrink-0"
                    aria-label="Search"
                >
                    <i class="ph-bold ph-magnifying-glass text-[20px] cursor-pointer select-none transition-colors duration-300"
                       class=("text-white", effective_active)
                       class=("hover:text-gray-300", move || !effective_active())>
                    </i>
                </button>

                <input
                    node_ref=input_ref
                    type="text"
                    placeholder="Titles, people, genres"
                    class="bg-transparent border-none outline-none text-white text-xs md:text-sm ml-2 transition-all duration-300 font-sans"
                    class=("w-full", effective_active)
                    class=("opacity-100", effective_active)
                    class=("w-0", move || !effective_active())
                    class=("opacity-0", move || !effective_active())
                    class=("pointer-events-none", move || !effective_active())
                    prop:value=move || input_val.get()
                    on:input=on_input
                    on:keydown=on_keydown
                    aria-label="Search Input"
                />

                <Show when=move || effective_active()>
                    <button
                        on:mousedown=clear_search
                        class="text-white/70 hover:text-white cursor-pointer mx-1 flex-shrink-0 transition-all duration-200 hover:scale-125 active:scale-95"
                        aria-label="Clear Search"
                    >
                        <i class="ph-bold ph-x text-[20px]"></i>
                    </button>
                </Show>
            </div>
        </div>
    }
}
