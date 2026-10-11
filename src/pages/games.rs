use leptos::prelude::*;
use crate::components::layout::Layout;

#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <Layout>
            <div class="min-h-[75vh] w-full flex flex-col items-center justify-center text-center px-4 pt-24 pb-16 bg-page select-none">
                <div class="relative flex items-center justify-center mb-6">
                    <div class="w-16 h-16 md:w-20 md:h-20 rounded-sheet bg-ink/[0.04] border border-ink/[0.08] backdrop-blur-md flex items-center justify-center shadow-lg">
                        <i class="ph-fill ph-game-controller text-3xl md:text-4xl text-ink/60"></i>
                    </div>
                </div>

                <h1 class="text-4xl sm:text-5xl md:text-6xl font-black text-ink tracking-tight">
                    {crate::i18n::t("games-coming-soon")}
                </h1>
            </div>
        </Layout>
    }
}
