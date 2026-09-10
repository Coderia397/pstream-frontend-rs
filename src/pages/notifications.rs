use leptos::prelude::*;

fn format_date(iso: &str) -> String {
    // Simple date parser: expects "YYYY-MM-DD..." format
    let parts: Vec<&str> = iso.splitn(3, '-').collect();
    if parts.len() < 3 {
        return iso.to_string();
    }
    let months = [
        "", "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month_n: usize = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2][..2.min(parts[2].len())].parse().unwrap_or(0);
    let month_str = months.get(month_n).copied().unwrap_or("");
    format!("{} {}", day, month_str)
}

#[derive(Clone)]
pub struct Notification {
    pub id: String,
    pub headline: String,
    pub body: String,
    pub date: String,
    pub thumb: Option<String>,
}

#[component]
pub fn NotificationsPage() -> impl IntoView {

    // In the React source, notifications come from useNotifications() which calls
    // fetch_trending. In the Rust port we show an empty state (no Supabase).
    let items: Vec<Notification> = vec![];
    let is_loading = false;

    let (read_ids, set_read_ids) = signal(std::collections::HashSet::<String>::new());

    view! {
        <div class="min-h-screen bg-black pb-28">
            // Sticky header
            <div class="sticky top-0 z-10 bg-black/95 backdrop-blur-sm flex items-center gap-4 px-4 md:px-8 pt-[calc(0.75rem+env(safe-area-inset-top,0px))] pb-3">
                <button
                    on:click=move |_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.history().ok())
                            .map(|h| h.back());
                    }
                    class="text-white active:scale-90 transition-transform -ml-1"
                    aria-label="Back"
                >
                    <i class="ph-bold ph-arrow-left text-2xl" />
                </button>
                <h1 class="text-white text-[22px] font-bold">"Notifications"</h1>
            </div>

            {if is_loading {
                view! {
                    <div class="flex justify-center pt-20">
                        <div class="w-8 h-8 border-2 border-white/20 border-t-white rounded-full animate-spin" />
                    </div>
                }.into_any()
            } else if items.is_empty() {
                view! {
                    <p class="text-white/40 text-center text-[14px] mt-16 px-8">
                        "You're all caught up — new arrivals will show up here."
                    </p>
                }.into_any()
            } else {
                let items_view = items.into_iter().map(|n| {
                    let nid = n.id.clone();
                    let nid2 = n.id.clone();
                    view! {
                        <button
                            on:click={
                                let nid = nid.clone();
                                move |_| {
                                    set_read_ids.update(|s| { s.insert(nid.clone()); });
                                }
                            }
                            class="w-full flex items-center gap-3 px-4 md:px-8 py-3 text-left active:bg-white/5 transition-colors"
                        >
                            <span class=move || {
                                let unread = !read_ids.get().contains(&nid2);
                                if unread { "w-2 h-2 rounded-full shrink-0 bg-[#E50914]" }
                                else { "w-2 h-2 rounded-full shrink-0 bg-transparent" }
                            } />
                            <div class="w-[132px] aspect-video rounded-md overflow-hidden bg-zinc-900 shrink-0">
                                {n.thumb.as_ref().map(|src| view! {
                                    <img src=src.clone() alt="" class="w-full h-full object-cover" loading="lazy" />
                                })}
                            </div>
                            <div class="flex-1 min-w-0">
                                <p class="text-white text-[16px] font-bold leading-snug">{n.headline.clone()}</p>
                                <p class="text-white/50 text-[15px] truncate mt-0.5">{n.body.clone()}</p>
                                <p class="text-white/40 text-[13px] mt-1">{format_date(&n.date)}</p>
                            </div>
                        </button>
                    }
                }).collect::<Vec<_>>();
                view! { <div class="pt-2">{items_view}</div> }.into_any()
            }}
        </div>
    }
}
