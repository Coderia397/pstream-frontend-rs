//! A second guard for Kids profiles, in the app. The engine decides first (a known rating within the tier and a title made for children); this keeps lists that
//! come from TMDB or from an engine that could not be reached from showing a murder mystery or a reality show because it carries a PG label.
use crate::services::tmdb::MediaItem;

/// TMDB genre ids a Kids profile may see: Animation, Family, Kids.
const KID_GENRES: [u32; 3] = [16, 10751, 10762];
/// Genre ids never shown to a Kids profile: Crime, Horror, Mystery, Thriller, War, News, Reality, Talk, War & Politics.
const BLOCKED_GENRES: [u32; 9] = [80, 27, 9648, 53, 10752, 10763, 10764, 10767, 10768];

/// Is the title made for children? Needs a children's genre and none of the blocked ones; a title with no genres is not shown.
pub fn kid_friendly(item: &MediaItem) -> bool {
    if item.adult == Some(true) {
        return false;
    }
    item.genre_ids.iter().any(|g| KID_GENRES.contains(g)) && !item.genre_ids.iter().any(|g| BLOCKED_GENRES.contains(g))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(genres: &[u32]) -> MediaItem {
        MediaItem { genre_ids: genres.to_vec(), ..Default::default() }
    }

    #[test]
    fn children_titles_pass_and_mysteries_do_not() {
        assert!(kid_friendly(&with(&[16, 35])));
        assert!(kid_friendly(&with(&[10751])));
        assert!(!kid_friendly(&with(&[9648, 18, 80])));
        assert!(!kid_friendly(&with(&[10751, 10764])), "a family reality show");
        assert!(!kid_friendly(&with(&[])));
    }
}
