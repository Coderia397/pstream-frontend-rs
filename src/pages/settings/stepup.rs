//! "First, let's make sure it's you": the check before the screens that change account data. Two ways, like the Netflix page: a
//! code sent by email, or the account password. A check counts for `RECENT_SIGN_IN_SECONDS` (the database asks for a recent
//! sign in too). Without an account session the profile screens open freely and the account screens ask to sign in.
use super::account::error_text;
use super::prefs::{now_secs, use_prefs};
use super::widgets::{ActionRow, DigitBoxes, Message, PasswordField, Section, Tone};
use crate::config::{routes, CODE_LENGTH, CODE_VALID_MINUTES};
use crate::i18n::{t, t_args};
use crate::services::account::{self, AuthError};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Choose,
    Password,
    Code,
}

/// "ada@example.com" shown as "a••@example.com".
pub fn mask_email(email: &str) -> String {
    match email.split_once('@') {
        Some((local, domain)) if !local.is_empty() => {
            let first: String = local.chars().take(1).collect();
            let dots = "•".repeat(local.chars().count().saturating_sub(1).clamp(2, 8));
            format!("{first}{dots}@{domain}")
        }
        _ => email.to_string(),
    }
}

#[component]
pub fn Gate(account: bool, children: ChildrenFn) -> impl IntoView {
    let prefs = use_prefs();
    let step = RwSignal::new(Step::Choose);
    let password = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let note = RwSignal::new(None::<(Tone, String)>);
    let busy = RwSignal::new(false);
    let session = account::session();
    let masked = StoredValue::new(session.as_ref().map(|s| mask_email(&s.email)).unwrap_or_default());
    let signed_in = session.is_some();
    let verified = move || prefs.verified_until.get() > now_secs();

    let fail = move |e: AuthError| note.set(Some((Tone::Error, if e == AuthError::Invalid { t("set-code-wrong") } else { error_text(&e) })));
    let done = move || {
        password.set(String::new());
        code.set(String::new());
        prefs.mark_verified();
    };
    let submit_password = Callback::new(move |_| {
        if busy.get_untracked() || password.get_untracked().is_empty() {
            return;
        }
        busy.set(true);
        note.set(None);
        let value = password.get_untracked();
        leptos::task::spawn_local(async move {
            match account::verify_password(&value).await {
                Ok(()) => done(),
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    let send_code = Callback::new(move |_| {
        if busy.get_untracked() {
            return;
        }
        busy.set(true);
        note.set(None);
        leptos::task::spawn_local(async move {
            match account::send_code().await {
                Ok(()) => {
                    code.set(String::new());
                    step.set(Step::Code);
                }
                Err(e) => note.set(Some((Tone::Error, error_text(&e)))),
            }
            busy.set(false);
        });
    });
    let submit_code = Callback::new(move |_| {
        if busy.get_untracked() || code.get_untracked().chars().count() != CODE_LENGTH {
            return;
        }
        busy.set(true);
        note.set(None);
        let value = code.get_untracked();
        leptos::task::spawn_local(async move {
            match account::verify_code(&value).await {
                Ok(()) => done(),
                Err(e) => fail(e),
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
                        <a href=routes::LOGIN class="btn-ink inline-block rounded-control px-8 py-3">{t("help-ui-signin")}</a>
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
                <h2 class="mb-2 text-account-title-sm font-bold leading-tight lg:text-title">{move || if step.get() == Step::Code { t("set-code-title") } else { t("set-verify-title") }}</h2>
                <p class="mb-6 text-body-md text-ink/60">
                    {move || if step.get() == Step::Code { t_args("set-code-body", &[("target", masked.get_value().into()), ("minutes", CODE_VALID_MINUTES.into())]) } else { t("set-verify-body") }}
                </p>
                <Message text=note />
                {move || match step.get() {
                    Step::Choose => view! {
                        <Section>
                            <ActionRow icon="envelope-simple" title=t("set-code-email") hint=masked.get_value() chevron=true on_click=send_code />
                            <ActionRow icon="lock-simple" title=t("set-password-confirm") chevron=true on_click=Callback::new(move |_| { note.set(None); step.set(Step::Password) }) />
                        </Section>
                    }.into_any(),
                    Step::Password => view! {
                        <form on:submit=move |ev| { ev.prevent_default(); submit_password.run(()); }>
                            <PasswordField label=t("set-password-current") value=password />
                            <div class="flex flex-wrap items-center gap-4">
                                <button type="submit" disabled=move || busy.get() class="btn-ink rounded-control px-8 py-2.5 disabled:opacity-50">{t("set-continue")}</button>
                                <a href="/forgot" class="text-body-md text-ink/70 underline underline-offset-2 hover:text-ink">{t("set-forgot-password")}</a>
                            </div>
                            <button type="button" on:click=move |_| { note.set(None); step.set(Step::Choose) } class="mt-6 text-body-md font-semibold text-ink underline underline-offset-2">{t("set-try-another-way")}</button>
                        </form>
                    }.into_any(),
                    Step::Code => view! {
                        <form on:submit=move |ev| { ev.prevent_default(); submit_code.run(()); }>
                            <DigitBoxes label=t("set-code-label") value=code length=CODE_LENGTH one_time_code=true />
                            <button type="submit" disabled=move || busy.get() || code.with(|c| c.chars().count() != CODE_LENGTH) class="btn-ink mb-3 block w-full rounded-control px-8 py-3 disabled:opacity-50">{t("set-code-submit")}</button>
                            <button type="button" on:click=move |_| send_code.run(()) disabled=move || busy.get() class="mb-4 block w-full rounded-control bg-ink/10 px-8 py-3 text-body-md font-bold hover:bg-ink/15 disabled:opacity-50">{t("set-code-resend")}</button>
                            <button type="button" on:click=move |_| { note.set(None); step.set(Step::Choose) } class="block w-full py-2 text-center text-body-md font-semibold text-ink underline underline-offset-2">{t("set-try-another-way")}</button>
                        </form>
                    }.into_any(),
                }}
                <p class="mt-8 text-body-md text-ink/70">{t("set-need-help")}" "<a href=routes::HELP class="underline underline-offset-2 hover:text-ink">{t("set-visit-help")}</a></p>
            </div>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::mask_email;

    #[test]
    fn the_address_is_masked_but_recognisable() {
        assert_eq!(mask_email("ada@example.com"), "a••@example.com");
        assert_eq!(mask_email("ibrahim@example.com"), "i••••••@example.com");
        assert_eq!(mask_email("nothing"), "nothing");
    }
}
