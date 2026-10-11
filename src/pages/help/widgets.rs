use super::search::{self, Hit};
use super::{art, encode, tr, tr_with};
use fluent_bundle::FluentValue;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;

/// Text with the matching search words in bold.
pub fn highlighted(text: &str, words: &[String]) -> impl IntoView + use<> {
    search::highlight(text, words)
        .into_iter()
        .map(|(piece, hit)| if hit { view! { <b class="font-bold text-paper-ink">{piece}</b> }.into_any() } else { view! { {piece} }.into_any() })
        .collect_view()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    /// Gradient frame and a magnifier, used in the home hero.
    Hero,
    /// Plain frame and a magnifier, used on the results page.
    Plain,
    /// Plain frame and an arrow button on the right, used on the contact page.
    Arrow,
}

/// Search field with suggestions while typing. Enter opens the results page, or the highlighted suggestion.
#[component]
pub fn SearchBox(#[prop(into)] initial: String, placeholder: &'static str, #[prop(default = Variant::Plain)] variant: Variant) -> impl IntoView {
    let navigate = use_navigate();
    let (value, set_value) = signal(initial);
    let (open, set_open) = signal(false);
    let (active, set_active) = signal(None::<usize>);
    let root = NodeRef::<leptos::html::Div>::new();

    let suggestions = Memo::new(move |_| -> Vec<Hit> {
        let q = value.get();
        if q.trim().is_empty() { vec![] } else { search::search(&q).into_iter().take(5).collect() }
    });
    let results_url = move || format!("/help/search?q={}", encode(value.get_untracked().trim()));

    let nav_submit = navigate.clone();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if !value.get_untracked().trim().is_empty() {
            set_open.set(false);
            nav_submit(&results_url(), Default::default());
        }
    };
    let nav_key = navigate.clone();
    let on_key = move |ev: leptos::ev::KeyboardEvent| {
        let n = suggestions.get_untracked().len();
        match ev.key().as_str() {
            "ArrowDown" if n > 0 => {
                ev.prevent_default();
                set_open.set(true);
                set_active.update(|a| *a = Some(a.map_or(0, |i| (i + 1) % n)));
            }
            "ArrowUp" if n > 0 => {
                ev.prevent_default();
                set_active.update(|a| *a = Some(a.map_or(n - 1, |i| (i + n - 1) % n)));
            }
            "Escape" => set_open.set(false),
            "Enter" => {
                if let Some(hit) = active.get_untracked().and_then(|i| suggestions.get_untracked().get(i).cloned()) {
                    ev.prevent_default();
                    set_open.set(false);
                    nav_key(&format!("/help/article/{}", hit.slug), Default::default());
                }
            }
            _ => {}
        }
    };
    let on_focusout = move |ev: leptos::ev::FocusEvent| {
        let inside = ev.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok()).zip(root.get_untracked()).is_some_and(|(n, r)| r.contains(Some(&n)));
        if !inside {
            set_open.set(false);
        }
    };

    let frame = match variant {
        Variant::Hero => "rounded-field p-0.5 bg-linear-to-r from-accent via-brand-glow to-brand shadow-raise",
        Variant::Plain => "rounded-field border border-paper-edge focus-within:border-paper-ink",
        Variant::Arrow => "rounded-control border border-paper-line focus-within:border-paper-ink",
    };
    let arrow = variant == Variant::Arrow;
    view! {
        <div node_ref=root class="relative w-full" on:focusout=on_focusout>
            <form role="search" on:submit=on_submit class=frame>
                <div class="flex items-center rounded-field bg-paper">
                    {(!arrow).then(|| view! { <i class="ph-bold ph-magnifying-glass ps-4 text-title text-paper-ink" aria-hidden="true"></i> })}
                    <input
                        type="search"
                        autocomplete="off"
                        enterkeyhint="search"
                        aria-label=move || tr("help-ui-search-label")
                        placeholder=move || tr(placeholder)
                        prop:value=value
                        on:input=move |ev| {
                            set_value.set(event_target_value(&ev));
                            set_active.set(None);
                            set_open.set(true);
                        }
                        on:focus=move |_| set_open.set(true)
                        on:keydown=on_key
                        class="h-12 w-full min-w-0 bg-transparent px-3 text-body-md text-paper-ink placeholder:text-paper-mute focus:outline-none"
                    />
                    {arrow.then(|| view! {
                        <button type="submit" aria-label=move || tr("help-ui-search-label") class="flex h-12 w-12 shrink-0 items-center justify-center text-title text-paper-mute hover:text-paper-ink">
                            <i class="ph-bold ph-arrow-right" aria-hidden="true"></i>
                        </button>
                    })}
                </div>
            </form>
            {move || {
                let list = suggestions.get();
                (open.get() && !list.is_empty()).then(|| {
                    let words = search::tokens(&value.get());
                    view! {
                        <ul
                            role="listbox"
                            aria-label=move || tr("help-ui-suggestions")
                            on:mousedown=|ev| ev.prevent_default()
                            class="absolute inset-x-0 top-full z-(--z-dialog) mt-2 overflow-hidden rounded-field border border-paper-line bg-paper text-left shadow-popover"
                        >
                            {list
                                .into_iter()
                                .enumerate()
                                .map(|(i, hit)| {
                                    let title = art(hit.slug, "title");
                                    view! {
                                        <li role="option" aria-selected=move || (active.get() == Some(i)).to_string()>
                                            <a
                                                href=format!("/help/article/{}", hit.slug)
                                                class="flex items-center gap-3 px-4 py-3 text-body-sm text-paper-ink hover:bg-paper-soft"
                                                class=("bg-paper-soft", move || active.get() == Some(i))
                                            >
                                                <i class="ph-bold ph-article text-paper-mute"></i>
                                                <span>{highlighted(&title, &words)}</span>
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()}
                            <li>
                                <a href=results_url() class="flex items-center gap-3 border-t border-paper-line px-4 py-3 text-body-sm font-semibold text-paper-ink hover:bg-paper-soft">
                                    <i class="ph-bold ph-magnifying-glass text-paper-mute"></i>
                                    {tr_with("help-ui-results-count-link", &[("query", FluentValue::from(value.get().trim().to_string()))])}
                                </a>
                            </li>
                        </ul>
                    }
                })
            }}
        </div>
    }
}
