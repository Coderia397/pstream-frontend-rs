use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

// ── Size constants ────────────────────────────────────────────────────────────

const CARD_CLASS: &str = "h-[110px] w-[75px] sm:h-[140px] sm:w-[95px] md:h-[170px] md:w-[115px] lg:h-[200px] lg:w-[135px]";

// ── RankNumber SVG ────────────────────────────────────────────────────────────

#[component]
fn RankNumber(index: usize) -> impl IntoView {
    let is_ten = index == 9;
    let viewbox = if is_ten { "0 0 280 210" } else { "0 0 200 210" };
    let transform = if is_ten { "scale(1.0, 0.98)" } else { "scale(1.1, 1.0)" };
    let letter_spacing = if is_ten { "-22" } else { "-6" };
    let width_style = if is_ten { "width:60%" } else { "width:50%" };
    let left_class = "absolute bottom-[8%] h-[48%] z-20 pointer-events-none overflow-visible drop-shadow-[0_0_8px_rgba(0,0,0,0.8)] left-[-20px] sm:left-[-26px] md:left-[-34px] lg:left-[-42px]";
    let num = (index + 1).to_string();

    view! {
        <div class=left_class style=width_style>
            <svg
                viewBox=viewbox
                class="h-full w-auto"
                preserve-aspect-ratio="xMinYMid meet"
                style="overflow:visible"
            >
                <g transform=transform style="transform-origin:0px 205px">
                    <text x="8" y="195" text-anchor="start" dominant-baseline="auto"
                        fill="none" stroke="#737373" stroke-width="12" stroke-linejoin="round"
                        font-size="180" font-weight="900"
                        font-family="'Inter', sans-serif" letter-spacing=letter_spacing>
                        {num.clone()}
                    </text>
                    <text x="8" y="195" text-anchor="start" dominant-baseline="auto"
                        fill="#090909" stroke="#090909" stroke-width="4" stroke-linejoin="round"
                        font-size="180" font-weight="900"
                        font-family="'Inter', sans-serif" letter-spacing=letter_spacing>
                        {num}
                    </text>
                </g>
            </svg>
        </div>
    }
}

// ── LandingTopTenCard ─────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct TopMovie {
    pub id: i64,
    pub title: String,
    pub poster_path: String,
}

#[component]
fn LandingTopTenCard(movie: TopMovie, index: usize) -> impl IntoView {
    let poster_src = if movie.poster_path.starts_with("http") {
        movie.poster_path.clone()
    } else {
        format!("https://image.tmdb.org/t/p/w780{}", movie.poster_path)
    };
    let alt = movie.title.clone();

    view! {
        <div class=format!(
            "relative flex-none {} ml-6 sm:ml-8 md:ml-10 lg:ml-12 mr-4 md:mr-6 flex items-end select-none z-10 cursor-default",
            CARD_CLASS
        )>
            <RankNumber index=index />
            <div class="absolute right-0 bottom-0 w-full h-full z-10 rounded-lg overflow-hidden shadow-[0_0_15px_rgba(0,0,0,0.5)]">
                <img
                    src=poster_src
                    class="w-full h-full object-cover object-top"
                    alt=alt
                    loading="lazy"
                />
                <div class="absolute inset-0 bg-black/10 pointer-events-none" />
            </div>
        </div>
    }
}

// ── LandingTopTenRow ──────────────────────────────────────────────────────────

