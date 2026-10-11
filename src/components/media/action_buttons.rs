use leptos::prelude::*;
use crate::components::media::movie_card_rating::{RatingIcon, MovieRating};
use crate::components::media::tooltip_wrapper::TooltipWrapper;

// ── Pill Play Button ──────────────────────────────────────────────────────────
// Used in Hero (pill: rounded-full) and InfoModal (shape="rounded-control": rounded-field).
#[component]
pub fn PlayPillButton(
    #[prop(optional, into)] href: Option<Signal<String>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(default = Signal::derive(|| false), into)] is_resume: Signal<bool>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(default = "pill".to_string(), into)] shape: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (pad_classes, text_classes, icon_classes) = match size.as_str() {
        "sm" => ("px-3.5 py-1", "text-xs", "text-sm"),
        "lg" => ("px-5 h-12", "text-title-sm leading-[18px]", "text-heading"),
        _ => ("px-4 sm:px-5 py-1.5 sm:py-2", "text-label sm:text-sm", "text-base sm:text-lg"),
    };

    let radius_classes = match shape.as_str() {
        "rounded-control" | "rect" => match size.as_str() {
            "sm" => "rounded-control",
            "lg" => "rounded-card",
            _ => "rounded-field",
        },
        _ => "rounded-full",
    };

    let extra_class = class.unwrap_or_default();
    let is_link = href.is_some();
    let base_classes = format!(
        "bg-ink text-void font-medium flex items-center justify-center gap-2 hover:bg-ink/70 active:scale-95 transition-all cursor-pointer select-none {} {} {} {} {}",
        pad_classes, text_classes, radius_classes, extra_class,
        if is_link { "no-underline" } else { "" }
    );

    view! {
        {if let Some(url_sig) = href {
            view! {
                <a
                    href=move || url_sig.get()
                    class=base_classes
                    title=move || if is_resume.get() { "Resume" } else { "Play" }
                >
                    <i class=format!("ph-fill ph-play {}", icon_classes)></i>
                    <span>{move || if is_resume.get() { "Resume" } else { "Play" }}</span>
                </a>
            }.into_any()
        } else {
            let click_cb = on_click.unwrap_or_else(|| Callback::new(|_| {}));
            view! {
                <button
                    type="button"
                    on:click=move |_| click_cb.run(())
                    class=base_classes
                    title=move || if is_resume.get() { "Resume" } else { "Play" }
                >
                    <i class=format!("ph-fill ph-play {}", icon_classes)></i>
                    <span>{move || if is_resume.get() { "Resume" } else { "Play" }}</span>
                </button>
            }.into_any()
        }}
    }
}

// ── Pill More Info Button ─────────────────────────────────────────────────────
// Used in Hero billboard. Stadium pill (rounded-full) with authentic Netflix #6d6d6e semi-transparency.
#[component]
pub fn MoreInfoPillButton(
    on_click: Callback<()>,
    #[prop(default = "More Info".to_string(), into)] label: String,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(default = "pill".to_string(), into)] shape: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (pad_classes, text_classes, icon_classes) = match size.as_str() {
        "sm" => ("px-3.5 py-1", "text-xs", "text-sm"),
        "lg" => ("px-5 h-12", "text-title-sm leading-[18px]", "text-heading"),
        _ => ("px-4 sm:px-5 py-1.5 sm:py-2", "text-label sm:text-sm", "text-base sm:text-lg"),
    };

    let (bg_classes, radius_classes) = match shape.as_str() {
        "rounded-control" | "rect" => (
            "bg-ink/20 hover:bg-ink/30 backdrop-blur-md",
            match size.as_str() {
                "sm" => "rounded-control",
                "lg" => "rounded-card",
                _ => "rounded-field",
            },
        ),
        _ => (
            "bg-mute/40 hover:bg-mute/30",
            "rounded-full",
        ),
    };

    let extra_class = class.unwrap_or_default();
    let label_title = label.clone();
    let base_classes = format!(
        "{} text-ink font-medium flex items-center justify-center gap-2 active:scale-95 transition-all cursor-pointer select-none {} {} {} {} {}",
        bg_classes, pad_classes, text_classes, radius_classes, extra_class, ""
    );

    view! {
        <button
            type="button"
            on:click=move |_| on_click.run(())
            class=base_classes
            title=label_title
        >
            <i class=format!("ph ph-info {}", icon_classes)></i>
            <span>{label}</span>
        </button>
    }
}

