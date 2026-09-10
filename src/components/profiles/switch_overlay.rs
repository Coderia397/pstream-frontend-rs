use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::models::profile::Profile;

#[component]
pub fn ProfileSwitchOverlay(
    profile: Profile,
    on_finish: Callback<()>,
) -> impl IntoView {
    let name = profile.name.clone();
    let avatar = profile.avatar_url.clone().unwrap_or_else(|| {
        "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string()
    });
    let is_kids = profile.is_kids;

    // Auto-dismiss after 800ms
    Effect::new(move |_| {
        if let Some(win) = web_sys::window() {
            let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                on_finish.run(());
            }));
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 850);
            cb.forget();
        }
    });

    view! {
        <div class="fixed inset-0 z-[300] bg-black flex flex-col items-center justify-center overflow-hidden animate-in fade-in duration-200">
            // Kids bloom backdrop
            {if is_kids {
                view! {
                    <div class="absolute inset-0 flex opacity-40 animate-pulse">
                        <div class="flex-1 h-full bg-gradient-to-b from-[#2e8b4f] to-[#8d3fd1]"></div>
                        <div class="flex-1 h-full bg-gradient-to-b from-[#f6c244] to-[#e05238]"></div>
                        <div class="flex-1 h-full bg-gradient-to-b from-[#f66ab5] to-[#ec2fa0]"></div>
                        <div class="flex-1 h-full bg-gradient-to-b from-[#cfc4f5] to-[#2f6bf0]"></div>
                    </div>
                }.into_any()
            } else {
                view! { <div /> }.into_any()
            }}

            // Center avatar with spinning red orbit arc (Task 103)
            <div class="relative flex items-center justify-center">
                // Orbit Arc
                <svg
                    width="220"
                    height="220"
                    viewBox="0 0 220 220"
                    class="absolute animate-spin select-none pointer-events-none"
                    style="animation-duration: 1.1s;"
                >
                    <circle
                        cx="110"
                        cy="110"
                        r="95"
                        fill="none"
                        stroke="#e50914"
                        stroke-width="8"
                        stroke-linecap="round"
                        stroke-dasharray="210 390"
                    />
                </svg>

                // Profile Avatar
                <div class="w-24 h-24 sm:w-28 sm:h-28 rounded-md overflow-hidden shadow-2xl ring-2 ring-white/40 z-10">
                    {if is_kids && profile.avatar_url.is_none() {
                        view! { <crate::components::profiles::kids_avatar::KidsAvatar size=112.0 /> }.into_any()
                    } else {
                        view! {
                            <img
                                src=avatar
                                alt=name.clone()
                                class="w-full h-full object-cover"
                            />
                        }.into_any()
                    }}
                </div>
            </div>

            // Profile name
            <h2 class="text-white text-xl sm:text-2xl font-bold mt-8 tracking-wide z-10 animate-pulse">
                {name}
            </h2>
        </div>
    }
}
