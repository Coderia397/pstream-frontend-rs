use leptos::prelude::*;
use crate::components::layout::Layout;
use crate::store::use_library_store;
use crate::components::media::movie_card::MovieCard;

#[component]
pub fn MyListPage() -> impl IntoView {
    let library = use_library_store();
    
    let sorted_list = move || {
        let mut items: Vec<_> = library.my_list.get().into_values().collect();
        items.sort_by(|a, b| b.added_at.cmp(&a.added_at)); // newest first
        items
    };

    view! {
        <Layout>
            <div class="relative min-h-screen">
                <div class="pt-36 md:pt-44 px-[var(--app-x,56px)] pb-12">
                    <h1 class="text-3xl font-bold text-white mb-8">"My List"</h1>
                    
                    {move || {
                        let items = sorted_list();
                        if items.is_empty() {
                            view! {
                                <div class="flex flex-col items-center justify-center mt-32 text-gray-500 animate-fadeIn">
                                    <div class="w-20 h-20 rounded-full bg-[#222] flex items-center justify-center mb-6 border border-white/5">
                                        <i class="ph-bold ph-playlist text-[40px] text-gray-600"></i>
                                    </div>
                                    <p class="text-xl font-medium text-gray-300">"Your list is empty"</p>
                                    <p class="text-sm mt-2 max-w-md text-center">"Add shows and movies to keep track of what you want to watch."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-2.5 gap-y-12 md:gap-y-6 animate-fadeIn">
                                    {items.into_iter().map(|entry| {
                                        let item = entry.media;
                                        let movie_id = item.id;
                                        let is_tv = item.is_tv();
                                        let title = item.display_title().to_string();
                                        let backdrop = item.backdrop_url("w780").unwrap_or_default();
                                        let poster = item.poster_url("w342").unwrap_or_default();
                                        view! { 
                                            <MovieCard movie_id=movie_id is_tv=is_tv title=title backdrop_path=backdrop poster_path=poster vote_average=item.vote_average /> 
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </Layout>
    }
}
