use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use crate::components::layout::bottom_nav_mobile::{BottomNavMobile, BottomNavItem};
use crate::store::use_profile_store;

#[component]
pub fn NavbarMobile(
    #[prop(optional, default = 0.0)] scroll_y: f64,
    #[prop(optional)] _search_query: Option<ReadSignal<String>>,
    #[prop(optional)] _set_search_query: Option<WriteSignal<String>>,
) -> impl IntoView {
    let profile_store = use_profile_store();
    let navigate = use_navigate();
    
    let (active_bottom_nav, set_active_bottom_nav) = signal("home");
    let (profile_sheet_open, set_profile_sheet_open) = signal(false);

    let nav_bg_opacity = if scroll_y <= 40.0 {
        scroll_y / 80.0
    } else {
        (0.5 + (scroll_y - 40.0) / 160.0).min(1.0)
    };
    let top_alpha = 0.30 * (1.0 - nav_bg_opacity);
    let mid_alpha = 0.10 * (1.0 - nav_bg_opacity);
    let scroll_style = format!(
        "background: linear-gradient(to bottom, rgba(0,0,0,{:.3}) 0%, rgba(0,0,0,{:.3}) 60%, rgba(0,0,0,0) 100%), rgba(20,20,20,{:.3});",
        top_alpha, mid_alpha, nav_bg_opacity
    );

    // Page title next to the logo, like the app header ("Home", "Series", ...).
    let location = use_location();
    let page_title = move || {
        let p = location.pathname.get();
        if p.starts_with("/browse/my-list") { crate::i18n::t("nav-my-list") }
        else if p.starts_with("/browse/series") || p.starts_with("/browse/shows") { crate::i18n::t("nav-series") }
        else if p.starts_with("/browse/films") || p.starts_with("/browse/movies") { crate::i18n::t("nav-films") }
        else if p.starts_with("/browse/games") || p.starts_with("/games") { crate::i18n::t("nav-games") }
        else if p.starts_with("/latest") { crate::i18n::t("nav-new-hot") }
        else if p == "/browse" || p == "/" || p.is_empty() { crate::i18n::t("nav-home") }
        else { String::new() }
    };

    // Avatar of the active profile (first profile until the profile store knows better), used in the profile tab and the header
    let current_avatar = move || {
        profile_store.profiles.get().first()
            .and_then(|p| p.avatar_url.clone())
            .unwrap_or_else(|| crate::brand::DEFAULT_AVATAR.to_string())
    };

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
                class="fixed top-0 left-0 right-0 z-(--z-nav) pl-1 pr-1 pt-[env(safe-area-inset-top)] h-[calc(var(--nav-h-mobile)+env(safe-area-inset-top))] box-content flex items-center justify-between pointer-events-auto transition-colors duration-(--motion-base)"
                style=scroll_style
            >
                // Logo mark (48 x 44 box) and page title, as in the app header
                <div class="flex items-center min-w-0">
                    <a href="/browse" class="flex h-11 w-11 flex-shrink-0 items-center justify-center ms-1">
                        <img
                            src=crate::brand::LOGO_EMBLEM
                            alt=crate::brand::NAME
                            class="h-7 w-7 cursor-pointer select-none object-contain transition-transform active:scale-95"
                        />
                    </a>
                    <span class="ms-1 truncate text-heading font-bold leading-none tracking-tight text-ink">{page_title}</span>
                </div>

                // Action buttons: Cast & Profile Avatar
                <div class="flex items-center">
                    // Screencast icon (Task 087)
                    <button
                        class="w-11 h-11 flex items-center justify-center text-ink hover:text-gray-300 transition-colors cursor-pointer"
                        aria-label=crate::i18n::t("navbar-mobile-cast-to-tv")
                        title=crate::i18n::t("navbar-mobile-cast")
                    >
                        <i class="ph-bold ph-screencast text-heading"></i>
                    </button>

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
