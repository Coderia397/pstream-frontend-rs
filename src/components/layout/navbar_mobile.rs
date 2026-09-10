use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::components::layout::bottom_nav_mobile::{BottomNavMobile, BottomNavItem};
use crate::store::use_profile_store;

#[component]
pub fn NavbarMobile(
    #[prop(optional, default = 0.0)] scroll_y: f64,
    #[prop(optional)] search_query: Option<ReadSignal<String>>,
    #[prop(optional)] set_search_query: Option<WriteSignal<String>>,
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
    let scroll_style = format!("background-color: rgba(20,20,20,{:.3});", nav_bg_opacity);

    let nav_to_browse = navigate.clone();
    let nav_to_latest = navigate.clone();
    let nav_to_search = navigate.clone();
    let nav_to_settings = navigate.clone();

    let bottom_nav_items = vec![
        BottomNavItem {
            id: "home",
            icon: view! { <i class="ph-bold ph-house text-[22px]"></i> }.into_any(),
            label: "Home",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("home");
                nav_to_browse("/browse", Default::default());
            }),
        },
        BottomNavItem {
            id: "clips",
            icon: view! { <i class="ph-bold ph-play-circle text-[22px]"></i> }.into_any(),
            label: "Clips",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("clips");
                nav_to_latest("/latest", Default::default());
            }),
        },
        BottomNavItem {
            id: "search",
            icon: view! { <i class="ph-bold ph-magnifying-glass text-[22px]"></i> }.into_any(),
            label: "Search",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("search");
                nav_to_search("/search", Default::default());
            }),
        },
        BottomNavItem {
            id: "settings",
            icon: view! { 
                <div class="w-[22px] h-[22px] rounded overflow-hidden flex items-center justify-center bg-[#E50914] text-white font-bold text-[10px] ring-[1.5px] transition-all duration-300 ring-transparent">
                    <img src="https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg" alt="Profile" class="w-full h-full object-cover" />
                </div>
            }.into_any(),
            label: "My Netflix",
            on_click: Callback::new(move |_| {
                set_active_bottom_nav.set("settings");
                nav_to_settings("/browse/my-list", Default::default());
            }),
        },
    ];

    let current_avatar = move || {
        profile_store.profiles.get().first()
            .and_then(|p| p.avatar_url.clone())
            .unwrap_or_else(|| "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string())
    };

    view! {
        <div class="sm:hidden select-none">
            // ── Top Brand Header ───────────────────────────────────────────────
            <header
                class="fixed top-0 left-0 right-0 z-[80] px-4 pt-[calc(0.75rem+env(safe-area-inset-top))] pb-3 flex items-center justify-between pointer-events-auto transition-colors duration-200"
                style=scroll_style
            >
                // PSTREAM Emblem Logo matching React source
                <a href="/browse" class="flex-shrink-0 flex items-center gap-2">
                    <img
                        src="/assets/logos/p-pstream-logo.svg"
                        alt="Pstream Emblem Logo"
                        class="h-[38px] w-auto cursor-pointer select-none transition-transform active:scale-95"
                    />
                </a>

                // Action buttons: Cast & Profile Avatar
                <div class="flex items-center gap-4">
                    // Screencast icon (Task 087)
                    <button
                        class="text-white hover:text-gray-300 transition-colors p-1 cursor-pointer"
                        aria-label="Cast to TV"
                        title="Cast"
                    >
                        <i class="ph-bold ph-screencast text-[24px]"></i>
                    </button>

                    // Profile avatar button (Task 087)
                    <button
                        on:click=move |_| set_profile_sheet_open.update(|o| *o = !*o)
                        class="w-8 h-8 rounded overflow-hidden ring-1 ring-white/20 active:scale-95 transition-transform cursor-pointer"
                        aria-label="Profile menu"
                    >
                        <img
                            src=current_avatar
                            alt="Avatar"
                            class="w-full h-full object-cover"
                        />
                    </button>
                </div>
            </header>

            // ── Bottom Nav ───────────────────────────────────────────────────
            <BottomNavMobile items=bottom_nav_items active_id=active_bottom_nav />

            // ── Profile Bottom Sheet Modal ───────────────────────────────────
            <Show when=move || profile_sheet_open.get()>
                <div class="fixed inset-0 z-[10030]">
                    <div
                        on:click=move |_| set_profile_sheet_open.set(false)
                        class="absolute inset-0 bg-black/70 backdrop-blur-sm"
                    ></div>

                    <div class="absolute bottom-0 inset-x-0 bg-[#141414] border-t border-white/10 rounded-t-2xl max-h-[80vh] flex flex-col z-10 shadow-2xl animate-in slide-in-from-bottom duration-200">
                        <div class="flex items-center justify-between p-4 border-b border-white/10">
                            <span class="text-[18px] font-bold text-white">"Profiles & More"</span>
                            <button
                                on:click=move |_| set_profile_sheet_open.set(false)
                                class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-white/70 hover:text-white"
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
                                let av = p.avatar_url.unwrap_or_else(|| "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string());

                                view! {
                                    <button
                                        on:click=move |_| {
                                            profile_store.active_profile_id.set(Some(pid.clone()));
                                            set_profile_sheet_open.set(false);
                                        }
                                        class="flex items-center gap-3 p-2 rounded-lg hover:bg-white/5 transition-colors text-left w-full cursor-pointer"
                                    >
                                        <div
                                            class=if is_active {
                                                "w-10 h-10 rounded overflow-hidden shrink-0 border border-white ring-2 ring-white/50"
                                            } else {
                                                "w-10 h-10 rounded overflow-hidden shrink-0 border border-transparent"
                                            }
                                        >
                                            <img src=av alt=p_name.clone() class="w-full h-full object-cover" />
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <span class="text-white text-[15px] font-medium">{p_name}</span>
                                        </div>
                                        {if is_active {
                                            view! { <i class="ph-bold ph-check text-green-500 text-lg"></i> }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}

                            <div class="border-t border-white/10 my-2"></div>

                            // Quick settings links
                            <a
                                href="/browse/my-list"
                                class="flex items-center gap-3 p-2 text-white/80 hover:text-white transition-colors text-[15px]"
                                on:click=move |_| set_profile_sheet_open.set(false)
                            >
                                <i class="ph-bold ph-list-plus text-xl"></i>
                                <span>"My List"</span>
                            </a>
                            <a
                                href="/browse"
                                class="flex items-center gap-3 p-2 text-white/80 hover:text-white transition-colors text-[15px]"
                                on:click=move |_| set_profile_sheet_open.set(false)
                            >
                                <i class="ph-bold ph-gear text-xl"></i>
                                <span>"Account Settings"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
