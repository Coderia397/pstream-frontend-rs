use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::services::tmdb::fetch_trending;
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;
use crate::components::layout::category_sub_nav::{CategorySubNav, SubNavGenre};
use crate::data::{MOVIE_GENRES, TV_GENRES};

/// The page behind `/browse/genre/<id>`: the Series page (83), the Films page (34399), or a genre page of one of them (`?bc=` says which; without it the genre decides). An id that
/// is not a genre keeps the older grid of a row.
#[component]
pub fn GenreRoute() -> impl IntoView {
    use crate::data::page_genres::{genre_url, page_kind_of_genre_route, FILMS_PAGE_ID, SERIES_PAGE_ID};
    let params = leptos_router::hooks::use_params_map();
    let query = leptos_router::hooks::use_query_map();
    let navigate = leptos_router::hooks::use_navigate();
    // A page the engine no longer offers (an event out of season, a genre without enough titles) is not shown: the address goes back to the page it belongs to, so no
    // reference to a page that is gone stays reachable.
    let listing = LocalResource::new(move || {
        let id = params.read().get("id").and_then(|i| i.parse::<u32>().ok());
        let bc = query.read().get("bc");
        async move {
            let (Some(id), Some(_)) = (id, bc.as_ref().or(None)) else { return Some(true) };
            let kind = if bc.as_deref() == Some("83") { "tv" } else { "movie" };
            let list = crate::services::ai_engine::fetch_genre_list(kind).await?;
            Some(list.iter().find(|g| g.id == id).is_none_or(|g| g.supported))
        }
    });
    let chips = LocalResource::new(|| async { crate::services::ai_engine::fetch_season_chips().await });
    Effect::new(move |_| {
        let id = params.read().get("id").and_then(|i| i.parse::<u32>().ok());
        let bc = query.read().get("bc");
        match (id, bc.as_deref()) {
            (Some(i), Some(_)) if listing.get() == Some(Some(false)) => {
                navigate(&genre_url(if bc.as_deref() == Some("83") { "tv" } else { "movie" }, None), Default::default());
                let _ = i;
            }
            (Some(i), None) if crate::data::page_genres::genre_name(i).is_some() && i == 10016 => {
                if chips.get().is_some_and(|c| !c.iter().any(|x| x.genre_id == i)) { navigate("/browse", Default::default()); }
            }
            _ => {}
        }
    });
    move || {
        let id = params.read().get("id").unwrap_or_default();
        let bc = query.read().get("bc").map(|b| format!("?bc={b}")).unwrap_or_default();
        let kind = page_kind_of_genre_route(&format!("/browse/genre/{id}"), &bc);
        let has_bc = !bc.is_empty();
        match (id.parse::<u32>(), kind) {
            (Ok(SERIES_PAGE_ID), _) => view! { <CategoryPage kind="tv" /> }.into_any(),
            (Ok(FILMS_PAGE_ID), _) => view! { <CategoryPage kind="movie" /> }.into_any(),
            // an event page without a page to belong to (Halloween) mixes films and series, like Home
            (Ok(10016), _) if !has_bc => view! { <crate::pages::browse::BrowseHome event=(10016, "Halloween") /> }.into_any(),
            (Ok(n), Some(k)) if MOVIE_GENRES.iter().chain(TV_GENRES.iter()).any(|g| g.id == n) => {
                view! { <CategoryPage kind=if k == "tv" { "tv" } else { "movie" } genre_id=n /> }.into_any()
            }
            _ => view! { <crate::pages::browse_grid::BrowseGridPage /> }.into_any(),
        }
    }
}

