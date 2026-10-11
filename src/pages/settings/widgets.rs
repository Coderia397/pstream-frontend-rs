//! Building blocks the settings screens share: grouped cards, link rows, switches, choice lists, confirm dialog, messages.
use crate::i18n::t;
use leptos::prelude::*;

/// Row classes shared by every row of a card: the divider sits inside the card edge and the last row has none.
const ROW: &str = "flex min-h-settings-row w-full items-center border-b border-set-line px-1 py-3 text-start last:border-b-0 hover:bg-ink/5 focus-visible:bg-ink/5 focus-visible:outline-none";
const CARD: &str = "rounded-card border border-set-edge bg-set-card px-4";

#[component]
pub fn Section(#[prop(optional, into)] title: Option<String>, children: Children) -> impl IntoView {
    view! {
        <section class="mb-8">
            {title.map(|t| view! { <h2 class="mb-4 text-title-sm leading-row text-ink/85">{t}</h2> })}
            <div class=CARD>{children()}</div>
        </section>
    }
}

fn row_body(icon: &'static str, title: String, hint: Option<String>, value: Option<Signal<String>>, chevron: bool) -> impl IntoView {
    view! {
        <i class=format!("ph-bold ph-{icon} me-3 text-2xl text-ink/70") aria-hidden="true"></i>
        <span class="min-w-0 flex-1">
            <span class="block text-title-sm font-semibold leading-row text-ink">{title}</span>
            {hint.map(|h| view! { <span class="block pt-1 text-body-sm leading-normal text-ink/70">{h}</span> })}
        </span>
        {value.map(|v| view! { <span class="ms-3 max-w-2/5 truncate text-body-sm text-ink/70">{move || v.get()}</span> })}
        {chevron.then(|| view! { <i class="ph-bold ph-caret-right ms-3 text-2xl text-ink/70 rtl:-scale-x-100" aria-hidden="true"></i> })}
    }
}

/// A row that opens another screen.
#[component]
pub fn LinkRow(
    icon: &'static str,
    #[prop(into)] title: String,
    #[prop(into)] href: String,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional)] value: Option<Signal<String>>,
) -> impl IntoView {
    view! {
        <a href=href class=ROW>
            {row_body(icon, title, hint, value, true)}
        </a>
    }
}

/// A row that does something here (opens a dialog, runs a test).
#[component]
pub fn ActionRow(
    icon: &'static str,
    #[prop(into)] title: String,
    on_click: Callback<()>,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional)] value: Option<Signal<String>>,
    #[prop(optional)] chevron: bool,
) -> impl IntoView {
    view! {
        <button type="button" on:click=move |_| on_click.run(()) class=ROW>
            {row_body(icon, title, hint, value, chevron)}
        </button>
    }
}

/// A row with an on/off switch.
#[component]
pub fn SwitchRow(
    icon: &'static str,
    #[prop(into)] title: String,
    #[prop(optional, into)] hint: Option<String>,
    checked: Signal<bool>,
    on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <div class=ROW>
            {row_body(icon, title.clone(), hint, None, false)}
            <button
                type="button"
                role="switch"
                aria-checked=move || checked.get().to_string()
                aria-label=title
                on:click=move |_| on_change.run(!checked.get_untracked())
                class="relative ms-3 inline-flex h-6 w-11 shrink-0 items-center rounded-full transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brand"
                class=("bg-brand", move || checked.get())
                class=("bg-ink/25", move || !checked.get())
            >
                <span
                    class="inline-block h-4 w-4 rounded-full bg-ink transition-transform"
                    class=("translate-x-6", move || checked.get())
                    class=("rtl:-translate-x-6", move || checked.get())
                    class=("translate-x-1", move || !checked.get())
                    class=("rtl:-translate-x-1", move || !checked.get())
                ></span>
            </button>
        </div>
    }
}

