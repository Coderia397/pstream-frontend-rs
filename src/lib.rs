pub mod brand;
pub mod components;
pub mod config;
pub mod consent;
pub mod data;
pub mod i18n;
pub mod keys;
pub mod legacy_keys;
pub mod models;
pub mod pages;
pub mod services;
pub mod store;
pub mod utils;


use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use pages::browse::BrowseHome;

#[component]
pub fn App() -> impl IntoView {
    crate::store::provide_ui_store();
    crate::store::provide_profile_store();
    crate::store::provide_library_store();
    crate::store::provide_watch_store();
    crate::consent::provide_ui();
    crate::pages::settings::provide_prefs();
    // a browser that starts already signed in tells the account it is still here
    crate::services::device::register_in_background();

    // On help.<domain> the front door is the help center
    let on_help_host = web_sys::window().and_then(|w| w.location().hostname().ok()).map(|h| h.starts_with("help.")).unwrap_or(false);

    let profile_store = crate::store::use_profile_store();
    let has_no_profile = move || profile_store.active_profile_id.get().is_none();

    view! {
        <crate::components::media::info_modal::InfoModal />
        <Show when=has_no_profile>
            <crate::components::profiles::whos_watching::WhosWatchingGate />
        </Show>
        <Router>
            <Routes fallback=crate::pages::not_found::NotFoundPage>
                <Route path=path!("/browse") view=BrowseHome />
                <Route path=path!("/dev/badges") view=crate::pages::dev_badges::DevBadgesPage />
                <Route path=path!("/browse/genre/:id") view=crate::pages::category::GenreRoute />
                <Route path=path!("/watch/:id") view=crate::pages::player::PlayerPage />
                <Route path=path!("/watch/:kind/:id") view=crate::pages::player::PlayerPage />
                <Route
                    path=path!("")
                    view=move || {
                        if on_help_host {
                            view! { <leptos_router::components::Redirect path="/help" /> }.into_any()
                        } else if crate::services::account::is_returning() {
                            view! { <leptos_router::components::Redirect path="/browse" /> }.into_any()
                        } else {
                            view! { <crate::pages::login::LandingPage /> }.into_any()
                        }
                    }
                />
                <Route path=path!("/help") view=crate::pages::help::HelpPage />
                <Route path=path!("/help/search") view=crate::pages::help::ResultsPage />
                <Route path=path!("/help/contact") view=crate::pages::help::ContactPage />
                <Route path=path!("/help/article/:slug") view=crate::pages::help::ArticlePage />
                <Route path=path!("/search") view=crate::pages::search::SearchPage />
                <Route path=path!("/browse/my-list") view=crate::pages::my_list::MyListPage />
                <Route path=path!("/browse/movies") view=|| view! { <crate::pages::category::CategoryPage kind="movie" /> } />
                <Route path=path!("/browse/films") view=|| view! { <crate::pages::category::CategoryPage kind="movie" /> } />
                <Route path=path!("/browse/shows") view=|| view! { <crate::pages::category::CategoryPage kind="tv" /> } />
                <Route path=path!("/browse/series") view=|| view! { <crate::pages::category::CategoryPage kind="tv" /> } />
                <Route path=path!("/browse/games") view=crate::pages::games::GamesPage />
                <Route path=path!("/games") view=crate::pages::games::GamesPage />
                <Route path=path!("/tv") view=|| view! { <leptos_router::components::Redirect path="/browse/shows" /> } />
                <Route path=path!("/movies") view=|| view! { <leptos_router::components::Redirect path="/browse/movies" /> } />
                <Route path=path!("/new") view=|| view! { <leptos_router::components::Redirect path="/latest" /> } />
                <Route path=path!("/list") view=|| view! { <leptos_router::components::Redirect path="/browse/my-list" /> } />
                <Route path=path!("/browse/language") view=crate::pages::language::BrowseLanguagePage />
                <Route path=path!("/latest") view=crate::pages::new_popular::NewPopularPage />
                <Route path=path!("/clips") view=crate::pages::clips::ClipsPage />
                <Route path=path!("/reads") view=crate::pages::reads::ReadsPage />
                <Route path=path!("/settings") view=crate::pages::settings::SettingsPage />
                <Route path=path!("/settings/*any") view=crate::pages::settings::SettingsPage />
                <Route path=path!("/login") view=crate::pages::auth::LoginPage />
                <Route path=path!("/signup") view=crate::pages::auth::SignupPage />
                <Route path=path!("/forgot") view=crate::pages::auth::ForgotPage />
                <Route path=path!("/auth/confirm") view=crate::pages::auth::ConfirmPage />
                <Route path=path!("/welcome") view=|| view! { <leptos_router::components::Redirect path="/" /> } />
                <Route path=path!("/privacy") view=|| view! { <crate::pages::help::LegalPage doc="privacy" /> } />
                <Route path=path!("/terms") view=|| view! { <crate::pages::help::LegalPage doc="terms" /> } />
                <Route path=path!("/cookies") view=|| view! { <crate::pages::help::LegalPage doc="cookie-policy" /> } />
                <Route path=path!("/dmca") view=|| view! { <crate::pages::help::LegalPage doc="dmca" /> } />
                <Route path=path!("/credits") view=|| view! { <crate::pages::help::LegalPage doc="credits" /> } />
                <Route path=path!("/disclaimer") view=|| view! { <crate::pages::help::LegalPage doc="disclaimer" /> } />
                <Route path=path!("/contact") view=|| view! { <leptos_router::components::Redirect path="/help/contact" /> } />
                <Route path=path!("/notifications") view=crate::pages::notifications::NotificationsPage />
            </Routes>
            <crate::components::consent::CookieManager />
        </Router>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_target_linkage() {
        // Confirms host rlib target linkage and basic mapping resolution
        let result = models::mapping::map_genre_key_to_tmdb("1365");
        assert!(result.is_some());
        let ctx = result.unwrap();
        assert_eq!(ctx.title, "Action & Adventure");
        assert_eq!(ctx.tmdb_genre_id, Some(28));
    }

    #[test]
    fn test_static_data_linkage() {
        assert_eq!(data::GENRES.len(), 27);
        assert_eq!(data::DISPLAY_LANGUAGES.len(), 20);
        assert_eq!(data::ALL_AVATARS.len(), 41);
        assert_eq!(data::MOVIE_GENRES.len(), 26);
        assert_eq!(data::TV_GENRES.len(), 24);
    }
}
