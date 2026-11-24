//! The interactive parts of the site. Each island renders on the server like any view
//! and, compiled to WebAssembly, hydrates in the browser from the same code.

use paquets::prelude::*;

use crate::i18n::Lang;
use crate::icons::{icon, kind_icon};

fn lang(props: &str) -> Lang {
    Lang::parse(props).unwrap_or(Lang::Fr)
}

/// Every island, registered under the name used by [`island`].
pub const ISLANDS: [(&str, paquets::active::IslandFn); 3] = [
    ("simulator", simulator),
    ("share", share),
    ("install", install),
];

// ----- Transfer simulator ----------------------------------------------------------------

/// What the simulator is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Idle,
    Sending,
    Done,
}

/// The demo files: name and size (in tenths of a megabyte), one per kind.
pub fn demo_file(kind: usize, lang: Lang) -> (&'static str, u32) {
    match kind % 4 {
        0 => ("IMG_2048.HEIC", 48),
        1 => ("VID_0712.MOV", 864),
        2 => (lang.texts().files[2], 12),
        _ => ("playlist.m4a", 74),
    }
}

/// `4,8 Mo`, `4.8 MB`: a size in tenths of a megabyte, written for `lang`.
pub fn size(tenths: u32, lang: Lang) -> String {
    let (sep, unit) = match lang {
        Lang::Fr => (',', "Mo"),
        Lang::En => ('.', "MB"),
        Lang::Es => (',', "MB"),
    };
    format!("{}{sep}{} {unit}", tenths / 10, tenths % 10)
}

/// Two phones on the same network: choose a kind of file, send it, watch it go.
pub fn simulator(props: &str) -> Node {
    let lang = lang(props);
    let t = lang.texts();
    let kind = use_state(0usize);
    let phase = use_state(Phase::Idle);
    let progress = use_state(0u32);

    let start = move || match phase.get() {
        Phase::Sending => {}
        Phase::Done => {
            phase.set(Phase::Idle);
            progress.set(0);
        }
        Phase::Idle => {
            phase.set(Phase::Sending);
            progress.set(0);
            #[cfg(target_arch = "wasm32")]
            crate::client::animate_progress(move |p| {
                progress.set(p);
                if p >= 100 {
                    phase.set(Phase::Done);
                }
            });
        }
    };

    let file_tile = move |receiver: bool| {
        div()
            .class("sim__file")
            .class(if receiver {
                "sim__file--in"
            } else {
                "sim__file--out"
            })
            .child(span().class("sim__file-icon").child(fragment_dyn(move || {
                vec![icon(kind_icon(kind.get())).into()]
            })))
            .child(
                span()
                    .class("sim__file-meta")
                    .child(strong().text_dyn(move || demo_file(kind.get(), lang).0.to_owned()))
                    .child(small().text_dyn(move || size(demo_file(kind.get(), lang).1, lang))),
            )
    };

    div()
        .class("sim")
        .class_if("is-sending", move || phase.get() == Phase::Sending)
        .class_if("is-done", move || phase.get() == Phase::Done)
        .attr_dyn("style", move || format!("--p:{}", progress.get()))
        .child(
            div()
                .class("sim__stage")
                .child(
                    div()
                        .class("sim__device sim__device--from")
                        .child(span().class("sim__avatar").child(icon("phone")))
                        .child(strong().class("sim__name").text(t.sim_from))
                        .child(file_tile(false)),
                )
                .child(
                    div()
                        .class("sim__link")
                        .attr("aria-hidden", "true")
                        .child(span().class("sim__wifi").child(icon("wifi")))
                        .child(small().class("sim__network").text(t.sim_network))
                        .child(div().class("sim__wire").children(
                            (0..5).map(|i| span().class("sim__packet").style(format!("--i:{i}"))),
                        )),
                )
                .child(
                    div()
                        .class("sim__device sim__device--to")
                        .child(span().class("sim__avatar").child(icon("phone")))
                        .child(strong().class("sim__name").text(t.sim_to))
                        .child(file_tile(true))
                        .child(span().class("sim__check").child(icon("check"))),
                ),
        )
        .child(
            div()
                .class("sim__bar")
                .attr("role", "progressbar")
                .attr("aria-valuemin", "0")
                .attr("aria-valuemax", "100")
                .attr_dyn("aria-valuenow", move || progress.get().to_string())
                .child(span().class("sim__fill")),
        )
        .child(
            p().class("sim__status")
                .attr("aria-live", "polite")
                .text_dyn(move || match phase.get() {
                    Phase::Idle => t.sim_pick.to_owned(),
                    Phase::Sending => format!("{} {} %", t.sim_sending, progress.get()),
                    Phase::Done => t.sim_done.to_owned(),
                }),
        )
        .child(
            div()
                .class("sim__kinds")
                .attr("role", "radiogroup")
                .attr("aria-label", t.sim_pick)
                .children(t.sim_kinds.iter().enumerate().map(|(i, label)| {
                    button()
                        .class("chip")
                        .class_if("is-active", move || kind.get() == i)
                        .attr("type", "button")
                        .attr("role", "radio")
                        .attr_dyn("aria-checked", move || (kind.get() == i).to_string())
                        .bool_attr("disabled", move || phase.get() == Phase::Sending)
                        .on_click(move |_| {
                            kind.set(i);
                            if phase.get() == Phase::Done {
                                phase.set(Phase::Idle);
                                progress.set(0);
                            }
                        })
                        .child(icon(kind_icon(i)))
                        .child(span().text(*label))
                })),
        )
        .child(
            button()
                .class("btn btn--primary sim__send")
                .attr("type", "button")
                .attr("data-magnetic", "")
                .bool_attr("disabled", move || phase.get() == Phase::Sending)
                .on_click(move |_| start())
                .child(span().class("btn__label").text_dyn(move || {
                    match phase.get() {
                        Phase::Idle => t.sim_send,
                        Phase::Sending => t.sim_sending,
                        Phase::Done => t.sim_again,
                    }
                    .to_owned()
                }))
                .child(icon("arrow")),
        )
        .child(p().class("sim__note").text(t.sim_note))
        .into()
}

