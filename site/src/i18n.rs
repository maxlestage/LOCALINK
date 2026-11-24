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
    pub nav_share: &'static str,
    pub nav_try: &'static str,
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
    pub phone_code: &'static str,
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
    // Ephemeral sharing
    pub share_label: &'static str,
    pub share_title: &'static str,
    pub share_lead: &'static str,
    pub share_code_label: &'static str,
    pub share_choose: &'static str,
    pub share_tab_code: &'static str,
    pub share_tab_qr: &'static str,
    pub share_new: &'static str,
    pub share_copy: &'static str,
    pub share_copied: &'static str,
    pub share_expires: &'static str,
    pub share_erased: &'static str,
    pub share_again: &'static str,
    pub share_server: &'static str,
    pub share_demo: &'static str,
    pub share_steps: [Item; 3],
    // Features
    pub feat_label: &'static str,
    pub feat_title: &'static str,
    pub feat_lead: &'static str,
    pub features: [Item; 8],
    // Privacy
    pub priv_label: &'static str,
    pub priv_title: &'static str,
    pub priv_lead: &'static str,
    pub priv_cloud: &'static str,
    pub priv_cloud_path: [&'static str; 3],
    pub priv_local: &'static str,
    pub priv_local_path: [&'static str; 2],
    pub priv_remote: &'static str,
    pub priv_remote_path: [&'static str; 3],
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
    pub faq: [Item; 8],
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
    meta_title: "LOCALINK — Transfert de fichiers entre téléphones, près ou loin, par QR code ou par code",
    meta_description: "Envoyez vos fichiers d'un téléphone à l'autre, sur le même réseau ou à distance. QR code ou code au choix, sans compte, effacés du serveur après 2 minutes.",
    skip: "Aller au contenu",
    nav_how: "Fonctionnement",
    nav_features: "Atouts",
    nav_privacy: "Confidentialité",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_share: "Partage 2 min",
    nav_try: "Essayer",
    nav_home: "Accueil",
    nav_menu: "Menu",
    nav_cta: "Découvrir",
    loader: "Mise en relation…",
    lang_label: "Langue",
    hero_eyebrow: "Transfert",
    hero_title: ["Vos fichiers,", "d'un téléphone à l'autre."],
    hero_send: "Envoyez vos",
    hero_words: ["photos", "vidéos", "documents", "musiques"],
    hero_lead: "LOCALINK envoie vos fichiers d'un téléphone à l'autre, où que vous soyez. Sur le même réseau, ils passent en direct ; à distance, par le serveur, qui les efface au bout de 2 minutes. Pour vous connecter, choisissez : QR code ou code.",
    hero_primary: "Comment ça marche",
    hero_secondary: "L'app iPhone",
    hero_badges: ["Sans compte", "Près ou loin", "Effacé en 2 min"],
    hero_scroll: "Défiler",
    phone_sender: "Envoi",
    phone_receiver: "Réception",
    phone_scan: "Scannez pour vous connecter",
    phone_code: "ou tapez le code",
    phone_connected: "Connecté",
    phone_received: "Reçu",
    files: ["vacances.jpg", "concert.mov", "contrat.pdf"],
    marquee: [
        "Sans compte",
        "QR code ou code",
        "Près ou loin",
        "Effacé après 2 minutes",
        "Direct en réseau local",
        "Tous types de fichiers",
    ],
    how_label: "Fonctionnement",
    how_title: "Trois gestes, zéro inscription.",
    how_lead: "Pas d'identifiant à retenir : les deux téléphones se relient par QR code ou par code, au choix, puis échangent.",
    steps: [
        item(
            "Ouvrez LOCALINK",
            "Sur les deux téléphones, qu'ils soient côte à côte ou à l'autre bout du pays.",
        ),
        item(
            "QR code ou code, au choix",
            "Le téléphone qui reçoit affiche son QR code et son code LOCALINK. L'autre choisit : scanner le QR code ou taper le code.",
        ),
        item(
            "Envoyez",
            "Sur le même réseau, les fichiers passent directement d'un appareil à l'autre. À distance, ils transitent par le serveur et s'effacent après 2 minutes.",
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
    share_label: "Partage éphémère",
    share_title: "À distance, effacé en 2 minutes.",
    share_lead: "Pas à côté l'un de l'autre ? Déposez votre fichier sur le serveur LOCALINK et partagez, au choix, votre code ou votre QR code. Votre contact le récupère, puis tout s'efface automatiquement au bout de 2 minutes.",
    share_code_label: "Mon code LOCALINK",
    share_choose: "Se connecter avec",
    share_tab_code: "Code",
    share_tab_qr: "QR code",
    share_new: "Nouveau code",
    share_copy: "Copier le code",
    share_copied: "Code copié",
    share_expires: "S'efface dans",
    share_erased: "Effacé du serveur",
    share_again: "Partager à nouveau",
    share_server: "Serveur LOCALINK",
    share_demo: "Démonstration accélérée : 2 minutes en 12 secondes.",
    share_steps: [
        item(
            "Déposez",
            "Choisissez un fichier : il part sur le serveur LOCALINK.",
        ),
        item(
            "Donnez votre code",
            "De vive voix ou en QR code : votre contact le tape et récupère le fichier.",
        ),
        item(
            "Tout s'efface",
            "2 minutes plus tard, le partage est supprimé du serveur, automatiquement.",
        ),
    ],
    feat_label: "Atouts",
    feat_title: "Simple comme un QR code.",
    feat_lead: "Tout ce qu'il faut pour partager un fichier avec quelqu'un, à côté de vous ou loin, et rien de plus.",
    features: [
        item(
            "Aucun compte",
            "Pas d'inscription, pas d'e‑mail, pas de mot de passe. Vous ouvrez l'app, vous partagez.",
        ),
        item(
            "Connexion par QR code",
            "Un scan suffit pour relier deux appareils, sans rien taper.",
        ),
        item(
            "Connexion par code",
            "Pas envie de scanner ? Choisissez le code : tapez celui de l'autre téléphone et vous êtes connectés.",
        ),
        item(
            "En direct sur le réseau",
            "Sur le même Wi‑Fi ou un partage de connexion, le transfert passe directement d'un téléphone à l'autre, sans serveur.",
        ),
        item(
            "Partage à distance",
            "Loin l'un de l'autre ? Le fichier passe par le serveur LOCALINK et s'efface automatiquement au bout de 2 minutes.",
        ),
        item(
            "Rien n'est conservé",
            "Pas d'historique, pas de galerie en ligne, pas de stockage cloud : rien ne reste derrière vous.",
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
    priv_title: "Vos fichiers ne s'éternisent nulle part.",
    priv_lead: "Un service de partage en ligne garde vos fichiers sur ses serveurs aussi longtemps qu'il le veut. LOCALINK passe en direct sur votre réseau, ou ne garde votre partage que 2 minutes.",
    priv_cloud: "Avec le cloud",
    priv_cloud_path: ["Votre téléphone", "Serveur · sans limite", "Son téléphone"],
    priv_local: "En local avec LOCALINK",
    priv_local_path: ["Votre téléphone", "Son téléphone"],
    priv_remote: "À distance avec LOCALINK",
    priv_remote_path: ["Votre téléphone", "Serveur · 2 min", "Son téléphone"],
    priv_points: [
        "En local, vos fichiers ne passent par aucun serveur.",
        "À distance, ils sont effacés du serveur au bout de 2 minutes, automatiquement.",
        "Personne n'a besoin de créer un compte ou de donner ses coordonnées.",
    ],
    num_label: "En chiffres",
    num_title: "Le partage, réduit à l'essentiel.",
    numbers: [
        figure(0, "", "compte à créer"),
        figure(1, "", "QR code ou code, au choix"),
        figure(2, "min", "avant l'effacement automatique"),
        figure(0, "", "copie conservée"),
    ],
    app_label: "iPhone",
    app_title: "Une app native, écrite en Swift.",
    app_lead: "LOCALINK sur iPhone est développée en Swift, au plus près d'iOS : une interface fluide, un appareil photo prêt à scanner, et vos fichiers à portée de main.",
    app_points: [
        "Scan du QR code avec l'appareil photo",
        "Connexion par code, sans rien scanner",
        "Partage à distance effacé après 2 minutes",
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
            "Comment se connecter ?",
            "Au choix : scannez le QR code affiché par l'autre téléphone, ou tapez son code LOCALINK. Les deux mènent à la même connexion.",
        ),
        item(
            "Les deux téléphones doivent-ils être sur le même réseau ?",
            "Non. Sur le même réseau, le transfert est direct ; sinon, LOCALINK passe par son serveur, avec un effacement automatique au bout de 2 minutes.",
        ),
        item(
            "Mes fichiers passent-ils par un serveur ?",
            "Sur le même réseau, non : ils vont directement d'un téléphone à l'autre. À distance, ils passent par le serveur LOCALINK et sont effacés au bout de 2 minutes.",
        ),
        item(
            "Que se passe-t-il après 2 minutes ?",
            "Le partage est supprimé du serveur automatiquement et le code ne mène plus à rien. Pour renvoyer le fichier, il suffit de le partager à nouveau.",
        ),
        item(
            "Faut-il une connexion Internet ?",
            "Sur le même réseau, le transfert s'en passe. À distance, il en faut une, puisque le fichier passe par le serveur.",
        ),
        item(
            "Quels fichiers puis-je envoyer ?",
            "Photos, vidéos, documents, musique : tous les fichiers présents sur votre téléphone.",
        ),
        item(
            "Sur quels appareils ?",
            "LOCALINK est une application native pour iPhone, écrite en Swift.",
        ),
    ],
    cta_title: "Partagez, côte à côte ou à distance.",
    cta_lead: "QR code ou code, au choix, et vos fichiers changent de téléphone. À distance, ils s'effacent en 2 minutes.",
    cta_button: "Voir comment ça marche",
    footer_tagline: "Transfert de fichiers entre téléphones, côte à côte ou à distance. QR code ou code au choix, sans compte, effacement automatique du serveur après 2 minutes.",
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
                "L'application ne demande aucun compte. En local, les fichiers vont directement d'un appareil à l'autre et ne passent par aucun serveur. Avec le partage à distance, le fichier est déposé temporairement sur le serveur de l'application LOCALINK, puis supprimé automatiquement au bout de 2 minutes. Le QR code et le code LOCALINK servent uniquement à relier les appareils.",
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
                "Ce site présente LOCALINK, une application de transfert de fichiers entre téléphones, sur le même réseau ou à distance.",
            ),
            item(
                "Accès",
                "Le site est accessible gratuitement, sans inscription. Nous faisons de notre mieux pour qu'il soit disponible, sans pouvoir le garantir en permanence.",
            ),
            item(
                "Utilisation de l'application",
                "Vous restez responsable des fichiers que vous envoyez et devez disposer des droits nécessaires pour les partager. N'envoyez des fichiers qu'aux personnes qui souhaitent les recevoir. Un partage à distance n'est disponible que 2 minutes, puis il est supprimé : gardez vos propres copies.",
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
    pick_text: "Transfert de fichiers entre téléphones, près ou loin, sans compte.",
};

// ----- English ----------------------------------------------------------------------

pub static EN: Texts = Texts {
    meta_title: "LOCALINK — File transfer between phones, near or far, with a QR code or a code",
    meta_description: "Send files from one phone to another, on the same network or far apart. A QR code or a code, your choice, no account, erased from the server after 2 minutes.",
    skip: "Skip to content",
    nav_how: "How it works",
    nav_features: "Features",
    nav_privacy: "Privacy",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_share: "2-min sharing",
    nav_try: "Try it",
    nav_home: "Home",
    nav_menu: "Menu",
    nav_cta: "Discover",
    loader: "Connecting…",
    lang_label: "Language",
    hero_eyebrow: "Transfer",
    hero_title: ["Your files,", "from one phone to another."],
    hero_send: "Send your",
    hero_words: ["photos", "videos", "documents", "music"],
    hero_lead: "LOCALINK sends your files from one phone to another, wherever you are. On the same network, they go straight across; far apart, through the server, which erases them after 2 minutes. To connect, you choose: QR code or code.",
    hero_primary: "How it works",
    hero_secondary: "The iPhone app",
    hero_badges: ["No account", "Near or far", "Erased in 2 min"],
    hero_scroll: "Scroll",
    phone_sender: "Sending",
    phone_receiver: "Receiving",
    phone_scan: "Scan to connect",
    phone_code: "or type the code",
    phone_connected: "Connected",
    phone_received: "Received",
    files: ["holidays.jpg", "concert.mov", "contract.pdf"],
    marquee: [
        "No account",
        "QR code or code",
        "Near or far",
        "Erased after 2 minutes",
        "Direct on a local network",
        "Any kind of file",
    ],
    how_label: "How it works",
    how_title: "Three steps, zero sign-up.",
    how_lead: "No username to remember: the two phones link up with a QR code or a code, your choice, then exchange.",
    steps: [
        item(
            "Open LOCALINK",
            "On both phones, side by side or at the other end of the country.",
        ),
        item(
            "QR code or code, your choice",
            "The receiving phone shows its QR code and its LOCALINK code. The other one chooses: scan the QR code or type the code.",
        ),
        item(
            "Send",
            "On the same network, files go straight from one device to the other. Far apart, they go through the server and are erased after 2 minutes.",
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
    share_label: "Ephemeral sharing",
    share_title: "Far apart, erased in 2 minutes.",
    share_lead: "Not next to each other? Drop your file on the LOCALINK server and share your code or your QR code, your choice. Your contact picks it up, then everything is erased automatically after 2 minutes.",
    share_code_label: "My LOCALINK code",
    share_choose: "Connect with",
    share_tab_code: "Code",
    share_tab_qr: "QR code",
    share_new: "New code",
    share_copy: "Copy the code",
    share_copied: "Code copied",
    share_expires: "Erased in",
    share_erased: "Erased from the server",
    share_again: "Share again",
    share_server: "LOCALINK server",
    share_demo: "Sped-up demo: 2 minutes in 12 seconds.",
    share_steps: [
        item("Drop", "Pick a file: it goes to the LOCALINK server."),
        item(
            "Give your code",
            "Out loud or as a QR code: your contact types it and gets the file.",
        ),
        item(
            "Everything is erased",
            "2 minutes later, the share is deleted from the server, automatically.",
        ),
    ],
    feat_label: "Features",
    feat_title: "As simple as a QR code.",
    feat_lead: "Everything you need to share a file with someone, next to you or far away, and nothing more.",
    features: [
        item(
            "No account",
            "No sign-up, no email, no password. Open the app and share.",
        ),
        item(
            "QR code pairing",
            "One scan links two devices, with nothing to type.",
        ),
        item(
            "Code pairing",
            "Don't feel like scanning? Choose the code: type the other phone's code and you're connected.",
        ),
        item(
            "Direct on the network",
            "On the same Wi‑Fi or a hotspot, the transfer goes straight from one phone to the other, with no server.",
        ),
        item(
            "Remote sharing",
            "Far apart? The file goes through the LOCALINK server and is erased automatically after 2 minutes.",
        ),
        item(
            "Nothing is kept",
            "No history, no online gallery, no cloud storage: nothing is left behind.",
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
    priv_title: "Your files don't linger anywhere.",
    priv_lead: "An online sharing service keeps your files on its servers for as long as it likes. LOCALINK goes straight through your network, or keeps your share for just 2 minutes.",
    priv_cloud: "With the cloud",
    priv_cloud_path: ["Your phone", "Server · no limit", "Their phone"],
    priv_local: "Locally with LOCALINK",
    priv_local_path: ["Your phone", "Their phone"],
    priv_remote: "Remotely with LOCALINK",
    priv_remote_path: ["Your phone", "Server · 2 min", "Their phone"],
    priv_points: [
        "Locally, your files go through no server.",
        "Remotely, they are erased from the server after 2 minutes, automatically.",
        "Nobody has to create an account or share their details.",
    ],
    num_label: "In numbers",
    num_title: "Sharing, down to the essentials.",
    numbers: [
        figure(0, "", "account to create"),
        figure(1, "", "QR code or code, your choice"),
        figure(2, "min", "before automatic erasure"),
        figure(0, "", "copy kept"),
    ],
    app_label: "iPhone",
    app_title: "A native app, written in Swift.",
    app_lead: "LOCALINK for iPhone is built in Swift, close to iOS: a smooth interface, a camera ready to scan, and your files at hand.",
    app_points: [
        "Scan the QR code with the camera",
        "Connect with a code, no scanning",
        "Remote sharing erased after 2 minutes",
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
            "How do I connect?",
            "Your choice: scan the QR code shown by the other phone, or type its LOCALINK code. Both lead to the same connection.",
        ),
        item(
            "Do both phones need to be on the same network?",
            "No. On the same network, the transfer is direct; otherwise LOCALINK goes through its server, with automatic erasure after 2 minutes.",
        ),
        item(
            "Do my files go through a server?",
            "On the same network, no: they go straight from one phone to the other. Far apart, they go through the LOCALINK server and are erased after 2 minutes.",
        ),
        item(
            "What happens after 2 minutes?",
            "The share is deleted from the server automatically and the code leads nowhere. To send the file again, just share it again.",
        ),
        item(
            "Do I need an Internet connection?",
            "On the same network, the transfer does without. Far apart, you need one, since the file goes through the server.",
        ),
        item(
            "Which files can I send?",
            "Photos, videos, documents, music: any file on your phone.",
        ),
        item(
            "On which devices?",
            "LOCALINK is a native iPhone app, written in Swift.",
        ),
    ],
    cta_title: "Share, side by side or far apart.",
    cta_lead: "A QR code or a code, your choice, and your files change phones. Far apart, they are erased in 2 minutes.",
    cta_button: "See how it works",
    footer_tagline: "File transfer between phones, side by side or far apart. A QR code or a code, your choice, no account, automatic erasure from the server after 2 minutes.",
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
                "The app asks for no account. Locally, files go straight from one device to the other and through no server. With remote sharing, the file is stored temporarily on the LOCALINK app server, then deleted automatically after 2 minutes. The QR code and the LOCALINK code are only used to link the devices.",
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
                "This site presents LOCALINK, an app to transfer files between phones, on the same network or far apart.",
            ),
            item(
                "Access",
                "The site is free and requires no sign-up. We do our best to keep it available, without being able to guarantee it at all times.",
            ),
            item(
                "Using the app",
                "You remain responsible for the files you send and must hold the rights needed to share them. Only send files to people who want to receive them. A remote share is only available for 2 minutes, then it is deleted: keep your own copies.",
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
    pick_text: "File transfer between phones, near or far, no account.",
};

// ----- Español ----------------------------------------------------------------------

pub static ES: Texts = Texts {
    meta_title: "LOCALINK — Transferencia de archivos entre teléfonos, cerca o lejos, con QR o con código",
    meta_description: "Envía archivos de un teléfono a otro, en la misma red o a distancia. Código QR o código, a elegir, sin cuenta, borrados del servidor a los 2 minutos.",
    skip: "Ir al contenido",
    nav_how: "Cómo funciona",
    nav_features: "Ventajas",
    nav_privacy: "Privacidad",
    nav_app: "iPhone",
    nav_faq: "FAQ",
    nav_share: "Compartir 2 min",
    nav_try: "Probar",
    nav_home: "Inicio",
    nav_menu: "Menú",
    nav_cta: "Descubrir",
    loader: "Conectando…",
    lang_label: "Idioma",
    hero_eyebrow: "Transferencia",
    hero_title: ["Tus archivos,", "de un teléfono a otro."],
    hero_send: "Envía tus",
    hero_words: ["fotos", "vídeos", "documentos", "canciones"],
    hero_lead: "LOCALINK envía tus archivos de un teléfono a otro, estés donde estés. En la misma red pasan directamente; a distancia, por el servidor, que los borra a los 2 minutos. Para conectarte, tú eliges: código QR o código.",
    hero_primary: "Cómo funciona",
    hero_secondary: "La app para iPhone",
    hero_badges: ["Sin cuenta", "Cerca o lejos", "Borrado en 2 min"],
    hero_scroll: "Desliza",
    phone_sender: "Envío",
    phone_receiver: "Recepción",
    phone_scan: "Escanea para conectarte",
    phone_code: "o teclea el código",
    phone_connected: "Conectado",
    phone_received: "Recibido",
    files: ["vacaciones.jpg", "concierto.mov", "contrato.pdf"],
    marquee: [
        "Sin cuenta",
        "QR o código",
        "Cerca o lejos",
        "Borrado a los 2 minutos",
        "Directo en red local",
        "Cualquier tipo de archivo",
    ],
    how_label: "Cómo funciona",
    how_title: "Tres gestos, cero registros.",
    how_lead: "Ningún usuario que recordar: los dos teléfonos se unen con un código QR o con un código, a elegir, y luego intercambian.",
    steps: [
        item(
            "Abre LOCALINK",
            "En los dos teléfonos, uno al lado del otro o en la otra punta del país.",
        ),
        item(
            "QR o código, a elegir",
            "El teléfono que recibe muestra su código QR y su código LOCALINK. El otro elige: escanear el QR o teclear el código.",
        ),
        item(
            "Envía",
            "En la misma red, los archivos pasan directamente de un dispositivo a otro. A distancia, pasan por el servidor y se borran a los 2 minutos.",
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
    share_label: "Compartir efímero",
    share_title: "A distancia, borrado en 2 minutos.",
    share_lead: "¿No estáis juntos? Sube tu archivo al servidor de LOCALINK y comparte, a elegir, tu código o tu código QR. Tu contacto lo recoge y todo se borra automáticamente a los 2 minutos.",
    share_code_label: "Mi código LOCALINK",
    share_choose: "Conectarse con",
    share_tab_code: "Código",
    share_tab_qr: "Código QR",
    share_new: "Nuevo código",
    share_copy: "Copiar el código",
    share_copied: "Código copiado",
    share_expires: "Se borra en",
    share_erased: "Borrado del servidor",
    share_again: "Compartir de nuevo",
    share_server: "Servidor LOCALINK",
    share_demo: "Demostración acelerada: 2 minutos en 12 segundos.",
    share_steps: [
        item("Sube", "Elige un archivo: va al servidor de LOCALINK."),
        item(
            "Da tu código",
            "De viva voz o en código QR: tu contacto lo teclea y recibe el archivo.",
        ),
        item(
            "Todo se borra",
            "2 minutos después, lo compartido se elimina del servidor, automáticamente.",
        ),
    ],
    feat_label: "Ventajas",
    feat_title: "Tan sencillo como un código QR.",
    feat_lead: "Todo lo necesario para compartir un archivo con alguien, a tu lado o lejos, y nada más.",
    features: [
        item(
            "Sin cuenta",
            "Sin registro, sin correo, sin contraseña. Abres la app y compartes.",
        ),
        item(
            "Conexión por código QR",
            "Un escaneo basta para unir dos dispositivos, sin teclear nada.",
        ),
        item(
            "Conexión por código",
            "¿No quieres escanear? Elige el código: teclea el del otro teléfono y ya estáis conectados.",
        ),
        item(
            "Directo en la red",
            "En la misma Wi‑Fi o un punto de acceso, la transferencia va directamente de un teléfono a otro, sin servidor.",
        ),
        item(
            "Compartir a distancia",
            "¿Estáis lejos? El archivo pasa por el servidor de LOCALINK y se borra automáticamente a los 2 minutos.",
        ),
        item(
            "No se guarda nada",
            "Sin historial, sin galería en línea, sin almacenamiento en la nube: no queda nada detrás de ti.",
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
    priv_title: "Tus archivos no se quedan en ninguna parte.",
    priv_lead: "Un servicio para compartir en línea guarda tus archivos en sus servidores todo el tiempo que quiere. LOCALINK va directo por tu red, o guarda lo compartido solo 2 minutos.",
    priv_cloud: "Con la nube",
    priv_cloud_path: ["Tu teléfono", "Servidor · sin límite", "Su teléfono"],
    priv_local: "En local con LOCALINK",
    priv_local_path: ["Tu teléfono", "Su teléfono"],
    priv_remote: "A distancia con LOCALINK",
    priv_remote_path: ["Tu teléfono", "Servidor · 2 min", "Su teléfono"],
    priv_points: [
        "En local, tus archivos no pasan por ningún servidor.",
        "A distancia, se borran del servidor a los 2 minutos, automáticamente.",
        "Nadie tiene que crear una cuenta ni dar sus datos.",
    ],
    num_label: "En cifras",
    num_title: "Compartir, reducido a lo esencial.",
    numbers: [
        figure(0, "", "cuentas que crear"),
        figure(1, "", "código QR o código, a elegir"),
        figure(2, "min", "antes del borrado automático"),
        figure(0, "", "copias guardadas"),
    ],
    app_label: "iPhone",
    app_title: "Una app nativa, escrita en Swift.",
    app_lead: "LOCALINK para iPhone está desarrollada en Swift, muy cerca de iOS: una interfaz fluida, una cámara lista para escanear y tus archivos a mano.",
    app_points: [
        "Escaneo del código QR con la cámara",
        "Conexión por código, sin escanear",
        "Compartir a distancia borrado a los 2 minutos",
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
            "¿Cómo me conecto?",
            "Tú eliges: escanea el código QR que muestra el otro teléfono, o teclea su código LOCALINK. Los dos llevan a la misma conexión.",
        ),
        item(
            "¿Los dos teléfonos deben estar en la misma red?",
            "No. En la misma red, la transferencia es directa; si no, LOCALINK pasa por su servidor, con borrado automático a los 2 minutos.",
        ),
        item(
            "¿Mis archivos pasan por un servidor?",
            "En la misma red, no: van directamente de un teléfono a otro. A distancia, pasan por el servidor de LOCALINK y se borran a los 2 minutos.",
        ),
        item(
            "¿Qué pasa a los 2 minutos?",
            "Lo compartido se elimina del servidor automáticamente y el código ya no lleva a nada. Para enviar el archivo otra vez, basta con compartirlo de nuevo.",
        ),
        item(
            "¿Hace falta conexión a Internet?",
            "En la misma red, la transferencia no la necesita. A distancia sí, porque el archivo pasa por el servidor.",
        ),
        item(
            "¿Qué archivos puedo enviar?",
            "Fotos, vídeos, documentos, música: cualquier archivo de tu teléfono.",
        ),
        item(
            "¿En qué dispositivos?",
            "LOCALINK es una app nativa para iPhone, escrita en Swift.",
        ),
    ],
    cta_title: "Comparte, uno al lado del otro o a distancia.",
    cta_lead: "Un código QR o un código, a elegir, y tus archivos cambian de teléfono. A distancia, se borran en 2 minutos.",
    cta_button: "Ver cómo funciona",
    footer_tagline: "Transferencia de archivos entre teléfonos, uno al lado del otro o a distancia. Código QR o código, a elegir, sin cuenta, borrado automático del servidor a los 2 minutos.",
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
                "La app no pide ninguna cuenta. En local, los archivos van directamente de un dispositivo a otro y no pasan por ningún servidor. Al compartir a distancia, el archivo se guarda temporalmente en el servidor de la app LOCALINK y se elimina automáticamente a los 2 minutos. El código QR y el código LOCALINK solo sirven para unir los dispositivos.",
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
                "Este sitio presenta LOCALINK, una app para transferir archivos entre teléfonos, en la misma red o a distancia.",
            ),
            item(
                "Acceso",
                "El sitio es gratuito y no requiere registro. Hacemos todo lo posible para que esté disponible, sin poder garantizarlo en todo momento.",
            ),
            item(
                "Uso de la app",
                "Eres responsable de los archivos que envías y debes tener los derechos necesarios para compartirlos. Envía archivos solo a quien quiera recibirlos. Lo compartido a distancia solo está disponible 2 minutos y luego se elimina: guarda tus propias copias.",
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
    pick_text: "Transferencia de archivos entre teléfonos, cerca o lejos, sin cuenta.",
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
