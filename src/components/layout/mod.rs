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

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let is_mobile = false; 

    view! {
        <div class="bg-void md:bg-page min-h-screen flex flex-col font-sans text-ink selection:bg-brand selection:text-ink">
            <Navbar />
            
            <div class="flex-1 transition-all duration-(--motion-slow)"
                 class=("pb-[calc(var(--bottom-nav-h)+env(safe-area-inset-bottom))]", move || is_mobile)
                 class=("pb-0", move || !is_mobile)>
                {children()}
            </div>
            
            <Footer />
        </div>
    }
}
