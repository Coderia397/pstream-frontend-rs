use leptos::prelude::*;

#[component]
pub fn ReadsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-[#141414] text-white pt-24 pb-12 px-4 md:px-12">
            <h1 class="text-2xl md:text-3xl font-bold mb-6">"Comics & Manga"</h1>
            <div class="px-4 py-8 bg-zinc-900/50 rounded-lg border border-white/10">
                <div class="text-white/40 text-sm">"No comics in library"</div>
            </div>
        </div>
    }
}
