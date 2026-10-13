use leptos::prelude::*;

/// How a rating is drawn: the BBFC label, its colour and its shape.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BadgeStyle {
    pub label: &'static str,
    pub fill: &'static str,
    pub triangle: bool,
    pub square: bool,
}

/// The badge for a certificate text. British ratings map to themselves, common US and other labels map to the nearest British one (the engine
/// already sends British ratings). An adult flag from the data counts as 18. Anything else, including an empty text, is NR (not rated):
/// nothing is guessed from genres or scores.
pub fn badge_style(certification: &str, adult: bool) -> BadgeStyle {
    let style = |label, fill, triangle, square| BadgeStyle { label, fill, triangle, square };
    match certification.trim().to_uppercase().as_str() {
        "U" | "G" | "TV-G" | "TV-Y" | "ALL" | "0" | "0+" => style("U", "#00a826", true, false),
        "PG" | "TV-PG" | "TV-Y7" | "TV-Y7-FV" | "6" | "7" | "6+" | "7+" => style("PG", "#f5a800", true, false),
        "12A" | "PG-13" => style("12A", "#f05a1a", false, false),
        "12" | "12+" | "13" | "14" => style("12", "#f05a1a", false, false),
        "15" | "15+" | "16" | "16+" | "TV-14" => style("15", "#eb287f", false, false),
        "18" | "R" | "TV-MA" | "NC-17" | "18+" => style("18", "#dc1420", false, false),
        "R18" => style("R18", "#0060cf", false, true),
        _ if adult => style("18", "#dc1420", false, false),
        _ => style("NR", "#6b6b72", false, false),
    }
}

#[component]
pub fn MaturityBadge(
    #[prop(default = String::new())] certification: String,
    #[prop(default = false)] adult: bool,
    #[prop(default = "sm".to_string())] size: String,
) -> impl IntoView {
    let BadgeStyle { label, fill, triangle: is_triangle, square: is_square } = badge_style(&certification, adult);

    let aria_label = if label == "NR" { crate::i18n::t("media-rating-nr") } else { crate::i18n::t_args("media-rating-aria", &[("rating", fluent_bundle::FluentValue::from(label.to_string()))]) };
    let (w_px, h_px) = match size.as_str() {
        "xs" => (20, if is_triangle { 18 } else { 20 }),
        "sm" => (24, if is_triangle { 21 } else { 24 }),
        "md" => (32, if is_triangle { 28 } else { 32 }),
        "lg" => (40, if is_triangle { 35 } else { 40 }),
        _ => (24, if is_triangle { 21 } else { 24 }),
    };

    if is_triangle {
        let (fs, y_pos) = if label == "U" { (42, 58) } else { (34, 58) };
        view! {
            <svg
                width=w_px
                height=h_px
                viewBox="0 0 110 96"
                class="shrink-0 inline-block align-middle select-none drop-shadow-sm"
                aria-label=aria_label.clone()
            >
                // Outer dark outline
                <path d="M 55 12 L 98 86 L 12 86 Z" fill="black" stroke="black" stroke-width="10" stroke-linejoin="round" />
                // White margin ring
                <path d="M 55 14 L 96 84 L 14 84 Z" fill="white" stroke="white" stroke-width="6" stroke-linejoin="round" />
                // Official BBFC colored triangle
                <path d="M 55 20 L 91 80 L 19 80 Z" fill=fill stroke=fill stroke-width="2" stroke-linejoin="round" />
                <text
                    x="55"
                    y=y_pos
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size=fs
                    font-weight="900"
                    font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif"
                    fill="white"
                >
                    {label}
                </text>
            </svg>
        }.into_any()
    } else if is_square {
        view! {
            <svg
                width=w_px
                height=h_px
                viewBox="0 0 100 100"
                class="shrink-0 inline-block align-middle select-none drop-shadow-sm"
                aria-label=aria_label.clone()
            >
                // Outer dark outline
                <rect x="3" y="3" width="94" height="94" rx="18" ry="18" fill="black" />
                // White margin ring
                <rect x="5" y="5" width="90" height="90" rx="16" ry="16" fill="white" />
                // Official BBFC blue rounded-control rect
                <rect x="13" y="13" width="74" height="74" rx="11" ry="11" fill=fill />
                <text
                    x="50"
                    y="51"
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size="30"
                    font-weight="900"
                    font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif"
                    fill="white"
                >
                    {label}
                </text>
            </svg>
        }.into_any()
    } else {
        let fs = if label.len() <= 2 { 42 } else { 31 };
        view! {
            <svg
                width=w_px
                height=h_px
                viewBox="0 0 100 100"
                class="shrink-0 inline-block align-middle select-none drop-shadow-sm"
                aria-label=aria_label.clone()
            >
                // Outer dark outline
                <circle cx="50" cy="50" r="49" fill="black" />
                // White margin ring
                <circle cx="50" cy="50" r="47" fill="white" />
                // Official BBFC colored circle
                <circle cx="50" cy="50" r="39" fill=fill />
                <text
                    x="50"
                    y="51"
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size=fs
                    font-weight="900"
                    font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif"
                    fill="white"
                >
                    {label}
                </text>
            </svg>
        }.into_any()
    }
}

/// The badge for a title: shows the given certificate when the caller already has it, otherwise asks the engine (NR until it answers
/// or when the title has none).
#[component]
pub fn RatingBadge(
    id: u32,
    #[prop(default = false)] is_tv: bool,
    #[prop(default = "sm".to_string())] size: String,
    #[prop(optional)] known: Option<String>,
) -> impl IntoView {
    let resource = LocalResource::new(move || {
        let known = known.clone().filter(|k| !k.trim().is_empty());
        async move {
            match known {
                Some(k) => k,
                None => crate::services::rating::fetch(id, is_tv).await,
            }
        }
    });
    view! {
        {move || {
            let label = resource.get().map(|r| r.to_string()).unwrap_or_else(|| "NR".to_string());
            view! { <MaturityBadge certification=label size=size.clone() /> }
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::badge_style;

    #[test]
    fn british_ratings_map_to_themselves() {
        for r in ["U", "PG", "12A", "12", "15", "18", "R18"] {
            assert_eq!(badge_style(r, false).label, r);
        }
    }

    #[test]
    fn us_ratings_map_to_the_nearest_british_one() {
        assert_eq!(badge_style("TV-Y7", false).label, "PG");
        assert_eq!(badge_style("PG-13", false).label, "12A");
        assert_eq!(badge_style("TV-14", false).label, "15");
        assert_eq!(badge_style("TV-MA", false).label, "18");
        assert_eq!(badge_style(" r ", false).label, "18");
    }

    #[test]
    fn no_certificate_is_not_rated_and_nothing_is_guessed() {
        for r in ["", "  ", "NR", "Unrated", "Not Rated", "X", "???"] {
            let b = badge_style(r, false);
            assert_eq!(b.label, "NR", "{r:?}");
            assert_eq!(b.fill, "#6b6b72");
            assert!(!b.triangle && !b.square);
        }
    }

    #[test]
    fn an_adult_flag_is_the_only_signal_for_an_unrated_title() {
        assert_eq!(badge_style("", true).label, "18");
        assert_eq!(badge_style("NR", true).label, "18");
        assert_eq!(badge_style("PG", true).label, "PG");
    }

    #[test]
    fn shapes_follow_the_bbfc_style() {
        assert!(badge_style("U", false).triangle && badge_style("PG", false).triangle);
        assert!(badge_style("R18", false).square);
        assert!(!badge_style("15", false).triangle);
    }
}
