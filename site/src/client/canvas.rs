//! The 2D network of the hero, drawn on a `<canvas>`: devices drift, link up when they
//! come close, and files travel along the links as glowing packets. The pointer joins
//! the network. Paused off screen; a still image for visitors who ask for less motion.

use std::cell::{Cell, RefCell};
use std::f64::consts::TAU;
use std::rc::Rc;

use js_sys::Math::random;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{document, on_passive, reduced_motion, request_frame, window};

struct Node {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    r: f64,
    /// 0 for a plain node, 1 for a "device" (brighter, with a halo).
    device: bool,
    /// Time of the last packet received, for the ripple.
    hit: f64,
}

struct Packet {
    from: usize,
    to: usize,
    t: f64,
    speed: f64,
    hue: bool,
}

struct Net {
    nodes: Vec<Node>,
    packets: Vec<Packet>,
    w: f64,
    h: f64,
    dpr: f64,
    pointer: Option<(f64, f64)>,
    last: f64,
    spawn: f64,
}

const LINK: f64 = 132.0;
const CYAN: &str = "45, 226, 255";
const VIOLET: &str = "124, 92, 255";

impl Net {
    fn new() -> Net {
        Net {
            nodes: Vec::new(),
            packets: Vec::new(),
            w: 0.0,
            h: 0.0,
            dpr: 1.0,
            pointer: None,
            last: 0.0,
            spawn: 0.0,
        }
    }

    fn populate(&mut self) {
        let count = ((self.w * self.h) / 15_000.0).clamp(16.0, 64.0) as usize;
        self.nodes = (0..count)
            .map(|i| Node {
                x: random() * self.w,
                y: random() * self.h,
                vx: (random() - 0.5) * 0.28,
                vy: (random() - 0.5) * 0.28,
                r: 1.2 + random() * 1.8,
                device: i % 7 == 0,
                hit: -10_000.0,
            })
            .collect();
        self.packets.clear();
    }

