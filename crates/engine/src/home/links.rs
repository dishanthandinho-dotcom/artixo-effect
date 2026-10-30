//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's link buttons).

pub const APP_NAME: &str = "ARTIXO Effect";
pub const APP_SLUG: &str = "artixo-effect";
pub const WEBSITE: &str = "https://artixo-one.vercel.app";
pub const APP_PAGE: &str = "https://artixo-one.vercel.app";
pub const GITHUB: &str = "https://github.com/dishanthandinho-dotcom/artixo-effect";
pub const ISSUES: &str = "https://github.com/dishanthandinho-dotcom/artixo-effect/issues";

/// Sibling apps of the ARTIXO ONE family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("ARTIXO CUT", "artixo-cut"),
    ("ARTIXO DESIGN", "artixo-design"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/dishanthandinho-dotcom/{slug}")
}
pub fn page_of(_slug: &str) -> String {
    WEBSITE.to_string()
}
