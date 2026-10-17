//! Keeps what belongs to an account in the account's database and in this browser: the profiles, and for each profile its My List, ratings and viewing progress (Continue Watching).
//!
//! The browser keeps a copy of every profile's data (see `store/scoped.rs` for the keys) so the app works at once and offline. After a sign in, when a profile is chosen and when the tab comes
//! back into focus the changes since the last visit are pulled (`sync_pull`, merged by the time each side changed a row, the newer wins); about four seconds after a local change the changed
//! rows are pushed (`sync_push`, at most 500 rows a call). A row that disappears locally is sent as a removal (My List `in_list = false`, progress `hidden = true`), so other devices learn of it.
//! Titles that arrive without their names and pictures are filled in from TMDB.

use crate::components::media::movie_card_rating::MovieRating;
use leptos::prelude::*;
use crate::models::profile::Profile;
use crate::services::account::{self, AuthError};
use crate::store::watch_store::WatchRecord;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};

/// The stores the sync works on. Timers and requests run outside the page's reactive context, so the stores are handed over once at start (they are Copy handles).
#[derive(Clone, Copy)]
struct Stores {
    profiles: crate::store::ProfileStore,
    library: crate::store::LibraryStore,
    watch: crate::store::watch_store::WatchStore,
}

thread_local! {
    static STORES: std::cell::Cell<Option<Stores>> = const { std::cell::Cell::new(None) };
}

/// Hands the stores to the sync (called once by the app, inside its reactive context).
pub fn init(profiles: crate::store::ProfileStore, library: crate::store::LibraryStore, watch: crate::store::watch_store::WatchStore) {
    STORES.with(|s| s.set(Some(Stores { profiles, library, watch })));
}

fn stores() -> Option<Stores> {
    STORES.with(|s| s.get())
}

const GOOGLE: &str = "https://lh3.googleusercontent.com/d/";
const PUSH_DELAY_MS: u64 = 4000;

pub fn is_uuid(s: &str) -> bool {
    s.len() == 36 && s.matches('-').count() == 4
}

/// The avatar as the database keeps it (at most 64 characters): `default`, or the id of a picture.
pub fn avatar_to_server(url: Option<&str>) -> String {
    match url {
        Some(u) if u.starts_with(GOOGLE) && u.len() - GOOGLE.len() <= 60 => u[GOOGLE.len()..].to_string(),
        _ => "default".to_string(),
    }
}

pub fn avatar_from_server(avatar: &str) -> Option<String> {
    if avatar.is_empty() || avatar == "default" { Some(crate::brand::DEFAULT_AVATAR.to_string()) } else { Some(format!("{GOOGLE}{avatar}")) }
}

fn iso(ms: u64) -> String {
    js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(ms as f64)).to_iso_string().as_string().unwrap_or_default()
}

fn from_iso(s: &str) -> u64 {
    let t = js_sys::Date::parse(s);
    if t.is_nan() { 0 } else { t as u64 }
}

fn now() -> u64 {
    js_sys::Date::now() as u64
}

// ── profiles ─────────────────────────────────────────────────────────────────────────────────

/// The account's profiles, main first, in the order the person set.
pub async fn load_profiles() -> Option<Vec<Profile>> {
    let rows = account::rest("GET", "/rest/v1/profiles?select=id,name,avatar,kind,maturity,has_pin,is_main,sort_order&order=sort_order.asc,created_at.asc", None, "").await.ok()?;
    let list: Vec<Profile> = rows
        .as_array()?
        .iter()
        .map(|r| Profile {
            id: r["id"].as_str().unwrap_or_default().to_string(),
            name: r["name"].as_str().unwrap_or_default().to_string(),
            avatar_url: avatar_from_server(r["avatar"].as_str().unwrap_or("default")),
            is_kids: r["kind"].as_str() == Some("kids"),
            is_default: Some(r["is_main"].as_bool().unwrap_or(false)),
            pin: r["has_pin"].as_bool().filter(|h| *h).map(|_| String::new()),
            sort_order: r["sort_order"].as_i64().unwrap_or(0) as i32,
        })
        .collect();
    (!list.is_empty()).then_some(list)
}