    fn step(&mut self, now: f64) {
        let dt = if self.last == 0.0 {
            16.0
        } else {
            (now - self.last).min(48.0)
        };
        self.last = now;
        let k = dt / 16.0;
        for n in &mut self.nodes {
            if let Some((px, py)) = self.pointer {
                let (dx, dy) = (px - n.x, py - n.y);
                let d2 = dx * dx + dy * dy;
                if d2 < 180.0 * 180.0 && d2 > 1.0 {
                    let d = d2.sqrt();
                    n.vx += dx / d * 0.012 * k;
                    n.vy += dy / d * 0.012 * k;
                }
            }
            n.vx = (n.vx * 0.995).clamp(-0.6, 0.6);
            n.vy = (n.vy * 0.995).clamp(-0.6, 0.6);
            if n.vx.abs() < 0.04 {
                n.vx += (random() - 0.5) * 0.02;
            }
            if n.vy.abs() < 0.04 {
                n.vy += (random() - 0.5) * 0.02;
            }
            n.x += n.vx * k;
            n.y += n.vy * k;
            if n.x < -10.0 {
                n.x = self.w + 10.0;
            } else if n.x > self.w + 10.0 {
                n.x = -10.0;
            }
            if n.y < -10.0 {
                n.y = self.h + 10.0;
            } else if n.y > self.h + 10.0 {
                n.y = -10.0;
            }
        }
        // A new file leaves a device every ~220 ms, towards a neighbour.
        self.spawn -= dt;
        if self.spawn <= 0.0 && self.packets.len() < 14 && !self.nodes.is_empty() {
            self.spawn = 160.0 + random() * 160.0;
            let from = (random() * self.nodes.len() as f64) as usize % self.nodes.len();
            let a = &self.nodes[from];
            let to = self
                .nodes
                .iter()
                .enumerate()
                .filter(|(i, b)| *i != from && dist(a, b) < LINK)
                .min_by(|x, y| dist(a, x.1).total_cmp(&dist(a, y.1)))
                .map(|(i, _)| i);
            if let Some(to) = to {
                self.packets.push(Packet {
                    from,
                    to,
                    t: 0.0,
                    speed: 0.010 + random() * 0.012,
                    hue: random() > 0.5,
                });
            }
        }
        let nodes = &mut self.nodes;
        self.packets.retain_mut(|p| {
            p.t += p.speed * k;
            if p.t >= 1.0 {
                if let Some(n) = nodes.get_mut(p.to) {
                    n.hit = now;
                }
                return false;
            }
            nodes
                .get(p.from)
                .zip(nodes.get(p.to))
                .is_some_and(|(a, b)| dist(a, b) < LINK * 1.3)
        });
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d, now: f64) {
        let _ = ctx.set_transform(self.dpr, 0.0, 0.0, self.dpr, 0.0, 0.0);
        ctx.clear_rect(0.0, 0.0, self.w, self.h);
        ctx.set_line_width(1.0);
        // Links.
        for (i, a) in self.nodes.iter().enumerate() {
            for b in &self.nodes[i + 1..] {
                let d = dist(a, b);
                if d < LINK {
                    let alpha = (1.0 - d / LINK) * 0.32;
                    ctx.set_stroke_style_str(&format!("rgba({CYAN}, {alpha:.3})"));
                    ctx.begin_path();
                    ctx.move_to(a.x, a.y);
                    ctx.line_to(b.x, b.y);
                    ctx.stroke();
                }
            }
            if let Some((px, py)) = self.pointer {
                let d = ((px - a.x).powi(2) + (py - a.y).powi(2)).sqrt();
                if d < 170.0 {
                    let alpha = (1.0 - d / 170.0) * 0.55;
                    ctx.set_stroke_style_str(&format!("rgba({VIOLET}, {alpha:.3})"));
                    ctx.begin_path();
                    ctx.move_to(a.x, a.y);
                    ctx.line_to(px, py);
                    ctx.stroke();
                }
            }
        }
        // Nodes, with a ripple when a packet arrives.
        for n in &self.nodes {
            let since = now - n.hit;
            if (0.0..700.0).contains(&since) {
                let p = since / 700.0;
                ctx.set_stroke_style_str(&format!("rgba({CYAN}, {:.3})", (1.0 - p) * 0.7));
                ctx.begin_path();
                let _ = ctx.arc(n.x, n.y, n.r + 3.0 + p * 18.0, 0.0, TAU);
                ctx.stroke();
            }
            if n.device {
                ctx.set_fill_style_str(&format!("rgba({VIOLET}, 0.18)"));
                ctx.begin_path();
                let _ = ctx.arc(n.x, n.y, n.r * 4.0, 0.0, TAU);
                ctx.fill();
            }
            let color = if n.device { CYAN } else { "200, 214, 255" };
            ctx.set_fill_style_str(&format!(
                "rgba({color}, {})",
                if n.device { 0.95 } else { 0.6 }
            ));
            ctx.begin_path();
            let _ = ctx.arc(n.x, n.y, n.r, 0.0, TAU);
            ctx.fill();
        }
        // Packets: a bright head and a short tail.
        for p in &self.packets {
            let (Some(a), Some(b)) = (self.nodes.get(p.from), self.nodes.get(p.to)) else {
                continue;
            };
            let color = if p.hue { CYAN } else { VIOLET };
            for (lag, alpha, r) in [(0.0, 1.0, 2.6), (0.05, 0.45, 2.0), (0.1, 0.2, 1.5)] {
                let t = (p.t - lag).max(0.0);
                let x = a.x + (b.x - a.x) * t;
                let y = a.y + (b.y - a.y) * t;
                ctx.set_fill_style_str(&format!("rgba({color}, {alpha})"));
                ctx.begin_path();
                let _ = ctx.arc(x, y, r, 0.0, TAU);
                ctx.fill();
            }
        }
    }
}

