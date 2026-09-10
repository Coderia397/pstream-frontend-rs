use leptos::prelude::*;
use leptos::portal::Portal;
use crate::models::movie::Movie;

#[component]
pub fn ContinueWatchingOptionsSheet(
    movie: Option<Movie>,
    on_close: Callback<()>,
    on_remove: Callback<Movie>,
) -> impl IntoView {
    let has_movie = movie.is_some();

    // Body scroll lock effect
    Effect::new(move |_| {
        if let Some(w) = web_sys::window() {
            if let Some(doc) = w.document() {
                if let Some(body) = doc.body() {
                    if has_movie {
                        let _ = body.style().set_property("overflow", "hidden");
                    } else {
                        let _ = body.style().set_property("overflow", "");
                    }
                }
            }
        }
    });

    let movie_stored = StoredValue::new(movie);
    let on_close_stored = StoredValue::new(on_close);
    let on_remove_stored = StoredValue::new(on_remove);

    view! {
        <Portal>
            {move || if let Some(m) = movie_stored.get_value() {
                let title = m.display_title();
                let m_for_remove = m.clone();

                let handle_remove = move |_| {
                    on_remove_stored.get_value().run(m_for_remove.clone());
                    on_close_stored.get_value().run(());
                };

                let handle_close = move |_| {
                    on_close_stored.get_value().run(());
                };

                view! {
                    <div class="fixed inset-0 z-[10040] flex flex-col justify-end">
                        // Backdrop
                        <div
                            class="fixed inset-0 bg-black/60 transition-opacity duration-200"
                            on:click=handle_close
                        />

                        // Sheet content
                        <div class="relative z-[10050] bg-[#1f1f1f] rounded-t-2xl px-4 pt-2 pb-8 max-w-lg mx-auto w-full shadow-2xl transition-transform duration-300">
                            <div class="w-10 h-1 rounded-full bg-white/30 mx-auto mb-4" />

                            <div class="px-1 mb-4">
                                <span class="text-white text-[17px] font-bold truncate block">
                                    {title}
                                </span>
                            </div>

                            <button
                                type="button"
                                on:click=handle_remove
                                class="w-full flex items-center gap-4 bg-[#2a2a2a] rounded-xl px-4 py-4 active:scale-[0.99] active:bg-[#333] transition-all cursor-pointer"
                            >
                                <i class="ph-bold ph-x-circle text-white text-2xl shrink-0"></i>
                                <span class="text-white text-[16px] font-medium">
                                    "Remove from Continue Watching"
                                </span>
                            </button>

                            <button
                                type="button"
                                on:click=handle_close
                                class="w-full text-center text-white/60 text-[15px] font-medium py-4 mt-1 active:text-white/90 cursor-pointer"
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span /> }.into_any()
            }}
        </Portal>
    }
}
