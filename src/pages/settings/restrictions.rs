//! Viewing restrictions of one profile: the maturity level (profiles.maturity) and the titles that are blocked.
use super::prefs::use_prefs;
use super::widgets::{toast, LinkRow, Section, Stepper};
use super::Screen;
use crate::config::{BLOCKED_TITLES_MAX, BLOCKED_TITLE_MAX};
use crate::data::settings::{maturity, MATURITY_LEVELS};
use crate::i18n::{t, t_args};
use crate::store::use_profile_store;
use leptos::prelude::*;

fn is_kids(id: &str) -> bool {
    use_profile_store().profiles.with(|l| l.iter().find(|p| p.id == id).is_some_and(|p| p.is_kids))
}

#[component]
pub fn Restrictions(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let level_value = Signal::derive(move || t(maturity(prefs.profile(&id.get_value()).maturity_for(is_kids(&id.get_value()))).label_key));
    let blocked_value = Signal::derive(move || {
        let n = prefs.profile(&id.get_value()).blocked_titles.len();
        if n == 0 { t("set-none") } else { t_args("set-blocked-count", &[("count", n.into())]) }
    });
    view! {
        <Section>
            <LinkRow icon="warning-circle" title=t("set-title-ratings") href=Screen::Ratings(id.get_value()).path() value=level_value />
            <LinkRow icon="prohibit" title=t("set-title-blocked") href=Screen::Blocked(id.get_value()).path() value=blocked_value />
        </Section>
    }
}

/// Maturity levels as a stepper: each level allows everything below it, with what it allows written under it. A Kids profile is
/// offered the two kids levels only.
#[component]
pub fn Ratings(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let kids = is_kids(&id.get_value());
    let selected = Signal::derive(move || prefs.profile(&id.get_value()).maturity_for(kids).to_string());
    let options: Vec<(String, String, String)> = MATURITY_LEVELS.iter().filter(|m| !kids || m.kids).map(|m| (m.key.to_string(), t(m.label_key), t(m.hint_key))).collect();
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-ratings-hint")}</p>
            <Stepper options=options selected=selected on_pick=Callback::new(move |v: String| { prefs.update_profile(&id.get_value(), |p| p.maturity = Some(v)); toast(t("set-saved")); }) />
        </div>
    }
}

#[component]
pub fn Blocked(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let text = RwSignal::new(String::new());
    let list = Memo::new(move |_| prefs.profile(&id.get_value()).blocked_titles);
    let add = move || {
        let value = text.get_untracked().trim().chars().take(BLOCKED_TITLE_MAX).collect::<String>();
        if value.is_empty() {
            return;
        }
        prefs.update_profile(&id.get_value(), |p| {
            if p.blocked_titles.len() < BLOCKED_TITLES_MAX && !p.blocked_titles.iter().any(|b| b.to_lowercase() == value.to_lowercase()) {
                p.blocked_titles.push(value);
            }
        });
        text.set(String::new());
    };
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-blocked-hint")}</p>
            <div class="mb-5 flex gap-3">
                <input
                    type="text"
                    maxlength=BLOCKED_TITLE_MAX
                    placeholder=move || t("set-blocked-placeholder")
                    aria-label=move || t("set-title-blocked")
                    prop:value=move || text.get()
                    on:input=move |ev| text.set(event_target_value(&ev))
                    on:keydown=move |ev| if ev.key() == "Enter" { add() }
                    class="min-w-0 flex-1 rounded-control border border-ink/20 bg-ink/10 px-4 py-3 text-body-md text-ink focus:border-ink focus:outline-none"
                />
                <button type="button" on:click=move |_| add() class="btn-ink rounded-control px-6">{t("set-add")}</button>
            </div>
            <ul class="overflow-hidden rounded-card border border-ink/10 bg-ink/5 divide-y divide-ink/10">
                {move || list.get().into_iter().map(|title| {
                    let gone = title.clone();
                    view! {
                        <li class="flex items-center px-5 py-3">
                            <span class="flex-1 truncate text-body-md">{title}</span>
                            <button type="button" aria-label=t("set-remove") on:click=move |_| prefs.update_profile(&id.get_value(), |p| p.blocked_titles.retain(|b| b != &gone)) class="flex h-9 w-9 items-center justify-center rounded-full text-ink/70 hover:bg-ink/10">
                                <i class="ph-bold ph-trash text-xl" aria-hidden="true"></i>
                            </button>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </div>
    }
}
