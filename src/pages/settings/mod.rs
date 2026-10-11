//! Settings and Account, built as a folder of screens like the Netflix app: a hub (profile card, other profiles, App settings,
//! Account, Help, Sign out) and one route per screen below it. Phones and tablets show the hub and full screens with a back
//! arrow; from 1024 px wide the same routes show a sidebar next to the screen, and the hub address opens the profile screen.
//! The tree is recorded in notes/32-settings-rebuild-plan-2026-10-11.md.
mod account;
mod app;
mod hub;
mod languages;
mod prefs;
mod profile;
mod restrictions;
mod subtitles;
mod widgets;

pub use prefs::{provide_prefs, use_prefs};

use crate::i18n::t;
use crate::store::use_profile_store;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// One screen of the tree. Profile screens carry the profile id they edit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Screen {
    Hub,
    Profiles,
    Profile(String),
    Avatar(String),
    Restrictions(String),
    Ratings(String),
    Blocked(String),
    Lock(String),
    MainProfile(String),
    Language(String),
    AudioSubtitles(String),
    Subtitles(String),
    Playback(String),
    Notifications(String),
    App,
    Licences,
    Account,
    AccountSecurity,
    AccountDevices,
    AccountProfiles,
    Name,
    Password,
    Email,
    Phone,
    Access,
    Privacy,
    Activity,
    Delete,
}

/// Which sidebar entry a screen belongs to (desktop). The phone has no sidebar, it has the hub list.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Overview,
    Security,
    Devices,
    Profiles,
    App,
}

const ROOT: &str = "/settings";

impl Screen {
    /// Reads an address like `/settings/profile/p1/subtitles`. `active` is the profile used when the address names none.
    pub fn parse(path: &str, active: &str) -> Screen {
        let rest = path.trim_start_matches('/').strip_prefix("settings").unwrap_or("");
        let parts: Vec<&str> = rest.split('/').filter(|p| !p.is_empty()).collect();
        let id = || parts.get(1).map(|s| s.to_string()).unwrap_or_else(|| active.to_string());
        match parts.as_slice() {
            ["profiles"] => Screen::Profiles,
            ["profile"] | ["profile", _] => Screen::Profile(id()),
            ["profile", _, "avatar"] => Screen::Avatar(id()),
            ["profile", _, "restrictions"] => Screen::Restrictions(id()),
            ["profile", _, "restrictions", "ratings"] => Screen::Ratings(id()),
            ["profile", _, "restrictions", "blocked"] => Screen::Blocked(id()),
            ["profile", _, "lock"] => Screen::Lock(id()),
            ["profile", _, "main"] => Screen::MainProfile(id()),
            ["profile", _, "language"] => Screen::Language(id()),
            ["profile", _, "audio-subtitles"] => Screen::AudioSubtitles(id()),
            ["profile", _, "subtitles"] => Screen::Subtitles(id()),
            ["profile", _, "playback"] => Screen::Playback(id()),
            ["profile", _, "notifications"] => Screen::Notifications(id()),
            ["app"] => Screen::App,
            ["app", "licences"] => Screen::Licences,
            ["account"] => Screen::Account,
            ["account", "security"] => Screen::AccountSecurity,
            ["account", "devices"] => Screen::AccountDevices,
            ["account", "profiles"] => Screen::AccountProfiles,
            ["account", "name"] => Screen::Name,
            ["account", "password"] => Screen::Password,
            ["account", "email"] => Screen::Email,
            ["account", "phone"] => Screen::Phone,
            ["account", "access"] => Screen::Access,
            ["account", "privacy"] => Screen::Privacy,
            ["account", "privacy", "activity"] => Screen::Activity,
            ["account", "delete"] => Screen::Delete,
            _ => Screen::Hub,
        }
    }

