use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewPopularTab {
    Watching,
    JustLanded,
    Top10Movies,
    Top10Series,
    ComingSoon,
}

#[component]
pub fn NewPopularSubNavMobile(
    active_tab: ReadSignal<NewPopularTab>,
    on_tab_change: Callback<NewPopularTab>,
) -> impl IntoView {
    let tabs = vec![
        (NewPopularTab::Watching, "Everyone's Watching"),
        (NewPopularTab::JustLanded, "Just Landed"),
        (NewPopularTab::Top10Movies, "Top 10 Movies"),
        (NewPopularTab::Top10Series, "Top 10 Series"),
        (NewPopularTab::ComingSoon, "Coming Soon"),
    ];

    let total = tabs.len();

    view! {
        <div class="relative w-full overflow-hidden select-none">
            <div class="pt-2 pb-2 px-4 flex items-center justify-start overflow-x-auto scrollbar-hide max-w-full">
                <div class="flex items-center gap-1 shrink-0">
                    {tabs.into_iter().enumerate().map(|(idx, (tab, label))| {
                        let is_active = move || active_tab.get() == tab;
                        let on_change = on_tab_change;
                        let corner_class = if idx == 0 {
                            "rounded-l-[20px] rounded-r-[10px]"
                        } else if idx == total - 1 {
                            "rounded-l-[10px] rounded-r-[20px]"
                        } else {
                            "rounded-[10px]"
                        };

                        view! {
                            <button
                                on:click=move |_| on_change.run(tab)
                                class=move || {
                                    let active_cls = if is_active() {
                                        "bg-white/[0.18] backdrop-blur-md text-white border-[1.6px] border-white/40"
                                    } else {
                                        "bg-white/[0.06] backdrop-blur-md text-[#e5e5e5] border-[1.6px] border-white/15"
                                    };
                                    format!(
                                        "flex items-center justify-center h-[38px] px-4 {} text-[14px] font-semibold whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer {}",
                                        corner_class, active_cls
                                    )
                                }
                            >
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
