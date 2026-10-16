use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use crate::components::layout::bottom_nav_mobile::{BottomNavMobile, BottomNavItem};
use crate::store::use_profile_store;

#[component]
pub fn NavbarMobile(
    /// How far the page is scrolled; the header goes from transparent to solid with it
    #[prop(into)] scroll_y: Signal<f64>,
    #[prop(optional)] _search_query: Option<ReadSignal<String>>,
    #[prop(optional)] _set_search_query: Option<WriteSignal<String>>,
) -> impl IntoView {
    let profile_store = use_profile_store();
    let navigate = use_navigate();
    
    let (active_bottom_nav, set_active_bottom_nav) = signal("home");
    // the tab follows the page, also on a direct visit or Back
    let loc_tab = use_location();
    Effect::new(move |_| {
        let p = loc_tab.pathname.get();
        let tab = if p.starts_with("/search") { "search" } else if p.starts_with("/clips") || p.starts_with("/latest") { "clips" } else if p.starts_with("/browse/my-list") { "settings" } else { "home" };
        set_active_bottom_nav.set(tab);
    });
    let (profile_sheet_open, set_profile_sheet_open) = signal(false);

    // As in the app: a soft shade over the picture at the top, solid black (the colour of the pill row under it) by about 60px of scrolling
    let scroll_style = move || {
        let solid = (scroll_y.get() / 60.0).clamp(0.0, 1.0);
        format!(
            "background: linear-gradient(to bottom, rgba(0,0,0,{:.3}) 0%, rgba(0,0,0,{:.3}) 60%, rgba(0,0,0,0) 100%), rgba(0,0,0,{:.3});",
            0.45 * (1.0 - solid), 0.12 * (1.0 - solid), solid
        )
    };

    // Page title next to the logo, like the app header ("Home", "Series", ...).
    let location = use_location();
    let page_title = move || {
        let p = location.pathname.get();
        if p.starts_with("/browse/my-list") { crate::i18n::t("nav-my-list") }
        else if p.starts_with("/browse/series") || p.starts_with("/browse/shows") { crate::i18n::t("nav-series") }
        else if p.starts_with("/browse/films") || p.starts_with("/browse/movies") { crate::i18n::t("nav-films") }
        else if p.starts_with("/browse/games") || p.starts_with("/games") { crate::i18n::t("nav-games") }
        else if p.starts_with("/latest") { crate::i18n::t("nav-new-hot") }
        else if let Some(id) = p.strip_prefix("/browse/genre/").and_then(|r| r.split('/').next()).and_then(|r| r.parse::<u32>().ok()) {
            match id {
                crate::data::page_genres::SERIES_PAGE_ID => crate::i18n::t("nav-series"),
                crate::data::page_genres::FILMS_PAGE_ID => crate::i18n::t("nav-films"),
                n => crate::data::page_genres::genre_name(n).map(crate::i18n::genre_label).unwrap_or_default(),
            }
        }
        else if p == "/browse" || p == "/" || p.is_empty() { crate::i18n::t("nav-home") }
        else { String::new() }
    };

    // The pages under Home (Series, Films, a genre) have a back arrow instead of the logo and no icons on the right
    let on_sub_page = move || {
        let p = location.pathname.get();
        p.starts_with("/browse/series") || p.starts_with("/browse/shows") || p.starts_with("/browse/films") || p.starts_with("/browse/movies") || p.starts_with("/browse/genre/") || p.starts_with("/latest")
    };
    let on_search = move || location.pathname.get().starts_with("/search");
    let go_back = {
        let navigate = navigate.clone();
        Callback::new(move |_: ()| {
            let can_go_back = web_sys::window().and_then(|w| w.history().ok()).and_then(|h| h.length().ok()).is_some_and(|n| n > 1);
            if can_go_back { if let Some(h) = web_sys::window().and_then(|w| w.history().ok()) { let _ = h.back(); } } else { navigate("/browse", Default::default()); }
        })
    };

    // Avatar of the active profile (first profile until the profile store knows better), used in the profile tab and the header
    let current_avatar = move || {
        profile_store.profiles.get().first()
            .and_then(|p| p.avatar_url.clone())
            .unwrap_or_else(|| crate::brand::DEFAULT_AVATAR.to_string())
    };

    let unread = LocalResource::new(|| async { crate::components::layout::notifications_dropdown::unread_count().await });
    let nav_to_browse = navigate.clone();
    let nav_to_latest = navigate.clone();
    let nav_to_search = navigate.clone();
    let nav_to_settings = navigate.clone();

    let bottom_nav_items = vec![
        BottomNavItem {
            id: "home",
            icon: view! { <i class="ph-bold ph-house text-heading"></i> }.into_any(),
            label: "nav-home",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("home");
                nav_to_browse("/browse", Default::default());
            }),
        },
        BottomNavItem {
            id: "clips",
            icon: view! { <i class="ph-bold ph-play-circle text-heading"></i> }.into_any(),
            label: "nav-clips",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("clips");
                nav_to_latest("/latest", Default::default());
            }),
        },
        BottomNavItem {
            id: "search",
            icon: view! { <i class="ph-bold ph-magnifying-glass text-heading"></i> }.into_any(),
            label: "nav-search",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("search");
                nav_to_search("/search", Default::default());
            }),
        },
        BottomNavItem {
            id: "settings",
            icon: view! { 
                <div class="w-6 h-6 rounded-control overflow-hidden flex items-center justify-center bg-brand text-ink font-bold text-micro">
                    <img src=move || current_avatar() alt=crate::i18n::t("navbar-mobile-profile") class="w-full h-full object-cover" />
                </div>
            }.into_any(),
            label: "nav-my-list",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("settings");
                nav_to_settings("/browse/my-list", Default::default());
            }),
        },
    ];


    view! {
        <div class="sm:hidden select-none">
            // ── Top Brand Header ───────────────────────────────────────────────
            <header
                class:hidden=on_search
                class="fixed top-0 left-0 right-0 z-(--z-nav) pl-1 pr-1 pt-[env(safe-area-inset-top)] h-[calc(var(--nav-h-mobile)+env(safe-area-inset-top))] box-content flex items-center justify-between pointer-events-auto transition-colors duration-(--motion-base)"
                style=scroll_style
            >
                // Logo mark and page title on Home, a back arrow and the page title on the pages under it, as in the app header
                <div class="flex items-center min-w-0">
                    {move || if on_sub_page() {
                        view! {
                            <button
                                on:click=move |_| go_back.run(())
                                class="flex h-11 w-11 flex-shrink-0 items-center justify-center ms-1 text-ink active:scale-95 transition-transform cursor-pointer"
                                aria-label=crate::i18n::t("navbar-mobile-back")
                            >
                                <i class="ph-bold ph-arrow-left text-heading"></i>
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <a href="/browse" class="flex h-11 w-11 flex-shrink-0 items-center justify-center ms-1">
                                <img
                                    src=crate::brand::LOGO_EMBLEM
                                    alt=crate::brand::NAME
                                    class="h-7 w-7 cursor-pointer select-none object-contain transition-transform active:scale-95"
                                />
                            </a>
                        }.into_any()
                    }}
                    <span class="ms-1 truncate text-heading font-bold leading-none tracking-tight text-ink">{page_title}</span>
                </div>

                // Action buttons: notifications and the profile (on Home only, as in the app)
                <Show when=move || !on_sub_page()>
                <div class="flex items-center">
                    // Notifications (the bell of the app header)
                    <a
                        href="/notifications"
                        class="relative w-11 h-11 flex items-center justify-center text-ink active:scale-95 transition-transform cursor-pointer"
                        aria-label=crate::i18n::t("navbar-mobile-notifications")
                    >
                        <i class="ph-bold ph-bell text-heading"></i>
                        {move || unread.get().filter(|n| *n > 0).map(|n| view! {
                            <span class="absolute top-1 right-0.5 min-w-[18px] h-[18px] px-1 rounded-full bg-red-600 text-white text-micro font-semibold flex items-center justify-center leading-none">
                                {if n > 99 { "99+".to_string() } else { n.to_string() }}
                            </span>
                        })}
                    </a>

                    // Profile avatar button (Task 087)
                    <button
                        on:click=move |_| set_profile_sheet_open.update(|o| *o = !*o)
                        class="w-11 h-11 flex items-center justify-center active:scale-95 transition-transform cursor-pointer"
                        aria-label=crate::i18n::t("navbar-mobile-profile-menu")
                    >
                        <img
                            src=current_avatar
                            alt=crate::i18n::t("navbar-mobile-avatar")
                            class="w-8 h-8 rounded-control object-cover"
                        />
                    </button>
                </div>
                </Show>
            </header>

            // ── Bottom Nav ───────────────────────────────────────────────────
            <BottomNavMobile items=bottom_nav_items active_id=active_bottom_nav />

            // ── Profile Bottom Sheet Modal ───────────────────────────────────
            <Show when=move || profile_sheet_open.get()>
                <div class="fixed inset-0 z-(--z-drawer)">
                    <div
                        on:click=move |_| set_profile_sheet_open.set(false)
                        class="absolute inset-0 bg-void/70 backdrop-blur-sm"
                    ></div>

                    <div class="absolute bottom-0 inset-x-0 bg-page border-t border-ink/10 rounded-t-sheet max-h-[80vh] flex flex-col z-10 shadow-2xl animate-in slide-in-from-bottom duration-(--motion-base)">
                        <div class="flex items-center justify-between p-4 border-b border-ink/10">
                            <span class="text-title-sm font-bold text-ink">{crate::i18n::t("nav-profiles-more")}</span>
                            <button
                                on:click=move |_| set_profile_sheet_open.set(false)
                                class="w-8 h-8 rounded-full bg-ink/10 flex items-center justify-center text-ink/70 hover:text-ink"
                            >
                                <i class="ph-bold ph-x text-base"></i>
                            </button>
                        </div>

                        // Profile tiles
                        <div class="p-4 flex flex-col gap-3">
                            {move || profile_store.profiles.get().into_iter().map(|p| {
                                let pid = p.id.clone();
                                let is_active = profile_store.active_profile_id.get() == Some(pid.clone());
                                let p_name = p.name.clone();
                                let av = p.avatar_url.unwrap_or_else(|| crate::brand::DEFAULT_AVATAR.to_string());

                                view! {
                                    <button
                                        on:click=move |_| {
                                            profile_store.active_profile_id.set(Some(pid.clone()));
                                            set_profile_sheet_open.set(false);
                                        }
                                        class="flex items-center gap-3 p-2 rounded-card hover:bg-ink/5 transition-colors text-left w-full cursor-pointer"
                                    >
                                        <div
                                            class=if is_active {
                                                "w-10 h-10 rounded-control overflow-hidden shrink-0 border border-ink ring-2 ring-ink/50"
                                            } else {
                                                "w-10 h-10 rounded-control overflow-hidden shrink-0 border border-transparent"
                                            }
                                        >
                                            <img src=av alt=p_name.clone() class="w-full h-full object-cover" />
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <span class="text-ink text-body font-medium">{p_name}</span>
                                        </div>
                                        {if is_active {
                                            view! { <i class="ph-bold ph-check text-green-500 text-lg"></i> }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}

                            <div class="border-t border-ink/10 my-2"></div>

                            // Quick settings links
                            <a
                                href="/browse/my-list"
                                class="flex items-center gap-3 p-2 text-ink/80 hover:text-ink transition-colors text-body"
                                on:click=move |_| set_profile_sheet_open.set(false)
                            >
                                <i class="ph-bold ph-list-plus text-xl"></i>
                                <span>{crate::i18n::t("nav-my-list")}</span>
                            </a>
                            <a
                                href="/settings"
                                class="flex items-center gap-3 p-2 text-ink/80 hover:text-ink transition-colors text-body"
                                on:click=move |_| set_profile_sheet_open.set(false)
                            >
                                <i class="ph-bold ph-gear text-xl"></i>
                                <span>{crate::i18n::t("nav-account-settings")}</span>
                            </a>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
