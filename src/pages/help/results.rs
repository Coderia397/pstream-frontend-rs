use super::shell::{set_title, HelpShell};
use super::widgets::{highlighted, SearchBox};
use super::{art, search, tr, tr_with};
use crate::data::help::POPULAR;
use fluent_bundle::FluentValue;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

const PER_PAGE: usize = 6;

#[component]
pub fn ResultsPage() -> impl IntoView {
    let query = use_query_map();
    let q = Memo::new(move |_| query.read().get("q").unwrap_or_default());
    let (page, set_page) = signal(0usize);
    Effect::new(move |_| {
        q.track();
        set_page.set(0);
        set_title(&tr("help-ui-search-label"));
    });
    let hits = Memo::new(move |_| search::search(&q.get()));

    view! {
        <HelpShell>
            <main class="mx-auto w-full max-w-[720px] px-6 pb-20 pt-8">
                <nav class="mb-5 flex items-center gap-2 text-body-sm text-paper-mute" aria-label="Breadcrumb">
                    <a href="/help" class="underline underline-offset-2 hover:text-brand-active">{tr("help-ui-center")}</a>
                    <i class="ph-bold ph-caret-right text-fine" aria-hidden="true"></i>
                    <span>{tr("help-ui-search-label")}</span>
                </nav>
                {move || view! { <SearchBox initial=q.get() placeholder="help-ui-search-placeholder" /> }}

                {move || {
                    let words = search::tokens(&q.get());
                    let all = hits.get();
                    if q.get().trim().is_empty() {
                        return view! { <p class="mt-10 text-body-md text-paper-mute">{tr("help-ui-results-empty")}</p> }.into_any();
                    }
                    if all.is_empty() {
                        return view! {
                            <div class="mt-10" role="status">
                                <h2 class="text-title font-bold text-paper-ink">{tr_with("help-ui-results-none", &[("query", FluentValue::from(q.get()))])}</h2>
                                <p class="mt-2 text-body-md text-paper-mute">{tr("help-ui-results-hint")}</p>
                                <h3 class="mb-3 mt-8 text-body-md font-bold text-paper-ink">{tr("help-ui-popular")}</h3>
                                <ul class="space-y-3">
                                    {POPULAR.iter().map(|s| view! { <li><a href=format!("/help/article/{s}") class="text-body-md text-paper-ink underline underline-offset-2 hover:text-brand-active">{art(s, "title")}</a></li> }).collect_view()}
                                </ul>
                            </div>
                        }
                        .into_any();
                    }
                    let best = all[0].clone();
                    let rest: Vec<_> = all[1..].to_vec();
                    let pages = rest.len().div_ceil(PER_PAGE).max(1);
                    let current = page.get().min(pages - 1);
                    let shown: Vec<_> = rest.iter().skip(current * PER_PAGE).take(PER_PAGE).cloned().collect();
                    let count_note = tr_with("help-ui-results-count", &[("count", FluentValue::from(all.len())), ("query", FluentValue::from(q.get()))]);
                    let best_summary = art(best.slug, "summary");
                    let best_title = art(best.slug, "title");
                    view! {
                        <p class="sr-only" role="status">{count_note}</p>
                        {(current == 0).then(|| view! {
                            <section class="mt-10 border-b border-paper-line pb-8">
                                <p class="mb-4 text-body-sm font-semibold text-paper-ink">{tr("help-ui-recommended")}</p>
                                <h2 class="mb-3 text-heading-md font-bold text-paper-ink">{best_title}</h2>
                                <p class="mb-6 text-body-md leading-7 text-paper-mute">{highlighted(&best_summary, &words)}</p>
                                <a href=format!("/help/article/{}", best.slug) class="inline-block rounded-control bg-void px-5 py-3 text-body-sm font-bold text-ink hover:bg-brand-deep">{tr("help-ui-read")}</a>
                            </section>
                        })}
                        {(!shown.is_empty()).then(|| view! {
                            <section class="mt-8">
                                <h2 class="mb-4 text-body-sm font-semibold text-paper-ink">{tr("help-ui-additional")}</h2>
                                <ul>
                                    {shown.iter().map(|h| {
                                        let summary = art(h.slug, "summary");
                                        view! {
                                            <li class="border-b border-paper-line py-5 first:pt-1">
                                                <a href=format!("/help/article/{}", h.slug) class="text-body-lg font-semibold text-paper-ink underline underline-offset-2 hover:text-brand-active">{art(h.slug, "title")}</a>
                                                <p class="mt-2 text-body-sm leading-6 text-paper-mute">{highlighted(&summary, &words)}</p>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </section>
                        })}
                        {(pages > 1).then(|| view! {
                            <nav class="mt-8 flex items-center gap-1" aria-label="Pagination">
                                <button
                                    aria-label=tr("help-ui-page-prev")
                                    disabled=move || page.get() == 0
                                    on:click=move |_| set_page.update(|p| *p = p.saturating_sub(1))
                                    class="flex h-9 w-9 items-center justify-center rounded-control text-paper-ink hover:bg-paper-soft disabled:opacity-30"
                                >
                                    <i class="ph-bold ph-caret-left"></i>
                                </button>
                                {(0..pages).map(|n| view! {
                                    <button
                                        aria-label=tr_with("help-ui-page-n", &[("n", FluentValue::from(n + 1))])
                                        aria-current=move || (page.get() == n).then_some("page")
                                        on:click=move |_| set_page.set(n)
                                        class="h-9 min-w-9 rounded-control px-2 text-body-sm text-paper-ink hover:bg-paper-soft"
                                        class=("bg-paper-soft", move || page.get() == n)
                                        class=("font-bold", move || page.get() == n)
                                    >
                                        {n + 1}
                                    </button>
                                }).collect_view()}
                                <button
                                    aria-label=tr("help-ui-page-next")
                                    disabled=move || page.get() + 1 >= pages
                                    on:click=move |_| set_page.update(|p| *p += 1)
                                    class="flex h-9 w-9 items-center justify-center rounded-control text-paper-ink hover:bg-paper-soft disabled:opacity-30"
                                >
                                    <i class="ph-bold ph-caret-right"></i>
                                </button>
                            </nav>
                        })}
                    }
                    .into_any()
                }}
            </main>
        </HelpShell>
    }
}
