//! Every page of the site, written with active views (through Paquets' prelude).

use paquets::prelude::*;

use super::{Site, qr};
use crate::i18n::{Lang, Legal, Texts};
use crate::icons::{icon, logo};
use crate::routes::{Page as PageKind, Route, base, link};

/// The repository of the project (source code, contact).
pub const REPO: &str = "https://github.com/maxlestage/LOCALINK";
/// The framework the site is written with.
pub const PAQUETS: &str = "https://github.com/maxlestage/Paquets";
/// The author, for the legal notice and the copyright.
pub const AUTHOR: &str = "Maxime Nathan Lestage";
pub const YEAR: &str = "2026";

/// What a request needs to render a page.
pub struct Ctx<'a> {
    pub site: &'a Site,
    pub route: Route,
    /// Absolute origin of the site (with the base path), for canonical and Open Graph
    /// URLs and the QR code, e.g. `https://maxlestage.github.io/LOCALINK`.
    pub origin: String,
}

impl Ctx<'_> {
    fn t(&self) -> &'static Texts {
        self.route.lang.texts()
    }

    fn lang(&self) -> Lang {
        self.route.lang
    }

    /// A section of the home page: `#how` on the home page, `/fr/#how` elsewhere.
    fn anchor(&self, id: &str) -> String {
        if self.route.page == PageKind::Home {
            format!("#{id}")
        } else {
            format!("{}#{id}", Route::new(self.lang(), PageKind::Home).href())
        }
    }

    fn href(&self, page: PageKind) -> String {
        Route::new(self.lang(), page).href()
    }
}

// ----- Small pieces ---------------------------------------------------------------------

/// Words that rise one after the other (`--i`, CSS), from `offset`.
fn words(text: &str, offset: usize) -> Vec<Node> {
    let count = text.split_whitespace().count();
    text.split_whitespace()
        .enumerate()
        .flat_map(|(i, word)| {
            let word = span()
                .class("w")
                .child(
                    span()
                        .class("w__in")
                        .style(format!("--i:{}", i + offset))
                        .text(word.to_owned()),
                )
                .into();
            if i + 1 < count {
                vec![word, text_node(" ")]
            } else {
                vec![word]
            }
        })
        .collect()
}

fn text_node(text: &str) -> Node {
    paquets::active::text(text.to_owned())
}

/// Letters that wave (`--i`, CSS).
fn letters(text: &str, class: &str) -> Element {
    span()
        .class(class.to_owned())
        .attr("aria-label", text.to_owned())
        .children(text.chars().enumerate().map(|(i, c)| {
            span()
                .attr("aria-hidden", "true")
                .style(format!("--i:{i}"))
                .text(c.to_string())
        }))
}

/// A button attracted by the cursor (`data-magnetic`, moved by the browser bundle).
fn button_link(href: &str, class: &str, label: &str, icon_name: &str) -> Element {
    a().class(format!("btn {class}"))
        .href(href.to_owned())
        .attr("data-magnetic", "")
        .child(span().class("btn__label").text(label.to_owned()))
        .child(icon(icon_name))
}

/// The head of a section: number, label, title (words rising) and lead.
fn section_head(num: &str, label: &str, title: &str, lead: Option<&str>) -> Element {
    let mut head = header()
        .class("section-head")
        .child(
            p().class("label")
                .attr("data-reveal", "")
                .child(span().class("label__num").text(num.to_owned()))
                .child(span().class("label__line").attr("aria-hidden", "true"))
                .child(span().text(label.to_owned())),
        )
        .child(
            h2().class("section-title split")
                .attr("data-reveal", "")
                .children(words(title, 0)),
        );
    if let Some(lead) = lead {
        head = head.child(
            p().class("lead")
                .attr("data-reveal", "")
                .style("--d:120ms")
                .text(lead.to_owned()),
        );
    }
    head
}

/// The QR code of the page's own address.
fn qr_code(ctx: &Ctx<'_>, class: &str) -> Node {
    let url = format!("{}{}", ctx.origin, ctx.route.path());
    let svg = qr::svg(&url, class, ctx.t().qr_caption).unwrap_or_default();
    div().class("qr-wrap").html(svg).into()
}

// ----- Layout ---------------------------------------------------------------------------

