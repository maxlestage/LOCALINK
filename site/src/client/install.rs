//! "Install the site": keeps the browser's `beforeinstallprompt` event to show its
//! prompt when the button is pressed, and notices when the site runs installed.

use std::cell::RefCell;

use paquets::prelude::State;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use super::{matches, on, window};

thread_local! {
    /// The deferred `beforeinstallprompt` event, while the browser offers one.
    static DEFERRED: RefCell<Option<web_sys::Event>> = const { RefCell::new(None) };
}

/// Watches the install events; `installed` becomes `true` once the site is installed
/// (or already runs as an installed app).
pub fn watch(installed: State<bool>) {
    if matches("(display-mode: standalone)") || matches("(display-mode: fullscreen)") {
        installed.set(true);
        return;
    }
    let target: web_sys::EventTarget = window().unchecked_into();
    on(&target, "beforeinstallprompt", |e: web_sys::Event| {
        e.prevent_default();
        DEFERRED.with(|d| *d.borrow_mut() = Some(e));
    });
    on(&target, "appinstalled", move |_e: web_sys::Event| {
        installed.set(true);
        DEFERRED.with(|d| d.borrow_mut().take());
    });
}

/// Shows the browser's install prompt; `false` when it has none (Safari, Firefox).
pub fn prompt() -> bool {
    let Some(event) = DEFERRED.with(|d| d.borrow_mut().take()) else {
        return false;
    };
    let Ok(prompt) = js_sys::Reflect::get(&event, &JsValue::from_str("prompt")) else {
        return false;
    };
    let Some(prompt) = prompt.dyn_ref::<js_sys::Function>() else {
        return false;
    };
    prompt.call0(&event).is_ok()
}
