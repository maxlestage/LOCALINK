<div align="center">

<img src="site/public/icons/icon-192.png" width="96" alt="LOCALINK"/>

# LOCALINK — Transfert

**Transfert de fichiers entre téléphones sur le réseau, QR pour établir une connexion, sans compte et sans stockage cloud.**

**Français** · [English](#english) · [Español](#español)

[Site sur Heroku](https://localink-cb296cd42039.herokuapp.com/) · [Site sur GitHub Pages](https://maxlestage.github.io/LOCALINK/) · [Déployer sur Heroku](https://heroku.com/deploy?template=https://github.com/maxlestage/LOCALINK)

</div>

---

Ce dépôt contient le **site de présentation** de LOCALINK, écrit **uniquement en Rust** avec
le framework [Paquets](https://github.com/maxlestage/Paquets) : les pages sont rendues par le
serveur Rust, et tout ce qui bouge dans le navigateur (le réseau 2D animé du haut de page, le
simulateur de transfert, les révélations au défilement, les compteurs, le curseur, les boutons
magnétiques…) est du Rust compilé en WebAssembly. L'application iPhone, elle, est écrite en
Swift.

## Ce que fait le site

- **Mobile first** : chaque mise en page part de 320 px de large ; aucun défilement horizontal
  n'est possible (vérifié de 320 à 1440 px).
- **2D et beaucoup d'animations** : préchargement, réseau de nœuds sur `<canvas>` qui suit le
  doigt ou la souris, téléphones qui s'envoient des fichiers, vrai QR code qui se dessine
  module par module, bandeaux défilants, schémas SVG animés cloud vs local, radar iPhone,
  compteurs, cartes inclinables, projecteur, accordéon FAQ, vagues et lettres du pied de page,
  barre de progression, menu plein écran… Le réglage « réduire les animations » de l'appareil
  est respecté.
- **Trois langues** : français, anglais, espagnol (`/fr/`, `/en/`, `/es/`), choisies selon le
  navigateur ; un texte manquant ne compile pas.
- **PWA** : installable sur l'écran d'accueil (manifeste par langue, icônes, service worker),
  consultable hors ligne.
- **Pied de page complet** : produit, ressources, légal, contact, installation, langues,
  statut, retour en haut avec anneau de progression.
- **Pages légales** : confidentialité, conditions d'utilisation, mentions légales.
- **Zéro cookie, zéro traceur** : polices et images servies par le site lui-même, CSP stricte.

## Structure

| Dossier | Contenu |
|---|---|
| [`site/src/server/`](site/src/server) | le serveur Paquets : pages, QR code, sitemap, export statique |
| [`site/src/client/`](site/src/client) | le bundle WebAssembly : canvas 2D, animations, installation PWA |
| [`site/src/islands.rs`](site/src/islands.rs) | les îlots interactifs : simulateur de transfert, bouton d'installation |
| [`site/src/i18n.rs`](site/src/i18n.rs) | tous les textes, en trois langues |
| [`site/public/`](site/public) | feuille de style, polices, icônes, logo, image Open Graph |
| [`.github/workflows/`](.github/workflows) | CI + fusion automatique, GitHub Pages, Heroku |

## Tout faire depuis un téléphone

### Heroku (buildpack `emk/rust`)

1. Ouvrez <https://heroku.com/deploy?template=https://github.com/maxlestage/LOCALINK>, choisissez
   un nom, touchez **Deploy app** : Heroku lit [`app.json`](app.json) et construit le site avec
   le buildpack [`emk/rust`](https://elements.heroku.com/buildpacks/emk/heroku-buildpack-rust)
   sur la pile heroku-26 ([`RustConfig`](RustConfig), [`heroku-build.sh`](heroku-build.sh),
   [`Procfile`](Procfile)).
2. Déploiement automatique de `master` : GitHub → *Settings → Secrets and variables → Actions*,
   ajoutez le secret `HEROKU_API_KEY` (Heroku → *Account settings* → *API Key*) et, si l'app ne
   s'appelle pas `localink`, la variable `HEROKU_APP_NAME`.
   [`heroku.yml`](.github/workflows/heroku.yml) déploie alors chaque fusion.

### GitHub Pages

Une seule fois : GitHub → *Settings → Pages → Build and deployment → Source* : **GitHub
Actions**. Ensuite [`pages.yml`](.github/workflows/pages.yml) publie le site à chaque fusion
dans `master` (ou à la main : *Actions → Deploy to GitHub Pages → Run workflow*).

### Fusion automatique dans `master`

Chaque push lance la [CI](.github/workflows/ci.yml) : formatage, clippy (serveur et
WebAssembly), tests, bundle WebAssembly et export statique. Quand tout est vert, la branche
est **fusionnée automatiquement dans `master`** (sauf les branches `wip/...`), puis GitHub Pages
et Heroku sont déployés.

## Développer

```bash
cd site
# Le bundle du navigateur (îlots, canvas 2D, animations) → pkg/
cargo build --lib --target wasm32-unknown-unknown --profile wasm-release
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/wasm-release/localink_site.wasm
# Le serveur, sur http://localhost:3000 (PORT sur Heroku)
cargo run --release
# L'export statique (GitHub Pages)
BASE_PATH=/LOCALINK cargo run --release -- export dist
# Les vérifications
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

`wasm-bindgen-cli` doit avoir la version de `wasm-bindgen` de `site/Cargo.lock` (0.2.130).

---

## English

The **presentation website** of LOCALINK — file transfer between phones on the local network,
a QR code to connect, no account, no cloud storage — written **only in Rust** with
[Paquets](https://github.com/maxlestage/Paquets): server-rendered pages and a WebAssembly
bundle for every animation (2D canvas network, transfer simulator, scroll reveals…). The
iPhone app is written in Swift. Mobile first with no horizontal scrolling, French, English
and Spanish, installable PWA, complete footer, legal pages, no cookies and no trackers.
Deploy with the Heroku button above (emk/rust buildpack), enable GitHub Pages once
(*Settings → Pages → Source: GitHub Actions*); every green branch is merged into `master`
automatically, then deployed.

## Español

El **sitio de presentación** de LOCALINK — transferencia de archivos entre teléfonos por la
red local, un código QR para conectarse, sin cuenta y sin almacenamiento en la nube — escrito
**solo en Rust** con [Paquets](https://github.com/maxlestage/Paquets): páginas generadas en el
servidor y un bundle WebAssembly para todas las animaciones. La app de iPhone está escrita en
Swift. Pensado primero para móvil y sin desplazamiento horizontal, en francés, inglés y
español, PWA instalable, pie de página completo, páginas legales, sin cookies ni rastreadores.
Despliega con el botón de Heroku (buildpack emk/rust), activa GitHub Pages una vez
(*Settings → Pages → Source: GitHub Actions*); cada rama en verde se fusiona sola en `master`
y se despliega.

## Licence

© 2026 Maxime Nathan Lestage. Tous droits réservés.
