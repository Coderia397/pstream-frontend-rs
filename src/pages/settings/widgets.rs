//! Building blocks the settings screens share: grouped cards, link rows, switches, choice lists, confirm dialog, messages.
use crate::i18n::t;
use leptos::prelude::*;

/// Row classes shared by every row of a card: the divider sits inside the card edge and the last row has none.
const ROW: &str = "set-row flex min-h-settings-row w-full items-center border-b border-set-line px-1 py-3 text-start last:border-b-0 hover:bg-ink/5 focus-visible:bg-ink/5 focus-visible:outline-none";
const CARD: &str = "set-card rounded-card border border-set-edge bg-set-card px-4";

#[component]
pub fn Section(#[prop(optional, into)] title: Option<String>, children: Children) -> impl IntoView {
    view! {
        <section class="mb-8">
            {title.map(|t| view! { <h2 class="set-section-title mb-3 text-body-md leading-row text-ink/85 lg:mb-4 lg:text-title-sm">{t}</h2> })}
            <div class=CARD>{children()}</div>
        </section>
    }
}

fn row_body(icon: &'static str, title: String, hint: Option<String>, value: Option<Signal<String>>, chevron: bool) -> impl IntoView {
    view! {
        <i class=format!("ph-bold ph-{icon} me-3 text-2xl text-ink/70") aria-hidden="true"></i>
        <span class="min-w-0 flex-1">
            <span class="block text-body-md font-semibold leading-row text-ink lg:text-title-sm">{title}</span>
            {hint.clone().map(|h| view! { <span class="block pt-0.5 text-label leading-normal text-ink/70 lg:pt-1 lg:text-body-sm">{h}</span> })}
            // on a phone the value sits under the title, as in the Android app, unless the row has a hint of its own
            {hint.is_none().then(|| value.map(|v| view! { <span class="block truncate pt-0.5 text-label text-ink/70 lg:hidden">{move || v.get()}</span> }))}
        </span>
        {value.map(|v| view! { <span class="ms-3 hidden max-w-2/5 truncate text-body-sm text-ink/70 lg:block">{move || v.get()}</span> })}
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
pub fn ChoiceList(options: Vec<(String, String)>, selected: Signal<String>, on_pick: Callback<String>, #[prop(default = false)] plain: bool) -> impl IntoView {
    view! {
        <div role="radiogroup" class=if plain { "" } else { CARD }>
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
                            class=if plain { "flex min-h-12 w-full items-center gap-4 py-2 text-start focus-visible:outline-none" } else { ROW }
                        >
                            {plain.then(|| view! {
                                <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full border-2 border-ink/70" class=("border-brand", { let s = value.clone(); move || selected.get() == s })>
                                    <span class="h-3 w-3 rounded-full bg-brand" class=("invisible", { let s = value.clone(); move || selected.get() != s })></span>
                                </span>
                            })}
                            <span class="flex-1 text-body-md leading-row text-ink lg:text-title-sm">{label}</span>
                            {(!plain).then(|| view! { <i class="ph-bold ph-check text-xl text-brand" class=("invisible", { let s = value.clone(); move || selected.get() != s }) aria-hidden="true"></i> })}
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
    /// Show the label only from 1024 px (the Android app has none above the name field).
    #[prop(default = false)] label_wide_only: bool,
    value: RwSignal<String>,
    #[prop(default = "text")] kind: &'static str,
    #[prop(default = "off")] autocomplete: &'static str,
    #[prop(optional)] maxlength: Option<u32>,
    #[prop(optional)] inputmode: Option<&'static str>,
) -> impl IntoView {
    view! {
        <label class="mb-4 block">
            <span class=if label_wide_only { "mb-1.5 hidden text-label font-semibold text-ink/60 lg:block" } else { "mb-1.5 block text-label font-semibold text-ink/60" }>{label}</span>
            <input
                type=kind
                autocomplete=autocomplete
                maxlength=maxlength
                inputmode=inputmode
                prop:value=move || value.get()
                on:input=move |ev| value.set(event_target_value(&ev))
                class="w-full rounded-control border border-set-edge bg-set-card px-4 py-3 text-body-md text-ink focus:border-ink focus:outline-none"
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
        // a phone stacks them full width, like the Android app; wide screens put them side by side
        <div class="mt-2 flex flex-col items-stretch gap-3 lg:flex-row lg:flex-wrap lg:items-center">
            <button
                type="button"
                on:click=move |_| on_save.run(())
                disabled=move || busy.map(|b| b.get()).unwrap_or(false)
                class="btn-ink rounded-control px-8 py-3 disabled:opacity-50 lg:py-2.5"
            >
                {save_label}
            </button>
            <a href=cancel_href class="rounded-control px-6 py-2.5 text-center text-body-md font-bold text-ink/80 hover:bg-ink/10">{t("set-cancel")}</a>
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

/// Password field with a show or hide button.
#[component]
pub fn PasswordField(#[prop(into)] label: String, value: RwSignal<String>, #[prop(default = "current-password")] autocomplete: &'static str) -> impl IntoView {
    let shown = RwSignal::new(false);
    view! {
        <label class="mb-4 block">
            <span class="mb-1.5 block text-label font-semibold text-ink/60">{label}</span>
            <span class="flex items-center rounded-control border border-set-edge bg-set-card focus-within:border-ink">
                <input
                    type=move || if shown.get() { "text" } else { "password" }
                    autocomplete=autocomplete
                    prop:value=move || value.get()
                    on:input=move |ev| value.set(event_target_value(&ev))
                    class="min-w-0 flex-1 bg-transparent px-4 py-3 text-body-md text-ink focus:outline-none"
                />
                <button
                    type="button"
                    aria-label=move || if shown.get() { t("set-password-hide") } else { t("set-password-show") }
                    aria-pressed=move || shown.get().to_string()
                    on:click=move |_| shown.update(|s| *s = !*s)
                    class="flex h-11 w-11 shrink-0 items-center justify-center text-ink/70 hover:text-ink"
                >
                    <i class=move || if shown.get() { "ph-bold ph-eye-slash text-xl" } else { "ph-bold ph-eye text-xl" } aria-hidden="true"></i>
                </button>
            </span>
        </label>
    }
}

/// A row of digit boxes for a PIN or a code. One real input sits over the boxes, so typing, pasting, the keyboard type and
/// one time code autofill all work; the boxes only show what was typed.
#[component]
pub fn DigitBoxes(#[prop(into)] label: String, value: RwSignal<String>, length: usize, #[prop(default = false)] secret: bool, #[prop(default = false)] one_time_code: bool) -> impl IntoView {
    let focused = RwSignal::new(false);
    view! {
        <div class="mb-4">
            <span class="mb-1.5 block text-label font-semibold text-ink/60">{label.clone()}</span>
            <div class="relative inline-block">
                <div class="flex gap-2" aria-hidden="true">
                    {(0..length).map(|i| view! {
                        <span
                            class="flex h-14 w-11 items-center justify-center rounded-control border bg-ink/10 text-title font-bold text-ink"
                            class=("border-ink", move || focused.get() && value.with(|v| v.chars().count()) == i)
                            class=("border-ink/20", move || !(focused.get() && value.with(|v| v.chars().count()) == i))
                        >
                            {move || value.with(|v| v.chars().nth(i).map(|c| if secret { '•' } else { c }).map(String::from).unwrap_or_default())}
                        </span>
                    }).collect_view()}
                </div>
                <input
                    type="text"
                    inputmode="numeric"
                    pattern="[0-9]*"
                    autocomplete=if one_time_code { "one-time-code" } else { "off" }
                    maxlength=length
                    aria-label=label
                    prop:value=move || value.get()
                    on:input=move |ev| value.set(event_target_value(&ev).chars().filter(|c| c.is_ascii_digit()).take(length).collect())
                    on:focus=move |_| focused.set(true)
                    on:blur=move |_| focused.set(false)
                    class="absolute inset-0 h-full w-full cursor-text bg-transparent text-transparent caret-transparent opacity-0"
                />
            </div>
        </div>
    }
}

/// Levels joined by a line, each level allowing everything below it (the age ratings stepper in the Netflix app). `options` run
/// from the lowest to the highest level: (value, label, hint).
#[component]
pub fn Stepper(options: Vec<(String, String, String)>, selected: Signal<String>, on_pick: Callback<String>) -> impl IntoView {
    let values: Vec<String> = options.iter().map(|(v, _, _)| v.clone()).collect();
    let count = options.len();
    view! {
        <div role="radiogroup" class="px-0 py-2 lg:rounded-card lg:border lg:border-set-edge lg:bg-set-card lg:px-4">
            {options
                .into_iter()
                .enumerate()
                .map(|(i, (value, label, hint))| {
                    let values = values.clone();
                    let v = value.clone();
                    let reached = {
                        let values = values.clone();
                        move || values.iter().position(|x| *x == selected.get()).is_some_and(|at| i <= at)
                    };
                    let reached2 = reached.clone();
                    view! {
                        <button
                            type="button"
                            role="radio"
                            aria-checked=move || (selected.get() == v).to_string()
                            on:click=move |_| on_pick.run(value.clone())
                            class="relative flex w-full items-start gap-4 py-3 text-start focus-visible:outline-none"
                        >
                            <span class="relative flex w-7 shrink-0 justify-center">
                                {(i + 1 < count).then(|| view! { <span class="absolute start-1/2 top-7 -bottom-6 w-0.5 -translate-x-1/2 bg-set-line" class=("bg-brand", { let r = reached2.clone(); move || r() }) aria-hidden="true"></span> })}
                                <span
                                    class="relative flex h-7 w-7 items-center justify-center rounded-full border-2"
                                    class=("border-brand", { let r = reached.clone(); move || r() })
                                    class=("bg-brand", { let r = reached.clone(); move || r() })
                                    class=("border-set-edge", { let r = reached.clone(); move || !r() })
                                >
                                    <i class="ph-bold ph-check text-fine text-chrome-ink" class=("invisible", { let r = reached.clone(); move || !r() }) aria-hidden="true"></i>
                                </span>
                            </span>
                            <span class="min-w-0 flex-1">
                                <span class="block text-body-md font-semibold leading-row text-ink lg:text-title-sm">{label}</span>
                                <span class="block text-label text-ink/70 lg:text-body-sm">{hint}</span>
                            </span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Round colour swatches, one selected.
#[component]
pub fn SwatchPicker(#[prop(into)] label: String, colors: Vec<(String, String, String)>, selected: Signal<String>, on_pick: Callback<String>) -> impl IntoView {
    view! {
        <div class="set-row block w-full border-b border-set-line px-1 py-3 last:border-b-0">
            <span class="mb-2 block text-title-sm font-semibold leading-row text-ink">{label}</span>
            <div role="radiogroup" class="flex flex-wrap gap-3">
                {colors
                    .into_iter()
                    .map(|(id, hex, name)| {
                        let hex_none = hex == "transparent";
                        let sel = id.clone();
                        let pick = id.clone();
                        view! {
                            <button
                                type="button"
                                role="radio"
                                aria-label=name
                                aria-checked=move || (selected.get() == sel).to_string()
                                on:click=move |_| on_pick.run(pick.clone())
                                style=format!("background-color:{hex}")
                                class="h-9 w-9 rounded-full border-2 border-set-edge focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-brand"
                                class=("swatch-none", hex_none)
                                class=("ring-2", { let s = id.clone(); move || selected.get() == s })
                                class=("ring-brand", { let s = id.clone(); move || selected.get() == s })
                                class=("ring-offset-2", { let s = id.clone(); move || selected.get() == s })
                            ></button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// A row that opens to show more (a device card).
#[component]
pub fn Expandable(icon: &'static str, #[prop(into)] title: String, #[prop(optional, into)] meta: Option<String>, #[prop(optional, into)] badge: Option<String>, children: ChildrenFn) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <div class="rounded-card border border-set-edge bg-set-card px-4">
            <button type="button" aria-expanded=move || open.get().to_string() on:click=move |_| open.update(|o| *o = !*o) class="flex w-full items-center gap-3 py-4 text-start focus-visible:outline-none">
                <i class=format!("ph-bold ph-{icon} text-2xl text-ink/70") aria-hidden="true"></i>
                <span class="min-w-0 flex-1">
                    {badge.filter(|b| !b.is_empty()).map(|b| view! { <Badge text=b /> })}
                    <span class="block truncate text-title-sm font-semibold leading-row text-ink">{title}</span>
                    {meta.filter(|m| !m.is_empty()).map(|m| view! { <span class="block text-body-sm text-ink/70">{m}</span> })}
                </span>
                <i class="ph-bold ph-caret-down text-2xl text-ink/70 transition-transform" class=("rotate-180", move || open.get()) aria-hidden="true"></i>
            </button>
            <Show when=move || open.get()>
                <div class="border-t border-set-line py-3 text-body-sm text-ink/80">{children()}</div>
            </Show>
        </div>
    }
}

/// Small label: New, Verified, This device.
#[component]
pub fn Badge(#[prop(into)] text: String) -> impl IntoView {
    view! { <span class="mb-1 inline-block rounded-control bg-badge px-1.5 pb-0.5 pt-px text-body-sm font-medium text-paper-ink">{text}</span> }
}

/// A banner that explains something and may carry a button.
#[component]
pub fn Notice(icon: &'static str, #[prop(into)] title: Signal<String>, #[prop(into)] body: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div class="mb-6 rounded-card border border-set-edge bg-set-card p-5">
            <p class="mb-1 flex items-center gap-2 text-title-sm font-semibold leading-row"><i class=format!("ph-bold ph-{icon} text-2xl") aria-hidden="true"></i>{move || title.get()}</p>
            <p class="mb-4 text-body-md text-ink/70">{move || body.get()}</p>
            {children()}
        </div>
    }
}

/// Message that appears for a moment after something was saved. Provided once by the settings page.
#[derive(Clone, Copy)]
pub struct Toast(pub RwSignal<Option<String>>);

/// How long a toast stays, in milliseconds.
pub const TOAST_MS: i32 = 2500;

pub fn toast(text: String) {
    if let Some(t) = use_context::<Toast>() {
        t.0.set(Some(text));
        if let Some(window) = web_sys::window() {
            let cb = wasm_bindgen::closure::Closure::once_into_js(move || t.0.set(None));
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(wasm_bindgen::JsCast::unchecked_ref(&cb), TOAST_MS);
        }
    }
}

#[component]
pub fn Toaster() -> impl IntoView {
    let Toast(message) = use_context::<Toast>().expect("Toast provided");
    view! {
        <div class="pointer-events-none fixed inset-x-0 bottom-6 z-(--z-dialog) flex justify-center px-4" role="status" aria-live="polite">
            {move || message.get().map(|m| view! { <p class="rounded-control bg-chrome px-5 py-3 text-body-md text-chrome-ink shadow-popover">{m}</p> })}
        </div>
    }
}

/// A time from the database (ISO text) written for the reader's language.
pub fn local_time(iso: &str) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(iso));
    if date.get_time().is_nan() {
        return iso.to_string();
    }
    date.to_locale_string(&crate::i18n::language(), &wasm_bindgen::JsValue::UNDEFINED).as_string().unwrap_or_else(|| iso.to_string())
}

/// Offers `text` to the browser as a file to save.
pub fn save_as_file(name: &str, text: &str, mime: &str) {
    use wasm_bindgen::JsCast;
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(a) = doc.create_element("a") else { return };
    let href = format!("data:{mime};charset=utf-8,{}", js_sys::encode_uri_component(text));
    let _ = a.set_attribute("href", &href);
    let _ = a.set_attribute("download", name);
    if let Some(el) = a.dyn_ref::<web_sys::HtmlElement>() {
        el.click();
    }
}


/// A time written for the reader's language: date only.
pub fn local_date(millis: f64) -> String {
    js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(millis)).to_locale_date_string(&crate::i18n::language(), &wasm_bindgen::JsValue::UNDEFINED).as_string().unwrap_or_default()
}

/// True from the 1024 px width, where the website layout is used.
pub fn is_wide() -> bool {
    web_sys::window().and_then(|w| w.match_media("(min-width: 64rem)").ok().flatten()).is_some_and(|m| m.matches())
}
