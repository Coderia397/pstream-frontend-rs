//! Article text format. A body is plain text in the .ftl file so translators never touch markup beyond a few tags.
//!   blank line      ends a paragraph or list
//!   ## Heading      section heading
//!   1. text         numbered step (a following unmarked line continues the step)
//!   - text          bullet
//!   !! text         note (the words before the first colon are shown in bold)
//!   >> Title        collapsible section that runs to the next >> or the end
//!   @@ tool         an interactive tool from actions.rs (diagnostics, data, profiles, language, cookies, refresh)
//!   <b>text</b>     bold            <a=/path>text</a>  link (a path, mailto: or https address)
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    P(String),
    H(String),
    Steps(Vec<String>),
    List(Vec<String>),
    Note(String),
    Fold(String, Vec<Block>),
    Action(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(String),
    Link(String, String),
}

enum Acc {
    None,
    Para(String),
    Steps(Vec<String>),
    List(Vec<String>),
}

fn step_text(line: &str) -> Option<&str> {
    let digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    line[digits..].strip_prefix(". ")
}

fn flush(acc: &mut Acc, out: &mut Vec<Block>) {
    match std::mem::replace(acc, Acc::None) {
        Acc::None => {}
        Acc::Para(t) => out.push(Block::P(t)),
        Acc::Steps(v) => out.push(Block::Steps(v)),
        Acc::List(v) => out.push(Block::List(v)),
    }
}

pub fn parse(src: &str) -> Vec<Block> {
    let lines: Vec<&str> = src.lines().map(str::trim).collect();
    let mut out = Vec::new();
    let mut acc = Acc::None;
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        if line.is_empty() {
            flush(&mut acc, &mut out);
        } else if let Some(title) = line.strip_prefix(">>") {
            flush(&mut acc, &mut out);
            let start = i;
            while i < lines.len() && !lines[i].starts_with(">>") {
                i += 1;
            }
            out.push(Block::Fold(title.trim().to_string(), parse(&lines[start..i].join("\n"))));
        } else if let Some(tool) = line.strip_prefix("@@ ") {
            flush(&mut acc, &mut out);
            out.push(Block::Action(tool.trim().to_string()));
        } else if let Some(h) = line.strip_prefix("## ") {
            flush(&mut acc, &mut out);
            out.push(Block::H(h.to_string()));
        } else if let Some(n) = line.strip_prefix("!! ") {
            flush(&mut acc, &mut out);
            out.push(Block::Note(n.to_string()));
        } else if let Some(s) = step_text(line) {
            match &mut acc {
                Acc::Steps(v) => v.push(s.to_string()),
                _ => {
                    flush(&mut acc, &mut out);
                    acc = Acc::Steps(vec![s.to_string()]);
                }
            }
        } else if let Some(s) = line.strip_prefix("- ") {
            match &mut acc {
                Acc::List(v) => v.push(s.to_string()),
                _ => {
                    flush(&mut acc, &mut out);
                    acc = Acc::List(vec![s.to_string()]);
                }
            }
        } else {
            match &mut acc {
                Acc::Para(t) => {
                    t.push(' ');
                    t.push_str(line);
                }
                Acc::Steps(v) | Acc::List(v) => {
                    if let Some(last) = v.last_mut() {
                        last.push(' ');
                        last.push_str(line);
                    }
                }
                Acc::None => acc = Acc::Para(line.to_string()),
            }
        }
    }
    flush(&mut acc, &mut out);
    out
}

pub fn inline(s: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    let mut rest = s;
    loop {
        let b = rest.find("<b>");
        let a = rest.find("<a=");
        let (pos, is_bold) = match (b, a) {
            (Some(b), Some(a)) if b < a => (b, true),
            (Some(b), None) => (b, true),
            (_, Some(a)) => (a, false),
            (None, None) => break,
        };
        if pos > 0 {
            out.push(Inline::Text(rest[..pos].to_string()));
        }
        let after = &rest[pos..];
        if is_bold {
            match after[3..].find("</b>") {
                Some(end) => {
                    out.push(Inline::Bold(after[3..3 + end].to_string()));
                    rest = &after[3 + end + 4..];
                }
                None => {
                    out.push(Inline::Text(after.to_string()));
                    return out;
                }
            }
        } else {
            let parsed = after[3..].find('>').and_then(|close| {
                let url = &after[3..3 + close];
                let body = &after[3 + close + 1..];
                body.find("</a>").map(|end| (url.to_string(), body[..end].to_string(), 3 + close + 1 + end + 4))
            });
            match parsed {
                Some((url, text, used)) => {
                    out.push(Inline::Link(url, text));
                    rest = &after[used..];
                }
                None => {
                    out.push(Inline::Text(after.to_string()));
                    return out;
                }
            }
        }
    }
    if !rest.is_empty() {
        out.push(Inline::Text(rest.to_string()));
    }
    out
}

/// The text of a body or line with every tag and marker removed, for searching.
pub fn plain(src: &str) -> String {
    let mut words = Vec::new();
    for line in src.lines() {
        let line = line.trim();
        if line.starts_with("@@ ") {
            continue;
        }
        let line = line.strip_prefix("## ").or_else(|| line.strip_prefix("!! ")).or_else(|| line.strip_prefix(">>")).or_else(|| line.strip_prefix("- ")).or_else(|| step_text(line)).unwrap_or(line);
        let text: String = inline(line)
            .into_iter()
            .map(|p| match p {
                Inline::Text(t) | Inline::Bold(t) | Inline::Link(_, t) => t,
            })
            .collect();
        words.push(text);
    }
    words.join(" ")
}

