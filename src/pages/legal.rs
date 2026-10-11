use leptos::prelude::*;

/// Looks up a legal text with the product name and contact address filled in.
fn legal_text(key: &str) -> String {
    crate::i18n::t_args(
        key,
        &[("brand", fluent_bundle::FluentValue::from(crate::brand::NAME)), ("support_email", fluent_bundle::FluentValue::from(crate::brand::SUPPORT_EMAIL))],
    )
}

// ── Shared wrapper ─────────────────────────────────────────────────────────────

/// Mirrors the outer wrapper every legal page uses in React.
#[component]
fn LegalWrapper(children: Children) -> impl IntoView {
    view! {
        <div class="pt-20 md:pt-24 flex-1 bg-void md:bg-page text-ink font-sans transition-colors duration-(--motion-base)">
            <main class="max-w-4xl mx-auto px-6 py-16 md:py-24 text-ink-soft space-y-10 md:space-y-12">
                {children()}
            </main>
        </div>
    }
}

/// Title + last-updated paragraph block — reused by every standard legal page.
#[component]
fn LegalHeader(title: &'static str, last_updated: &'static str) -> impl IntoView {
    view! {
        <div>
            <h1 class="text-4xl md:text-5xl font-bold text-ink tracking-tight mb-2">{legal_text(title)}</h1>
            <p class="text-base text-mute font-semibold mt-4 mb-10 block">{legal_text(last_updated)}</p>
        </div>
    }
}

/// A single section: h2 + body paragraph + optional bullet list.
/// Bullets may contain HTML (for <strong> tags mirroring <Trans />).
#[component]
fn LegalSection(
    title: &'static str,
    body: &'static str,
    #[prop(default = &[])] bullets: &'static [&'static str],
) -> impl IntoView {
    let title = legal_text(title);
    let body = legal_text(body);
    view! {
        <section class="space-y-5">
            <h2 class="text-2xl md:text-3xl font-bold text-ink tracking-tight mb-4">{title}</h2>
            <p class="text-base md:text-body-lg leading-8 text-ink-body">{body}</p>
            {if !bullets.is_empty() {
                view! {
                    <ul class="list-disc pl-8 space-y-3 text-base md:text-body-lg leading-8 text-ink-body marker:text-mute">
                        {bullets.iter().map(|b| view! {
                            <li inner_html=legal_text(b) />
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </section>
    }
}

// ── Privacy Page ───────────────────────────────────────────────────────────────

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="legal-privacy-title" last_updated="legal-privacy-updated" />

            <LegalSection
                title="legal-privacy-1-title"
                body="legal-privacy-1-body"
            />

            <LegalSection
                title="legal-privacy-2-title"
                body="legal-privacy-2-body"
                bullets=&["legal-privacy-2-b1", "legal-privacy-2-b2", "legal-privacy-2-b3"]
            />

            <LegalSection
                title="legal-privacy-3-title"
                body="legal-privacy-3-body"
                bullets=&["legal-privacy-3-b1", "legal-privacy-3-b2", "legal-privacy-3-b3"]
            />

            <LegalSection
                title="legal-privacy-4-title"
                body="legal-privacy-4-body"
                bullets=&["legal-privacy-4-b1", "legal-privacy-4-b2"]
            />

            <LegalSection
                title="legal-privacy-5-title"
                body="legal-privacy-5-body"
            />

            <LegalSection
                title="legal-privacy-6-title"
                body="legal-privacy-6-body"
            />

            <LegalSection
                title="legal-privacy-7-title"
                body="legal-privacy-7-body"
                bullets=&["legal-privacy-7-b1", "legal-privacy-7-b2", "legal-privacy-7-b3", "legal-privacy-7-b4"]
            />

            <LegalSection
                title="legal-privacy-8-title"
                body="legal-privacy-8-body"
            />
        </LegalWrapper>
    }
}

// ── Terms Page ─────────────────────────────────────────────────────────────────

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="legal-terms-title" last_updated="legal-terms-updated" />

            <LegalSection
                title="legal-terms-1-title"
                body="legal-terms-1-body"
            />
            <LegalSection
                title="legal-terms-2-title"
                body="legal-terms-2-body"
            />
            <LegalSection
                title="legal-terms-3-title"
                body="legal-terms-3-body"
            />
            <LegalSection
                title="legal-terms-4-title"
                body="legal-terms-4-body"
                bullets=&["legal-terms-4-b1", "legal-terms-4-b2", "legal-terms-4-b3", "legal-terms-4-b4"]
            />
            <LegalSection
                title="legal-terms-5-title"
                body="legal-terms-5-body"
            />
            <LegalSection
                title="legal-terms-6-title"
                body="legal-terms-6-body"
            />
            <LegalSection
                title="legal-terms-7-title"
                body="legal-terms-7-body"
            />
            <LegalSection
                title="legal-terms-8-title"
                body="legal-terms-8-body"
            />
            <LegalSection
                title="legal-terms-9-title"
                body="legal-terms-9-body"
            />
            <LegalSection
                title="legal-terms-10-title"
                body="legal-terms-10-body"
            />
        </LegalWrapper>
    }
}

// ── Cookie Policy Page ─────────────────────────────────────────────────────────

#[component]
pub fn CookiePolicyPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="legal-cookie-policy-title" last_updated="legal-cookie-policy-updated" />
            <LegalSection
                title="legal-cookie-policy-1-title"
                body="legal-cookie-policy-1-body"
            />
        </LegalWrapper>
    }
}

