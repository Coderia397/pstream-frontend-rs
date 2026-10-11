use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::layout::Layout;
use crate::services::tmdb::search_media;
use crate::components::media::movie_card::MovieCard;

#[component]
pub fn SearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let q = move || query_map.read().get("q").unwrap_or_default();

    let results = LocalResource::new(move || {
        let current_q = q();
        async move {
            if current_q.trim().is_empty() {
                vec![]
            } else {
                search_media(&current_q).await.unwrap_or_default()
            }
        }
    });

    view! {
        <Layout>
            <div class="pt-[calc(5rem+env(safe-area-inset-top))] md:pt-28 px-[var(--app-x,56px)] pb-12 min-h-screen bg-void md:bg-page">
                <div class="mb-8">
                    <h1 class="text-gray-400 text-lg">
                        {move || if !q().is_empty() { format!("Results for \"{}\"", q()) } else { crate::i18n::t("nav-search") }}
                    </h1>
                </div>

                <Suspense fallback=move || view! {
                    <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-6 animate-pulse">
                        {(0..10).map(|_| view! {
                            <div class="aspect-video bg-card rounded-control"></div>
                        }).collect::<Vec<_>>()}
                    </div>
                }>
                    {move || results.get().map(|media| {
                        if media.is_empty() && !q().is_empty() {
                            return view! {
                                <div class="flex flex-col items-center justify-center mt-20 text-center">
                                    <div class="text-xl text-ink mb-2">{crate::i18n::t("search-no-matches")}</div>
                                    <div class="text-gray-400 text-sm">{crate::i18n::t("search-try-another")}</div>
                                </div>
                            }.into_any();
                        }

                        if media.is_empty() {
                            return view! {
                                <div class="flex flex-col items-center justify-center mt-20 text-center">
                                    <div class="text-xl text-ink mb-2">{crate::i18n::t("search-explore")}</div>
                                    <div class="text-gray-400 text-sm">{crate::i18n::t("search-explore-hint")}</div>
                                </div>
                            }.into_any();
                        }
                        
                        view! {
                            <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-12 md:gap-y-6">
                                {media.into_iter().map(|item| {
                                    let movie_id = item.id;
                                    let is_tv = item.is_tv();
                                    let title = item.display_title().to_string();
                                    let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                    let poster = item.poster_url("w342").unwrap_or_default();
                                    view! { 
                                        <MovieCard
                                            movie_id=movie_id
                                            is_tv=is_tv
                                            title=title
                                            backdrop_path=backdrop
                                            poster_path=poster
                                            vote_average=item.vote_average
                                            overview=item.overview.clone()
                                            genre_ids=item.genre_ids.clone()
                                            vibe_pills=item.vibe_pills.clone()
                                            is_grid=true
                                        /> 
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    })}
                </Suspense>
            </div>
        </Layout>
    }
}
