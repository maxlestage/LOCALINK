//! The ephemeral shares of the iOS app, when the two phones are far apart.
//!
//! - `POST /api/shares`: the body is the file (at most 25 MiB), `X-File-Name` its name
//!   (percent-encoded), `Content-Type` its type. Answers `201` with
//!   `{"code","token","name","size","expires_in"}`.
//! - `GET /api/shares/{code}`: the file, while it lives; `404` after.
//! - `GET /api/shares/{code}/info`: `{"name","size","type","expires_in"}`.
//! - `DELETE /api/shares/{code}` with `X-Localink-Token`: deletes it before the end.
//!
//! A share lives [`LIFETIME`] (2 minutes), in memory only: nothing is ever written to
//! disk, and a restart forgets everything. Codes are `XXXX-XXXX` from an alphabet
//! without 0/O and 1/I/L, the same as the site and the app.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use paquets::prelude::*;

/// How long a share stays on the server.
pub const LIFETIME: Duration = Duration::from_secs(120);
/// The largest file (Heroku gives a request 30 seconds: more would not get through on
/// a phone network anyway).
pub const MAX_FILE: usize = 25 * 1024 * 1024;
/// What all the shares may hold together.
pub const MAX_TOTAL: usize = 300 * 1024 * 1024;
/// The characters of a code (as on the site and in the app).
pub const ALPHABET: &[u8] = b"23456789ABCDEFGHJKMNPQRSTUVWXYZ";

struct Share {
    name: String,
    mime: String,
    size: usize,
    data: Vec<u8>,
    token: String,
    expires: Instant,
}

fn store() -> &'static Mutex<HashMap<String, Share>> {
    static STORE: OnceLock<Mutex<HashMap<String, Share>>> = OnceLock::new();
    STORE.get_or_init(Mutex::default)
}

/// Forgets every share whose time is over at `now`. Returns how many are left.
pub fn purge(now: Instant) -> usize {
    let Ok(mut shares) = store().lock() else {
        return 0;
    };
    shares.retain(|_, share| share.expires > now);
    shares.len()
}

/// Purges every 10 seconds, on a thread of its own, for the life of the server.
pub fn start_purge() {
    let _ = std::thread::Builder::new()
        .name("shares-purge".into())
        .spawn(|| {
            loop {
                std::thread::sleep(Duration::from_secs(10));
                purge(Instant::now());
            }
        });
}

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    // The system's source never fails on the platforms the server runs on; if it
    // did, the clock keeps codes different.
    if getrandom::fill(&mut bytes).is_err() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (nanos >> ((i % 16) * 8)) as u8;
        }
    }
    bytes
}

/// A new code, `XXXX-XXXX`.
pub fn new_code() -> String {
    let random = random_bytes::<8>();
    let mut code = String::with_capacity(9);
    for (i, b) in random.iter().enumerate() {
        if i == 4 {
            code.push('-');
        }
        code.push(char::from(ALPHABET[usize::from(*b) % ALPHABET.len()]));
    }
    code
}

