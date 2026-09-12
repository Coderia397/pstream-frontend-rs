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
    let (selected_genre, set_selected_genre) = signal(None::<SubNavGenre>);

    let genres: Vec<SubNavGenre> = if kind == "tv" {
        TV_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    } else {
        MOVIE_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    };

    let title = if kind == "tv" { "Series" } else { "Films" };

    let ui_store = crate::store::use_ui_store();

    // Hero: pull trending item for background
    let hero = LocalResource::new(move || async move {
        fetch_trending(kind).await.unwrap_or_default()
    });

    let ambient_bg_style = move || {
        let (r, g, b) = ui_store.ambient_color.get();
        format!(
            "background: \
             radial-gradient(ellipse 110% 55% at 50% 0%, rgba({r}, {g}, {b}, 0.32) 0%, rgba({r}, {g}, {b}, 0.16) 35%, rgba({r}, {g}, {b}, 0.05) 50%, transparent 68%), \
             linear-gradient(to bottom, \
                 rgba({r}, {g}, {b}, 0.25) 0%, \
                 rgba({r}, {g}, {b}, 0.20) 30%, \
                 rgba({r}, {g}, {b}, 0.12) 48%, \
                 rgba({r}, {g}, {b}, 0.04) 65%, \
                 rgba(20, 20, 20, 0.01) 80%, \
                 rgba(20, 20, 20, 0) 95%);",
            r = r, g = g, b = b
        )
    };

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
                    on_genre_select=Callback::new(move |g| set_selected_genre.set(g))
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
                    {move || {
                        if let Some(genre) = selected_genre.get() {
                            let gid = genre.id.to_string();
                            let gname = genre.name.clone();
                            view! {
                                <Row title=format!("Popular in {}", gname) genre_id=gid.clone() kind=kind />
                                <Row title=format!("Trending in {}", gname) genre_id=gid.clone() kind=kind />
                                <Row title=format!("Critically Acclaimed {}", gname) genre_id=gid kind=kind />
                            }.into_any()
                        } else if kind == "movie" {
                            view! {
                                <TopTenRow title="Top 10 Films in the UK Today" kind="movie" />
                                <Row title="Action & Adventure" genre_id="1365" kind="movie" />
                                <Row title="Comedies" genre_id="6548" kind="movie" />
                                <Row title="Sci-Fi Films" genre_id="1492" kind="movie" />
                                <Row title="Thrillers" genre_id="8933" kind="movie" />
                                <Row title="Dramas" genre_id="5763" kind="movie" />
                                <Row title="Horror Films" genre_id="8711" kind="movie" />
                                <Row title="Romantic Films" genre_id="8883" kind="movie" />
                                <Row title="Documentaries" genre_id="6839" kind="movie" />
                                <Row title="Crime Films" genre_id="9875" kind="movie" />
                                <Row title="Children & Family Films" genre_id="783" kind="movie" />
                                <Row title="Anime Films" genre_id="7424" kind="movie" />
                            }.into_any()
                        } else {
                            view! {
                                <TopTenRow title="Top 10 Series in the UK Today" kind="tv" />
                                <Row title="Binge-worthy TV Shows" genre_id="1191605" kind="tv" />
                                <Row title="British TV" genre_id="52117" kind="tv" />
                                <Row title="Sci-Fi & Fantasy TV" genre_id="1372" kind="tv" />
                                <Row title="Crime TV Shows" genre_id="26146" kind="tv" />
                                <Row title="TV Dramas" genre_id="11714" kind="tv" />
                                <Row title="TV Comedies" genre_id="10375" kind="tv" />
                                <Row title="Docuseries" genre_id="6839" kind="tv" />
                                <Row title="Anime Series" genre_id="7424" kind="tv" />
                                <Row title="Action & Adventure TV" genre_id="10673" kind="tv" />
                                <Row title="Kids & Family TV" genre_id="783" kind="tv" />
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </Layout>
    }
}