/// The page around `content`: head tags, navigation, footer, cursor, background.
fn layout(ctx: &Ctx<'_>, title: &str, description: &str, content: Element) -> Page {
    let lang = ctx.lang();
    let route = ctx.route;
    let mut seo = Seo::new()
        .site_name("LOCALINK")
        .base_url(ctx.origin.clone())
        .image("/assets/og-image.png")
        .meta_property("og:image:width", "1200")
        .meta_property("og:image:height", "630")
        .meta_property("og:image:alt", "LOCALINK")
        .og_type("website")
        .locale(lang.locale())
        .meta_name("author", AUTHOR)
        .meta_name("twitter:card", "summary_large_image");
    if route.page == PageKind::NotFound {
        seo = seo.noindex(true);
    } else {
        seo = seo.canonical(route.path());
        for l in Lang::ALL {
            seo = seo.alternate(l.code(), route.with_lang(l).path());
        }
        seo = seo.alternate("x-default", route.with_lang(Lang::Fr).path());
    }
    if route.page == PageKind::Home {
        seo = seo.json_ld(format!(
            r#"{{"@context":"https://schema.org","@type":"SoftwareApplication","name":"LOCALINK","applicationCategory":"UtilitiesApplication","operatingSystem":"iOS","inLanguage":"{}","description":{},"url":"{}{}","author":{{"@type":"Person","name":"{AUTHOR}"}}}}"#,
            lang.code(),
            json_string(ctx.t().meta_description),
            ctx.origin,
            route.path()
        ));
    }
    let b = base();
    let head = format!(
        concat!(
            r#"<meta name="color-scheme" content="dark"/>"#,
            r#"<meta name="format-detection" content="telephone=no"/>"#,
            r#"<link rel="icon" href="{b}/assets/logo.svg" type="image/svg+xml"/>"#,
            r#"<link rel="preload" href="{b}/fonts/space-grotesk.woff2" as="font" type="font/woff2" crossorigin=""/>"#,
            r#"<link rel="preload" href="{b}/fonts/inter.woff2" as="font" type="font/woff2" crossorigin=""/>"#,
        ),
        b = b
    );
    let mut root = div()
        .class("site")
        .attr("data-base", b)
        .attr("data-page", route.page.slug())
        .child(a().class("skip").href("#main").text(ctx.t().skip))
        .child(div().class("progress").attr("aria-hidden", "true"))
        .child(background());
    if route.page == PageKind::Home {
        root = root.child(loader(ctx));
    }
    root = root
        .child(
            div()
                .class("cursor")
                .attr("aria-hidden", "true")
                .child(span().class("cursor__ring"))
                .child(span().class("cursor__dot")),
        )
        .child(navbar(ctx))
        .child(content.id("main"))
        .child(footer_view(ctx));
    Page::new(title)
        .lang(lang.code())
        .description(description)
        .seo(seo)
        .plain()
        .view_transitions()
        .stylesheet(format!("{}?v={}", link("/site.css"), ctx.site.css_version))
        .head(head)
        .body(root)
}

/// A JSON string (for JSON-LD).
fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The animated background: blurred blobs and a grid.
fn background() -> Element {
    div()
        .class("bg")
        .attr("aria-hidden", "true")
        .child(
            span()
                .class("bg__blob bg__blob--a")
                .attr("data-parallax", "0.15"),
        )
        .child(
            span()
                .class("bg__blob bg__blob--b")
                .attr("data-parallax", "-0.1"),
        )
        .child(
            span()
                .class("bg__blob bg__blob--c")
                .attr("data-parallax", "0.06"),
        )
        .child(span().class("bg__grid"))
        .child(span().class("bg__noise"))
}

/// The preloader of the home page: two dots that find each other, then the curtain.
fn loader(ctx: &Ctx<'_>) -> Element {
    div()
        .class("loader")
        .attr("aria-hidden", "true")
        .child(
            div()
                .class("loader__link")
                .child(span().class("loader__dot"))
                .child(span().class("loader__line"))
                .child(span().class("loader__dot loader__dot--b")),
        )
        .child(letters("LOCALINK", "loader__word"))
        .child(p().class("loader__text").text(ctx.t().loader))
}

fn brand(ctx: &Ctx<'_>) -> Element {
    a().class("brand")
        .href(ctx.href(PageKind::Home))
        .attr("aria-label", "LOCALINK")
        .child(logo())
        .child(span().class("brand__name").text("LOCALINK"))
}

/// The links of the home page sections.
fn sections(ctx: &Ctx<'_>) -> [(String, &'static str); 5] {
    let t = ctx.t();
    [
        (ctx.anchor("how"), t.nav_how),
        (ctx.anchor("features"), t.nav_features),
        (ctx.anchor("privacy"), t.nav_privacy),
        (ctx.anchor("app"), t.nav_app),
        (ctx.anchor("faq"), t.nav_faq),
    ]
}

/// The language links of the current page.
fn languages(ctx: &Ctx<'_>, class: &str) -> Element {
    let index = Lang::ALL.iter().position(|&l| l == ctx.lang()).unwrap_or(0);
    nav()
        .class(class.to_owned())
        .attr("aria-label", ctx.t().lang_label)
        .style(format!("--idx:{index}"))
        .child(span().class("langs__pill").attr("aria-hidden", "true"))
        .children(Lang::ALL.iter().map(|&l| {
            let page = if ctx.route.page == PageKind::NotFound {
                PageKind::Home
            } else {
                ctx.route.page
            };
            let mut link = a()
                .class("langs__link")
                .href(Route::new(l, page).href())
                .attr("hreflang", l.code())
                .attr("lang", l.code())
                .attr("title", l.name())
                .text(l.code().to_uppercase());
            if l == ctx.lang() {
                link = link.attr("aria-current", "true");
            }
            link
        }))
}

fn navbar(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    header().class("nav").child(
        div()
            .class("nav__inner")
            .child(brand(ctx))
            .child(
                nav()
                    .class("nav__links")
                    .attr("aria-label", t.nav_menu)
                    .children(sections(ctx).into_iter().map(|(href, label)| {
                        a().class("nav__link")
                            .href(href)
                            .attr("data-scramble", "")
                            .text(label)
                    })),
            )
            .child(languages(ctx, "langs nav__langs"))
            .child(menu(ctx)),
    )
}

