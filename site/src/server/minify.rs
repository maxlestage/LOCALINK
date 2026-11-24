//! `localink-site minify <file.css>`: minifies the stylesheet in place with
//! lightningcss, before a deployment (Heroku, GitHub Pages). The repository keeps the
//! readable version.

use std::io;
use std::path::Path;

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

/// The minified form of `css`.
pub fn minify(css: &str) -> io::Result<String> {
    let options = ParserOptions {
        // A rule lightningcss does not know yet is kept as it is, not refused.
        error_recovery: true,
        ..ParserOptions::default()
    };
    let mut sheet = StyleSheet::parse(css, options).map_err(|e| io::Error::other(e.to_string()))?;
    sheet
        .minify(MinifyOptions::default())
        .map_err(|e| io::Error::other(e.to_string()))?;
    let out = sheet
        .to_css(PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        })
        .map_err(|e| io::Error::other(e.to_string()))?;
    Ok(out.code)
}

/// Minifies `path` in place; returns the sizes before and after.
pub fn minify_file(path: &Path) -> io::Result<(usize, usize)> {
    let css = std::fs::read_to_string(path)?;
    let min = minify(&css)?;
    std::fs::write(path, &min)?;
    Ok((css.len(), min.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stylesheet_minifies_and_keeps_its_modern_rules() {
        let css = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/public/site.css"))
            .unwrap();
        let min = minify(&css).unwrap();
        assert!(
            min.len() * 10 < css.len() * 8,
            "{} → {}",
            css.len(),
            min.len()
        );
        for kept in [
            "@property --angle",
            "::details-content",
            "interpolate-size",
            "@font-face",
            "prefers-reduced-motion",
            "env(safe-area-inset-top)",
            "fonts/inter.woff2",
            "conic-gradient(from var(--angle)",
        ] {
            assert!(min.contains(kept), "{kept} is missing");
        }
    }
}
