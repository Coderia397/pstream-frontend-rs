use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::JsCast;
use std::collections::HashSet;
use crate::store::use_ui_store;
use crate::services::tmdb::fetch_trending;

#[derive(Clone, Debug, PartialEq)]
pub struct AppNotification {
    pub id: String,
    pub movie_id: u32,
    pub is_tv: bool,
    pub headline: String,
    pub body: String,
    pub backdrop: String,
    pub relative_time: String,
}

fn load_read_ids() -> HashSet<String> {
    let mut set = HashSet::new();
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(raw)) = storage.get_item("pstream-notifications-read:default") {
            if let Ok(parsed) = serde_json::from_str::<Vec<String>>(&raw) {
                for id in parsed {
                    set.insert(id);
                }
            }
        }
    }
    set
}

fn save_read_ids(ids: &HashSet<String>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let vec: Vec<&String> = ids.iter().collect();
        if let Ok(json) = serde_json::to_string(&vec) {
            let _ = storage.set_item("pstream-notifications-read:default", &json);
        }
    }
}

#[component]
pub fn NotificationsDropdown() -> impl IntoView {
    let ui_store = use_ui_store();
    let (open, set_open) = signal(false);
    let (read_ids, set_read_ids) = signal(load_read_ids());
    let (is_loading, set_is_loading) = signal(true);
    let (items, set_items) = signal(Vec::<AppNotification>::new());

    let root_ref = NodeRef::<Div>::new();

    // Fetch trending media to generate real notification arrivals
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            let movies = fetch_trending("movie").await.unwrap_or_default();
            let shows = fetch_trending("tv").await.unwrap_or_default();
            
            let mut list = Vec::new();
            let mut count = 0;
            
            for (idx, m) in movies.into_iter().enumerate() {
                if count >= 8 { break; }
                if let Some(backdrop) = m.backdrop_url("w300").or_else(|| m.poster_url("w300")) {
                    let rel = match idx {
                        0 | 1 => "Today".to_string(),
                        2 | 3 => "1 day ago".to_string(),
                        4 | 5 => "2 days ago".to_string(),
                        _ => format!("{} days ago", idx),
                    };
                    list.push(AppNotification {
                        id: format!("notif-movie-{}", m.id),
                        movie_id: m.id,
                        is_tv: false,
                        headline: "New Movie".to_string(),
                        body: m.title.clone().unwrap_or_default(),
                        backdrop,
                        relative_time: rel,
                    });
                    count += 1;
                }
            }

            for (idx, s) in shows.into_iter().enumerate() {
                if count >= 16 { break; }
                if let Some(backdrop) = s.backdrop_url("w300").or_else(|| s.poster_url("w300")) {
                    let rel = match idx {
                        0 => "Today".to_string(),
                        1 | 2 => "3 days ago".to_string(),
                        _ => format!("{} days ago", idx + 2),
                    };
                    list.push(AppNotification {
                        id: format!("notif-tv-{}", s.id),
                        movie_id: s.id,
                        is_tv: true,
                        headline: "New Series".to_string(),
                        body: s.title.clone().unwrap_or_default(),
                        backdrop,
                        relative_time: rel,
                    });
                    count += 1;
                }
            }

            set_items.set(list);
            set_is_loading.set(false);
        });
    });

    // Close on click outside
    Effect::new(move |_| {
        if open.get() {
            let window = web_sys::window();
            if let Some(win) = window {
                let document = win.document();
                if let Some(doc) = document {
                    let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
                        if let Some(root) = root_ref.get() {
                            let target = e.target();
                            if let Some(target_node) = target.and_then(|t| t.dyn_into::<web_sys::Node>().ok()) {
                                let root_node: &web_sys::Node = root.as_ref();
                                if !root_node.contains(Some(&target_node)) {
                                    set_open.set(false);
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

    let unread_count = move || {
        let current_reads = read_ids.get();
        items.get().iter().filter(|n| !current_reads.contains(&n.id)).count()
    };

    let toggle_open = move |_| {
        set_open.update(|o| *o = !*o);
    };

    let on_select = move |notif: AppNotification| {
        let notif_id = notif.id.clone();
        set_read_ids.update(|set| {
            set.insert(notif_id);
            save_read_ids(set);
        });
        set_open.set(false);
        ui_store.info_modal_movie_id.set(Some(notif.movie_id));
        ui_store.info_modal_is_tv.set(notif.is_tv);
        ui_store.info_modal_open.set(true);
    };

    let has_unread = move || unread_count() > 0;

    view! {
        <div node_ref=root_ref class="relative">
            // Bell trigger button
            <button
                on:click=toggle_open
                class="relative p-1.5 flex items-center justify-center rounded-full text-white/85 hover:text-white active:scale-95 transition-colors cursor-pointer"
                title="Notifications"
                aria-label="Notifications"
            >
                <i class="ph-bold ph-bell text-[22px]"></i>
                <Show when=has_unread>
                    <span class="absolute -top-0.5 -right-0.5 min-w-[16px] h-[16px] px-1 rounded-full bg-[#E50914] text-white text-[10px] font-bold flex items-center justify-center leading-none">
                        {move || {
                            let c = unread_count();
                            if c > 9 { "9+".to_string() } else { c.to_string() }
                        }}
                    </span>
                </Show>
            </button>

            // Dropdown panel (Task 094)
            <Show when=move || open.get()>
                <div class="absolute right-0 top-[calc(100%+10px)] w-[360px] sm:w-[420px] max-h-[560px] overflow-y-auto bg-[#141414] border border-white/10 rounded-sm shadow-2xl z-50 divide-y divide-white/[0.06] scrollbar-hide">
                    <Show
                        when=move || !is_loading.get()
                        fallback=move || view! {
                            <div class="flex justify-center py-10">
                                <div class="w-6 h-6 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
                            </div>
                        }
                    >
                        <Show
                            when=move || !items.get().is_empty()
                            fallback=move || view! {
                                <p class="text-white/40 text-center text-[13px] py-10 px-6">
                                    "You're all caught up — new arrivals will show up here."
                                </p>
                            }
                        >
                            <div class="flex flex-col">
                                {move || items.get().into_iter().map(|n| {
                                    let notif = n.clone();
                                    let id = n.id.clone();
                                    let id_unread = id.clone();
                                    let is_unread = move || !read_ids.get().contains(&id_unread);
                                    let id_read = id.clone();
                                    let is_read = move || read_ids.get().contains(&id_read);
                                    let on_click_notif = notif.clone();

                                    view! {
                                        <button
                                            on:click=move |_| on_select(on_click_notif.clone())
                                            class="w-full flex items-start gap-3 px-4 py-3 text-left border-b border-white/[0.06] last:border-0 hover:bg-white/[0.06] transition-colors cursor-pointer group"
                                        >
                                            // Red unread indicator dot
                                            <span
                                                class="mt-1.5 w-[7px] h-[7px] rounded-full shrink-0 transition-colors"
                                                class=("bg-[#E50914]", is_unread)
                                                class=("bg-transparent", is_read)
                                            ></span>

                                            // Backdrop thumbnail
                                            <div class="w-[90px] h-[60px] rounded overflow-hidden bg-zinc-900 shrink-0">
                                                <img
                                                    src=n.backdrop.clone()
                                                    alt=n.body.clone()
                                                    class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                                                    loading="lazy"
                                                />
                                            </div>

                                            // Text content
                                            <div class="flex-1 min-w-0">
                                                <p class="text-white text-[14px] font-bold leading-snug">{n.headline}</p>
                                                <p class="text-white/80 text-[14px] leading-snug line-clamp-1">{n.body}</p>
                                                <p class="text-white/40 text-[12px] mt-1">{n.relative_time}</p>
                                            </div>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </Show>
                    </Show>
                </div>
            </Show>
        </div>
    }
}
