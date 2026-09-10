use leptos::prelude::*;

#[component]
pub fn MaturityBadge(
    #[prop(default = String::new())] certification: String,
    #[prop(default = false)] adult: bool,
    #[prop(default = "sm".to_string())] size: String,
) -> impl IntoView {
    let cert = certification.trim().to_uppercase();

    let (label, fill, is_triangle) = match cert.as_str() {
        "U" | "G" | "TV-G" | "TV-Y" | "TV-Y7" => ("U", "#4CAF50", true),
        "PG" | "TV-PG" => ("PG", "#FFA000", true),
        "12A" | "PG-13" | "TV-14" => ("12A", "#E65100", false),
        "12" => ("12", "#E65100", false),
        "15" => ("15", "#D81B8C", false),
        "18" | "R" | "TV-MA" | "NC-17" => ("18", "#C62828", false),
        "R18" => ("R18", "#6A0D83", false),
        _ => {
            if adult {
                ("18", "#C62828", false)
            } else {
                ("", "", false)
            }
        }
    };

    if label.is_empty() {
        return view! { <span /> }.into_any();
    }

    let px = if size == "md" { 40 } else if size == "xs" { 20 } else { 30 };

    if is_triangle {
        let fs = if label.len() == 1 { 46 } else { 34 };
        let outer_path = "M48.3,17.7 Q55,5.3 61.7,17.7 L98.7,86 Q105.4,98.4 91.4,98.4 L18.6,98.4 Q4.6,98.4 11.3,86 Z";
        let inner_path = "M50.2,17.8 Q55,9 59.8,17.8 L97.2,85.7 Q102,94.5 92,94.5 L18,94.5 Q8,94.5 12.8,85.7 Z";
        view! {
            <svg
                width=px
                height=px * 100 / 110
                viewBox="0 0 110 100"
                class="shrink-0 inline-block"
            >
                <path d=outer_path fill="white" />
                <path d=inner_path fill=fill />
                <text
                    x="55"
                    y="79"
                    text-anchor="middle"
                    font-size=fs
                    font-weight="600"
                    font-family="'Arial Black', Arial, sans-serif"
                    fill="white"
                >
                    {label}
                </text>
            </svg>
        }.into_any()
    } else {
        view! {
            <svg
                width=px
                height=px
                viewBox="0 0 100 100"
                class="shrink-0 inline-block"
            >
                <circle cx="50" cy="50" r="49" fill="white" />
                <circle cx="50" cy="50" r="42" fill=fill />
                <text
                    x="50"
                    y="63"
                    text-anchor="middle"
                    font-size=if label.len() <= 2 { 38 } else { 28 }
                    font-weight="bold"
                    font-family="'Arial Black', Arial, sans-serif"
                    fill="white"
                >
                    {label}
                </text>
            </svg>
        }.into_any()
    }
}
