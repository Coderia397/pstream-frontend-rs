use leptos::prelude::*;
use crate::services::tmdb::{fetch_trending};
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;

#[component]
pub fn BrowseHome() -> impl IntoView {
    use crate::components::layout::Layout;
    use crate::store::use_ui_store;

    let ui_store = use_ui_store();

    // Hero: pull a single trending item for the background
    let hero = LocalResource::new(move || async move {
        fetch_trending("movie").await.unwrap_or_default()
    });

    let ambient_bg_style = move || {
        let (r, g, b) = ui_store.ambient_color.get();
        format!(
            "background: \
             radial-gradient(ellipse 110% 55% at 50% 0%, rgba({r}, {g}, {b}, 0.52) 0%, rgba({r}, {g}, {b}, 0.28) 35%, rgba({r}, {g}, {b}, 0.10) 50%, transparent 68%), \
             linear-gradient(to bottom, \
                 rgba({r}, {g}, {b}, 0.40) 0%, \
                 rgba({r}, {g}, {b}, 0.35) 30%, \
                 rgba({r}, {g}, {b}, 0.24) 48%, \
                 rgba({r}, {g}, {b}, 0.10) 65%, \
                 rgba(20, 20, 20, 0.02) 80%, \
                 rgba(20, 20, 20, 0) 95%);",
            r = r, g = g, b = b
        )
    };

    view! {
        <Layout>
            <div class="w-full pb-20 bg-black md:bg-[#141414] min-h-screen relative overflow-x-hidden">
                // Ambient Atmospheric Glow behind navbar and hero billboard (fades after 50% of hero height)
                <div
                    class="absolute inset-x-0 top-0 h-[580px] md:h-[660px] lg:h-[720px] pointer-events-none -z-0 transition-all duration-700 ease-out"
                    style=ambient_bg_style
                />

                // Hero Carousel driven by real TMDB data
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
                    <TopTenRow title="Top 10 Movies Today" kind="movie" />
                    <TopTenRow title="Top 10 TV Shows Today" kind="tv" />
                    <Row title="Action & Adventure" genre_id="1365" />
                    <Row title="Comedies" genre_id="6548" />
                    <Row title="Sci-Fi & Fantasy" genre_id="1492" />
                    <Row title="British TV Shows" genre_id="52117" />
                    <Row title="Thrillers" genre_id="8933" />
                    <Row title="TV Dramas" genre_id="11714" />
                    <Row title="Horror Films" genre_id="8711" />
                    <Row title="Documentaries" genre_id="6839" />
                    <Row title="Crime TV Shows" genre_id="26146" />
                    <Row title="Romantic Films" genre_id="8883" />
                    <Row title="TV Sci-Fi & Fantasy" genre_id="1372" />
                    <Row title="Children & Family" genre_id="783" />
                    <Row title="Anime & Animation" genre_id="7424" />
                    <Row title="Crime Films" genre_id="9875" />
                </div>
            </div>
        </Layout>
    }
}
