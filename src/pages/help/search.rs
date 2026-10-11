//! Search over the help articles in the language the page is shown in. A title match counts most, then keywords, then the
//! summary, then the body. Every word has to be found somewhere in the article.
use super::{art, markup};
use crate::data::help::ARTICLES;

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub slug: &'static str,
    pub score: u32,
}

pub fn tokens(q: &str) -> Vec<String> {
    let all: Vec<String> = q.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()).map(str::to_string).collect();
    // one letter pieces such as the t in can't match everything, so they only count when nothing else was typed
    let long: Vec<String> = all.iter().filter(|t| t.chars().count() > 1).cloned().collect();
    if long.is_empty() { all } else { long }
}

pub fn search(q: &str) -> Vec<Hit> {
    let words = tokens(q);
    if words.is_empty() {
        return vec![];
    }
    let phrase = words.join(" ");
    let mut hits: Vec<Hit> = ARTICLES
        .iter()
        .filter_map(|a| {
            let title = art(a.slug, "title").to_lowercase();
            let keywords = art(a.slug, "keywords").to_lowercase();
            let summary = art(a.slug, "summary").to_lowercase();
            let body = markup::plain(&art(a.slug, "body")).to_lowercase();
            let mut score = if title.contains(&phrase) { 40 } else { 0 };
            for w in &words {
                let mut found = 0;
                if title.contains(w.as_str()) {
                    found += 10;
                }
                if keywords.contains(w.as_str()) {
                    found += 8;
                }
                if summary.contains(w.as_str()) {
                    found += 4;
                }
                if body.contains(w.as_str()) {
                    found += 1 + body.matches(w.as_str()).count().min(3) as u32;
                }
                if found == 0 {
                    return None;
                }
                score += found;
            }
            Some(Hit { slug: a.slug, score })
        })
        .collect();
    hits.sort_by(|a, b| b.score.cmp(&a.score));
    hits
}

/// Splits text into pieces and marks the ones that match a search word, ignoring case.
pub fn highlight(text: &str, words: &[String]) -> Vec<(String, bool)> {
    let chars: Vec<char> = text.chars().collect();
    let lower: Vec<char> = chars.iter().map(|c| c.to_lowercase().next().unwrap_or(*c)).collect();
    let mut marked = vec![false; chars.len()];
    for w in words {
        let w: Vec<char> = w.chars().collect();
        if w.is_empty() || w.len() > lower.len() {
            continue;
        }
        for start in 0..=lower.len() - w.len() {
            if lower[start..start + w.len()] == w[..] {
                marked[start..start + w.len()].iter_mut().for_each(|m| *m = true);
            }
        }
    }
    let mut out: Vec<(String, bool)> = Vec::new();
    for (c, m) in chars.into_iter().zip(marked) {
        match out.last_mut() {
            Some((s, flag)) if *flag == m => s.push(c),
            _ => out.push((c.to_string(), m)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_case_insensitively() {
        let v = highlight("Reset your Password now", &["password".to_string()]);
        assert_eq!(v, vec![("Reset your ".to_string(), false), ("Password".to_string(), true), (" now".to_string(), false)]);
    }

    #[test]
    fn tokens_split_on_punctuation() {
        assert_eq!(tokens("  Can't sign-in! "), vec!["can", "sign", "in"]);
    }
}
