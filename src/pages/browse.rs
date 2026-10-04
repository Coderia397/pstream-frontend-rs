use leptos::prelude::*;
use crate::services::tmdb::{fetch_trending};
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;
use crate::components::media::continue_watching_row::ContinueWatchingRow;
use crate::components::layout::category_sub_nav::{CategorySubNav, SubNavGenre};
use crate::data::HOME_MOBILE_GENRES;

#[component]
pub fn BrowseHome() -> impl IntoView {
    use crate::components::layout::Layout;
    use crate::store::use_ui_store;

    let ui_store = use_ui_store();

    // Mobile genre picker: Home has genres on mobile only. The genre travels as its name (several pickers share an
    // id, for example the three documentary entries) and the engine resolves it for movies and series together.
    let genres: Vec<SubNavGenre> = HOME_MOBILE_GENRES
        .iter()
        .map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() })
        .collect();
    let query_map = leptos_router::hooks::use_query_map();
    let genres_for_init = genres.clone();
    let initial_genre = query_map.with_untracked(|q| {
        q.get("genre").and_then(|n| {
            let lower = n.to_lowercase();
            genres_for_init.iter().find(|g| g.name.to_lowercase() == lower).cloned()
        })
    });
    let (selected_genre, set_selected_genre) = signal(initial_genre);
    let on_genre_select_cb = Callback::new(move |opt_g: Option<SubNavGenre>| {
        set_selected_genre.set(opt_g.clone());
        if let Some(win) = web_sys::window() {
            if let Ok(history) = win.history() {
                let pathname = win.location().pathname().unwrap_or_default();
                let new_url = match opt_g {
                    Some(g) => {
                        let encoded: String = js_sys::encode_uri_component(&g.name).into();
                        format!("{}?genre={}", pathname, encoded)
                    }
                    None => pathname,
                };
                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
            }
        }
    });

    // Hero: pull AI-curated home feed with trending fallback
    let hero = LocalResource::new(move || {
        let g = selected_genre.get().map(|g| g.name);
        async move {
            if let Some(items) = crate::services::ai_engine::fetch_hero_feed("home", g.as_deref()).await {
                if !items.is_empty() {
                    return items;
                }
            }
            fetch_trending("all").await.unwrap_or_default()
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

    // Dynamic Feed: AI-curated polymorphic rows from sovereign intelligence engine
    let feed = LocalResource::new(move || {
        let g = selected_genre.get().map(|g| g.name);
        async move {
            crate::services::ai_engine::fetch_dynamic_feed("home", g.as_deref(), Some(24)).await.unwrap_or_default()
        }
    });

    view! {
        <Layout>
            <div class="w-full pb-20 bg-black md:bg-[#141414] min-h-screen relative overflow-x-hidden">
                // Ambient Atmospheric Glow behind navbar and hero billboard (fades after 50% of hero height)
                <div
                    class="absolute inset-x-0 top-0 h-[580px] md:h-[660px] lg:h-[720px] pointer-events-none -z-0 transition-all duration-700 ease-out"
                    style=ambient_bg_style
                />

                // Genre picker, mobile only
                <div class="sm:hidden">
                    <CategorySubNav
                        title="Home".to_string()
                        genres=genres
                        selected_genre=selected_genre
                        on_genre_select=on_genre_select_cb
                    />
                </div>

                // Hero Carousel driven by real TMDB / AI feed
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
                    <ContinueWatchingRow />

                    <Suspense fallback=move || view! {
                        <div class="space-y-6">
                            <div class="h-40 bg-white/[0.02] rounded-lg animate-pulse mx-[var(--app-x,56px)]" />
                            <div class="h-40 bg-white/[0.02] rounded-lg animate-pulse mx-[var(--app-x,56px)]" />
                        </div>
                    }>
                        {move || feed.get().map(|rows| {
                            if rows.is_empty() {
                                view! {
                                    <TopTenRow title="Top 10 in the UK Today" kind="all" />
                                    <Row title="Action & Adventure" genre_id="1365" kind="movie" />
                                    <Row title="Comedies" genre_id="6548" kind="movie" />
                                    <Row title="Sci-Fi & Fantasy" genre_id="1492" kind="movie" />
                                }.into_any()
                            } else {
                                rows.into_iter().map(|r| {
                                    let rtype = r.row_type.clone();
                                    let title = r.title.clone();
                                    let tagline = r.tagline.clone();
                                    let mood_pills = r.mood_pills.clone();
                                    let media_items = r.to_media_items();

                                    if rtype == "top_ten" {
                                        view! {
                                            <TopTenRow title=title kind="all" items=Some(media_items) />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <crate::components::media::vibe_row::VibeRow
                                                title=title
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
