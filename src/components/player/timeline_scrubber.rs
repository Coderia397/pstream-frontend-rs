use leptos::prelude::*;
use web_sys::MouseEvent;

fn format_time(seconds: f64) -> String {
    if seconds.is_nan() || seconds < 0.0 {
        return "00:00".to_string();
    }
    let total_secs = seconds.floor() as u64;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}

#[component]
pub fn TimelineScrubber(
    current_time: ReadSignal<f64>,
    duration: ReadSignal<f64>,
    buffered_fraction: ReadSignal<f64>,
    on_seek: Callback<f64>,
) -> impl IntoView {
    let scrubber_ref = NodeRef::<leptos::html::Div>::new();
    let (is_hovering, set_is_hovering) = signal(false);
    let (hover_pct, set_hover_pct) = signal(0.0);
    let (hover_time_str, set_hover_time_str) = signal("00:00".to_string());
    let (is_dragging, set_is_dragging) = signal(false);

    let progress_pct = move || {
        let d = duration.get();
        if d <= 0.0 {
            0.0
        } else {
            ((current_time.get() / d) * 100.0).clamp(0.0, 100.0)
        }
    };

    let buffer_pct = move || {
        (buffered_fraction.get() * 100.0).clamp(0.0, 100.0)
    };

    let calculate_seek_time = move |e: &MouseEvent| -> Option<f64> {
        let el = scrubber_ref.get()?;
        let rect = el.get_bounding_client_rect();
        let width = rect.width();
        if width <= 0.0 {
            return None;
        }
        let client_x = e.client_x() as f64;
        let left = rect.left();
        let fraction = ((client_x - left) / width).clamp(0.0, 1.0);
        let d = duration.get();
        Some(fraction * d)
    };

    let on_mouse_move = move |e: MouseEvent| {
        if let Some(el) = scrubber_ref.get() {
            let rect = el.get_bounding_client_rect();
            let width = rect.width();
            if width > 0.0 {
                let fraction = ((e.client_x() as f64 - rect.left()) / width).clamp(0.0, 1.0);
                let d = duration.get();
                set_hover_pct.set(fraction * 100.0);
                set_hover_time_str.set(format_time(fraction * d));
                set_is_hovering.set(true);

                if is_dragging.get() {
                    on_seek.run(fraction * d);
                }
            }
        }
    };

    let on_mouse_leave = move |_| {
        set_is_hovering.set(false);
        set_is_dragging.set(false);
    };

    let on_mouse_down = move |e: MouseEvent| {
        set_is_dragging.set(true);
        if let Some(seek_to) = calculate_seek_time(&e) {
            on_seek.run(seek_to);
        }
    };

    let on_mouse_up = move |_| {
        set_is_dragging.set(false);
    };

    view! {
        <div
            node_ref=scrubber_ref
            on:mousemove=on_mouse_move
            on:mouseleave=on_mouse_leave
            on:mousedown=on_mouse_down
            on:mouseup=on_mouse_up
            class="relative w-full h-5 flex items-center cursor-pointer group select-none py-1"
        >
            // Background Track
            <div class="relative w-full h-1 group-hover:h-2 bg-white/25 rounded-full transition-all duration-150 overflow-hidden pointer-events-none">
                // Buffered Range
                <div
                    class="absolute top-0 bottom-0 left-0 bg-white/40 rounded-full transition-all duration-300"
                    style=move || format!("width: {}%;", buffer_pct())
                />
                // Played Progress Bar (Netflix Red)
                <div
                    class="absolute top-0 bottom-0 left-0 bg-[#e50914] rounded-full transition-all duration-75"
                    style=move || format!("width: {}%;", progress_pct())
                />
            </div>

            // Scrubber Knob (Thumb)
            <div
                class="absolute top-1/2 -translate-y-1/2 w-4 h-4 bg-[#e50914] rounded-full shadow-lg transition-transform duration-150 scale-0 group-hover:scale-100 pointer-events-none -ml-2"
                style=move || format!("left: {}%;", progress_pct())
            />

            // Hover Timestamp Tooltip
            {move || {
                if is_hovering.get() && duration.get() > 0.0 {
                    view! {
                        <div
                            class="absolute bottom-6 px-2 py-1 bg-[#181818]/90 backdrop-blur-md text-white text-xs font-semibold rounded shadow-lg border border-white/10 pointer-events-none -translate-x-1/2"
                            style=move || format!("left: {}%;", hover_pct.get())
                        >
                            {hover_time_str.get()}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
