//! The LOCALINK website, written in Rust with Paquets: the texts, routes and islands
//! shared by the server and the browser, the server (native) and the browser bundle
//! (wasm32: islands, 2D canvas and page motion).

pub mod i18n;
pub mod icons;
pub mod islands;
pub mod routes;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#[cfg(target_arch = "wasm32")]
mod client;
