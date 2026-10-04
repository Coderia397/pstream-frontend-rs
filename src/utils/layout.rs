//! Layout tokens that live in CSS (`style/input.css`) and are also needed by Rust code that positions overlays.

/// Page side margin in px, read from the `--app-x` custom property of the root element.
pub fn app_x() -> f64 {
    css_px("--app-x").unwrap_or(48.0)
}

fn css_px(name: &str) -> Option<f64> {
    let window = web_sys::window()?;
    let root = window.document()?.document_element()?;
    let style = window.get_computed_style(&root).ok()??;
    let raw = style.get_property_value(name).ok()?;
    raw.trim().trim_end_matches("px").trim().parse().ok()
}