/// The menu of small screens: a `<details>`, so it opens without the bundle too.
fn menu(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    details()
        .class("menu")
        .child(
            summary()
                .class("menu__toggle")
                .attr("aria-label", t.nav_menu)
                .child(span().class("menu__bars").attr("aria-hidden", "true"))
                .child(span().class("sr-only").text(t.nav_menu)),
        )
        .child(
            div()
                .class("menu__panel")
                .child(
                    nav()
                        .class("menu__links")
                        .attr("aria-label", t.nav_menu)
                        .children(sections(ctx).into_iter().enumerate().map(
                            |(i, (href, label))| {
                                a().class("menu__link")
                                    .href(href)
                                    .style(format!("--i:{i}"))
                                    .child(span().class("menu__num").text(format!("0{}", i + 1)))
                                    .child(span().text(label))
                            },
                        )),
                )
                .child(languages(ctx, "langs menu__langs")),
        )
}

// ----- Footer ---------------------------------------------------------------------------

fn footer_column(title: &str, links: Vec<Element>) -> Element {
    div()
        .class("footer__col")
        .attr("data-reveal", "")
        .child(h2().class("footer__title").text(title.to_owned()))
        .child(
            ul().class("footer__list")
                .children(links.into_iter().map(|l| li().child(l))),
        )
}

fn footer_link(href: &str, label: &str, icon_name: Option<&str>, external: bool) -> Element {
    let mut link = a().class("footer__link").href(href.to_owned());
    if external {
        link = link.attr("target", "_blank").attr("rel", "noopener");
    }
    if let Some(name) = icon_name {
        link = link.child(icon(name));
    }
    link.child(
        span()
            .class("footer__label")
            .attr("data-text", label.to_owned())
            .text(label.to_owned()),
    )
}

fn footer_view(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    let lang = ctx.lang();
    let product: Vec<Element> = sections(ctx)
        .into_iter()
        .map(|(href, label)| footer_link(&href, label, None, false))
        .chain([footer_link(&ctx.anchor("demo"), t.sim_label, None, false)])
        .collect();
    let resources = vec![
        footer_link(REPO, t.footer_source, Some("github"), true),
        footer_link(PAQUETS, t.footer_paquets, Some("box"), true),
        footer_link(
            &link("/assets/logo.svg"),
            t.footer_press,
            Some("download"),
            false,
        )
        .attr("download", "localink-logo.svg"),
        footer_link(&link("/sitemap.xml"), t.footer_sitemap, Some("map"), false),
    ];
    let legal = vec![
        footer_link(&ctx.href(PageKind::Privacy), t.privacy.title, None, false),
        footer_link(&ctx.href(PageKind::Terms), t.terms.title, None, false),
        footer_link(&ctx.href(PageKind::Legal), t.legal.title, None, false),
    ];
    footer()
        .class("footer")
        .child(
            div()
                .class("footer__wave")
                .attr("aria-hidden", "true")
                .child(span().class("wave wave--a"))
                .child(span().class("wave wave--b")),
        )
        .child(
            div()
                .class("container footer__inner")
                .child(
                    div()
                        .class("footer__hero")
                        .attr("data-reveal", "")
                        .child(letters("LOCALINK", "footer__word"))
                        .child(
                            a().class("footer__top")
                                .href("#main")
                                .attr("data-top", "")
                                .attr("aria-label", t.footer_top)
                                .html(
                                    r#"<svg class="footer__ring" viewBox="0 0 48 48" aria-hidden="true"><circle cx="24" cy="24" r="22"/><circle class="footer__ring-fill" cx="24" cy="24" r="22" pathLength="100"/></svg>"#,
                                )
                                .child(icon("arrow-up")),
                        ),
                )
                .child(
                    div()
                        .class("footer__grid")
                        .child(
                            div()
                                .class("footer__brand")
                                .attr("data-reveal", "")
                                .child(brand(ctx))
                                .child(p().class("footer__tagline").text(t.footer_tagline))
                                .child(ul().class("footer__zero").children(
                                    t.footer_zero.iter().enumerate().map(|(i, z)| {
                                        li().style(format!("--i:{i}")).child(icon("check")).child(span().text(*z))
                                    }),
                                ))
                                .child(
                                    p().class("footer__status")
                                        .child(span().class("pulse").attr("aria-hidden", "true"))
                                        .text(t.footer_status)
                                        .child(span().class("footer__sep").text("·"))
                                        .child(icon("offline"))
                                        .child(span().text(t.footer_offline)),
                                ),
                        )
                        .child(footer_column(t.footer_product, product))
                        .child(footer_column(t.footer_resources, resources))
                        .child(footer_column(t.footer_legal, legal))
                        .child(
                            div()
                                .class("footer__col footer__contact")
                                .attr("data-reveal", "")
                                .child(h2().class("footer__title").text(t.footer_contact))
                                .child(p().class("footer__text").text(t.footer_contact_text))
                                .child(footer_link(
                                    &format!("{REPO}/issues"),
                                    t.footer_contact_link,
                                    Some("chat"),
                                    true,
                                ))
                                .child(island("install", lang.code(), crate::islands::install))
                                .child(
                                    div()
                                        .class("footer__lang")
                                        .child(icon("globe"))
                                        .child(languages(ctx, "langs footer__langs")),
                                ),
                        ),
                )
                .child(
                    div()
                        .class("footer__bottom")
                        .child(p().text(format!(
                            "© {YEAR} {AUTHOR} — LOCALINK. {}",
                            t.footer_rights
                        )))
                        .child(
                            p().class("footer__made")
                                .child(icon("spark"))
                                .text(t.footer_made),
                        )
                        .child(p().class("footer__hosted").text(t.footer_hosted)),
                ),
        )
}