// ── Circular Play Button ──────────────────────────────────────────────────────
// Used in Movie Card hover popups and recommendation overlays.
#[component]
pub fn PlayCircleButton(
    on_click: Callback<()>,
    #[prop(default = Signal::derive(|| false), into)] is_resume: Signal<bool>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, icon_classes, _) = circular_button_specs(&size);

    let extra_class = class.unwrap_or_default();

    view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_click.run(());
            }
            class=format!(
                "rounded-full bg-ink text-void flex items-center justify-center hover:bg-ink/80 active:scale-95 transition-all shadow-md cursor-pointer select-none shrink-0 {} {}",
                dim_classes, extra_class
            )
            title=move || if is_resume.get() { "Resume" } else { "Play" }
        >
            <i class=format!("ph-fill ph-play translate-x-0.5 {}", icon_classes)></i>
        </button>
    }
}

// ── Shared Circular Button Styling ───────────────────────────────────────────
// Guarantees that MyList, Rating, Mute/Replay, Close, and Caret buttons share 100%
// identical geometry, background, border, hover states, and animations across all surfaces.
fn circular_button_specs(size: &str) -> (&'static str, &'static str, u32) {
    match size {
        "xs" | "sm" => ("w-7 h-7", "text-xs", 12),
        "lg" => ("w-11 h-11 md:w-12 md:h-12", "text-xl md:text-2xl", 22),
        _ => ("w-[38px] h-[38px]", "text-title-sm", 18),
    }
}

fn circular_button_classes(dim_classes: &str, extra_class: &str) -> String {
    format!(
        "rounded-full border-[1.8px] border-ink/50 bg-menu/60 hover:bg-ink/15 hover:border-ink text-ink flex items-center justify-center transition-all duration-(--motion-fast) active:scale-95 cursor-pointer select-none shrink-0 {} {}",
        dim_classes, extra_class
    )
}

// ── My List Button ────────────────────────────────────────────────────────────
// Used across InfoModal, MovieCardPopup, InfoModalRecommendations.
#[component]
pub fn MyListButton(
    #[prop(into)] is_in_list: Signal<bool>,
    on_toggle: Callback<()>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(default = false)] show_tooltip: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, icon_classes, _) = circular_button_specs(&size);
    let extra_class = class.unwrap_or_default();
    let btn_classes = circular_button_classes(dim_classes, &extra_class);

    let title_text = move || if is_in_list.get() { "Remove from My List" } else { "Add to My List" };

    let btn = view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_toggle.run(());
            }
            class=btn_classes.clone()
            title=title_text
        >
            {move || if is_in_list.get() {
                view! { <i class=format!("ph-bold ph-check text-green-400 {}", icon_classes)></i> }.into_any()
            } else {
                view! { <i class=format!("ph-bold ph-plus {}", icon_classes)></i> }.into_any()
            }}
        </button>
    };

    view! {
        {if show_tooltip {
            view! {
                <TooltipWrapper label=crate::i18n::t("action-buttons-add-to-my-list").to_string()>
                    {btn}
                </TooltipWrapper>
            }.into_any()
        } else {
            btn.into_any()
        }}
    }
}

