//! App settings (per device): data use, notifications, about, diagnostics and the legal pages. Everything runs inside the app,
//! the tests read the browser itself.
use super::prefs::use_prefs;
use super::widgets::{ActionRow, ChoiceList, LinkRow, Section, SwitchRow};
use crate::data::settings::{CODEC_PROBES, DATA_USAGE, LEGAL_LINKS, LICENCES};
use crate::i18n::t;
use gloo_net::http::Request;
use js_sys::{Function, Reflect};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

fn window_value(name: &str) -> Option<JsValue> {
    let w = web_sys::window()?;
    Reflect::get(&w, &JsValue::from_str(name)).ok().filter(|v| !v.is_undefined() && !v.is_null())
}

fn call(target: &JsValue, method: &str, args: &[JsValue]) -> Option<JsValue> {
    let f: Function = Reflect::get(target, &JsValue::from_str(method)).ok()?.dyn_into().ok()?;
    let list = js_sys::Array::new();
    for a in args {
        list.push(a);
    }
    f.apply(target, &list).ok()
}

/// True when this browser can play the codec through Media Source (hardware or software, it does not say which).
fn codec_supported(mime: &str) -> bool {
    window_value("MediaSource").and_then(|ms| call(&ms, "isTypeSupported", &[JsValue::from_str(mime)])).and_then(|v| v.as_bool()).unwrap_or(false)
}

fn media_query(query: &str) -> bool {
    web_sys::window().and_then(|w| w.match_media(query).ok().flatten()).is_some_and(|m| m.matches())
}

/// What this browser says about playback: Widevine level, codecs, HDR and the screen.
#[derive(Clone)]
struct Spec {
    widevine: Option<&'static str>,
    codecs: Vec<&'static str>,
    hdr: bool,
    screen: String,
}

/// Asks the browser for a Widevine session at the highest robustness it will grant.
async fn widevine_level() -> Option<&'static str> {
    let navigator = window_value("navigator")?;
    let config = js_sys::Object::new();
    let codecs = js_sys::Array::new();
    let make = |robustness: &str| {
        let c = js_sys::Object::new();
        let _ = Reflect::set(&c, &"contentType".into(), &"video/mp4; codecs=\"avc1.640028\"".into());
        let _ = Reflect::set(&c, &"robustness".into(), &robustness.into());
        c
    };
    for (robustness, label) in [("HW_SECURE_ALL", "L1"), ("SW_SECURE_DECODE", "L3")] {
        codecs.set(0, make(robustness).into());
        let _ = Reflect::set(&config, &"videoCapabilities".into(), &codecs);
        let _ = Reflect::set(&config, &"initDataTypes".into(), &js_sys::Array::of1(&"cenc".into()));
        let list = js_sys::Array::of1(&config);
        if let Some(promise) = call(&navigator, "requestMediaKeySystemAccess", &["com.widevine.alpha".into(), list.into()]) {
            if let Ok(p) = promise.dyn_into::<js_sys::Promise>() {
                if wasm_bindgen_futures::JsFuture::from(p).await.is_ok() {
                    return Some(label);
                }
            }
        }
    }
    None
}

async fn read_spec() -> Spec {
    let codecs = CODEC_PROBES.iter().filter(|(_, mime)| codec_supported(mime)).map(|(name, _)| *name).collect();
    let screen = web_sys::window()
        .and_then(|w| {
            let s = w.screen().ok()?;
            let ratio = w.device_pixel_ratio();
            Some(format!("{} x {}", (s.width().ok()? as f64 * ratio).round(), (s.height().ok()? as f64 * ratio).round()))
        })
        .unwrap_or_default();
    Spec { widevine: widevine_level().await, codecs, hdr: media_query("(dynamic-range: high)"), screen }
}

fn browser_name() -> String {
    web_sys::window().and_then(|w| w.navigator().user_agent().ok()).unwrap_or_default()
}

fn platform() -> String {
    web_sys::window().and_then(|w| w.navigator().platform().ok()).unwrap_or_default()
}

async fn check_network() -> String {
    let online = web_sys::window().map(|w| w.navigator().on_line()).unwrap_or(true);
    if !online {
        return t("set-check-offline");
    }
    let start = js_sys::Date::now();
    match Request::get(&format!("{}/api/providers/health", crate::config::RESOLVER_URL)).send().await {
        Ok(_) => crate::i18n::t_args("set-check-ok", &[("ms", ((js_sys::Date::now() - start).round() as i64).into())]),
        Err(_) => t("set-check-service"),
    }
}

