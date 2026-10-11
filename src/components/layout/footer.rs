use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full bg-page text-mute py-12 px-6 md:px-20 lg:px-32 xl:px-44 2xl:px-56 text-sm mt-auto border-t border-ink/5">
            <div class="max-w-[1000px] mx-auto">
                // Social Icons
                <div class="flex space-x-6 mb-8 text-xl text-ink/70">
                    <i class="ph-fill ph-instagram-logo hover:text-ink cursor-pointer transition-colors"></i>
                    <i class="ph-fill ph-twitter-logo hover:text-ink cursor-pointer transition-colors"></i>
                    <i class="ph-fill ph-youtube-logo hover:text-ink cursor-pointer transition-colors"></i>
                    <a href="https://github.com" target="_blank" rel="noreferrer" class="hover:text-ink transition-colors">
                        <i class="ph-fill ph-github-logo"></i>
                    </a>
                </div>

                // Links Grid
                <nav class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-y-4 gap-x-8 mb-8 text-xs md:text-sm">
                    <a href="/browse" class="hover:underline hover:text-ink transition-colors">"Home"</a>
                    <a href="/browse/shows" class="hover:underline hover:text-ink transition-colors">"TV Shows"</a>
                    <a href="/browse/movies" class="hover:underline hover:text-ink transition-colors">"Movies"</a>
                    <a href="/latest" class="hover:underline hover:text-ink transition-colors">"New & Popular"</a>
                    <a href="/browse/my-list" class="hover:underline hover:text-ink transition-colors">"My List"</a>
                    <a href="/settings" class="hover:underline hover:text-ink transition-colors">"Settings"</a>
                    <a href="/privacy" class="hover:underline hover:text-ink transition-colors">"Privacy Policy"</a>
                    <a href="/terms" class="hover:underline hover:text-ink transition-colors">"Terms of Service"</a>
                    <a href="/dmca" class="hover:underline hover:text-ink transition-colors">"DMCA"</a>
                    <a href="/disclaimer" class="hover:underline hover:text-ink transition-colors">"Disclaimer"</a>
                    <a href="/cookies" class="hover:underline hover:text-ink transition-colors">"Cookie Policy"</a>
                    <a href="/contact" class="hover:underline hover:text-ink transition-colors">"Contact Us"</a>
                </nav>

                // Copyright
                <div class="text-xs text-ink/30 mt-8">
                    {format!("\u{a9} 2026 {}", crate::brand::COMPANY)}
                </div>
            </div>
        </footer>
    }
}
