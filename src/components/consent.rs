//! Cookie banner and Privacy Preference Center. State and rules are in crate::consent.
use crate::config::{cookie_banner_position, BannerPosition};
use crate::consent::use_ui;
use crate::pages::help::markup;
use crate::pages::help::tr;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    General,
    Essential,
    Personal,
    Third,
    Ads,
}

const SECTIONS: [(Section, &str); 5] = [
    (Section::General, "cookie-nav-general"),
    (Section::Essential, "cookie-nav-essential"),
    (Section::Personal, "cookie-nav-personal"),
    (Section::Third, "cookie-nav-third"),
    (Section::Ads, "cookie-nav-ads"),
];

#[component]
fn Toggle(on: RwSignal<bool>, label: &'static str) -> impl IntoView {
    view! {
        <button
            role="switch"
            aria-checked=move || on.get().to_string()
            aria-label=move || tr(label)
            on:click=move |_| on.update(|v| *v = !*v)
            class="relative h-6 w-11 shrink-0 rounded-full transition-colors duration-(--motion-base)"
            class=("bg-brand", move || on.get())
            class=("bg-paper-edge", move || !on.get())
        >
            <span
                class="absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-paper transition-transform duration-(--motion-base)"
                class=("translate-x-5", move || on.get())
            ></span>
        </button>
    }
}

#[component]
fn Details(list_key: &'static str) -> impl IntoView {
    let (open, set_open) = signal(false);
    view! {
        <button on:click=move |_| set_open.update(|o| *o = !*o) class="mt-4 text-body-sm text-brand-active hover:underline">
            {move || if open.get() { tr("cookie-hide-details") } else { tr("cookie-details") }}
        </button>
        <Show when=move || open.get()>
            <div class="mt-3 text-body-sm">{markup::blocks(markup::parse(&tr(list_key)))}</div>
        </Show>
    }
}