    /// The address of this screen. The only place addresses are written, so pages link with `Screen::X.path()`.
    pub fn path(&self) -> String {
        let profile = |id: &String, tail: &str| if id.is_empty() { format!("{ROOT}/profile") } else { format!("{ROOT}/profile/{id}{tail}") };
        match self {
            Screen::Hub => ROOT.into(),
            Screen::Profiles => format!("{ROOT}/profiles"),
            Screen::Profile(id) => profile(id, ""),
            Screen::Avatar(id) => profile(id, "/avatar"),
            Screen::Restrictions(id) => profile(id, "/restrictions"),
            Screen::Ratings(id) => profile(id, "/restrictions/ratings"),
            Screen::Blocked(id) => profile(id, "/restrictions/blocked"),
            Screen::Lock(id) => profile(id, "/lock"),
            Screen::MainProfile(id) => profile(id, "/main"),
            Screen::Language(id) => profile(id, "/language"),
            Screen::AudioSubtitles(id) => profile(id, "/audio-subtitles"),
            Screen::Subtitles(id) => profile(id, "/subtitles"),
            Screen::Playback(id) => profile(id, "/playback"),
            Screen::Notifications(id) => profile(id, "/notifications"),
            Screen::App => format!("{ROOT}/app"),
            Screen::Licences => format!("{ROOT}/app/licences"),
            Screen::Account => format!("{ROOT}/account"),
            Screen::AccountSecurity => format!("{ROOT}/account/security"),
            Screen::AccountDevices => format!("{ROOT}/account/devices"),
            Screen::AccountProfiles => format!("{ROOT}/account/profiles"),
            Screen::Name => format!("{ROOT}/account/name"),
            Screen::Password => format!("{ROOT}/account/password"),
            Screen::Email => format!("{ROOT}/account/email"),
            Screen::Phone => format!("{ROOT}/account/phone"),
            Screen::Access => format!("{ROOT}/account/access"),
            Screen::Privacy => format!("{ROOT}/account/privacy"),
            Screen::Activity => format!("{ROOT}/account/privacy/activity"),
            Screen::Delete => format!("{ROOT}/account/delete"),
        }
    }

    /// The screen one level up on a phone or tablet. The back arrow goes here, so back always lands inside the tree.
    pub fn parent(&self) -> String {
        match self {
            Screen::Hub => "/browse".into(),
            Screen::Profiles | Screen::App | Screen::Account | Screen::Profile(_) | Screen::AccountSecurity | Screen::AccountDevices | Screen::AccountProfiles => Screen::Hub.path(),
            Screen::Avatar(id) | Screen::Restrictions(id) | Screen::Lock(id) | Screen::MainProfile(id) | Screen::Language(id) | Screen::AudioSubtitles(id) | Screen::Subtitles(id) | Screen::Playback(id) | Screen::Notifications(id) => Screen::Profile(id.clone()).path(),
            Screen::Ratings(id) | Screen::Blocked(id) => Screen::Restrictions(id.clone()).path(),
            Screen::Licences => Screen::App.path(),
            Screen::Name | Screen::Password | Screen::Email | Screen::Phone | Screen::Access | Screen::Privacy | Screen::Delete => Screen::Account.path(),
            Screen::Activity => Screen::Privacy.path(),
        }
    }

    /// The screen one level up from 1024 px wide, where the account area is split into Security, Devices and Profiles pages
    /// like the Netflix website. Everything else goes where the phone goes.
    pub fn parent_wide(&self) -> String {
        match self {
            Screen::Name | Screen::Password | Screen::Email | Screen::Phone | Screen::Privacy | Screen::Delete => Screen::AccountSecurity.path(),
            Screen::Access => Screen::AccountDevices.path(),
            Screen::Profiles | Screen::Profile(_) => Screen::AccountProfiles.path(),
            Screen::AccountSecurity | Screen::AccountDevices | Screen::AccountProfiles | Screen::App => Screen::Account.path(),
            Screen::Account => "/browse".into(),
            other => other.parent(),
        }
    }

