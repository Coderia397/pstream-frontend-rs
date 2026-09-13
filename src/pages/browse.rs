use leptos::prelude::*;
use crate::services::tmdb::{fetch_trending};
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;
use crate::components::media::continue_watching_row::ContinueWatchingRow;

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
                    <ContinueWatchingRow />
                    <TopTenRow title="Top 10 Movies Today" kind="movie" />
                    <TopTenRow title="Top 10 TV Shows Today" kind="tv" />
                    <Row title="Action & Adventure" genre_id="1365" kind="movie" />
                    <Row title="Comedies" genre_id="6548" kind="movie" />
                    <Row title="Sci-Fi & Fantasy" genre_id="1492" kind="movie" />
                    <Row title="British TV Shows" genre_id="british" kind="tv" />
                    <Row title="Thrillers" genre_id="8933" kind="movie" />
                    <Row title="TV Dramas" genre_id="18" kind="tv" />
                    <Row title="Horror Films" genre_id="8711" kind="movie" />
                    <Row title="Documentaries" genre_id="6839" kind="movie" />
                    <Row title="Crime TV Shows" genre_id="80" kind="tv" />
                    <Row title="Romantic Films" genre_id="8883" kind="movie" />
                    <Row title="TV Sci-Fi & Fantasy" genre_id="10765" kind="tv" />
                    <Row title="Children & Family" genre_id="783" kind="movie" />
                    <Row title="Anime & Animation" genre_id="anime" kind="movie" />
                    <Row title="Crime Films" genre_id="9875" kind="movie" />
                </div>
            </div>
        </Layout>
    }
}
