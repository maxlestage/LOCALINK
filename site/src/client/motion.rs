//! Page motion that CSS cannot do alone: scroll reveals, progress bars, the navigation
//! that hides, parallax, counters, spotlight cards, 3D tilt, magnetic buttons, the
//! cursor, scrambled links, the menu, and the language of the root page. Everything
//! degrades to a static page.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};

use super::{all, document, later, matches, on, on_passive, reduced_motion, request_frame, window};
use crate::i18n::Lang;
use crate::routes::base;

/// On the root page of a static host (`data-auto-lang`): goes to the visitor's language.
/// `true` when the page is being replaced.
pub fn auto_language() -> bool {
    let Some(el) = document().query_selector("[data-auto-lang]").ok().flatten() else {
        return false;
    };
    let target = el.get_attribute("data-auto-lang").unwrap_or_default();
    let location = window().location();
    let path = location.pathname().unwrap_or_default();
    let rest = path.strip_prefix(base()).unwrap_or(&path);
    let from_path = rest
        .split('/')
        .find(|p| !p.is_empty())
        .and_then(Lang::parse);
    let from_browser = || {
        let langs = window().navigator().languages();
        langs.iter().find_map(|l| {
            let l = l.as_string()?.to_ascii_lowercase();
            Lang::parse(l.get(..2)?)
        })
    };
    let lang = from_path.or_else(from_browser).unwrap_or(Lang::Fr);
    let url = format!(
        "{}/{}/{target}{}{}",
        base(),
        lang.code(),
        location.search().unwrap_or_default(),
        location.hash().unwrap_or_default()
    );
    location.replace(&url).is_ok()
}

pub fn init() {
    let root: HtmlElement = document().document_element().unwrap().unchecked_into();
    let _ = root.class_list().add_1("js");
    reveals();
    // Only now may the CSS hide what is not revealed yet: what is already on screen was
    // marked just above, so nothing blinks.
    let _ = root.class_list().add_1("hydrated");
    scroll_state(&root);
    counters();
    menu(&root);
    if matches("(hover: hover) and (pointer: fine)") && !reduced_motion() {
        let _ = root.class_list().add_1("has-cursor");
        cursor();
        magnetic();
        tilt();
        spotlight();
        scramble();
    }
}

fn observe(
    selector: &str,
    options: &IntersectionObserverInit,
    f: impl Fn(&IntersectionObserverEntry, &IntersectionObserver) + 'static,
) {
    let callback = Closure::<dyn Fn(js_sys::Array, IntersectionObserver)>::new(
        move |entries: js_sys::Array, io: IntersectionObserver| {
            for entry in entries.iter() {
                if let Ok(entry) = entry.dyn_into::<IntersectionObserverEntry>() {
                    f(&entry, &io);
                }
            }
        },
    );
    if let Ok(io) =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), options)
    {
        for el in all(selector) {
            io.observe(&el);
        }
    }
    callback.forget();
}

/// `[data-reveal]` gets `is-in` when it enters the screen (once).
fn reveals() {
    let height = window()
        .inner_height()
        .ok()
        .and_then(|h| h.as_f64())
        .unwrap_or(800.0);
    for el in all("[data-reveal]") {
        let rect = el.get_bounding_client_rect();
        if rect.top() < height * 0.92 && rect.bottom() > 0.0 {
            let _ = el.class_list().add_1("is-in");
        }
    }
    let options = IntersectionObserverInit::new();
    options.set_root_margin("0px 0px -8% 0px");
    options.set_threshold(&JsValue::from_f64(0.12));
    observe("[data-reveal]", &options, |entry, io| {
        if entry.is_intersecting() {
            let target = entry.target();
            let _ = target.class_list().add_1("is-in");
            io.unobserve(&target);
        }
    });
}

