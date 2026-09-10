use leptos::prelude::*;

// ── Shared wrapper ─────────────────────────────────────────────────────────────

/// Mirrors the outer wrapper every legal page uses in React.
#[component]
fn LegalWrapper(children: Children) -> impl IntoView {
    view! {
        <div class="pt-20 md:pt-24 flex-1 bg-black md:bg-[#141414] text-white font-sans transition-colors duration-200">
            <main class="max-w-4xl mx-auto px-6 py-16 md:py-24 text-[#e5e5e5] space-y-10 md:space-y-12">
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
            <h1 class="text-4xl md:text-5xl font-bold text-white tracking-tight mb-2">{title}</h1>
            <p class="text-base text-[#808080] font-semibold mt-4 mb-10 block">{last_updated}</p>
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
    view! {
        <section class="space-y-5">
            <h2 class="text-2xl md:text-3xl font-bold text-white tracking-tight mb-4">{title}</h2>
            <p class="text-base md:text-[17px] leading-8 text-[#b3b3b3]">{body}</p>
            {if !bullets.is_empty() {
                view! {
                    <ul class="list-disc pl-8 space-y-3 text-base md:text-[17px] leading-8 text-[#b3b3b3] marker:text-[#808080]">
                        {bullets.iter().map(|b| view! {
                            <li inner_html=*b />
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
            <LegalHeader title="Privacy Policy" last_updated="Last updated: August 2026" />

            <LegalSection
                title="1. Introduction"
                body="Pstream is a link aggregator and search engine. This Privacy Policy explains how we collect, use, and protect your personal information when you use our service."
            />

            <LegalSection
                title="2. Information We Collect"
                body="We collect limited personal information to provide and improve the service:"
                bullets=&[
                    "<strong>Account Information:</strong> Email address and display name when you create an account.",
                    "<strong>Usage Data:</strong> Anonymised viewing history and preferences, scoped to your profile.",
                    "<strong>Session Cookies:</strong> Used to keep you signed in securely.",
                ]
            />

            <LegalSection
                title="3. What We Do Not Do"
                body="We are committed to your privacy:"
                bullets=&[
                    "<strong>Share Your Data:</strong> We do not share personal data with advertisers or data brokers.",
                    "<strong>Sell Your Data:</strong> We do not sell, rent, or trade any personal data to third parties, ever.",
                    "<strong>Fingerprint Your Browser:</strong> We do not perform browser fingerprinting or any form of device identification.",
                ]
            />

            <LegalSection
                title="4. Lawful Basis for Processing (UK GDPR)"
                body="Under the UK General Data Protection Regulation (UK GDPR), the lawful bases we rely on for processing this information are:"
                bullets=&[
                    "<strong>Contractual Necessity:</strong> To provide you with the link aggregation and profile features you signed up for.",
                    "<strong>Legitimate Interests:</strong> To analyse general platform usage and improve performance, using strictly anonymised, aggregated data.",
                ]
            />

            <LegalSection
                title="5. How We Store Your Data"
                body="Your data is securely stored using Supabase (our backend infrastructure provider). We use Row-Level Security (RLS) policies to ensure that only you can access your data."
            />

            <LegalSection
                title="6. Third-Party Services"
                body="We use third-party APIs including TMDB (for metadata) and Cloudflare Turnstile (for bot protection). These services operate under their own privacy policies."
            />

            <LegalSection
                title="7. Your Data Protection Rights"
                body="Under UK GDPR, you have the following rights:"
                bullets=&[
                    "<strong>Right of Access:</strong> Request a copy of the personal data we hold about you.",
                    "<strong>Right to Erasure:</strong> Request deletion of your personal data (the \"Right to be Forgotten\"). You can delete your account directly from the Settings page.",
                    "<strong>Right to Rectification:</strong> Request correction of any inaccurate personal data we hold.",
                    "<strong>Right to Object:</strong> Object to processing of your personal data where we rely on legitimate interests.",
                ]
            />

            <LegalSection
                title="8. Contact"
                body="If you have questions about this Privacy Policy or wish to exercise your data rights, please contact the repository maintainer at support@pstream.watch."
            />
        </LegalWrapper>
    }
}

// ── Terms Page ─────────────────────────────────────────────────────────────────

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="Terms of Service" last_updated="Last updated: August 2026" />

            <LegalSection
                title="1. Acceptance of Terms"
                body="By accessing or using Pstream, you agree to be bound by these Terms of Service and our Privacy Policy. If you do not agree, do not use the service."
            />
            <LegalSection
                title="2. Nature of the Service"
                body="Pstream is a link aggregator and search engine. We do not host, upload, transmit, or control any video content on our servers. All media is sourced from third-party platforms."
            />
            <LegalSection
                title="3. User Accounts"
                body="To access certain features (such as saving a watchlist or syncing progress), you must create an account. You are responsible for maintaining the security of your account credentials."
            />
            <LegalSection
                title="4. Acceptable Use"
                body="You agree not to use the service to:"
                bullets=&[
                    "Circumvent, disable, or interfere with security-related features of the service.",
                    "Use automated tools (such as bots, scrapers, or crawlers) to access or query the service at a rate that could impair performance.",
                    "Attempt to probe, scan, or test the vulnerability of the service's infrastructure.",
                    "Engage in any conduct that is unlawful, harmful, or that we determine to be inappropriate.",
                ]
            />
            <LegalSection
                title="5. Intellectual Property"
                body="The Pstream brand, interface design, and original software code are protected by copyright. All media content accessible via Pstream remains the property of its respective rights holders."
            />
            <LegalSection
                title="6. Disclaimers and Limitation of Liability"
                body="The service is provided \"as is\" and \"as available\" without any warranty of any kind. Pstream does not guarantee continuous availability and is not liable for any indirect or consequential damages."
            />
            <LegalSection
                title="7. Third-Party Content"
                body="Pstream provides access to links pointing to content hosted on independent third-party servers. We have no control over, and assume no responsibility for, the content, privacy policies, or practices of any third-party sites."
            />
            <LegalSection
                title="8. Termination"
                body="We reserve the right to suspend or terminate your account and access to the service at our sole discretion, without prior notice, for conduct that we believe violates these Terms or is harmful to other users, us, or third parties."
            />
            <LegalSection
                title="9. Governing Law"
                body="These Terms of Service shall be governed by and construed in accordance with the laws of England and Wales, without regard to conflict-of-law provisions."
            />
            <LegalSection
                title="10. Contact"
                body="For any questions regarding these Terms of Service, please contact us via the Contact page."
            />
        </LegalWrapper>
    }
}

// ── Cookie Policy Page ─────────────────────────────────────────────────────────

#[component]
pub fn CookiePolicyPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="Cookie Policy" last_updated="Last Updated: August 16, 2026" />
            <LegalSection
                title="How we use cookies"
                body="We use cookies and similar tracking technologies to track the activity on our service and hold certain information. Cookies are files with small amount of data which may include an anonymous unique identifier."
            />
        </LegalWrapper>
    }
}

// ── DMCA Page ──────────────────────────────────────────────────────────────────

#[component]
pub fn DmcaPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="DMCA Safe Harbor Notice" last_updated="Last updated: August 2026" />

            <LegalSection
                title="1. Our Role — Link Aggregator, Not a Host"
                body="Pstream is a link aggregator and search engine. We do not host, upload, store, transmit, or control any video or media content. All content is served from third-party servers that are entirely outside of our control."
            />
            <LegalSection
                title="2. Notice-and-Takedown Procedure"
                body="If you are a copyright owner, or an agent authorised to act on behalf of a copyright owner, and you believe that a link in our index infringes your copyright, please send a written notice to support@pstream.watch containing:"
                bullets=&[
                    "<strong>Physical or electronic signature</strong> of the copyright owner or their authorised agent.",
                    "<strong>Identification of the copyrighted work</strong> you claim has been infringed, or if multiple works are covered by a single notice, a representative list of such works.",
                    "<strong>Identification of the material</strong> (i.e. the specific Pstream link or URL) that you claim is infringing, with sufficient information to permit us to locate it.",
                ]
            />
            <LegalSection
                title="3. Additional Required Elements"
                body="Your DMCA notice must also include:"
                bullets=&[
                    "<strong>Your contact information</strong> — name, address, telephone number, and email address.",
                    "<strong>A statement of good faith</strong> — that you have a good faith belief that the use of the material in the manner complained of is not authorised by the copyright owner, its agent, or the law.",
                    "<strong>A statement of accuracy</strong> — that the information in the notice is accurate and, under penalty of perjury, that you are authorised to act on behalf of the copyright owner.",
                    "<strong>Send notices to:</strong> support@pstream.watch",
                    "We will review all notices and remove links from our index that satisfy the above statutory requirements.",
                    "<strong>Counter-Notification:</strong> If you believe your link was removed in error, you may submit a counter-notification under 17 U.S.C. § 512(g).",
                ]
            />
            <LegalSection
                title="4. Important Limitation — We Can Only Remove Links"
                body="Because Pstream does not host any media files, we are only able to remove the link from our search index. We cannot remove the underlying content from the server on which it is hosted."
            />
            <LegalSection
                title="5. Repeat Infringer Policy"
                body="In appropriate circumstances, and at our sole discretion, Pstream will disable or terminate access for users or accounts that are repeat infringers."
            />
            <LegalSection
                title="6. No Admission of Liability"
                body="Removal of a link from our index in response to a DMCA notice does not constitute an admission by Pstream that any infringement has occurred."
            />
        </LegalWrapper>
    }
}

// ── Disclaimer Page ────────────────────────────────────────────────────────────

#[component]
pub fn DisclaimerPage() -> impl IntoView {
    view! {
        <LegalWrapper>
            <LegalHeader title="Disclaimer" last_updated="Last updated: August 2026" />

            <LegalSection
                title="1. No Content Hosting"
                body="Pstream is a link aggregator and search engine for publicly available embedded media links. We do not host, upload, store, transmit, or control any video, audio, or media content on our servers."
            />
            <LegalSection
                title="2. Third-Party Content — No Liability"
                body="All content accessible via links indexed by Pstream originates from external, independent third-party sources. Pstream has no control over, and assumes no responsibility for, the legality, accuracy, quality, or availability of such content."
            />
            <LegalSection
                title="3. User Assumption of Risk"
                body="You acknowledge and agree that your use of Pstream and your access to any content via external embedded links is entirely at your own risk. Pstream expressly disclaims all warranties, express or implied."
            />
            <LegalSection
                title="4. Link Availability"
                body="Embedded links indexed by Pstream point to resources on third-party servers. We cannot guarantee the ongoing availability, quality, or completeness of any linked content, and links may break or become unavailable without notice."
            />
            <LegalSection
                title="5. Accuracy of Information"
                body="Metadata displayed on Pstream (including titles, descriptions, ratings, and images) is sourced from third-party data providers such as TMDB. We do not guarantee the accuracy or completeness of this metadata."
            />
        </LegalWrapper>
    }
}

// ── Contact Page ───────────────────────────────────────────────────────────────

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="pt-20 md:pt-24 flex-1 bg-black md:bg-[#141414] text-white font-sans transition-colors duration-200">
            <main class="max-w-3xl mx-auto px-6 py-12 md:py-20 text-gray-300 space-y-8">
                <div>
                    <h1 class="text-3xl md:text-5xl font-black text-white mb-4">"Contact Us"</h1>
                </div>
                <section class="space-y-4">
                    <p>"If you have any questions or concerns about our services, please contact us at:"</p>
                    <a href="mailto:support@pstream.watch" class="text-red-500 hover:underline">
                        "support@pstream.watch"
                    </a>
                </section>
            </main>
        </div>
    }
}
