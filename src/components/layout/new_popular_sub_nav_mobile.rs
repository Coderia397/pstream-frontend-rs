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
        (NewPopularTab::Watching, "tabs-everyone-watching"),
        (NewPopularTab::JustLanded, "tabs-just-landed"),
        (NewPopularTab::Top10Movies, "tabs-top10-movies"),
        (NewPopularTab::Top10Series, "tabs-top10-series"),
        (NewPopularTab::ComingSoon, "tabs-coming-soon"),
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
                            "rounded-l-chip rounded-r-chip-inner"
                        } else if idx == total - 1 {
                            "rounded-l-chip-inner rounded-r-chip"
                        } else {
                            "rounded-chip-inner"
                        };

                        view! {
                            <button
                                on:click=move |_| on_change.run(tab)
                                class=move || {
                                    let active_cls = if is_active() {
                                        "bg-ink/[0.18] backdrop-blur-md text-ink border-[1.6px] border-ink/40"
                                    } else {
                                        "bg-ink/[0.06] backdrop-blur-md text-ink-soft border-[1.6px] border-ink/15"
                                    };
                                    format!(
                                        "flex items-center justify-center h-[38px] px-4 {} text-body-sm font-semibold whitespace-nowrap active:scale-95 transition-all leading-none shrink-0 cursor-pointer {}",
                                        corner_class, active_cls
                                    )
                                }
                            >
                                {crate::i18n::t(label)}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
