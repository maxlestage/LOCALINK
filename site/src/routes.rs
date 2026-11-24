//! The URLs of the site: `/{lang}/` and its legal pages, behind an optional base path
//! (`/LOCALINK` when the site is published on GitHub Pages).

use std::sync::OnceLock;

pub use crate::i18n::Lang;

static BASE: OnceLock<String> = OnceLock::new();

/// Sets the base path once, before anything is rendered: `""` (the default) or
/// `/LOCALINK`. The server takes it from `BASE_PATH`, the browser from `data-base`.
pub fn set_base(base: &str) {
    let _ = BASE.set(normalize_base(base));
}

/// `""`, or `/something` without a trailing slash.
pub fn normalize_base(base: &str) -> String {
    let base = base.trim().trim_end_matches('/');
    match base {
        "" => String::new(),
        b if b.starts_with('/') => b.to_owned(),
        b => format!("/{b}"),
    }
}

/// The base path, without trailing slash (empty by default).
pub fn base() -> &'static str {
    BASE.get().map_or("", String::as_str)
}

/// A link to `path` (which starts with `/`), behind the base path.
pub fn link(path: &str) -> String {
    format!("{}{path}", base())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Home,
    Privacy,
    Terms,
    Legal,
    NotFound,
}

impl Page {
    /// The pages that have an address (not the "not found" page).
    pub const LISTED: [Page; 4] = [Page::Home, Page::Privacy, Page::Terms, Page::Legal];

    pub fn slug(self) -> &'static str {
        match self {
            Page::Home => "",
            Page::Privacy => "privacy",
            Page::Terms => "terms",
            Page::Legal => "legal",
            Page::NotFound => "404",
        }
    }

    fn from_slug(slug: &str) -> Page {
        Page::LISTED
            .into_iter()
            .find(|p| p.slug() == slug)
            .unwrap_or(Page::NotFound)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    pub lang: Lang,
    pub page: Page,
}

impl Route {
    pub fn new(lang: Lang, page: Page) -> Route {
        Route { lang, page }
    }

    /// The route of a path (without the base path); `None` when it does not start
    /// with a language.
    pub fn parse(path: &str) -> Option<Route> {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let lang = Lang::parse(parts.first()?)?;
        let page = match parts.len() {
            1 => Page::Home,
            2 => Page::from_slug(parts[1]),
            _ => Page::NotFound,
        };
        Some(Route { lang, page })
    }

    /// The path, without the base path: `/fr/`, `/en/privacy/`.
    pub fn path(self) -> String {
        match self.page {
            Page::Home => format!("/{}/", self.lang.code()),
            page => format!("/{}/{}/", self.lang.code(), page.slug()),
        }
    }

    /// The link, behind the base path.
    pub fn href(self) -> String {
        link(&self.path())
    }

    pub fn with_lang(self, lang: Lang) -> Route {
        Route { lang, ..self }
    }

    /// Every page of the site, in every language.
    pub fn all() -> Vec<Route> {
        Lang::ALL
            .into_iter()
            .flat_map(|lang| Page::LISTED.into_iter().map(move |p| Route::new(lang, p)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_round_trip() {
        for route in Route::all() {
            assert_eq!(Route::parse(&route.path()), Some(route));
        }
        assert_eq!(Route::all().len(), 12);
        assert_eq!(
            Route::parse("/fr/nope/").map(|r| r.page),
            Some(Page::NotFound)
        );
        assert_eq!(Route::parse("/de/"), None);
        assert_eq!(Route::parse("/"), None);
    }

    #[test]
    fn bases_are_normalized() {
        assert_eq!(normalize_base(""), "");
        assert_eq!(normalize_base("/"), "");
        assert_eq!(normalize_base("LOCALINK/"), "/LOCALINK");
        assert_eq!(normalize_base("/LOCALINK"), "/LOCALINK");
    }
}
