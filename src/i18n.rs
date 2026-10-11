//! Interface text. English is compiled in; another language is fetched from /i18n/<lang>.ftl before the app mounts. A missing
//! key falls back to English, then to the key itself, so a gap in a translation never shows an empty string.
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use std::cell::RefCell;

const EN: &str = include_str!("../i18n/en.ftl");
/// Languages with a file in i18n/. Add the code here when a new .ftl file is added.
pub const AVAILABLE: &[&str] = &["en"];
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

/// `?lang=xx` in the address first (for testing and shared links), then the saved choice, then the browser language, then English.
pub fn resolve_language() -> String {
    let wanted = query_language().or_else(stored_language).or_else(|| web_sys::window().and_then(|w| w.navigator().language()));
    let code = wanted.map(|l| l.split('-').next().unwrap_or("en").to_lowercase()).unwrap_or_else(|| "en".into());
    if AVAILABLE.contains(&code.as_str()) { code } else { "en".into() }
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
