pub mod navbar;
pub mod navbar_mobile;
pub mod bottom_nav_mobile;
pub mod search_bar;
pub mod notifications_dropdown;
pub mod category_sub_nav;
pub mod category_sub_nav_mobile;
pub mod new_popular_sub_nav_mobile;
pub mod footer;

use leptos::prelude::*;
use navbar::Navbar;
use footer::Footer;

/// The page frame of the browsing pages. On phones and tablets (under 1024 px) the bottom bar is the navigation and there is no footer, so the
/// content keeps room above the bar; from 1024 px the website layout shows the footer. Help, Settings, legal and sign-in pages have their own frames.
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="bg-void md:bg-page min-h-screen flex flex-col font-sans text-ink selection:bg-brand selection:text-ink">
            <Navbar />

            <div class="flex-1 transition-all duration-(--motion-slow) pb-[calc(var(--bottom-nav-h)+env(safe-area-inset-bottom))] lg:pb-0">
                {children()}
            </div>

            <div class="hidden lg:block">
                <Footer />
            </div>
        </div>
    }
}
