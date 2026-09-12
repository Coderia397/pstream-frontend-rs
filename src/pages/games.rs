use leptos::prelude::*;
use crate::components::layout::Layout;

#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="min-h-[75vh] w-full flex flex-col items-center justify-center text-center px-4 pt-24 pb-16 bg-[#141414] select-none">
                <div class="relative flex items-center justify-center mb-6">
                    <div class="w-16 h-16 md:w-20 md:h-20 rounded-2xl bg-white/[0.04] border border-white/[0.08] backdrop-blur-md flex items-center justify-center shadow-lg">
                        <i class="ph-fill ph-game-controller text-3xl md:text-4xl text-white/60"></i>
                    </div>
                </div>

                <h1 class="text-4xl sm:text-5xl md:text-6xl font-black text-white tracking-tight">
                    "Coming Soon"
                </h1>
            </div>
        </Layout>
    }
}
