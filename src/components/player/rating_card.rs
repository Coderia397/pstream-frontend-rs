use leptos::prelude::*;
use crate::components::media::movie_card_badges::AgeBadge;
use crate::data::ratings::age_label;
use crate::pages::settings::prefs::use_prefs;

/// The age rating card at the start of a play, laid out as measured on Netflix at 1666 x 934 and kept in vw so it scales the same way: the box sits at 4.5vw, 8.14vh, is 3.87vw high with 0.3vw / 0.9vw padding,
/// the bar (0.222vw wide, 3.48vw high) hangs just outside its left edge, the text is 1.5vw. It reads "Rating:", the badge, then the reasons in plain words ("violence, strong language"). The rating belongs to the
/// show, so every episode has the same badge; the reasons come per video from the engine. A title without a rating gets no card (nothing is guessed). It shows once when the picture starts, stays about 5 seconds and fades.
#[component]
pub fn RatingCard(media_id: u32, is_tv: bool, season: Option<u32>, episode: Option<u32>, current_time: ReadSignal<f64>) -> impl IntoView {
    let prefs = use_prefs();
    let info = LocalResource::new(move || async move { crate::services::rating::fetch(media_id, is_tv).await });
    let reasons = LocalResource::new(move || async move { crate::services::ai_engine::fetch_content_reasons(is_tv, media_id, season, episode).await });
    let label = move || info.get().map(|i| age_label(prefs.rating_system(), i.gb.as_deref(), i.us.as_deref()));
    let rated = move || label().is_some_and(|l| l.rank != 0);
    // 0 waiting for the picture, 1 shown, 2 fading out
    let (phase, set_phase) = signal(0u8);
    Effect::new(move |_| {
        if current_time.get() > 0.0 && phase.get_untracked() == 0 {
            set_phase.set(1);
            set_timeout(move || set_phase.set(2), std::time::Duration::from_millis(5000));
        }
    });
    let reasons_text = move || {
        // the words are the British classification board's; a reader of the American system gets the American word where there is one (reason-us-...), else the same word
        let us = prefs.rating_system() == crate::data::ratings::RatingSystem::Us;
        // a word the page has no text for (an old answer) is left out, never shown as a key; the first three of the list are shown
        reasons.get().map(|r| r.into_iter().filter_map(|x| {
            let slug = x.replace(' ', "-");
            let key = format!("reason-us-{slug}");
            let word = if us { crate::i18n::t(&key) } else { key.clone() };
            let word = if word == key { crate::i18n::t(&format!("reason-{slug}")) } else { word };
            (!word.starts_with("reason-")).then_some(word)
        }).take(3).collect::<Vec<_>>().join(", ")).unwrap_or_default()
    };
    view! {
        <Show when=move || (phase.get() > 0) && rated()>
            <div
                class=move || format!(
                    "pointer-events-none absolute z-10 flex items-center text-ink transition-opacity duration-(--motion-slower) {}",
                    if phase.get() == 1 { "opacity-100" } else { "opacity-0" }
                )
                style="left:4.5vw;top:8.14vh;height:3.87vw;padding:0.3vw 0.9vw;font-size:1.5vw;line-height:1.8vw;"
            >
                <span class="absolute bg-brand" style="left:-0.222vw;top:0.19vw;height:3.48vw;width:0.222vw;"></span>
                <span>{crate::i18n::t("player-rating-label")}</span>
                <span style="margin-left:0.75vw;">
                    {move || label().map(|l| view! { <AgeBadge label=l system=prefs.rating_system() size="md".to_string() /> })}
                </span>
                <span style="margin-left:0.75vw;">{reasons_text}</span>
            </div>
        </Show>
    }
}
