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
    let (view_mode, set_view_mode) = signal("row".to_string());

    let genres: Vec<SubNavGenre> = if kind == "tv" {
        TV_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    } else {
        MOVIE_GENRES.iter().map(|g| SubNavGenre { id: g.id as u32, name: g.name.to_string() }).collect()
    };

    let title = if kind == "tv" { "Series" } else { "Films" };

    // Hero: pull trending item for background
    let hero = LocalResource::new(move || async move {
        fetch_trending(kind).await.unwrap_or_default()
    });

    view! {
        <Layout>
            <div class="w-full pb-20 pt-16 md:pt-20">
                // Category Sub-Navigation bar
                <CategorySubNav
                    title=title.to_string()
                    genres=genres
                    selected_genre=selected_genre
                    on_genre_select=Callback::new(move |g| set_selected_genre.set(g))
                    view_mode=view_mode
                    on_view_mode_change=Callback::new(move |m| set_view_mode.set(m))
                />

                <Suspense fallback=move || view! {
                    <div class="relative w-full bg-gray-900 animate-pulse"
                         style="height:56.25vw;max-height:85vh;min-height:500px;"></div>
                }>
                    {move || hero.get().map(|result| {
                        let movies = result;
                        let movie = movies.into_iter().next();
                        match movie {
                            None => view! { <div class="h-[500px] bg-black"></div> }.into_any(),
                            Some(m) => {
                                let movie_id = m.id;
                                let is_tv = m.is_tv();
                                let title = m.display_title().to_string();
                                let backdrop = m.backdrop_url("original").unwrap_or_default();
                                let overview = m.overview.clone();
                                view! {
                                    <HeroSection movie_id=movie_id is_tv=is_tv title=title backdrop=backdrop overview=overview />
                                }.into_any()
                            }
                        }
                    })}
                </Suspense>

                <div class="relative z-30 -mt-32 space-y-4 md:space-y-6">
                    {if kind == "movie" {
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
                    }}
                </div>
            </div>
        </Layout>
    }
}
