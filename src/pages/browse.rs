use leptos::prelude::*;
use crate::services::tmdb::{fetch_trending};
use crate::components::media::row::Row;
use crate::components::media::top_ten_row::TopTenRow;
use crate::components::media::hero::HeroSection;

#[component]
pub fn BrowseHome() -> impl IntoView {
    use crate::components::layout::Layout;

    // Hero: pull a single trending item for the background
    let hero = LocalResource::new(move || async move {
        fetch_trending("movie").await.unwrap_or_default()
    });

    view! {
        <Layout>
            <div class="w-full pb-20 -mt-16">
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

                <div class="relative z-30 -mt-32 space-y-2 sm:space-y-4 md:space-y-6">
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
