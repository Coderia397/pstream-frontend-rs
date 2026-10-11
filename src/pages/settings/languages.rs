//! Display language (profile_settings.ui_language) and the audio and subtitle languages (audio_language, subtitle_language).
use super::prefs::use_prefs;
use super::widgets::ChoiceList;
use crate::data::SUBTITLE_LANGUAGES;
use crate::i18n::t;
use leptos::prelude::*;

/// Value that stands for "follow the display language".
const FOLLOW: &str = "";
/// Value for no subtitles at all.
const OFF: &str = "off";

#[component]
pub fn DisplayLanguage(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let current = RwSignal::new(crate::i18n::language());
    let options = crate::i18n::AVAILABLE.iter().map(|c| (c.to_string(), crate::i18n::native_name(c))).collect::<Vec<_>>();
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-language-hint")}</p>
            <ChoiceList
                options=options
                selected=current.into()
                on_pick=Callback::new(move |code: String| {
                    crate::i18n::set_language(&code);
                    prefs.update_profile(&id.get_value(), |p| p.ui_language = Some(code.clone()));
                    current.set(code);
                })
            />
        </div>
    }
}

fn language_options(extra_first: Vec<(&'static str, &'static str)>) -> Vec<(String, String)> {
    let mut list: Vec<(String, String)> = extra_first.into_iter().map(|(v, key)| (v.to_string(), t(key))).collect();
    list.extend(SUBTITLE_LANGUAGES.iter().map(|l| (l.code.to_string(), crate::i18n::language_name(l.code))));
    list
}

#[component]
pub fn AudioSubtitles(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let audio = Signal::derive(move || prefs.profile(&id.get_value()).audio_language.unwrap_or_else(|| FOLLOW.to_string()));
    let subtitles = Signal::derive(move || prefs.profile(&id.get_value()).subtitle_language.unwrap_or_else(|| FOLLOW.to_string()));
    let none_if_follow = |v: String| if v == FOLLOW { None } else { Some(v) };
    view! {
        <div class="space-y-8">
            <p class="text-body-md text-ink/60">{t("set-audio-subtitles-hint")}</p>
            <section>
                <h2 class="mb-3 ms-1 text-label font-bold uppercase tracking-wider text-ink/50">{t("set-audio-language")}</h2>
                <ChoiceList
                    options=language_options(vec![(FOLLOW, "set-lang-follow")])
                    selected=audio
                    on_pick=Callback::new(move |v: String| prefs.update_profile(&id.get_value(), |p| p.audio_language = none_if_follow(v)))
                />
            </section>
            <section>
                <h2 class="mb-3 ms-1 text-label font-bold uppercase tracking-wider text-ink/50">{t("set-subtitle-language")}</h2>
                <ChoiceList
                    options=language_options(vec![(FOLLOW, "set-lang-follow"), (OFF, "set-subtitles-off")])
                    selected=subtitles
                    on_pick=Callback::new(move |v: String| prefs.update_profile(&id.get_value(), |p| p.subtitle_language = none_if_follow(v)))
                />
            </section>
        </div>
    }
}
