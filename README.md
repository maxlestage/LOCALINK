<div align="center">

<img src="site/public/icons/icon-192.png" width="96" alt="LOCALINK"/>

# LOCALINK — Transfert

**Transfert de fichiers entre téléphones, côte à côte ou à distance. Connexion par QR code ou par code, au choix, sans compte. À distance, le partage passe par le serveur de l'application et s'efface automatiquement après 2 minutes.**

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
- **Connexion par code ou par QR code, au choix, et partage éphémère** : un vrai code LOCALINK
  et son QR code, régénérés dans le navigateur, un sélecteur « Code / QR code », et un compte à rebours de 2 minutes (accéléré) jusqu'à l'effacement du
  fichier sur le serveur.
- **Pages légales** : confidentialité, conditions d'utilisation, mentions légales.
- **Zéro cookie, zéro traceur** : polices et images servies par le site lui-même, CSP stricte.
- **Compression maximale** : sur Heroku, toutes les réponses texte (pages, CSS, JavaScript,
  WebAssembly, SVG, manifeste…) partent en **Brotli 11** (crate `brotli`), calculé une fois puis
  gardé en mémoire, avec gzip pour les rares navigateurs sans Brotli. La feuille de style est
  minifiée par `lightningcss` au déploiement et le WebAssembly compilé en `opt-level = "z"`.
  Accueil : 106 Ko → 11 Ko, WebAssembly : 281 Ko → 98 Ko, CSS : 81 Ko → 13 Ko. GitHub Pages ne
  permet pas de choisir la compression : il sert le site en gzip.

## L'application iOS (Swift)

Le dossier [`ios/`](ios) contient l'application iPhone, en **Swift 6 et SwiftUI** (iOS 17 et
plus) :

- **Envoyer** : photos, vidéos ou fichiers, puis **À côté** (en direct sur le même réseau, ou
  par le Wi‑Fi pair-à-pair d'Apple) ou **À distance** (par le serveur, effacé après 2 minutes).
- **Recevoir** : votre code LOCALINK et son QR code, **au choix**, pour un téléphone à côté ; ou
  le code reçu d'un téléphone loin, tapé ou scanné, au choix.
- **Reçus** : les fichiers reçus, à ouvrir, partager ou supprimer, aussi visibles dans l'app
  Fichiers (« Sur mon iPhone › LOCALINK »).
- À côté, la connexion est chiffrée (TLS) avec une clé tirée du code : seul le téléphone qui
  connaît le code peut envoyer. À distance, le fichier ne vit que 2 minutes, en mémoire, sur le
  serveur (`/api/shares`), puis il est effacé.
- Français, anglais, espagnol ; animations SwiftUI, réglage « réduire les animations » respecté.

| Dossier | Contenu |
|---|---|
| [`ios/LocalinkCore/`](ios/LocalinkCore) | le cœur en Swift pur (codes, protocole entre téléphones, client de l'API), testé sur macOS et Linux |
| [`ios/Localink/`](ios/Localink) | l'app SwiftUI : écrans, réseau (Network.framework), scanner (VisionKit), ressources |
| [`ios/project.yml`](ios/project.yml) | le projet Xcode, généré par XcodeGen sur les machines macOS de GitHub |

**Sans Mac** : chaque push compile l'app sur macOS (Xcode 26) dans la CI et garde une IPA non
signée en artefact. Pour l'installer sur un iPhone, [`testflight.yml`](.github/workflows/testflight.yml)
signe l'app dans le cloud et l'envoie sur **TestFlight** : il faut un compte Apple Developer, une
clé API App Store Connect et quatre réglages GitHub, tous faisables depuis un téléphone (les
étapes sont en tête du fichier).

## Structure

| Dossier | Contenu |
|---|---|
| [`site/src/server/`](site/src/server) | le serveur Paquets : pages, sitemap, Brotli 11, minification, export statique |
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
side by side or far apart, a QR code or a code to connect (your choice), no account; remote shares go through the app server and are erased after 2 minutes — written **only in Rust** with
[Paquets](https://github.com/maxlestage/Paquets): server-rendered pages and a WebAssembly
bundle for every animation (2D canvas network, transfer simulator, scroll reveals…). The
iPhone app is written in Swift. Mobile first with no horizontal scrolling, French, English
and Spanish, installable PWA, complete footer, legal pages, no cookies and no trackers.
Deploy with the Heroku button above (emk/rust buildpack), enable GitHub Pages once
(*Settings → Pages → Source: GitHub Actions*); every green branch is merged into `master`
automatically, then deployed.

## Español

El **sitio de presentación** de LOCALINK — transferencia de archivos entre teléfonos por la
red local, cerca o lejos, un código QR o un código para conectarse (a elegir), sin cuenta; lo compartido a distancia pasa por el servidor de la app y se borra a los 2 minutos — escrito
**solo en Rust** con [Paquets](https://github.com/maxlestage/Paquets): páginas generadas en el
servidor y un bundle WebAssembly para todas las animaciones. La app de iPhone está escrita en
Swift. Pensado primero para móvil y sin desplazamiento horizontal, en francés, inglés y
español, PWA instalable, pie de página completo, páginas legales, sin cookies ni rastreadores.
Despliega con el botón de Heroku (buildpack emk/rust), activa GitHub Pages una vez
(*Settings → Pages → Source: GitHub Actions*); cada rama en verde se fusiona sola en `master`
y se despliega.

## Licence

© 2026 Maxime Nathan Lestage. Tous droits réservés.
