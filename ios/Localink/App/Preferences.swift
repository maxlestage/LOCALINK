import Foundation

/// The keys of the user's choices (UserDefaults, `@AppStorage`).
enum Preferences {
    /// How codes are shown and taken by default: `code` or `qr`.
    static let method = "defaultMethod"
    /// Side by side or far apart by default: `nearby` or `remote`.
    static let distance = "defaultDistance"
    /// The presentation was seen (or skipped).
    static let onboarded = "hasSeenOnboarding"

    static var defaultMethod: CodeMethod {
        CodeMethod(rawValue: UserDefaults.standard.string(forKey: method) ?? "") ?? .code
    }

    static var defaultDistance: Distance {
        Distance(rawValue: UserDefaults.standard.string(forKey: distance) ?? "") ?? .nearby
    }
}

/// The links of the app.
enum Links {
    /// The language of the website: the app's, else French.
    static var language: String {
        let code = Locale.current.language.languageCode?.identifier ?? "fr"
        return ["fr", "en", "es"].contains(code) ? code : "fr"
    }

    static var website: URL { URL(string: "https://maxlestage.github.io/LOCALINK/\(language)/")! }
    static var privacy: URL { URL(string: "https://maxlestage.github.io/LOCALINK/\(language)/privacy/")! }
    static var terms: URL { URL(string: "https://maxlestage.github.io/LOCALINK/\(language)/terms/")! }
    static let source = URL(string: "https://github.com/maxlestage/LOCALINK")!
    static let paquets = URL(string: "https://github.com/maxlestage/Paquets")!
}
