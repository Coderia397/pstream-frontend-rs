//! The settings hub (the Profile sheet of the Netflix app) and Manage Profiles.
use super::widgets::{ActionRow, LinkRow, Section};
use crate::components::profiles::add_edit_modal::{AddEditProfileModal, ProfileFormData};
use crate::i18n::t;
use crate::models::profile::Profile;
use crate::store::{use_profile_store, ProfileStore};
use leptos::prelude::*;

/// Opens the "Sign out?" dialog that the settings page renders once.
#[derive(Clone, Copy)]
pub struct SignOutOpen(pub RwSignal<bool>);

/// Non kids profiles an account can hold. The Kids profile does not count toward it.
pub const MAX_PROFILES: usize = 5;

pub fn persist_profiles(store: ProfileStore) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(json) = serde_json::to_string(&store.profiles.get_untracked()) {
            let _ = storage.set_item(crate::keys::PROFILES, &json);
        }
    }
}

/// Signs out of the account and returns to the landing page with nothing left of this person's choices in the page.
pub fn sign_out_now() {
    crate::services::account::sign_out();
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            let _ = storage.remove_item(crate::keys::ACTIVE_PROFILE);
        }
        let _ = window.location().set_href(crate::config::routes::HOME);
    }
}

/// Makes `profile` the one in use. A profile with a PIN goes through the Who's watching screen so the PIN is asked.
pub fn switch_to(store: ProfileStore, profile: &Profile) {
    if profile.pin.is_some() {
        store.active_profile_id.set(None);
        return;
    }
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(crate::keys::ACTIVE_PROFILE, &profile.id);
    }
    store.is_kids_mode.set(profile.is_kids);
    store.active_profile_id.set(Some(profile.id.clone()));
}

fn avatar_of(p: &Profile) -> String {
    p.avatar_url.clone().unwrap_or_else(|| crate::brand::DEFAULT_AVATAR.to_string())
}

#[component]
pub fn Hub() -> impl IntoView {
    let store = use_profile_store();
    let signout = use_context::<SignOutOpen>();
    let active = Memo::new(move |_| store.profiles.with(|l| store.active_profile_id.with(|id| l.iter().find(|p| Some(&p.id) == id.as_ref()).or(l.first()).cloned())));
    let others = Memo::new(move |_| {
        let id = active.get().map(|p| p.id);
        store.profiles.with(|l| l.iter().filter(|p| Some(&p.id) != id.as_ref()).cloned().collect::<Vec<_>>())
    });

    view! {
        <div>
            {move || active.get().map(|p| view! {
                <a href=format!("/settings/profile/{}", p.id) class="mb-6 flex items-center gap-5 rounded-card border border-ink/10 bg-ink/5 p-5 hover:bg-ink/10">
                    <img src=avatar_of(&p) alt="" class="h-20 w-20 rounded-control object-cover" />
                    <span class="min-w-0 flex-1">
                        <span class="block truncate text-heading font-bold">{p.name.clone()}</span>
                        <span class="mt-0.5 block text-label text-ink/50">{t("set-edit-profile-hint")}</span>
                    </span>
                    <i class="ph-bold ph-pencil-simple text-2xl text-ink/70" aria-hidden="true"></i>
                </a>
            })}
            <ul class="mb-6 flex flex-wrap justify-center gap-5">
                {move || others.get().into_iter().map(|p| {
                    let target = p.clone();
                    view! {
                        <li>
                            <button type="button" on:click=move |_| switch_to(store, &target) class="flex w-20 flex-col items-center gap-2 text-label text-ink/80 hover:text-ink">
                                <img src=avatar_of(&p) alt="" class="h-16 w-16 rounded-control object-cover" />
                                <span class="w-full truncate text-center">{p.name.clone()}</span>
                            </button>
                        </li>
                    }
                }).collect_view()}
            </ul>
            <div class="mb-10 flex justify-center">
                <a href="/settings/profiles" class="rounded-full bg-ink/10 px-6 py-3 text-body-md font-bold hover:bg-ink/20">{t("set-manage-profiles")}</a>
            </div>
            <Section>
                <LinkRow icon="gear-six" title=t("set-nav-app") href="/settings/app" />
                <LinkRow icon="user" title=t("set-nav-account") href="/settings/account" />
                <LinkRow icon="question" title=t("set-nav-help") href=crate::config::routes::HELP />
                <ActionRow icon="sign-out" title=t("set-sign-out") on_click=Callback::new(move |_| if let Some(s) = signout { s.0.set(true) }) />
            </Section>
        </div>
    }
}

#[component]
pub fn ManageProfiles() -> impl IntoView {
    let store = use_profile_store();
    let adding = RwSignal::new(false);
    let can_add = move || store.profiles.with(|l| l.iter().filter(|p| !p.is_kids).count() < MAX_PROFILES) || store.profiles.with(|l| !l.iter().any(|p| p.is_kids));
    let on_save = Callback::new(move |data: ProfileFormData| {
        let name = data.name.trim().to_string();
        if name.is_empty() {
            return;
        }
        store.profiles.update(|list| {
            let order = list.iter().map(|p| p.sort_order).max().unwrap_or(0) + 1;
            list.push(Profile { id: format!("p{}", js_sys::Date::now() as u64), name, avatar_url: data.avatar_url, is_kids: data.is_kids, is_default: Some(false), pin: data.pin, sort_order: order });
        });
        persist_profiles(store);
        adding.set(false);
    });

    view! {
        <div>
            <p class="mb-6 text-body-md text-ink/60">{t("set-profiles-hint")}</p>
            <ul class="grid grid-cols-2 gap-6 sm:grid-cols-3">
                {move || store.profiles.get().into_iter().map(|p| view! {
                    <li>
                        <a href=format!("/settings/profile/{}", p.id) class="group flex flex-col items-center gap-3 text-center">
                            <span class="relative block h-28 w-28 overflow-hidden rounded-card">
                                <img src=avatar_of(&p) alt="" class="h-full w-full object-cover opacity-70" />
                                <span class="absolute inset-0 flex items-center justify-center bg-void/40">
                                    <i class="ph-bold ph-pencil-simple text-3xl text-ink" aria-hidden="true"></i>
                                </span>
                            </span>
                            <span class="w-full truncate text-body-md font-semibold">{p.name.clone()}</span>
                            {p.is_kids.then(|| view! { <span class="text-label text-ink/50">{t("set-kids-profile")}</span> })}
                        </a>
                    </li>
                }).collect_view()}
                <Show when=can_add>
                    <li>
                        <button type="button" on:click=move |_| adding.set(true) class="flex w-full flex-col items-center gap-3">
                            <span class="flex h-28 w-28 items-center justify-center rounded-card bg-ink/10 hover:bg-ink/20">
                                <i class="ph-bold ph-plus text-4xl text-ink/80" aria-hidden="true"></i>
                            </span>
                            <span class="text-body-md font-semibold">{t("set-add-profile")}</span>
                        </button>
                    </li>
                </Show>
            </ul>
            <Show when=move || adding.get()>
                <AddEditProfileModal mode="add" initial=None on_save=on_save on_delete=None on_cancel=Callback::new(move |_| adding.set(false)) />
            </Show>
        </div>
    }
}