// ----- Home -----------------------------------------------------------------------------

pub fn home(ctx: &Ctx<'_>) -> Page {
    let t = ctx.t();
    let content = main()
        .class("home")
        .child(hero(ctx))
        .child(marquee(ctx))
        .child(how(ctx))
        .child(demo(ctx))
        .child(features(ctx))
        .child(privacy(ctx))
        .child(numbers(ctx))
        .child(app_section(ctx))
        .child(faq(ctx))
        .child(cta(ctx));
    layout(ctx, t.meta_title, t.meta_description, content)
}

/// A phone of the hero: a notch, a status line, a screen.
fn phone(class: &str, title: &str, screen: Vec<Node>) -> Element {
    div()
        .class(format!("phone {class}"))
        .child(span().class("phone__island").attr("aria-hidden", "true"))
        .child(
            div()
                .class("phone__screen")
                .child(
                    p().class("phone__title")
                        .child(span().class("pulse").attr("aria-hidden", "true"))
                        .text(title.to_owned()),
                )
                .children(screen),
        )
}

fn hero(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    let file_icons = ["image", "video", "pdf"];
    let sending: Vec<Node> = t
        .files
        .iter()
        .enumerate()
        .map(|(i, name)| {
            div()
                .class("phone__file")
                .style(format!("--i:{i}"))
                .child(icon(file_icons[i]))
                .child(
                    span()
                        .class("phone__file-meta")
                        .child(span().class("phone__file-name").text(*name))
                        .child(span().class("phone__bar").child(span())),
                )
                .into()
        })
        .collect();
    let receiving: Vec<Node> = vec![
        div()
            .class("phone__qr")
            .child(qr_code(ctx, "qr qr--mini"))
            .child(span().class("phone__corners").attr("aria-hidden", "true"))
            .into(),
        p().class("phone__caption").text(t.phone_scan).into(),
        p().class("phone__connected")
            .child(icon("check"))
            .text(t.phone_connected)
            .into(),
        div()
            .class("phone__received")
            .children(t.files.iter().enumerate().map(|(i, name)| {
                span()
                    .class("phone__got")
                    .style(format!("--i:{i}"))
                    .child(icon(file_icons[i]))
                    .child(span().text(*name))
            }))
            .into(),
    ];
    section()
        .class("hero")
        .attr("aria-labelledby", "hero-title")
        .child(
            canvas()
                .class("hero__canvas")
                .attr("data-network", "")
                .attr("aria-hidden", "true"),
        )
        .child(
            div()
                .class("container hero__grid")
                .child(
                    div()
                        .class("hero__copy")
                        .child(
                            p().class("eyebrow intro")
                                .style("--d:0ms")
                                .child(span().class("pulse").attr("aria-hidden", "true"))
                                .child(span().text(t.hero_eyebrow))
                                .child(span().class("eyebrow__sep").text("/"))
                                .child(span().text("LOCALINK")),
                        )
                        .child(
                            h1().class("hero__title")
                                .id("hero-title")
                                .child(
                                    span()
                                        .class("hero__line")
                                        .children(words(t.hero_title[0], 0)),
                                )
                                .child(
                                    span()
                                        .class("hero__line hero__line--grad")
                                        .children(words(t.hero_title[1], 2)),
                                ),
                        )
                        .child(
                            p().class("rotator intro")
                                .style(format!("--d:700ms;--n:{}", t.hero_words.len()))
                                .child(span().text(t.hero_send))
                                .child(span().class("rotator__words").children(
                                    t.hero_words.iter().enumerate().map(|(i, w)| {
                                        span()
                                            .class("rotator__word")
                                            .style(format!("--i:{i}"))
                                            .text(*w)
                                    }),
                                )),
                        )
                        .child(
                            p().class("hero__lead intro")
                                .style("--d:850ms")
                                .text(t.hero_lead),
                        )
                        .child(
                            div()
                                .class("hero__ctas intro")
                                .style("--d:1000ms")
                                .child(button_link(
                                    &ctx.anchor("how"),
                                    "btn--primary",
                                    t.hero_primary,
                                    "arrow-down",
                                ))
                                .child(button_link(
                                    &ctx.anchor("app"),
                                    "btn--ghost",
                                    t.hero_secondary,
                                    "phone",
                                )),
                        )
                        .child(ul().class("hero__badges").children(
                            t.hero_badges.iter().enumerate().map(|(i, b)| {
                                li().class("intro")
                                    .style(format!("--d:{}ms", 1150 + i * 120))
                                    .child(icon("check"))
                                    .child(span().text(*b))
                            }),
                        )),
                )
                .child(
                    div()
                        .class("hero__stage intro")
                        .style("--d:400ms")
                        .attr("data-tilt-stage", "")
                        .child(phone("phone--a", t.phone_sender, sending))
                        .child(
                            div()
                                .class("beam")
                                .attr("aria-hidden", "true")
                                .child(span().class("beam__line"))
                                .children((0..3).map(|i| {
                                    span()
                                        .class("beam__file")
                                        .style(format!("--i:{i}"))
                                        .child(span().class("beam__arc").child(icon(file_icons[i])))
                                })),
                        )
                        .child(phone("phone--b", t.phone_receiver, receiving)),
                ),
        )
        .child(
            a().class("hero__scroll")
                .href(ctx.anchor("how"))
                .child(span().class("hero__mouse").attr("aria-hidden", "true"))
                .child(span().text(t.hero_scroll)),
        )
}

