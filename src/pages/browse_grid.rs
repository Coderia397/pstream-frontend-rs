use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};
use crate::components::layout::Layout;
use crate::components::media::movie_card::MovieCard;
use crate::services::tmdb::fetch_for_genre_route;
use crate::models::mapping::map_netflix_id_to_tmdb;

#[component]
pub fn BrowseGridPage() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();
    let query = use_query_map();

    let row_key = move || params.read().get("id").or_else(|| params.read().get("rowKey")).unwrap_or_default();
    let title_param = move || query.read().get("title");

    let display_title = move || {
        if let Some(t) = title_param() {
            return t;
        }
        let key = row_key();
        if let Some(ctx) = map_netflix_id_to_tmdb(&key) {
            return ctx.title.to_string();
        }
        key.split('-')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    let items_resource = LocalResource::new(move || {
        let key = row_key();
        async move {
            if key.is_empty() {
                vec![]
            } else {
                fetch_for_genre_route(&key).await.unwrap_or_default()
            }
        }
    });

    let on_back = move |_| {
        if let Some(win) = web_sys::window() {
            let _ = win.history().map(|h| h.back());
        } else {
            navigate("/browse", Default::default());
        }
    };

    view! {
        <Layout>
            <div class="bg-black md:bg-[#141414] min-h-screen pb-16 pt-[calc(4.5rem+env(safe-area-inset-top))] md:pt-24">
                <div class="px-4 md:px-10 lg:px-14">
                    // Header with back button
                    <div class="flex items-center gap-3.5 mb-8 md:mb-12">
                        <button
                            on:click=on_back
                            class="flex items-center justify-center text-white p-1.5 rounded-full hover:bg-white/10 transition-colors shrink-0 cursor-pointer"
                            aria-label="Go back"
                        >
                            <i class="ph-bold ph-arrow-left text-2xl md:text-3xl text-white"></i>
                        </button>
                        <h1 class="text-white font-bold text-2xl md:text-4xl tracking-tight">
                            {move || display_title()}
                        </h1>
                    </div>

                    // Content grid
                    <Suspense fallback=move || view! {
                        <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-6 animate-pulse">
                            {(0..15).map(|_| view! {
                                <div class="aspect-video bg-[#1e1e1e] rounded-sm border border-white/[0.04]"></div>
                            }).collect::<Vec<_>>()}
                        </div>
                    }>
                        {move || items_resource.get().map(|items| {
                            if items.is_empty() {
                                view! {
                                    <div class="flex flex-col items-center justify-center mt-24 text-center">
                                        <i class="ph ph-film-slate text-5xl text-white/30 mb-3"></i>
                                        <p class="text-white/50 text-lg">"Nothing to show here yet."</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-6">
                                        {items.into_iter().map(|item| {
                                            let movie_id = item.id;
                                            let is_tv = item.is_tv();
                                            let title = item.display_title().to_string();
                                            let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                            let poster = item.poster_url("w342").unwrap_or_default();
                                            let vote_avg = item.vote_average;
                                            view! {
                                                <MovieCard
                                                    movie_id=movie_id
                                                    is_tv=is_tv
                                                    title=title
                                                    backdrop_path=backdrop
                                                    poster_path=poster
                                                    vote_average=vote_avg
                                                />
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