#[component]
fn Center() -> impl IntoView {
    let ui = use_ui();
    let (section, set_section) = signal(Section::General);
    let personal = ui.personalization;
    view! {
        <div class="fixed inset-0 z-(--z-modal-top) flex items-center justify-center bg-void/60 p-3" on:click=move |_| ui.center.set(false)>
            <div
                role="dialog"
                aria-modal="true"
                aria-label=move || tr("cookie-center-title")
                on:click=|ev| ev.stop_propagation()
                class="flex h-[min(620px,92dvh)] w-full max-w-[830px] flex-col overflow-hidden bg-paper font-sans text-paper-ink shadow-popover"
            >
                <div class="flex items-center justify-between gap-4 border-b border-paper-line px-5 py-3">
                    <img src=crate::brand::LOGO_WORDMARK_PRIMARY alt=crate::brand::NAME class="h-6 w-auto shrink-0" />
                    <h2 class="flex-1 text-center text-title font-bold md:text-start md:ps-10">{tr("cookie-center-title")}</h2>
                    <button aria-label=move || tr("cookie-dismiss") on:click=move |_| ui.center.set(false) class="flex h-9 w-9 items-center justify-center text-title hover:bg-paper-soft">
                        <i class="ph-bold ph-x"></i>
                    </button>
                </div>
                <div class="flex min-h-0 flex-1 flex-col md:flex-row">
                    <nav class="flex shrink-0 overflow-x-auto bg-paper-soft md:w-[250px] md:flex-col md:overflow-visible">
                        {SECTIONS
                            .iter()
                            .map(|(s, key)| {
                                let s = *s;
                                view! {
                                    <button
                                        on:click=move |_| set_section.set(s)
                                        aria-current=move || (section.get() == s).then_some("true")
                                        class="shrink-0 border-b border-paper-line border-s-4 px-4 py-3.5 text-start text-body-sm font-bold"
                                        class=("border-s-brand", move || section.get() == s)
                                        class=("bg-paper", move || section.get() == s)
                                        class=("border-s-transparent", move || section.get() != s)
                                    >
                                        {tr(key)}
                                    </button>
                                }
                            })
                            .collect_view()}
                    </nav>
                    <div class="min-h-0 flex-1 overflow-y-auto p-5 text-body-sm leading-6 md:p-7">
                        {move || match section.get() {
                            Section::General => view! {
                                <h3 class="mb-4 text-body-md font-bold">{tr("cookie-nav-general")}</h3>
                                {markup::blocks(markup::parse(&tr("cookie-general")))}
                            }
                            .into_any(),
                            Section::Essential => view! {
                                <div class="mb-4 flex items-center justify-between gap-4">
                                    <h3 class="text-body-md font-bold">{tr("cookie-nav-essential")}</h3>
                                    <span class="text-fine font-bold text-brand-active">{tr("cookie-always-active")}</span>
                                </div>
                                {markup::blocks(markup::parse(&tr("cookie-essential")))}
                                <Details list_key="cookie-essential-list" />
                            }
                            .into_any(),
                            Section::Personal => view! {
                                <div class="mb-4 flex items-center justify-between gap-4">
                                    <h3 class="text-body-md font-bold">{tr("cookie-nav-personal")}</h3>
                                    <Toggle on=personal label="cookie-nav-personal" />
                                </div>
                                {markup::blocks(markup::parse(&tr("cookie-personal")))}
                                <Details list_key="cookie-personal-list" />
                            }
                            .into_any(),
                            Section::Third => view! {
                                <h3 class="mb-4 text-body-md font-bold">{tr("cookie-nav-third")}</h3>
                                {markup::blocks(markup::parse(&tr("cookie-third")))}
                            }
                            .into_any(),
                            Section::Ads => view! {
                                <h3 class="mb-4 text-body-md font-bold">{tr("cookie-nav-ads")}</h3>
                                {markup::blocks(markup::parse(&tr("cookie-ads")))}
                            }
                            .into_any(),
                        }}
                    </div>
                </div>
                <div class="border-t border-paper-line px-5 py-4">
                    <button on:click=move |_| ui.decide(personal.get_untracked()) class="rounded-control bg-brand px-6 py-3 text-body-sm font-bold text-ink hover:bg-brand-hover">
                        {tr("cookie-save")}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Mounted once in App. Shows the banner until a choice is saved, and the Privacy Preference Center when asked.
#[component]
pub fn CookieManager() -> impl IntoView {
    let ui = use_ui();
    let location = use_location();
    let on_player = move || location.pathname.get().starts_with("/watch");
    // Where the banner sits is a setting (crate::config). Narrow screens always get a full width sheet at the bottom.
    let place = match cookie_banner_position() {
        BannerPosition::Top => "top-3 inset-x-3 mx-auto max-w-[1100px] max-md:inset-x-0 max-md:bottom-0 max-md:top-auto",
        BannerPosition::Bottom => "bottom-3 inset-x-3 mx-auto max-w-[1100px] max-md:inset-x-0 max-md:bottom-0",
        BannerPosition::BottomStart => "bottom-3 start-3 w-[min(440px,calc(100%-1.5rem))] max-md:inset-x-0 max-md:bottom-0 max-md:start-0 max-md:w-auto",
        BannerPosition::BottomEnd => "bottom-3 end-3 w-[min(440px,calc(100%-1.5rem))] max-md:inset-x-0 max-md:bottom-0 max-md:end-0 max-md:w-auto",
        BannerPosition::Center => "start-1/2 top-1/2 w-[min(560px,calc(100%-1.5rem))] -translate-x-1/2 -translate-y-1/2",
    };
    view! {
        <Show when=move || ui.banner.get() && !on_player()>
            <div role="region" aria-label=move || tr("cookie-center-title") class=format!("fixed z-(--z-modal) border border-paper-line bg-paper-soft px-5 py-4 font-sans text-paper-ink shadow-popover md:px-7 {place}")>
                <div class="flex items-start gap-4">
                    <div class="flex-1">
                        <p class="text-body-sm leading-6">{markup::text(&tr("cookie-banner"))}</p>
                        <div class="mt-4 flex flex-wrap gap-3">
                            <button on:click=move |_| ui.open_center() class="rounded-control border border-paper-edge bg-paper px-5 py-2.5 text-body-sm font-bold hover:bg-paper-soft">{tr("cookie-learn")}</button>
                            <button on:click=move |_| ui.decide(false) class="rounded-control bg-brand px-5 py-2.5 text-body-sm font-bold text-ink hover:bg-brand-hover">{tr("cookie-reject")}</button>
                            <button on:click=move |_| ui.decide(true) class="rounded-control bg-brand px-5 py-2.5 text-body-sm font-bold text-ink hover:bg-brand-hover">{tr("cookie-accept")}</button>
                        </div>
                    </div>
                    <button aria-label=move || tr("cookie-dismiss") on:click=move |_| ui.banner.set(false) class="text-title text-paper-ink">
                        <i class="ph-bold ph-x"></i>
                    </button>
                </div>
            </div>
        </Show>
        <Show when=move || ui.center.get()>
            <Center />
        </Show>
    }
}