fn render_inline(parts: Vec<Inline>) -> Vec<AnyView> {
    parts
        .into_iter()
        .map(|p| match p {
            Inline::Text(t) => view! { {t} }.into_any(),
            Inline::Bold(t) => view! { <b class="font-bold">{t}</b> }.into_any(),
            Inline::Link(url, t) => {
                let external = url.starts_with("http");
                view! {
                    <a
                        href=url
                        target=external.then_some("_blank")
                        rel=external.then_some("noreferrer")
                        class="text-brand-active underline underline-offset-2 hover:text-brand-deep"
                    >
                        {t}
                    </a>
                }
                .into_any()
            }
        })
        .collect()
}

pub fn text(s: &str) -> Vec<AnyView> {
    render_inline(inline(s))
}

fn note_view(n: &str) -> AnyView {
    // "Note: text" shows the label in bold capitals, the way the Netflix articles do
    let split = n.find(':').filter(|&p| p <= 24);
    let (label, rest) = match split {
        Some(p) => (Some(n[..=p].to_uppercase()), &n[p + 1..]),
        None => (None, n),
    };
    view! {
        <div class="mb-5 border-l-4 border-paper-line bg-paper-soft px-4 py-3 text-body-sm leading-6 text-paper-ink">
            {label.map(|l| view! { <b class="mr-1 font-bold">{l}</b> })}
            {render_inline(inline(rest))}
        </div>
    }
    .into_any()
}

/// Headings of the top level, in order, as (anchor id, text), for a table of contents.
pub fn headings(list: &[Block]) -> Vec<(String, String)> {
    list.iter().filter_map(|b| if let Block::H(t) = b { Some(t.clone()) } else { None }).enumerate().map(|(i, t)| (format!("h-{}", i + 1), t)).collect()
}

pub fn blocks(list: Vec<Block>) -> Vec<AnyView> {
    render_blocks(list, true)
}

/// `anchors` gives the top level headings the ids that `headings` lists; headings inside a collapsible section have none.
fn render_blocks(list: Vec<Block>, anchors: bool) -> Vec<AnyView> {
    let mut heading = 0;
    list.into_iter()
        .map(|b| match b {
            Block::P(t) => view! { <p class="mb-5 text-body-md leading-7 text-paper-ink">{render_inline(inline(&t))}</p> }.into_any(),
            Block::H(t) => {
                heading += 1;
                let id = anchors.then(|| format!("h-{heading}"));
                view! { <h2 id=id class="mb-3 mt-8 scroll-mt-6 text-title font-bold text-paper-ink">{t}</h2> }.into_any()
            }
            Block::Steps(v) => view! {
                <ol class="mb-5 list-decimal space-y-3 pl-6 text-body-md leading-7 text-paper-ink marker:font-semibold">
                    {v.into_iter().map(|s| view! { <li>{render_inline(inline(&s))}</li> }).collect_view()}
                </ol>
            }
            .into_any(),
            Block::List(v) => view! {
                <ul class="mb-5 list-disc space-y-2 pl-6 text-body-md leading-7 text-paper-ink">
                    {v.into_iter().map(|s| view! { <li>{render_inline(inline(&s))}</li> }).collect_view()}
                </ul>
            }
            .into_any(),
            Block::Note(t) => note_view(&t),
            Block::Action(id) => super::actions::render(&id),
            Block::Fold(title, inner) => view! {
                <details class="group mb-5 rounded-control border border-paper-line">
                    <summary class="flex cursor-pointer list-none items-center gap-3 px-4 py-3 text-body-sm font-semibold text-paper-ink">
                        <i class="ph-bold ph-caret-right text-paper-mute transition-transform duration-(--motion-base) group-open:rotate-90"></i>
                        {title}
                    </summary>
                    <div class="px-4 pt-2">{render_blocks(inner, false)}</div>
                </details>
            }
            .into_any(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_block_kind() {
        let src = "Intro line\ncontinues.\n\n## Heading\n1. one\ncontinued\n2. two\n\n- a\n- b\n\n!! Note: careful\n\n>> Fold\ninside\n\n## Inner";
        let b = parse(src);
        assert_eq!(b[0], Block::P("Intro line continues.".into()));
        assert_eq!(b[1], Block::H("Heading".into()));
        assert_eq!(b[2], Block::Steps(vec!["one continued".into(), "two".into()]));
        assert_eq!(b[3], Block::List(vec!["a".into(), "b".into()]));
        assert_eq!(b[4], Block::Note("Note: careful".into()));
        assert_eq!(b[5], Block::Fold("Fold".into(), vec![Block::P("inside".into()), Block::H("Inner".into())]));
    }

    #[test]
    fn parses_inline_tags() {
        let v = inline("a <b>bold</b> and <a=/x>link</a> end");
        assert_eq!(v, vec![Inline::Text("a ".into()), Inline::Bold("bold".into()), Inline::Text(" and ".into()), Inline::Link("/x".into(), "link".into()), Inline::Text(" end".into())]);
        assert_eq!(plain("## Head\n1. <b>Go</b> to <a=/p>page</a>"), "Head Go to page");
    }
}