// ----- Ephemeral sharing ----------------------------------------------------------------

/// How long a share stays on the server, in seconds.
pub const SHARE_SECONDS: u32 = 120;
/// The code rendered by the server; the browser draws a new one once the island starts.
pub const FIRST_CODE: &str = "7KQ4-M9XP";
/// The characters of a code: no 0/O, 1/I/L, which are easy to mix up when typed.
pub const ALPHABET: &[u8] = b"23456789ABCDEFGHJKMNPQRSTUVWXYZ";

/// A LOCALINK code, `XXXX-XXXX`, from a source of numbers in `[0, 1)`.
pub fn make_code(mut random: impl FnMut() -> f64) -> String {
    let mut code = String::with_capacity(9);
    for i in 0..8 {
        if i == 4 {
            code.push('-');
        }
        let index = (random() * ALPHABET.len() as f64) as usize % ALPHABET.len();
        code.push(char::from(ALPHABET[index]));
    }
    code
}

/// `2:00`, `0:07`.
pub fn clock(seconds: u32) -> String {
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// The QR code of a LOCALINK code.
pub fn code_qr(code: &str, label: &str) -> String {
    crate::qr::svg(&format!("LOCALINK:{code}"), "qr qr--code", label).unwrap_or_default()
}

/// A code and its QR code, a shared file whose 2 minutes run out (sped up), then
/// vanish from the server.
pub fn share(props: &str) -> Node {
    let lang = lang(props);
    let t = lang.texts();
    let code = use_state(FIRST_CODE.to_owned());
    let left = use_state(SHARE_SECONDS);
    let copied = use_state(false);
    // How the contact connects: 0, the code; 1, the QR code.
    let method = use_state(0usize);
    // The current countdown: a new share stops the previous one.
    let generation = use_state(0u32);
    // Only the browser runs countdowns.
    #[cfg(not(target_arch = "wasm32"))]
    let _ = generation;

    let restart = move || {
        #[cfg(target_arch = "wasm32")]
        {
            code.set(make_code(js_sys::Math::random));
            left.set(SHARE_SECONDS);
            let current = generation.get() + 1;
            generation.set(current);
            crate::client::share::countdown(move |seconds| {
                if generation.get() != current {
                    return false;
                }
                left.set(seconds);
                true
            });
        }
    };

    div()
        .class("share")
        .class_if("is-erased", move || left.get() == 0)
        .class_if("is-ending", move || (1..=20).contains(&left.get()))
        .attr_dyn("style", move || {
            format!(
                "--left:{:.4}",
                f64::from(left.get()) / f64::from(SHARE_SECONDS)
            )
        })
        .on_mount(move |_root| {
            #[cfg(target_arch = "wasm32")]
            crate::client::share::when_visible(_root, restart);
        })
        .child(
            div()
                .class("share__card")
                .child(p().class("share__label").text(t.share_code_label))
                .child(
                    div()
                        .class("share__switch")
                        .attr("role", "tablist")
                        .attr("aria-label", t.share_choose)
                        .attr_dyn("style", move || format!("--idx:{}", method.get()))
                        .child(span().class("share__pill").attr("aria-hidden", "true"))
                        .children(
                            [(0usize, t.share_tab_code, "key"), (1, t.share_tab_qr, "qr")].map(
                                |(i, label, icon_name)| {
                                    button()
                                        .class("share__tab")
                                        .class_if("is-active", move || method.get() == i)
                                        .attr("type", "button")
                                        .attr("role", "tab")
                                        .attr_dyn("aria-selected", move || {
                                            (method.get() == i).to_string()
                                        })
                                        .on_click(move |_| method.set(i))
                                        .child(icon(icon_name))
                                        .child(span().text(label))
                                },
                            ),
                        ),
                )
                .child(
                    div()
                        .class("share__view share__view--code")
                        .attr("role", "tabpanel")
                        .bool_attr("hidden", move || method.get() != 0)
                        .child(
                            p().class("share__code")
                                .attr("aria-live", "polite")
                                .text_dyn(move || code.get()),
                        )
                        .child(p().class("share__path").text_dyn(move || {
                            format!("localink · {}", code.get().replace('-', ""))
                        })),
                )
                .child(
                    div()
                        .class("share__view share__view--qr")
                        .attr("role", "tabpanel")
                        .bool_attr("hidden", move || method.get() != 1)
                        .child(
                            div()
                                .class("share__qr")
                                .html_dyn(move || code_qr(&code.get(), t.share_code_label)),
                        ),
                )
                .child(
                    div()
                        .class("share__actions")
                        .child(
                            button()
                                .class("chip share__copy")
                                .attr("type", "button")
                                .on_click(move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        crate::client::share::copy(&code.get());
                                        copied.set(true);
                                        crate::client::later(1600, move || copied.set(false));
                                    }
                                })
                                .child(fragment_dyn(move || {
                                    vec![icon(if copied.get() { "check" } else { "copy" }).into()]
                                }))
                                .child(span().text_dyn(move || {
                                    if copied.get() {
                                        t.share_copied
                                    } else {
                                        t.share_copy
                                    }
                                    .to_owned()
                                })),
                        )
                        .child(
                            button()
                                .class("chip share__new")
                                .attr("type", "button")
                                .on_click(move |_| restart())
                                .child(icon("refresh"))
                                .child(span().text(t.share_new)),
                        ),
                ),
        )
        .child(
            div()
                .class("share__drop")
                .child(
                    div()
                        .class("share__server")
                        .child(icon("server"))
                        .child(strong().text(t.share_server)),
                )
                .child(
                    div()
                        .class("share__file")
                        .child(span().class("share__file-icon").child(icon("image")))
                        .child(
                            span()
                                .class("share__file-meta")
                                .child(strong().text(t.files[0]))
                                .child(small().text(crate::islands::size(48, lang))),
                        )
                        .child(
                            span()
                                .class("share__dust")
                                .attr("aria-hidden", "true")
                                .children((0..8).map(|i| span().style(format!("--i:{i}")))),
                        ),
                )
                .child(
                    div()
                        .class("share__timer")
                        .attr("role", "timer")
                        .child(
                            span()
                                .class("share__ring")
                                .attr("aria-hidden", "true")
                                .child(span().class("share__ring-inner")),
                        )
                        .child(
                            span()
                                .class("share__clock")
                                .child(small().text_dyn(move || {
                                    if left.get() == 0 {
                                        t.share_erased
                                    } else {
                                        t.share_expires
                                    }
                                    .to_owned()
                                }))
                                .child(strong().text_dyn(move || clock(left.get()))),
                        ),
                )
                .child(
                    button()
                        .class("btn btn--ghost share__again")
                        .attr("type", "button")
                        .bool_attr("hidden", move || left.get() != 0)
                        .on_click(move |_| restart())
                        .child(span().class("btn__label").text(t.share_again))
                        .child(icon("refresh")),
                )
                .child(p().class("share__demo").text(t.share_demo)),
        )
        .into()
}

