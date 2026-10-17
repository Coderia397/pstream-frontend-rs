use super::icon_picker::IconPicker;
use leptos::prelude::*;

/// The page of pictures over the profile form (Who's watching, Manage profiles): pick one and it closes with the choice.
#[component]
pub fn ChooseIconModal(
    current_avatar_url: Option<String>,
    on_select: Callback<String>,
    on_cancel: Callback<()>,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    view! { <IconPicker name=name current=current_avatar_url on_pick=on_select on_back=on_cancel /> }
}