/// Keeps the profiles in this browser (the copy the app starts from).
fn save_profiles_locally(store: crate::store::ProfileStore) {
    if let (Some(storage), Ok(json)) = (web_sys::window().and_then(|w| w.local_storage().ok().flatten()), serde_json::to_string(&store.profiles.get_untracked())) {
        let _ = storage.set_item(crate::keys::PROFILES, &json);
    }
}

/// Makes the profiles of the app the profiles of the account: read after a sign in and at start.
pub fn refresh_profiles(store: crate::store::ProfileStore) {
    leptos::task::spawn_local(async move {
        if let Some(list) = load_profiles().await {
            store.profiles.set(list);
            save_profiles_locally(store);
            // a chosen profile that no longer exists is forgotten
            if let Some(active) = store.active_profile_id.get_untracked() {
                if !store.profiles.with_untracked(|l| l.iter().any(|p| p.id == active)) {
                    store.active_profile_id.set(None);
                }
            }
        }
    });
}

/// Sends the profiles of the app to the account: a profile with an id of the account is updated, one made in the app is created (and takes the id the account gives it).
pub fn push_profiles(store: crate::store::ProfileStore) {
    leptos::task::spawn_local(async move {
        let Some(session) = account::ensure_fresh().await else { return };
        let local = store.profiles.get_untracked();
        // the profiles of the app are the account's once they have been read (they carry account ids); before that nothing is sent, so a stale local list can never replace the account's
        if !local.iter().any(|p| is_uuid(&p.id)) { return; }
        let Some(remote) = load_profiles().await else { return };
        for p in &local {
            let kind = if p.is_kids { "kids" } else { "adult" };
            let maturity = if p.is_kids { "older_kids" } else { "adult" };
            if is_uuid(&p.id) {
                let _ = account::rest("PATCH", &format!("/rest/v1/profiles?id=eq.{}", p.id), Some(&json!({ "name": p.name, "avatar": avatar_to_server(p.avatar_url.as_deref()), "sort_order": p.sort_order })), "").await;
            } else {
                let body = json!({ "user_id": session.user_id, "name": p.name, "avatar": avatar_to_server(p.avatar_url.as_deref()), "kind": kind, "maturity": maturity, "sort_order": p.sort_order });
                if let Ok(rows) = account::rest("POST", "/rest/v1/profiles", Some(&body), "return=representation").await {
                    if let Some(new_id) = rows.get(0).and_then(|r| r["id"].as_str()) {
                        let old = p.id.clone();
                        let new_id = new_id.to_string();
                        store.profiles.update(|l| { if let Some(x) = l.iter_mut().find(|x| x.id == old) { x.id = new_id.clone(); } });
                        if store.active_profile_id.get_untracked().as_deref() == Some(old.as_str()) { store.active_profile_id.set(Some(new_id)); }
                    }
                }
            }
        }
        // a profile that was deleted in the app is deleted in the account (the main profile and the last one are protected by the database)
        for r in &remote {
            if !local.iter().any(|p| p.id == r.id) {
                let _ = account::rest("DELETE", &format!("/rest/v1/profiles?id=eq.{}", r.id), None, "").await;
            }
        }
        save_profiles_locally(store);
    });
}

// ── what a profile holds ─────────────────────────────────────────────────────────────────────

fn rating_word_to_server(w: &str) -> Option<&'static str> {
    match w { "like" => Some("up"), "love" => Some("love"), "dislike" => Some("down"), _ => None }
}

fn rating_from_server(w: &str) -> Option<MovieRating> {
    match w { "up" => Some(MovieRating::Like), "love" => Some(MovieRating::Love), "down" => Some(MovieRating::Dislike), _ => None }
}

