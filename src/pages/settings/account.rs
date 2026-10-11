//! Account screens (per account): security, devices, privacy and data, viewing activity, delete. The screens that change account
//! data sit behind a password check (`Gate`), like the "make sure it's you" page in the Netflix app.
use super::hub::{persist_profiles, sign_out_now};
use super::prefs::use_prefs;
use super::widgets::{ActionRow, ConfirmDialog, Field, FormButtons, LinkRow, Message, OutlineButton, Section, SwitchRow, Tone};
use super::Screen;
use crate::config::{MIN_PASSWORD_LENGTH, PHONE_DIGITS};
use crate::i18n::t;
use crate::services::account::{self, AuthError};
use crate::store::use_profile_store;
use leptos::prelude::*;

/// How many security events the Devices screen lists.
const SECURITY_EVENTS_SHOWN: u32 = 10;

fn error_text(e: &AuthError) -> String {
    t(match e {
        AuthError::Invalid => "set-err-invalid",
        AuthError::Weak(_) => "set-err-weak",
        AuthError::RateLimited => "set-err-rate",
        AuthError::RecentSignIn => "set-err-recent",
        AuthError::Network => "set-err-network",
        _ => "set-err-other",
    })
}

fn go(path: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href(path);
    }
}

/// A time from the database (ISO text) written for the reader's language.
fn local_time(iso: &str) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(iso));
    if date.get_time().is_nan() {
        return iso.to_string();
    }
    date.to_locale_string(&crate::i18n::language(), &wasm_bindgen::JsValue::UNDEFINED).as_string().unwrap_or_else(|| iso.to_string())
}

/// Offers `text` to the browser as a file to save.
fn save_as_file(name: &str, text: &str) {
    use wasm_bindgen::JsCast;
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Ok(a) = doc.create_element("a") else { return };
    let href = format!("data:application/json;charset=utf-8,{}", js_sys::encode_uri_component(text));
    let _ = a.set_attribute("href", &href);
    let _ = a.set_attribute("download", name);
    if let Some(el) = a.dyn_ref::<web_sys::HtmlElement>() {
        el.click();
    }
}

