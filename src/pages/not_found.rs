use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::components::layout::navbar::Navbar;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DimensionConfig {
    pub id: &'static str,
    pub title: &'static str,
    pub backdrop: &'static str,
    pub poster: &'static str,
    pub title_img: &'static str,
    pub subtitle: &'static str,
    pub synopsis: &'static str,
    pub error_code: &'static str,
}

pub const DIMENSIONS: [DimensionConfig; 6] = [
    DimensionConfig {
        id: "dim1",
        title: "404: The Silicon Void",
        backdrop: "/assets/404_assets/dim1-backdrop.jpeg",
        poster: "/assets/404_assets/dim1-poster.jpeg",
        title_img: "/assets/404_assets/dim1-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "Deep inside the motherboard, a glowing red pixel cube falls into a dark vortex, watched by a silhouetted crimson eye peeking from the shadows of a torn canvas.",
        error_code: "NSES-404-CUBE",
    },
    DimensionConfig {
        id: "dim2",
        title: "404: Offline Signal",
        backdrop: "/assets/404_assets/dim2-backdrop.jpeg",
        poster: "/assets/404_assets/dim2-poster.jpeg",
        title_img: "/assets/404_assets/dim2-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "Row after row of velvet cinema seats sit completely empty. On stage, a broken retro CRT television flickers with color-bars and signal static under a glitchy sky.",
        error_code: "NSES-404-SIGNAL",
    },
    DimensionConfig {
        id: "dim3",
        title: "404: Shattered Clues",
        backdrop: "/assets/404_assets/dim3-backdrop.jpeg",
        poster: "/assets/404_assets/dim3-poster.jpeg",
        title_img: "/assets/404_assets/dim3-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "A lone detective in a trench coat searches a dark room, tracking a missing web asset that shattered into a thousand shards of glass. Every lead is a broken redirect.",
        error_code: "NSES-404-SHATTER",
    },
    DimensionConfig {
        id: "dim4",
        title: "404: Lost Reels",
        backdrop: "/assets/404_assets/dim4-backdrop.jpeg",
        poster: "/assets/404_assets/dim4-poster.jpeg",
        title_img: "/assets/404_assets/dim4-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "A massive, breathing grid of forgotten movie reels pulses in high-contrast red. As you search the archive, the celluloid strip snaps, leaving you in the cinematic dark.",
        error_code: "NSES-404-REEL",
    },
    DimensionConfig {
        id: "dim5",
        title: "404: The Watcher",
        backdrop: "/assets/404_assets/dim5-backdrop.jpeg",
        poster: "/assets/404_assets/dim5-poster.jpeg",
        title_img: "/assets/404_assets/dim5-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "A glowing digital projector beam cuts through a swirling black hole, shining through a shattered lens that stares back like a towering mechanical watcher.",
        error_code: "NSES-404-EYE",
    },
    DimensionConfig {
        id: "dim6",
        title: "404: Cosmic Drift",
        backdrop: "/assets/404_assets/dim6-backdrop.jpeg",
        poster: "/assets/404_assets/dim6-poster.jpeg",
        title_img: "/assets/404_assets/dim6-title.png",
        subtitle: "AN ERROR 404 ORIGINAL",
        synopsis: "Adrift in a cosmic cloud, ancient space cables float through a stellar nebula, trying to route packets through a tear in the space-time fabric. Connection timed out.",
        error_code: "NSES-404-NEBULA",
    },
];

pub const GLITCH_ZONES: [&str; 6] = [
    "Hardware Glitches",
    "Signal Failures",
    "Lost Data Files",
    "Analog Corruption",
    "Void Sentinels",
    "Cosmic Drift",
];

