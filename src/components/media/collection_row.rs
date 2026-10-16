use leptos::prelude::*;
use crate::services::ai_engine::{engine_text, FeedRow};
use super::movie_card::MovieCard;

/// The page that lists a whole collection: the films page of the genre the engine names (Halloween, Moods), when the page has such a genre.
pub fn collection_link(target: &serde_json::Value) -> Option<String> {
    let name = target.get("genre")?.as_str()?;
    let known = crate::data::page_genres::MOVIE_GENRES.iter().any(|g| g.name.eq_ignore_ascii_case(name));
    known.then(|| {
        let encoded: String = js_sys::encode_uri_component(name).into();
        format!("/browse/films?genre={encoded}")
    })
}

/// A themed row: the backdrops of its lead titles as a darkened background, a panel with the label, a title, a sentence and an Explore all
/// button on the left, the titles on the right. Everything it shows comes from the row the engine sent (`collection` object).
#[component]
pub fn CollectionRow(row: FeedRow) -> impl IntoView {
    let Some(c) = row.collection.clone() else { return view! { <span /> }.into_any() };
    let eyebrow = engine_text(&c.label_key, None).unwrap_or_else(|| c.label.clone());
    let is_event = c.label_key.starts_with("event.");
    let heading = if is_event {
        engine_text("collection.title", Some(&serde_json::json!({ "label": eyebrow }))).unwrap_or_else(|| row.title.clone())
    } else {
        row.title.clone()
    };
    let link = collection_link(&c.target);
    let eyebrow_text = if is_event { crate::i18n::t("collection-eyebrow-event") } else { eyebrow.clone() };
    let backdrops: Vec<String> = c.images.iter().take(5).map(|p| format!("{}/w780{}", crate::config::TMDB_IMAGE_URL, p)).collect();
    let items = row.to_media_items();
    let description = c.description.clone();
    let aria = heading.clone();

    view! {
        <section class="collection-row relative isolate overflow-hidden mx-[var(--app-x,56px)] my-6 rounded-card bg-void" aria-label=aria>
            <div class="absolute inset-0 -z-10 flex" aria-hidden="true">
                {backdrops.into_iter().map(|src| view! {
                    <img src=src alt="" class="h-full flex-1 min-w-0 object-cover opacity-60" loading="lazy" decoding="async" draggable="false" />
                }).collect::<Vec<_>>()}
            </div>
            <div class="absolute inset-0 -z-10 bg-gradient-to-r from-black/95 via-black/75 to-black/35" aria-hidden="true" />
            <div class="relative flex flex-col md:flex-row md:items-center gap-5 md:gap-8 px-5 py-6 md:px-10 md:py-10 md:min-h-[360px]">
                <div class="md:w-[34%] md:flex-none">
                    <p class="text-xs md:text-sm font-semibold uppercase tracking-[0.18em] text-ink/70">{eyebrow_text}</p>
                    <h2 class="mt-2 text-2xl md:text-4xl font-bold leading-tight text-ink">{heading}</h2>
                    {(!description.is_empty()).then(|| view! { <p class="mt-3 text-sm md:text-base text-ink/80 max-w-[34ch]">{description}</p> })}
                    {link.map(|href| view! {
                        <a href=href class="mt-5 inline-flex items-center rounded-full bg-white px-5 py-2 text-sm font-semibold text-black hover:bg-white/85 transition-colors">
                            {crate::i18n::t("collection-explore-all")}
                        </a>
                    })}
                </div>
                <div class="flex-1 min-w-0 flex gap-[var(--tile-gap)] overflow-x-auto scrollbar-hide py-2" style="scroll-behavior: smooth;">
                    {items.into_iter().map(|item| {
                        let is_tv = item.media_type.as_deref() == Some("tv");
                        let backdrop = item.backdrop_path.as_ref().map(|p| format!("{}/w780{}", crate::config::TMDB_IMAGE_URL, p)).unwrap_or_default();
                        let poster = item.poster_path.as_ref().map(|p| format!("{}/w342{}", crate::config::TMDB_IMAGE_URL, p));
                        let is_new = item.badges.iter().any(|b| b == "new");
                        view! {
                            <div class="relative flex-none w-[clamp(150px,17vw,250px)] aspect-video overflow-visible rounded-tile">
                                <MovieCard
                                    movie_id=item.id
                                    is_tv=is_tv
                                    title=item.display_title().to_string()
                                    backdrop_path=backdrop
                                    poster_path=poster.unwrap_or_default()
                                    vote_average=item.vote_average
                                    overview=item.overview.clone()
                                    genre_ids=item.genre_ids.clone()
                                    match_percentage=item.match_percentage
                                    vibe_pills=item.vibe_pills.clone()
                                    is_new=is_new
                                />
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }.into_any()
}
