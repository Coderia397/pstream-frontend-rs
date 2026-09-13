use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const WATCH_STORAGE_KEY: &str = "pstream_watch_progress_v1";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WatchRecord {
    pub media_id: u32,
    pub is_tv: bool,
    pub title: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub season: Option<u32>,
    pub episode: Option<u32>,
    pub episode_title: Option<String>,
    pub current_time: f64,
    pub duration: f64,
    pub percentage: f32,
    pub updated_at: u64,
}

impl WatchRecord {
    pub fn composite_key(media_id: u32, is_tv: bool) -> String {
        format!("{}_{}", if is_tv { "tv" } else { "movie" }, media_id)
    }

    pub fn key(&self) -> String {
        Self::composite_key(self.media_id, self.is_tv)
    }

    pub fn backdrop_url(&self, size: &str) -> Option<String> {
        self.backdrop_path.as_ref().map(|p| {
            if p.starts_with("http") {
                p.clone()
            } else {
                format!("https://image.tmdb.org/t/p/{}{}", size, p)
            }
        })
    }

    pub fn poster_url(&self, size: &str) -> Option<String> {
        self.poster_path.as_ref().map(|p| {
            if p.starts_with("http") {
                p.clone()
            } else {
                format!("https://image.tmdb.org/t/p/{}{}", size, p)
            }
        })
    }
}

#[derive(Clone, Copy)]
pub struct WatchStore {
    pub records: RwSignal<HashMap<String, WatchRecord>>,
}

fn load_from_storage() -> HashMap<String, WatchRecord> {
    let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
    storage
        .as_ref()
        .and_then(|s| s.get_item(WATCH_STORAGE_KEY).ok().flatten())
        .and_then(|raw| serde_json::from_str::<HashMap<String, WatchRecord>>(&raw).ok())
        .unwrap_or_default()
}

fn save_to_storage(records: &HashMap<String, WatchRecord>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(raw) = serde_json::to_string(records) {
            let _ = storage.set_item(WATCH_STORAGE_KEY, &raw);
        }
    }
}

pub fn provide_watch_store() {
    let initial_records = load_from_storage();
    let records_signal = RwSignal::new(initial_records);

    provide_context(WatchStore {
        records: records_signal,
    });
}

pub fn use_watch_store() -> WatchStore {
    use_context::<WatchStore>().expect("WatchStore not provided in context")
}

impl WatchStore {
    pub fn get_record(&self, media_id: u32, is_tv: bool) -> Option<WatchRecord> {
        let key = WatchRecord::composite_key(media_id, is_tv);
        self.records.with(|m| m.get(&key).cloned())
    }

    pub fn get_resume_time(&self, media_id: u32, is_tv: bool, season: Option<u32>, episode: Option<u32>) -> f64 {
        if let Some(record) = self.get_record(media_id, is_tv) {
            if is_tv {
                if record.season == season && record.episode == episode {
                    // If within last 30s of completion, don't resume right at the credits
                    if record.duration > 60.0 && record.current_time >= record.duration - 30.0 {
                        0.0
                    } else {
                        record.current_time
                    }
                } else {
                    0.0
                }
            } else {
                if record.duration > 60.0 && record.current_time >= record.duration - 30.0 {
                    0.0
                } else {
                    record.current_time
                }
            }
        } else {
            0.0
        }
    }

    pub fn save_progress(
        &self,
        media_id: u32,
        is_tv: bool,
        title: String,
        poster_path: Option<String>,
        backdrop_path: Option<String>,
        season: Option<u32>,
        episode: Option<u32>,
        episode_title: Option<String>,
        current_time: f64,
        duration: f64,
    ) {
        if media_id == 0 || duration <= 0.0 {
            return;
        }

        let percentage = ((current_time / duration) * 100.0).clamp(0.0, 100.0) as f32;
        let now = js_sys::Date::now() as u64;

        let record = WatchRecord {
            media_id,
            is_tv,
            title,
            poster_path,
            backdrop_path,
            season,
            episode,
            episode_title,
            current_time,
            duration,
            percentage,
            updated_at: now,
        };

        let key = record.key();
        self.records.update(|map| {
            map.insert(key, record);
            save_to_storage(map);
        });
    }

    pub fn remove_record(&self, media_id: u32, is_tv: bool) {
        let key = WatchRecord::composite_key(media_id, is_tv);
        self.records.update(|map| {
            map.remove(&key);
            save_to_storage(map);
        });
    }

    pub fn get_continue_watching_list(&self) -> Vec<WatchRecord> {
        self.records.with(|map| {
            let mut list: Vec<WatchRecord> = map
                .values()
                .filter(|r| {
                    // Show items that have been watched more than 15s and less than 95%
                    r.current_time > 15.0 && r.percentage < 95.0
                })
                .cloned()
                .collect();
            // Sort newest updated first
            list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
            list
        })
    }
}
