pub mod components;
pub mod data;
pub mod models;
pub mod pages;
pub mod services;
pub mod store;
pub mod utils;

rust_i18n::i18n!("locales", fallback = "en");

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
                <Route path=path!("/browse/genre/:id") view=crate::pages::browse_grid::BrowseGridPage />
                <Route path=path!("/watch/:id") view=crate::pages::player::PlayerPage />
                <Route path=path!("") view=|| view! { <leptos_router::components::Redirect path="/browse" /> } />
                <Route path=path!("/search") view=crate::pages::search::SearchPage />
                <Route path=path!("/browse/my-list") view=crate::pages::my_list::MyListPage />
                <Route path=path!("/browse/movies") view=|| view! { <crate::pages::category::CategoryPage kind="movie" /> } />
                <Route path=path!("/browse/films") view=|| view! { <crate::pages::category::CategoryPage kind="movie" /> } />
                <Route path=path!("/browse/shows") view=|| view! { <crate::pages::category::CategoryPage kind="tv" /> } />
                <Route path=path!("/browse/series") view=|| view! { <crate::pages::category::CategoryPage kind="tv" /> } />
                <Route path=path!("/tv") view=|| view! { <leptos_router::components::Redirect path="/browse/shows" /> } />
                <Route path=path!("/movies") view=|| view! { <leptos_router::components::Redirect path="/browse/movies" /> } />
                <Route path=path!("/new") view=|| view! { <leptos_router::components::Redirect path="/latest" /> } />
                <Route path=path!("/list") view=|| view! { <leptos_router::components::Redirect path="/browse/my-list" /> } />
                <Route path=path!("/browse/language") view=crate::pages::language::BrowseLanguagePage />
                <Route path=path!("/latest") view=crate::pages::new_popular::NewPopularPage />
                <Route path=path!("/clips") view=crate::pages::clips::ClipsPage />
                <Route path=path!("/reads") view=crate::pages::reads::ReadsPage />
                <Route path=path!("/settings") view=crate::pages::settings::SettingsPage />
                <Route path=path!("/login") view=crate::pages::login::LoginPage />
                <Route path=path!("/privacy") view=crate::pages::legal::PrivacyPage />
                <Route path=path!("/terms") view=crate::pages::legal::TermsPage />
                <Route path=path!("/cookies") view=crate::pages::legal::CookiePolicyPage />
                <Route path=path!("/dmca") view=crate::pages::legal::DmcaPage />
                <Route path=path!("/disclaimer") view=crate::pages::legal::DisclaimerPage />
                <Route path=path!("/contact") view=crate::pages::legal::ContactPage />
                <Route path=path!("/notifications") view=crate::pages::notifications::NotificationsPage />
            </Routes>
        </Router>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_target_linkage() {
        // Confirms host rlib target linkage and basic mapping resolution
        let result = models::mapping::map_netflix_id_to_tmdb("1365");
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
        assert_eq!(data::MOVIE_GENRES.len(), 27);
        assert_eq!(data::TV_GENRES.len(), 25);
    }
}
