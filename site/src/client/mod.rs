//! The browser bundle: hydrates the islands, draws the 2D network of the hero and starts
//! the page motion.

pub mod canvas;
pub mod install;
pub mod motion;
pub mod share;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // Links of a site published under a sub-path (GitHub Pages: `/LOCALINK`).
    if let Some(base) = document()
        .query_selector("[data-base]")
        .ok()
        .flatten()
        .and_then(|el| el.get_attribute("data-base"))
    {
        crate::routes::set_base(&base);
    }
    if motion::auto_language() {
        return;
    }
    paquets::hydrate(&crate::islands::ISLANDS);
    motion::init();
    canvas::init();
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

pub fn document() -> web_sys::Document {
    window().document().expect("document")
}

pub fn reduced_motion() -> bool {
    matches("(prefers-reduced-motion: reduce)")
}

pub fn matches(query: &str) -> bool {
    window()
        .match_media(query)
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

/// Adds an event listener for the page's lifetime.
pub fn on<E: JsCast + 'static>(
    target: &web_sys::EventTarget,
    event: &str,
    f: impl FnMut(E) + 'static,
) {
    let mut f = f;
    let closure =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| f(e.unchecked_into()));
    let _ = target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Like [`on`], with `{ passive: true }` (scroll and pointer listeners).
pub fn on_passive<E: JsCast + 'static>(
    target: &web_sys::EventTarget,
    event: &str,
    f: impl FnMut(E) + 'static,
) {
    let mut f = f;
    let closure =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| f(e.unchecked_into()));
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(true);
    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        event,
        closure.as_ref().unchecked_ref(),
        &options,
    );
    closure.forget();
}

type FrameCallback = Closure<dyn FnMut(f64)>;

/// Calls `f(timestamp)` on every animation frame while it returns `true`.
pub fn request_frame(f: impl FnMut(f64) -> bool + 'static) {
    let f = Rc::new(RefCell::new(f));
    let slot: Rc<RefCell<Option<FrameCallback>>> = Rc::new(RefCell::new(None));
    let again = slot.clone();
    *slot.borrow_mut() = Some(Closure::new(move |now: f64| {
        if (f.borrow_mut())(now) {
            if let Some(cb) = again.borrow().as_ref() {
                let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
            }
        } else {
            again.borrow_mut().take();
        }
    }));
    if let Some(cb) = slot.borrow().as_ref() {
        let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
    }
}

/// Every element matching `selector`.
pub fn all(selector: &str) -> Vec<web_sys::HtmlElement> {
    let Ok(list) = document().query_selector_all(selector) else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|i| list.item(i)?.dyn_into().ok())
        .collect()
}

/// Calls `f` once after `ms` milliseconds.
pub fn later(ms: i32, f: impl FnOnce() + 'static) {
    let f = Closure::once_into_js(f);
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(f.unchecked_ref(), ms);
}

/// The progress of the simulator's transfer, from 0 to 100 in about 2.6 s, eased like a
/// real transfer (fast start, a little slower at the end): `f` gets each new value.
pub fn animate_progress(f: impl Fn(u32) + 'static) {
    let duration = if reduced_motion() { 400.0 } else { 2600.0 };
    let mut start: Option<f64> = None;
    let mut last = u32::MAX;
    request_frame(move |now| {
        let t0 = *start.get_or_insert(now);
        let x = ((now - t0) / duration).clamp(0.0, 1.0);
        // easeOutCubic with a slight wobble, like a network.
        let eased = 1.0 - (1.0 - x).powi(3);
        let wobble = (x * 40.0).sin() * 0.012 * (1.0 - x);
        let p = ((eased + wobble).clamp(0.0, 1.0) * 100.0).round() as u32;
        let p = if x >= 1.0 { 100 } else { p.min(99) };
        if p != last {
            last = p;
            f(p);
        }
        x < 1.0
    });
}
