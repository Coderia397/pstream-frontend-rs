//! What is new for the profile in use, from the engine's real release dates: titles that came out in the last two weeks, titles coming in the
//! next two weeks, and what is trending today. Nothing here is invented: the time shown is the title's own release date.
use crate::services::tmdb::MediaItem;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoticeKind {
    NewRelease,
    ComingSoon,
    Trending,
}

#[derive(Clone, Debug)]
pub struct Notice {
    pub item: MediaItem,
    pub kind: NoticeKind,
    /// Days since the release (0 today); negative for a title that has not come out yet; None when the date is unknown
    pub days_since: Option<i64>,
}

impl Notice {
    /// The headline of the notice, in the reader's language.
    pub fn headline(&self) -> String {
        crate::i18n::t(match self.kind {
            NoticeKind::NewRelease => "notif-h-new-arrival",
            NoticeKind::ComingSoon => "notif-h-coming-soon",
            NoticeKind::Trending => "notif-h-trending",
        })
    }

    /// When: "Today", "3 days ago", "2 weeks ago", "Coming 21 Oct". A trending title has no time of its own and reads "Today".
    pub fn time_label(&self) -> String {
        use fluent_bundle::FluentValue;
        let n = |v: i64| [("n", FluentValue::from(v))];
        match (self.kind, self.days_since) {
            (NoticeKind::ComingSoon, _) => {
                let date = self.item.release_date.clone().or_else(|| self.item.first_air_date.clone()).unwrap_or_default();
                crate::i18n::t_args("media-coming", &[("when", FluentValue::from(crate::i18n::short_date(&date)))])
            }
            (NoticeKind::NewRelease, Some(d)) if d >= 30 => crate::i18n::t_args("notif-ago-months", &n(d / 30)),
            (NoticeKind::NewRelease, Some(d)) if d >= 7 => crate::i18n::t_args("notif-ago-weeks", &n(d / 7)),
            (NoticeKind::NewRelease, Some(d)) if d >= 1 => crate::i18n::t_args("notif-ago-days", &n(d)),
            _ => crate::i18n::t("notif-today"),
        }
    }
}

/// Days from a `YYYY-MM-DD` date to today, positive in the past.
fn days_since(date: &str) -> Option<i64> {
    let when = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&format!("{}T12:00:00", date.get(..10)?))).get_time();
    if when.is_nan() {
        return None;
    }
    Some(((js_sys::Date::now() - when) / 86_400_000.0).floor() as i64)
}

fn date_of(item: &MediaItem) -> Option<&str> {
    item.release_date.as_deref().or(item.first_air_date.as_deref()).filter(|d| d.len() >= 10)
}

/// New releases of the last 14 days (newest first), then titles due within 14 days (soonest first), then up to four trending titles.
/// An engine that cannot be reached gives an empty list.
pub async fn fetch() -> Vec<Notice> {
    let engine = crate::services::ai_engine::fetch_new_popular;
    let (landed, soon, watching) = (engine(Some("just_landed")).await, engine(Some("coming_soon")).await, engine(Some("watching")).await);
    let items = |rows: Option<Vec<crate::services::ai_engine::FeedRow>>| rows.and_then(|r| r.into_iter().next()).map(|r| r.to_media_items()).unwrap_or_default();
    let mut out: Vec<Notice> = Vec::new();
    for item in items(landed) {
        if let Some(d) = date_of(&item).and_then(days_since).filter(|d| (0..=14).contains(d)) {
            out.push(Notice { item, kind: NoticeKind::NewRelease, days_since: Some(d) });
        }
    }
    for item in items(soon) {
        if let Some(d) = date_of(&item).and_then(days_since).filter(|d| (-14..0).contains(d)) {
            out.push(Notice { item, kind: NoticeKind::ComingSoon, days_since: Some(d) });
        }
    }
    let mut trending = 0;
    for item in items(watching) {
        if trending >= 4 {
            break;
        }
        if !out.iter().any(|n| n.item.id == item.id && n.item.media_type == item.media_type) {
            out.push(Notice { item, kind: NoticeKind::Trending, days_since: None });
            trending += 1;
        }
    }
    out.retain(|n| n.item.backdrop_path.is_some() || n.item.poster_path.is_some());
    out.truncate(12);
    out
}
