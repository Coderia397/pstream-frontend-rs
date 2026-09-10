use leptos::prelude::*;
use leptos_router::hooks::use_location;
use wasm_bindgen::JsCast;
use crate::components::layout::search_bar::SearchBar;
use crate::components::layout::notifications_dropdown::NotificationsDropdown;
use crate::components::layout::navbar_mobile::NavbarMobile;

#[component]
pub fn Navbar(
    #[prop(optional)] scroll_y: Option<f64>,
) -> impl IntoView {
    let (scroll_y_sig, set_scroll_y_sig) = signal(scroll_y.unwrap_or(0.0));

    // Dynamic window scroll listener (Task 085)
    Effect::new(move |_| {
        if scroll_y.is_none() {
            if let Some(win) = web_sys::window() {
                let win_clone = win.clone();
                let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                    let y = win_clone.scroll_y().unwrap_or(0.0);
                    set_scroll_y_sig.set(y);
                }));
                let _ = win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
                cb.forget();
            }
        }
    });

    // Scroll opacity formula (Task 085: scroll_y <= 40 ? scroll_y / 80 : 0.5 + (scroll_y - 40) / 160)
    let nav_bg_opacity = move || {
        let y = scroll_y_sig.get();
        if y <= 40.0 {
            y / 80.0
        } else {
            (0.5 + (y - 40.0) / 160.0).min(1.0)
        }
    };

    let scroll_style = move || {
        format!("background-color: rgba(20,20,20,{:.3});", nav_bg_opacity())
    };

    let location = use_location();
    let current_path = move || location.pathname.get();

    // Active tab identification based on route (Task 086)
    let active_tab = move || {
        let p = current_path();
        if p.starts_with("/browse/shows") || p.starts_with("/browse/series") {
            "tv"
        } else if p.starts_with("/browse/movies") || p.starts_with("/browse/films") {
            "movies"
        } else if p.starts_with("/latest") {
            "new"
        } else if p.starts_with("/browse/my-list") {
            "list"
        } else if p.starts_with("/browse/language") {
            "language"
        } else {
            "home"
        }
    };

    let nav_items = vec![
        ("home", "Home", "/browse"),
        ("tv", "Series", "/browse/shows"),
        ("movies", "Films", "/browse/movies"),
        ("new", "New & Popular", "/latest"),
        ("list", "My List", "/browse/my-list"),
        ("language", "Browse by Language", "/browse/language"),
    ];

    view! {
        <>
            // ── Desktop Primary Nav (Task 085 & 086) ──────────────────────────
            <nav
                class="fixed inset-x-0 top-0 z-[80] h-16 hidden sm:flex items-center px-14 transition-colors duration-200"
                style=scroll_style
            >
                // Logo — exact pstream-logo.svg matching React source
                <a href="/browse" class="flex-shrink-0">
                    <img
                        src="/assets/logos/pstream-logo.svg"
                        alt="Pstream"
                        class="h-[26px] cursor-pointer flex-shrink-0"
                    />
                </a>

                // Nav links — exact spacing and typography (Task 086)
                <ul class="flex items-center gap-5 ml-8 text-[14px] tracking-[-0.2px] font-normal text-[#e5e5e5]">
                    {nav_items.into_iter().map(|(id, label, href)| {
                        let is_active = move || active_tab() == id;
                        view! {
                            <li
                                class=move || if is_active() {
                                    "cursor-pointer transition-colors whitespace-nowrap text-white font-bold"
                                } else {
                                    "cursor-pointer transition-colors whitespace-nowrap hover:text-[#8c8c8c]"
                                }
                            >
                                <a href=href class="no-underline text-inherit">{label}</a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>

            // Subnav portal — sits flush below primary nav (z-[79])
            <div id="category-subnav-portal" class="fixed inset-x-0 top-16 z-[79] hidden sm:block" style=scroll_style />

            // ── Secondary nav: search + notifications + profile ─────────────
            <div class="fixed top-0 right-14 z-[85] h-16 hidden sm:flex items-center gap-4">
                // Expandable Search Bar (Task 089 & 090)
                <SearchBar />

                // Notifications Dropdown (Task 094)
                <NotificationsDropdown />

                // Profile Menu
                <crate::components::profiles::navbar_menu::NavbarProfileMenu />
            </div>

            // ── Mobile Header & Bottom Nav (Task 087 & 088) ───────────────────
            <NavbarMobile scroll_y=scroll_y_sig.get_untracked() />
        </>
    }
}
