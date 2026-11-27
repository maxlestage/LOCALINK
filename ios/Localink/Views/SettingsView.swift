import SwiftUI
import UIKit

/// Options, privacy, about and credits.
struct SettingsView: View {
    @Environment(ReceivedFiles.self) private var received
    @Environment(\.openURL) private var openURL
    @AppStorage(Preferences.method) private var method = CodeMethod.code
    @AppStorage(Preferences.distance) private var distance = Distance.nearby
    @AppStorage(Preferences.onboarded) private var onboarded = true
    @State private var confirmClear = false

    private var version: String {
        let info = Bundle.main.infoDictionary
        let short = info?["CFBundleShortVersionString"] as? String ?? "1.0"
        let build = info?["CFBundleVersion"] as? String ?? "1"
        return "\(short) (\(build))"
    }

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Picker(selection: $method) {
                        Text("Code").tag(CodeMethod.code)
                        Text("QR code").tag(CodeMethod.qr)
                    } label: {
                        Label("Connexion par défaut", systemImage: "key.horizontal")
                    }
                    Picker(selection: $distance) {
                        Text("À côté").tag(Distance.nearby)
                        Text("À distance").tag(Distance.remote)
                    } label: {
                        Label("Mode par défaut", systemImage: "point.3.connected.trianglepath.dotted")
                    }
                    Button {
                        if let url = URL(string: UIApplication.openSettingsURLString) { openURL(url) }
                    } label: {
                        Label("Langue, appareil photo et réseau local", systemImage: "gearshape")
                    }
                } header: {
                    Text("Options")
                } footer: {
                    Text("La langue et les autorisations se règlent dans l'app Réglages d'iOS.")
                }
                .listRowBackground(Theme.surface)

                Section {
                    Button(role: .destructive) {
                        confirmClear = true
                    } label: {
                        Label("Vider les fichiers reçus", systemImage: "trash")
                            .foregroundStyle(Theme.red)
                    }
                    .disabled(received.files.isEmpty)
                } header: {
                    Text("Fichiers reçus")
                } footer: {
                    Text("Supprime de cet iPhone les \(received.files.count) fichier(s) reçu(s).")
                }
                .listRowBackground(Theme.surface)

                Section("Confidentialité") {
                    Label("Aucun compte, aucun traceur, aucune publicité", systemImage: "eye.slash")
                    Label("À côté, vos fichiers ne passent par aucun serveur", systemImage: "wifi")
                    Label("À distance, ils sont effacés du serveur après 2 minutes", systemImage: "timer")
                    Link(destination: Links.privacy) {
                        Label("Politique de confidentialité", systemImage: "hand.raised")
                    }
                    Link(destination: Links.terms) {
                        Label("Conditions d'utilisation", systemImage: "doc.text")
                    }
                }
                .listRowBackground(Theme.surface)

                Section("À propos") {
                    LabeledContent {
                        Text(verbatim: version).monospacedDigit()
                    } label: {
                        Label("Version", systemImage: "info.circle")
                    }
                    LabeledContent {
                        Text(verbatim: "Maxime Nathan Lestage")
                    } label: {
                        Label("Créé par", systemImage: "person.fill")
                    }
                    Link(destination: Links.website) {
                        Label("Site web de LOCALINK", systemImage: "safari")
                    }
                    Button {
                        onboarded = false
                    } label: {
                        Label("Revoir la présentation", systemImage: "sparkles")
                    }
                }
                .listRowBackground(Theme.surface)

                Section {
                    Label("Conception et développement : Maxime Nathan Lestage", systemImage: "hammer")
                    Label("App écrite en Swift et SwiftUI", systemImage: "swift")
                    Label("Serveur et site écrits en Rust avec Paquets", systemImage: "server.rack")
                    Label("Réseau : Network.framework d'Apple, TLS à clé partagée", systemImage: "network")
                    Label("Scanner : VisionKit · QR codes : Core Image", systemImage: "qrcode.viewfinder")
                    Link(destination: Links.source) {
                        Label("Code source sur GitHub", systemImage: "chevron.left.forwardslash.chevron.right")
                    }
                    Link(destination: Links.paquets) {
                        Label("Framework Paquets", systemImage: "shippingbox")
                    }
                } header: {
                    Text("Crédits")
                } footer: {
                    Text("© 2026 Maxime Nathan Lestage. Tous droits réservés.")
                        .frame(maxWidth: .infinity, alignment: .center)
                        .padding(.top, 12)
                }
                .listRowBackground(Theme.surface)
            }
            .tint(Theme.cyan)
            .scrollContentBackground(.hidden)
            .background(AnimatedBackground())
            .navigationTitle("Réglages")
            .confirmationDialog("Vider les fichiers reçus ?", isPresented: $confirmClear, titleVisibility: .visible) {
                Button("Supprimer tous les fichiers", role: .destructive) {
                    withAnimation { received.deleteAll() }
                }
            } message: {
                Text("Cette action est définitive.")
            }
        }
    }
}
