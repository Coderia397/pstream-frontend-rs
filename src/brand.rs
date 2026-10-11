//! Product identity. Change the name, contact details and logo paths here; colors, radii and fonts are in style/brand.css.
//! Text that needs the name uses the placeholders `{brand}` and `{support_email}`, filled by `fill`.

pub const NAME: &str = "Streams";
pub const COMPANY: &str = "Streams";
pub const SUPPORT_EMAIL: &str = "support@streams.watch";
pub const LOGO_WORDMARK: &str = "/assets/logos/pstream-logo.svg";
pub const DEFAULT_AVATAR: &str = "/assets/avatars/default.svg";
pub const LOGO_EMBLEM: &str = "/assets/logos/p-pstream-logo.svg";

pub fn fill(text: &str) -> String {
    text.replace("{brand}", NAME).replace("{support_email}", SUPPORT_EMAIL)
}