/// The rows a profile holds now, as the database keeps them: (titles, progress), each keyed so that two states can be compared.
fn current_rows() -> (BTreeMap<String, Value>, BTreeMap<String, Value>) {
    let Some(st) = stores() else { return (BTreeMap::new(), BTreeMap::new()) };
    let (library, watch) = (st.library, st.watch);
    let ratings = crate::store::ratings::all();
    let mut titles: BTreeMap<String, Value> = BTreeMap::new();
    library.my_list.with_untracked(|m| {
        for e in m.values() {
            let kind = if e.media.media_type.as_deref() == Some("tv") || (e.media.media_type.is_none() && e.media.name.is_some() && e.media.title.is_none()) { "tv" } else { "movie" };
            let key = format!("{kind}:{}", e.media.id);
            titles.insert(key, json!({ "media_type": kind, "tmdb_id": e.media.id, "in_list": true, "added_at": iso(e.added_at), "client_updated_at": iso(e.added_at) }));
        }
    });
    for (key, (word, ts)) in ratings {
        let Some((kind, id)) = key.split_once(':') else { continue };
        let Ok(id) = id.parse::<u32>() else { continue };
        let Some(server) = rating_word_to_server(&word) else { continue };
        let row = titles.entry(key.clone()).or_insert_with(|| json!({ "media_type": kind, "tmdb_id": id, "in_list": false }));
        row["rating"] = json!(server);
        let ts = ts.max(row["client_updated_at"].as_str().map(from_iso).unwrap_or(0));
        row["client_updated_at"] = json!(iso(ts));
    }
    let mut progress: BTreeMap<String, Value> = BTreeMap::new();
    watch.records.with_untracked(|m| {
        for r in m.values() {
            progress.insert(r.key(), json!({
                "media_type": if r.is_tv { "tv" } else { "movie" }, "tmdb_id": r.media_id, "season": r.season.unwrap_or(0), "episode": r.episode.unwrap_or(0),
                "position_seconds": r.current_time as i64, "duration_seconds": r.duration as i64, "completed": r.percentage >= 95.0, "hidden": false, "client_updated_at": iso(r.updated_at),
            }));
        }
    });
    (titles, progress)
}

fn snapshot_key(profile: &str) -> String { format!("streams_sync_snapshot:{profile}") }
fn cursor_key(profile: &str) -> String { format!("streams_sync_cursor:{profile}") }

fn read_storage(key: &str) -> Option<String> {
    web_sys::window()?.local_storage().ok().flatten()?.get_item(key).ok().flatten()
}

fn write_storage(key: &str, value: &str) {
    if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) { let _ = s.set_item(key, value); }
}

/// What the account was last told, per row (the row without its time), so only changes are sent.
fn read_snapshot(profile: &str) -> HashMap<String, String> {
    read_storage(&snapshot_key(profile)).and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or_default()
}

fn signature(row: &Value) -> String {
    let mut r = row.clone();
    if let Some(o) = r.as_object_mut() { o.remove("client_updated_at"); }
    r.to_string()
}

fn active_account_profile() -> Option<String> {
    let store = stores()?.profiles;
    store.active_profile_id.get_untracked().filter(|id| is_uuid(id)).filter(|_| account::has_account())
}