// ── Rating Button ("I like this") ─────────────────────────────────────────────
// Used in InfoModal and MovieCardPopup.
#[component]
pub fn RatingButton(
    #[prop(into)] rating: Signal<Option<MovieRating>>,
    on_rate: Callback<()>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(default = false)] show_tooltip: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, _, icon_pixel_size) = circular_button_specs(&size);
    let extra_class = class.unwrap_or_default();
    let btn_classes = circular_button_classes(dim_classes, &extra_class);

    let btn = view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_rate.run(());
            }
            class=btn_classes.clone()
            title=crate::i18n::t("action-buttons-rate")
        >
            <RatingIcon rating=rating.get() size=icon_pixel_size />
        </button>
    };

    view! {
        {if show_tooltip {
            view! {
                <TooltipWrapper label=crate::i18n::t("action-buttons-i-like-this").to_string()>
                    {btn}
                </TooltipWrapper>
            }.into_any()
        } else {
            btn.into_any()
        }}
    }
}

// ── Mute / Replay Button ──────────────────────────────────────────────────────
// "mute/add mute that doubles as a reload button"
// Used in Hero, InfoModal, MovieCardPopup.
#[component]
pub fn MuteReplayButton(
    #[prop(into)] is_muted: Signal<bool>,
    #[prop(into)] is_ended: Signal<bool>,
    on_toggle: Callback<()>,
    #[prop(optional, into)] on_force_replay: Option<Callback<()>>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, icon_classes, _) = circular_button_specs(&size);
    let extra_class = class.unwrap_or_default();
    let btn_classes = circular_button_classes(dim_classes, &extra_class);

    let title_text = move || {
        if is_ended.get() {
            crate::i18n::t("action-buttons-replay-trailer")
        } else if is_muted.get() {
            crate::i18n::t("action-buttons-unmute-trailer")
        } else {
            crate::i18n::t("action-buttons-mute-trailer")
        }
    };

    view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_toggle.run(());
            }
            on:dblclick=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                if let Some(cb) = on_force_replay {
                    cb.run(());
                }
            }
            class=btn_classes
            title=title_text
            aria-label=title_text
        >
            {move || {
                if is_ended.get() {
                    view! { <i class=format!("ph ph-arrow-counter-clockwise text-ink {}", icon_classes)></i> }.into_any()
                } else if is_muted.get() {
                    view! { <i class=format!("ph ph-speaker-slash text-ink {}", icon_classes)></i> }.into_any()
                } else {
                    view! { <i class=format!("ph ph-speaker-high text-ink {}", icon_classes)></i> }.into_any()
                }
            }}
        </button>
    }
}

// ── Close Button ──────────────────────────────────────────────────────────────
// Modal ✕ button matching Like, Add to My List, and Mute/Replay in size and color.
#[component]
pub fn CloseButton(
    on_close: Callback<()>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, icon_classes, _) = circular_button_specs(&size);
    let extra_class = class.unwrap_or_default();
    let btn_classes = circular_button_classes(dim_classes, &extra_class);

    view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_close.run(());
            }
            class=btn_classes
            title=crate::i18n::t("action-buttons-close")
            aria-label=crate::i18n::t("action-buttons-close")
        >
            <i class=format!("ph-bold ph-x {}", icon_classes)></i>
        </button>
    }
}

// ── More Info Caret Button ────────────────────────────────────────────────────
// Caret down circular button on movie card popups.
#[component]
pub fn MoreInfoCaretButton(
    on_open: Callback<()>,
    #[prop(default = "md".to_string(), into)] size: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let (dim_classes, icon_classes, _) = circular_button_specs(&size);
    let extra_class = class.unwrap_or_default();
    let btn_classes = circular_button_classes(dim_classes, &extra_class);

    view! {
        <button
            type="button"
            on:click=move |e: leptos::ev::MouseEvent| {
                e.stop_propagation();
                on_open.run(());
            }
            class=btn_classes
            title=crate::i18n::t("action-buttons-more-info")
            aria-label=crate::i18n::t("action-buttons-more-info")
        >
            <i class=format!("ph-bold ph-caret-down {}", icon_classes)></i>
        </button>
    }
}
