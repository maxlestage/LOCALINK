//! Line icons (24×24, stroked) and the LOCALINK logo, as active views: the same markup
//! on the server and in islands.

use paquets::prelude::*;

/// The stroke paths of an icon.
pub fn paths(name: &str) -> &'static [&'static str] {
    match name {
        "arrow" => &["M5 12h14", "m13 6 6 6-6 6"],
        "arrow-up" => &["M12 19V5", "m6 11 6-6 6 6"],
        "arrow-down" => &["M12 5v14", "m6 13 6 6 6-6"],
        "check" => &["m5 12.5 4.5 4.5L19 7.5"],
        "menu" => &["M4 8h16", "M4 16h16"],
        "close" => &["M6 6l12 12", "M18 6 6 18"],
        "user-off" => &[
            "M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8z",
            "M4 20c.6-3.4 3.9-6 8-6 1.7 0 3.3.4 4.6 1.2",
            "M3 3l18 18",
        ],
        "cloud-off" => &[
            "M7.5 18H17a4 4 0 0 0 1.6-7.7A6 6 0 0 0 8 7.4",
            "M5.2 10.2A4 4 0 0 0 7 18",
            "M3 3l18 18",
        ],
        "qr" => &[
            "M4 4h6v6H4z",
            "M14 4h6v6h-6z",
            "M4 14h6v6H4z",
            "M14 14h2v2h-2z",
            "M18 18h2v2h-2z",
            "M14 18h2",
            "M18 14h2",
        ],
        "wifi" => &[
            "M2.5 9a14 14 0 0 1 19 0",
            "M5.5 12.5a9.5 9.5 0 0 1 13 0",
            "M8.8 16a4.8 4.8 0 0 1 6.4 0",
            "M12 19.5h.01",
        ],
        "files" => &[
            "M14 3H7a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2V7z",
            "M14 3v4h4",
            "M9 21h8a2 2 0 0 0 2-2V9",
        ],
        "phone" => &[
            "M8 2h8a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z",
            "M11 18h2",
        ],
        "swift" => &[
            "M19.5 14.5c1.6-4.8-1.4-9.6-6-12 2.6 3 3.3 6.4 2.2 9.3C12.8 10 9 7.2 5 4c2.2 2.8 4.5 5.2 6.5 7.3-2.4-1.3-5-3-7.5-5 2.6 4.4 6 7.6 9.6 9.2-2.6 1.3-6 1.5-9.6.2 3 2.8 7.4 4.4 11.4 2.6 1.4-.7 2.8-.4 3.6.7.4-1.4.2-3.3-1.4-4.5z",
        ],
        "shield" => &[
            "M12 3 4 6v6c0 5 3.5 8 8 9 4.5-1 8-4 8-9V6z",
            "m9 12 2 2 4-4",
        ],
        "bolt" => &["M13 2 4 14h7l-1 8 9-12h-7z"],
        "image" => &["M4 5h16v14H4z", "m4 16 5-5 4 4 2-2 5 5", "M15.5 9.5h.01"],
        "video" => &["M3 6h12v12H3z", "m15 10 6-3.5v11L15 14"],
        "pdf" => &["M14 3H6v18h12V7z", "M14 3v4h4", "M8.5 13h7", "M8.5 16.5h5"],
        "music" => &[
            "M9 18V5l11-2v13",
            "M9 18a3 3 0 1 1-6 0 3 3 0 0 1 6 0z",
            "M20 16a3 3 0 1 1-6 0 3 3 0 0 1 6 0z",
        ],
        "github" => &[
            "M9 19c-4.3 1.4-4.3-2.5-6-3m12 5v-3.5c0-1 .1-1.4-.5-2 2.8-.3 5.5-1.4 5.5-6a4.6 4.6 0 0 0-1.3-3.2 4.2 4.2 0 0 0-.1-3.2s-1.1-.3-3.5 1.3a12.3 12.3 0 0 0-6.2 0C6.5 2.8 5.4 3.1 5.4 3.1a4.2 4.2 0 0 0-.1 3.2A4.6 4.6 0 0 0 4 9.5c0 4.6 2.7 5.7 5.5 6-.6.6-.6 1.2-.5 2V21",
        ],
        "globe" => &[
            "M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18z",
            "M3 12h18",
            "M12 3c2.5 2.7 3.8 5.7 3.8 9s-1.3 6.3-3.8 9c-2.5-2.7-3.8-5.7-3.8-9S9.5 5.7 12 3z",
        ],
        "download" => &["M12 4v11", "m7 10 5 5 5-5", "M5 20h14"],
        "offline" => &[
            "M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18z",
            "m8.5 12.5 2.5 2.5 4.5-5",
        ],
        "box" => &[
            "M12 2.5 3.5 7v10l8.5 4.5 8.5-4.5V7z",
            "m3.5 7 8.5 4.5L20.5 7",
            "M12 11.5v10",
        ],
        "map" => &["M9 4 3 6v14l6-2 6 2 6-2V4l-6 2z", "M9 4v14", "M15 6v14"],
        "chat" => &["M4 5h16v11H9l-5 4z", "M8 9.5h8", "M8 12.5h5"],
        "doc" => &["M14 3H6v18h12V7z", "M14 3v4h4", "M9 12h6", "M9 16h6"],
        "link" => &[
            "M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1",
            "M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1",
        ],
        "plus" => &["M12 5v14", "M5 12h14"],
        "spark" => {
            &["M12 3c.8 4.5 2.5 6.2 7 7-4.5.8-6.2 2.5-7 7-.8-4.5-2.5-6.2-7-7 4.5-.8 6.2-2.5 7-7z"]
        }
        _ => &[],
    }
}

