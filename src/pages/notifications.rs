use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::services::tmdb::{fetch_trending, fetch_top_rated};
use crate::store::use_ui_store;

#[derive(Clone, Debug, PartialEq)]
pub struct NotificationItem {
    pub id: String,
    pub movie_id: u32,
    pub is_tv: bool,
    pub badge: String,
    pub title: String,
    pub description: String,
    pub backdrop: Option<String>,
    pub time_label: String,
}

#[component]
pub fn NotificationsPage() -> impl IntoView {
    let ui_store = use_ui_store();
    let (read_ids, set_read_ids) = signal(std::collections::HashSet::<String>::new());

    let notifications_resource = LocalResource::new(move || async move {
        let trending_movies = fetch_trending("movie").await.unwrap_or_default();
        let trending_tv = fetch_trending("tv").await.unwrap_or_default();
        let top_movies = fetch_top_rated("movie").await.unwrap_or_default();

        let mut list = Vec::new();

        // 1. Trending Movie Alert
        if let Some(m) = trending_movies.first() {
            list.push(NotificationItem {
                id: format!("notif-trend-m-{}", m.id),
                movie_id: m.id,
                is_tv: false,
                badge: "Trending #1".to_string(),
                title: m.display_title().to_string(),
                description: "Everyone is watching this right now. Watch the trailer or add to My List.".to_string(),
                backdrop: m.backdrop_url("w780").or_else(|| m.poster_url("w500")),
                time_label: "Today".to_string(),
            });
        }

        // 2. Hot TV Series Arrival
        if let Some(tv) = trending_tv.first() {
            list.push(NotificationItem {
                id: format!("notif-trend-tv-{}", tv.id),
                movie_id: tv.id,
                is_tv: true,
                badge: "Top Series".to_string(),
                title: tv.display_title().to_string(),
                description: "New episodes are streaming. Catch up on the story today.".to_string(),
                backdrop: tv.backdrop_url("w780").or_else(|| tv.poster_url("w500")),
                time_label: "1 day ago".to_string(),
            });
        }

        // 3. Critically Acclaimed Pick
        if let Some(top) = top_movies.first() {
            list.push(NotificationItem {
                id: format!("notif-top-m-{}", top.id),
                movie_id: top.id,
                is_tv: false,
                badge: "Critic's Choice".to_string(),
                title: top.display_title().to_string(),
                description: "Highest rated title of the season. Highly recommended based on your tastes.".to_string(),
                backdrop: top.backdrop_url("w780").or_else(|| top.poster_url("w500")),
                time_label: "2 days ago".to_string(),
            });
        }

        // 4. Fill with remaining trending titles
        for (idx, item) in trending_movies.iter().skip(1).take(6).enumerate() {
            list.push(NotificationItem {
                id: format!("notif-m-{}", item.id),
                movie_id: item.id,
                is_tv: item.is_tv(),
                badge: "New Arrival".to_string(),
                title: item.display_title().to_string(),
                description: if item.overview.trim().is_empty() {
                    "Now available in your streaming library.".to_string()
                } else {
                    item.overview.clone()
                },
                backdrop: item.backdrop_url("w780").or_else(|| item.poster_url("w500")),
                time_label: format!("{} days ago", idx + 3),
            });
        }

        list
    });

    let on_card_click = Callback::new(move |item: NotificationItem| {
        set_read_ids.update(|set| {
            set.insert(item.id.clone());
        });
        ui_store.info_modal_movie_id.set(Some(item.movie_id));
        ui_store.info_modal_is_tv.set(item.is_tv);
        ui_store.info_modal_open.set(true);
    });

    view! {
        <Layout>
            <div class="pt-[calc(5.5rem+env(safe-area-inset-top))] md:pt-32 px-4 sm:px-8 md:px-[var(--app-x,56px)] pb-16 min-h-screen bg-void md:bg-page text-ink">
                <div class="max-w-4xl mx-auto">
                    // ── Header ───────────────────────────────────────────────
                    <div class="flex items-center justify-between mb-8 border-b border-ink/10 pb-4">
                        <div>
                            <h1 class="text-2xl md:text-3xl font-bold tracking-tight text-ink flex items-center gap-3">
                                <i class="ph-fill ph-bell-ringing text-brand text-3xl"></i>
                                {crate::i18n::t("notifications-notifications-arrivals")}
                            </h1>
                            <p class="text-ink/50 text-sm mt-1">
                                {crate::i18n::t("notifications-stay-up-to-date-with")}
                            </p>
                        </div>
                    </div>

                    // ── Notification Feed ────────────────────────────────────
                    <Suspense fallback=move || view! {
                        <div class="space-y-4 animate-pulse">
                            {(0..6).map(|_| view! {
                                <div class="h-28 bg-card rounded-modal border border-ink/5"></div>
                            }).collect::<Vec<_>>()}
                        </div>
                    }>
                        {move || notifications_resource.get().map(|items| {
                            if items.is_empty() {
                                return view! {
                                    <div class="flex flex-col items-center justify-center mt-20 text-center select-none">
                                        <div class="w-16 h-16 rounded-full bg-ink/5 flex items-center justify-center mb-4 text-ink/40">
                                            <i class="ph-bold ph-bell text-3xl"></i>
                                        </div>
                                        <div class="text-xl text-ink font-semibold mb-1">"You're all caught up"</div>
                                        <div class="text-ink/40 text-sm">"New notifications and arrivals will appear here."</div>
                                    </div>
                                }.into_any();
                            }

                            let click_cb = on_card_click.clone();
                            view! {
                                <div class="space-y-3.5">
                                    {items.into_iter().map(|item| {
                                        let item_clone = item.clone();
                                        let is_read = read_ids.get().contains(&item.id);
                                        let cb = click_cb.clone();

                                        view! {
                                            <div
                                                on:click=move |_| cb.run(item_clone.clone())
                                                class="group relative flex flex-col sm:flex-row items-start sm:items-center justify-between p-4 bg-raised hover:bg-placeholder border border-ink/10 hover:border-ink/30 rounded-modal transition-all duration-(--motion-base) cursor-pointer shadow-lg gap-4"
                                            >
                                                // Unread Indicator Dot
                                                <div class="flex items-center gap-4 w-full sm:w-auto">
                                                    <div class="w-2.5 h-2.5 rounded-full shrink-0"
                                                        class=("bg-brand", !is_read)
                                                        class=("bg-transparent", is_read)
                                                    />

                                                    // Media Thumbnail
                                                    <div class="w-24 sm:w-32 aspect-video bg-zinc-900 rounded-card overflow-hidden shrink-0 border border-ink/10 group-hover:scale-105 transition-transform duration-(--motion-base)">
                                                        {if let Some(src) = item.backdrop {
                                                            view! { <img src=src alt="" class="w-full h-full object-cover" loading="lazy" /> }.into_any()
                                                        } else {
                                                            view! { <div class="w-full h-full flex items-center justify-center text-ink/20"><i class="ph-bold ph-film-strip text-2xl" /></div> }.into_any()
                                                        }}
                                                    </div>

                                                    // Text Content
                                                    <div class="flex-1 min-w-0 pr-2">
                                                        <div class="flex items-center gap-2 mb-1">
                                                            <span class="px-2 py-0.5 rounded-control text-caption font-bold uppercase tracking-wider bg-ink/10 text-ink/90">
                                                                {item.badge}
                                                            </span>
                                                            <span class="text-ink/40 text-xs">{item.time_label}</span>
                                                        </div>
                                                        <h3 class="text-ink font-bold text-base sm:text-lg truncate group-hover:text-brand transition-colors">
                                                            {item.title}
                                                        </h3>
                                                        <p class="text-ink/60 text-xs sm:text-sm line-clamp-2 mt-0.5">
                                                            {item.description}
                                                        </p>
                                                    </div>
                                                </div>

                                                // Action Icon
                                                <div class="hidden sm:flex items-center text-ink/30 group-hover:text-ink group-hover:translate-x-1 transition-all pr-2 shrink-0">
                                                    <i class="ph-bold ph-caret-right text-xl"></i>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        })}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
