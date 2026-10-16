//! The logo of a collection, made to look hand made: lettering on a sticker or a paper cut out, as the logos of the app's collections are. The title is set in a hand lettered face
//! and given a finish (a die cut sticker border, a paper cut out, marker on a highlighter swipe, a rough ink stamp, a double outline), a little tilt, and a second colour for the last word.
//! Ten lettering treatments x five finishes make 50 designs; a collection always has the same one (it follows its id) and its colours come from its hue. The gallery of all 50 is at `/dev/logos`.

use leptos::prelude::*;

pub const TREATMENTS: usize = 10;
pub const FINISHES: usize = 5;
pub const DESIGNS: usize = TREATMENTS * FINISHES;

/// The design number (0 to 49) of a collection: a hash of its id, so it never changes between visits.
pub fn design_of(id: &str) -> usize {
    id.bytes().fold(7usize, |h, b| h.wrapping_mul(131).wrapping_add(b as usize)) % DESIGNS
}

/// A lettering treatment: the face, the case, the tilt, the slant and the size of the last word compared with the rest.
struct Treatment {
    family: &'static str,
    uppercase: bool,
    rotate: f32,
    skew: f32,
    tracking: &'static str,
    /// font size factor (hand lettered faces differ a lot in how big they look)
    size: f32,
    /// the words go on two lines, the last one larger
    stack: bool,
}

const TREATMENTS_TABLE: [Treatment; TREATMENTS] = [
    // 0 marker capitals
    Treatment { family: "'Logo Marker', cursive", uppercase: true, rotate: -4.0, skew: -6.0, tracking: "0.01em", size: 0.92, stack: true },
    // 1 brush
    Treatment { family: "'Logo Brush', cursive", uppercase: false, rotate: -5.0, skew: 0.0, tracking: "0", size: 1.2, stack: true },
    // 2 scrawl
    Treatment { family: "'Logo Scrawl', cursive", uppercase: true, rotate: -3.0, skew: 0.0, tracking: "0.02em", size: 0.62, stack: true },
    // 3 hand cut block
    Treatment { family: "'Logo Cutout', var(--font-brand)", uppercase: true, rotate: -3.0, skew: -4.0, tracking: "0.01em", size: 1.08, stack: true },
    // 4 graffiti
    Treatment { family: "'Logo Graffiti', cursive", uppercase: false, rotate: -4.0, skew: 0.0, tracking: "0", size: 1.15, stack: true },
    // 5 editorial italic serif
    Treatment { family: "'Logo Editorial', ui-serif, Georgia, serif", uppercase: false, rotate: -2.0, skew: 0.0, tracking: "-0.01em", size: 1.15, stack: false },
    // 6 casual hand
    Treatment { family: "'Logo Casual', cursive", uppercase: false, rotate: -3.0, skew: 0.0, tracking: "0.01em", size: 1.15, stack: true },
    // 7 handwriting
    Treatment { family: "'Logo Hand', cursive", uppercase: false, rotate: -4.0, skew: 0.0, tracking: "0", size: 1.0, stack: true },
    // 8 black italic serif
    Treatment { family: "'Logo Playfair', ui-serif, Georgia, serif", uppercase: false, rotate: -2.0, skew: 0.0, tracking: "-0.02em", size: 0.98, stack: false },
    // 9 cut paper headline
    Treatment { family: "'Logo Abril', ui-serif, Georgia, serif", uppercase: true, rotate: -2.0, skew: 0.0, tracking: "0.01em", size: 0.9, stack: true },
];

