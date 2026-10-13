use crate::data::ratings::{age_label, colors, AgeLabel, Say};
use crate::pages::settings::prefs::use_prefs;
use leptos::prelude::*;

/// A badge for an age label: one look for both rating systems (solid colour by rank, the text in a readable colour, a thin ring so it
/// stands out on any poster). `size` is "xs", "sm", "md" or "lg"; the badge is drawn in em, so it also follows the text around it.
#[component]
pub fn AgeBadge(label: AgeLabel, #[prop(default = "sm".to_string())] size: String) -> impl IntoView {
    let (bg, fg) = colors(label.rank);
    let px = match size.as_str() {
        "xs" => 11,
        "md" => 16,
        "lg" => 20,
        _ => 13,
    };
    let (shown, aria) = match &label.say {
        Say::Rated(text) => (text.clone(), crate::i18n::t_args("media-rating-aria", &[("rating", fluent_bundle::FluentValue::from(text.clone()))])),
        Say::All => (crate::i18n::t("age-badge-all"), crate::i18n::t("age-aria-all")),
        Say::Plus(n) => (format!("{n}+"), crate::i18n::t_args("age-aria-plus", &[("age", fluent_bundle::FluentValue::from(*n as i64))])),
        Say::NotRated => ("NR".to_string(), crate::i18n::t("media-rating-nr")),
    };
    view! {
        <span class="age-badge" role="img" aria-label=aria style=format!("--age-bg:{bg};--age-fg:{fg};font-size:{px}px")>{shown}</span>
    }
}

/// The badge for a title: shows the certificates the caller already has, otherwise asks the engine (NR until it answers or when the title
/// has none). The label follows the rating system the person reads (see `Prefs::rating_system`).
#[component]
pub fn RatingBadge(
    id: u32,
    #[prop(default = false)] is_tv: bool,
    #[prop(default = "sm".to_string())] size: String,
    #[prop(default = None)] gb: Option<String>,
    #[prop(default = None)] us: Option<String>,
) -> impl IntoView {
    let prefs = use_prefs();
    let known = gb.is_some() || us.is_some();
    let resource = LocalResource::new(move || {
        let (gb, us) = (gb.clone(), us.clone());
        async move {
            if known {
                crate::services::rating::RatingInfo { gb, us }
            } else {
                crate::services::rating::fetch(id, is_tv).await
            }
        }
    });
    view! {
        {move || {
            let info = resource.get().unwrap_or_default();
            let label = age_label(prefs.rating_system(), info.gb.as_deref(), info.us.as_deref());
            view! { <AgeBadge label=label size=size.clone() /> }
        }}
    }
}
