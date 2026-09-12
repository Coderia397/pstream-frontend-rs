use leptos::prelude::*;

#[component]
pub fn HeroSkeleton() -> impl IntoView {
    view! {
        <div class="w-full relative overflow-hidden bg-black select-none">
            // 1. MOBILE SKELETON (< 640px)
            <div class="block sm:hidden relative w-full px-4 pt-[calc(122px+env(safe-area-inset-top))] pb-6 flex flex-col items-center justify-center animate-pulse">
                <div class="absolute inset-x-0 top-0 h-[50vh] bg-gradient-to-b from-white/[0.04] to-transparent pointer-events-none -z-10" />
                <div class="absolute top-[15%] w-[80%] aspect-square rounded-full bg-white/[0.02] filter blur-[40px] pointer-events-none -z-10" />

                <div class="w-full max-w-[440px] aspect-[2/2.9] bg-[#141414] rounded-2xl border border-white/[0.08] shadow-[0_20px_60px_rgba(0,0,0,0.85)] relative flex flex-col justify-end p-4">
                    <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent pointer-events-none" />

                    <div class="relative z-10 w-full flex flex-col items-center gap-3">
                        <div class="h-10 w-1/2 bg-white/10 rounded-md" />
                        <div class="h-3 w-1/3 bg-white/[0.06] rounded" />
                        <div class="flex w-full gap-3 mt-2">
                            <div class="h-[50px] sm:h-[56px] flex-1 bg-white/10 rounded-[4px]" />
                            <div class="h-[50px] sm:h-[56px] flex-1 bg-white/[0.05] rounded-[4px]" />
                        </div>
                    </div>
                </div>
            </div>

            // 2. TABLET SKELETON (640px - 767px)
            <div class="hidden sm:flex md:hidden relative w-full h-[60vh] min-h-[480px] bg-[#141414] flex-col justify-end pl-12 pb-12 pr-6 animate-pulse">
                <div class="absolute inset-0 bg-gradient-to-tr from-black/90 via-black/40 to-transparent pointer-events-none" />
                <div class="absolute inset-0 bg-white/[0.02] pointer-events-none" />

                <div class="relative z-10 w-full max-w-[480px] flex flex-col items-start gap-4">
                    <div class="h-16 w-3/4 bg-white/10 rounded-lg shadow-lg" />
                    <div class="h-4 w-1/2 bg-white/[0.08] rounded" />
                    <div class="flex gap-3 w-full max-w-[340px]">
                        <div class="h-[50px] sm:h-[56px] w-[140px] bg-white/10 rounded-[4px]" />
                        <div class="h-[50px] sm:h-[56px] w-[140px] bg-white/[0.05] rounded-[4px]" />
                    </div>
                </div>
            </div>

            // 3. DESKTOP SKELETON (>= 768px)
            <div class="hidden md:block w-full px-6 md:px-14 pt-20 md:pt-22 pb-2">
                <div class="relative w-full aspect-[16/9] md:aspect-[2.15/1] min-h-[480px] max-h-[72vh] rounded-2xl md:rounded-[24px] overflow-hidden bg-[#181818] border border-white/[0.06] p-8 md:p-12 flex flex-col justify-end animate-pulse shadow-2xl">
                    <div class="relative z-20 w-full max-w-xl flex flex-col items-start gap-3">
                        <div class="h-16 w-3/5 bg-white/10 rounded-lg shadow-2xl" />
                        <div class="flex gap-2">
                            <div class="h-4 w-16 bg-white/10 rounded" />
                            <div class="h-4 w-14 bg-white/10 rounded" />
                            <div class="h-4 w-16 bg-white/10 rounded" />
                            <div class="h-5 w-5 bg-white/20 rounded-full" />
                        </div>
                        <div class="space-y-2 w-full max-w-md">
                            <div class="h-3.5 w-full bg-white/[0.07] rounded" />
                            <div class="h-3.5 w-4/5 bg-white/[0.07] rounded" />
                        </div>
                        <div class="flex gap-3 pt-2">
                            <div class="h-10 w-28 bg-white/15 rounded-full" />
                            <div class="h-10 w-32 bg-white/10 rounded-full" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
