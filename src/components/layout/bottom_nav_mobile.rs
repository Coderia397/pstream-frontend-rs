use leptos::prelude::*;


pub struct BottomNavItem {
    pub id: &'static str,
    pub icon: AnyView,
    pub label: &'static str,
    pub on_click: Callback<()>,
}

#[component]
pub fn BottomNavMobile(
    items: Vec<BottomNavItem>,
    active_id: ReadSignal<&'static str>,
) -> impl IntoView {
    view! {
        <div class="fixed bottom-[calc(12px+env(safe-area-inset-bottom))] left-1/2 -translate-x-1/2 z-[10020]
                    w-auto max-w-[calc(100%-2rem)]
                    bg-[#2a2a2a]/95 backdrop-blur-md border border-white/[0.06] rounded-full
                    px-1.5 py-1 shadow-[0_8px_30px_rgba(0,0,0,0.55)]">
            <div class="flex items-center">
                {items.into_iter().map(|item| {
                    let id = item.id;
                    let label = item.label;
                    let icon = item.icon;
                    let on_click = item.on_click;
                    
                    let is_active = move || active_id.get() == id;

                    view! {
                        <button
                            on:click=move |_| on_click.run(())
                            class="relative flex flex-col items-center justify-center gap-0.5 rounded-full px-6 sm:px-7 py-1.5 select-none transition-colors duration-200 active:scale-95"
                            class=("bg-white/[0.12]", move || is_active())
                            class=("text-white", move || is_active())
                            class=("text-white/55", move || !is_active())
                            class=("hover:text-white/80", move || !is_active())
                        >
                            <span class="flex items-center justify-center h-[22px]">
                                {icon}
                            </span>
                            <span 
                                class="text-[10px] tracking-wide whitespace-nowrap"
                                class=("font-medium", move || is_active())
                                class=("font-normal", move || !is_active())
                            >
                                {label}
                            </span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
