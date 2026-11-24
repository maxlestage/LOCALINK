//! The whole site through Paquets' in-memory test client: every page of every language,
//! the redirections, the PWA, the sitemap and the static files.

use std::sync::Arc;

use localink_site::routes::{Lang, Page, Route};
use localink_site::server::{Site, app};
use paquets::test::TestClient;

async fn client() -> TestClient {
    app(Arc::new(Site::load(None))).test().await
}

#[tokio::test]
async fn every_page_renders_in_every_language() {
    let client = client().await;
    let routes = Route::all();
    assert_eq!(routes.len(), 3 * 4);
    for route in routes {
        let path = route.path();
        let res = client.get(&path).header("host", "example.com").await;
        assert_eq!(res.status(), 200, "{path}");
        assert!(
            res.header("set-cookie").is_none(),
            "{path}: no cookie, ever"
        );
        let html = res.text();
        let code = route.lang.code();
        assert!(
            html.starts_with(&format!("<!DOCTYPE html><html lang=\"{code}\">")),
            "{path}"
        );
        assert!(
            html.contains(&format!(
                r#"<link rel="canonical" href="http://example.com{path}"/>"#
            )),
            "{path}"
        );
        for lang in Lang::ALL {
            let alternate = route.with_lang(lang).path();
            assert!(
                html.contains(&format!(
                    r#"hreflang="{}" href="http://example.com{alternate}""#,
                    lang.code()
                )),
                "{path}: {alternate}"
            );
        }
        // Every page has an island (the install button of the footer), so the browser
        // bundle starts the page motion everywhere.
        assert!(
            html.contains(r#"<active-island data-island="install""#),
            "{path}"
        );
        assert!(html.contains("/site.css?v="), "{path}");
        assert!(html.contains("class=\"footer\""), "{path}");
        assert!(
            html.contains(&format!(
                r#"<link rel="manifest" href="/{code}/manifest.webmanifest"/>"#
            )),
            "{path}"
        );
        // Every legal page is linked from the footer.
        for page in [Page::Privacy, Page::Terms, Page::Legal] {
            assert!(
                html.contains(&format!("href=\"{}\"", Route::new(route.lang, page).path())),
                "{path}: {page:?}"
            );
        }
        let csp = res
            .header("content-security-policy")
            .unwrap_or_default()
            .to_owned();
        assert!(csp.contains("'sha256-"), "{path}: {csp}");
        assert!(
            !csp.contains("fonts.googleapis.com"),
            "fonts are served by the site"
        );
        // Nothing is loaded from another site.
        assert!(!html.contains("src=\"http"), "{path}");
    }
}

#[tokio::test]
async fn the_home_page_has_every_section() {
    let client = client().await;
    for lang in Lang::ALL {
        let html = client.get(&format!("/{}/", lang.code())).await.text();
        let t = lang.texts();
        for id in ["how", "demo", "share", "features", "privacy", "app", "faq"] {
            assert!(html.contains(&format!("id=\"{id}\"")), "{id}");
        }
        assert!(html.contains(r#"<active-island data-island="simulator""#));
        // Connection by code, and sharing erased after 2 minutes.
        assert!(html.contains(r#"<active-island data-island="share""#));
        assert!(html.contains(localink_site::islands::FIRST_CODE));
        assert!(html.contains("2:00"));
        assert!(html.contains("class=\"qr qr--code\""));
        assert!(html.contains("route--remote"));
        assert!(html.contains(&escape(t.share_label)));
        assert!(html.contains("data-network"), "the 2D canvas");
        assert!(html.contains("class=\"qr qr--big\""), "the QR code");
        assert!(html.contains("animateMotion"), "the privacy diagrams");
        assert!(html.contains("application/ld+json"));
        for item in t.faq {
            assert!(html.contains(&escape(item.title)), "{}", item.title);
        }
        assert!(html.contains(t.hero_primary));
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&#39;")
        .replace('"', "&quot;")
}

#[tokio::test]
async fn the_root_goes_to_the_language_of_the_browser() {
    let client = client().await;
    for (header, lang) in [
        ("es-ES,es;q=0.9", "es"),
        ("en-GB", "en"),
        ("de", "fr"),
        ("", "fr"),
    ] {
        let res = client.get("/").header("accept-language", header).await;
        assert_eq!(res.status(), 302);
        assert_eq!(res.header("location"), Some(format!("/{lang}/").as_str()));
    }
    let res = client.get("/en/privacy").await;
    assert_eq!(res.status(), 301);
    assert_eq!(res.header("location"), Some("/en/privacy/"));
}

#[tokio::test]
async fn unknown_pages_are_not_found_in_their_language() {
    let client = client().await;
    for (path, text) in [
        ("/es/nada/", Lang::Es.texts().nf_title),
        ("/en/a/b/", Lang::En.texts().nf_title),
        ("/nope", Lang::Fr.texts().nf_title),
    ] {
        let res = client.get(path).await;
        assert_eq!(res.status(), 404, "{path}");
        let html = res.text();
        assert!(html.contains(&escape(text)), "{path}");
        assert!(html.contains("noindex"), "{path}");
    }
}

#[tokio::test]
async fn the_site_is_installable() {
    let client = client().await;
    let manifest = client.get("/es/manifest.webmanifest").await;
    assert_eq!(manifest.status(), 200);
    let json = manifest.text();
    assert!(json.contains(r#""start_url":"/es/""#), "{json}");
    assert!(json.contains("/icons/icon-512.png"));
    assert!(json.contains("maskable"));
    let worker = client.get("/sw.js").await;
    assert_eq!(worker.status(), 200);
    assert!(worker.text().contains("/icons/icon-192.png"));
    for file in [
        "/icons/icon-192.png",
        "/icons/icon-512.png",
        "/icons/icon-maskable-512.png",
        "/icons/apple-touch-icon.png",
        "/assets/logo.svg",
        "/assets/og-image.png",
        "/fonts/inter.woff2",
        "/fonts/space-grotesk.woff2",
        "/fonts/jetbrains-mono.woff2",
        "/site.css",
    ] {
        assert_eq!(client.get(file).await.status(), 200, "{file}");
    }
}

#[tokio::test]
async fn sitemap_and_robots() {
    let client = client().await;
    let xml = client
        .get("/sitemap.xml")
        .header("host", "localink.example")
        .await
        .text();
    assert_eq!(xml.matches("<url>").count(), 12);
    assert!(xml.contains("<loc>http://localink.example/es/legal/</loc>"));
    let robots = client.get("/robots.txt").await.text();
    assert!(robots.contains("Sitemap: "));
}
