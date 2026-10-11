//! Product identity. Change the name, contact details and logo paths here; colors, radii and fonts are in style/brand.css.
//! Text that needs the name uses the placeholders `{brand}` and `{support_email}`, filled by `fill`.

pub const NAME: &str = "Streams";
pub const COMPANY: &str = "Streams";
pub const SUPPORT_EMAIL: &str = "support@streams.watch";
pub const LOGO_WORDMARK: &str = "/assets/logos/streams-wordmark-white.svg";
pub const LOGO_WORDMARK_PRIMARY: &str = "/assets/logos/streams-wordmark-primary.svg";
pub const DEFAULT_AVATAR: &str = "/assets/avatars/default.svg";
pub const LOGO_EMBLEM: &str = "/assets/logos/streams-mark-white.svg";

pub fn fill(text: &str) -> String {
    text.replace("{brand}", NAME).replace("{support_email}", SUPPORT_EMAIL)
}

pub const HELP_EMAIL: &str = "help@streams.watch";
pub const CONTRIBUTE_EMAIL: &str = "contribute@streams.watch";
