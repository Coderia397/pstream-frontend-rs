use serde::{Deserialize, Serialize};
use crate::services::tmdb::{fetch_videos, select_best_tmdb_trailer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrailerCandidate {
    pub video_id: String,
    pub title: String,
    pub channel_title: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedTrailer {
    pub video_id: String,
    pub is_teaser: bool,
    pub score: i32,
}

fn normalize_text(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Score a YouTube candidate against title, media type, and quality signals.
/// Returns (score, is_teaser). Higher score = stronger match.
pub fn score_candidate(
    query_title: &str,
    query_year: Option<&str>,
    is_tv: bool,
    is_anime: bool,
    candidate: &TrailerCandidate,
) -> (i32, bool) {
    let q = normalize_text(query_title);
    let t = normalize_text(&candidate.title);
    let c = normalize_text(&candidate.channel_title);

    let mut score = 0i32;
    let mut is_teaser = false;

    // ── 1. Title Relevance ──────────────────────────────────────────────────
    if t.contains(&q) {
        score += 50;
    } else {
        let q_words: Vec<&str> = q.split_whitespace().filter(|w| w.len() > 2).collect();
        let t_words: std::collections::HashSet<&str> = t.split_whitespace().collect();
        let mut overlap = 0;
        for w in &q_words {
            if t_words.contains(w) {
                overlap += 1;
            }
        }
        if !q_words.is_empty() {
            score += ((overlap as f32 / q_words.len() as f32) * 30.0).round() as i32;
        }
    }

    // ── 2. Media Type Guidance ───────────────────────────────────────────────
    if is_tv {
        if t.contains("series") || t.contains("show") || t.contains("tv") {
            score += 20;
        }
        if t.contains("season") {
            score += 25;
        }
        if t.contains("movie") || t.contains("film") {
            score -= 40;
        }
    } else {
        if t.contains("movie") || t.contains("film") {
            score += 20;
        }
        if t.contains("season") || t.contains("episode") || t.contains("series") {
            score -= 40;
        }
    }

    if is_anime {
        if t.contains("anime") {
            score += 50;
        }
        if t.contains("full episode") || t.contains("live action") || t.contains("full movie") {
            score -= 100;
        }
    }

    // ── 3. Quality & Trailer Signals ─────────────────────────────────────────
    if t.contains("teaser") {
        score += 100;
        is_teaser = true;
    }
    if t.contains("trailer") {
        score += 45;
    }
    if t.contains("official") {
        score += 25;
    }
    if t.contains("4k") {
        score += 120;
    }
    if t.contains("hdr") {
        score += 50;
    }
    if t.contains("hd") {
        score += 75;
    }

    // ── 4. Major Studio / Verified Channel Boost ─────────────────────────────
    let studio_tokens = [
        "netflix", "hbo", "max", "disney", "paramount", "sony", "universal",
        "warner", "wb", "mgm", "lionsgate", "apple tv", "peacock", "hulu",
        "prime video", "bbc", "amc", "crunchyroll", "a24",
    ];
    for studio in studio_tokens {
        if t.contains(studio) || c.contains(studio) {
            score += 80;
            break;
        }
    }

    // ── 5. Year Boost ────────────────────────────────────────────────────────
    if let Some(yr_str) = query_year {
        if let Ok(target_year) = yr_str.parse::<i32>() {
            let prev_year = target_year - 1;
            if t.contains(&target_year.to_string()) || t.contains(&prev_year.to_string()) {
                score += 15;
            }
        }
    }

    // ── 6. Penalties & Anti-Junk Filters ──────────────────────────────────────
    let heavy_penalties = [
        "fan made", "concept trailer", "fan trailer", "fake", "ai generated",
        "parody", "spoof", "pitch", "#shorts", "shorts", "tiktok", "reels",
    ];
    for p in heavy_penalties {
        if t.contains(p) {
            score -= 250;
            break;
        }
    }

    let meta_penalties = [
        "reaction", "review", "ending explained", "breakdown", "recap",
        "easter eggs", "theory", "analysis", "behind the scenes", "blooper",
    ];
    for p in meta_penalties {
        if t.contains(p) {
            score -= 200;
            break;
        }
    }

    let media_penalties = [
        "soundtrack", "ost", "music video", "song", "lyrics", "gameplay",
        "walkthrough", "full movie", "full episode",
    ];
    for p in media_penalties {
        if t.contains(p) {
            score -= 120;
            break;
        }
    }

    // Foreign dub/sub keywords (unless title itself contains them)
    let non_english = [
        "hindi", "tamil", "telugu", "malayalam", "kannada", "bhojpuri",
        "dubbed", "dublado", "legendado", "subtitulado", "vostfr", "altyazı",
        "русский", "español", "français", "deutsch", "italiano",
    ];
    for lang in non_english {
        if t.contains(lang) && !q.contains(lang) {
            score -= 150;
            break;
        }
    }

    (score, is_teaser)
}

/// Fetch and resolve the best YouTube trailer for a media item.
/// Returns (video_key, is_teaser).
pub async fn resolve_best_trailer(
    tmdb_id: u32,
    is_tv: bool,
) -> Option<(String, bool)> {
    if tmdb_id == 0 {
        return None;
    }

    // 1. Cooperative Check: Fetch official TMDB videos
    if let Ok(videos) = fetch_videos(tmdb_id, is_tv).await {
        if let Some((key, is_teaser)) = select_best_tmdb_trailer(&videos) {
            return Some((key, is_teaser));
        }
    }

    None
}