fn marquee(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    let band = |class: &str, reverse: bool| {
        let items: Vec<Node> = (0..2)
            .flat_map(|copy| {
                t.marquee.iter().map(move |m| {
                    let item = span().class("marquee__item").text(*m);
                    if copy == 1 {
                        item.attr("aria-hidden", "true").into()
                    } else {
                        item.into()
                    }
                })
            })
            .collect();
        div()
            .class(format!("marquee {class}"))
            .class(if reverse { "marquee--reverse" } else { "" })
            .child(div().class("marquee__track").children(items))
    };
    section()
        .class("marquees")
        .attr("aria-label", t.hero_badges.join(" · "))
        .child(band("marquee--a", false))
        .child(band("marquee--b", true).attr("aria-hidden", "true"))
}

/// The little animated drawing of each step.
fn step_art(index: usize) -> Element {
    match index {
        0 => div()
            .class("art art--open")
            .child(span().class("art__ring"))
            .child(span().class("art__ring art__ring--2"))
            .child(span().class("art__ring art__ring--3"))
            .child(span().class("art__icon").child(icon("wifi"))),
        1 => div()
            .class("art art--scan")
            .child(span().class("art__qr").child(icon("qr")))
            .child(span().class("art__laser"))
            .child(span().class("art__corners")),
        _ => div()
            .class("art art--send")
            .child(span().class("art__node").child(icon("phone")))
            .child(
                span()
                    .class("art__wire")
                    .children((0..3).map(|i| span().class("art__bit").style(format!("--i:{i}")))),
            )
            .child(span().class("art__node art__node--b").child(icon("phone"))),
    }
}

fn how(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section().class("section how").id("how").child(
        div()
            .class("container")
            .child(section_head(
                "01",
                t.how_label,
                t.how_title,
                Some(t.how_lead),
            ))
            .child(
                ol().class("steps")
                    .attr("data-steps", "")
                    .child(
                        span()
                            .class("steps__track")
                            .attr("aria-hidden", "true")
                            .child(span().class("steps__fill")),
                    )
                    .children(t.steps.iter().enumerate().map(|(i, step)| {
                        li().class("step card")
                            .attr("data-reveal", "")
                            .attr("data-spotlight", "")
                            .style(format!("--d:{}ms", i * 120))
                            .child(span().class("step__num").text(format!("0{}", i + 1)))
                            .child(step_art(i))
                            .child(h3().class("step__title").text(step.title))
                            .child(p().class("step__text").text(step.text))
                    })),
            )
            .child(
                figure()
                    .class("qr-card card")
                    .attr("data-reveal", "")
                    .attr("data-tilt", "")
                    .child(qr_code(ctx, "qr qr--big"))
                    .child(
                        figcaption()
                            .child(icon("qr"))
                            .child(span().text(t.qr_caption)),
                    ),
            ),
    )
}

fn demo(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section().class("section demo").id("demo").child(
        div()
            .class("container")
            .child(section_head(
                "02",
                t.sim_label,
                t.sim_title,
                Some(t.sim_lead),
            ))
            .child(
                div()
                    .class("demo__frame card")
                    .attr("data-reveal", "")
                    .child(island(
                        "simulator",
                        ctx.lang().code(),
                        crate::islands::simulator,
                    )),
            ),
    )
}

fn features(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    let icons = ["user-off", "cloud-off", "qr", "wifi", "files", "swift"];
    section().class("section features").id("features").child(
        div()
            .class("container")
            .child(section_head(
                "03",
                t.feat_label,
                t.feat_title,
                Some(t.feat_lead),
            ))
            .child(
                div()
                    .class("features__grid")
                    .children(t.features.iter().enumerate().map(|(i, f)| {
                        article()
                            .class("feature card")
                            .attr("data-reveal", "")
                            .attr("data-spotlight", "")
                            .attr("data-tilt", "")
                            .style(format!("--d:{}ms", (i % 3) * 100))
                            .child(span().class("feature__icon").child(icon(icons[i])))
                            .child(h3().class("feature__title").text(f.title))
                            .child(p().class("feature__text").text(f.text))
                            .child(
                                span()
                                    .class("feature__index")
                                    .attr("aria-hidden", "true")
                                    .text(format!("0{}", i + 1)),
                            )
                    })),
            ),
    )
}