fn new_token() -> String {
    random_bytes::<16>()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A code as typed (`7kq4 m9xp`, `7KQ4M9XP`) in its form `7KQ4-M9XP`; `None` if it
/// cannot be one.
pub fn normalize(code: &str) -> Option<String> {
    let chars: Vec<char> = code
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect();
    if chars.len() != 8
        || !chars
            .iter()
            .all(|c| c.is_ascii() && ALPHABET.contains(&(*c as u8)))
    {
        return None;
    }
    let mut out: String = chars[..4].iter().collect();
    out.push('-');
    out.extend(&chars[4..]);
    Some(out)
}

/// `%E2%82%AC` → `€`; invalid sequences are kept as they are.
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(b) = u8::from_str_radix(&text[i + 1..i + 3], 16)
        {
            out.push(b);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Percent-encodes everything but unreserved characters (RFC 3986).
pub fn percent_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// A file name safe to give back: no folders, no control characters, 120 characters
/// at most, `fichier` when nothing is left.
pub fn clean_name(raw: &str) -> String {
    let decoded = percent_decode(raw);
    let base = decoded.rsplit(['/', '\\']).next().unwrap_or("");
    let name: String = base
        .chars()
        .filter(|c| !c.is_control() && !matches!(c, '"' | '<' | '>' | '|' | ':' | '*' | '?'))
        .take(120)
        .collect();
    let name = name.trim().trim_start_matches('.').trim().to_owned();
    if name.is_empty() {
        "fichier".to_owned()
    } else {
        name
    }
}

/// A MIME type that is only `type/subtype` made of safe characters, else
/// `application/octet-stream`.
fn clean_mime(raw: Option<&str>) -> String {
    let mime = raw.unwrap_or("").split(';').next().unwrap_or("").trim();
    let valid = mime.len() <= 100
        && mime.split_once('/').is_some_and(|(a, b)| {
            !a.is_empty()
                && !b.is_empty()
                && mime
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "/+-.".contains(c))
        });
    if valid {
        mime.to_ascii_lowercase()
    } else {
        "application/octet-stream".to_owned()
    }
}

fn json(status: u16, body: String) -> Response {
    Response::new()
        .status(status)
        .header("content-type", "application/json; charset=utf-8")
        .header("cache-control", "no-store")
        .send(body)
}

fn error(status: u16, code: &str, message: &str) -> Response {
    json(
        status,
        format!(r#"{{"error":"{code}","message":"{message}"}}"#),
    )
}

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn seconds_left(expires: Instant, now: Instant) -> u64 {
    expires.saturating_duration_since(now).as_secs_f64().ceil() as u64
}

/// `POST /api/shares`.
async fn create(mut req: Request) -> Response {
    req.set_body_limit(MAX_FILE);
    let declared = req
        .header("content-length")
        .and_then(|v| v.trim().parse::<usize>().ok());
    if declared.is_some_and(|n| n > MAX_FILE) {
        return error(413, "too_large", "The file is larger than 25 MiB.");
    }
    let name = clean_name(req.header("x-file-name").unwrap_or(""));
    let mime = clean_mime(req.header("content-type"));
    let Ok(bytes) = req.bytes().await else {
        return error(413, "too_large", "The file is larger than 25 MiB.");
    };
    if bytes.is_empty() {
        return error(400, "empty", "The file is empty.");
    }
    let now = Instant::now();
    let Ok(mut shares) = store().lock() else {
        return error(500, "unavailable", "Try again.");
    };
    shares.retain(|_, share| share.expires > now);
    let used: usize = shares.values().map(|s| s.size).sum();
    if used + bytes.len() > MAX_TOTAL {
        return error(
            507,
            "full",
            "The server is full for a moment: try again in two minutes.",
        );
    }
    let mut code = new_code();
    while shares.contains_key(&code) {
        code = new_code();
    }
    let token = new_token();
    let size = bytes.len();
    let body = format!(
        r#"{{"code":"{code}","token":"{token}","name":{},"size":{size},"expires_in":{}}}"#,
        json_string(&name),
        LIFETIME.as_secs()
    );
    shares.insert(
        code,
        Share {
            name,
            mime,
            size,
            data: bytes.to_vec(),
            token,
            expires: now + LIFETIME,
        },
    );
    json(201, body)
}

/// `GET /api/shares/{code}` and `GET /api/shares/{code}/info`.
async fn fetch(req: Request) -> Response {
    let Some(code) = req.param("code").and_then(normalize) else {
        return error(404, "not_found", "This code leads to no share.");
    };
    let info = req.path().ends_with("/info");
    let now = Instant::now();
    let Ok(mut shares) = store().lock() else {
        return error(500, "unavailable", "Try again.");
    };
    shares.retain(|_, share| share.expires > now);
    let Some(share) = shares.get(&code) else {
        return error(
            404,
            "not_found",
            "This code leads to no share, or its 2 minutes are over.",
        );
    };
    let left = seconds_left(share.expires, now);
    if info {
        return json(
            200,
            format!(
                r#"{{"code":"{code}","name":{},"size":{},"type":{},"expires_in":{left}}}"#,
                json_string(&share.name),
                share.size,
                json_string(&share.mime)
            ),
        );
    }
    Response::new()
        .header("content-type", share.mime.clone())
        .header(
            "content-disposition",
            format!(
                "attachment; filename=\"{}\"; filename*=UTF-8''{}",
                share
                    .name
                    .chars()
                    .map(|c| if c.is_ascii() && c != '"' { c } else { '_' })
                    .collect::<String>(),
                percent_encode(&share.name)
            ),
        )
        .header("x-localink-name", percent_encode(&share.name))
        .header("x-localink-expires-in", left.to_string())
        .header("cache-control", "no-store")
        .send(share.data.clone())
}

/// `DELETE /api/shares/{code}`.
async fn remove(req: Request) -> Response {
    let Some(code) = req.param("code").and_then(normalize) else {
        return error(404, "not_found", "This code leads to no share.");
    };
    let token = req
        .header("x-localink-token")
        .unwrap_or("")
        .trim()
        .to_owned();
    let Ok(mut shares) = store().lock() else {
        return error(500, "unavailable", "Try again.");
    };
    match shares.get(&code) {
        Some(share) if !token.is_empty() && share.token == token => {
            shares.remove(&code);
            Response::new()
                .status(204)
                .header("cache-control", "no-store")
        }
        Some(_) => error(
            403,
            "forbidden",
            "Only the phone that shared this file can delete it.",
        ),
        None => error(404, "not_found", "This code leads to no share."),
    }
}

/// Mounts the routes.
pub fn mount(app: &mut App) {
    app.post("/api/shares", create);
    app.get("/api/shares/:code", fetch);
    app.get("/api/shares/:code/info", fetch);
    app.delete("/api/shares/:code", remove);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_made_and_read_back() {
        for _ in 0..200 {
            let code = new_code();
            assert_eq!(normalize(&code).as_deref(), Some(code.as_str()));
        }
        assert_eq!(normalize("7kq4 m9xp").as_deref(), Some("7KQ4-M9XP"));
        assert_eq!(normalize("7KQ4M9XP").as_deref(), Some("7KQ4-M9XP"));
        assert_eq!(normalize("7KQ4-M9X"), None);
        assert_eq!(normalize("0KQ4-M9XP"), None, "0 is not in the alphabet");
        assert_eq!(new_token().len(), 32);
    }

    #[test]
    fn names_and_types_are_cleaned() {
        assert_eq!(clean_name("..%2F..%2Fetc%2Fpasswd"), "passwd");
        assert_eq!(
            clean_name("vacances%20%C3%A9t%C3%A9.jpg"),
            "vacances été.jpg"
        );
        assert_eq!(clean_name(""), "fichier");
        assert_eq!(clean_name(".env"), "env");
        assert_eq!(clean_name("a\u{0}b\"c.txt"), "abc.txt");
        assert_eq!(clean_mime(Some("image/jpeg")), "image/jpeg");
        assert_eq!(clean_mime(Some("text/html; charset=utf-8")), "text/html");
        assert_eq!(clean_mime(Some("nope")), "application/octet-stream");
        assert_eq!(clean_mime(None), "application/octet-stream");
        assert_eq!(percent_encode("été 1.jpg"), "%C3%A9t%C3%A9%201.jpg");
        assert_eq!(percent_decode(&percent_encode("€ ü/x")), "€ ü/x");
    }
}