// ── DMCA Page ──────────────────────────────────────────────────────────────────

#[component]
pub fn DmcaPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="legal-dmca-title" last_updated="legal-dmca-updated" />

            <LegalSection
                title="legal-dmca-1-title"
                body="legal-dmca-1-body"
            />
            <LegalSection
                title="legal-dmca-2-title"
                body="legal-dmca-2-body"
                bullets=&["legal-dmca-2-b1", "legal-dmca-2-b2", "legal-dmca-2-b3"]
            />
            <LegalSection
                title="legal-dmca-3-title"
                body="legal-dmca-3-body"
                bullets=&["legal-dmca-3-b1", "legal-dmca-3-b2", "legal-dmca-3-b3", "legal-dmca-3-b4", "legal-dmca-3-b5", "legal-dmca-3-b6"]
            />
            <LegalSection
                title="legal-dmca-4-title"
                body="legal-dmca-4-body"
            />
            <LegalSection
                title="legal-dmca-5-title"
                body="legal-dmca-5-body"
            />
            <LegalSection
                title="legal-dmca-6-title"
                body="legal-dmca-6-body"
            />
        </LegalWrapper>
    }
}

// ── Disclaimer Page ────────────────────────────────────────────────────────────

#[component]
pub fn DisclaimerPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="legal-disclaimer-title" last_updated="legal-disclaimer-updated" />

            <LegalSection
                title="legal-disclaimer-1-title"
                body="legal-disclaimer-1-body"
            />
            <LegalSection
                title="legal-disclaimer-2-title"
                body="legal-disclaimer-2-body"
            />
            <LegalSection
                title="legal-disclaimer-3-title"
                body="legal-disclaimer-3-body"
            />
            <LegalSection
                title="legal-disclaimer-4-title"
                body="legal-disclaimer-4-body"
            />
            <LegalSection
                title="legal-disclaimer-5-title"
                body="legal-disclaimer-5-body"
            />
        </LegalWrapper>
    }
}

// ── Contact Page ───────────────────────────────────────────────────────────────

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="pt-20 md:pt-24 flex-1 bg-void md:bg-page text-ink font-sans transition-colors duration-(--motion-base)">
            <main class="max-w-3xl mx-auto px-6 py-12 md:py-20 text-gray-300 space-y-8">
                <div>
                    <h1 class="text-3xl md:text-5xl font-black text-ink mb-4">{legal_text("legal-contact-title")}</h1>
                </div>
                <section class="space-y-4">
                    <p>{legal_text("legal-contact-intro")}</p>
                    <a href=format!("mailto:{}", crate::brand::SUPPORT_EMAIL) class="text-brand hover:underline">
                        {crate::brand::SUPPORT_EMAIL}
                    </a>
                </section>
            </main>
        </div>
    }
}
