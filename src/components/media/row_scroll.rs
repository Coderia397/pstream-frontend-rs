//! What every horizontal row shares: how a page turn ends, and the buttons at the two edges.
//!
//! A page turn moves the row by one screenful; the last page is cut to the end of the row (so it shows the last cards rather than a half empty page); one more turn forward goes back to
//! the start and one more turn back from the start goes to the end, as in the app, so a row can be paged round for ever.

use leptos::prelude::*;

/// The scroll position a page turn ends at. `step` is the width of a page.
pub fn page_target(forward: bool, current: f64, client_width: f64, scroll_width: f64, step: f64) -> f64 {
    let max = (scroll_width - client_width).max(0.0);
    if forward {
        if current >= max - 4.0 { 0.0 } else { (current + step).min(max) }
    } else if current <= 4.0 {
        max
    } else {
        (current - step).max(0.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Left,
    Right,
}

/// The control at one edge of a row: a tall area over the margin (and the cards that peek into it), invisible until the row is hovered; hovering the area darkens and blurs the peeking cards,
/// and leaving it lets them clear again slowly. In its middle a round pill with the arrow. `inset` is the top and bottom offset in Tailwind classes (to stop at the cards).
#[component]
pub fn EdgeButton(
    edge: Edge,
    #[prop(into)] visible: Signal<bool>,
    on_click: Callback<()>,
    #[prop(into)] label: String,
    #[prop(default = "top-2 bottom-6")] inset: &'static str,
) -> impl IntoView {
    let side = if edge == Edge::Left { "left-0" } else { "right-0" };
    let icon = if edge == Edge::Left { "ph-bold ph-caret-left" } else { "ph-bold ph-caret-right" };
    view! {
        <button
            type="button"
            class=move || format!(
                "edge-btn hidden md:flex absolute {inset} {side} z-30 items-center justify-center cursor-pointer select-none border-none outline-none opacity-0 group-hover/row:opacity-100 {}",
                if visible.get() { "" } else { "!pointer-events-none !opacity-0" }
            )
            style="width: var(--app-x, 56px);"
            on:click=move |e: web_sys::MouseEvent| { e.stop_propagation(); on_click.run(()); }
            aria-label=label
        >
            <span class="edge-pill"><i class=icon></i></span>
        </button>
    }
}