#[component]
fn LandingTopTenRow(title: &'static str) -> impl IntoView {
    let movies = LocalResource::new(move || async move {
        crate::services::tmdb::fetch_trending("all").await.unwrap_or_default()
    });

    let scroll_left = Signal::derive(|| false);
    let scroll_right = Signal::derive(|| true);

    view! {
        <div class="relative group/row">
            <h2 class="text-white text-xl md:text-2xl font-black mb-4 px-6 md:px-20 lg:px-32 xl:px-44 2xl:px-56">
                {title}
            </h2>
            <div class="relative mx-6 md:mx-20 lg:mx-32 xl:mx-44 2xl:mx-56">
                // Left arrow — only when scrolled right
                {move || if scroll_left.get() {
                    view! {
                        <button class="absolute left-0 top-0 bottom-0 z-20 w-12 md:w-16 flex items-center justify-center bg-gradient-to-r from-black/90 to-transparent opacity-0 group-hover/row:opacity-100 transition-opacity">
                            <i class="ph-bold ph-caret-left text-4xl text-white drop-shadow-lg" />
                        </button>
                    }.into_any()
                } else { view! {<></>}.into_any() }}
                // Right arrow
                {move || if scroll_right.get() {
                    view! {
                        <button class="absolute right-0 top-0 bottom-0 z-20 w-12 md:w-16 flex items-center justify-center bg-gradient-to-l from-black/90 to-transparent opacity-0 group-hover/row:opacity-100 transition-opacity">
                            <i class="ph-bold ph-caret-right text-4xl text-white drop-shadow-lg" />
                        </button>
                    }.into_any()
                } else { view! {<></>}.into_any() }}

                <Suspense fallback=move || view! {
                    <div class="flex gap-2 overflow-x-auto overflow-y-hidden py-4 -my-4" style="scrollbar-width:none">
                        {(0..6).map(|_| view! {
                            <div class=format!("{} flex-none bg-[#333] rounded animate-pulse", CARD_CLASS) />
                        }).collect::<Vec<_>>()}
                    </div>
                }>
                    {move || {
                        movies.get().map(|data| {
                            let items: Vec<_> = data.into_iter().take(10).enumerate()
                                .map(|(i, m)| {
                                    let movie = TopMovie {
                                        id: m.id as i64,
                                        title: m.display_title().to_string(),
                                        poster_path: m.poster_path.clone().unwrap_or_default(),
                                    };
                                    view! { <LandingTopTenCard movie=movie index=i /> }
                                }).collect();
                            view! {
                                <div
                                    class="flex gap-0 overflow-x-auto overflow-y-hidden scroll-smooth py-4 -my-4"
                                    style="scrollbar-width:none;-ms-overflow-style:none"
                                >
                                    {items}
                                </div>
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

// ── Feature Icons ─────────────────────────────────────────────────────────────

#[component]
fn FeatureIcon(id: &'static str) -> impl IntoView {
    match id {
        "tv" => view! {
            <div class="relative w-20 h-16 flex flex-col items-center justify-end">
                <div class="absolute top-0 w-16 h-12 rounded bg-gradient-to-br from-[#ff1f75]/20 to-[#8000ff]/20 blur-md pointer-events-none" />
                <div class="w-16 h-11 bg-[#101018] rounded-t-md border-2 border-[#2b2b3d] p-[1.5px] relative overflow-hidden flex items-center justify-center shadow-lg">
                    <div class="w-full h-full rounded-[2px] bg-gradient-to-tr from-[#9900ff] via-[#ff007c] to-[#ff8c00] opacity-80 relative">
                        <div class="absolute inset-0 bg-gradient-to-b from-white/20 via-transparent to-transparent rotate-12 scale-150 transform -translate-y-4" />
                    </div>
                </div>
                <div class="w-2.5 h-2 bg-gradient-to-b from-[#2b2b3d] to-[#12121c] border-x border-[#3f3f5a]" />
                <div class="w-10 h-[3px] bg-gradient-to-r from-[#ff0055] via-[#ff7700] to-[#ff0055] rounded-full shadow-md" />
            </div>
        }.into_any(),

        "stream" => view! {
            <div class="relative w-16 h-16 flex items-center justify-center">
                <div class="absolute inset-0 rounded-full bg-gradient-to-br from-[#ff007f]/30 to-[#7b2cbf]/30 blur-md pointer-events-none" />
                <div class="w-14 h-14 rounded-full bg-gradient-to-tr from-[#7b2cbf] via-[#d90429] to-[#ff007f] p-[1.5px] shadow-[0_4px_12px_rgba(0,0,0,0.5)] relative flex items-center justify-center overflow-hidden">
                    <div class="absolute inset-0 bg-[radial-gradient(circle_at_30%_30%,rgba(255,255,255,0.4)_0%,transparent_60%)]" />
                    <svg class="w-6 h-6 text-white relative z-10 drop-shadow-[0_2px_4px_rgba(0,0,0,0.4)]" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M12 4V16M12 16L6 10M12 16L18 10" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
                        <path d="M4 20H20" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" />
                    </svg>
                </div>
            </div>
        }.into_any(),

        "subtitles" => view! {
            <div class="relative w-16 h-16 flex items-center justify-center">
                <div class="absolute -top-1 right-2 text-[#ff0055] text-xs animate-pulse font-bold">"✦"</div>
                <div class="absolute bottom-2 -left-1 text-[#ff7700] text-[10px] animate-pulse font-bold">"✦"</div>
                <div class="absolute top-4 -left-2 text-[#ff0055] text-[8px] animate-pulse font-bold">"✦"</div>
                <div class="absolute inset-2 rounded-full bg-[#ff007f]/10 blur-lg pointer-events-none" />
                <svg class="w-14 h-14 relative z-10" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <defs>
                        <linearGradient id="scopeGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                            <stop offset="0%" stop-color="#ff007f" />
                            <stop offset="50%" stop-color="#7e0fff" />
                            <stop offset="100%" stop-color="#ff5500" />
                        </linearGradient>
                        <linearGradient id="standGrad" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" stop-color="#555577" />
                            <stop offset="100%" stop-color="#222233" />
                        </linearGradient>
                    </defs>
                    <line x1="32" y1="36" x2="20" y2="58" stroke="url(#standGrad)" stroke-width="3" stroke-linecap="round" />
                    <line x1="32" y1="36" x2="44" y2="58" stroke="url(#standGrad)" stroke-width="3" stroke-linecap="round" />
                    <line x1="32" y1="36" x2="32" y2="55" stroke="url(#standGrad)" stroke-width="3.5" stroke-linecap="round" />
                    <circle cx="32" cy="36" r="4.5" fill="#3f3f5a" stroke="#fff" stroke-width="1" />
                    <g transform="rotate(-30 32 32)">
                        <rect x="42" y="24" width="6" height="16" rx="1" fill="#fff" />
                        <path d="M16 26H42V38H16C15 38 14 37 14 36V28C14 27 15 26 16 26Z" fill="url(#scopeGrad)" stroke="#fff" stroke-width="1" />
                        <rect x="22" y="25" width="3" height="14" fill="#ffea00" />
                        <rect x="6" y="29" width="8" height="6" fill="#222" stroke="#fff" stroke-width="1" />
                        <path d="M2 28H6V36H2C1 36 0 35 0 34V30C0 29 1 28 2 28Z" fill="#555" />
                    </g>
                </svg>
            </div>
        }.into_any(),

        _ => view! { // "place"
            <div class="relative w-16 h-16 flex items-center justify-center">
                <div class="absolute inset-0 rounded-full bg-gradient-to-br from-[#ff0055]/10 to-[#ff500f]/10 blur-md pointer-events-none" />
                // Back card (beige)
                <div class="absolute top-2 left-2 w-10 h-10 rounded-lg bg-gradient-to-br from-[#ffd2a0] to-[#ff9f68] border border-white/20 shadow-md flex flex-col items-center justify-center transform -rotate-6">
                    <div class="flex gap-2.5 mb-1">
                        <div class="w-1.5 h-1.5 rounded-full bg-[#3d1e03]" />
                        <div class="w-1.5 h-1.5 rounded-full bg-[#3d1e03]" />
                    </div>
                    <svg class="w-3.5 h-1.5 text-[#3d1e03]" viewBox="0 0 24 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M2 2C6 9 18 9 22 2" stroke="currentColor" stroke-width="4" stroke-linecap="round" />
                    </svg>
                </div>
                // Front card (red/magenta)
                <div class="absolute bottom-2 right-2 w-11 h-11 rounded-lg bg-gradient-to-br from-[#ff0055] to-[#ff500f] border border-white/30 shadow-lg flex flex-col items-center justify-center transform rotate-6">
                    <div class="flex gap-3 mb-1">
                        <div class="w-1.5 h-1.5 rounded-full bg-white" />
                        <div class="w-1.5 h-1.5 rounded-full bg-white" />
                    </div>
                    <svg class="w-4 h-2 text-white" viewBox="0 0 24 12" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M2 2C6 9 18 9 22 2" stroke="currentColor" stroke-width="4" stroke-linecap="round" />
                    </svg>
                </div>
            </div>
        }.into_any(),
    }
}

// ── FeatureCard ───────────────────────────────────────────────────────────────

#[component]
fn FeatureCard(id: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-[#1A2144] from-40% to-[#30181b] rounded-2xl p-5 md:p-7 flex flex-col justify-between min-h-[175px] md:min-h-[210px] relative overflow-hidden group transition-all duration-300">
            <div>
                <h3 class="text-white text-xl md:text-2xl font-black mb-2.5 leading-tight">{title}</h3>
                <p class="text-zinc-400 text-sm md:text-base font-normal leading-relaxed">{desc}</p>
            </div>
            <div class="mt-4 self-end opacity-90 transition-transform duration-300 group-hover:scale-110">
                <FeatureIcon id=id />
            </div>
        </div>
    }
}

// ── AuthModal ─────────────────────────────────────────────────────────────────

const SIGNUP_AVATARS: &[&str] = &[
    "https://lh3.googleusercontent.com/d/198aosLkzeCyglhaKy5vPMeWktSJhFui_",
    "https://lh3.googleusercontent.com/d/1i3UrprAcfhKSNaSwFE1FXwTD6NXOfjaV",
    "https://lh3.googleusercontent.com/d/1ZYyoo8gUHeugXIa5ciA6pJySe3OPdkNB",
    "https://lh3.googleusercontent.com/d/1wW1ox6Uc1g368rqZ5CAphVSH84KW711n",
    "https://lh3.googleusercontent.com/d/1CCWWd9W3ODzxAn1lJ6TsKRYyAxdLxeq8",
];

#[component]
fn AuthModal(
    initial_view: &'static str, // "signin" | "signup"
    initial_email: ReadSignal<String>,
    on_close: impl Fn() + 'static + Copy,
    on_set_view: impl Fn(&'static str) + 'static + Copy,
) -> impl IntoView {
    let navigate = use_navigate();

    let (view_mode, set_view_mode) = signal(initial_view);
    let (email, set_email) = signal(initial_email.get_untracked());
    let (password, set_password) = signal(String::new());
    let (display_name, set_display_name) = signal(String::new());
    let (selected_avatar, set_selected_avatar) = signal(SIGNUP_AVATARS[0]);
    let (show_password, set_show_password) = signal(false);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(String::new());
    let (success_msg, set_success_msg) = signal(String::new());

    // Sync email when parent updates it
    Effect::new(move |_| {
        set_email.set(initial_email.get());
    });

    let navigate2 = navigate.clone();

    let handle_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        // Demo bypass — navigate to /browse
        navigate("/browse", Default::default());
        let _ = set_loading;
        let _ = set_success_msg;
    };

    view! {
        <div
            class="fixed inset-0 z-[200] flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm"
            on:click=move |_| on_close()
        >
            <div
                class="w-full max-w-[410px] bg-black/90 border border-white/10 rounded-xl p-8 shadow-2xl animate-fadeIn"
                on:click=move |e| e.stop_propagation()
            >
                <h2 class="text-2xl font-bold text-white mb-1">
                    {move || if view_mode.get() == "signin" { "Sign In" } else { "Create Account" }}
                </h2>
                <p class="text-white/40 text-sm mb-6">
                    {move || if view_mode.get() == "signin" { "Welcome back to Pstream." } else { "Join Pstream today." }}
                </p>

                // Google
                <button
                    on:click=move |_| navigate2("/browse", Default::default())
                    disabled=move || loading.get()
                    class="w-full h-12 mb-5 bg-white text-black rounded-[4px] font-bold flex items-center justify-center gap-3 hover:bg-gray-100 transition-colors duration-150 active:scale-95 disabled:opacity-60"
                >
                    <i class="ph-bold ph-google-logo text-[20px]" />
                    "Continue with Google"
                </button>

                <div class="flex items-center gap-3 mb-5">
                    <div class="flex-1 h-px bg-white/10" />
                    <span class="text-white/30 text-xs uppercase tracking-widest">"or"</span>
                    <div class="flex-1 h-px bg-white/10" />
                </div>

                {move || {
                    let msg = success_msg.get();
                    if msg.is_empty() {
                        view! { <></> }.into_any()
                    } else {
                        view! {
                            <div class="mb-4 p-3 bg-green-500/20 border border-green-500/30 rounded text-green-300 text-sm">{msg}</div>
                        }.into_any()
                    }
                }}

                <form on:submit=handle_submit class="space-y-3">
                    // Display name — signup only
                    {move || if view_mode.get() == "signup" {
                        view! {
                            <div class="relative">
                                <i class="ph-bold ph-user absolute left-4 top-1/2 -translate-y-1/2 text-white/30 pointer-events-none text-[16px]" />
                                <input
                                    type="text"
                                    required
                                    prop:value=display_name
                                    on:input=move |e| set_display_name.set(event_target_value(&e))
                                    placeholder="Display name"
                                    class="w-full h-13 bg-[#2a2a2a] border border-white/10 text-white rounded-lg pl-10 pr-4 py-3.5 text-sm focus:outline-none focus:border-red-500 transition-colors placeholder:text-white/30"
                                />
                            </div>
                        }.into_any()
                    } else { view! { <></> }.into_any() }}

                    // Email
                    <div class="relative">
                        <i class="ph-bold ph-envelope absolute left-4 top-1/2 -translate-y-1/2 text-white/30 pointer-events-none text-[16px]" />
                        <input
                            type="email"
                            required
                            autocomplete="email"
                            prop:value=email
                            on:input=move |e| set_email.set(event_target_value(&e))
                            placeholder="Email address"
                            class="w-full h-13 bg-[#2a2a2a] border border-white/10 text-white rounded-lg pl-10 pr-4 py-3.5 text-sm focus:outline-none focus:border-red-500 transition-colors placeholder:text-white/30"
                        />
                    </div>

                    // Password
                    <div class="relative">
                        <i class="ph-bold ph-lock-simple absolute left-4 top-1/2 -translate-y-1/2 text-white/30 pointer-events-none text-[16px]" />
                        <input
                            prop:type=move || if show_password.get() { "text" } else { "password" }
                            required
                            min-length="6"
                            prop:value=password
                            on:input=move |e| set_password.set(event_target_value(&e))
                            placeholder="Password"
                            class="w-full h-13 bg-[#2a2a2a] border border-white/10 text-white rounded-lg pl-10 pr-10 py-3.5 text-sm focus:outline-none focus:border-red-500 transition-colors placeholder:text-white/30"
                        />
                        <button
                            type="button"
                            on:click=move |_| set_show_password.update(|v| *v = !*v)
                            class="absolute right-3 top-1/2 -translate-y-1/2 text-white/30 hover:text-white/60 transition-colors"
                        >
                            <i class=move || if show_password.get() { "ph-bold ph-eye-slash text-[16px]" } else { "ph-bold ph-eye text-[16px]" } />
                        </button>
                    </div>

                    // Avatar picker — signup only
                    {move || if view_mode.get() == "signup" {
                        let avatars = SIGNUP_AVATARS.iter().map(|url| {
                            let url = *url;
                            view! {
                                <div
                                    on:click=move |_| set_selected_avatar.set(url)
                                    class=move || {
                                        let selected = selected_avatar.get() == url;
                                        if selected {
                                            "w-11 h-11 rounded-md overflow-hidden cursor-pointer border-[2px] transition-all border-red-500 scale-105 shadow-md shadow-red-500/20"
                                        } else {
                                            "w-11 h-11 rounded-md overflow-hidden cursor-pointer border-[2px] transition-all border-transparent hover:scale-105"
                                        }
                                    }
                                >
                                    <img src=url class="w-full h-full object-cover" alt="" />
                                </div>
                            }
                        }).collect::<Vec<_>>();
                        view! {
                            <div class="space-y-2 pt-1 pb-1">
                                <span class="text-white/40 text-xs font-semibold block">"Choose your profile icon"</span>
                                <div class="flex gap-3 justify-center pb-2">{avatars}</div>
                            </div>
                        }.into_any()
                    } else { view! { <></> }.into_any() }}

                    {move || {
                        let err = error.get();
                        if err.is_empty() { view! { <></> }.into_any() }
                        else { view! { <p class="text-red-400 text-xs">{err}</p> }.into_any() }
                    }}

                    <button
                        type="submit"
                        disabled=move || loading.get()
                        class="w-full py-3.5 bg-[#e50914] hover:bg-[#b80710] text-white rounded-[4px] font-bold text-sm shadow-lg transition-colors duration-150 active:scale-95 disabled:opacity-50 mt-1"
                    >
                        {move || if view_mode.get() == "signup" { "Create Account" } else { "Sign In" }}
                    </button>
                </form>

                <p class="mt-5 text-center text-white/40 text-sm">
                    {move || if view_mode.get() == "signin" { "Don't have an account? " } else { "Already have an account? " }}
                    <button
                        on:click=move |_| {
                            let next = if view_mode.get() == "signin" { "signup" } else { "signin" };
                            set_view_mode.set(next);
                            set_error.set(String::new());
                            set_success_msg.set(String::new());
                            on_set_view(next);
                        }
                        class="text-white hover:underline font-medium"
                    >
                        {move || if view_mode.get() == "signin" { "Sign Up" } else { "Sign In" }}
                    </button>
                </p>
            </div>
        </div>
    }
}

// ── LoginPage ─────────────────────────────────────────────────────────────────

#[component]
pub fn LoginPage() -> impl IntoView {
    let (auth_view, set_auth_view) = signal("none"); // "none" | "signin" | "signup"
    let (hero_email, set_hero_email) = signal(String::new());

    let handle_get_started = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        set_auth_view.set("signup");
    };

    view! {
        <div class="bg-black text-white font-sans min-h-screen">

            // ── Navbar ──────────────────────────────────────────────────────
            <nav class="fixed top-0 w-full z-[80] bg-gradient-to-b from-black/80 to-transparent px-6 md:px-20 lg:px-32 xl:px-44 2xl:px-56 py-4 flex items-center justify-between">
                <img
                    src="/assets/logos/pstream-logo.svg"
                    alt="Pstream"
                    class="h-5 md:h-7 cursor-pointer"
                />
                <button
                    on:click=move |_| set_auth_view.set("signin")
                    class="px-5 py-2 bg-[#e50914] text-white text-sm font-bold rounded-[4px] hover:bg-[#b80710] shadow-lg transition-colors duration-150 active:scale-95"
                >
                    "Sign In"
                </button>
            </nav>

            // ── Hero Section ────────────────────────────────────────────────
            <section class="relative min-h-[60vh] sm:min-h-[65vh] md:min-h-[70vh] pt-32 pb-20 md:pt-44 md:pb-28 flex flex-col items-center justify-center text-center px-6 overflow-hidden">
                <div class="absolute inset-0 z-0 overflow-hidden">
                    <img src="/assets/landing-bg.png" class="w-full h-full object-cover opacity-50 scale-105" alt="" />
                    <div class="absolute inset-0 bg-gradient-to-b from-black/60 via-black/20 to-black" />
                    <div class="absolute inset-0 bg-[radial-gradient(circle_at_center,transparent_30%,rgba(0,0,0,0.6)_100%)]" />
                </div>

                <div class="relative z-10 max-w-[720px] mx-auto space-y-6 w-full px-4">
                    <h1 class="text-4xl sm:text-5xl md:text-7xl font-black leading-tight tracking-tight drop-shadow-2xl">
                        "Unlimited films, series and more"
                    </h1>
                    <p class="text-base sm:text-lg md:text-xl font-normal text-white/90 max-w-[600px] mx-auto leading-relaxed">
                        "Ready to watch? Enter your email to create or restart your membership."
                    </p>

                    <form
                        on:submit=handle_get_started
                        class="flex flex-col sm:flex-row gap-2.5 justify-center mt-8 max-w-[600px] mx-auto w-full"
                    >
                        <input
                            type="email"
                            prop:value=hero_email
                            on:input=move |e| set_hero_email.set(event_target_value(&e))
                            placeholder="Email address"
                            class="w-full sm:w-[65%] h-14 bg-black/50 border border-white/30 text-white rounded px-4 text-base focus:outline-none focus:border-white transition-colors placeholder:text-white/40 backdrop-blur-md"
                        />
                        <button
                            type="submit"
                            class="w-full sm:w-[35%] h-14 bg-[#e50914] hover:bg-[#b80710] text-white font-bold text-lg rounded-[4px] flex items-center justify-center gap-2 whitespace-nowrap transition-all duration-150 active:scale-95 shadow-xl"
                        >
                            "Get Started"
                            <i class="ph-bold ph-caret-right text-[22px]" />
                        </button>
                    </form>
                </div>
            </section>

            // ── Top 10 Trending Row ─────────────────────────────────────────
            <section class="pt-12 pb-6 bg-black">
                <LandingTopTenRow title="Trending Now" />
            </section>

            // ── More Reasons to Join ────────────────────────────────────────
            <section class="py-12 md:py-20 px-6 md:px-20 lg:px-32 xl:px-44 2xl:px-56 bg-black">
                <h2 class="text-white text-2xl md:text-4xl font-black mb-8 md:mb-12">"More reasons to join"</h2>
                <div class="grid grid-cols-[repeat(auto-fit,minmax(240px,1fr))] gap-4">
                    <FeatureCard
                        id="tv"
                        title="Watch on any screen"
                        desc="Stream on your smart TV, laptop, tablet or phone. Wherever you are, Pstream comes with you."
                    />
                    <FeatureCard
                        id="stream"
                        title="Stream in HD & 4K"
                        desc="Enjoy stunning picture quality with HD and Ultra HD streams, powered by the best available sources."
                    />
                    <FeatureCard
                        id="subtitles"
                        title="Subtitles your way"
                        desc="Choose from 40+ languages. Customise font size, colour, and background to your taste."
                    />
                    <FeatureCard
                        id="place"
                        title="Never lose your place"
                        desc="Continue Watching picks up exactly where you left off — across every show and season, on any device."
                    />
                </div>
            </section>

            // ── Bottom CTA ──────────────────────────────────────────────────
            <section class="py-16 px-6 text-center bg-black flex flex-col items-center w-full">
                <p class="text-base sm:text-lg md:text-xl font-normal mb-6 max-w-[600px] mx-auto leading-relaxed text-white/90">
                    "Ready to watch? Enter your email to create or log in to your account."
                </p>
                <form
                    on:submit=handle_get_started
                    class="flex flex-col sm:flex-row gap-2.5 justify-center max-w-[600px] mx-auto w-full"
                >
                    <input
                        type="email"
                        prop:value=hero_email
                        on:input=move |e| set_hero_email.set(event_target_value(&e))
                        placeholder="Email address"
                        class="w-full sm:w-[65%] h-14 bg-black/50 border border-white/30 text-white rounded px-4 text-base focus:outline-none focus:border-white transition-colors placeholder:text-white/40 backdrop-blur-md"
                    />
                    <button
                        type="submit"
                        class="w-full sm:w-[35%] h-14 bg-[#e50914] hover:bg-[#b80710] text-white font-bold text-lg rounded-[4px] flex items-center justify-center gap-2 whitespace-nowrap transition-all duration-150 active:scale-95 shadow-xl"
                    >
                        "Get Started"
                        <i class="ph-bold ph-caret-right text-[22px]" />
                    </button>
                </form>
            </section>

            // ── Footer ──────────────────────────────────────────────────────
            <crate::components::layout::footer::Footer />

            // ── Auth Modal ──────────────────────────────────────────────────
            {move || {
                let v = auth_view.get();
                if v == "none" {
                    view! { <></> }.into_any()
                } else {
                    let init = if v == "signin" { "signin" } else { "signup" };
                    view! {
                        <AuthModal
                            initial_view=init
                            initial_email=hero_email
                            on_close=move || set_auth_view.set("none")
                            on_set_view=move |next| set_auth_view.set(next)
                        />
                    }.into_any()
                }
            }}
        </div>
    }
}
