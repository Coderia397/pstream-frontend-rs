//! Search over the help articles and the policy sections in the language the page is shown in. A title match counts most, then
//! keywords, then the summary, then the body. Every word has to be found somewhere in the entry. A word that is a typo away
//! from a word in the entry still counts, with a lower score, and an empty result can offer a corrected query.
use super::{art, legal_text, markup};
use crate::data::help::{ARTICLES, LEGAL};
use std::cell::RefCell;
use std::collections::HashSet;

/// What a result points at: a help article, or one section of a policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Article(&'static str),
    Section(&'static str, u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    Articles,
    Policies,
}

impl Target {
    pub fn href(&self) -> String {
        match self {
            Target::Article(slug) => format!("/help/article/{slug}"),
            Target::Section(doc, n) => format!("{}#s-{n}", crate::data::help::legal_doc(doc).map(|d| d.path).unwrap_or("/help")),
        }
    }

    pub fn is_policy(&self) -> bool {
        matches!(self, Target::Section(..))
    }

    pub fn title(&self) -> String {
        match self {
            Target::Article(slug) => art(slug, "title"),
            Target::Section(doc, n) => format!("{} › {}", legal_text(&format!("legal-{doc}-title")), legal_text(&format!("legal-{doc}-{n}-title"))),
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Target::Article(slug) => art(slug, "summary"),
            Target::Section(doc, n) => {
                let body = legal_text(&format!("legal-{doc}-{n}-body"));
                let mut short: String = body.chars().take(170).collect();
                if body.chars().count() > 170 {
                    short.push('…');
                }
                short
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub target: Target,
    pub score: u32,
}

struct Entry {
    target: Target,
    title: String,
    summary: String,
    keywords: String,
    body: String,
    /// Words of the title, keywords and summary, for typo matching.
    words: HashSet<String>,
}

thread_local! {
    static INDEX: RefCell<Option<Vec<Entry>>> = const { RefCell::new(None) };
}

fn words_of(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric()).filter(|w| w.chars().count() >= 3).map(str::to_string)
}

fn build() -> Vec<Entry> {
    let mut entries = Vec::new();
    for a in ARTICLES {
        let title = art(a.slug, "title").to_lowercase();
        let keywords = art(a.slug, "keywords").to_lowercase();
        let summary = art(a.slug, "summary").to_lowercase();
        let words = words_of(&format!("{title} {keywords} {summary}")).collect();
        entries.push(Entry { target: Target::Article(a.slug), body: markup::plain(&art(a.slug, "body")).to_lowercase(), title, summary, keywords, words });
    }
    for d in LEGAL {
        for n in 1..=d.sections {
            let doc_title = legal_text(&format!("legal-{}-title", d.id)).to_lowercase();
            let section_title = legal_text(&format!("legal-{}-{n}-title", d.id)).to_lowercase();
            let mut body = legal_text(&format!("legal-{}-{n}-body", d.id)).to_lowercase();
            let mut k = 1;
            while crate::i18n::has(&format!("legal-{}-{n}-b{k}", d.id)) {
                body.push(' ');
                body.push_str(&markup::plain(&legal_text(&format!("legal-{}-{n}-b{k}", d.id))).to_lowercase());
                k += 1;
            }
            let title = format!("{doc_title} {section_title}");
            let words = words_of(&title).collect();
            entries.push(Entry { target: Target::Section(d.id, n), summary: String::new(), keywords: doc_title, body, title, words });
        }
    }
    entries
}

fn with_index<R>(f: impl FnOnce(&[Entry]) -> R) -> R {
    INDEX.with(|cell| {
        let mut slot = cell.borrow_mut();
        let index = slot.get_or_insert_with(build);
        f(index)
    })
}

pub fn tokens(q: &str) -> Vec<String> {
    let all: Vec<String> = q.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()).map(str::to_string).collect();
    // one letter pieces such as the t in can't match everything, so they only count when nothing else was typed
    let long: Vec<String> = all.iter().filter(|t| t.chars().count() > 1).cloned().collect();
    if long.is_empty() { all } else { long }
}

/// Number of single character changes (insert, delete, replace, swap of two neighbours) between two words. Gives up early
/// above `limit` and then returns `limit + 1`.
pub fn distance(a: &str, b: &str, limit: usize) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if a.len().abs_diff(b.len()) > limit {
        return limit + 1;
    }
    let mut d = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=b.len() {
        d[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[a.len()][b.len()]
}

/// How many typos a word of this length may contain.
fn tolerance(word: &str) -> usize {
    match word.chars().count() {
        0..=3 => 0,
        4..=7 => 1,
        _ => 2,
    }
}

pub fn search(q: &str, filter: Filter) -> Vec<Hit> {
    let words = tokens(q);
    if words.is_empty() {
        return vec![];
    }
    let phrase = words.join(" ");
    let mut hits: Vec<Hit> = with_index(|index| {
        index
            .iter()
            .filter(|e| match filter {
                Filter::All => true,
                Filter::Articles => !e.target.is_policy(),
                Filter::Policies => e.target.is_policy(),
            })
            .filter_map(|e| {
                let mut score = if e.title.contains(&phrase) { 40 } else { 0 };
                for w in &words {
                    let mut found = 0;
                    if e.title.contains(w.as_str()) {
                        found += 10;
                    }
                    if e.keywords.contains(w.as_str()) {
                        found += 8;
                    }
                    if e.summary.contains(w.as_str()) {
                        found += 4;
                    }
                    if e.body.contains(w.as_str()) {
                        found += 1 + e.body.matches(w.as_str()).count().min(3) as u32;
                    }
                    if found == 0 {
                        let tol = tolerance(w);
                        if tol > 0 && e.words.iter().any(|x| distance(w, x, tol) <= tol) {
                            found = 3;
                        }
                    }
                    if found == 0 {
                        return None;
                    }
                    score += found;
                }
                Some(Hit { target: e.target, score })
            })
            .collect()
    });
    hits.sort_by(|a, b| b.score.cmp(&a.score));
    hits
}

/// A corrected query for when nothing matched: each word is replaced by the closest word known to the index.
pub fn correction(q: &str) -> Option<String> {
    let words = tokens(q);
    if words.is_empty() {
        return None;
    }
    let mut changed = false;
    let fixed: Vec<String> = with_index(|index| {
        words
            .iter()
            .map(|w| {
                let known = index.iter().any(|e| e.title.contains(w.as_str()) || e.keywords.contains(w.as_str()) || e.body.contains(w.as_str()));
                if known {
                    return w.clone();
                }
                let tol = tolerance(w).max(1);
                let best = index.iter().flat_map(|e| e.words.iter()).filter_map(|x| (distance(w, x, tol) <= tol).then(|| (distance(w, x, tol), x.clone()))).min_by_key(|(d, x)| (*d, x.len()));
                match best {
                    Some((_, x)) => {
                        changed = true;
                        x
                    }
                    None => w.clone(),
                }
            })
            .collect()
    });
    changed.then(|| fixed.join(" "))
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

    #[test]
    fn distance_counts_single_edits() {
        assert_eq!(distance("subtitle", "subtitel", 2), 1);
        assert_eq!(distance("profile", "profiel", 2), 1);
        assert_eq!(distance("profile", "profiles", 2), 1);
        assert_eq!(distance("cookie", "cookies", 1), 1);
        assert!(distance("abc", "xyz", 1) > 1);
    }
}
