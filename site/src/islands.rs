//! The interactive parts of the site. Each island renders on the server like any view
//! and, compiled to WebAssembly, hydrates in the browser from the same code.

use paquets::prelude::*;

use crate::i18n::Lang;
use crate::icons::{icon, kind_icon};

fn lang(props: &str) -> Lang {
    Lang::parse(props).unwrap_or(Lang::Fr)
}

/// Every island, registered under the name used by [`island`].
pub const ISLANDS: [(&str, paquets::active::IslandFn); 2] =
    [("simulator", simulator), ("install", install)];

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
        let html = paquets::active::island("simulator", "fr", simulator).render();
        assert!(html.contains("IMG_2048.HEIC"));
        assert!(html.contains("Envoyer"));
    }
}
