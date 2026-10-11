use super::home::QuickLinks;
use super::shell::{set_title, HelpShell};
use super::widgets::{SearchBox, Variant};
use super::tr;
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    Effect::new(move |_| set_title(&tr("help-ui-contact-title")));
    view! {
        <HelpShell contact_band=false>
            <main class="mx-auto w-full max-w-[720px] px-6 pb-20 pt-10">
                <h1 class="mb-10 text-heading-xl font-extrabold text-paper-ink md:text-display">{tr("help-ui-contact-title")}</h1>
                <h2 class="mb-4 text-title font-semibold text-paper-ink">{tr("help-ui-contact-prompt")}</h2>
                <SearchBox initial="" placeholder="help-ui-describe" variant=Variant::Arrow />
                <div class="mt-10">
                    <QuickLinks />
                </div>
            </main>
        </HelpShell>
    }
}