/// The diagram of a route: devices in a row, with packets moving along the path.
fn route_svg(kind: &str, labels: &[&str]) -> String {
    let esc = |s: &str| s.replace('&', "&amp;").replace('<', "&lt;");
    if kind == "cloud" {
        format!(
            concat!(
                r#"<svg class="route__svg" viewBox="0 0 320 170" aria-hidden="true">"#,
                r#"<path id="cloud-path" class="route__path route__path--long" d="M40 130 C40 40 120 30 160 30 C200 30 280 40 280 130"/>"#,
                r#"<g class="route__packets route__packets--slow">"#,
                r##"<circle r="4"><animateMotion dur="4.2s" repeatCount="indefinite"><mpath href="#cloud-path"/></animateMotion></circle>"##,
                r##"<circle r="4"><animateMotion dur="4.2s" begin="1.4s" repeatCount="indefinite"><mpath href="#cloud-path"/></animateMotion></circle>"##,
                r##"<circle r="4"><animateMotion dur="4.2s" begin="2.8s" repeatCount="indefinite"><mpath href="#cloud-path"/></animateMotion></circle>"##,
                r#"</g>"#,
                r#"<g class="route__server"><rect x="136" y="10" width="48" height="40" rx="10"/><path d="M146 24h28M146 36h20"/></g>"#,
                r#"<g class="route__device"><rect x="24" y="112" width="32" height="48" rx="8"/></g>"#,
                r#"<g class="route__device"><rect x="264" y="112" width="32" height="48" rx="8"/></g>"#,
                r#"<text x="40" y="104" text-anchor="middle">{a}</text>"#,
                r#"<text x="160" y="68" text-anchor="middle">{b}</text>"#,
                r#"<text x="280" y="104" text-anchor="middle">{c}</text>"#,
                r#"</svg>"#,
            ),
            a = esc(labels[0]),
            b = esc(labels[1]),
            c = esc(labels[2]),
        )
    } else {
        format!(
            concat!(
                r#"<svg class="route__svg" viewBox="0 0 320 170" aria-hidden="true">"#,
                r#"<path id="local-path" class="route__path route__path--short" d="M72 136 L248 136"/>"#,
                r#"<g class="route__packets route__packets--fast">"#,
                r##"<circle r="4"><animateMotion dur="1.1s" repeatCount="indefinite"><mpath href="#local-path"/></animateMotion></circle>"##,
                r##"<circle r="4"><animateMotion dur="1.1s" begin=".37s" repeatCount="indefinite"><mpath href="#local-path"/></animateMotion></circle>"##,
                r##"<circle r="4"><animateMotion dur="1.1s" begin=".74s" repeatCount="indefinite"><mpath href="#local-path"/></animateMotion></circle>"##,
                r#"</g>"#,
                r#"<g class="route__wifi"><path d="M130 76a42 42 0 0 1 60 0"/><path d="M140 88a28 28 0 0 1 40 0"/><path d="M150 100a14 14 0 0 1 20 0"/></g>"#,
                r#"<g class="route__device route__device--ok"><rect x="24" y="112" width="32" height="48" rx="8"/></g>"#,
                r#"<g class="route__device route__device--ok"><rect x="264" y="112" width="32" height="48" rx="8"/></g>"#,
                r#"<text x="40" y="104" text-anchor="middle">{a}</text>"#,
                r#"<text x="280" y="104" text-anchor="middle">{b}</text>"#,
                r#"</svg>"#,
            ),
            a = esc(labels[0]),
            b = esc(labels[1]),
        )
    }
}

fn privacy(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section().class("section privacy").id("privacy").child(
        div()
            .class("container")
            .child(section_head(
                "04",
                t.priv_label,
                t.priv_title,
                Some(t.priv_lead),
            ))
            .child(
                div()
                    .class("routes")
                    .child(
                        figure()
                            .class("route route--cloud card")
                            .attr("data-reveal", "")
                            .child(
                                figcaption()
                                    .class("route__title")
                                    .child(icon("cloud-off"))
                                    .child(span().text(t.priv_cloud)),
                            )
                            .child(
                                div()
                                    .class("route__art")
                                    .html(route_svg("cloud", &t.priv_cloud_path)),
                            )
                            .child(
                                p().class("route__path-text")
                                    .text(t.priv_cloud_path.join(" → ")),
                            ),
                    )
                    .child(
                        figure()
                            .class("route route--local card")
                            .attr("data-reveal", "")
                            .style("--d:140ms")
                            .child(
                                figcaption()
                                    .class("route__title")
                                    .child(icon("shield"))
                                    .child(span().text(t.priv_local)),
                            )
                            .child(
                                div()
                                    .class("route__art")
                                    .html(route_svg("local", &t.priv_local_path)),
                            )
                            .child(
                                p().class("route__path-text")
                                    .text(t.priv_local_path.join(" ↔ ")),
                            ),
                    ),
            )
            .child(
                ul().class("checks")
                    .children(t.priv_points.iter().enumerate().map(|(i, point)| {
                        li().attr("data-reveal", "")
                            .style(format!("--d:{}ms", i * 100))
                            .child(span().class("checks__icon").child(icon("check")))
                            .child(span().text(*point))
                    })),
            ),
    )
}

fn numbers(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section()
        .class("section numbers")
        .attr("aria-labelledby", "numbers-title")
        .child(
            div()
                .class("container")
                .child(
                    header()
                        .class("section-head section-head--center")
                        .child(
                            p().class("label")
                                .attr("data-reveal", "")
                                .child(span().class("label__num").text("05"))
                                .child(span().class("label__line").attr("aria-hidden", "true"))
                                .child(span().text(t.num_label)),
                        )
                        .child(
                            h2().class("section-title split")
                                .id("numbers-title")
                                .attr("data-reveal", "")
                                .children(words(t.num_title, 0)),
                        ),
                )
                .child(
                    div()
                        .class("numbers__grid")
                        .children(t.numbers.iter().enumerate().map(|(i, n)| {
                            let from = if n.value == 0 { 9 } else { 0 };
                            div()
                                .class("number card")
                                .attr("data-reveal", "")
                                .style(format!("--d:{}ms", i * 110))
                                .child(
                                    p().class("number__value")
                                        .child(
                                            span()
                                                .class("number__count")
                                                .attr("data-count", n.value.to_string())
                                                .attr("data-from", from.to_string())
                                                .text(n.value.to_string()),
                                        )
                                        .child(span().class("number__unit").text(n.unit)),
                                )
                                .child(p().class("number__label").text(n.label))
                                .child(span().class("number__orbit").attr("aria-hidden", "true"))
                        })),
                ),
        )
}

