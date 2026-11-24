//! Every text of the site, in French, English and Spanish. A missing text does not
//! compile: each language fills the same [`Texts`] structure.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    Fr,
    En,
    Es,
}

impl Lang {
    /// French first: the default language of the site.
    pub const ALL: [Lang; 3] = [Lang::Fr, Lang::En, Lang::Es];

    pub fn code(self) -> &'static str {
        match self {
            Lang::Fr => "fr",
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    /// The name of the language, in that language.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Fr => "Français",
            Lang::En => "English",
            Lang::Es => "Español",
        }
    }

    /// For Open Graph (`og:locale`).
    pub fn locale(self) -> &'static str {
        match self {
            Lang::Fr => "fr_FR",
            Lang::En => "en_US",
            Lang::Es => "es_ES",
        }
    }

    pub fn parse(code: &str) -> Option<Lang> {
        Lang::ALL.into_iter().find(|l| l.code() == code)
    }

    pub fn texts(self) -> &'static Texts {
        match self {
            Lang::Fr => &FR,
            Lang::En => &EN,
            Lang::Es => &ES,
        }
    }
}

/// The preferred language of an `Accept-Language` header (French by default).
pub fn pick_lang(header: &str) -> Lang {
    let mut best: Option<(Lang, f32)> = None;
    for part in header.split(',') {
        let mut pieces = part.trim().split(';');
        let tag = pieces.next().unwrap_or("").trim().to_ascii_lowercase();
        let q = pieces
            .find_map(|p| p.trim().strip_prefix("q=").and_then(|q| q.parse().ok()))
            .unwrap_or(1.0_f32);
        let Some(lang) = Lang::parse(tag.get(..2).unwrap_or("")) else {
            continue;
        };
        if best.is_none_or(|(_, b)| q > b) {
            best = Some((lang, q));
        }
    }
    best.map_or(Lang::Fr, |(lang, _)| lang)
}

/// A titled paragraph (features, steps, questions, legal sections).
#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub title: &'static str,
    pub text: &'static str,
}

const fn item(title: &'static str, text: &'static str) -> Item {
    Item { title, text }
}

/// A figure of the "in numbers" section: value, unit and caption.
#[derive(Clone, Copy, Debug)]
pub struct Figure {
    pub value: u32,
    pub unit: &'static str,
    pub label: &'static str,
}

const fn figure(value: u32, unit: &'static str, label: &'static str) -> Figure {
    Figure { value, unit, label }
}

/// A legal page: title, introduction and sections.
#[derive(Clone, Copy, Debug)]
pub struct Legal {
    pub title: &'static str,
    pub lead: &'static str,
    pub sections: &'static [Item],
}

