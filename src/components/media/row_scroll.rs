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
/// and leaving it lets them clear again slowly. In its middle a round pill with the arrow. `inset` is the top and bottom offset in Tailwind classes; the area reaches past the cards above and below
/// and a little way over the card beside it (14px), so the pointer finds the button without crossing a card (which would open its hover card).
#[component]
pub fn EdgeButton(
    edge: Edge,
    #[prop(into)] visible: Signal<bool>,
    on_click: Callback<()>,
    #[prop(into)] label: String,
    #[prop(default = "-top-3 -bottom-8")] inset: &'static str,
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
            style="width: calc(var(--app-x, 56px) + 14px);"
            on:click=move |e: web_sys::MouseEvent| { e.stop_propagation(); on_click.run(()); }
            aria-label=label
        >
            <span class="edge-pill"><i class=icon></i></span>
        </button>
    }
}

// ── The endless row ────────────────────────────────────────────────────────────────────────────────
// A row that loops carries a copy of its first cards after its last card (`data-loop-clone` on the first card of the copy, `data-loop-first` on the first real card). Paging forward from the last
// page goes on into the copy, which looks exactly like the start; when the move has ended the row is put back to the start without animation. Paging back from the start first jumps to the
// same view in the copy, then pages back to the last page. So the row always moves in the direction of the button.

use wasm_bindgen::JsCast;

/// (distance from the first card to the copy, scroll position at which the last real card ends flush with the right margin), or None for a row without a copy.
fn loop_geometry(strip: &web_sys::HtmlElement, margin: f64) -> Option<(f64, f64)> {
    let clone: web_sys::HtmlElement = strip.query_selector("[data-loop-clone]").ok()??.dyn_into().ok()?;
    let first: web_sys::HtmlElement = strip.query_selector("[data-loop-first]").ok()??.dyn_into().ok()?;
    let prev: web_sys::HtmlElement = clone.previous_element_sibling()?.dyn_into().ok()?;
    let l = (clone.offset_left() - first.offset_left()) as f64;
    let end_of_last = (prev.offset_left() + prev.offset_width()) as f64;
    Some((l, (end_of_last - strip.client_width() as f64 + margin).max(0.0)))
}

fn go(strip: &web_sys::HtmlElement, x: f64, instant: bool) {
    let opts = web_sys::ScrollToOptions::new();
    opts.set_left(x.max(0.0));
    opts.set_behavior(if instant { web_sys::ScrollBehavior::Instant } else { web_sys::ScrollBehavior::Smooth });
    strip.scroll_to_with_scroll_to_options(&opts);
}

/// One page turn of an endless row. Returns false when the row has no copy (the caller then uses `page_target`).
pub fn turn_page(strip: &web_sys::HtmlElement, forward: bool, step: f64, margin: f64) -> bool {
    let Some((l, e)) = loop_geometry(strip, margin) else { return false };
    let mut cur = strip.scroll_left() as f64;
    if cur >= l - 2.0 {
        // inside the copy (dragged there): the same view at the start
        cur -= l;
        go(strip, cur, true);
    }
    if forward {
        go(strip, if cur < e - 4.0 { (cur + step).min(e) } else { l }, false);
    } else if cur <= 4.0 {
        go(strip, l, true);
        go(strip, e, false);
    } else {
        go(strip, (cur - step).max(0.0), false);
    }
    let strip = strip.clone();
    leptos::leptos_dom::helpers::set_timeout(
        move || {
            let c = strip.scroll_left() as f64;
            if c >= l - 2.0 { go(&strip, c - l, true); }
        },
        std::time::Duration::from_millis(900),
    );
    true
}