#[component]
pub fn AppSettings() -> impl IntoView {
    let prefs = use_prefs();
    let consent = crate::consent::use_ui();
    let data_open = RwSignal::new(false);
    let net_note = RwSignal::new(None::<String>);
    let net_busy = RwSignal::new(false);
    let spec = RwSignal::new(None::<Spec>);
    let spec_open = RwSignal::new(false);

    let data_label = |v: &str| DATA_USAGE.iter().find(|(value, _)| *value == v).map(|(_, key)| t(key)).unwrap_or_default();
    let data_value = Signal::derive(move || data_label(&prefs.app.get().data_usage));
    let options = DATA_USAGE.iter().map(|(v, key)| (v.to_string(), t(key))).collect::<Vec<_>>();

    let run_check = Callback::new(move |_| {
        if net_busy.get_untracked() {
            return;
        }
        net_busy.set(true);
        net_note.set(Some(t("set-check-running")));
        leptos::task::spawn_local(async move {
            net_note.set(Some(check_network().await));
            net_busy.set(false);
        });
    });
    let show_spec = Callback::new(move |_| {
        spec_open.update(|o| *o = !*o);
        if spec.get_untracked().is_none() {
            leptos::task::spawn_local(async move { spec.set(Some(read_spec().await)) });
        }
    });

    view! {
        <div>
            <Section title=t("set-app-playback")>
                <ActionRow icon="cell-signal-full" title=t("set-data-usage") value=data_value chevron=true on_click=Callback::new(move |_| data_open.update(|o| *o = !*o)) />
            </Section>
            <Show when=move || data_open.get()>
                <div class="-mt-4 mb-8">
                    <ChoiceList
                        options=options.clone()
                        selected=Signal::derive(move || prefs.app.get().data_usage)
                        on_pick=Callback::new(move |v: String| prefs.app.update(|a| a.data_usage = v))
                    />
                </div>
            </Show>

            <Section title=t("set-app-notifications")>
                <SwitchRow
                    icon="bell"
                    title=t("set-allow-notifications")
                    hint=t("set-allow-notifications-hint")
                    checked=Signal::derive(move || prefs.app.get().notifications)
                    on_change=Callback::new(move |v| prefs.app.update(|a| a.notifications = v))
                />
            </Section>

            <Section title=t("set-app-about")>
                <div class="px-5 py-4">
                    <p class="text-body-md font-bold">{t("set-app-device")}</p>
                    <p class="mt-1 break-words text-label text-ink/60">{format!("{} · {}", platform(), browser_name())}</p>
                </div>
                <div class="px-5 py-4">
                    <p class="text-body-md font-bold">{t("set-app-version")}</p>
                    <p class="mt-1 text-label text-ink/60">{env!("CARGO_PKG_VERSION")}</p>
                </div>
            </Section>

            <Section title=t("set-app-diagnostics")>
                <ActionRow icon="wifi-high" title=t("set-check-network") hint=t("set-check-network-hint") on_click=run_check />
                {move || net_note.get().map(|n| view! { <p role="status" class="px-5 py-3 text-body-sm text-ink/70">{n}</p> })}
                <ActionRow icon="film-reel" title=t("set-playback-spec") chevron=true on_click=show_spec />
                <Show when=move || spec_open.get()>
                    <div class="space-y-3 px-5 py-4 text-body-sm">
                        {move || match spec.get() {
                            None => view! { <p class="text-ink/60">{t("set-check-running")}</p> }.into_any(),
                            Some(s) => view! {
                                <p><span class="font-bold">{t("set-spec-drm")}</span>" "{s.widevine.map(|l| format!("Widevine {l}")).unwrap_or_else(|| t("set-spec-none"))}</p>
                                <p><span class="font-bold">{t("set-spec-codecs")}</span>" "{if s.codecs.is_empty() { t("set-spec-none") } else { s.codecs.join(", ") }}</p>
                                <p><span class="font-bold">{t("set-spec-hdr")}</span>" "{if s.hdr { t("set-spec-yes") } else { t("set-spec-no") }}</p>
                                <p><span class="font-bold">{t("set-spec-screen")}</span>" "{s.screen}</p>
                            }.into_any(),
                        }}
                    </div>
                </Show>
            </Section>

            <Section title=t("set-app-legal")>
                {LEGAL_LINKS
                    .iter()
                    .map(|(icon, key, href)| {
                        if href.is_empty() {
                            view! { <ActionRow icon=*icon title=t(key) on_click=Callback::new(move |_| consent.open_center()) /> }.into_any()
                        } else {
                            view! { <LinkRow icon=*icon title=t(key) href=*href /> }.into_any()
                        }
                    })
                    .collect_view()}
            </Section>
        </div>
    }
}

#[component]
pub fn Licences() -> impl IntoView {
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-licences-hint")}</p>
            <Section>
                {LICENCES.iter().map(|(name, licence)| view! {
                    <div class="flex items-center justify-between gap-4 px-5 py-4">
                        <span class="text-body-md font-bold">{*name}</span>
                        <span class="text-label text-ink/60">{*licence}</span>
                    </div>
                }).collect_view()}
            </Section>
            <p class="text-label text-ink/50">{t("set-licences-tmdb")}</p>
        </div>
    }
}