fn app_section(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    let screen = div()
        .class("iphone__screen")
        .child(
            div()
                .class("iphone__status")
                .child(span().text("9:41"))
                .child(span().class("iphone__icons").child(icon("wifi"))),
        )
        .child(p().class("iphone__title").text(t.app_screen_title))
        .child(
            div()
                .class("radar")
                .attr("aria-hidden", "true")
                .child(span().class("radar__ring"))
                .child(span().class("radar__ring radar__ring--2"))
                .child(span().class("radar__ring radar__ring--3"))
                .child(span().class("radar__sweep"))
                .child(span().class("radar__me").child(icon("phone")))
                .children((0..3).map(|i| {
                    span()
                        .class(format!("radar__peer radar__peer--{i}"))
                        .child(span().class("radar__avatar").text(["L", "M", "S"][i]))
                })),
        )
        .child(
            p().class("iphone__nearby")
                .child(span().class("pulse"))
                .text(t.app_screen_nearby),
        )
        .child(
            div()
                .class("iphone__list")
                .children(t.files.iter().enumerate().map(|(i, f)| {
                    div()
                        .class("iphone__row")
                        .style(format!("--i:{i}"))
                        .child(icon(["image", "video", "pdf"][i]))
                        .child(span().class("iphone__name").text(*f))
                        .child(span().class("iphone__bar").child(span()))
                })),
        );
    section().class("section app").id("app").child(
        div()
            .class("container app__grid")
            .child(
                div()
                    .class("app__copy")
                    .child(section_head(
                        "06",
                        t.app_label,
                        t.app_title,
                        Some(t.app_lead),
                    ))
                    .child(ul().class("checks checks--app").children(
                        t.app_points.iter().enumerate().map(|(i, point)| {
                            li().attr("data-reveal", "")
                                .style(format!("--d:{}ms", i * 90))
                                .child(span().class("checks__icon").child(icon("check")))
                                .child(span().text(*point))
                        }),
                    ))
                    .child(
                        div()
                            .class("app__badges")
                            .attr("data-reveal", "")
                            .style("--d:200ms")
                            .child(
                                span()
                                    .class("store")
                                    .attr("aria-disabled", "true")
                                    .child(icon("phone"))
                                    .child(
                                        span()
                                            .class("store__text")
                                            .child(small().text(t.app_store_small))
                                            .child(strong().text(t.app_store)),
                                    ),
                            )
                            .child(
                                span()
                                    .class("chip chip--swift")
                                    .child(icon("swift"))
                                    .child(span().text(t.app_swift)),
                            ),
                    ),
            )
            .child(
                div()
                    .class("app__device")
                    .attr("data-reveal", "")
                    .attr("data-parallax-self", "0.06")
                    .child(
                        div()
                            .class("iphone")
                            .attr("data-tilt", "")
                            .child(span().class("iphone__island"))
                            .child(screen),
                    )
                    .child(span().class("float float--a").child(icon("image")))
                    .child(span().class("float float--b").child(icon("music")))
                    .child(span().class("float float--c").child(icon("pdf")))
                    .child(span().class("float float--d").child(icon("video"))),
            ),
    )
}

fn faq(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section().class("section faq").id("faq").child(
        div()
            .class("container faq__grid")
            .child(section_head("07", t.faq_label, t.faq_title, None))
            .child(
                div()
                    .class("faq__list")
                    .children(t.faq.iter().enumerate().map(|(i, q)| {
                        let mut item = details()
                            .class("faq__item")
                            .attr("name", "faq")
                            .attr("data-reveal", "")
                            .style(format!("--d:{}ms", i * 70));
                        if i == 0 {
                            item = item.attr("open", "");
                        }
                        item.child(
                            summary()
                                .class("faq__q")
                                .child(span().text(q.title))
                                .child(span().class("faq__plus").child(icon("plus"))),
                        )
                        .child(div().class("faq__a").child(p().text(q.text)))
                    })),
            ),
    )
}

fn cta(ctx: &Ctx<'_>) -> Element {
    let t = ctx.t();
    section().class("section cta").child(
        div().class("container").child(
            div()
                .class("cta__box")
                .attr("data-reveal", "")
                .child(
                    div()
                        .class("cta__orbits")
                        .attr("aria-hidden", "true")
                        .child(
                            span()
                                .class("orbit orbit--1")
                                .child(span().class("orbit__dot")),
                        )
                        .child(
                            span()
                                .class("orbit orbit--2")
                                .child(span().class("orbit__dot")),
                        )
                        .child(
                            span()
                                .class("orbit orbit--3")
                                .child(span().class("orbit__dot")),
                        ),
                )
                .child(logo())
                .child(
                    h2().class("cta__title split")
                        .children(words(t.cta_title, 0)),
                )
                .child(p().class("cta__lead").text(t.cta_lead))
                .child(button_link(
                    &ctx.anchor("how"),
                    "btn--primary btn--lg",
                    t.cta_button,
                    "arrow",
                )),
        ),
    )
}

// ----- Legal pages ----------------------------------------------------------------------

