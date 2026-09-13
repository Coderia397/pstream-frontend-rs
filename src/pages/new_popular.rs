use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::spotlight_card::SpotlightCard;
use crate::components::layout::new_popular_sub_nav_mobile::{NewPopularSubNavMobile, NewPopularTab};
use crate::services::tmdb::{fetch_trending, fetch_top_rated, fetch_row_content};

#[component]
pub fn NewPopularPage() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(NewPopularTab::Watching);

    // Resource for mobile spotlight feed
    let mobile_feed = LocalResource::new(move || {
        let tab = active_tab.get();
        async move {
            match tab {
                NewPopularTab::Watching => fetch_trending("all").await.unwrap_or_default(),
                NewPopularTab::Top10Movies => fetch_top_rated("movie").await.unwrap_or_default(),
                NewPopularTab::Top10Series => fetch_top_rated("tv").await.unwrap_or_default(),
                NewPopularTab::JustLanded => fetch_row_content("movie", None, None, None, Some("/movie/now_playing"), 1).await.unwrap_or_default(),
                NewPopularTab::ComingSoon => fetch_row_content("movie", None, None, None, Some("/movie/upcoming"), 1).await.unwrap_or_default(),
            }
        }
    });

    view! {
        <Layout>
            <div class="relative min-h-screen bg-black md:bg-[#141414]">
                // Top spacing under fixed navbar
                <div class="h-16 sm:h-20 md:h-28" />

                // ── MOBILE TABBED FEED (< md) ───────────────────────────
                <div class="block md:hidden">
                    // Mobile sticky subnav
                    <NewPopularSubNavMobile
                        active_tab=active_tab
                        on_tab_change=Callback::new(move |t| set_active_tab.set(t))
                    />

                    // Mobile vertical spotlight stream
                    <div class="px-3 pt-2 pb-16 space-y-6">
                        <Suspense fallback=move || view! {
                            <div class="space-y-4 animate-pulse">
                                {(0..3).map(|_| view! {
                                    <div class="w-full aspect-video bg-[#1e1e1e] rounded-xl"></div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }>
                            {move || mobile_feed.get().map(|items| {
                                let tab = active_tab.get();
                                let is_top10 = tab == NewPopularTab::Top10Movies || tab == NewPopularTab::Top10Series;
                                let is_coming = tab == NewPopularTab::ComingSoon;
                                view! {
                                    <div class="space-y-6">
                                        {items.into_iter().enumerate().take(10).map(|(idx, m)| {
                                            let movie_id = m.id;
                                            let is_tv = m.is_tv();
                                            let title = m.display_title().to_string();
                                            let backdrop = m.backdrop_url("w780").unwrap_or_default();
                                            let overview = m.overview.clone();
                                            let rank = if is_top10 { Some(idx + 1) } else { None };
                                            view! {
                                                <SpotlightCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    overview=overview
                                                    rank=rank
                                                    is_coming_soon=is_coming
                                                />
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            })}
                        </Suspense>
                    </div>
                </div>

                // ── DESKTOP DYNAMIC ROWS (>= md) ──────────────────────────
                <main class="hidden md:block relative z-10 pb-16 space-y-4 md:space-y-6">
                    <TopTenRow title="Top 10 Films in the UK Today" kind="movie" />
                    <TopTenRow title="Top 10 Series in the UK Today" kind="tv" />
                    <Row title="Newly Added to the Collection" endpoint="/movie/now_playing" kind="movie" />
                    <Row title="Worth the Wait" sort_by="vote_average.desc" extra_params="&vote_count.gte=1000" kind="movie" />
                    <Row title="Rising Stars: Under the Radar" sort_by="popularity.desc" extra_params="&vote_count.lte=2500&vote_count.gte=150&vote_average.gte=7.0" kind="movie" />
                    <Row title="The Best of 2026 So Far" sort_by="vote_average.desc" extra_params="&primary_release_date.gte=2025-01-01&vote_count.gte=80" kind="movie" />
                    <Row title="New Series Everyone Is Watching" endpoint="/trending/tv/week" kind="tv" />
                    <Row title="The Shows the Internet Can't Stop Talking About" genre_id="18" kind="tv" sort_by="popularity.desc" />
                    <Row title="The Films Everyone Is Discussing" endpoint="/trending/movie/week" kind="movie" />
                    <Row title="Coming to the Collection Soon" endpoint="/movie/upcoming" kind="movie" />
                    <Row title="New and Acclaimed" sort_by="vote_average.desc" extra_params="&vote_count.gte=400&primary_release_date.gte=2024-01-01" kind="movie" />
                    <Row title="Series Picking Up Steam" genre_id="10765" kind="tv" sort_by="popularity.desc" />
                    <Row title="International Discoveries" genre_id="international" kind="tv" />
                </main>
            </div>
        </Layout>
    }
}