    pub fn title_key(&self) -> &'static str {
        match self {
            Screen::Hub => "set-title",
            Screen::Profiles => "set-title-profiles",
            Screen::Profile(_) => "set-title-edit-profile",
            Screen::Avatar(_) => "set-title-avatar",
            Screen::Restrictions(_) => "set-title-restrictions",
            Screen::Ratings(_) => "set-title-ratings",
            Screen::Blocked(_) => "set-title-blocked",
            Screen::Lock(_) => "set-title-lock",
            Screen::MainProfile(_) => "set-main-profile",
            Screen::Language(_) => "set-title-language",
            Screen::AudioSubtitles(_) => "set-title-audio-subtitles",
            Screen::Subtitles(_) => "set-title-subtitles",
            Screen::Playback(_) => "set-title-playback",
            Screen::Notifications(_) => "set-title-notifications",
            Screen::App => "set-title-app",
            Screen::Licences => "set-app-licences",
            Screen::Account => "set-title-account",
            Screen::AccountSecurity => "set-account-security",
            Screen::AccountDevices => "set-account-devices",
            Screen::AccountProfiles => "set-account-profiles",
            Screen::Name => "set-title-name",
            Screen::Password => "set-title-password",
            Screen::Email => "set-title-email",
            Screen::Phone => "set-title-phone",
            Screen::Access => "set-title-devices",
            Screen::Privacy => "set-title-privacy",
            Screen::Activity => "set-title-activity",
            Screen::Delete => "set-title-delete",
        }
    }

    fn section(&self) -> Section {
        match self {
            Screen::Hub | Screen::Account => Section::Overview,
            Screen::AccountSecurity | Screen::Name | Screen::Password | Screen::Email | Screen::Phone | Screen::Privacy | Screen::Activity | Screen::Delete => Section::Security,
            Screen::AccountDevices | Screen::Access => Section::Devices,
            Screen::App | Screen::Licences => Section::App,
            _ => Section::Profiles,
        }
    }

    /// Screens that change account data and ask for the password first.
    fn needs_account_check(&self) -> bool {
        matches!(self, Screen::Name | Screen::Password | Screen::Email | Screen::Phone | Screen::Access | Screen::Delete)
    }

    /// Screens that change a profile's protection and ask for the password when signed in. Changing the maturity level is one
    /// of them in the database too (it needs a recent sign in).
    fn needs_profile_check(&self) -> bool {
        matches!(self, Screen::Ratings(_) | Screen::Blocked(_) | Screen::Lock(_) | Screen::MainProfile(_))
    }
}

/// The sidebar entries from 1024 px wide: icon, translation key, the screen and the section it highlights. The same groups as the
/// Netflix website's account menu (Overview, Security, Devices, Profiles) plus the app's own settings.
fn sidebar() -> Vec<(&'static str, &'static str, Screen, Section)> {
    vec![
        ("squares-four", "set-nav-overview", Screen::Account, Section::Overview),
        ("shield-check", "set-account-security", Screen::AccountSecurity, Section::Security),
        ("devices", "set-account-devices", Screen::AccountDevices, Section::Devices),
        ("users-three", "set-account-profiles", Screen::AccountProfiles, Section::Profiles),
        ("gear-six", "set-nav-app", Screen::App, Section::App),
    ]
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let location = use_location();
    let profiles = use_profile_store();
    let active = Memo::new(move |_| profiles.active_profile_id.get().unwrap_or_default());
    let screen = Memo::new(move |_| Screen::parse(&location.pathname.get(), &active.get()));
    Effect::new(move |_| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            let title = if screen.get() == Screen::Hub { t(Screen::Account.title_key()) } else { t(screen.get().title_key()) };
            doc.set_title(&format!("{title} | {}", crate::brand::NAME));
        }
    });
    let signout_open = RwSignal::new(false);
    provide_context(hub::SignOutOpen(signout_open));

    view! {
        <div class="min-h-screen bg-void font-sans text-ink selection:bg-brand selection:text-ink">
            <div class="mx-auto flex max-w-[1100px] gap-12 px-4 pb-16 pt-4 sm:px-8 lg:px-12 lg:pt-10">
                <aside class="hidden w-60 shrink-0 lg:block">
                    <a href="/browse" class="mb-6 flex items-center" aria-label=crate::brand::NAME>
                        <img src=crate::brand::LOGO_WORDMARK alt=crate::brand::NAME class="h-7 w-auto" />
                    </a>
                    <a href="/browse" class="mb-4 flex items-center gap-2 rounded-control border border-ink/30 px-4 py-2 text-body-sm font-semibold hover:bg-ink/10">
                        <i class="ph-bold ph-arrow-left text-lg rtl:-scale-x-100" aria-hidden="true"></i>
                        {move || crate::i18n::t_args("set-back-to-app", &[("brand", crate::brand::NAME.into())])}
                    </a>
                    <nav class="flex flex-col gap-1" aria-label=move || t("set-title")>
                        {sidebar()
                            .into_iter()
                            .map(|(icon, key, target, section)| view! {
                                <a
                                    href=target.path()
                                    class="flex items-center gap-3 rounded-control px-4 py-3 text-body-md font-semibold hover:bg-ink/10"
                                    class=("bg-ink/10", move || screen.get().section() == section)
                                    class=("text-ink", move || screen.get().section() == section)
                                    class=("text-ink/70", move || screen.get().section() != section)
                                    aria-current=move || (screen.get().section() == section).then_some("page")
                                >
                                    <i class=format!("ph-bold ph-{icon} text-xl") aria-hidden="true"></i>
                                    {move || t(key)}
                                </a>
                            })
                            .collect_view()}
                        <a href="/help" class="flex items-center gap-3 rounded-control px-4 py-3 text-body-md font-semibold text-ink/70 hover:bg-ink/10">
                            <i class="ph-bold ph-question text-xl" aria-hidden="true"></i>
                            {move || t("set-nav-help")}
                        </a>
                        <button type="button" on:click=move |_| signout_open.set(true) class="flex items-center gap-3 rounded-control px-4 py-3 text-start text-body-md font-semibold text-ink/70 hover:bg-ink/10">
                            <i class="ph-bold ph-sign-out text-xl" aria-hidden="true"></i>
                            {move || t("set-sign-out")}
                        </button>
                    </nav>
                </aside>

                <main class="mx-auto w-full min-w-0 max-w-[720px] flex-1 lg:mx-0">
                    <header class="mb-6 flex items-center gap-3 lg:mb-8">
                        // phones and tablets go up one level in the phone tree; from 1024 px the website's tree is used
                        <a
                            href=move || screen.get().parent()
                            aria-label=move || t("set-back")
                            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full text-ink hover:bg-ink/10 lg:hidden"
                        >
                            <i class="ph-bold ph-arrow-left text-2xl rtl:-scale-x-100" aria-hidden="true"></i>
                        </a>
                        <a
                            href=move || screen.get().parent_wide()
                            aria-label=move || t("set-back")
                            class="hidden h-10 w-10 shrink-0 items-center justify-center rounded-full text-ink hover:bg-ink/10 lg:flex"
                            class=("lg:hidden", move || screen.get() == Screen::Account || screen.get() == Screen::Hub)
                        >
                            <i class="ph-bold ph-arrow-left text-2xl rtl:-scale-x-100" aria-hidden="true"></i>
                        </a>
                        <h1 class="truncate text-heading font-bold lg:text-title">
                            <span class=("lg:hidden", move || screen.get() == Screen::Hub)>{move || t(screen.get().title_key())}</span>
                            <span class="hidden" class=("lg:inline", move || screen.get() == Screen::Hub)>{move || t(Screen::Account.title_key())}</span>
                        </h1>
                    </header>
                    {move || {
                        let current = screen.get();
                        if current.needs_account_check() || current.needs_profile_check() {
                            let account = current.needs_account_check();
                            let shown = StoredValue::new(current);
                            view! { <account::Gate account=account>{move || screen_body(shown.get_value())}</account::Gate> }.into_any()
                        } else {
                            screen_body(current)
                        }
                    }}
                </main>
            </div>
            <widgets::ConfirmDialog open=signout_open title=t("set-sign-out") body=t("set-sign-out-body") confirm_label=t("set-sign-out") on_confirm=Callback::new(|_| hub::sign_out_now()) />
        </div>
    }
}

