// Modified by The Agent Factory for Claude Academy: rebranding and web language detection (see BRANDING.md).
//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "EffectCraft";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://www.skool.com/claudeacademy";
pub const WEBSITE: &str = "https://claudeacademy.ca";
pub const APP_PAGE: &str = "https://claudeacademy.ca";
pub const GITHUB: &str = "https://github.com/storytold/effectcraft";
pub const ISSUES: &str = "https://github.com/storytold/effectcraft/issues";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://claudeacademy.ca/#{slug}")
}
