//! Deployment settings. Every address and switch that differs between a laptop, a preview and production is here, and each
//! can be overridden when building, for example:
//!   STREAMS_RESOLVER_URL=https://resolver.example.com trunk build --release
//! so nothing environment specific is written into the pages. Names, contact addresses and logos are in brand.rs.

const fn setting(value: Option<&'static str>, default: &'static str) -> &'static str {
    match value {
        Some(v) => v,
        None => default,
    }
}

/// The public address of the site, used for canonical links and sharing.
pub const SITE_URL: &str = setting(option_env!("STREAMS_SITE_URL"), "https://streams.watch");

/// Stream resolver. It runs on a phone behind a Cloudflare tunnel (see pstream-resolver-rs/deploy).
pub const RESOLVER_URL: &str = setting(option_env!("STREAMS_RESOLVER_URL"), "https://resolver.streams.watch");

/// Recommendation engine. The old variable name still works so existing build scripts keep working.
pub const ENGINE_URL: &str = setting(option_env!("STREAMS_ENGINE_URL"), setting(option_env!("PSTREAM_ENGINE_URL"), "http://127.0.0.1:8088"));

/// Film and series catalog. The key is visible in the browser, as TMDB keys always are, and is free to replace.
pub const TMDB_API_URL: &str = setting(option_env!("STREAMS_TMDB_URL"), "https://api.themoviedb.org/3");
pub const TMDB_IMAGE_URL: &str = setting(option_env!("STREAMS_TMDB_IMAGE_URL"), "https://image.tmdb.org/t/p");
pub const TMDB_API_KEY: &str = setting(option_env!("STREAMS_TMDB_KEY"), "fc5fec3b73d8605daaeb1eb3b91157eb");

/// Accounts (Supabase). The key is the project's publishable key, which is meant to be public; the database rules protect the data.
pub const SUPABASE_URL: &str = setting(option_env!("STREAMS_SUPABASE_URL"), "https://emvyjhleyhimxxdqybsn.supabase.co");
pub const SUPABASE_KEY: &str = setting(option_env!("STREAMS_SUPABASE_KEY"), "sb_publishable_m286TDvNziieAgk--uDLlQ_MJQHgxNV");

/// Account and settings limits. They mirror the database rules in pstream-backend-rs/supabase (five profiles not counting the
/// automatic Kids profile, 4 digit PINs, a password check no older than five minutes), so a change there is made here too.
pub const MAX_PROFILES: usize = 5;
pub const PROFILE_NAME_MAX: usize = 30;
pub const PIN_LENGTH: usize = 4;
pub const MIN_PASSWORD_LENGTH: usize = 8;
/// How long a password check counts for the screens that change account data.
pub const RECENT_SIGN_IN_SECONDS: u64 = 300;
pub const PHONE_DIGITS: std::ops::RangeInclusive<usize> = 6..=15;
pub const BLOCKED_TITLE_MAX: usize = 100;
pub const BLOCKED_TITLES_MAX: usize = 200;
/// Longest first or last name (account_details allows 60).
pub const NAME_MAX: usize = 60;
/// How many security events the Access and devices screen lists.
pub const SECURITY_EVENTS_SHOWN: u32 = 10;

/// Addresses of pages other code links to, so a page is never written as a literal path inside a screen.
pub mod routes {
    pub const HOME: &str = "/";
    pub const BROWSE: &str = "/browse";
    pub const LOGIN: &str = "/login";
    pub const HELP: &str = "/help";
    pub const PRIVACY: &str = "/privacy";
    pub const TERMS: &str = "/terms";
    pub const DMCA: &str = "/dmca";
    pub const DISCLAIMER: &str = "/disclaimer";
}

/// Where the cookie banner sits. Set STREAMS_COOKIE_BANNER to top, bottom, bottom-start, bottom-end or center when building,
/// or add ?cookie_banner=top to the address to try one. Narrow screens always get a full width sheet at the bottom.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BannerPosition {
    Top,
    Bottom,
    BottomStart,
    BottomEnd,
    Center,
}

impl BannerPosition {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "top" => Self::Top,
            "bottom" => Self::Bottom,
            "bottom-start" => Self::BottomStart,
            "bottom-end" => Self::BottomEnd,
            "center" => Self::Center,
            _ => return None,
        })
    }
}

pub fn cookie_banner_position() -> BannerPosition {
    let from_address = web_sys::window().and_then(|w| w.location().search().ok()).and_then(|q| q.trim_start_matches('?').split('&').find_map(|kv| kv.strip_prefix("cookie_banner=").map(str::to_string)));
    from_address.as_deref().or(option_env!("STREAMS_COOKIE_BANNER")).and_then(BannerPosition::parse).unwrap_or(BannerPosition::Bottom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_positions_parse() {
        assert_eq!(BannerPosition::parse("bottom-start"), Some(BannerPosition::BottomStart));
        assert_eq!(BannerPosition::parse("nowhere"), None);
    }
}
