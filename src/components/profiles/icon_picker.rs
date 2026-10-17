//! The page "Choose a profile icon": a back arrow, the heading, who it is for, the picture the profile has now, and a row for each set of pictures.
//! On the website it is a page of its own (no top bar); the rows page like the rows of the browse page.
use super::icon_row::{IconChoice, IconRow};
use crate::data::avatars::AVATAR_CATEGORIES;
use crate::i18n::t;
use leptos::prelude::*;

#[component]
pub fn IconPicker(
    /// The name of the profile the picture is for.
    #[prop(into)] name: Signal<String>,
    /// The picture the profile has now.
    #[prop(into)] current: Signal<Option<String>>,
    on_pick: Callback<String>,
    on_back: Callback<()>,
) -> impl IntoView {
    let history: Vec<IconChoice> = current.get_untracked().map(|url| vec![IconChoice { name: t("profile-icon-current"), url }]).unwrap_or_default();
    view! {
        // the left and right margin of the page, which the rows use as the margin of their first picture
        <div class="fixed inset-0 z-(--z-screen) overflow-y-auto bg-page text-ink" style="--app-x: calc(8.3333vw + 72px);">
            <div class="px-[var(--app-x)] pt-[26px]">
                <button type="button" on:click=move |_| on_back.run(()) aria-label=t("choose-icon-back") class="-ms-2 flex h-10 w-10 items-center justify-center rounded-full hover:bg-ink/10">
                    <i class="ph ph-arrow-left text-[28px] rtl:-scale-x-100" aria-hidden="true"></i>
                </button>
                <h1 class="mt-[30px] text-[36px] font-bold leading-[46px]">{t("profile-choose-icon")}</h1>
                <p class="mt-1 flex items-center gap-3 text-[22px] text-ink/80">
                    {move || crate::i18n::t_args("profile-icon-for", &[("name", name.get().into())])}
                    {move || current.get().map(|u| view! { <img src=u alt="" class="h-9 w-9 rounded-full object-cover" /> })}
                </p>
            </div>
            <div class="pb-16 pt-6">
                {(!history.is_empty()).then(|| view! { <IconRow title=t("profile-icon-history") items=history.clone() current=current on_pick=on_pick /> })}
                {AVATAR_CATEGORIES.iter().map(|cat| {
                    let items = cat.avatars.iter().map(|a| IconChoice { name: a.name.to_string(), url: a.url.to_string() }).collect();
                    view! { <IconRow title=cat.name items=items current=current on_pick=on_pick /> }
                }).collect_view()}
            </div>
        </div>
    }
}