/// The content of one screen. The hub is two things at once: on phones it is the list, from 1024 px it is the profile screen.
fn screen_body(screen: Screen) -> AnyView {
    match screen {
        // phones: the hub list; from 1024 px the hub address shows the account overview
        Screen::Hub => view! {
            <div class="lg:hidden"><hub::Hub /></div>
            <div class="hidden lg:block"><account::Account /></div>
        }
        .into_any(),
        Screen::Profiles => view! { <hub::ManageProfiles /> }.into_any(),
        Screen::Profile(id) => view! { <profile::EditProfile id=id /> }.into_any(),
        Screen::Avatar(id) => view! { <profile::AvatarPicker id=id /> }.into_any(),
        Screen::Restrictions(id) => view! { <restrictions::Restrictions id=id /> }.into_any(),
        Screen::Ratings(id) => view! { <restrictions::Ratings id=id /> }.into_any(),
        Screen::Blocked(id) => view! { <restrictions::Blocked id=id /> }.into_any(),
        Screen::Lock(id) => view! { <profile::Lock id=id /> }.into_any(),
        Screen::MainProfile(id) => view! { <profile::MainProfile id=id /> }.into_any(),
        Screen::Language(id) => view! { <languages::DisplayLanguage id=id /> }.into_any(),
        Screen::AudioSubtitles(id) => view! { <languages::AudioSubtitles id=id /> }.into_any(),
        Screen::Subtitles(id) => view! { <subtitles::Subtitles id=id /> }.into_any(),
        Screen::Playback(id) => view! { <profile::Playback id=id /> }.into_any(),
        Screen::Notifications(id) => view! { <profile::Notifications id=id /> }.into_any(),
        Screen::App => view! { <app::AppSettings /> }.into_any(),
        Screen::Licences => view! { <app::Licences /> }.into_any(),
        Screen::Account => view! { <account::Account /> }.into_any(),
        Screen::AccountSecurity => view! { <account::SecurityPage /> }.into_any(),
        Screen::AccountDevices => view! { <account::DevicesPage /> }.into_any(),
        Screen::AccountProfiles => view! { <account::ProfilesPage /> }.into_any(),
        Screen::Name => view! { <account::Name /> }.into_any(),
        Screen::Password => view! { <account::Password /> }.into_any(),
        Screen::Email => view! { <account::Email /> }.into_any(),
        Screen::Phone => view! { <account::Phone /> }.into_any(),
        Screen::Access => view! { <account::Access /> }.into_any(),
        Screen::Privacy => view! { <account::Privacy /> }.into_any(),
        Screen::Activity => view! { <account::Activity /> }.into_any(),
        Screen::Delete => view! { <account::DeleteAccount /> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_screen() -> Vec<Screen> {
        let id = || "p1".to_string();
        vec![
            Screen::Profiles,
            Screen::Profile(id()),
            Screen::Avatar(id()),
            Screen::Restrictions(id()),
            Screen::Ratings(id()),
            Screen::Blocked(id()),
            Screen::Lock(id()),
            Screen::MainProfile(id()),
            Screen::Language(id()),
            Screen::AudioSubtitles(id()),
            Screen::Subtitles(id()),
            Screen::Playback(id()),
            Screen::Notifications(id()),
            Screen::App,
            Screen::Licences,
            Screen::Account,
            Screen::AccountSecurity,
            Screen::AccountDevices,
            Screen::AccountProfiles,
            Screen::Name,
            Screen::Password,
            Screen::Email,
            Screen::Phone,
            Screen::Access,
            Screen::Privacy,
            Screen::Activity,
            Screen::Delete,
        ]
    }

    #[test]
    fn every_address_reads_back_to_its_screen() {
        for s in every_screen() {
            assert_eq!(Screen::parse(&s.path(), "other"), s, "{}", s.path());
        }
        assert_eq!(Screen::parse("/settings", "p1"), Screen::Hub);
        assert_eq!(Screen::parse("/settings/profile", "p1"), Screen::Profile("p1".into()));
        assert_eq!(Screen::parse("/settings/nonsense", "p1"), Screen::Hub);
    }

    #[test]
    fn back_stays_inside_the_tree_and_ends_at_browse() {
        for s in every_screen() {
            let mut at = s.clone();
            for _ in 0..6 {
                if at == Screen::Hub {
                    break;
                }
                at = Screen::parse(&at.parent(), "p1");
            }
            assert_eq!(at, Screen::Hub, "{:?} never reaches the hub on a phone", s);
            // from 1024 px the website's tree is used; it ends at the overview, which ends at /browse
            let mut at = s.clone();
            for _ in 0..6 {
                if at == Screen::Account {
                    break;
                }
                at = Screen::parse(&at.parent_wide(), "p1");
            }
            assert_eq!(at, Screen::Account, "{:?} never reaches the overview on a wide screen", s);
        }
        assert_eq!(Screen::Hub.parent(), "/browse");
        assert_eq!(Screen::Ratings("p1".into()).parent(), "/settings/profile/p1/restrictions");
    }

    #[test]
    fn every_screen_has_a_title_that_exists_in_english() {
        let en = include_str!("../../../i18n/en.ftl");
        for s in every_screen() {
            assert!(en.lines().any(|l| l.starts_with(&format!("{} =", s.title_key()))), "missing {}", s.title_key());
        }
    }

    #[test]
    fn addresses_written_in_data_files_match_the_tree() {
        assert!(crate::data::settings::LEGAL_LINKS.iter().any(|(_, _, href)| *href == Screen::Licences.path()));
        assert!(crate::data::help::QUICK_LINKS.iter().any(|(id, _, href)| *id == "profiles" && *href == Screen::Profiles.path()));
    }

    #[test]
    fn only_account_changes_need_the_check() {
        assert!(Screen::Password.needs_account_check());
        assert!(Screen::Name.needs_account_check());
        assert!(Screen::Lock("p1".into()).needs_profile_check());
        assert!(!Screen::Subtitles("p1".into()).needs_profile_check());
        assert!(!Screen::Privacy.needs_account_check());
    }
}
