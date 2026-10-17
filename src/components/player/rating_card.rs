use leptos::prelude::*;
use crate::components::media::movie_card_badges::RatingBadge;

/// The age rating card at the start of a play: a bar in the accent colour, "Rating:" and the badge. It shows once when the picture starts, stays about 5 seconds and fades.
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
                    "pointer-events-none absolute left-[110px] top-[106px] z-10 flex items-center gap-5 border-s-[3px] border-brand ps-5 py-2 text-ink transition-opacity duration-(--motion-slower) {}",
                    if phase.get() == 1 { "opacity-100" } else { "opacity-0" }
                )
            >
                <span class="text-[28px] leading-8">{crate::i18n::t("player-rating-label")}</span>
                <span class="text-[28px]"><RatingBadge id=media_id is_tv=is_tv size="md".to_string() /></span>
            </div>
        </Show>
    }
}
