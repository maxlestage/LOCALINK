//! `cargo run`: the website on `http://localhost:3000` (`PORT` on Heroku). Run it from
//! `site/`, after building the browser bundle (see `README.md`).
//!
//! `cargo run -- minify public/site.css`: minifies the stylesheet in place, before a
//! deployment.
//!
//! `cargo run -- export dist`: the whole site as static files in `dist/`, for GitHub
//! Pages. `BASE_PATH` (e.g. `/LOCALINK`) prefixes every link, `SITE_URL` (e.g.
//! `https://maxlestage.github.io/LOCALINK`) makes the absolute URLs.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("export") => {
            let dir = args.next().unwrap_or_else(|| "dist".into());
            let base = std::env::var("BASE_PATH").unwrap_or_default();
            let site_url = std::env::var("SITE_URL").ok();
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?;
            let files = runtime.block_on(localink_site::server::export::export(
                std::path::Path::new(&dir),
                &base,
                site_url,
            ))?;
            println!("LOCALINK: {files} files written to {dir}");
            Ok(())
        }
        Some("minify") => {
            let file = args.next().unwrap_or_else(|| "public/site.css".into());
            let (before, after) =
                localink_site::server::minify::minify_file(std::path::Path::new(&file))?;
            println!("LOCALINK: {file} minified, {before} → {after} bytes");
            Ok(())
        }
        Some(other) => {
            eprintln!(
                "unknown command {other:?}: run without arguments, `export <dir>` or `minify <file.css>`"
            );
            std::process::exit(2);
        }
        None => localink_site::server::run(),
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
