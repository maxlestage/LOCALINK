import LocalinkCore
import SwiftUI

/// What the receive screen is doing.
@MainActor
@Observable
final class ReceiveModel {
    enum Nearby: Equatable {
        case waiting
        case receiving(device: String, fraction: Double, file: String)
        case done([String])
        case failed(String)
    }

    enum Remote: Equatable {
        case idle
        case checking
        case found(ConnectionCode, ShareInfo, Date)
        case downloading
        case saved(String)
        case failed(String)
    }

    private(set) var code = ConnectionCode.random()
    var nearby = Nearby.waiting
    var remote = Remote.idle
    private let listener = NearbyListener()
    private var device = ""

    /// Announces this phone under its code.
    func start(_ received: ReceivedFiles) {
        do {
            try listener.start(code: code) { [weak self] event in
                self?.handle(event, received)
            }
        } catch {
            nearby = .failed(error.localizedDescription)
        }
    }

    func stop() {
        listener.stop()
    }

    /// A new code: the old one stops working.
    func newCode(_ received: ReceivedFiles) {
        code = .random()
        nearby = .waiting
        start(received)
    }

    private func handle(_ event: IncomingEvent, _ received: ReceivedFiles) {
        switch event {
        case .started(let device, _, _):
            self.device = device
            nearby = .receiving(device: device, fraction: 0, file: "")
        case .progress(let fraction, let file):
            nearby = .receiving(device: device, fraction: fraction, file: file)
        case .finished(let urls):
            nearby = .done(urls.map(\.lastPathComponent))
            received.reload()
        case .failed(let message):
            nearby = .failed(message)
            received.reload()
        }
    }

    /// Looks up a code given by a phone far away.
    func lookUp(_ code: ConnectionCode) {
        remote = .checking
        Task {
            do {
                let info = try await RemoteShares.shared.info(code)
                remote = .found(code, info, Date().addingTimeInterval(TimeInterval(info.expiresIn)))
            } catch {
                remote = .failed(error.localizedDescription)
            }
        }
    }

    func download(_ code: ConnectionCode, _ received: ReceivedFiles) {
        remote = .downloading
        Task {
            do {
                let url = try await RemoteShares.shared.download(code)
                received.reload()
                remote = .saved(url.lastPathComponent)
            } catch {
                remote = .failed(error.localizedDescription)
            }
        }
    }
}

/// Receive: show my code (as text or as a QR code) to a phone nearby, or take the code of
/// a phone far away.
struct ReceiveView: View {
    @Environment(ReceivedFiles.self) private var received
    @State private var model = ReceiveModel()
    @State private var distance = Distance.nearby
    @State private var pulse = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    ScreenHeader(title: "Recevoir", subtitle: "Votre code ou votre QR code, au choix")
                    DistancePicker(distance: $distance)
                    switch distance {
                    case .nearby: nearbySection
                    case .remote: remoteSection
                    }
                }
                .padding(20)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: distance)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: model.nearby)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: model.remote)
            }
            .scrollDismissesKeyboard(.interactively)
            .background(AnimatedBackground())
            .toolbar(.hidden, for: .navigationBar)
        }
        .onAppear { model.start(received) }
        .onDisappear { model.stop() }
    }

    private var nearbySection: some View {
        VStack(alignment: .leading, spacing: 16) {
            CodeCard(code: model.code)
                .id(model.code)
                .transition(.scale(scale: 0.9).combined(with: .opacity))
            switch model.nearby {
            case .waiting:
                HStack(spacing: 12) {
                    Circle()
                        .fill(Theme.green)
                        .frame(width: 10, height: 10)
                        .overlay(Circle().stroke(Theme.green, lineWidth: 2).scaleEffect(pulse ? 3 : 1).opacity(pulse ? 0 : 1))
                        .onAppear {
                            withAnimation(.easeOut(duration: 1.6).repeatForever(autoreverses: false)) { pulse = true }
                        }
                    Text("En attente d'un téléphone à proximité…").foregroundStyle(Theme.muted)
                }
            case .receiving(let device, let fraction, let file):
                Card {
                    Text("Réception depuis \(device)").font(.headline)
                    if !file.isEmpty { Text(file).font(.subheadline).foregroundStyle(Theme.muted).lineLimit(1) }
                    ProgressBar(value: fraction)
                }
            case .done(let names):
                Card {
                    Label("Reçu !", systemImage: "checkmark.circle.fill")
                        .font(.headline)
                        .foregroundStyle(Theme.green)
                        .symbolEffect(.bounce, value: names.count)
                    ForEach(names, id: \.self) { name in
                        Label(name, systemImage: symbol(forExtension: (name as NSString).pathExtension))
                            .lineLimit(1)
                    }
                    Text("Retrouvez-les dans l'onglet Reçus et dans l'app Fichiers.")
                        .font(.footnote)
                        .foregroundStyle(Theme.muted)
                }
                .sensoryFeedback(.success, trigger: names)
            case .failed(let message):
                StatusLine(text: message, symbol: "exclamationmark.triangle.fill", color: Theme.red)
            }
            Button {
                model.newCode(received)
            } label: {
                Label("Nouveau code", systemImage: "arrow.triangle.2.circlepath")
            }
            .buttonStyle(SecondaryButtonStyle())
            Text("Sur l'autre téléphone : LOCALINK › Envoyer › À côté, puis tapez ce code ou scannez ce QR code.")
                .font(.footnote)
                .foregroundStyle(Theme.muted)
        }
    }

    @ViewBuilder
    private var remoteSection: some View {
        switch model.remote {
        case .idle, .failed:
            VStack(alignment: .leading, spacing: 16) {
                Card {
                    CodeEntryView(title: "Le code de partage reçu") { code in model.lookUp(code) }
                }
                if case .failed(let message) = model.remote {
                    StatusLine(text: message, symbol: "exclamationmark.triangle.fill", color: Theme.red)
                }
            }
        case .checking, .downloading:
            Card {
                HStack(spacing: 12) {
                    ProgressView()
                    Text(model.remote == .checking ? LocalizedStringKey("Recherche du partage…") : LocalizedStringKey("Téléchargement…"))
                }
            }
        case .found(let code, let info, let expires):
            VStack(spacing: 18) {
                Card {
                    HStack(spacing: 12) {
                        Image(systemName: symbol(forExtension: (info.name as NSString).pathExtension))
                            .font(.title2)
                            .frame(width: 48, height: 48)
                            .background(Theme.gradient, in: RoundedRectangle(cornerRadius: 14))
                            .foregroundStyle(Theme.background)
                        VStack(alignment: .leading) {
                            Text(info.name).font(.headline).lineLimit(2)
                            Text(formatSize(info.size)).foregroundStyle(Theme.muted)
                        }
                    }
                }
                CountdownRing(expires: expires)
                Button {
                    model.download(code, received)
                } label: {
                    Label("Télécharger", systemImage: "arrow.down.circle")
                }
                .buttonStyle(PrimaryButtonStyle())
                .disabled(expires < .now)
            }
        case .saved(let name):
            Card {
                Label("Enregistré : \(name)", systemImage: "checkmark.circle.fill")
                    .foregroundStyle(Theme.green)
                Button("Recevoir un autre fichier") { model.remote = .idle }
                    .buttonStyle(PrimaryButtonStyle())
            }
            .sensoryFeedback(.success, trigger: name)
        }
    }
}
