//! Interface text. English is compiled in; another language is fetched from /i18n/<lang>.ftl before the app mounts. A missing
//! key falls back to English, then to the key itself, so a gap in a translation never shows an empty string.
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use std::cell::RefCell;

const EN: &str = include_str!("../i18n/en.ftl");
/// Languages with a file in i18n/. Add the code here when a new .ftl file is added.
/// Languages shown in the pickers and used for the browser default. A language is added here once its file is translated, so
/// nobody lands on a page that is mostly English. Files exist for: ar, de, es, fr, hi, it, ja, ko, nl, pl, pt, ru, sv, tr, zh.
pub const AVAILABLE: &[&str] = &["en"];
/// Every language that has a file. `?lang=xx` in the address previews any of them, also those not in AVAILABLE yet.
pub const KNOWN: &[&str] = &["en", "ar", "de", "es", "fr", "hi", "it", "ja", "ko", "nl", "pl", "pt", "ru", "sv", "tr", "zh"];

/// Name of a language in that language, for pickers.
pub fn native_name(code: &str) -> String {
    match code {
        "en" => "English",
        "ar" => "العربية",
        "de" => "Deutsch",
        "es" => "Español",
        "fr" => "Français",
        "hi" => "हिन्दी",
        "it" => "Italiano",
        "ja" => "日本語",
        "ko" => "한국어",
        "nl" => "Nederlands",
        "pl" => "Polski",
        "pt" => "Português",
        "ru" => "Русский",
        "sv" => "Svenska",
        "tr" => "Türkçe",
        "zh" => "中文",
        other => other,
    }
    .to_string()
}
const RTL: &[&str] = &["ar", "he", "fa", "ur"];

struct State {
    lang: String,
    en: FluentBundle<FluentResource>,
    active: Option<FluentBundle<FluentResource>>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

fn bundle(lang: &str, ftl: &str) -> Option<FluentBundle<FluentResource>> {
    let res = FluentResource::try_new(ftl.to_string()).ok().or_else(|| {
        log::warn!("i18n: {lang} file has errors, using the valid messages only");
        FluentResource::try_new(ftl.to_string()).map_err(|(r, _)| r).err()
    })?;
    let id: unic_langid::LanguageIdentifier = lang.parse().ok()?;
    let mut b = FluentBundle::new(vec![id]);
    b.set_use_isolating(false);
    let _ = b.add_resource(res);
    Some(b)
}

fn stored_language() -> Option<String> {
    web_sys::window()?.local_storage().ok()??.get_item(crate::keys::LANGUAGE).ok()?
}

fn query_language() -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    search.trim_start_matches('?').split('&').find_map(|kv| kv.strip_prefix("lang=").map(|v| v.to_string()))
}

/// The first of the browser's preferred languages that the app has a file for.
fn browser_language() -> Option<String> {
    let nav = web_sys::window()?.navigator();
    let mut tags: Vec<String> = nav.languages().iter().filter_map(|v| v.as_string()).collect();
    if tags.is_empty() {
        tags.extend(nav.language());
    }
    tags.into_iter().map(|t| t.split('-').next().unwrap_or("en").to_lowercase()).find(|c| AVAILABLE.contains(&c.as_str()))
}

/// `?lang=xx` in the address first (for testing and shared links), then the language the visitor picked, then the browser's
/// preferred languages in order, then English. A first visit therefore opens in the browser language.
pub fn resolve_language() -> String {
    let code = |l: String| l.split('-').next().unwrap_or("en").to_lowercase();
    if let Some(preview) = query_language().map(code).filter(|c| KNOWN.contains(&c.as_str())) {
        return preview;
    }
    match stored_language().map(code) {
        Some(c) if AVAILABLE.contains(&c.as_str()) => c,
        _ => browser_language().unwrap_or_else(|| "en".into()),
    }
}

/// True when the visitor has not picked a language yet and the one in use came from the browser.
pub fn is_browser_default() -> bool {
    query_language().or_else(stored_language).is_none()
}

/// Forgets the saved choice so the browser language applies again.
pub fn use_browser_language() {
    if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = s.remove_item(crate::keys::LANGUAGE);
    }
    if let Some(w) = web_sys::window() {
        let _ = w.location().reload();
    }
}

pub fn set_language(code: &str) {
    if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = s.set_item(crate::keys::LANGUAGE, code);
    }
    if let Some(w) = web_sys::window() {
        let _ = w.location().reload();
    }
}

/// Loads the chosen language and sets the page language and direction. Call once before mounting.
pub async fn load() {
    let lang = resolve_language();
    let en = bundle("en", EN).expect("i18n/en.ftl must parse");
    let active = if lang == "en" {
        None
    } else {
        match gloo_net::http::Request::get(&format!("/i18n/{lang}.ftl")).send().await {
            Ok(r) if r.ok() => r.text().await.ok().and_then(|t| bundle(&lang, &t)),
            _ => None,
        }
    };
    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = root.set_attribute("lang", &lang);
        let _ = root.set_attribute("dir", if RTL.contains(&lang.as_str()) { "rtl" } else { "ltr" });
    }
    STATE.with(|s| *s.borrow_mut() = Some(State { lang, en, active }));
}

pub fn language() -> String {
    STATE.with(|s| s.borrow().as_ref().map(|s| s.lang.clone()).unwrap_or_else(|| "en".into()))
}

