//! Brotli at its best quality for everything the site sends.
//!
//! Vitesse's own `compress()` makes static files at Brotli 11 but generated responses at
//! Brotli 1, which is right for pages that change on every request. The pages of this
//! site never do (only the address of the site changes them), so they get the same
//! treatment as files: Brotli 11 with a window that fits the body, made once and kept
//! in memory, keyed by the content itself. Static files share the cache, filled at
//! startup ([`warm`]), so the first visitor waits for nothing. Browsers without Brotli
//! get gzip from Vitesse's middleware, which runs around this one.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

use paquets::prelude::*;
use paquets::vitesse::Body;

/// Smaller bodies are not worth a `Content-Encoding`.
const MIN: usize = 512;
/// What the cache may hold; beyond, it starts again empty.
const CACHE_BYTES: usize = 32 * 1024 * 1024;

#[derive(Default)]
struct Cache {
    entries: HashMap<(usize, u64), Arc<Vec<u8>>>,
    bytes: usize,
}

fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(Mutex::default)
}

/// FNV-1a over the whole body: with the length, the key of the cache.
fn fingerprint(bytes: &[u8]) -> (usize, u64) {
    let hash = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    });
    (bytes.len(), hash)
}

/// Whether `Accept-Encoding` accepts Brotli (`br`, with a quality above 0).
pub fn accepts_brotli(header: &str) -> bool {
    header.split(',').any(|part| {
        let mut pieces = part.trim().split(';');
        let name = pieces.next().unwrap_or("").trim();
        let quality = pieces
            .find_map(|p| {
                p.trim()
                    .strip_prefix("q=")
                    .and_then(|q| q.trim().parse().ok())
            })
            .unwrap_or(1.0_f32);
        name.eq_ignore_ascii_case("br") && quality > 0.0
    })
}

/// Text-like types, which compress well (fonts in WOFF2 and images already are).
pub fn compressible(content_type: &str) -> bool {
    let mime = content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    mime.starts_with("text/")
        || matches!(
            mime.as_str(),
            "application/javascript"
                | "application/json"
                | "application/ld+json"
                | "application/manifest+json"
                | "application/xml"
                | "application/wasm"
                | "image/svg+xml"
                | "image/x-icon"
        )
}

/// Brotli 11, with the largest window the body can use (up to 16 MiB).
pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let window = (usize::BITS - bytes.len().max(2).saturating_sub(1).leading_zeros()).clamp(10, 24);
    let params = brotli::enc::BrotliEncoderParams {
        quality: 11,
        lgwin: window as i32,
        size_hint: bytes.len(),
        ..Default::default()
    };
    let mut out = Vec::with_capacity(bytes.len() / 3);
    let mut input = bytes;
    // Writing into a Vec cannot fail.
    let _ = brotli::BrotliCompress(&mut input, &mut out, &params);
    out
}

/// The Brotli form of `bytes`, from the cache or made (and kept) now. `None` when it is
/// not smaller.
pub fn compressed(bytes: &[u8]) -> Option<Arc<Vec<u8>>> {
    let key = fingerprint(bytes);
    if let Some(found) = cache().lock().ok()?.entries.get(&key) {
        return (found.len() < bytes.len()).then(|| found.clone());
    }
    let encoded = Arc::new(encode(bytes));
    if let Ok(mut cache) = cache().lock() {
        if cache.bytes + encoded.len() > CACHE_BYTES {
            *cache = Cache::default();
        }
        cache.bytes += encoded.len();
        cache.entries.insert(key, encoded.clone());
    }
    (encoded.len() < bytes.len()).then_some(encoded)
}

/// Compresses, on a background thread, every text-like file of `dirs` (the stylesheet,
/// the browser bundle, the logo…), so that their first request finds them ready.
pub fn warm(dirs: &[&Path]) {
    let files: Vec<_> = dirs.iter().flat_map(|dir| files(dir)).collect();
    let _ = std::thread::Builder::new()
        .name("brotli-warm".into())
        .spawn(move || {
            for file in files {
                let compressible = matches!(
                    file.extension().and_then(|e| e.to_str()),
                    Some("css" | "js" | "wasm" | "svg" | "json" | "txt" | "xml")
                );
                if compressible && let Ok(bytes) = std::fs::read(&file) {
                    compressed(&bytes);
                }
            }
        });
}

fn files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                found.extend(files(&path));
            } else {
                found.push(path);
            }
        }
    }
    found
}

/// The middleware: Brotli 11 for every response worth it, when the browser accepts it.
pub async fn brotli_best(req: Request, next: Next) -> Response {
    let brotli = accepts_brotli(req.header("accept-encoding").unwrap_or(""));
    let mut res = next.run(req).await;
    let headers = res.headers();
    let worth = brotli
        && res.error().is_none()
        && res.status_code().is_success()
        && res.status_code().as_u16() != 206
        && !headers.contains_key("content-encoding")
        && !headers.contains_key("content-range")
        && headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .is_some_and(compressible)
        && res
            .body()
            .size()
            .is_some_and(|n| n >= MIN as u64 && n <= CACHE_BYTES as u64);
    if !worth {
        return res;
    }
    let body = std::mem::take(res.body_mut());
    let Ok(bytes) = body.to_bytes().await else {
        res.set_status(500);
        return res;
    };
    let data = bytes.clone();
    let encoded = tokio::task::spawn_blocking(move || compressed(&data))
        .await
        .ok()
        .flatten();
    let Some(encoded) = encoded else {
        *res.body_mut() = Body::from(bytes);
        return res;
    };
    let headers = res.headers_mut();
    headers.remove("content-length");
    // The compressed form is another representation: its own validator.
    if let Some(etag) = headers.get("etag").and_then(|v| v.to_str().ok()) {
        let etag = etag.trim_end_matches('"').to_owned() + "-br\"";
        res.set_header("etag", etag);
    }
    let vary = match res.headers().get("vary").and_then(|v| v.to_str().ok()) {
        Some(v) if v.to_ascii_lowercase().contains("accept-encoding") => v.to_owned(),
        Some(v) => format!("{v}, accept-encoding"),
        None => "accept-encoding".to_owned(),
    };
    res.set_header("vary", vary);
    res.set_header("content-encoding", "br");
    *res.body_mut() = Body::from(encoded.as_ref().clone());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiates_brotli() {
        assert!(accepts_brotli("gzip, deflate, br, zstd"));
        assert!(accepts_brotli("br;q=0.5"));
        assert!(!accepts_brotli("br;q=0, gzip"));
        assert!(!accepts_brotli("gzip"));
        assert!(!accepts_brotli(""));
    }

    #[test]
    fn text_compresses_and_round_trips() {
        assert!(compressible("text/html; charset=utf-8"));
        assert!(compressible("application/wasm"));
        assert!(!compressible("font/woff2"));
        assert!(!compressible("image/png"));
        let text = "LOCALINK — près ou loin. ".repeat(400);
        let encoded = compressed(text.as_bytes()).unwrap();
        assert!(encoded.len() * 20 < text.len(), "{}", encoded.len());
        let mut decoded = Vec::new();
        brotli::BrotliDecompress(&mut encoded.as_slice(), &mut decoded).unwrap();
        assert_eq!(decoded, text.as_bytes());
        // Kept: the same body is not compressed twice.
        assert!(Arc::ptr_eq(&encoded, &compressed(text.as_bytes()).unwrap()));
    }
}
