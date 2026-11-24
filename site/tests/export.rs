//! The static export for GitHub Pages, behind `/LOCALINK` (its own test binary: the base
//! path is set once per process).

use std::path::Path;

fn files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            files(&path, out);
        } else {
            out.push(path);
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn exports_every_page_behind_the_base_path() {
    let out = std::env::temp_dir().join(format!("localink-export-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    let written = localink_site::server::export::export(
        &out,
        "/LOCALINK",
        Some("https://maxlestage.github.io/LOCALINK".into()),
    )
    .await
    .unwrap();
    assert!(written > 20);
    for file in [
        "index.html",
        "404.html",
        "fr/index.html",
        "en/privacy/index.html",
        "es/terms/index.html",
        "fr/legal/index.html",
        "en/404/index.html",
        "sitemap.xml",
        "robots.txt",
        "manifest.webmanifest",
        "fr/manifest.webmanifest",
        "sw.js",
        "site.css",
        "fonts/inter.woff2",
        "icons/icon-512.png",
        "assets/logo.svg",
        ".nojekyll",
    ] {
        assert!(out.join(file).is_file(), "{file}");
    }
    let home = std::fs::read_to_string(out.join("fr/index.html")).unwrap();
    assert!(
        home.contains(
            r#"<link rel="canonical" href="https://maxlestage.github.io/LOCALINK/fr/"/>"#
        )
    );
    assert!(home.contains("data-base=\"/LOCALINK\""));
    assert!(home.contains(r#"<link rel="manifest" href="/LOCALINK/fr/manifest.webmanifest"/>"#));
    let manifest = std::fs::read_to_string(out.join("fr/manifest.webmanifest")).unwrap();
    assert!(
        manifest.contains(r#""start_url":"/LOCALINK/fr/""#),
        "{manifest}"
    );
    assert!(
        manifest.contains(r#""/LOCALINK/icons/icon-192.png""#),
        "{manifest}"
    );
    let worker = std::fs::read_to_string(out.join("sw.js")).unwrap();
    assert!(
        worker.contains("\"/LOCALINK/icons/icon-192.png\""),
        "{worker}"
    );
    // The root and `404.html` send the visitor to their language.
    let root = std::fs::read_to_string(out.join("index.html")).unwrap();
    assert!(root.contains(r#"data-auto-lang="""#));
    assert!(root.contains(r#"href="/LOCALINK/es/""#));
    assert!(root.contains(r#"content="0; url=/LOCALINK/fr/""#));
    let missing = std::fs::read_to_string(out.join("404.html")).unwrap();
    assert!(missing.contains(r#"data-auto-lang="404/""#));

    // Every link of every page stays under the base path.
    let mut all = Vec::new();
    files(&out, &mut all);
    for file in all
        .iter()
        .filter(|f| f.extension().is_some_and(|e| e == "html"))
    {
        let html = std::fs::read_to_string(file).unwrap();
        for attr in ["href=\"/", "src=\"/", "from \"/", "path:\"/"] {
            for rest in html.split(attr).skip(1) {
                assert!(
                    rest.starts_with("LOCALINK/"),
                    "{}: {attr}{}",
                    file.display(),
                    &rest[..rest.len().min(40)]
                );
            }
        }
    }
    std::fs::remove_dir_all(&out).unwrap();
}