#[component]
pub fn NotFoundPage() -> impl IntoView {
    let navigate = use_navigate();
    let (active_index, set_active_index) = signal(0usize);
    let (is_drawer_open, set_is_drawer_open) = signal(false);
    let (dropdown_open, set_dropdown_open) = signal(false);

    let active = move || DIMENSIONS[active_index.get()];

    let current_path = move || {
        web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_else(|| "/404".to_string())
    };

    let user_agent = move || {
        web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .unwrap_or_else(|| "Unknown agent".to_string())
    };

    let iso_now = move || {
        js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default()
    };

    let scroll_to_top = move || {
        if let Some(win) = web_sys::window() {
            win.scroll_to_with_x_and_y(0.0, 0.0);
        }
    };

    view! {
        <div class="relative min-h-screen bg-[#141414] text-white overflow-x-hidden select-none pb-16">
            // Primary Navbar
            <Navbar />

            // CRT scanlines overlay
            <div class="fixed inset-0 pointer-events-none z-[60] opacity-[0.25] mix-blend-overlay bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.3)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] bg-[size:100%_4px,6px_100%]" />

            // SubNav Header: "Alternate Dimensions" + "Glitch Zones ▾"
            <div class="pt-20 px-6 md:px-14 lg:px-16 flex items-center gap-6 z-30 relative">
                <h1 class="text-3xl md:text-4xl font-bold tracking-tight text-white">
                    "Alternate Dimensions"
                </h1>

                // Glitch Zones dropdown
                <div class="relative">
                    <button
                        on:click=move |_| set_dropdown_open.update(|o| *o = !*o)
                        class="flex items-center gap-2 bg-black/80 hover:bg-black border border-white/70 px-4 py-1.5 text-sm font-bold text-white rounded transition-colors"
                    >
                        <span>{move || GLITCH_ZONES[active_index.get()]}</span>
                        <i class="ph-bold ph-caret-down text-xs" />
                    </button>

                    {move || if dropdown_open.get() {
                        view! {
                            <div class="absolute left-0 top-full mt-2 w-48 bg-black/95 border border-white/20 rounded shadow-2xl py-1 z-50">
                                {GLITCH_ZONES.iter().enumerate().map(|(idx, &name)| {
                                    let is_active = move || active_index.get() == idx;
                                    view! {
                                        <button
                                            on:click=move |_| {
                                                set_active_index.set(idx);
                                                set_dropdown_open.set(false);
                                                scroll_to_top();
                                            }
                                            class=move || {
                                                if is_active() {
                                                    "w-full text-left px-4 py-2 text-xs font-bold text-white bg-white/20 hover:bg-white/30 transition-colors"
                                                } else {
                                                    "w-full text-left px-4 py-2 text-xs font-medium text-white/80 hover:bg-white/10 hover:text-white transition-colors"
                                                }
                                            }
                                        >
                                            {name}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }}
                </div>
            </div>

            // Dynamic Banner Backdrop
            <div class="relative h-[60vh] sm:h-[70vh] md:h-[80vh] w-full overflow-hidden bg-black mt-4">
                <div class="absolute inset-0 z-0 pointer-events-none transition-all duration-700">
                    <img
                        src=move || active().backdrop
                        class="w-full h-full object-cover opacity-70 scale-105 transition-all duration-1000 ease-out"
                        alt="Cinematic Background"
                    />
                    <div class="absolute inset-0 bg-gradient-to-b from-black/80 via-black/15 to-transparent" />
                    <div class="absolute inset-0 bg-gradient-to-t from-[#141414] via-[#141414]/30 to-transparent" />
                    <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,transparent_20%,#000000_90%)] opacity-65" />
                </div>

                // Hero Content
                <div class="absolute top-0 left-0 w-full h-full flex flex-col justify-end z-20 pl-[calc(1.5rem+env(safe-area-inset-left,0px))] md:pl-14 lg:pl-16 pr-4 md:pr-12 pointer-events-none pb-[12%] sm:pb-[9%] md:pb-[6%]">
                    <div class="max-w-[95%] sm:max-w-lg md:max-w-xl lg:max-w-2xl space-y-4 md:space-y-5 pointer-events-auto">
                        <div class="text-[10px] sm:text-xs font-bold tracking-[0.25em] text-red-500 drop-shadow-[0_2px_8px_rgba(229,9,20,0.4)]">
                            {move || active().subtitle}
                        </div>

                        // Neon 404 Title Image with layers
                        <div class="relative flex items-end mb-2 md:mb-4">
                            <div class="relative inline-flex items-end">
                                <img
                                    src=move || active().title_img
                                    aria-hidden="true"
                                    class="absolute object-contain object-bottom select-none opacity-45 blur-[20px] scale-[1.06]"
                                    style="filter: brightness(1.5) sepia(1) hue-rotate(-50deg); width: 100%; height: 100%; inset: 0; mix-blend-mode: screen;"
                                />
                                <img
                                    src=move || active().title_img
                                    aria-hidden="true"
                                    class="absolute object-contain object-bottom select-none opacity-60 blur-[4px] scale-[1.01]"
                                    style="filter: brightness(1.2) sepia(1) hue-rotate(-50deg); width: 100%; height: 100%; inset: 0; mix-blend-mode: screen;"
                                />
                                <img
                                    src=move || active().title_img
                                    alt=move || active().title
                                    class="relative object-contain object-bottom select-none animate-pulse duration-[3000ms]"
                                    style="max-height: clamp(80px, 18vw, 190px); max-width: clamp(250px, 55vw, 550px); mix-blend-mode: screen;"
                                />
                            </div>
                        </div>

                        <p class="text-[12px] sm:text-[13px] md:text-[15px] font-medium text-white/90 line-clamp-3 leading-relaxed max-w-[90%] sm:max-w-lg transition-all duration-700 drop-shadow-md">
                            {move || active().synopsis}
                        </p>

                        <div class="flex items-center flex-wrap gap-2 md:gap-3 pt-2">
                            <button
                                on:click={
                                    let nav = navigate.clone();
                                    move |_| nav("/browse", Default::default())
                                }
                                class="flex items-center justify-center bg-white text-black px-6 sm:px-8 h-[36px] md:h-[42px] rounded-[4px] font-bold hover:bg-white/80 transition-colors text-[14px] md:text-[17px] gap-2 active:scale-95 shadow-lg duration-200"
                            >
                                <i class="ph-fill ph-house text-xl text-black" />
                                <span>"Return to Home"</span>
                            </button>

                            <button
                                on:click=move |_| set_is_drawer_open.set(true)
                                class="flex items-center justify-center bg-[#6d6d6e]/80 text-white px-5 sm:px-8 h-[36px] md:h-[42px] rounded-[4px] font-bold hover:bg-[#6d6d6e]/60 transition-all text-[14px] md:text-[17px] gap-2 active:scale-95 shadow-lg duration-200"
                            >
                                <i class="ph-bold ph-info text-xl" />
                                <span>"More Info"</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            // Alternate Dimensions Row (Horizontal 16:9 cards)
            <main class="relative z-10 -mt-8 sm:-mt-12 md:-mt-16 space-y-4 md:space-y-6 px-6 md:px-14 lg:px-16">
                <h2 class="text-white text-xl md:text-2xl font-black mb-4">
                    "Alternate Dimensions: More Like This"
                </h2>
                <div class="flex gap-3 overflow-x-auto pb-4 scrollbar-hide py-2">
                    {DIMENSIONS.iter().enumerate().map(|(idx, dim)| {
                        let is_sel = move || active_index.get() == idx;
                        view! {
                            <div
                                on:click=move |_| {
                                    set_active_index.set(idx);
                                    scroll_to_top();
                                }
                                class=move || {
                                    format!(
                                        "relative flex-none w-[200px] sm:w-[240px] md:w-[280px] aspect-video rounded-md overflow-hidden cursor-pointer transition-all duration-300 transform hover:scale-105 border-2 bg-zinc-900 {}",
                                        if is_sel() { "border-red-600 shadow-[0_0_15px_rgba(229,9,20,0.6)]" } else { "border-transparent hover:border-white/30" }
                                    )
                                }
                            >
                                <img
                                    src=dim.backdrop
                                    alt=dim.title
                                    class="w-full h-full object-cover"
                                    loading="lazy"
                                />
                                <div class="absolute inset-0 bg-black/20 hover:bg-black/0 transition-colors flex items-center justify-center p-4">
                                    <img
                                        src=dim.title_img
                                        alt=dim.title
                                        class="max-h-[70%] max-w-[85%] object-contain select-none filter drop-shadow-[0_2px_8px_rgba(0,0,0,0.8)]"
                                        style="mix-blend-mode: screen;"
                                    />
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </main>

            // Technical Details Drawer
            {move || if is_drawer_open.get() {
                view! {
                    <div class="fixed inset-0 z-[100] flex items-center justify-end bg-black/60 backdrop-blur-sm">
                        <div
                            class="absolute inset-0"
                            on:click=move |_| set_is_drawer_open.set(false)
                        />
                        <div class="relative w-full max-w-md sm:max-w-lg h-full bg-[#181818]/95 border-l border-white/10 p-6 sm:p-8 flex flex-col justify-between shadow-[0_0_50px_rgba(0,0,0,0.8)] z-10">
                            <div class="space-y-6">
                                <div class="flex items-center justify-between border-b border-white/10 pb-4">
                                    <div class="flex items-center gap-3">
                                        <i class="ph-fill ph-warning text-red-500 text-2xl" />
                                        <h3 class="text-xl sm:text-2xl font-bold tracking-tight text-white font-sans">
                                            "Diagnostic Report"
                                        </h3>
                                    </div>
                                    <button
                                        on:click=move |_| set_is_drawer_open.set(false)
                                        class="p-1 rounded-full hover:bg-white/10 text-white/60 hover:text-white transition-colors"
                                    >
                                        <i class="ph-bold ph-x text-2xl" />
                                    </button>
                                </div>

                                <div class="space-y-4 text-sm font-sans text-white/80">
                                    <div class="space-y-1">
                                        <span class="text-xs text-white/40 uppercase tracking-wider">"System Code"</span>
                                        <p class="text-base font-mono text-red-500 font-semibold">{move || active().error_code}</p>
                                    </div>
                                    <div class="space-y-1">
                                        <span class="text-xs text-white/40 uppercase tracking-wider">"Requested Endpoint"</span>
                                        <p class="text-base font-mono bg-black/40 px-3 py-1.5 rounded border border-white/5 whitespace-nowrap overflow-x-auto select-text">
                                            {current_path()}
                                        </p>
                                    </div>
                                    <div class="space-y-1">
                                        <span class="text-xs text-white/40 uppercase tracking-wider">"Incident Timestamp"</span>
                                        <p class="text-base font-mono">{iso_now()}</p>
                                    </div>
                                    <div class="space-y-1">
                                        <span class="text-xs text-white/40 uppercase tracking-wider">"Platform Agent"</span>
                                        <p class="text-xs font-mono text-white/50 bg-black/20 p-2 rounded leading-normal border border-white/5 max-h-24 overflow-y-auto select-text">
                                            {user_agent()}
                                        </p>
                                    </div>
                                </div>

                                <div class="bg-red-500/10 border border-red-500/20 rounded p-4 text-[13px] leading-relaxed text-red-300 font-sans">
                                    <strong>"System Notice: "</strong>
                                    "The targeted router endpoint has been de-referenced or has failed to load from our CDN space. Please redirect back to our secure home streaming environment to resume playback."
                                </div>
                            </div>

                            <button
                                on:click={
                                    let nav = navigate.clone();
                                    move |_| {
                                        set_is_drawer_open.set(false);
                                        nav("/browse", Default::default());
                                    }
                                }
                                class="w-full py-3.5 bg-red-600 hover:bg-red-700 text-white font-bold rounded flex items-center justify-center gap-2 transition-colors duration-200 active:scale-[0.98] shadow-lg font-sans text-sm sm:text-base mt-8"
                            >
                                <i class="ph-bold ph-caret-left text-xl" />
                                <span>"Return to Home"</span>
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}
