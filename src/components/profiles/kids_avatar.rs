use leptos::prelude::*;

#[component]
pub fn KidsBadge(
    #[prop(optional, default = 15.0)] size: f64,
    #[prop(optional, default = "".to_string())] class: String,
) -> impl IntoView {
    let bg_stroke_width = size * 0.42;
    let fg_stroke_width = size * 0.04;
    let tracking = -(size * 0.035);
    let width = size * 2.8;
    let height = size * 1.5;

    view! {
        <span
            class=format!("inline-flex items-center justify-center shrink-0 select-none {class}")
            style=format!("width: {width}px; height: {height}px;")
        >
            <svg
                width=format!("{width}")
                height=format!("{height}")
                class="overflow-visible drop-shadow-sm"
                style=format!("font-family: 'Arial Rounded MT Bold', ui-rounded, 'SF Pro Rounded', 'Nunito', sans-serif; letter-spacing: {tracking}px;")
            >
                // Outer shell
                <text
                    x="50%"
                    y="52%"
                    text-anchor="middle"
                    dominant-baseline="central"
                    fill="white"
                    stroke="white"
                    stroke-width=format!("{bg_stroke_width}")
                    stroke-linejoin="round"
                    stroke-linecap="round"
                    font-weight="900"
                    font-size=format!("{size}")
                >
                    "kids"
                </text>
                // Foreground text
                <text
                    x="50%"
                    y="52%"
                    text-anchor="middle"
                    dominant-baseline="central"
                    fill="#e50914"
                    stroke="#e50914"
                    stroke-width=format!("{fg_stroke_width}")
                    stroke-linejoin="round"
                    stroke-linecap="round"
                    font-weight="900"
                    font-size=format!("{size}")
                >
                    "kids"
                </text>
            </svg>
        </span>
    }
}

#[component]
pub fn KidsAvatar(
    #[prop(optional, default = 100.0)] size: f64,
    #[prop(optional, default = "".to_string())] class: String,
) -> impl IntoView {
    let badge_size = (11.0_f64).max(size * 0.3);

    view! {
        <div class=format!("relative w-full h-full flex overflow-hidden {class}")>
            <div class="flex-1 h-full" style="background: linear-gradient(180deg, #2e8b4f 0%, #6f6f8f 55%, #8d3fd1 100%);" />
            <div class="flex-1 h-full" style="background: linear-gradient(180deg, #f6c244 0%, #ef8f3a 60%, #e05238 100%);" />
            <div class="flex-1 h-full" style="background: linear-gradient(180deg, #f66ab5 0%, #ec2fa0 100%);" />
            <div class="flex-1 h-full" style="background: linear-gradient(180deg, #cfc4f5 0%, #7f8ff0 55%, #2f6bf0 100%);" />
            <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
                <KidsBadge size=badge_size />
            </div>
        </div>
    }
}