fn legal_page(ctx: &Ctx<'_>, legal: &Legal) -> Page {
    let t = ctx.t();
    let others = [
        (PageKind::Privacy, t.privacy.title),
        (PageKind::Terms, t.terms.title),
        (PageKind::Legal, t.legal.title),
    ];
    let content = main().class("legal").child(
        div()
            .class("container legal__grid")
            .child(
                header()
                    .class("legal__head")
                    .child(
                        a().class("back intro")
                            .href(ctx.href(PageKind::Home))
                            .child(icon("arrow"))
                            .child(span().text(t.nav_home)),
                    )
                    .child(
                        h1().class("legal__title split")
                            .children(words(legal.title, 0)),
                    )
                    .child(p().class("lead intro").style("--d:300ms").text(legal.lead))
                    .child(
                        p().class("legal__updated intro")
                            .style("--d:400ms")
                            .text(t.updated),
                    ),
            )
            .child(
                nav()
                    .class("legal__nav")
                    .attr("aria-label", t.footer_legal)
                    .children(others.iter().map(|(page, title)| {
                        let mut link = a().class("legal__tab").href(ctx.href(*page)).text(*title);
                        if *page == ctx.route.page {
                            link = link.attr("aria-current", "page");
                        }
                        link
                    })),
            )
            .child(
                article()
                    .class("legal__body")
                    .children(legal.sections.iter().enumerate().map(|(i, s)| {
                        section()
                            .class("legal__section card")
                            .attr("data-reveal", "")
                            .style(format!("--d:{}ms", (i % 3) * 80))
                            .child(
                                h2().child(
                                    span().class("legal__num").text(format!("{:02}", i + 1)),
                                )
                                .child(span().text(s.title)),
                            )
                            .child(p().text(s.text))
                    })),
            ),
    );
    let title = format!("{} — LOCALINK", legal.title);
    layout(ctx, &title, legal.lead, content)
}

// ----- Not found, language picker -------------------------------------------------------

fn lost(ctx: &Ctx<'_>, title: &str, text: &str, actions: Element) -> Element {
    main()
        .class("lost")
        .child(
            div()
                .class("lost__art")
                .attr("aria-hidden", "true")
                .child(span().class("lost__phone").child(icon("phone")))
                .child(span().class("lost__link"))
                .child(
                    span()
                        .class("lost__phone lost__phone--b")
                        .child(icon("phone")),
                ),
        )
        .child(p().class("lost__code").attr("data-glitch", "404").text(
            if ctx.route.page == PageKind::NotFound {
                "404"
            } else {
                ""
            },
        ))
        .child(h1().class("lost__title split").children(words(title, 0)))
        .child(
            p().class("lead intro")
                .style("--d:400ms")
                .text(text.to_owned()),
        )
        .child(actions)
}

pub fn not_found(ctx: &Ctx<'_>) -> Page {
    let t = ctx.t();
    let actions = div()
        .class("lost__actions intro")
        .style("--d:550ms")
        .child(button_link(
            &ctx.href(PageKind::Home),
            "btn--primary",
            t.nf_back,
            "arrow",
        ));
    layout(
        ctx,
        &format!("404 — {}", t.nf_title),
        t.nf_text,
        lost(ctx, t.nf_title, t.nf_text, actions),
    )
    .status(404)
}

/// The page a static host serves at the root (`target` empty) and for an unknown
/// address (`target` = `404/`): the browser bundle sends the visitor to their language
/// (`data-auto-lang`); the links do it without it.
pub fn language_picker(site: &Site, origin: &str, target: &str) -> Page {
    let ctx = Ctx {
        site,
        route: Route::new(
            Lang::Fr,
            if target.is_empty() {
                PageKind::Home
            } else {
                PageKind::NotFound
            },
        ),
        origin: origin.to_owned(),
    };
    let actions = div()
        .class("lost__actions intro")
        .style("--d:500ms")
        .attr("data-auto-lang", target.to_owned())
        .children(Lang::ALL.iter().map(|l| {
            a().class("btn btn--ghost")
                .href(link(&format!("/{}/{target}", l.code())))
                .attr("hreflang", l.code())
                .attr("lang", l.code())
                .attr("data-magnetic", "")
                .child(span().class("btn__label").text(l.name()))
                .child(icon("arrow"))
        }));
    let content = lost(&ctx, ctx.t().pick_title, ctx.t().pick_text, actions);
    // Without the bundle (JavaScript off), French.
    let fallback = format!(
        r#"<noscript><meta http-equiv="refresh" content="0; url={}"/></noscript>"#,
        link(&format!("/fr/{target}"))
    );
    let mut page = layout(&ctx, "LOCALINK", ctx.t().meta_description, content).head(fallback);
    if !target.is_empty() {
        page = page.status(404);
    }
    page
}

/// The page of a route.
pub fn render(ctx: &Ctx<'_>) -> Page {
    let t = ctx.t();
    match ctx.route.page {
        PageKind::Home => home(ctx),
        PageKind::Privacy => legal_page(ctx, &t.privacy),
        PageKind::Terms => legal_page(ctx, &t.terms),
        PageKind::Legal => legal_page(ctx, &t.legal),
        PageKind::NotFound => not_found(ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_strings_are_escaped() {
        assert_eq!(json_string("a\"b</c"), r#""a\"b\u003c/c""#);
    }

    #[test]
    fn route_diagrams_escape_their_labels() {
        let svg = route_svg("cloud", &["a<b", "c", "d"]);
        assert!(svg.contains("a&lt;b"));
        assert_eq!(svg.matches("animateMotion dur").count(), 3);
    }
}