thread_local! {
    static PUSH_TIMER: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

/// Asks for the changes to be sent in a few seconds (every call moves the time on, so a burst of changes is one call).
pub fn schedule_push() {
    if active_account_profile().is_none() { return; }
    let Some(win) = web_sys::window() else { return };
    PUSH_TIMER.with(|t| {
        if let Some(h) = t.get() { win.clear_timeout_with_handle(h); }
        let cb = wasm_bindgen::closure::Closure::once_into_js(move || { leptos::task::spawn_local(async { let _ = push_now().await; }); });
        t.set(win.set_timeout_with_callback_and_timeout_and_arguments_0(wasm_bindgen::JsCast::unchecked_ref(&cb), PUSH_DELAY_MS as i32).ok());
    });
}

/// Sends what changed since the account was last told.
pub async fn push_now() -> Result<(), AuthError> {
    let Some(profile) = active_account_profile() else { return Ok(()) };
    let (titles, progress) = current_rows();
    let snap = read_snapshot(&profile);
    let mut out_titles: Vec<Value> = Vec::new();
    let mut out_progress: Vec<Value> = Vec::new();
    let mut next: HashMap<String, String> = HashMap::new();
    for (k, row) in &titles {
        let sig = signature(row);
        if snap.get(&format!("t|{k}")) != Some(&sig) { out_titles.push(row.clone()); }
        next.insert(format!("t|{k}"), sig);
    }
    for (k, row) in &progress {
        let sig = signature(row);
        if snap.get(&format!("p|{k}")) != Some(&sig) { out_progress.push(row.clone()); }
        next.insert(format!("p|{k}"), sig);
    }
    // rows that were known and are gone: removals
    for k in snap.keys() {
        if next.contains_key(k) { continue; }
        let (kind, rest) = k.split_at(2);
        let Some((media, id)) = rest.split_once(':') else { continue };
        let Ok(id) = id.parse::<u32>() else { continue };
        let media = media.trim_start_matches('_');
        if kind == "t|" {
            out_titles.push(json!({ "media_type": media, "tmdb_id": id, "in_list": false, "rating": Value::Null, "client_updated_at": iso(now()) }));
        } else {
            out_progress.push(json!({ "media_type": media, "tmdb_id": id, "hidden": true, "client_updated_at": iso(now()) }));
        }
    }
    if out_titles.is_empty() && out_progress.is_empty() { return Ok(()); }
    for (t, p) in out_titles.chunks(400).map(|c| (c.to_vec(), Vec::new())).chain(out_progress.chunks(400).map(|c| (Vec::new(), c.to_vec()))) {
        account::rpc("sync_push", &json!({ "p_profile": profile, "p_titles": t, "p_progress": p })).await?;
    }
    write_storage(&snapshot_key(&profile), &serde_json::to_string(&next).unwrap_or_default());
    Ok(())
}

/// Reads what changed in the account since the last visit and merges it into this browser: for each row the newer side wins.
pub async fn pull_now() -> Result<(), AuthError> {
    let Some(profile) = active_account_profile() else { return Ok(()) };
    let mut cursor = read_storage(&cursor_key(&profile)).unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
    let Some(st) = stores() else { return Ok(()) };
    let (library, watch) = (st.library, st.watch);
    loop {
        let answer = account::rpc("sync_pull", &json!({ "p_profile": profile, "p_since": cursor })).await?;
        for t in answer["titles"].as_array().cloned().unwrap_or_default() {
            let (Some(kind), Some(id)) = (t["media_type"].as_str(), t["tmdb_id"].as_u64()) else { continue };
            let id = id as u32;
            let at = t["client_updated_at"].as_str().map(from_iso).unwrap_or(0);
            let is_tv = kind == "tv";
            // My List
            let in_list = t["in_list"].as_bool().unwrap_or(false);
            let local_added = library.my_list.with_untracked(|m| m.get(&id).map(|e| e.added_at));
            if in_list && local_added.is_none_or(|l| at > l) {
                let added = t["added_at"].as_str().map(from_iso).filter(|a| *a > 0).unwrap_or(at);
                library.my_list.update(|m| {
                    m.insert(id, crate::store::LibraryEntry { media: crate::services::tmdb::MediaItem { id, media_type: Some(kind.to_string()), ..Default::default() }, added_at: added });
                });
                hydrate_list_entry(id, is_tv);
            } else if !in_list && local_added.is_some_and(|l| at > l) {
                library.my_list.update(|m| { m.remove(&id); });
            }
            // rating
            let key = format!("{kind}:{id}");
            let local_ts = crate::store::ratings::stamp(&key);
            if at > local_ts {
                crate::store::ratings::apply_remote(id, is_tv, t["rating"].as_str().and_then(rating_from_server), at);
            }
        }
        for p in answer["progress"].as_array().cloned().unwrap_or_default() {
            let (Some(kind), Some(id)) = (p["media_type"].as_str(), p["tmdb_id"].as_u64()) else { continue };
            let id = id as u32;
            let is_tv = kind == "tv";
            let at = p["client_updated_at"].as_str().map(from_iso).unwrap_or(0);
            let local = watch.get_record(id, is_tv);
            if local.as_ref().is_some_and(|l| l.updated_at >= at) { continue; }
            if p["hidden"].as_bool() == Some(true) {
                watch.remove_record_quiet(id, is_tv);
                continue;
            }
            let duration = p["duration_seconds"].as_f64().unwrap_or(0.0);
            let position = p["position_seconds"].as_f64().unwrap_or(0.0);
            if duration <= 0.0 { continue; }
            let (season, episode) = (p["season"].as_u64().filter(|s| *s > 0).map(|s| s as u32), p["episode"].as_u64().filter(|e| *e > 0).map(|e| e as u32));
            watch.insert_remote(WatchRecord {
                media_id: id, is_tv, title: local.as_ref().map(|l| l.title.clone()).unwrap_or_default(), poster_path: local.as_ref().and_then(|l| l.poster_path.clone()), backdrop_path: local.as_ref().and_then(|l| l.backdrop_path.clone()),
                season, episode, episode_title: None, current_time: position, duration, percentage: ((position / duration) * 100.0).clamp(0.0, 100.0) as f32, updated_at: at,
            });
            hydrate_watch_record(id, is_tv);
        }
        cursor = answer["cursor"].as_str().unwrap_or(&cursor).to_string();
        write_storage(&cursor_key(&profile), &cursor);
        if answer["has_more"].as_bool() != Some(true) { break; }
    }
    // what the account holds is now what this browser holds: the next push sends only newer changes
    let (titles, progress) = current_rows();
    let mut snap: HashMap<String, String> = HashMap::new();
    for (k, row) in &titles { snap.insert(format!("t|{k}"), signature(row)); }
    for (k, row) in &progress { snap.insert(format!("p|{k}"), signature(row)); }
    write_storage(&snapshot_key(&profile), &serde_json::to_string(&snap).unwrap_or_default());
    // anything this browser changed while it was away goes up now
    schedule_push();
    Ok(())
}

/// Fills in the name and pictures of a My List title that came from the account without them.
fn hydrate_list_entry(id: u32, is_tv: bool) {
    leptos::task::spawn_local(async move {
        let Ok(d) = crate::services::tmdb::fetch_details(id, is_tv).await else { return };
        let Some(st) = stores() else { return };
        st.library.my_list.update(|m| {
            if let Some(e) = m.get_mut(&id) {
                e.media.title = d.title.clone();
                e.media.name = d.name.clone();
                e.media.overview = d.overview.clone();
                e.media.poster_path = d.poster_path.clone();
                e.media.backdrop_path = d.backdrop_path.clone();
                e.media.vote_average = d.vote_average;
                e.media.release_date = d.release_date.clone();
                e.media.first_air_date = d.first_air_date.clone();
            }
        });
    });
}

/// Fills in the name and pictures of a viewing record that came from the account without them.
fn hydrate_watch_record(id: u32, is_tv: bool) {
    leptos::task::spawn_local(async move {
        let Some(st) = stores() else { return };
        if st.watch.get_record(id, is_tv).is_some_and(|r| !r.title.is_empty()) { return; }
        let Ok(d) = crate::services::tmdb::fetch_details(id, is_tv).await else { return };
        let title = d.display_title().to_string();
        st.watch.fill_details(id, is_tv, title, d.poster_path.clone(), d.backdrop_path.clone());
    });
}

/// Called when a profile becomes the one in use, after a sign in and when the tab comes back: reads what changed in the account.
pub fn sync_in_background() {
    leptos::task::spawn_local(async { let _ = pull_now().await; });
}
