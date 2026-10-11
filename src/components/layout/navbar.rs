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

    // Scroll opacity formula and transparent top-to-bottom gradient (Task 085)
    let scroll_style = move || {
        let y = scroll_y_sig.get();
        let op = if y <= 40.0 {
            y / 80.0
        } else {
            (0.5 + (y - 40.0) / 160.0).min(1.0)
        };
        // Netflix at the top of the page: black 80% to transparent over the full 80px height.
        let top_alpha = 0.80 * (1.0 - op);
        format!(
            "background: linear-gradient(to bottom, rgba(0,0,0,{:.3}) 0%, rgba(0,0,0,0) 100%), rgba(20,20,20,{:.3});",
            top_alpha, op
        )
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
        } else if p.starts_with("/browse/games") || p.starts_with("/games") {
            "games"
        } else if p.starts_with("/latest") {
            "new"
        } else if p.starts_with("/browse/my-list") {
            "list"
        } else if p.starts_with("/browse/language") {
            "language"
        } else if p == "/browse" || p == "/" || p.is_empty() {
            "home"
        } else {
            ""
        }
    };

    let nav_items = vec![
        ("home", "Home", "/browse"),
        ("tv", "Series", "/browse/series"),
        ("movies", "Films", "/browse/films"),
        ("games", "Games", "/browse/games"),
        ("new", "New & Popular", "/latest"),
        ("list", "My List", "/browse/my-list"),
        ("language", "Browse by language", "/browse/language"),
    ];

    view! {
        <>
            // ── Desktop Primary Nav (Task 085 & 086) ──────────────────────────
            <nav
                class="fixed inset-x-0 top-0 z-(--z-nav) h-(--nav-h) hidden sm:flex items-center px-[var(--app-x)] transition-colors duration-(--motion-base)"
                style=scroll_style
            >
                // Logo
                <a href="/browse" class="flex-shrink-0">
                    <img
                        src=crate::brand::LOGO_WORDMARK
                        alt=crate::brand::NAME
                        class="h-[28px] cursor-pointer flex-shrink-0"
                    />
                </a>

                // Nav links — exact spacing and typography (Task 086)
                <ul class="flex items-center gap-[6px] ml-6 text-body-md font-normal text-ink">
                    {nav_items.into_iter().map(|(id, label, href)| {
                        let is_active = move || active_tab() == id;
                        view! {
                            <li
                                class=move || if is_active() {
                                    "cursor-pointer transition-all duration-(--motion-base) whitespace-nowrap text-ink font-medium text-body-md rounded-full bg-ink/20 h-[40px] px-4 inline-flex items-center justify-center"
                                } else {
                                    "cursor-pointer transition-all duration-(--motion-base) whitespace-nowrap text-ink/70 hover:text-ink text-body-md rounded-full bg-transparent hover:bg-mute/40 h-[40px] px-4 font-normal inline-flex items-center justify-center"
                                }
                            >
                                <a href=href class="no-underline text-inherit flex items-center justify-center w-full h-full">{label}</a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>

            // ── Secondary nav: search + notifications + kids + profile ─────
            <div class="fixed top-0 right-[var(--app-x)] z-(--z-nav-menu) h-(--nav-h) hidden sm:flex items-center gap-3">
                // Expandable Search Bar (Task 089 & 090)
                <SearchBar />

                // Notifications Dropdown (Task 094)
                <NotificationsDropdown />

                // Kids Mode Toggle Button
                {move || {
                    let p_store = crate::store::use_profile_store();
                    let active_id = p_store.active_profile_id.get();
                    let is_kids = p_store.profiles.get().iter().any(|p| Some(&p.id) == active_id.as_ref() && p.is_kids);
                    if is_kids {
                        view! {
                            <button
                                on:click=move |_| {
                                    if let Some(main) = p_store.profiles.get().into_iter().find(|p| !p.is_kids) {
                                        p_store.active_profile_id.set(Some(main.id.clone()));
                                        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
                                            let _ = storage.set_item(crate::keys::ACTIVE_PROFILE, &main.id);
                                        }
                                    }
                                }
                                class="btn-brand px-3 py-1 rounded-control text-fine select-none"
                            >
                                {crate::i18n::t("navbar-exit-kids")}
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                on:click=move |_| {
                                    if let Some(kids) = p_store.profiles.get().into_iter().find(|p| p.is_kids) {
                                        p_store.active_profile_id.set(Some(kids.id.clone()));
                                        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
                                            let _ = storage.set_item(crate::keys::ACTIVE_PROFILE, &kids.id);
                                        }
                                    }
                                }
                                class="flex items-center gap-2 text-ink hover:text-ink transition-all select-none cursor-pointer group h-9 pl-0.5 pr-2 ml-1.5 rounded-field hover:bg-mute/40"
                                title=crate::i18n::t("navbar-switch-to-kids-profile")
                            >
                                <div class="w-8 h-8 rounded-control overflow-hidden flex items-center justify-center">
                                    <crate::components::profiles::kids_avatar::KidsAvatar size=32.0 />
                                </div>
                                <span class="text-label leading-[13px] font-normal text-ink">"Kids"</span>
                            </button>
                        }.into_any()
                    }
                }}

                // Profile Menu
                <crate::components::profiles::navbar_menu::NavbarProfileMenu />
            </div>

            // ── Mobile Header & Bottom Nav (Task 087 & 088) ───────────────────
            <NavbarMobile scroll_y=scroll_y_sig.get_untracked() />
        </>
    }
}