pub struct Texts {
    // Head and navigation
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    pub skip: &'static str,
    pub nav_how: &'static str,
    pub nav_features: &'static str,
    pub nav_privacy: &'static str,
    pub nav_app: &'static str,
    pub nav_faq: &'static str,
    pub nav_home: &'static str,
    pub nav_menu: &'static str,
    pub nav_cta: &'static str,
    pub loader: &'static str,
    pub lang_label: &'static str,
    // Hero
    pub hero_eyebrow: &'static str,
    pub hero_title: [&'static str; 2],
    pub hero_send: &'static str,
    pub hero_words: [&'static str; 4],
    pub hero_lead: &'static str,
    pub hero_primary: &'static str,
    pub hero_secondary: &'static str,
    pub hero_badges: [&'static str; 3],
    pub hero_scroll: &'static str,
    pub phone_sender: &'static str,
    pub phone_receiver: &'static str,
    pub phone_scan: &'static str,
    pub phone_connected: &'static str,
    pub phone_received: &'static str,
    pub files: [&'static str; 3],
    pub marquee: [&'static str; 6],
    // How it works
    pub how_label: &'static str,
    pub how_title: &'static str,
    pub how_lead: &'static str,
    pub steps: [Item; 3],
    pub qr_caption: &'static str,
    // Simulator
    pub sim_label: &'static str,
    pub sim_title: &'static str,
    pub sim_lead: &'static str,
    pub sim_kinds: [&'static str; 4],
    pub sim_pick: &'static str,
    pub sim_send: &'static str,
    pub sim_sending: &'static str,
    pub sim_done: &'static str,
    pub sim_again: &'static str,
    pub sim_from: &'static str,
    pub sim_to: &'static str,
    pub sim_network: &'static str,
    pub sim_note: &'static str,
    // Features
    pub feat_label: &'static str,
    pub feat_title: &'static str,
    pub feat_lead: &'static str,
    pub features: [Item; 6],
    // Privacy
    pub priv_label: &'static str,
    pub priv_title: &'static str,
    pub priv_lead: &'static str,
    pub priv_cloud: &'static str,
    pub priv_cloud_path: [&'static str; 3],
    pub priv_local: &'static str,
    pub priv_local_path: [&'static str; 2],
    pub priv_points: [&'static str; 3],
    // Numbers
    pub num_label: &'static str,
    pub num_title: &'static str,
    pub numbers: [Figure; 4],
    // iPhone app
    pub app_label: &'static str,
    pub app_title: &'static str,
    pub app_lead: &'static str,
    pub app_points: [&'static str; 4],
    pub app_store: &'static str,
    pub app_store_small: &'static str,
    pub app_swift: &'static str,
    pub app_screen_title: &'static str,
    pub app_screen_nearby: &'static str,
    // FAQ
    pub faq_label: &'static str,
    pub faq_title: &'static str,
    pub faq: [Item; 6],
    // Call to action
    pub cta_title: &'static str,
    pub cta_lead: &'static str,
    pub cta_button: &'static str,
    // Footer
    pub footer_tagline: &'static str,
    pub footer_product: &'static str,
    pub footer_resources: &'static str,
    pub footer_legal: &'static str,
    pub footer_contact: &'static str,
    pub footer_contact_text: &'static str,
    pub footer_contact_link: &'static str,
    pub footer_source: &'static str,
    pub footer_paquets: &'static str,
    pub footer_sitemap: &'static str,
    pub footer_press: &'static str,
    pub footer_install: &'static str,
    pub footer_install_hint: &'static str,
    pub footer_installed: &'static str,
    pub footer_offline: &'static str,
    pub footer_top: &'static str,
    pub footer_rights: &'static str,
    pub footer_made: &'static str,
    pub footer_zero: [&'static str; 3],
    pub footer_hosted: &'static str,
    pub footer_status: &'static str,
    // Legal pages
    pub updated: &'static str,
    pub privacy: Legal,
    pub terms: Legal,
    pub legal: Legal,
    // Not found, language picker
    pub nf_title: &'static str,
    pub nf_text: &'static str,
    pub nf_back: &'static str,
    pub pick_title: &'static str,
    pub pick_text: &'static str,
}

// ----- Français ---------------------------------------------------------------------

pub static FR: Texts = Texts {
    meta_title: "LOCALINK — Transfert de fichiers entre téléphones, sans compte ni cloud",
    meta_description: "LOCALINK transfère vos fichiers d'un téléphone à l'autre sur votre réseau local. Un QR code pour se connecter, aucun compte, aucun stockage dans le cloud.",
    skip: "Aller au contenu",
    nav_how: "Fonctionnement",
    nav_features: "Atouts",
    nav_privacy: "Confidentialité",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_home: "Accueil",
    nav_menu: "Menu",
    nav_cta: "Découvrir",
    loader: "Connexion au réseau local…",
    lang_label: "Langue",
    hero_eyebrow: "Transfert",
    hero_title: ["Vos fichiers,", "d'un téléphone à l'autre."],
    hero_send: "Envoyez vos",
    hero_words: ["photos", "vidéos", "documents", "musiques"],
    hero_lead: "LOCALINK fait passer vos fichiers directement entre deux téléphones sur le même réseau. Un QR code pour se connecter, et c'est parti : sans compte, sans cloud, sans câble.",
    hero_primary: "Comment ça marche",
    hero_secondary: "L'app iPhone",
    hero_badges: ["Sans compte", "Sans cloud", "Réseau local"],
    hero_scroll: "Défiler",
    phone_sender: "Envoi",
    phone_receiver: "Réception",
    phone_scan: "Scannez pour vous connecter",
    phone_connected: "Connecté",
    phone_received: "Reçu",
    files: ["vacances.jpg", "concert.mov", "contrat.pdf"],
    marquee: [
        "Sans compte",
        "Sans cloud",
        "Un QR code",
        "Réseau local",
        "Direct d'appareil à appareil",
        "Tous types de fichiers",
    ],
    how_label: "Fonctionnement",
    how_title: "Trois gestes, zéro inscription.",
    how_lead: "Pas d'identifiant à retenir, pas de lien à copier : les deux téléphones se trouvent grâce à un QR code, puis échangent en direct.",
    steps: [
        item(
            "Ouvrez LOCALINK",
            "Sur les deux téléphones, connectés au même réseau Wi‑Fi ou au partage de connexion de l'un d'eux.",
        ),
        item(
            "Scannez le QR code",
            "Le téléphone qui reçoit affiche un QR code. L'autre le scanne : la connexion est établie, sans code à taper.",
        ),
        item(
            "Envoyez",
            "Choisissez vos fichiers : ils passent directement d'un appareil à l'autre, sans détour par un serveur.",
        ),
    ],
    qr_caption: "Un vrai QR code : scannez-le pour ouvrir ce site sur votre téléphone.",
    sim_label: "Essayez",
    sim_title: "Un transfert, en direct.",
    sim_lead: "Choisissez un fichier et envoyez-le d'un téléphone à l'autre, sur le réseau local.",
    sim_kinds: ["Photo", "Vidéo", "PDF", "Musique"],
    sim_pick: "Type de fichier",
    sim_send: "Envoyer",
    sim_sending: "Envoi en cours…",
    sim_done: "Reçu !",
    sim_again: "Envoyer un autre fichier",
    sim_from: "Mon iPhone",
    sim_to: "iPhone de Léa",
    sim_network: "Wi‑Fi de la maison",
    sim_note: "Démonstration : rien ne quitte votre navigateur.",
    feat_label: "Atouts",
    feat_title: "Simple comme un QR code.",
    feat_lead: "Tout ce qu'il faut pour partager un fichier avec la personne en face de vous, et rien de plus.",
    features: [
        item(
            "Aucun compte",
            "Pas d'inscription, pas d'e‑mail, pas de mot de passe. Vous ouvrez l'app, vous partagez.",
        ),
        item(
            "Aucun cloud",
            "Vos fichiers ne sont jamais déposés sur un serveur : ils vont d'un téléphone à l'autre.",
        ),
        item(
            "Connexion par QR code",
            "Un scan suffit pour relier deux appareils. Fini les codes d'appairage et les liens à copier.",
        ),
        item(
            "Réseau local",
            "Le transfert passe par votre Wi‑Fi ou un partage de connexion : il fonctionne même sans accès à Internet.",
        ),
        item(
            "Tous les fichiers",
            "Photos, vidéos, documents, musique : ce qui est sur votre téléphone peut partir.",
        ),
        item(
            "Natif sur iPhone",
            "Une application écrite en Swift, pensée pour iOS : rapide, légère et agréable.",
        ),
    ],
    priv_label: "Confidentialité",
    priv_title: "Vos fichiers ne font pas le tour du monde.",
    priv_lead: "Un service de partage en ligne dépose vos fichiers sur ses serveurs avant de les rendre au destinataire. LOCALINK prend le chemin le plus court.",
    priv_cloud: "Avec le cloud",
    priv_cloud_path: ["Votre téléphone", "Serveur distant", "Son téléphone"],
    priv_local: "Avec LOCALINK",
    priv_local_path: ["Votre téléphone", "Son téléphone"],
    priv_points: [
        "Aucune copie de vos fichiers ne reste sur un serveur.",
        "Personne n'a besoin de créer un compte ou de donner ses coordonnées.",
        "Le QR code sert uniquement à relier les deux appareils.",
    ],
    num_label: "En chiffres",
    num_title: "Le partage, réduit à l'essentiel.",
    numbers: [
        figure(0, "", "compte à créer"),
        figure(0, "", "fichier dans le cloud"),
        figure(1, "", "QR code pour se connecter"),
        figure(100, "%", "sur votre réseau local"),
    ],
    app_label: "iPhone",
    app_title: "Une app native, écrite en Swift.",
    app_lead: "LOCALINK sur iPhone est développée en Swift, au plus près d'iOS : une interface fluide, un appareil photo prêt à scanner, et vos fichiers à portée de main.",
    app_points: [
        "Scan du QR code avec l'appareil photo",
        "Envoi de photos, vidéos et fichiers",
        "Progression du transfert en temps réel",
        "Aucun compte, aucune publicité",
    ],
    app_store: "Bientôt sur l'App Store",
    app_store_small: "Disponible prochainement",
    app_swift: "Swift · iOS",
    app_screen_title: "À proximité",
    app_screen_nearby: "Prêt à recevoir",
    faq_label: "FAQ",
    faq_title: "Questions fréquentes.",
    faq: [
        item(
            "Faut-il créer un compte ?",
            "Non. LOCALINK fonctionne sans inscription, sans e‑mail et sans mot de passe.",
        ),
        item(
            "Mes fichiers passent-ils par un serveur ?",
            "Non. Ils sont envoyés directement d'un téléphone à l'autre sur votre réseau local, sans stockage dans le cloud.",
        ),
        item(
            "Les deux téléphones doivent-ils être sur le même réseau ?",
            "Oui : le même Wi‑Fi, ou le partage de connexion de l'un des deux téléphones.",
        ),
        item(
            "Faut-il une connexion Internet ?",
            "Non : le transfert reste sur le réseau local. Un Wi‑Fi sans accès à Internet ou un partage de connexion suffit.",
        ),
        item(
            "À quoi sert le QR code ?",
            "Il permet aux deux appareils de se trouver sur le réseau et d'établir la connexion, sans rien taper.",
        ),
        item(
            "Quels fichiers puis-je envoyer ?",
            "Photos, vidéos, documents, musique : tous les fichiers présents sur votre téléphone.",
        ),
    ],
    cta_title: "Partagez comme vous parlez : en face à face.",
    cta_lead: "Un QR code, un réseau local, et vos fichiers changent de téléphone.",
    cta_button: "Voir comment ça marche",
    footer_tagline: "Transfert de fichiers entre téléphones sur le réseau local. Un QR code pour se connecter, sans compte et sans stockage cloud.",
    footer_product: "Produit",
    footer_resources: "Ressources",
    footer_legal: "Légal",
    footer_contact: "Contact",
    footer_contact_text: "Une question, une idée ? Écrivez-nous sur GitHub.",
    footer_contact_link: "Ouvrir une discussion",
    footer_source: "Code source",
    footer_paquets: "Construit avec Paquets",
    footer_sitemap: "Plan du site",
    footer_press: "Logo (SVG)",
    footer_install: "Installer le site",
    footer_install_hint: "Ajoutez LOCALINK à votre écran d'accueil : le site fonctionne aussi hors ligne.",
    footer_installed: "Installé ✓",
    footer_offline: "Disponible hors ligne",
    footer_top: "Haut de page",
    footer_rights: "Tous droits réservés.",
    footer_made: "Conçu en Rust, de la page aux animations.",
    footer_zero: ["0 cookie", "0 traceur", "0 compte"],
    footer_hosted: "Hébergé sur Heroku et GitHub Pages",
    footer_status: "En ligne",
    updated: "Mis à jour le 24 novembre 2026",
    privacy: Legal {
        title: "Confidentialité",
        lead: "LOCALINK est conçu pour ne rien savoir de vous. Voici, simplement, ce que fait ce site et ce que fait l'application.",
        sections: &[
            item(
                "Ce site",
                "Ce site ne dépose aucun cookie, n'utilise aucun outil de mesure d'audience et ne charge aucune ressource tierce : les polices et les images sont servies par le site lui-même.",
            ),
            item(
                "Données techniques",
                "Comme tout site web, l'hébergeur (Heroku ou GitHub Pages) peut enregistrer temporairement l'adresse IP et les requêtes reçues, pour assurer la sécurité et le bon fonctionnement du service.",
            ),
            item(
                "L'application",
                "L'application ne demande aucun compte. Les fichiers sont envoyés directement d'un appareil à l'autre sur votre réseau local et ne sont stockés sur aucun serveur. Le QR code sert uniquement à établir la connexion entre les deux appareils.",
            ),
            item(
                "Stockage sur votre appareil",
                "Installé sur votre écran d'accueil, ce site garde une copie de ses pages dans le cache de votre navigateur pour fonctionner hors ligne. Vous pouvez la supprimer à tout moment dans les réglages du navigateur.",
            ),
            item(
                "Vos droits",
                "Aucune donnée personnelle n'étant collectée, il n'y a rien à consulter, rectifier ou supprimer. Pour toute question, contactez-nous via la page GitHub du projet.",
            ),
        ],
    },
    terms: Legal {
        title: "Conditions d'utilisation",
        lead: "Les règles simples qui encadrent l'utilisation de ce site et de l'application LOCALINK.",
        sections: &[
            item(
                "Objet",
                "Ce site présente LOCALINK, une application de transfert de fichiers entre téléphones sur un réseau local.",
            ),
            item(
                "Accès",
                "Le site est accessible gratuitement, sans inscription. Nous faisons de notre mieux pour qu'il soit disponible, sans pouvoir le garantir en permanence.",
            ),
            item(
                "Utilisation de l'application",
                "Vous restez responsable des fichiers que vous envoyez et devez disposer des droits nécessaires pour les partager. N'envoyez des fichiers qu'aux personnes qui souhaitent les recevoir.",
            ),
            item(
                "Propriété intellectuelle",
                "Le nom LOCALINK, le logo, les textes et le code de ce site appartiennent à leur auteur. Toute reproduction sans autorisation est interdite.",
            ),
            item(
                "Responsabilité",
                "Le site et l'application sont fournis « en l'état ». Leur auteur ne saurait être tenu responsable d'une perte de données ou d'un usage contraire à ces conditions.",
            ),
            item(
                "Droit applicable",
                "Ces conditions sont régies par le droit français.",
            ),
        ],
    },
    legal: Legal {
        title: "Mentions légales",
        lead: "Les informations sur l'éditeur et les hébergeurs de ce site.",
        sections: &[
            item(
                "Éditeur",
                "Maxime Nathan Lestage, éditeur et directeur de la publication. Contact : via la page GitHub du projet LOCALINK.",
            ),
            item(
                "Hébergement",
                "Heroku, service de Salesforce, Inc., 415 Mission Street, 3rd Floor, San Francisco, CA 94105, États-Unis. GitHub Pages, service de GitHub, Inc., 88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, États-Unis.",
            ),
            item(
                "Accessibilité",
                "Le site respecte le réglage « réduire les animations » de votre appareil, se parcourt au clavier et s'adapte à toutes les tailles d'écran, à commencer par le téléphone.",
            ),
            item(
                "Crédits",
                "Site écrit en Rust avec le framework Paquets. Polices Space Grotesk, Inter et JetBrains Mono, sous licence SIL Open Font License.",
            ),
        ],
    },
    nf_title: "Connexion perdue.",
    nf_text: "Cette page n'est pas sur le réseau. Elle a peut-être changé d'adresse.",
    nf_back: "Revenir à l'accueil",
    pick_title: "Choisissez votre langue",
    pick_text: "Transfert de fichiers entre téléphones, sans compte ni cloud.",
};

// ----- English ----------------------------------------------------------------------

pub static EN: Texts = Texts {
    meta_title: "LOCALINK — File transfer between phones, no account, no cloud",
    meta_description: "LOCALINK moves your files from one phone to another over your local network. A QR code to connect, no account, no cloud storage.",
    skip: "Skip to content",
    nav_how: "How it works",
    nav_features: "Features",
    nav_privacy: "Privacy",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_home: "Home",
    nav_menu: "Menu",
    nav_cta: "Discover",
    loader: "Joining the local network…",
    lang_label: "Language",
    hero_eyebrow: "Transfer",
    hero_title: ["Your files,", "from one phone to another."],
    hero_send: "Send your",
    hero_words: ["photos", "videos", "documents", "music"],
    hero_lead: "LOCALINK sends your files straight between two phones on the same network. Scan a QR code to connect and you're done: no account, no cloud, no cable.",
    hero_primary: "How it works",
    hero_secondary: "The iPhone app",
    hero_badges: ["No account", "No cloud", "Local network"],
    hero_scroll: "Scroll",
    phone_sender: "Sending",
    phone_receiver: "Receiving",
    phone_scan: "Scan to connect",
    phone_connected: "Connected",
    phone_received: "Received",
    files: ["holidays.jpg", "concert.mov", "contract.pdf"],
    marquee: [
        "No account",
        "No cloud",
        "One QR code",
        "Local network",
        "Device to device",
        "Any kind of file",
    ],
    how_label: "How it works",
    how_title: "Three steps, zero sign-up.",
    how_lead: "No username to remember, no link to copy: the two phones find each other with a QR code, then talk directly.",
    steps: [
        item(
            "Open LOCALINK",
            "On both phones, connected to the same Wi‑Fi network or to the hotspot of one of them.",
        ),
        item(
            "Scan the QR code",
            "The receiving phone shows a QR code. The other one scans it: you're connected, with no code to type.",
        ),
        item(
            "Send",
            "Pick your files: they go straight from one device to the other, without a detour through a server.",
        ),
    ],
    qr_caption: "A real QR code: scan it to open this site on your phone.",
    sim_label: "Try it",
    sim_title: "A transfer, live.",
    sim_lead: "Pick a file and send it from one phone to the other, over the local network.",
    sim_kinds: ["Photo", "Video", "PDF", "Music"],
    sim_pick: "File type",
    sim_send: "Send",
    sim_sending: "Sending…",
    sim_done: "Received!",
    sim_again: "Send another file",
    sim_from: "My iPhone",
    sim_to: "Lea's iPhone",
    sim_network: "Home Wi‑Fi",
    sim_note: "Demo: nothing leaves your browser.",
    feat_label: "Features",
    feat_title: "As simple as a QR code.",
    feat_lead: "Everything you need to share a file with the person in front of you, and nothing more.",
    features: [
        item(
            "No account",
            "No sign-up, no email, no password. Open the app and share.",
        ),
        item(
            "No cloud",
            "Your files are never uploaded to a server: they go from one phone to the other.",
        ),
        item(
            "QR code pairing",
            "One scan links two devices. No more pairing codes or links to copy.",
        ),
        item(
            "Local network",
            "Transfers go through your Wi‑Fi or a hotspot: they work even without Internet access.",
        ),
        item(
            "Every file",
            "Photos, videos, documents, music: whatever is on your phone can go.",
        ),
        item(
            "Native on iPhone",
            "An app written in Swift, made for iOS: fast, light and pleasant.",
        ),
    ],
    priv_label: "Privacy",
    priv_title: "Your files don't travel around the world.",
    priv_lead: "An online sharing service uploads your files to its servers before handing them to the recipient. LOCALINK takes the shortest path.",
    priv_cloud: "With the cloud",
    priv_cloud_path: ["Your phone", "Remote server", "Their phone"],
    priv_local: "With LOCALINK",
    priv_local_path: ["Your phone", "Their phone"],
    priv_points: [
        "No copy of your files is left on a server.",
        "Nobody has to create an account or share their details.",
        "The QR code only links the two devices.",
    ],
    num_label: "In numbers",
    num_title: "Sharing, down to the essentials.",
    numbers: [
        figure(0, "", "account to create"),
        figure(0, "", "file in the cloud"),
        figure(1, "", "QR code to connect"),
        figure(100, "%", "on your local network"),
    ],
    app_label: "iPhone",
    app_title: "A native app, written in Swift.",
    app_lead: "LOCALINK for iPhone is built in Swift, close to iOS: a smooth interface, a camera ready to scan, and your files at hand.",
    app_points: [
        "Scan the QR code with the camera",
        "Send photos, videos and files",
        "Live transfer progress",
        "No account, no ads",
    ],
    app_store: "Coming soon to the App Store",
    app_store_small: "Available soon",
    app_swift: "Swift · iOS",
    app_screen_title: "Nearby",
    app_screen_nearby: "Ready to receive",
    faq_label: "FAQ",
    faq_title: "Frequently asked questions.",
    faq: [
        item(
            "Do I need an account?",
            "No. LOCALINK works without sign-up, email or password.",
        ),
        item(
            "Do my files go through a server?",
            "No. They are sent straight from one phone to the other over your local network, with no cloud storage.",
        ),
        item(
            "Do both phones need to be on the same network?",
            "Yes: the same Wi‑Fi, or the hotspot of one of the two phones.",
        ),
        item(
            "Do I need an Internet connection?",
            "No: the transfer stays on the local network. A Wi‑Fi without Internet access or a hotspot is enough.",
        ),
        item(
            "What is the QR code for?",
            "It lets the two devices find each other on the network and connect, without typing anything.",
        ),
        item(
            "Which files can I send?",
            "Photos, videos, documents, music: any file on your phone.",
        ),
    ],
    cta_title: "Share the way you talk: face to face.",
    cta_lead: "One QR code, one local network, and your files change phones.",
    cta_button: "See how it works",
    footer_tagline: "File transfer between phones over the local network. A QR code to connect, no account and no cloud storage.",
    footer_product: "Product",
    footer_resources: "Resources",
    footer_legal: "Legal",
    footer_contact: "Contact",
    footer_contact_text: "A question, an idea? Write to us on GitHub.",
    footer_contact_link: "Start a discussion",
    footer_source: "Source code",
    footer_paquets: "Built with Paquets",
    footer_sitemap: "Sitemap",
    footer_press: "Logo (SVG)",
    footer_install: "Install the site",
    footer_install_hint: "Add LOCALINK to your home screen: the site works offline too.",
    footer_installed: "Installed ✓",
    footer_offline: "Available offline",
    footer_top: "Back to top",
    footer_rights: "All rights reserved.",
    footer_made: "Made in Rust, from the pages to the animations.",
    footer_zero: ["0 cookies", "0 trackers", "0 accounts"],
    footer_hosted: "Hosted on Heroku and GitHub Pages",
    footer_status: "Online",
    updated: "Updated on November 24, 2026",
    privacy: Legal {
        title: "Privacy",
        lead: "LOCALINK is designed to know nothing about you. Here is, simply, what this site does and what the app does.",
        sections: &[
            item(
                "This site",
                "This site sets no cookies, uses no analytics and loads no third-party resources: fonts and images are served by the site itself.",
            ),
            item(
                "Technical data",
                "Like any website, the host (Heroku or GitHub Pages) may temporarily log the IP address and the requests it receives, to keep the service secure and running.",
            ),
            item(
                "The app",
                "The app asks for no account. Files are sent straight from one device to the other over your local network and are stored on no server. The QR code is only used to connect the two devices.",
            ),
            item(
                "Storage on your device",
                "Once installed on your home screen, this site keeps a copy of its pages in your browser's cache to work offline. You can delete it at any time in your browser settings.",
            ),
            item(
                "Your rights",
                "Since no personal data is collected, there is nothing to access, correct or delete. For any question, contact us through the project's GitHub page.",
            ),
        ],
    },
    terms: Legal {
        title: "Terms of use",
        lead: "The simple rules for using this site and the LOCALINK app.",
        sections: &[
            item(
                "Purpose",
                "This site presents LOCALINK, an app to transfer files between phones over a local network.",
            ),
            item(
                "Access",
                "The site is free and requires no sign-up. We do our best to keep it available, without being able to guarantee it at all times.",
            ),
            item(
                "Using the app",
                "You remain responsible for the files you send and must hold the rights needed to share them. Only send files to people who want to receive them.",
            ),
            item(
                "Intellectual property",
                "The LOCALINK name, the logo, the texts and the code of this site belong to their author. Any reproduction without permission is forbidden.",
            ),
            item(
                "Liability",
                "The site and the app are provided “as is”. Their author cannot be held liable for any data loss or for any use contrary to these terms.",
            ),
            item("Governing law", "These terms are governed by French law."),
        ],
    },
    legal: Legal {
        title: "Legal notice",
        lead: "Information about the publisher and the hosts of this site.",
        sections: &[
            item(
                "Publisher",
                "Maxime Nathan Lestage, publisher and director of publication. Contact: through the GitHub page of the LOCALINK project.",
            ),
            item(
                "Hosting",
                "Heroku, a service of Salesforce, Inc., 415 Mission Street, 3rd Floor, San Francisco, CA 94105, USA. GitHub Pages, a service of GitHub, Inc., 88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, USA.",
            ),
            item(
                "Accessibility",
                "The site follows your device's “reduce motion” setting, can be used with a keyboard and adapts to every screen size, starting with phones.",
            ),
            item(
                "Credits",
                "Site written in Rust with the Paquets framework. Space Grotesk, Inter and JetBrains Mono fonts, under the SIL Open Font License.",
            ),
        ],
    },
    nf_title: "Connection lost.",
    nf_text: "This page is not on the network. It may have moved.",
    nf_back: "Back to the home page",
    pick_title: "Choose your language",
    pick_text: "File transfer between phones, no account, no cloud.",
};

// ----- Español ----------------------------------------------------------------------

pub static ES: Texts = Texts {
    meta_title: "LOCALINK — Transferencia de archivos entre teléfonos, sin cuenta ni nube",
    meta_description: "LOCALINK pasa tus archivos de un teléfono a otro por tu red local. Un código QR para conectarse, sin cuenta y sin almacenamiento en la nube.",
    skip: "Ir al contenido",
    nav_how: "Cómo funciona",
    nav_features: "Ventajas",
    nav_privacy: "Privacidad",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_home: "Inicio",
    nav_menu: "Menú",
    nav_cta: "Descubrir",
    loader: "Conectando a la red local…",
    lang_label: "Idioma",
    hero_eyebrow: "Transferencia",
    hero_title: ["Tus archivos,", "de un teléfono a otro."],
    hero_send: "Envía tus",
    hero_words: ["fotos", "vídeos", "documentos", "canciones"],
    hero_lead: "LOCALINK envía tus archivos directamente entre dos teléfonos de la misma red. Escanea un código QR para conectarte y listo: sin cuenta, sin nube, sin cable.",
    hero_primary: "Cómo funciona",
    hero_secondary: "La app para iPhone",
    hero_badges: ["Sin cuenta", "Sin nube", "Red local"],
    hero_scroll: "Desliza",
    phone_sender: "Envío",
    phone_receiver: "Recepción",
    phone_scan: "Escanea para conectarte",
    phone_connected: "Conectado",
    phone_received: "Recibido",
    files: ["vacaciones.jpg", "concierto.mov", "contrato.pdf"],
    marquee: [
        "Sin cuenta",
        "Sin nube",
        "Un código QR",
        "Red local",
        "De dispositivo a dispositivo",
        "Cualquier tipo de archivo",
    ],
    how_label: "Cómo funciona",
    how_title: "Tres gestos, cero registros.",
    how_lead: "Ningún usuario que recordar, ningún enlace que copiar: los dos teléfonos se encuentran con un código QR y luego se comunican directamente.",
    steps: [
        item(
            "Abre LOCALINK",
            "En los dos teléfonos, conectados a la misma red Wi‑Fi o al punto de acceso de uno de ellos.",
        ),
        item(
            "Escanea el código QR",
            "El teléfono que recibe muestra un código QR. El otro lo escanea: ya estáis conectados, sin teclear ningún código.",
        ),
        item(
            "Envía",
            "Elige tus archivos: pasan directamente de un dispositivo a otro, sin pasar por un servidor.",
        ),
    ],
    qr_caption: "Un código QR de verdad: escanéalo para abrir este sitio en tu teléfono.",
    sim_label: "Pruébalo",
    sim_title: "Una transferencia, en directo.",
    sim_lead: "Elige un archivo y envíalo de un teléfono a otro, por la red local.",
    sim_kinds: ["Foto", "Vídeo", "PDF", "Música"],
    sim_pick: "Tipo de archivo",
    sim_send: "Enviar",
    sim_sending: "Enviando…",
    sim_done: "¡Recibido!",
    sim_again: "Enviar otro archivo",
    sim_from: "Mi iPhone",
    sim_to: "iPhone de Lea",
    sim_network: "Wi‑Fi de casa",
    sim_note: "Demostración: nada sale de tu navegador.",
    feat_label: "Ventajas",
    feat_title: "Tan sencillo como un código QR.",
    feat_lead: "Todo lo necesario para compartir un archivo con la persona que tienes delante, y nada más.",
    features: [
        item(
            "Sin cuenta",
            "Sin registro, sin correo, sin contraseña. Abres la app y compartes.",
        ),
        item(
            "Sin nube",
            "Tus archivos nunca se suben a un servidor: van de un teléfono a otro.",
        ),
        item(
            "Conexión por código QR",
            "Un escaneo basta para unir dos dispositivos. Adiós a los códigos de emparejamiento y a los enlaces.",
        ),
        item(
            "Red local",
            "La transferencia pasa por tu Wi‑Fi o un punto de acceso: funciona incluso sin acceso a Internet.",
        ),
        item(
            "Todos los archivos",
            "Fotos, vídeos, documentos, música: todo lo que está en tu teléfono puede enviarse.",
        ),
        item(
            "Nativa en iPhone",
            "Una app escrita en Swift, pensada para iOS: rápida, ligera y agradable.",
        ),
    ],
    priv_label: "Privacidad",
    priv_title: "Tus archivos no dan la vuelta al mundo.",
    priv_lead: "Un servicio para compartir en línea sube tus archivos a sus servidores antes de entregarlos al destinatario. LOCALINK toma el camino más corto.",
    priv_cloud: "Con la nube",
    priv_cloud_path: ["Tu teléfono", "Servidor remoto", "Su teléfono"],
    priv_local: "Con LOCALINK",
    priv_local_path: ["Tu teléfono", "Su teléfono"],
    priv_points: [
        "Ninguna copia de tus archivos se queda en un servidor.",
        "Nadie tiene que crear una cuenta ni dar sus datos.",
        "El código QR solo sirve para unir los dos dispositivos.",
    ],
    num_label: "En cifras",
    num_title: "Compartir, reducido a lo esencial.",
    numbers: [
        figure(0, "", "cuentas que crear"),
        figure(0, "", "archivos en la nube"),
        figure(1, "", "código QR para conectarse"),
        figure(100, "%", "en tu red local"),
    ],
    app_label: "iPhone",
    app_title: "Una app nativa, escrita en Swift.",
    app_lead: "LOCALINK para iPhone está desarrollada en Swift, muy cerca de iOS: una interfaz fluida, una cámara lista para escanear y tus archivos a mano.",
    app_points: [
        "Escaneo del código QR con la cámara",
        "Envío de fotos, vídeos y archivos",
        "Progreso de la transferencia en tiempo real",
        "Sin cuenta, sin publicidad",
    ],
    app_store: "Próximamente en el App Store",
    app_store_small: "Disponible pronto",
    app_swift: "Swift · iOS",
    app_screen_title: "Cerca",
    app_screen_nearby: "Listo para recibir",
    faq_label: "FAQ",
    faq_title: "Preguntas frecuentes.",
    faq: [
        item(
            "¿Hay que crear una cuenta?",
            "No. LOCALINK funciona sin registro, sin correo y sin contraseña.",
        ),
        item(
            "¿Mis archivos pasan por un servidor?",
            "No. Se envían directamente de un teléfono a otro por tu red local, sin almacenamiento en la nube.",
        ),
        item(
            "¿Los dos teléfonos deben estar en la misma red?",
            "Sí: la misma Wi‑Fi, o el punto de acceso de uno de los dos teléfonos.",
        ),
        item(
            "¿Hace falta conexión a Internet?",
            "No: la transferencia se queda en la red local. Basta una Wi‑Fi sin acceso a Internet o un punto de acceso.",
        ),
        item(
            "¿Para qué sirve el código QR?",
            "Permite que los dos dispositivos se encuentren en la red y se conecten, sin teclear nada.",
        ),
        item(
            "¿Qué archivos puedo enviar?",
            "Fotos, vídeos, documentos, música: cualquier archivo de tu teléfono.",
        ),
    ],
    cta_title: "Comparte como hablas: cara a cara.",
    cta_lead: "Un código QR, una red local, y tus archivos cambian de teléfono.",
    cta_button: "Ver cómo funciona",
    footer_tagline: "Transferencia de archivos entre teléfonos por la red local. Un código QR para conectarse, sin cuenta y sin almacenamiento en la nube.",
    footer_product: "Producto",
    footer_resources: "Recursos",
    footer_legal: "Legal",
    footer_contact: "Contacto",
    footer_contact_text: "¿Una pregunta, una idea? Escríbenos en GitHub.",
    footer_contact_link: "Abrir una conversación",
    footer_source: "Código fuente",
    footer_paquets: "Hecho con Paquets",
    footer_sitemap: "Mapa del sitio",
    footer_press: "Logotipo (SVG)",
    footer_install: "Instalar el sitio",
    footer_install_hint: "Añade LOCALINK a tu pantalla de inicio: el sitio también funciona sin conexión.",
    footer_installed: "Instalado ✓",
    footer_offline: "Disponible sin conexión",
    footer_top: "Volver arriba",
    footer_rights: "Todos los derechos reservados.",
    footer_made: "Hecho en Rust, de las páginas a las animaciones.",
    footer_zero: ["0 cookies", "0 rastreadores", "0 cuentas"],
    footer_hosted: "Alojado en Heroku y GitHub Pages",
    footer_status: "En línea",
    updated: "Actualizado el 24 de noviembre de 2026",
    privacy: Legal {
        title: "Privacidad",
        lead: "LOCALINK está pensado para no saber nada de ti. Esto es, sencillamente, lo que hace este sitio y lo que hace la app.",
        sections: &[
            item(
                "Este sitio",
                "Este sitio no instala cookies, no usa herramientas de medición de audiencia y no carga recursos de terceros: las fuentes y las imágenes las sirve el propio sitio.",
            ),
            item(
                "Datos técnicos",
                "Como cualquier sitio web, el alojamiento (Heroku o GitHub Pages) puede registrar temporalmente la dirección IP y las peticiones recibidas, para mantener el servicio seguro y en funcionamiento.",
            ),
            item(
                "La app",
                "La app no pide ninguna cuenta. Los archivos se envían directamente de un dispositivo a otro por tu red local y no se guardan en ningún servidor. El código QR solo sirve para conectar los dos dispositivos.",
            ),
            item(
                "Almacenamiento en tu dispositivo",
                "Instalado en tu pantalla de inicio, este sitio guarda una copia de sus páginas en la caché del navegador para funcionar sin conexión. Puedes borrarla cuando quieras en los ajustes del navegador.",
            ),
            item(
                "Tus derechos",
                "Como no se recoge ningún dato personal, no hay nada que consultar, corregir o borrar. Para cualquier pregunta, contáctanos a través de la página de GitHub del proyecto.",
            ),
        ],
    },
    terms: Legal {
        title: "Condiciones de uso",
        lead: "Las reglas sencillas para usar este sitio y la app LOCALINK.",
        sections: &[
            item(
                "Objeto",
                "Este sitio presenta LOCALINK, una app para transferir archivos entre teléfonos por una red local.",
            ),
            item(
                "Acceso",
                "El sitio es gratuito y no requiere registro. Hacemos todo lo posible para que esté disponible, sin poder garantizarlo en todo momento.",
            ),
            item(
                "Uso de la app",
                "Eres responsable de los archivos que envías y debes tener los derechos necesarios para compartirlos. Envía archivos solo a quien quiera recibirlos.",
            ),
            item(
                "Propiedad intelectual",
                "El nombre LOCALINK, el logotipo, los textos y el código de este sitio pertenecen a su autor. Queda prohibida cualquier reproducción sin autorización.",
            ),
            item(
                "Responsabilidad",
                "El sitio y la app se ofrecen «tal cual». Su autor no se hace responsable de ninguna pérdida de datos ni de un uso contrario a estas condiciones.",
            ),
            item(
                "Ley aplicable",
                "Estas condiciones se rigen por la ley francesa.",
            ),
        ],
    },
    legal: Legal {
        title: "Aviso legal",
        lead: "La información sobre el editor y los alojamientos de este sitio.",
        sections: &[
            item(
                "Editor",
                "Maxime Nathan Lestage, editor y director de la publicación. Contacto: a través de la página de GitHub del proyecto LOCALINK.",
            ),
            item(
                "Alojamiento",
                "Heroku, un servicio de Salesforce, Inc., 415 Mission Street, 3rd Floor, San Francisco, CA 94105, EE. UU. GitHub Pages, un servicio de GitHub, Inc., 88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, EE. UU.",
            ),
            item(
                "Accesibilidad",
                "El sitio respeta el ajuste «reducir movimiento» de tu dispositivo, se puede usar con el teclado y se adapta a todos los tamaños de pantalla, empezando por el teléfono.",
            ),
            item(
                "Créditos",
                "Sitio escrito en Rust con el framework Paquets. Fuentes Space Grotesk, Inter y JetBrains Mono, con licencia SIL Open Font License.",
            ),
        ],
    },
    nf_title: "Conexión perdida.",
    nf_text: "Esta página no está en la red. Puede que haya cambiado de dirección.",
    nf_back: "Volver al inicio",
    pick_title: "Elige tu idioma",
    pick_text: "Transferencia de archivos entre teléfonos, sin cuenta ni nube.",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_preferred_language() {
        assert_eq!(pick_lang("es-ES,es;q=0.9,en;q=0.8"), Lang::Es);
        assert_eq!(pick_lang("en-US,en;q=0.9"), Lang::En);
        assert_eq!(pick_lang("de-DE,en;q=0.5,fr;q=0.7"), Lang::Fr);
        assert_eq!(pick_lang("de"), Lang::Fr);
        assert_eq!(pick_lang(""), Lang::Fr);
    }

    #[test]
    fn every_language_has_its_own_texts() {
        for lang in Lang::ALL {
            assert_eq!(Lang::parse(lang.code()), Some(lang));
            let t = lang.texts();
            assert!(!t.meta_title.is_empty());
            assert!(t.meta_description.len() <= 170, "{}", t.meta_description);
            for legal in [t.privacy, t.terms, t.legal] {
                assert!(legal.sections.len() >= 4);
            }
        }
        assert_ne!(FR.hero_lead, EN.hero_lead);
        assert_ne!(EN.hero_lead, ES.hero_lead);
        assert_ne!(FR.hero_lead, ES.hero_lead);
    }
}