#[component]
pub fn CategoryPage(
    kind: &'static str, // "movie" or "tv"
    /// The genre of the address (`/browse/genre/<id>`), when the page was opened that way
    #[prop(optional)] genre_id: Option<u32>,
) -> impl IntoView {
    let genres: Vec<SubNavGenre> = if kind == "tv" {
        TV_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    } else {
        MOVIE_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    };

    let query_map = leptos_router::hooks::use_query_map();
    let genres_clone = genres.clone();
    let initial_genre = query_map.with_untracked(|q| {
        if let Some(gid) = genre_id {
            genres_clone.iter().find(|g| g.id == gid).cloned()
        } else if let Some(gid_str) = q.get("genre") {
            if let Ok(gid) = gid_str.parse::<u32>() {
                genres_clone.iter().find(|g| g.id == gid).cloned()
            } else {
                let lower = gid_str.to_lowercase();
                genres_clone.iter().find(|g| g.name.to_lowercase() == lower).cloned()
            }
        } else {
            None
        }
    });

    let (selected_genre, set_selected_genre) = signal(initial_genre);

    let genres_for_effect = genres.clone();
    Effect::new(move |_| {
        let q = query_map.get();
        if let Some(gid_str) = q.get("genre") {
            let matched = if let Ok(gid) = gid_str.parse::<u32>() {
                genres_for_effect.iter().find(|g| g.id == gid).cloned()
            } else {
                let lower = gid_str.to_lowercase();
                genres_for_effect.iter().find(|g| g.name.to_lowercase() == lower).cloned()
            };
            if matched != selected_genre.get_untracked() {
                set_selected_genre.set(matched);
            }
        }
    });

    let on_genre_select_cb = Callback::new(move |opt_g: Option<SubNavGenre>| {
        set_selected_genre.set(opt_g.clone());
        if let Some(win) = web_sys::window() {
            if let Ok(history) = win.history() {
                let new_url = crate::data::page_genres::genre_url(kind, opt_g.map(|g| g.id));
                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            }
        }
    });

    let title = if kind == "tv" { crate::i18n::t("category-series") } else { crate::i18n::t("category-films") };

    let ui_store = crate::store::use_ui_store();

    // Hero: pull featured item for selected genre or surface-specific AI hero feed
    let hero = LocalResource::new(move || {
        let sel = selected_genre.get();
        let k = kind;
        async move {
            let surface = if k == "tv" { "tv" } else { "movies" };
            let gid_opt = sel.map(|g| g.id.to_string());
            if let Some(items) = crate::services::ai_engine::fetch_hero_feed(surface, gid_opt.as_deref()).await {
                if !items.is_empty() {
                    return items;
                }
            }
            // a genre page never borrows the trending titles of the whole site: no title fits better than a wrong one
            if gid_opt.is_some() {
                return Vec::new();
            }
            fetch_trending(k).await.unwrap_or_default()
        }
    });

    let ambient_bg_style = move || {
        let (r, g, b) = ui_store.ambient_color.get();
        format!(
            "background: \
             radial-gradient(ellipse 110% 65% at 50% 20%, rgba({r}, {g}, {b}, 0.22) 0%, rgba({r}, {g}, {b}, 0.11) 36%, rgba({r}, {g}, {b}, 0.02) 52%, transparent 70%), \
             linear-gradient(to bottom, \
                 rgba(6, 8, 10, 0.45) 0%, \
                 rgba(6, 8, 10, 0.15) 12%, \
                 transparent 22%, \
                 rgba(20, 20, 20, 0.50) 55%, \
                 rgba(20, 20, 20, 1.0) 82%);",
            r = r, g = g, b = b
        )
    };

    // Dynamic Category Feed: AI-curated sub-vibe rows & category Top 10
    let feed = LocalResource::new(move || {
        let k = kind.to_string();
        let g_opt = selected_genre.get().map(|g| g.name);
        async move {
            crate::services::ai_engine::fetch_dynamic_feed(&k, g_opt.as_deref(), Some(12)).await.unwrap_or_default()
        }
    });

    // The genre pages come from the engine (ids, order, and whether this profile is offered the page); the page's own list is the fallback when the engine cannot be reached
    let all_genres = genres.clone();
    let listing = LocalResource::new(move || async move { crate::services::ai_engine::fetch_genre_list(if kind == "tv" { "tv" } else { "movie" }).await });
    let offered = move || match listing.get() {
        None => Vec::new(), // still asking
        Some(Some(list)) => list.into_iter().filter(|g| g.supported).map(|g| SubNavGenre { id: g.id, name: g.name }).collect::<Vec<SubNavGenre>>(),
        Some(None) => all_genres.clone(),
    };

    view! {
        <Layout>
            <div class="with-subnav w-full pb-20 bg-void md:bg-page min-h-screen relative overflow-x-hidden">
                // Ambient Atmospheric Glow behind navbar, category sub-nav, and hero billboard (fades after 50% of hero height)
                <div
                    class="absolute inset-x-0 top-0 h-[580px] md:h-[660px] lg:h-[720px] pointer-events-none -z-0 transition-all duration-(--motion-slowest) ease-out"
                    style=ambient_bg_style
                />
                // Category Sub-Navigation bar (teleports to fixed header below navbar on desktop)
                {
                    let nav_title = title.to_string();
                    move || view! {
                        <CategorySubNav
                            title=nav_title.clone()
                            genres=offered()
                            selected_genre=selected_genre
                            on_genre_select=on_genre_select_cb
                        />
                    }
                }

                <Suspense fallback=move || view! {
                    <crate::components::media::hero_skeleton::HeroSkeleton />
                }>
                    {move || hero.get().map(|result| {
                        let movies = result;
                        if movies.is_empty() {
                            view! { <div class="h-[500px] bg-void"></div> }.into_any()
                        } else {
                            view! {
                                <HeroSection movies=movies />
                            }.into_any()
                        }
                    })}
                </Suspense>

                <div class="relative z-30 rows-stack">
                    <Suspense fallback=move || view! {
                        <div class="space-y-6">
                            <div class="h-40 bg-ink/[0.02] rounded-card animate-pulse mx-[var(--app-x,56px)]" />
                            <div class="h-40 bg-ink/[0.02] rounded-card animate-pulse mx-[var(--app-x,56px)]" />
                        </div>
                    }>
                        {move || feed.get().map(|rows| {
                            if rows.is_empty() {
                                let default_top10_title = if kind == "movie" { crate::i18n::t("category-top10-movies") } else { crate::i18n::t("category-top10-series") };
                                view! {
                                    <TopTenRow title=default_top10_title.to_string() kind=kind.to_string() />
                                    <Row title=crate::i18n::t("browse-action-adventure") genre_id="1365" kind=kind.to_string() />
                                    <Row title=crate::i18n::t("browse-comedies") genre_id="6548" kind=kind.to_string() />
                                    <Row title=crate::i18n::t("browse-scifi-fantasy") genre_id="1492" kind=kind.to_string() />
                                }.into_any()
                            } else {
                                rows.into_iter().filter(|r| !crate::components::media::feed_row::is_continue_row(r)).map(|r| {
                                    view! { <crate::components::media::feed_row::FeedRowView row=r kind=kind.to_string() /> }
                                }).collect::<Vec<_>>().into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