/// `n` text shadows laid round a circle of radius `r`: a thick rounded border, as the cut edge of a sticker.
fn ring(n: usize, r: f32, color: &str) -> String {
    (0..n)
        .map(|i| {
            let a = std::f32::consts::TAU * i as f32 / n as f32;
            format!("{:.2}px {:.2}px 0 {}", r * a.cos(), r * a.sin(), color)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The finish, from the hue `h`. Returns the CSS for the letters and the CSS of the highlighter swipe behind them (empty unless the finish has one).
fn finish(f: usize, h: u32) -> (String, String) {
    let accent = (h + 190) % 360;
    match f {
        // die cut sticker: dark letters, a cream border, a coloured edge outside it, a soft shadow
        0 => (
            format!(
                "color: hsl({h} 35% 10%); text-shadow: {}, {}, 0 6px 14px hsl({h} 50% 4% / 0.55);",
                ring(20, 5.0, "hsl(45 70% 96%)"),
                ring(20, 8.0, &format!("hsl({} 70% 72%)", (h + 330) % 360))
            ),
            String::new(),
        ),
        // paper cut out: cream letters, a dark hairline, a hard offset in the colour of the collection
        1 => (
            format!("color: hsl(45 60% 94%); text-shadow: {}, 4px 5px 0 hsl({h} 60% 42%), 7px 9px 12px hsl({h} 50% 4% / 0.5);", ring(14, 2.2, &format!("hsl({h} 40% 12%)"))),
            String::new(),
        ),
        // marker on a highlighter swipe
        2 => (
            "color: hsl(0 0% 6%); text-shadow: 0 0 1px hsl(0 0% 6%);".to_string(),
            format!("linear-gradient(transparent 52%, hsl({h} 90% 66% / 0.92) 52%, hsl({h} 90% 66% / 0.92) 90%, transparent 90%)"),
        ),
        // ink stamp: the colour of the collection, roughened edges, a little ghost
        3 => (
            format!("color: hsl({h} 70% 62%); text-shadow: 2px 2px 0 hsl({h} 60% 30% / 0.55), -1px -1px 0 hsl(45 60% 96% / 0.35); filter: url(#logo-rough);"),
            String::new(),
        ),
        // double outline: cream letters, a coloured border, a cream border outside it
        _ => (
            format!(
                "color: hsl(45 70% 96%); text-shadow: {}, {}, 3px 6px 12px hsl({accent} 40% 6% / 0.5);",
                ring(18, 3.6, &format!("hsl({accent} 75% 52%)")),
                ring(22, 7.0, "hsl(45 70% 96%)")
            ),
            String::new(),
        ),
    }
}

/// The CSS of a design (0 to 49) for a word mark of the hue `h`.
pub fn design_style(design: usize, h: u32) -> String {
    let t = &TREATMENTS_TABLE[(design % DESIGNS) / FINISHES];
    let (letters, swipe) = finish((design % DESIGNS) % FINISHES, h);
    let swipe_css = if swipe.is_empty() { String::new() } else { format!(" background-image: {swipe}; padding: 0 0.12em;") };
    format!(
        "font-family: {}; font-weight: 400; letter-spacing: {}; line-height: 1.02; text-transform: {}; transform: rotate({}deg) skewX({}deg); transform-origin: center; text-wrap: balance; {} {}",
        t.family, t.tracking, if t.uppercase { "uppercase" } else { "none" }, t.rotate, t.skew, letters, swipe_css
    )
}

/// The font size factor of a design (hand lettered faces differ in how large they look).
pub fn design_size(design: usize) -> f32 {
    TREATMENTS_TABLE[(design % DESIGNS) / FINISHES].size
}

/// The title as a word mark. `design` is the number of the look (from `design_of` of the collection's id); `size` is the base font size in px.
#[component]
pub fn CollectionLogo(
    #[prop(into)] text: String,
    #[prop(into)] hue: u32,
    #[prop(default = 0)] design: usize,
    #[prop(default = 44.0)] size: f32,
) -> impl IntoView {
    let t = &TREATMENTS_TABLE[(design % DESIGNS) / FINISHES];
    let finish_index = (design % DESIGNS) % FINISHES;
    let style = format!("{} font-size: {:.1}px;", design_style(design, hue), size * design_size(design));
    // the last word in a second colour (not on the finishes that already use two)
    let accent = format!("hsl({} 80% 60%)", (hue + 190) % 360);
    let mut words = text.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    let body = if words.len() > 1 && finish_index != 4 {
        let last = words.pop().unwrap_or_default();
        let first = words.join(" ");
        let last_style = if finish_index == 0 || finish_index == 3 { format!("color: {accent};") } else { String::new() };
        if t.stack {
            view! {
                <span class="block">{first}</span>
                <span class="block" style=format!("font-size: 1.22em; {last_style}")>{last}</span>
            }
            .into_any()
        } else {
            view! { <span>{first}" "</span><span style=last_style>{last}</span> }.into_any()
        }
    } else {
        view! { <span>{text.clone()}</span> }.into_any()
    };
    view! {
        <span class="inline-block max-w-full text-center" style=style>{body}</span>
    }
}

/// An invisible filter that roughens the edges of letters a little, like ink on paper (used by the stamp finish).
#[component]
pub fn LogoFilters() -> impl IntoView {
    view! {
        <svg width="0" height="0" style="position:absolute" aria-hidden="true">
            <filter id="logo-rough" x="-5%" y="-5%" width="110%" height="110%">
                <feTurbulence type="fractalNoise" baseFrequency="0.04" numOctaves="3" seed="7" result="n"></feTurbulence>
                <feDisplacementMap in="SourceGraphic" in2="n" scale="3.5"></feDisplacementMap>
            </filter>
        </svg>
    }
}

const GALLERY_TITLE: &str = concat!("Collection", " logos"); // developer page: not translated
const SAMPLE_TEXT: &str = concat!("Love Ya", " Forever");

/// The gallery of the 50 designs, at `/dev/logos`: each with its number, so one can be chosen for a collection (`design_of` picks by hash until then).
#[component]
pub fn LogoGalleryPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-page px-8 py-10 text-ink">
            <LogoFilters />
            <h1 class="mb-8 text-3xl font-bold">{GALLERY_TITLE}</h1>
            <div class="grid grid-cols-1 gap-6 md:grid-cols-2 xl:grid-cols-3">
                {(0..DESIGNS).map(|d| {
                    let hue = (d * 47) as u32 % 360;
                    view! {
                        <div class="relative overflow-hidden rounded-card border border-ink/10 p-6" style=format!("background: radial-gradient(circle at 30% 20%, hsl({hue} 38% 20%), hsl({hue} 45% 9%));")>
                            <span class="absolute right-3 top-2 text-xs text-ink/50">{format!("#{d}")}</span>
                            <div class="flex min-h-[170px] items-center justify-center">
                                <CollectionLogo text=SAMPLE_TEXT hue=hue design=d size=40.0 />
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