/// Scroll-driven state, once per frame: the progress bar, the navigation, the
/// back-to-top ring, the steps line and the parallax layers.
fn scroll_state(root: &HtmlElement) {
    let nav = document().query_selector(".nav").ok().flatten();
    let steps: Vec<HtmlElement> = all("[data-steps]");
    let layers: Vec<(HtmlElement, f64)> = all("[data-parallax]")
        .into_iter()
        .filter_map(|el| {
            let factor = el.get_attribute("data-parallax")?.parse().ok()?;
            Some((el, factor))
        })
        .collect();
    let still = reduced_motion();
    let last_y = Rc::new(Cell::new(0.0_f64));
    let pending = Rc::new(Cell::new(false));
    let root = root.clone();
    let update = Rc::new(move || {
        let win = window();
        let y = win.scroll_y().unwrap_or(0.0);
        let height = win
            .inner_height()
            .ok()
            .and_then(|h| h.as_f64())
            .unwrap_or(800.0);
        let total = (f64::from(root.scroll_height()) - height).max(1.0);
        let progress = (y / total).clamp(0.0, 1.0);
        let style = root.style();
        let _ = style.set_property("--scroll", &format!("{progress:.4}"));
        if let Some(nav) = &nav {
            let list = nav.class_list();
            let _ = list.toggle_with_force("nav--scrolled", y > 24.0);
            let down = y > last_y.get() + 4.0 && y > 240.0;
            let up = y < last_y.get() - 4.0;
            let menu_open = root.class_list().contains("menu-open");
            if down && !menu_open {
                let _ = list.add_1("nav--hidden");
            } else if up || y < 240.0 {
                let _ = list.remove_1("nav--hidden");
            }
        }
        last_y.set(y);
        for el in &steps {
            let rect = el.get_bounding_client_rect();
            let p = ((height * 0.7 - rect.top()) / rect.height().max(1.0)).clamp(0.0, 1.0);
            let _ = el.style().set_property("--p", &format!("{p:.4}"));
        }
        if !still {
            for (el, factor) in &layers {
                let _ = el
                    .style()
                    .set_property("--py", &format!("{:.1}px", y * factor));
            }
        }
    });
    update();
    let win: web_sys::EventTarget = window().unchecked_into();
    for event in ["scroll", "resize"] {
        let (update, pending) = (update.clone(), pending.clone());
        on_passive(&win, event, move |_e: web_sys::Event| {
            if pending.replace(true) {
                return;
            }
            let (update, pending) = (update.clone(), pending.clone());
            request_frame(move |_| {
                pending.set(false);
                update();
                false
            });
        });
    }
}

/// `[data-count]` counts from `data-from` to its value when it appears.
fn counters() {
    let options = IntersectionObserverInit::new();
    options.set_threshold(&JsValue::from_f64(0.6));
    observe("[data-count]", &options, |entry, io| {
        if !entry.is_intersecting() {
            return;
        }
        let target = entry.target();
        io.unobserve(&target);
        let Ok(el) = target.dyn_into::<HtmlElement>() else {
            return;
        };
        let to: f64 = el
            .get_attribute("data-count")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        let from: f64 = el
            .get_attribute("data-from")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        if reduced_motion() {
            el.set_text_content(Some(&format!("{to}")));
            return;
        }
        let duration = 1400.0;
        let mut start = None;
        request_frame(move |now| {
            let t0 = *start.get_or_insert(now);
            let x = ((now - t0) / duration).clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - x).powi(4);
            let value = from + (to - from) * eased;
            el.set_text_content(Some(&format!("{}", value.round() as i64)));
            if x >= 1.0 {
                let _ = el.class_list().add_1("is-done");
            }
            x < 1.0
        });
    });
}

/// The menu of small screens (a `<details>`): closes after a choice, locks the page
/// behind it while open, and closes with Escape.
fn menu(root: &HtmlElement) {
    let Some(menu) = document()
        .query_selector("details.menu")
        .ok()
        .flatten()
        .and_then(|m| m.dyn_into::<web_sys::HtmlDetailsElement>().ok())
    else {
        return;
    };
    {
        let (root, menu2) = (root.clone(), menu.clone());
        on(&menu, "toggle", move |_e: web_sys::Event| {
            let _ = root
                .class_list()
                .toggle_with_force("menu-open", menu2.open());
        });
    }
    for link in all(".menu__link, .menu__langs a") {
        let menu = menu.clone();
        on(&link, "click", move |_e: web_sys::Event| {
            menu.set_open(false)
        });
    }
    let doc: web_sys::EventTarget = document().unchecked_into();
    on(&doc, "keydown", move |e: web_sys::KeyboardEvent| {
        if e.key() == "Escape" && menu.open() {
            menu.set_open(false);
        }
    });
}

/// A cursor that follows the pointer with some delay, and grows over what can be
/// clicked.
fn cursor() {
    let Some(cursor) = document()
        .query_selector(".cursor")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let target = Rc::new(Cell::new((-100.0_f64, -100.0_f64)));
    let current = Rc::new(Cell::new((-100.0_f64, -100.0_f64)));
    let win: web_sys::EventTarget = window().unchecked_into();
    {
        let (target, cursor) = (target.clone(), cursor.clone());
        on_passive(&win, "pointermove", move |e: web_sys::PointerEvent| {
            target.set((f64::from(e.client_x()), f64::from(e.client_y())));
            let hovering = e
                .target()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .and_then(|el| {
                    el.closest("a, button, summary, [data-tilt], .chip")
                        .ok()
                        .flatten()
                })
                .is_some();
            let _ = cursor.class_list().toggle_with_force("is-hover", hovering);
            let _ = cursor.class_list().add_1("is-visible");
        });
    }
    {
        let cursor = cursor.clone();
        let doc: web_sys::EventTarget = document().unchecked_into();
        on_passive(&doc, "pointerleave", move |_e: web_sys::Event| {
            let _ = cursor.class_list().remove_1("is-visible");
        });
    }
    {
        let cursor = cursor.clone();
        on_passive(&win, "pointerdown", move |_e: web_sys::Event| {
            let _ = cursor.class_list().add_1("is-down");
        });
    }
    {
        let cursor = cursor.clone();
        on_passive(&win, "pointerup", move |_e: web_sys::Event| {
            let _ = cursor.class_list().remove_1("is-down");
        });
    }
    request_frame(move |_| {
        let (tx, ty) = target.get();
        let (cx, cy) = current.get();
        let (nx, ny) = (cx + (tx - cx) * 0.2, cy + (ty - cy) * 0.2);
        current.set((nx, ny));
        let style = cursor.style();
        let _ = style.set_property("--cx", &format!("{nx:.1}px"));
        let _ = style.set_property("--cy", &format!("{ny:.1}px"));
        let _ = style.set_property("--tx", &format!("{tx:.1}px"));
        let _ = style.set_property("--ty", &format!("{ty:.1}px"));
        true
    });
}

