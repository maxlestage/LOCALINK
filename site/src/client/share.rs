//! The ephemeral sharing island in the browser: its countdown (2 minutes, sped up ten
//! times), started when the island comes on screen, and the clipboard.

use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use super::{reduced_motion, request_frame, window};
use crate::islands::SHARE_SECONDS;

/// How long the demo takes to show the 2 minutes, in milliseconds.
const DEMO_MS: f64 = 12_000.0;

/// Calls `f(seconds left)` each time the displayed second changes, from 120 down to 0,
/// while it returns `true`.
pub fn countdown(f: impl Fn(u32) -> bool + 'static) {
    let duration = if reduced_motion() {
        DEMO_MS * 2.0
    } else {
        DEMO_MS
    };
    let mut start = None;
    let mut last = SHARE_SECONDS;
    request_frame(move |now| {
        let t0 = *start.get_or_insert(now);
        let x = ((now - t0) / duration).clamp(0.0, 1.0);
        let left = (f64::from(SHARE_SECONDS) * (1.0 - x)).ceil() as u32;
        if left != last {
            last = left;
            if !f(left) {
                return false;
            }
        }
        x < 1.0
    });
}

/// Calls `f` once, the first time `el` is on screen.
pub fn when_visible(el: &web_sys::Element, f: impl Fn() + 'static) {
    let f = Rc::new(f);
    let callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
        move |entries: js_sys::Array, io: web_sys::IntersectionObserver| {
            let visible = entries.iter().any(|e| {
                e.dyn_into::<web_sys::IntersectionObserverEntry>()
                    .is_ok_and(|e| e.is_intersecting())
            });
            if visible {
                io.disconnect();
                f();
            }
        },
    );
    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.35));
    if let Ok(io) =
        web_sys::IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
    {
        io.observe(el);
    }
    callback.forget();
}

/// Puts `text` on the clipboard (ignored where the browser refuses).
pub fn copy(text: &str) {
    let _ = window().navigator().clipboard().write_text(text);
}
