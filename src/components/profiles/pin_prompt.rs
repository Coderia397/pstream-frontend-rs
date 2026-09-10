use leptos::prelude::*;
use leptos::html::Input;
use wasm_bindgen::JsCast;
use crate::models::profile::Profile;

#[component]
pub fn ProfilePinPrompt(
    profile: Profile,
    on_unlock: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (digits, set_digits) = signal(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);
    let (shake, set_shake) = signal(false);

    let input_ref_0 = NodeRef::<Input>::new();
    let input_ref_1 = NodeRef::<Input>::new();
    let input_ref_2 = NodeRef::<Input>::new();
    let input_ref_3 = NodeRef::<Input>::new();

    // Focus first input on mount
    Effect::new(move |_| {
        if let Some(el) = input_ref_0.get() {
            let _ = el.focus();
        }
    });

    let profile_pin = profile.pin.clone().unwrap_or_default();
    let profile_name = profile.name.clone();
    let profile_avatar = profile.avatar_url.clone().unwrap_or_else(|| {
        "https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg".to_string()
    });

    let focus_input = move |idx: usize| {
        match idx {
            0 => { if let Some(el) = input_ref_0.get() { let _ = el.focus(); } },
            1 => { if let Some(el) = input_ref_1.get() { let _ = el.focus(); } },
            2 => { if let Some(el) = input_ref_2.get() { let _ = el.focus(); } },
            3 => { if let Some(el) = input_ref_3.get() { let _ = el.focus(); } },
            _ => {},
        }
    };

    let verify_candidate = move |candidate: String| {
        if candidate == profile_pin {
            on_unlock.run(());
        } else {
            set_shake.set(true);
            set_digits.set(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);
            focus_input(0);
            
            // Clear shake after 500ms
            if let Some(win) = web_sys::window() {
                let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::wrap(Box::new(move || {
                    set_shake.set(false);
                }));
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 500);
                cb.forget();
            }
        }
    };

    let on_input_digit = move |idx: usize, val: String| {
        let clean: String = val.chars().filter(|c| c.is_ascii_digit()).take(1).collect();
        set_digits.update(|d| {
            if idx < d.len() {
                d[idx] = clean.clone();
            }
        });

        if !clean.is_empty() {
            if idx < 3 {
                focus_input(idx + 1);
            } else {
                let all_digits = digits.get_untracked().join("");
                if all_digits.len() == 4 {
                    verify_candidate(all_digits);
                }
            }
        }
    };

    let on_keydown_digit = move |idx: usize, e: leptos::ev::KeyboardEvent| {
        if e.key() == "Backspace" {
            let current = digits.get();
            if idx < current.len() && current[idx].is_empty() && idx > 0 {
                focus_input(idx - 1);
            }
        } else if e.key() == "Escape" {
            on_cancel.run(());
        }
    };

    let stored_input_digit = StoredValue::new(on_input_digit);
    let stored_keydown_digit = StoredValue::new(on_keydown_digit);

    view! {
        <div class="fixed inset-0 z-[200] bg-black/90 backdrop-blur-sm flex flex-col items-center justify-center px-6 animate-in fade-in duration-200">
            // Close button
            <button
                on:click=move |_| on_cancel.run(())
                class="absolute top-6 right-6 w-9 h-9 rounded-full flex items-center justify-center text-white/60 hover:text-white hover:bg-white/10 transition-colors cursor-pointer"
                aria-label="Close"
            >
                <i class="ph-bold ph-x text-2xl"></i>
            </button>

            // Profile Avatar
            <div class="w-20 h-20 rounded-md overflow-hidden mb-6 shadow-xl ring-2 ring-white/30">
                <img
                    src=profile_avatar
                    alt=profile_name.clone()
                    class="w-full h-full object-cover"
                />
            </div>

            <p class="text-white/50 text-sm mb-1 font-medium">
                "Profile Lock is currently on."
            </p>
            <h1 class="text-white text-xl sm:text-2xl font-bold mb-8 text-center max-w-md">
                {format!("Enter your PIN to access {}'s profile.", profile_name)}
            </h1>

            // 4-box PIN inputs with shake animation (Task 098)
            <div
                class=move || if shake.get() {
                    "flex items-center gap-3.5 mb-8 animate-bounce duration-100"
                } else {
                    "flex items-center gap-3.5 mb-8"
                }
            >
                <input
                    node_ref=input_ref_0
                    type="password"
                    inputmode="numeric"
                    maxlength="1"
                    prop:value=move || digits.get().get(0).cloned().unwrap_or_default()
                    on:input=move |e| stored_input_digit.with_value(|f| f(0, event_target_value(&e)))
                    on:keydown=move |e| stored_keydown_digit.with_value(|f| f(0, e))
                    class="w-12 h-14 sm:w-14 sm:h-16 bg-[#333] border-2 border-white/20 focus:border-white rounded-md text-center text-white text-2xl font-bold outline-none transition-all caret-transparent select-none"
                    class=("!border-red-500", move || shake.get())
                />
                <input
                    node_ref=input_ref_1
                    type="password"
                    inputmode="numeric"
                    maxlength="1"
                    prop:value=move || digits.get().get(1).cloned().unwrap_or_default()
                    on:input=move |e| stored_input_digit.with_value(|f| f(1, event_target_value(&e)))
                    on:keydown=move |e| stored_keydown_digit.with_value(|f| f(1, e))
                    class="w-12 h-14 sm:w-14 sm:h-16 bg-[#333] border-2 border-white/20 focus:border-white rounded-md text-center text-white text-2xl font-bold outline-none transition-all caret-transparent select-none"
                    class=("!border-red-500", move || shake.get())
                />
                <input
                    node_ref=input_ref_2
                    type="password"
                    inputmode="numeric"
                    maxlength="1"
                    prop:value=move || digits.get().get(2).cloned().unwrap_or_default()
                    on:input=move |e| stored_input_digit.with_value(|f| f(2, event_target_value(&e)))
                    on:keydown=move |e| stored_keydown_digit.with_value(|f| f(2, e))
                    class="w-12 h-14 sm:w-14 sm:h-16 bg-[#333] border-2 border-white/20 focus:border-white rounded-md text-center text-white text-2xl font-bold outline-none transition-all caret-transparent select-none"
                    class=("!border-red-500", move || shake.get())
                />
                <input
                    node_ref=input_ref_3
                    type="password"
                    inputmode="numeric"
                    maxlength="1"
                    prop:value=move || digits.get().get(3).cloned().unwrap_or_default()
                    on:input=move |e| stored_input_digit.with_value(|f| f(3, event_target_value(&e)))
                    on:keydown=move |e| stored_keydown_digit.with_value(|f| f(3, e))
                    class="w-12 h-14 sm:w-14 sm:h-16 bg-[#333] border-2 border-white/20 focus:border-white rounded-md text-center text-white text-2xl font-bold outline-none transition-all caret-transparent select-none"
                    class=("!border-red-500", move || shake.get())
                />
            </div>

            // Cancel action button
            <button
                on:click=move |_| on_cancel.run(())
                class="px-6 py-2 border border-white/30 text-white/70 hover:text-white hover:border-white rounded text-sm font-semibold tracking-wider uppercase transition-colors cursor-pointer active:scale-95"
            >
                "Cancel"
            </button>
        </div>
    }
}