/// `[data-magnetic]` leans towards the pointer.
fn magnetic() {
    for el in all("[data-magnetic]") {
        {
            let el2 = el.clone();
            on_passive(&el, "pointermove", move |e: web_sys::PointerEvent| {
                let rect = el2.get_bounding_client_rect();
                let x = f64::from(e.client_x()) - rect.left() - rect.width() / 2.0;
                let y = f64::from(e.client_y()) - rect.top() - rect.height() / 2.0;
                let style = el2.style();
                let _ = style.set_property("--mx", &format!("{:.1}px", x * 0.28));
                let _ = style.set_property("--my", &format!("{:.1}px", y * 0.38));
            });
        }
        let el2 = el.clone();
        on_passive(&el, "pointerleave", move |_e: web_sys::Event| {
            let style = el2.style();
            let _ = style.set_property("--mx", "0px");
            let _ = style.set_property("--my", "0px");
        });
    }
}

/// `[data-tilt]` turns in 3D under the pointer; the hero stage follows it gently.
fn tilt() {
    for el in all("[data-tilt], [data-tilt-stage]") {
        let strength = if el.has_attribute("data-tilt-stage") {
            5.0
        } else {
            8.0
        };
        {
            let el2 = el.clone();
            on_passive(&el, "pointermove", move |e: web_sys::PointerEvent| {
                let rect = el2.get_bounding_client_rect();
                let x = (f64::from(e.client_x()) - rect.left()) / rect.width() - 0.5;
                let y = (f64::from(e.client_y()) - rect.top()) / rect.height() - 0.5;
                let style = el2.style();
                let _ = style.set_property("--rx", &format!("{:.2}deg", -y * strength));
                let _ = style.set_property("--ry", &format!("{:.2}deg", x * strength));
                let _ = el2.class_list().add_1("is-tilting");
            });
        }
        let el2 = el.clone();
        on_passive(&el, "pointerleave", move |_e: web_sys::Event| {
            let style = el2.style();
            let _ = style.set_property("--rx", "0deg");
            let _ = style.set_property("--ry", "0deg");
            let _ = el2.class_list().remove_1("is-tilting");
        });
    }
}

/// `[data-spotlight]`: a light that follows the pointer inside the card.
fn spotlight() {
    for el in all("[data-spotlight]") {
        let el2 = el.clone();
        on_passive(&el, "pointermove", move |e: web_sys::PointerEvent| {
            let rect = el2.get_bounding_client_rect();
            let style = el2.style();
            let _ = style.set_property(
                "--sx",
                &format!("{:.0}px", f64::from(e.client_x()) - rect.left()),
            );
            let _ = style.set_property(
                "--sy",
                &format!("{:.0}px", f64::from(e.client_y()) - rect.top()),
            );
        });
    }
}

/// `[data-scramble]`: on hover, the letters shuffle before settling, left to right.
fn scramble() {
    const GLYPHS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789#%&*+";
    for el in all("[data-scramble]") {
        let original = el.text_content().unwrap_or_default();
        let busy = Rc::new(Cell::new(false));
        let el2 = el.clone();
        on_passive(&el, "pointerenter", move |_e: web_sys::Event| {
            if busy.replace(true) {
                return;
            }
            let chars: Vec<char> = original.chars().collect();
            let (el, busy, original) = (el2.clone(), busy.clone(), original.clone());
            let mut frame = 0usize;
            request_frame(move |_| {
                frame += 1;
                let settled = frame / 2;
                let text: String = chars
                    .iter()
                    .enumerate()
                    .map(|(i, &c)| {
                        if i < settled || c == ' ' {
                            c
                        } else {
                            let r = (js_sys::Math::random() * GLYPHS.len() as f64) as usize;
                            char::from(GLYPHS[r % GLYPHS.len()])
                        }
                    })
                    .collect();
                if settled >= chars.len() {
                    el.set_text_content(Some(&original));
                    let busy = busy.clone();
                    later(120, move || busy.set(false));
                    false
                } else {
                    el.set_text_content(Some(&text));
                    true
                }
            });
        });
    }
}
