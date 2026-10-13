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

    // Resource for mobile spotlight feed: the engine's row for the tab, TMDB when the engine cannot be reached
    let mobile_feed = LocalResource::new(move || {
        let tab = active_tab.get();
        async move {
            let engine_tab = match tab {
                NewPopularTab::Watching => "watching",
                NewPopularTab::Top10Movies => "top10_movies",
                NewPopularTab::Top10Series => "top10_series",
                NewPopularTab::JustLanded => "just_landed",
                NewPopularTab::ComingSoon => "coming_soon",
            };
            if let Some(rows) = crate::services::ai_engine::fetch_new_popular(Some(engine_tab)).await {
                if let Some(row) = rows.first() {
                    return row.to_media_items();
                }
            }
            match tab {
                NewPopularTab::Watching => fetch_trending("all").await.unwrap_or_default(),
                NewPopularTab::Top10Movies => fetch_top_rated("movie").await.unwrap_or_default(),
                NewPopularTab::Top10Series => fetch_top_rated("tv").await.unwrap_or_default(),
                NewPopularTab::JustLanded => fetch_row_content("movie", None, None, None, Some("/movie/now_playing"), 1).await.unwrap_or_default(),
                NewPopularTab::ComingSoon => fetch_row_content("movie", None, None, None, Some("/movie/upcoming"), 1).await.unwrap_or_default(),
            }
        }
    });

    // Desktop rows from the engine; the fixed TMDB rows below are the fallback
    let desktop_feed = LocalResource::new(move || async move { crate::services::ai_engine::fetch_new_popular(None).await.unwrap_or_default() });

    view! {
        <Layout>
            <div class="relative min-h-screen bg-void md:bg-page">
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
                                    <div class="w-full aspect-video bg-card rounded-modal"></div>
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
                                            let release_date = if is_coming { m.release_date.clone().or_else(|| m.first_air_date.clone()) } else { None };
                                            view! {
                                                <SpotlightCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    overview=overview
                                                    rank=rank
                                                    is_coming_soon=is_coming
                                                    release_date=release_date
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
                <main class="hidden md:block relative z-10 pb-16 rows-stack">
                    <Suspense fallback=move || view! {
                        <div class="space-y-6">
                            <div class="h-40 bg-ink/[0.02] rounded-card animate-pulse mx-[var(--app-x,56px)]" />
                            <div class="h-40 bg-ink/[0.02] rounded-card animate-pulse mx-[var(--app-x,56px)]" />
                        </div>
                    }>
                        {move || desktop_feed.get().map(|rows| {
                            if rows.is_empty() {
                                view! { <FallbackRows /> }.into_any()
                            } else {
                                rows.into_iter().map(|r| {
                                    let row_title = r.title.clone();
                                    let media_items = r.to_media_items();
                                    if r.is_top_ten() {
                                        let kind = if r.items.first().is_some_and(|i| i.is_tv()) { "tv" } else { "movie" };
                                        view! { <TopTenRow title=row_title kind=kind.to_string() items=Some(media_items) /> }.into_any()
                                    } else {
                                        view! {
                                            <crate::components::media::vibe_row::VibeRow
                                                title=row_title
                                                tagline=r.tagline.clone()
                                                mood_pills=r.mood_pills.clone()
                                                items=media_items
                                            />
                                        }.into_any()
                                    }
                                }).collect::<Vec<_>>().into_any()
                            }
                        })}
                    </Suspense>
                </main>
            </div>
        </Layout>
    }
}

/// The page as it was before the engine served it: fixed TMDB rows, shown only when the engine cannot be reached.
#[component]
fn FallbackRows() -> impl IntoView {
    view! {
        <TopTenRow title=crate::i18n::t("new-popular-top-10-films-in-the") kind="movie" />
        <TopTenRow title=crate::i18n::t("new-popular-top-10-series-in-the") kind="tv" />
        <Row title=crate::i18n::t("new-popular-newly-added-to-the-collection") endpoint="/movie/now_playing" kind="movie" />
        <Row title=crate::i18n::t("new-popular-worth-the-wait") sort_by="vote_average.desc" extra_params="&vote_count.gte=1000" kind="movie" />
        <Row title=crate::i18n::t("new-popular-rising-stars-under-the-radar") sort_by="popularity.desc" extra_params="&vote_count.lte=2500&vote_count.gte=150&vote_average.gte=7.0" kind="movie" />
        <Row title=crate::i18n::t("new-popular-the-best-of-2026-so") sort_by="vote_average.desc" extra_params="&primary_release_date.gte=2025-01-01&vote_count.gte=80" kind="movie" />
        <Row title=crate::i18n::t("new-popular-new-series-everyone-is-watching") endpoint="/trending/tv/week" kind="tv" />
        <Row title=crate::i18n::t("new-popular-the-films-everyone-is-discussing") endpoint="/trending/movie/week" kind="movie" />
        <Row title=crate::i18n::t("new-popular-coming-to-the-collection-soon") endpoint="/movie/upcoming" kind="movie" />
        <Row title=crate::i18n::t("new-popular-new-and-acclaimed") sort_by="vote_average.desc" extra_params="&vote_count.gte=400&primary_release_date.gte=2024-01-01" kind="movie" />
    }
}
