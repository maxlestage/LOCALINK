//! The ephemeral shares of the iOS app, through Paquets' in-memory test client.

use std::sync::Arc;
use std::time::{Duration, Instant};

use localink_site::server::shares::{self, LIFETIME, MAX_FILE};
use localink_site::server::{Site, app};
use paquets::test::TestClient;

/// The shares live in one store for the whole process: these tests take turns, so
/// that the purge of one never removes the share of another.
static TURN: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn client() -> TestClient {
    app(Arc::new(Site::load(None))).test().await
}

fn field<'a>(json: &'a str, name: &str) -> &'a str {
    let start = json.find(&format!("\"{name}\":")).unwrap() + name.len() + 3;
    let rest = &json[start..];
    let rest = rest.trim_start_matches('"');
    let end = rest.find(['"', ',', '}']).unwrap();
    &rest[..end]
}

#[tokio::test]
async fn a_file_is_shared_fetched_and_deleted() {
    let _turn = TURN.lock().await;
    let client = client().await;
    let photo: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
    let res = client
        .post("/api/shares")
        .header("content-type", "image/jpeg")
        .header("x-file-name", "vacances%20%C3%A9t%C3%A9.jpg")
        .header("user-agent", "Localink/1.0 CFNetwork/3826 Darwin/25.0.0")
        .body(photo.clone())
        .await;
    assert_eq!(res.status(), 201, "{}", res.text());
    let json = res.text();
    let code = field(&json, "code").to_owned();
    let token = field(&json, "token").to_owned();
    assert_eq!(code.len(), 9);
    assert_eq!(field(&json, "expires_in"), "120");
    assert_eq!(field(&json, "size"), "200000");
    assert!(json.contains(r#""name":"vacances été.jpg""#), "{json}");

    // Typed by hand, in lower case and without the dash.
    let typed = code.replace('-', " ").to_lowercase();
    let info = client
        .get(&format!("/api/shares/{}/info", typed.replace(' ', "")))
        .await;
    assert_eq!(info.status(), 200);
    assert!(info.text().contains(r#""type":"image/jpeg""#));

    let file = client
        .get(&format!("/api/shares/{code}"))
        .header("accept-encoding", "br, gzip")
        .await;
    assert_eq!(file.status(), 200);
    assert_eq!(file.bytes().to_vec(), photo, "sent back as it came");
    assert_eq!(file.header("content-type"), Some("image/jpeg"));
    assert_eq!(
        file.header("x-localink-name"),
        Some("vacances%20%C3%A9t%C3%A9.jpg")
    );
    assert!(file.header("content-encoding").is_none());
    assert_eq!(file.header("cache-control"), Some("no-store"));
    let left: u64 = file
        .header("x-localink-expires-in")
        .unwrap()
        .parse()
        .unwrap();
    assert!((118..=120).contains(&left));

    // Only the phone that shared it can delete it.
    let wrong = client
        .delete(&format!("/api/shares/{code}"))
        .header("x-localink-token", "0000")
        .await;
    assert_eq!(wrong.status(), 403);
    let deleted = client
        .delete(&format!("/api/shares/{code}"))
        .header("x-localink-token", &token)
        .await;
    assert_eq!(deleted.status(), 204);
    assert_eq!(
        client.get(&format!("/api/shares/{code}")).await.status(),
        404
    );
}

#[tokio::test]
async fn a_share_is_erased_after_two_minutes() {
    let _turn = TURN.lock().await;
    let client = client().await;
    let res = client
        .post("/api/shares")
        .header("x-file-name", "note.txt")
        .header("content-type", "text/plain")
        .body("À tout à l'heure !")
        .await;
    assert_eq!(res.status(), 201);
    let code = field(&res.text(), "code").to_owned();
    assert_eq!(
        client.get(&format!("/api/shares/{code}")).await.status(),
        200
    );
    assert_eq!(LIFETIME, Duration::from_secs(120));
    // Two minutes and a second later, the purge has forgotten it.
    shares::purge(Instant::now() + LIFETIME + Duration::from_secs(1));
    let gone = client.get(&format!("/api/shares/{code}")).await;
    assert_eq!(gone.status(), 404);
    assert!(gone.text().contains("not_found"));
}

#[tokio::test]
async fn limits_are_enforced() {
    let _turn = TURN.lock().await;
    let client = client().await;
    let empty = client.post("/api/shares").body(Vec::<u8>::new()).await;
    assert_eq!(empty.status(), 400);
    let big = client
        .post("/api/shares")
        .header("x-file-name", "big.bin")
        .body(vec![0u8; MAX_FILE + 1])
        .await;
    assert_eq!(big.status(), 413, "{}", big.text());
    assert_eq!(client.get("/api/shares/nope").await.status(), 404);
    // A file of 3 MiB (above the default 2 MiB of the shield) gets through.
    let photo = client
        .post("/api/shares")
        .header("x-file-name", "IMG_2048.HEIC")
        .header("content-type", "image/heic")
        .body(vec![7u8; 3 * 1024 * 1024])
        .await;
    assert_eq!(photo.status(), 201, "{}", photo.text());
}