// ----- Install (PWA) ------------------------------------------------------------------

/// The "install the site" button of the footer: the browser's own prompt where it has
/// one, a hint otherwise (Safari: Share → Add to Home Screen).
pub fn install(props: &str) -> Node {
    let lang = lang(props);
    let t = lang.texts();
    let installed = use_state(false);
    let hint = use_state(false);
    div()
        .class("install")
        .class_if("is-installed", move || installed.get())
        .on_mount(move |_root| {
            #[cfg(target_arch = "wasm32")]
            crate::client::install::watch(installed);
        })
        .child(
            button()
                .class("btn btn--ghost install__btn")
                .attr("type", "button")
                .attr("data-magnetic", "")
                .bool_attr("disabled", move || installed.get())
                .on_click(move |_| {
                    #[cfg(target_arch = "wasm32")]
                    if !crate::client::install::prompt() {
                        hint.set(true);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    hint.set(true);
                })
                .child(icon("download"))
                .child(span().text_dyn(move || {
                    if installed.get() {
                        t.footer_installed
                    } else {
                        t.footer_install
                    }
                    .to_owned()
                })),
        )
        .child(
            p().class("install__hint")
                .class_if("is-visible", move || hint.get())
                .text(t.footer_install_hint),
        )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_and_clocks() {
        let mut n = 0.0;
        let code = make_code(|| {
            n += 0.13;
            n % 1.0
        });
        assert_eq!(code.len(), 9);
        assert_eq!(&code[4..5], "-");
        assert!(
            code.bytes()
                .filter(|&b| b != b'-')
                .all(|b| ALPHABET.contains(&b))
        );
        assert_eq!(make_code(|| 0.0), "2222-2222");
        assert_eq!(make_code(|| 0.999_999), "ZZZZ-ZZZZ");
        assert_eq!(clock(120), "2:00");
        assert_eq!(clock(7), "0:07");
        assert_eq!(clock(0), "0:00");
        for c in *b"0O1IL" {
            assert!(!ALPHABET.contains(&c));
        }
    }

    #[test]
    fn sizes_are_localized() {
        assert_eq!(size(48, Lang::Fr), "4,8 Mo");
        assert_eq!(size(864, Lang::En), "86.4 MB");
        assert_eq!(size(12, Lang::Es), "1,2 MB");
    }

    #[test]
    fn islands_render_on_the_server() {
        for (name, island_fn) in ISLANDS {
            for lang in Lang::ALL {
                let html = paquets::active::island(name, lang.code(), island_fn).render();
                assert!(html.contains("<active-island"), "{name}");
            }
        }
        let html = paquets::active::island("share", "fr", share).render();
        assert!(html.contains(FIRST_CODE));
        assert!(html.contains("2:00"));
        assert!(html.contains("qr qr--code"));
        let html = paquets::active::island("simulator", "fr", simulator).render();
        assert!(html.contains("IMG_2048.HEIC"));
        assert!(html.contains("Envoyer"));
    }
}