/// An icon: `<svg class="icon">`.
pub fn icon(name: &str) -> Element {
    svg()
        .class("icon")
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", "currentColor")
        .attr("stroke-width", "1.7")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .attr("aria-hidden", "true")
        .children(paths(name).iter().map(|d| path().attr("d", *d)))
}

/// The icon of a file type of the simulator.
pub fn kind_icon(index: usize) -> &'static str {
    ["image", "video", "pdf", "music"][index % 4]
}

/// The logo: two devices linked by a wave (the same drawing as
/// `public/assets/logo.svg`).
pub fn logo() -> Element {
    svg()
        .class("logo-mark")
        .attr("viewBox", "0 0 64 64")
        .attr("aria-hidden", "true")
        .child(
            defs().child(
                linear_gradient()
                    .id("lk-grad")
                    .attr("x1", "6")
                    .attr("y1", "6")
                    .attr("x2", "58")
                    .attr("y2", "58")
                    .attr("gradientUnits", "userSpaceOnUse")
                    .child(stop().attr("offset", "0").attr("stop-color", "#2de2ff"))
                    .child(stop().attr("offset", "1").attr("stop-color", "#7c5cff")),
            ),
        )
        .child(
            rect()
                .attr("x", "3")
                .attr("y", "3")
                .attr("width", "58")
                .attr("height", "58")
                .attr("rx", "18")
                .attr("fill", "url(#lk-grad)"),
        )
        .child(
            path()
                .class("logo-mark__wave")
                .attr("d", "M19 40c5-12 21-12 26-16")
                .attr("fill", "none")
                .attr("stroke", "#05060f")
                .attr("stroke-width", "4.5")
                .attr("stroke-linecap", "round")
                .attr("stroke-dasharray", "2 7"),
        )
        .child(
            circle()
                .class("logo-mark__dot")
                .attr("cx", "18")
                .attr("cy", "42")
                .attr("r", "7")
                .attr("fill", "#05060f"),
        )
        .child(
            circle()
                .class("logo-mark__dot logo-mark__dot--b")
                .attr("cx", "46")
                .attr("cy", "22")
                .attr("r", "7")
                .attr("fill", "#05060f"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_used_exists() {
        for name in [
            "arrow",
            "arrow-up",
            "arrow-down",
            "check",
            "menu",
            "close",
            "user-off",
            "cloud-off",
            "qr",
            "wifi",
            "files",
            "phone",
            "swift",
            "shield",
            "bolt",
            "image",
            "video",
            "pdf",
            "music",
            "github",
            "globe",
            "download",
            "offline",
            "box",
            "map",
            "chat",
            "doc",
            "link",
            "plus",
            "spark",
        ] {
            assert!(!paths(name).is_empty(), "{name}");
        }
        let html = Node::from(icon("qr")).render();
        assert!(html.starts_with("<svg class=\"icon\" viewBox=\"0 0 24 24\""));
    }
}