/// Asks for the password before the screens inside it. Without an account session (a profile only used on this browser) the
/// profile screens open freely and the account screens say that signing in is needed.
#[component]
pub fn Gate(account: bool, children: ChildrenFn) -> impl IntoView {
    let prefs = use_prefs();
    let password = RwSignal::new(String::new());
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let signed_in = account::session().is_some();
    let verified = move || prefs.verified_until.get() > super::prefs::now_secs();
    let submit = Callback::new(move |_| {
        if busy.get_untracked() || password.get_untracked().is_empty() {
            return;
        }
        busy.set(true);
        note.set(None);
        let value = password.get_untracked();
        leptos::task::spawn_local(async move {
            match account::verify_password(&value).await {
                Ok(()) => {
                    password.set(String::new());
                    prefs.mark_verified();
                }
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    let children = StoredValue::new(children);

    move || {
        if !signed_in {
            return if account {
                view! {
                    <div class="text-center">
                        <p class="mb-6 text-body-md text-ink/70">{t("set-signin-needed")}</p>
                        <a href="/login" class="btn-ink inline-block rounded-control px-8 py-3">{t("help-ui-signin")}</a>
                    </div>
                }
                .into_any()
            } else {
                children.with_value(|c| c()).into_any()
            };
        }
        if verified() {
            return children.with_value(|c| c()).into_any();
        }
        view! {
            <div class="max-w-md">
                <h2 class="mb-2 text-title font-bold">{t("set-verify-title")}</h2>
                <p class="mb-6 text-body-md text-ink/60">{t("set-verify-body")}</p>
                <Message text=note />
                <form on:submit=move |ev| { ev.prevent_default(); submit.run(()); }>
                    <Field label=t("set-password-current") value=password kind="password" autocomplete="current-password" />
                    <div class="flex flex-wrap items-center gap-4">
                        <button type="submit" disabled=move || busy.get() class="btn-ink rounded-control px-8 py-2.5 disabled:opacity-50">{t("set-continue")}</button>
                        <a href="/forgot" class="text-body-md text-ink/70 underline underline-offset-2 hover:text-ink">{t("set-forgot-password")}</a>
                    </div>
                </form>
            </div>
        }
        .into_any()
    }
}

#[component]
pub fn Account() -> impl IntoView {
    let session = account::session();
    let email = session.as_ref().map(|s| s.email.clone()).unwrap_or_default();
    let phone = session.as_ref().map(|s| s.phone.clone()).unwrap_or_default();
    let full_name = session.as_ref().map(|s| format!("{} {}", s.first_name, s.last_name).trim().to_string()).unwrap_or_default();
    let or_none = |v: String| Signal::derive(move || if v.is_empty() { t("set-none") } else { v.clone() });
    view! {
        <div>
            {session.is_none().then(|| view! {
                <div class="mb-8 rounded-card border border-ink/10 bg-ink/5 p-5 text-center">
                    <p class="mb-4 text-body-md text-ink/70">{t("set-signin-needed")}</p>
                    <a href="/login" class="btn-ink inline-block rounded-control px-8 py-3">{t("help-ui-signin")}</a>
                </div>
            })}
            <Section title=t("set-account-security")>
                <LinkRow icon="identification-card" title=t("set-title-name") href=Screen::Name.path() value=or_none(full_name) />
                <LinkRow icon="lock-simple" title=t("set-title-password") href=Screen::Password.path() />
                <LinkRow icon="envelope-simple" title=t("set-title-email") href=Screen::Email.path() value=or_none(email) />
                <LinkRow icon="device-mobile" title=t("set-title-phone") href=Screen::Phone.path() value=or_none(phone) />
            </Section>
            <Section title=t("set-account-devices")>
                <LinkRow icon="devices" title=t("set-title-devices") href=Screen::Devices.path() />
            </Section>
            <Section title=t("set-account-profiles")>
                <LinkRow icon="users-three" title=t("set-nav-profiles") href=Screen::Profiles.path() />
                <LinkRow icon="shield-check" title=t("set-title-privacy") href=Screen::Privacy.path() />
            </Section>
            <OutlineButton label=t("set-title-delete") danger=true on_click=Callback::new(|_| go(&Screen::Delete.path())) />
        </div>
    }
}

/// First and last name of the person behind the account (account_details). The main profile follows its first name.
#[component]
pub fn Name() -> impl IntoView {
    let session = account::session();
    let first = RwSignal::new(session.as_ref().map(|s| s.first_name.clone()).unwrap_or_default());
    let last = RwSignal::new(session.as_ref().map(|s| s.last_name.clone()).unwrap_or_default());
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let save = Callback::new(move |_| {
        let (f, l) = (first.get_untracked(), last.get_untracked());
        if f.trim().is_empty() || l.trim().is_empty() {
            note.set(Some((Tone::Error, t("set-name-required"))));
            return;
        }
        busy.set(true);
        note.set(None);
        leptos::task::spawn_local(async move {
            match account::update_names(&f, &l).await {
                Ok(()) => note.set(Some((Tone::Done, t("set-name-saved")))),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    view! {
        <div class="max-w-md">
            <p class="mb-6 text-body-md text-ink/60">{t("set-name-hint")}</p>
            <Message text=note />
            <Field label=t("set-name-first") value=first autocomplete="given-name" maxlength=60 />
            <Field label=t("set-name-last") value=last autocomplete="family-name" maxlength=60 />
            <FormButtons save_label=t("set-save") on_save=save cancel_href=Screen::Account.path() busy=busy />
        </div>
    }
}

#[component]
pub fn Password() -> impl IntoView {
    let new_password = RwSignal::new(String::new());
    let again = RwSignal::new(String::new());
    let all_devices = RwSignal::new(true);
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let save = Callback::new(move |_| {
        let value = new_password.get_untracked();
        if value.chars().count() < MIN_PASSWORD_LENGTH {
            note.set(Some((Tone::Error, crate::i18n::t_args("set-password-short", &[("min", MIN_PASSWORD_LENGTH.into())]))));
            return;
        }
        if value != again.get_untracked() {
            note.set(Some((Tone::Error, t("set-password-mismatch"))));
            return;
        }
        busy.set(true);
        note.set(None);
        let everywhere = all_devices.get_untracked();
        leptos::task::spawn_local(async move {
            match account::change_password(&value).await {
                Ok(()) if everywhere => {
                    let _ = account::sign_out_everywhere().await;
                    go("/login");
                }
                Ok(()) => {
                    new_password.set(String::new());
                    again.set(String::new());
                    note.set(Some((Tone::Done, t("set-password-saved"))));
                }
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    view! {
        <div class="max-w-md">
            <p class="mb-6 text-body-md text-ink/60">{crate::i18n::t_args("set-password-hint", &[("min", MIN_PASSWORD_LENGTH.into())])}</p>
            <Message text=note />
            <Field label=t("set-password-new") value=new_password kind="password" autocomplete="new-password" />
            <Field label=t("set-password-again") value=again kind="password" autocomplete="new-password" />
            <label class="mb-6 flex items-center gap-3 text-body-md">
                <input type="checkbox" prop:checked=move || all_devices.get() on:change=move |ev| all_devices.set(event_target_checked(&ev)) class="h-5 w-5 accent-brand" />
                {t("set-password-signout-all")}
            </label>
            <FormButtons save_label=t("set-save") on_save=save cancel_href=Screen::Account.path() busy=busy />
        </div>
    }
}

#[component]
pub fn Email() -> impl IntoView {
    let current = account::session().map(|s| s.email).unwrap_or_default();
    let new_email = RwSignal::new(String::new());
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let save = Callback::new(move |_| {
        let value = new_email.get_untracked();
        if !account::valid_email(&value) {
            note.set(Some((Tone::Error, t("set-email-invalid"))));
            return;
        }
        busy.set(true);
        note.set(None);
        leptos::task::spawn_local(async move {
            match account::change_email(&value).await {
                Ok(()) => note.set(Some((Tone::Done, t("set-email-sent")))),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    view! {
        <div class="max-w-md">
            <p class="mb-6 text-body-md text-ink/60">{t("set-email-hint")}</p>
            <Message text=note />
            <Field label=t("set-email-current") value=RwSignal::new(current) kind="email" />
            <Field label=t("set-email-new") value=new_email kind="email" autocomplete="email" />
            <FormButtons save_label=t("set-save") on_save=save cancel_href=Screen::Account.path() busy=busy />
        </div>
    }
}

#[component]
pub fn Phone() -> impl IntoView {
    let number = RwSignal::new(account::session().map(|s| s.phone).unwrap_or_default());
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let save = Callback::new(move |_| {
        let value = number.get_untracked();
        let digits = value.chars().filter(|c| c.is_ascii_digit()).count();
        if !value.trim().is_empty() && !PHONE_DIGITS.contains(&digits) {
            note.set(Some((Tone::Error, t("set-phone-invalid"))));
            return;
        }
        busy.set(true);
        note.set(None);
        leptos::task::spawn_local(async move {
            match account::save_phone(&value).await {
                Ok(()) => note.set(Some((Tone::Done, t("set-phone-saved")))),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    view! {
        <div class="max-w-md">
            <p class="mb-6 text-body-md text-ink/60">{t("set-phone-hint")}</p>
            <Message text=note />
            <Field label=t("set-title-phone") value=number kind="tel" autocomplete="tel" />
            <FormButtons save_label=t("set-save") on_save=save cancel_href=Screen::Account.path() busy=busy />
        </div>
    }
}

#[component]
pub fn Devices() -> impl IntoView {
    let confirm = RwSignal::new(false);
    let note = RwSignal::new(None::<(Tone, String)>);
    let sign_out_all = Callback::new(move |_| {
        leptos::task::spawn_local(async move {
            match account::sign_out_everywhere().await {
                Ok(()) => go("/login"),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
        });
    });
    let browser = web_sys::window().and_then(|w| w.navigator().user_agent().ok()).unwrap_or_default();
    // newest security events of the account; a failure (not signed in, tables not set up yet) just leaves the list out
    let events = RwSignal::new(Vec::<(String, String)>::new());
    leptos::task::spawn_local(async move {
        if let Ok(list) = account::security_events(SECURITY_EVENTS_SHOWN).await {
            events.set(list);
        }
    });
    view! {
        <div>
            <p class="mb-6 text-body-md text-ink/60">{t("set-devices-hint")}</p>
            <Message text=note />
            <div class="mb-6 rounded-card border border-ink/10 bg-ink/5 p-5">
                <span class="mb-2 inline-block rounded-control bg-brand/20 px-2 py-0.5 text-micro font-bold uppercase tracking-wide text-brand">{t("set-devices-current")}</span>
                <p class="text-body-md font-bold">{web_sys::window().and_then(|w| w.navigator().platform().ok()).unwrap_or_default()}</p>
                <p class="mt-1 break-words text-label text-ink/60">{browser}</p>
            </div>
            <p class="mb-6 text-label text-ink/50">{t("set-devices-note")}</p>
            <Show when=move || !events.get().is_empty()>
                <Section title=t("set-events-title")>
                    {move || events.get().into_iter().map(|(ts, event)| view! {
                        <div class="flex items-center justify-between gap-4 px-5 py-3">
                            <span class="text-body-md">{crate::i18n::t_or(&format!("set-event-{event}"), &event)}</span>
                            <span class="shrink-0 text-label text-ink/50">{local_time(&ts)}</span>
                        </div>
                    }).collect_view()}
                </Section>
            </Show>
            <OutlineButton label=t("set-devices-signout-all") danger=true on_click=Callback::new(move |_| confirm.set(true)) />
            <ConfirmDialog open=confirm title=t("set-devices-signout-all") body=t("set-devices-signout-body") confirm_label=t("set-sign-out") danger=true on_confirm=sign_out_all />
        </div>
    }
}

#[component]
pub fn Privacy() -> impl IntoView {
    let consent = crate::consent::use_ui();
    let profiles = use_profile_store();
    let active = Memo::new(move |_| profiles.profiles.with(|l| profiles.active_profile_id.with(|id| l.iter().find(|p| Some(&p.id) == id.as_ref()).or(l.first()).cloned())));
    let note = RwSignal::new(None::<(Tone, String)>);
    let export = Callback::new(move |_| {
        note.set(None);
        leptos::task::spawn_local(async move {
            match account::export_data().await {
                Ok(text) => save_as_file("my-data.json", &text),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
        });
    });
    view! {
        <div>
            {move || active.get().map(|p| view! { <p class="mb-6 text-body-md text-ink/60">{crate::i18n::t_args("set-for-profile", &[("name", p.name.clone().into())])}</p> })}
            <Message text=note />
            <Section title=t("set-privacy-viewing")>
                {move || active.get().map(|p| view! { <LinkRow icon="lock-simple" title=t("set-title-lock") hint=t("set-privacy-lock-hint") href=Screen::Lock(p.id).path() /> })}
                <LinkRow icon="clock-counter-clockwise" title=t("set-title-activity") hint=t("set-privacy-activity-hint") href=Screen::Activity.path() />
            </Section>
            <Section title=t("set-privacy-data")>
                <SwitchRow
                    icon="sparkle"
                    title=t("set-privacy-personalization")
                    hint=t("set-privacy-personalization-hint")
                    checked=Signal::derive(move || consent.personalization.get())
                    on_change=Callback::new(move |v| consent.decide(v))
                />
                <ActionRow icon="cookie" title=t("cookie-link") on_click=Callback::new(move |_| consent.open_center()) />
                <ActionRow icon="download-simple" title=t("set-privacy-export") hint=t("set-privacy-export-hint") on_click=export />
            </Section>
        </div>
    }
}

#[component]
pub fn Activity() -> impl IntoView {
    let watch = crate::store::use_watch_store();
    let confirm = RwSignal::new(false);
    let rows = Memo::new(move |_| {
        let mut list: Vec<_> = watch.records.with(|m| m.values().cloned().collect());
        list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        list
    });
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-activity-hint")}</p>
            {move || {
                let list = rows.get();
                if list.is_empty() {
                    return view! { <p class="rounded-card border border-ink/10 bg-ink/5 p-6 text-center text-body-md text-ink/50">{t("settings-no-history")}</p> }.into_any();
                }
                view! {
                    <ul class="mb-8 overflow-hidden rounded-card border border-ink/10 bg-ink/5 divide-y divide-ink/10">
                        {list.into_iter().map(|r| {
                            let (id, tv) = (r.media_id, r.is_tv);
                            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(r.updated_at as f64)).to_locale_date_string(&crate::i18n::language(), &wasm_bindgen::JsValue::UNDEFINED).as_string().unwrap_or_default();
                            let episode = match (r.season, r.episode) {
                                (Some(s), Some(e)) => format!(" · S{s}:E{e}"),
                                _ => String::new(),
                            };
                            view! {
                                <li class="flex items-center gap-4 px-5 py-3">
                                    <span class="min-w-0 flex-1">
                                        <span class="block truncate text-body-md font-bold">{format!("{}{episode}", r.title)}</span>
                                        <span class="block text-label text-ink/50">{date}</span>
                                    </span>
                                    <button type="button" aria-label=t("set-remove") on:click=move |_| watch.remove_record(id, tv) class="flex h-9 w-9 items-center justify-center rounded-full text-ink/70 hover:bg-ink/10">
                                        <i class="ph-bold ph-trash text-xl" aria-hidden="true"></i>
                                    </button>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                    <OutlineButton label=t("set-activity-clear") danger=true on_click=Callback::new(move |_| confirm.set(true)) />
                }
                .into_any()
            }}
            <ConfirmDialog open=confirm title=t("set-activity-clear") body=t("set-activity-clear-body") confirm_label=t("set-delete") danger=true on_confirm=Callback::new(move |_| watch.clear_all()) />
        </div>
    }
}

#[component]
pub fn DeleteAccount() -> impl IntoView {
    let confirm = RwSignal::new(false);
    let note = RwSignal::new(None::<(Tone, String)>);
    let profiles = use_profile_store();
    let prefs = use_prefs();
    let delete = Callback::new(move |_| {
        leptos::task::spawn_local(async move {
            match account::delete_account().await {
                Ok(()) => {
                    profiles.profiles.set(Vec::new());
                    persist_profiles(profiles);
                    prefs.forget_all();
                    sign_out_now();
                }
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
        });
    });
    view! {
        <div class="max-w-md">
            <p class="mb-4 text-body-md text-ink/70">{t("set-delete-account-body")}</p>
            <Message text=note />
            <OutlineButton label=t("set-title-delete") danger=true on_click=Callback::new(move |_| confirm.set(true)) />
            <ConfirmDialog open=confirm title=t("set-title-delete") body=t("set-delete-account-confirm") confirm_label=t("set-delete") danger=true on_confirm=delete />
        </div>
    }
}
