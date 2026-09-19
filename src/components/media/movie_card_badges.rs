use leptos::prelude::*;

#[component]
pub fn MaturityBadge(
    #[prop(default = String::new())] certification: String,
    #[prop(default = false)] adult: bool,
    #[prop(optional)] vote_average: Option<f64>,
    #[prop(default = "sm".to_string())] size: String,
) -> impl IntoView {
    let cert = certification.trim().to_uppercase();

    // Official BBFC classifications & exact colors
    let (label, fill, is_triangle, is_square) = match cert.as_str() {
        "U" | "G" | "TV-G" | "TV-Y" | "TV-Y7" | "ALL" | "0" | "0+" => ("U", "#00a826", true, false),
        "PG" | "TV-PG" | "6" | "7" | "6+" | "7+" => ("PG", "#f5a800", true, false),
        "12A" | "PG-13" => ("12A", "#f05a1a", false, false),
        "12" | "TV-14" | "12+" | "13" | "14" => ("12", "#f05a1a", false, false),
        "15" | "15+" | "16" | "16+" => ("15", "#eb287f", false, false),
        "18" | "R" | "TV-MA" | "NC-17" | "18+" => ("18", "#dc1420", false, false),
        "R18" => ("R18", "#0060cf", false, true),
        _ => {
            if adult {
                ("18", "#dc1420", false, false)
            } else if let Some(vote) = vote_average {
                if vote >= 7.8 {
                    ("18", "#dc1420", false, false)
                } else if vote >= 6.0 {
                    ("15", "#eb287f", false, false)
                } else {
                    ("12", "#f05a1a", false, false)
                }
            } else if !cert.is_empty() && (cert.contains("18") || cert.contains("MA") || cert.contains("R")) {
                ("18", "#dc1420", false, false)
            } else if !cert.is_empty() && (cert.contains("15") || cert.contains("16")) {
                ("15", "#eb287f", false, false)
            } else if !cert.is_empty() && cert.contains("PG") {
                ("PG", "#f5a800", true, false)
            } else if !cert.is_empty() && cert.contains("U") {
                ("U", "#00a826", true, false)
            } else {
                ("12", "#f05a1a", false, false)
            }
        }
    };

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
                aria-label=format!("BBFC {} rating", label)
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
                aria-label=format!("BBFC {} rating", label)
            >
                // Outer dark outline
                <rect x="3" y="3" width="94" height="94" rx="18" ry="18" fill="black" />
                // White margin ring
                <rect x="5" y="5" width="90" height="90" rx="16" ry="16" fill="white" />
                // Official BBFC blue rounded rect
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
                aria-label=format!("BBFC {} rating", label)
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
