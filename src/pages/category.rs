use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::services::tmdb::fetch_trending;
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;
use crate::components::layout::category_sub_nav::{CategorySubNav, SubNavGenre};
use crate::data::{MOVIE_GENRES, TV_GENRES};

#[component]
pub fn CategoryPage(
    kind: &'static str, // "movie" or "tv"
) -> impl IntoView {
    let genres: Vec<SubNavGenre> = if kind == "tv" {
        TV_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    } else {
        MOVIE_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    };

    let query_map = leptos_router::hooks::use_query_map();
    let genres_clone = genres.clone();
    let initial_genre = query_map.with_untracked(|q| {
        if let Some(gid_str) = q.get("genre") {
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
                let pathname = win.location().pathname().unwrap_or_default();
                let new_url = match opt_g {
                    Some(g) => format!("{}?genre={}", pathname, g.id),
                    None => pathname,
                };
                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            }
        }
    });

    let title = if kind == "tv" { "Series" } else { "Films" };

    let ui_store = crate::store::use_ui_store();

    // Hero: pull featured item for selected genre or surface-specific AI hero feed
    let hero = LocalResource::new(move || {
        let sel = selected_genre.get();
        let k = kind;
        async move {
            if let Some(genre) = sel {
                let gid = genre.id.to_string();
                let items = crate::services::tmdb::fetch_row_content(
                    k,
                    Some(&gid),
                    Some("popularity.desc"),
                    None,
                    None,
                    1,
                ).await.unwrap_or_default();
                if !items.is_empty() {
                    return items;
                }
            } else {
                let surface = if k == "tv" { "series" } else { "films" };
                if let Some(items) = crate::services::ai_engine::fetch_hero_feed(surface).await {
                    if !items.is_empty() {
                        return items;
                    }
                }
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

    view! {
        <Layout>
            <div class="w-full pb-20 bg-black md:bg-[#141414] min-h-screen relative overflow-x-hidden">
                // Ambient Atmospheric Glow behind navbar, category sub-nav, and hero billboard (fades after 50% of hero height)
                <div
                    class="absolute inset-x-0 top-0 h-[580px] md:h-[660px] lg:h-[720px] pointer-events-none -z-0 transition-all duration-700 ease-out"
                    style=ambient_bg_style
                />
                // Category Sub-Navigation bar (teleports to fixed header below navbar on desktop)
                <CategorySubNav
                    title=title.to_string()
                    genres=genres
                    selected_genre=selected_genre
                    on_genre_select=on_genre_select_cb
                />

                <Suspense fallback=move || view! {
                    <crate::components::media::hero_skeleton::HeroSkeleton />
                }>
                    {move || hero.get().map(|result| {
                        let movies = result;
                        if movies.is_empty() {
                            view! { <div class="h-[500px] bg-black"></div> }.into_any()
                        } else {
                            view! {
                                <HeroSection movies=movies />
                            }.into_any()
                        }
                    })}
                </Suspense>

                <div class="relative z-30 space-y-6 md:space-y-10 mt-6 md:mt-8">
                    <Suspense fallback=move || view! {
                        <div class="space-y-6">
                            <div class="h-40 bg-white/[0.02] rounded-lg animate-pulse mx-[var(--app-x,56px)]" />
                            <div class="h-40 bg-white/[0.02] rounded-lg animate-pulse mx-[var(--app-x,56px)]" />
                        </div>
                    }>
                        {move || feed.get().map(|rows| {
                            if rows.is_empty() {
                                let default_top10_title = if kind == "movie" { "Top 10 Movies Today" } else { "Top 10 TV Shows Today" };
                                view! {
                                    <TopTenRow title=default_top10_title.to_string() kind=kind.to_string() />
                                    <Row title="Action & Adventure" genre_id="1365" kind=kind.to_string() />
                                    <Row title="Comedies" genre_id="6548" kind=kind.to_string() />
                                    <Row title="Sci-Fi & Fantasy" genre_id="1492" kind=kind.to_string() />
                                }.into_any()
                            } else {
                                rows.into_iter().map(|r| {
                                    let rtype = r.row_type.clone();
                                    let row_title = r.title.clone();
                                    let tagline = r.tagline.clone();
                                    let mood_pills = r.mood_pills.clone();
                                    let media_items = r.to_media_items();

                                    if rtype == "top_ten" {
                                        view! {
                                            <TopTenRow title=row_title kind=kind.to_string() items=Some(media_items) />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <crate::components::media::vibe_row::VibeRow
                                                title=row_title
                                                tagline=tagline
                                                mood_pills=mood_pills
                                                items=media_items
                                            />
                                        }.into_any()
                                    }
                                }).collect::<Vec<_>>().into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
