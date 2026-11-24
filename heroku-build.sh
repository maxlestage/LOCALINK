#!/usr/bin/env bash
# Builds the server and the browser bundle (WebAssembly) of the website: run on Heroku
# by RustConfig once the emk/rust buildpack has installed Rust. Works by hand too, from
# the repository: ./heroku-build.sh
# The Procfile starts site/target/release/localink-site.
set -euo pipefail

cd "$(dirname "$0")/site"
# On Heroku, Cargo builds into the buildpack's cache (CARGO_TARGET_DIR).
target="${CARGO_TARGET_DIR:-target}"

echo "-----> Building the server"
cargo build --release --bin localink-site
mkdir -p target/release
if [ "$target" != target ]; then
  cp "$target/release/localink-site" target/release/
fi
echo "       target/release/localink-site ready"

echo "-----> Minifying the stylesheet"
target/release/localink-site minify public/site.css

echo "-----> Building the browser bundle (WebAssembly)"
rustup target add wasm32-unknown-unknown
cargo build --lib --target wasm32-unknown-unknown --profile wasm-release

# wasm-bindgen must be the version of site/Cargo.lock: a prebuilt binary, or compiled if
# it cannot be downloaded.
version=$(grep -A1 '^name = "wasm-bindgen"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')
bin="$target/wasm-bindgen-$version"
if [ ! -x "$bin/wasm-bindgen" ]; then
  mkdir -p "$bin"
  curl -sSfL "https://github.com/wasm-bindgen/wasm-bindgen/releases/download/${version}/wasm-bindgen-${version}-x86_64-unknown-linux-musl.tar.gz" \
    | tar -xz --strip-components=1 -C "$bin" \
    || cargo install wasm-bindgen-cli --version "$version" --locked --root "$bin" --bin wasm-bindgen
  [ -x "$bin/wasm-bindgen" ] || mv "$bin/bin/wasm-bindgen" "$bin/"
fi
"$bin/wasm-bindgen" --target web --no-typescript --remove-name-section --remove-producers-section \
  --out-dir pkg \
  "$target/wasm32-unknown-unknown/wasm-release/localink_site.wasm"
echo "       browser bundle ready in site/pkg"
