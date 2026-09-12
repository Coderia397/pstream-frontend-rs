use leptos::prelude::*;
use crate::models::movie::Movie;
use crate::store::{use_library_store, use_ui_store, LibraryEntry};
use crate::services::tmdb::{fetch_movie_logo, MediaItem};

#[component]
pub fn MobileHero(
    movie: Movie,
    #[prop(optional)] on_play: Option<Callback<Movie>>,
    #[prop(optional)] on_select: Option<Callback<Movie>>,
) -> impl IntoView {
    let library_store = use_library_store();
    let ui_store = use_ui_store();
    let movie_id = movie.id_u32();
    let is_tv = movie.is_tv();
    let title = movie.display_title();

    let is_added = Signal::derive(move || {
        library_store.my_list.get().contains_key(&movie_id)
    });

    let logo_resource = LocalResource::new(move || {
        let mid = movie_id;
        let tv = is_tv;
        async move {
            if mid > 0 {
                fetch_movie_logo(mid, tv).await.ok().flatten()
            } else {
                None
            }
        }
    });

    let poster_url = movie.poster_path.as_ref().or(movie.backdrop_path.as_ref()).map(|p| {
        if p.starts_with("http") {
            p.clone()
        } else {
            format!("https://image.tmdb.org/t/p/w780{}", p)
        }
    }).unwrap_or_default();

    let movie_c1 = movie.clone();
    let movie_c2 = movie.clone();
    let movie_c3 = movie.clone();

    let toggle_my_list = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        let mut list = library_store.my_list.get();
        if list.contains_key(&movie_id) {
            list.remove(&movie_id);
        } else {
            list.insert(movie_id, LibraryEntry {
                media: MediaItem {
                    id: movie_id,
                    title: movie_c1.title.clone(),
                    name: movie_c1.name.clone(),
                    overview: movie_c1.overview.clone(),
                    poster_path: movie_c1.poster_path.clone(),
                    backdrop_path: movie_c1.backdrop_path.clone(),
                    vote_average: movie_c1.vote_average,
                    release_date: movie_c1.release_date.clone(),
                    first_air_date: movie_c1.first_air_date.clone(),
                    media_type: movie_c1.media_type.clone(),
                },
                added_at: js_sys::Date::now() as u64,
            });
        }
        library_store.my_list.set(list);
    };

    let handle_card_click = move |_| {
        if let Some(cb) = on_select {
            cb.run(movie_c2.clone());
        }
    };

    let handle_play = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        if let Some(cb) = on_play {
            cb.run(movie_c3.clone());
        } else if let Some(w) = web_sys::window() {
            let kind = if is_tv { "tv" } else { "movie" };
            let _ = w.location().set_href(&format!("/watch/{}/{}", kind, movie_id));
        }
    };

    let mobile_ambient_style = move || {
        let (r, g, b) = ui_store.ambient_color.get();
        format!(
            "background: linear-gradient(to bottom, \
                rgba({r},{g},{b},0.22) 0%, \
                rgba({r},{g},{b},0.14) 25%, \
                rgba({r},{g},{b},0.06) 48%, \
                rgba({r},{g},{b},0.01) 65%, \
                rgba(0,0,0,0) 78%);",
            r = r, g = g, b = b
        )
    };

    view! {
        <div class="relative z-0 overflow-visible w-full px-4 pt-20 pb-5 flex flex-col items-center justify-center transition-all duration-700 ease-in-out md:hidden">
            // Ambient gradient glow (fades after 50% of hero height)
            <div
                class="absolute inset-x-0 top-0 h-[520px] pointer-events-none -z-10 transition-all duration-700 ease-out"
                style=mobile_ambient_style
            />

            // Floating Centered Card
            <div
                on:click=handle_card_click
                class="w-[94%] max-w-[400px] aspect-[2/3] relative rounded-2xl overflow-hidden border border-white/[0.15] shadow-[0_20px_60px_rgba(0,0,0,0.95)] cursor-pointer active:scale-[0.98] transition-all duration-200"
            >
                <img
                    src=poster_url
                    alt=title.clone()
                    class="absolute inset-0 w-full h-full object-cover select-none pointer-events-none"
                    loading="eager"
                />

                // Vignette overlays
                <div class="absolute inset-0 bg-gradient-to-t from-black/95 via-black/60 to-transparent pointer-events-none" />
                <div class="absolute inset-0 bg-gradient-to-r from-black/80 via-black/30 to-transparent pointer-events-none" />

                // Details Container
                <div class="absolute inset-x-0 bottom-0 px-4 pb-6 pt-12 flex flex-col items-center text-center z-10 w-full">
                    // Logo or Title
                    <div class="relative inline-flex items-end mb-4 max-w-[80%] max-h-[75px] w-full justify-center">
                        <Suspense fallback=move || view! { <div class="h-10 w-32 bg-white/10 animate-pulse rounded" /> }>
                            {
                                let tc = title.clone();
                                move || {
                                    let t = tc.clone();
                                    logo_resource.get().map(|res| match res {
                                        Some(url) if !url.is_empty() => view! {
                                            <img
                                                src=url
                                                alt=t.clone()
                                                class="object-contain object-bottom max-h-[70px] w-auto drop-shadow-xl"
                                            />
                                        }.into_any(),
                                        _ => view! {
                                            <h2 class="text-xl font-black font-leaner drop-shadow-xl leading-tight text-white tracking-wide uppercase line-clamp-2">
                                                {t.clone()}
                                            </h2>
                                        }.into_any()
                                    })
                                }
                            }
                        </Suspense>
                    </div>

                    // Genre / category chips
                    <div class="flex items-center justify-center flex-wrap gap-x-2 gap-y-1 mb-4 text-[11px] font-semibold text-white/80 tracking-wide select-none">
                        <span>"Trending"</span>
                        <span class="text-white/30">"•"</span>
                        <span>"Popular"</span>
                        <span class="text-white/30">"•"</span>
                        <span>{if is_tv { "Series" } else { "Film" }}</span>
                    </div>

                    // Buttons Row
                    <div class="flex items-center justify-center w-full max-w-[340px] gap-3 mt-1">
                        // Play button
                        <button
                            type="button"
                            on:click=handle_play
                            class="flex-1 flex items-center justify-center h-[46px] rounded-[4px] bg-white hover:bg-neutral-200 text-black font-bold text-lg gap-2 transition-all active:scale-95 shadow-md cursor-pointer"
                        >
                            <i class="ph-fill ph-play text-2xl"></i>
                            <span>"Play"</span>
                        </button>

                        // My List button
                        <button
                            type="button"
                            on:click=toggle_my_list
                            class="flex-1 flex items-center justify-center h-[46px] rounded-[4px] bg-[#6d6d6e]/40 hover:bg-[#6d6d6e]/25 text-white font-bold text-lg gap-2 transition-all active:scale-95 shadow-md cursor-pointer"
                        >
                            <i class=move || if is_added.get() { "ph-bold ph-check text-2xl" } else { "ph-bold ph-plus text-2xl" }></i>
                            <span>"My List"</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
