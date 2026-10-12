//! One time move of what browsers stored under the product's first name. Every storage key that starts with the old name
//! followed by `_` or `-` is copied to the same key under the current name, then removed, so nobody is signed out and no
//! choice (profiles, language, consent, preferences) is lost. This file keeps the old name on purpose; the rename script skips it.

const OLD: &str = "pstream";
const NEW: &str = "streams";

/// For each stored key with the old prefix: `(old key, new key)`. A key whose new name is already taken is left for removal
/// only (the newer value wins).
pub fn plan(keys: &[String]) -> Vec<(String, String)> {
    keys.iter()
        .filter_map(|k| {
            let rest = k.strip_prefix(OLD)?;
            matches!(rest.chars().next(), Some('_') | Some('-')).then(|| (k.clone(), format!("{NEW}{rest}")))
        })
        .collect()
}

/// Runs the move on local storage. Does nothing when the storage cannot be read.
pub fn migrate() {
    let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) else { return };
    let keys: Vec<String> = (0..storage.length().unwrap_or(0)).filter_map(|i| storage.key(i).ok().flatten()).collect();
    for (old, new) in plan(&keys) {
        if storage.get_item(&new).ok().flatten().is_none() {
            if let Some(value) = storage.get_item(&old).ok().flatten() {
                let _ = storage.set_item(&new, &value);
            }
        }
        let _ = storage.remove_item(&old);
    }
}

#[cfg(test)]
mod tests {
    use super::plan;

    fn keys(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn moves_underscore_and_dash_keys() {
        let moved = plan(&keys(&["pstream_profiles", "pstream-ambient:last", "pstream_preview_muted_p1"]));
        assert_eq!(moved[0], ("pstream_profiles".to_string(), "streams_profiles".to_string()));
        assert_eq!(moved[1].1, "streams-ambient:last");
        assert_eq!(moved[2].1, "streams_preview_muted_p1");
    }

    #[test]
    fn leaves_other_keys_alone() {
        assert!(plan(&keys(&["streams_profiles", "upstream_x", "pstreams", "other", "pstream"])).is_empty());
    }
}