fn format(b: &FluentBundle<FluentResource>, key: &str, args: Option<&FluentArgs>) -> Option<String> {
    let pattern = b.get_message(key)?.value()?;
    let mut errs = vec![];
    Some(b.format_pattern(pattern, args, &mut errs).into_owned())
}

pub fn t(key: &str) -> String {
    t_args(key, &[])
}

pub fn t_args(key: &str, args: &[(&str, FluentValue)]) -> String {
    let mut fa = FluentArgs::new();
    for (k, v) in args {
        fa.set(*k, v.clone());
    }
    let fa = if args.is_empty() { None } else { Some(&fa) };
    STATE.with(|s| {
        let s = s.borrow();
        let Some(st) = s.as_ref() else { return key.to_string() };
        st.active.as_ref().and_then(|b| format(b, key, fa)).or_else(|| format(&st.en, key, fa)).unwrap_or_else(|| key.to_string())
    })
}

/// A release date (`YYYY-MM-DD`) written short for the reader's language, for example "21 Oct" or "Oct 21". An unreadable date comes back unchanged.
pub fn short_date(iso: &str) -> String {
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&opts, &"day".into(), &"numeric".into());
    let _ = js_sys::Reflect::set(&opts, &"month".into(), &"short".into());
    // noon keeps the day the same in every time zone
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&format!("{}T12:00:00", iso.get(..10).unwrap_or(iso))));
    if date.get_time().is_nan() {
        return iso.to_string();
    }
    date.to_locale_date_string(&language(), &opts).as_string().unwrap_or_else(|| iso.to_string())
}

/// Name of a language in the language the interface is shown in, from the browser's own language data, so no list of names has
/// to be kept in sync. Falls back to the code.
pub fn language_name(code: &str) -> String {
    use wasm_bindgen::JsCast;
    let lang = language();
    let lookup = || -> Option<String> {
        let intl = js_sys::Reflect::get(&js_sys::global(), &"Intl".into()).ok()?;
        let ctor = js_sys::Reflect::get(&intl, &"DisplayNames".into()).ok()?.dyn_into::<js_sys::Function>().ok()?;
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"type".into(), &"language".into()).ok()?;
        let args = js_sys::Array::of2(&js_sys::Array::of1(&lang.as_str().into()), &options);
        let names = js_sys::Reflect::construct(&ctor, &args).ok()?;
        let of = js_sys::Reflect::get(&names, &"of".into()).ok()?.dyn_into::<js_sys::Function>().ok()?;
        of.call1(&names, &code.into()).ok()?.as_string()
    };
    let name = lookup().unwrap_or_else(|| code.to_string());
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => name,
    }
}

/// True when a message with this key exists in the active language or in English.
pub fn has(key: &str) -> bool {
    STATE.with(|s| s.borrow().as_ref().is_some_and(|st| st.active.as_ref().is_some_and(|b| b.has_message(key)) || st.en.has_message(key)))
}

/// Like `t`, but returns `fallback` when no message has this key, so names that come from data can be translated one at a time.
pub fn t_or(key: &str, fallback: &str) -> String {
    let known = STATE.with(|s| s.borrow().as_ref().is_some_and(|st| st.active.as_ref().is_some_and(|b| b.has_message(key)) || st.en.has_message(key)));
    if known { t(key) } else { fallback.to_string() }
}

/// Message key for a genre or category name, for example "Sci-Fi & Fantasy" becomes "genre-sci-fi-fantasy".
pub fn genre_key(name: &str) -> String {
    let mut slug = String::from("genre-");
    for c in name.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_string()
}

/// The shown name of a genre. Matching and links keep using the English name; only what is displayed changes.
pub fn genre_label(name: &str) -> String {
    t_or(&genre_key(name), name)
}

/// The language tag TMDB expects for the interface language, so titles, summaries and genre names come back translated.
pub fn tmdb_language() -> &'static str {
    match language().as_str() {
        "ar" => "ar-SA",
        "de" => "de-DE",
        "es" => "es-ES",
        "fr" => "fr-FR",
        "hi" => "hi-IN",
        "it" => "it-IT",
        "ja" => "ja-JP",
        "ko" => "ko-KR",
        "nl" => "nl-NL",
        "pl" => "pl-PL",
        "pt" => "pt-BR",
        "ru" => "ru-RU",
        "sv" => "sv-SE",
        "tr" => "tr-TR",
        "zh" => "zh-CN",
        _ => "en-US",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every language file must parse without errors, so no message is silently dropped at runtime.
    #[test]
    fn every_language_file_parses() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).expect("i18n dir") {
            let path = entry.expect("entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("ftl") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read");
            if let Err((_, errors)) = FluentResource::try_new(text) {
                panic!("{} has {} errors, first: {:?}", path.display(), errors.len(), errors.first());
            }
            count += 1;
        }
        assert!(count >= 1);
    }

    #[test]
    fn genre_keys_are_slugs() {
        assert_eq!(genre_key("Sci-Fi & Fantasy"), "genre-sci-fi-fantasy");
        assert_eq!(genre_key("Cars, Trucks & Trains"), "genre-cars-trucks-trains");
    }
}

/// The shown form of a mood word such as "Mind-Bending" or "Feel-Good". Words that have no message (for example ones that come
/// from the recommendation service) are shown as they are.
pub fn mood_label(word: &str) -> String {
    let mut key = String::from("mood-");
    for c in word.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            key.push(c);
        } else if !key.ends_with('-') {
            key.push('-');
        }
    }
    t_or(key.trim_end_matches('-'), word)
}
