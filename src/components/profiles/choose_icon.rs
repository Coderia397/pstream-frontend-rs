use super::icon_picker::IconPicker;
use leptos::prelude::*;

/// The page of pictures over the profile form (Who's watching, Manage profiles): the top bar with the logo, then the picker. Pick one and it closes with the choice.
#[component]
pub fn ChooseIconModal(
    current_avatar_url: Option<String>,
    on_select: Callback<String>,
    on_cancel: Callback<()>,
    #[prop(optional, into)] name: String,
    #[prop(default = false)] kids: bool,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-(--z-modal-top) overflow-y-auto bg-page text-ink" style="--app-x: calc(8.3333vw + 72px);">
            <header class="flex h-16 items-center px-[var(--app-x)]">
                <img src=crate::brand::LOGO_WORDMARK_PRIMARY alt=crate::brand::NAME class="h-7 w-auto" />
            </header>
            <IconPicker name=name current=current_avatar_url kids=kids on_pick=on_select on_back=on_cancel />
        </div>
    }
}
