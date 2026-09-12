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
                class="fixed inset-x-0 top-0 z-[80] h-16 hidden sm:flex items-center px-14 transition-colors duration-200 bg-gradient-to-b from-black/70 via-black/30 to-transparent"
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
                                    "cursor-pointer transition-all whitespace-nowrap text-white font-bold text-sm rounded-full border border-white/50 bg-[#262626]/70 px-4 py-1 shadow-sm"
                                } else {
                                    "cursor-pointer transition-colors whitespace-nowrap text-[#e5e5e5] hover:text-[#b3b3b3] text-sm px-1 py-1 font-normal"
                                }
                            >
                                <a href=href class="no-underline text-inherit">{label}</a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </nav>

            // Subnav portal — sits flush below primary nav (z-[79])
            <div id="category-subnav-portal" class="fixed inset-x-0 top-16 z-[79] hidden sm:block pointer-events-none transition-colors duration-200" style=scroll_style />

            // ── Secondary nav: search + notifications + kids + profile ─────
            <div class="fixed top-0 right-14 z-[85] h-16 hidden sm:flex items-center gap-5">
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
                                            let _ = storage.set_item("pstream_active_profile", &main.id);
                                        }
                                    }
                                }
                                class="px-3 py-1 rounded bg-[#e50914] hover:bg-[#f40612] text-white text-[12px] font-bold transition-colors select-none cursor-pointer"
                            >
                                "Exit Kids"
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                on:click=move |_| {
                                    if let Some(kids) = p_store.profiles.get().into_iter().find(|p| p.is_kids) {
                                        p_store.active_profile_id.set(Some(kids.id.clone()));
                                        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
                                            let _ = storage.set_item("pstream_active_profile", &kids.id);
                                        }
                                    }
                                }
                                class="flex items-center gap-2 text-white/90 hover:text-white transition-opacity select-none cursor-pointer group"
                                title="Switch to Kids Profile"
                            >
                                <div class="w-6 h-6 rounded-sm overflow-hidden flex items-center justify-center ring-1 ring-white/20 group-hover:ring-white/50 transition-all">
                                    <crate::components::profiles::kids_avatar::KidsAvatar size=24.0 />
                                </div>
                                <span class="text-sm font-medium text-white group-hover:text-white transition-colors">"Kids"</span>
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
