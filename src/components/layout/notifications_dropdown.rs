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
        if let Ok(Some(raw)) = storage.get_item(crate::keys::NOTIFICATIONS_READ) {
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
            let _ = storage.set_item(crate::keys::NOTIFICATIONS_READ, &json);
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
                        0 => crate::i18n::t("notif-new"),
                        1 => crate::i18n::t_args("notif-ago-days", &[("n", fluent_bundle::FluentValue::from(1))]),
                        2 => crate::i18n::t_args("notif-ago-days", &[("n", fluent_bundle::FluentValue::from(3))]),
                        3 => crate::i18n::t_args("notif-ago-weeks", &[("n", fluent_bundle::FluentValue::from(1))]),
                        4 => crate::i18n::t_args("notif-ago-weeks", &[("n", fluent_bundle::FluentValue::from(2))]),
                        5 => crate::i18n::t_args("notif-ago-weeks", &[("n", fluent_bundle::FluentValue::from(3))]),
                        _ => crate::i18n::t_args("notif-ago-months", &[("n", fluent_bundle::FluentValue::from(1))]),
                    };
                    list.push(AppNotification {
                        id: format!("notif-movie-{}", m.id),
                        movie_id: m.id,
                        is_tv: false,
                        headline: match idx {
                            0 => crate::i18n::t("notif-h-top-pick"),
                            1 => crate::i18n::t("notif-h-trending"),
                            2 => crate::i18n::t("notif-h-dont-miss"),
                            3 => crate::i18n::t("notif-h-new-arrival"),
                            4 => crate::i18n::t("notif-h-must-watch"),
                            5 => crate::i18n::t("notif-h-top10-films"),
                            6 => crate::i18n::t("notif-h-critics"),
                            _ => crate::i18n::t_args("notif-h-new-on", &[("brand", fluent_bundle::FluentValue::from(crate::brand::NAME))]),
                        },
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
                        0 => crate::i18n::t("notif-new"),
                        1 => crate::i18n::t_args("notif-ago-days", &[("n", fluent_bundle::FluentValue::from(2))]),
                        2 => crate::i18n::t_args("notif-ago-weeks", &[("n", fluent_bundle::FluentValue::from(1))]),
                        3 => "2 weeks ago".to_string(),
                        4 => "3 weeks ago".to_string(),
                        _ => crate::i18n::t_args("notif-ago-months", &[("n", fluent_bundle::FluentValue::from(1))]),
                    };
                    list.push(AppNotification {
                        id: format!("notif-tv-{}", s.id),
                        movie_id: s.id,
                        is_tv: true,
                        headline: match idx {
                            0 => crate::i18n::t("notif-h-new-season"),
                            1 => crate::i18n::t("notif-h-top-series"),
                            2 => crate::i18n::t("notif-h-everyone"),
                            3 => crate::i18n::t("notif-h-comedy"),
                            4 => crate::i18n::t("notif-h-binge"),
                            5 => crate::i18n::t("notif-h-top10-series"),
                            6 => crate::i18n::t("notif-h-new-arrival"),
                            _ => crate::i18n::t_args("notif-h-new-on", &[("brand", fluent_bundle::FluentValue::from(crate::brand::NAME))]),
                        },
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
        ui_store.hero_paused_by_modal.set(false);
        ui_store.modal_initial_time.set(0.0);
        ui_store.modal_current_time.set(0.0);
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
                class="relative w-10 h-10 flex items-center justify-center rounded-full text-ink hover:bg-mute/40 active:scale-95 transition-colors cursor-pointer"
                title=crate::i18n::t("notifications-dropdown-notifications")
                aria-label=crate::i18n::t("notifications-dropdown-notifications")
            >
                <i class="ph ph-bell text-heading"></i>
                <Show when=has_unread>
                    <span class="absolute top-[2px] right-[2px] min-w-[16px] h-[16px] px-1 rounded-full bg-brand text-ink text-micro font-medium flex items-center justify-center leading-none">
                        {move || {
                            let c = unread_count();
                            if c > 99 { "99+".to_string() } else { c.to_string() }
                        }}
                    </span>
                </Show>
            </button>

            // Dropdown panel
            <Show when=move || open.get()>
                <div class="absolute right-0 top-[calc(100%+8px)] w-[360px] sm:w-[420px] max-h-[560px] overflow-y-auto bg-menu rounded-modal shadow-2xl z-50 scrollbar-hide">
                    // Header
                    <div class="px-4 py-3">
                        <p class="text-ink font-bold text-body">{crate::i18n::t("notif-title")}</p>
                    </div>
                    <Show
                        when=move || !is_loading.get()
                        fallback=move || view! {
                            <div class="flex justify-center py-10">
                                <div class="w-6 h-6 border-2 border-ink/20 border-t-ink rounded-full animate-spin"></div>
                            </div>
                        }
                    >
                        <Show
                            when=move || !items.get().is_empty()
                            fallback=move || view! {
                                <p class="text-ink/40 text-center text-label py-10 px-6">
                                    {crate::i18n::t("notifications-dropdown-youre-all-caught-up-new")}
                                </p>
                            }
                        >
                            <div class="flex flex-col">
                                {move || items.get().into_iter().map(|n| {
                                    let notif = n.clone();
                                    let on_click_notif = notif.clone();

                                    view! {
                                        <button
                                            on:click=move |_| on_select(on_click_notif.clone())
                                            class="w-full flex items-start gap-3 px-3 py-3 text-left hover:bg-ink/[0.08] rounded-card mx-1 transition-colors cursor-pointer group"
                                        >
                                            // Backdrop thumbnail (16:9)
                                            <div class="w-[112px] h-[63px] rounded-field overflow-hidden bg-zinc-800 shrink-0">
                                                <img
                                                    src=n.backdrop.clone()
                                                    alt=n.body.clone()
                                                    class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-(--motion-slow)"
                                                    loading="lazy"
                                                />
                                            </div>

                                            // Text: headline big+bold, body smaller/muted, time tiny gray
                                            <div class="flex-1 min-w-0 pt-0.5">
                                                <p class="text-ink text-body font-bold leading-snug mb-0.5">{n.headline}</p>
                                                <p class="text-ink/60 text-label leading-snug line-clamp-2">{n.body}</p>
                                                <p class="text-ink/40 text-fine mt-1">{n.relative_time}</p>
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