/// Radio list: one choice picked, each row a full width button.
#[component]
pub fn ChoiceList(options: Vec<(String, String)>, selected: Signal<String>, on_pick: Callback<String>) -> impl IntoView {
    view! {
        <div role="radiogroup" class=CARD>
            {options
                .into_iter()
                .map(|(value, label)| {
                    let v = value.clone();
                    let v2 = value.clone();
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || (selected.get() == v).to_string()
                            on:click=move |_| on_pick.run(v2.clone())
                            class=ROW
                        >
                            <span class="flex-1 text-title-sm leading-row text-ink">{label}</span>
                            <i class="ph-bold ph-check text-xl text-brand" class=("invisible", { let s = value.clone(); move || selected.get() != s }) aria-hidden="true"></i>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Same list with several picks allowed.
#[component]
pub fn CheckList(options: Vec<(String, String)>, selected: Signal<Vec<String>>, on_toggle: Callback<String>) -> impl IntoView {
    view! {
        <div class=CARD>
            {options
                .into_iter()
                .map(|(value, label)| {
                    let v = value.clone();
                    let v2 = value.clone();
                    view! {
                        <button
                            type="button"
                            role="checkbox"
                            aria-checked=move || selected.with(|s| s.contains(&v)).to_string()
                            on:click=move |_| on_toggle.run(v2.clone())
                            class=ROW
                        >
                            <span class="flex-1 text-title-sm leading-row text-ink">{label}</span>
                            <i class="ph-bold ph-check text-xl text-brand" class=("invisible", { let s = value.clone(); move || !selected.with(|l| l.contains(&s)) }) aria-hidden="true"></i>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Labelled text field.
#[component]
pub fn Field(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(default = "text")] kind: &'static str,
    #[prop(default = "off")] autocomplete: &'static str,
    #[prop(optional)] maxlength: Option<u32>,
    #[prop(optional)] inputmode: Option<&'static str>,
) -> impl IntoView {
    view! {
        <label class="mb-4 block">
            <span class="mb-1.5 block text-label font-semibold text-ink/60">{label}</span>
            <input
                type=kind
                autocomplete=autocomplete
                maxlength=maxlength
                inputmode=inputmode
                prop:value=move || value.get()
                on:input=move |ev| value.set(event_target_value(&ev))
                class="w-full rounded-control border border-ink/20 bg-ink/10 px-4 py-3 text-body-md text-ink focus:border-ink focus:outline-none"
            />
        </label>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Error,
    Done,
}

/// A line under a form saying what went wrong or that it worked.
#[component]
pub fn Message(text: RwSignal<Option<(Tone, String)>>) -> impl IntoView {
    move || {
        text.get().map(|(tone, msg)| {
            let look = if tone == Tone::Error { "border-danger/50 bg-danger/10 text-danger" } else { "border-ink/20 bg-ink/10 text-ink" };
            view! { <p role="status" class=format!("mb-4 rounded-control border px-4 py-3 text-body-sm {look}")>{msg}</p> }
        })
    }
}

/// Buttons at the bottom of a form: the main one and a plain cancel.
#[component]
pub fn FormButtons(#[prop(into)] save_label: String, on_save: Callback<()>, #[prop(into)] cancel_href: String, #[prop(optional, into)] busy: Option<Signal<bool>>) -> impl IntoView {
    view! {
        <div class="mt-2 flex flex-wrap items-center gap-3">
            <button
                type="button"
                on:click=move |_| on_save.run(())
                disabled=move || busy.map(|b| b.get()).unwrap_or(false)
                class="btn-ink rounded-control px-8 py-2.5 disabled:opacity-50"
            >
                {save_label}
            </button>
            <a href=cancel_href class="rounded-control px-6 py-2.5 text-body-md font-bold text-ink/80 hover:bg-ink/10">{t("set-cancel")}</a>
        </div>
    }
}

/// Dialog that asks before something that cannot be undone.
#[component]
pub fn ConfirmDialog(
    open: RwSignal<bool>,
    #[prop(into)] title: String,
    #[prop(into)] body: String,
    #[prop(into)] confirm_label: String,
    on_confirm: Callback<()>,
    #[prop(optional)] danger: bool,
) -> impl IntoView {
    let confirm_look = if danger { "bg-danger text-ink hover:opacity-90" } else { "bg-ink text-void hover:bg-ink/90" };
    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 z-(--z-dialog) flex items-end justify-center bg-void/70 p-4 sm:items-center" on:click=move |_| open.set(false)>
                <div
                    role="alertdialog"
                    aria-modal="true"
                    aria-label=title.clone()
                    class="w-full max-w-md rounded-card border border-ink/10 bg-page p-6 shadow-popover"
                    on:click=|ev| ev.stop_propagation()
                    on:keydown=move |ev| if ev.key() == "Escape" { open.set(false) }
                >
                    <h2 class="mb-2 text-heading font-bold text-ink">{title.clone()}</h2>
                    <p class="mb-6 text-body-md text-ink/70">{body.clone()}</p>
                    <div class="flex justify-end gap-3">
                        <button type="button" on:click=move |_| open.set(false) class="rounded-control px-5 py-2.5 text-body-md font-bold text-ink hover:bg-ink/10">{t("set-cancel")}</button>
                        <button type="button" on:click=move |_| { open.set(false); on_confirm.run(()); } class=format!("rounded-control px-5 py-2.5 text-body-md font-bold {confirm_look}")>{confirm_label.clone()}</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Outline button at the bottom of a page, for the actions that remove things.
#[component]
pub fn OutlineButton(#[prop(into)] label: String, on_click: Callback<()>, #[prop(optional)] danger: bool) -> impl IntoView {
    let look = if danger { "border-danger/60 text-danger hover:bg-danger/10" } else { "border-ink/40 text-ink hover:bg-ink/10" };
    view! {
        <button type="button" on:click=move |_| on_click.run(()) class=format!("w-full rounded-control border px-6 py-3 text-body-md font-bold {look}")>{label}</button>
    }
}
