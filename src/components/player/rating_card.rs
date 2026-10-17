use leptos::prelude::*;
use crate::components::media::movie_card_badges::RatingBadge;

/// The age rating card at the start of a play, laid out as measured on Netflix at 1666 x 934 and kept in vw so it scales the same way: the box sits at 4.5vw, 8.14vh, is 3.87vw high with 0.3vw / 0.9vw padding,
/// the bar (0.222vw wide, 3.48vw high) hangs just outside its left edge, the text is 1.5vw. It shows once when the picture starts, stays about 5 seconds and fades.
#[component]
pub fn RatingCard(media_id: u32, is_tv: bool, current_time: ReadSignal<f64>) -> impl IntoView {
    // 0 waiting for the picture, 1 shown, 2 fading out
    let (phase, set_phase) = signal(0u8);
    Effect::new(move |_| {
        if current_time.get() > 0.0 && phase.get_untracked() == 0 {
            set_phase.set(1);
            set_timeout(move || set_phase.set(2), std::time::Duration::from_millis(5000));
        }
    });
    view! {
        <Show when=move || (phase.get() > 0)>
            <div
                class=move || format!(
                    "pointer-events-none absolute z-10 flex items-center text-ink transition-opacity duration-(--motion-slower) {}",
                    if phase.get() == 1 { "opacity-100" } else { "opacity-0" }
                )
                style="left:4.5vw;top:8.14vh;height:3.87vw;padding:0.3vw 0.9vw;font-size:1.5vw;line-height:1.8vw;"
            >
                <span class="absolute bg-brand" style="left:-0.222vw;top:0.19vw;height:3.48vw;width:0.222vw;"></span>
                <span>{crate::i18n::t("player-rating-label")}</span>
                <span style="margin-left:0.75vw;"><RatingBadge id=media_id is_tv=is_tv size="md".to_string() /></span>
            </div>
        </Show>
    }
}
