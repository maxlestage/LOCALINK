//! A real QR code, drawn as SVG modules that the page animates one by one (each module
//! gets its distance to the center, `--d`, for a ripple reveal).

use qrcode::{EcLevel, QrCode};

/// The SVG of a QR code for `data`: one `<rect>` per dark module, the three finder
/// patterns drawn as rounded squares. `None` if the data is too long.
pub fn svg(data: &str, class: &str, label: &str) -> Option<String> {
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M).ok()?;
    let width = code.width();
    let colors = code.to_colors();
    let quiet = 2;
    let size = width + quiet * 2;
    let center = (width as f32 - 1.0) / 2.0;
    let finder = |x: usize, y: usize| {
        let near = |a: usize| a < 7 || a >= width - 7;
        let corner = (x < 7 && y < 7) || (x >= width - 7 && y < 7) || (x < 7 && y >= width - 7);
        corner && near(x) && near(y)
    };
    let mut out = format!(
        r#"<svg class="{class}" viewBox="0 0 {size} {size}" role="img" aria-label="{label}" shape-rendering="crispEdges"><rect class="qr__bg" width="{size}" height="{size}" rx="2"/>"#,
        label = escape(label),
    );
    out.push_str(r#"<g class="qr__modules">"#);
    for y in 0..width {
        for x in 0..width {
            if colors[y * width + x] != qrcode::Color::Dark || finder(x, y) {
                continue;
            }
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let d = (dx * dx + dy * dy).sqrt().round() as u32;
            out.push_str(&format!(
                r#"<rect x="{}" y="{}" width="1" height="1" style="--d:{d}"/>"#,
                x + quiet,
                y + quiet
            ));
        }
    }
    out.push_str("</g>");
    for (fx, fy) in [(0, 0), (width - 7, 0), (0, width - 7)] {
        let (x, y) = (fx + quiet, fy + quiet);
        out.push_str(&format!(
            r#"<g class="qr__finder"><rect x="{}" y="{}" width="6" height="6" rx="1.6" fill="none" stroke-width="1"/><rect x="{}" y="{}" width="3" height="3" rx=".8"/></g>"#,
            x as f32 + 0.5,
            y as f32 + 0.5,
            x + 2,
            y + 2
        ));
    }
    out.push_str(r#"<rect class="qr__scan" x="0" y="0" width="100%" height="1.2"/></svg>"#);
    Some(out)
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_a_code() {
        let svg = svg(
            "https://maxlestage.github.io/LOCALINK/fr/",
            "qr",
            "QR \"code\"",
        )
        .unwrap();
        assert!(svg.starts_with("<svg class=\"qr\""));
        assert!(svg.contains("aria-label=\"QR &quot;code&quot;\""));
        assert!(svg.matches("<rect").count() > 100);
        assert_eq!(svg.matches("qr__finder").count(), 3);
    }
}
