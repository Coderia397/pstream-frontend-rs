//! Notification settings of a profile: an overview with Email and Push (each On or Off), and a page per channel with a master
//! switch and one switch per topic. Channels and topics are data (data::settings), the saved keys are "<channel>" and
//! "<channel>.<topic>". Turning push on asks the browser for permission.
use super::prefs::use_prefs;
use super::widgets::{toast, LinkRow, Message, Section, SwitchRow, Tone};
use super::Screen;
use crate::data::settings::{NOTIFICATION_CHANNELS, NOTIFICATION_TOPICS};
use crate::i18n::t;
use js_sys::{Function, Reflect};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

/// What the browser says about notification permission: "granted", "denied", "default", or "unsupported".
pub fn push_permission() -> String {
    let Some(window) = web_sys::window() else { return "unsupported".into() };
    let Ok(ctor) = Reflect::get(&window, &JsValue::from_str("Notification")) else { return "unsupported".into() };
    if ctor.is_undefined() {
        return "unsupported".into();
    }
    Reflect::get(&ctor, &JsValue::from_str("permission")).ok().and_then(|v| v.as_string()).unwrap_or_else(|| "unsupported".into())
}

/// Asks the browser for permission to show notifications and tells `done` whether it was given.
pub fn request_push_permission(done: Callback<bool>) {
    let Some(window) = web_sys::window() else { return done.run(false) };
    let Ok(ctor) = Reflect::get(&window, &JsValue::from_str("Notification")) else { return done.run(false) };
    if ctor.is_undefined() {
        return done.run(false);
    }
    let Some(f) = Reflect::get(&ctor, &JsValue::from_str("requestPermission")).ok().and_then(|f| f.dyn_into::<Function>().ok()) else { return done.run(false) };
    let Ok(promise) = f.call0(&ctor) else { return done.run(false) };
    let Ok(promise) = promise.dyn_into::<js_sys::Promise>() else { return done.run(false) };
    leptos::task::spawn_local(async move {
        let answer = wasm_bindgen_futures::JsFuture::from(promise).await.ok().and_then(|v| v.as_string());
        done.run(answer.as_deref() == Some("granted"));
    });
}

fn topic_key(channel: &str, topic: &str) -> String {
    format!("{channel}.{topic}")
}

#[component]
pub fn Notifications(id: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    view! {
        <div>
            <p class="mb-5 text-body-md text-ink/60">{t("set-notifications-body")}</p>
            <Section>
                {NOTIFICATION_CHANNELS
                    .iter()
                    .map(|(channel, label_key, hint_key)| {
                        let channel = *channel;
                        let value = Signal::derive(move || if prefs.profile(&id.get_value()).notify(channel) { t("set-lock-on") } else { t("set-lock-off") });
                        view! { <LinkRow icon=if channel == "push" { "bell" } else { "envelope-simple" } title=t(label_key) hint=t(hint_key) href=Screen::NotificationChannel(id.get_value(), channel.to_string()).path() value=value /> }
                    })
                    .collect_view()}
            </Section>
        </div>
    }
}

#[component]
pub fn Channel(id: String, channel: String) -> impl IntoView {
    let prefs = use_prefs();
    let id = StoredValue::new(id);
    let channel = StoredValue::new(channel);
    let note = RwSignal::new(None::<(Tone, String)>);
    let master = Signal::derive(move || prefs.profile(&id.get_value()).notify(&channel.get_value()));
    let set = move |key: String, on: bool| {
        prefs.update_profile(&id.get_value(), |p| {
            p.notifications.insert(key, on);
        });
        toast(t("set-saved"));
    };
    let set_master = Callback::new(move |on: bool| {
        let key = channel.get_value();
        if on && key == "push" {
            request_push_permission(Callback::new(move |granted| {
                if granted {
                    note.set(None);
                    set("push".to_string(), true);
                } else {
                    note.set(Some((Tone::Error, t(if push_permission() == "unsupported" { "set-push-unsupported" } else { "set-push-denied" }))));
                }
            }));
        } else {
            note.set(None);
            set(key, on);
        }
    });
    view! {
        <div>
            <Message text=note />
            <Section>
                <SwitchRow
                    icon=if channel.get_value() == "push" { "bell" } else { "envelope-simple" }
                    title=t(NOTIFICATION_CHANNELS.iter().find(|(c, _, _)| *c == channel.get_value()).map(|(_, label, _)| *label).unwrap_or("set-notify-email"))
                    checked=master
                    on_change=set_master
                />
            </Section>
            <Show when=move || master.get()>
                <Section title=t("set-topics-title")>
                    {NOTIFICATION_TOPICS
                        .iter()
                        .filter(|(_, channels)| channels.contains(&channel.get_value().as_str()))
                        .map(|(topic, _)| {
                            let key = topic_key(&channel.get_value(), topic);
                            let read = key.clone();
                            view! {
                                <SwitchRow
                                    icon="bell-simple"
                                    title=t(&format!("set-topic-{topic}"))
                                    hint=t(&format!("set-topic-{topic}-hint"))
                                    checked=Signal::derive(move || prefs.profile(&id.get_value()).notify(&read))
                                    on_change=Callback::new(move |on| set(key.clone(), on))
                                />
                            }
                        })
                        .collect_view()}
                </Section>
            </Show>
        </div>
    }
}