fn dist(a: &Node, b: &Node) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Sizes the canvas to its box, at the screen's density (at most 2).
fn fit(canvas: &HtmlCanvasElement, net: &mut Net) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let (w, h) = (rect.width(), rect.height());
    if w < 1.0 || h < 1.0 {
        return false;
    }
    let dpr = window().device_pixel_ratio().clamp(1.0, 2.0);
    let changed = (w - net.w).abs() > 1.0 || (h - net.h).abs() > 40.0 || dpr != net.dpr;
    if changed {
        canvas.set_width((w * dpr) as u32);
        canvas.set_height((h * dpr) as u32);
        let first = net.nodes.is_empty();
        let (sx, sy) = (w / net.w.max(1.0), h / net.h.max(1.0));
        net.w = w;
        net.h = h;
        net.dpr = dpr;
        if first {
            net.populate();
        } else {
            for n in &mut net.nodes {
                n.x *= sx;
                n.y *= sy;
            }
        }
    }
    changed
}

pub fn init() {
    let Some(canvas) = document()
        .query_selector("canvas[data-network]")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into::<HtmlCanvasElement>().ok())
    else {
        return;
    };
    let Some(ctx) = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
    else {
        return;
    };
    let net = Rc::new(RefCell::new(Net::new()));
    fit(&canvas, &mut net.borrow_mut());
    let _ = canvas.class_list().add_1("is-ready");

    if reduced_motion() {
        let mut n = net.borrow_mut();
        n.step(16.0);
        n.draw(&ctx, 0.0);
        return;
    }

    // Visible on screen and in a visible tab.
    let visible = Rc::new(Cell::new(true));
    let running = Rc::new(Cell::new(false));

    let start: Rc<dyn Fn()> = {
        let (net, ctx, visible, running) =
            (net.clone(), ctx.clone(), visible.clone(), running.clone());
        Rc::new(move || {
            if running.get() {
                return;
            }
            running.set(true);
            net.borrow_mut().last = 0.0;
            let (net, ctx, visible, running) =
                (net.clone(), ctx.clone(), visible.clone(), running.clone());
            request_frame(move |now| {
                if !visible.get() || document().hidden() {
                    running.set(false);
                    return false;
                }
                let mut n = net.borrow_mut();
                n.step(now);
                n.draw(&ctx, now);
                true
            });
        })
    };

    {
        let (start, visible) = (start.clone(), visible.clone());
        let callback = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
            for entry in entries.iter() {
                if let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() {
                    visible.set(entry.is_intersecting());
                    if entry.is_intersecting() {
                        start();
                    }
                }
            }
        });
        if let Ok(io) = web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref()) {
            io.observe(&canvas);
        }
        callback.forget();
    }
    {
        let start = start.clone();
        let doc: web_sys::EventTarget = document().unchecked_into();
        on_passive(&doc, "visibilitychange", move |_e: web_sys::Event| {
            if !document().hidden() {
                start();
            }
        });
    }
    {
        let (net, canvas2) = (net.clone(), canvas.clone());
        let win: web_sys::EventTarget = window().unchecked_into();
        on_passive(&win, "resize", move |_e: web_sys::Event| {
            fit(&canvas2, &mut net.borrow_mut());
        });
    }
    {
        let (net, canvas2) = (net.clone(), canvas.clone());
        let win: web_sys::EventTarget = window().unchecked_into();
        on_passive(&win, "pointermove", move |e: web_sys::PointerEvent| {
            let rect = canvas2.get_bounding_client_rect();
            let (x, y) = (
                f64::from(e.client_x()) - rect.left(),
                f64::from(e.client_y()) - rect.top(),
            );
            let inside = x >= 0.0 && y >= 0.0 && x <= rect.width() && y <= rect.height();
            net.borrow_mut().pointer = inside.then_some((x, y));
        });
    }
    {
        let net = net.clone();
        let doc: web_sys::EventTarget = document().unchecked_into();
        on_passive(&doc, "pointerleave", move |_e: web_sys::Event| {
            net.borrow_mut().pointer = None;
        });
    }
    start();
}
