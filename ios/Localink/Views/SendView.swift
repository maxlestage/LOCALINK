import LocalinkCore
import PhotosUI
import SwiftUI
import UIKit

/// What the send screen is doing.
@MainActor
@Observable
final class SendModel {
    enum Phase: Equatable {
        case choosing
        case connecting
        case sending(Double, String)
        case sent(Int)
        case uploading(Double)
        case shared(CreatedShare, Date)
        case failed(String)
    }

    var files: [PickedFile] = []
    var distance = Distance.nearby
    var phase = Phase.choosing
    private var task: Task<Void, Never>?

    var totalSize: Int64 { files.reduce(0) { $0 + $1.size } }
    var busy: Bool {
        switch phase {
        case .connecting, .sending, .uploading: true
        default: false
        }
    }

    /// Sends the files to the nearby phone that shows `code`.
    func sendNearby(to code: ConnectionCode) {
        guard !files.isEmpty, !busy else { return }
        let files = files
        phase = .connecting
        task = Task {
            do {
                try await NearbySender.send(files, to: code, device: UIDevice.current.name) { fraction, name in
                    Task { @MainActor [weak self] in
                        guard let self else { return }
                        switch self.phase {
                        case .connecting, .sending: self.phase = .sending(fraction, name)
                        default: break
                        }
                    }
                }
                phase = .sent(files.count)
            } catch is CancellationError {
                phase = .choosing
            } catch {
                phase = .failed(error.localizedDescription)
            }
        }
    }

    /// Puts the file on the server for 2 minutes.
    func shareRemotely() {
        guard let file = files.first, !busy else { return }
        phase = .uploading(0)
        task = Task {
            do {
                let share = try await RemoteShares.shared.upload(file) { fraction in
                    Task { @MainActor [weak self] in
                        if case .uploading = self?.phase { self?.phase = .uploading(fraction) }
                    }
                }
                phase = .shared(share, Date().addingTimeInterval(TimeInterval(share.expiresIn)))
            } catch is CancellationError {
                phase = .choosing
            } catch {
                phase = .failed(error.localizedDescription)
            }
        }
    }

    /// Erases the share now.
    func deleteShare() {
        guard case .shared(let share, _) = phase else { return }
        Task {
            try? await RemoteShares.shared.delete(share)
            phase = .choosing
        }
    }

    func cancel() {
        task?.cancel()
        task = nil
        phase = .choosing
    }

    func reset() {
        cancel()
        files.removeAll()
    }
}

/// Send: choose files, choose side by side or far apart, then the code (typed or scanned)
/// or a code to give.
struct SendView: View {
    @State private var model = SendModel()
    @State private var photoItems: [PhotosPickerItem] = []
    @State private var importing = false
    @State private var loadingPhotos = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    ScreenHeader(title: "Envoyer", subtitle: "Près ou loin, sans compte")
                    filesCard
                    if !model.files.isEmpty {
                        DistancePicker(distance: $model.distance)
                            .disabled(model.busy)
                            .transition(.move(edge: .bottom).combined(with: .opacity))
                        action
                    }
                    result
                }
                .padding(20)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: model.phase)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: model.files)
                .animation(.spring(response: 0.45, dampingFraction: 0.8), value: model.distance)
            }
            .scrollDismissesKeyboard(.interactively)
            .background(AnimatedBackground())
            .toolbar(.hidden, for: .navigationBar)
        }
        .fileImporter(isPresented: $importing, allowedContentTypes: [.item], allowsMultipleSelection: true) { result in
            guard case .success(let urls) = result else { return }
            for url in urls {
                let scoped = url.startAccessingSecurityScopedResource()
                defer { if scoped { url.stopAccessingSecurityScopedResource() } }
                if let file = try? PickedFile.copy(from: url) { model.files.append(file) }
            }
        }
        .onChange(of: photoItems) { _, items in
            guard !items.isEmpty else { return }
            loadingPhotos = true
            Task {
                for item in items {
                    if let picked = try? await item.loadTransferable(type: TransferableFile.self) {
                        model.files.append(picked.file)
                    }
                }
                photoItems = []
                loadingPhotos = false
            }
        }
    }

    private var filesCard: some View {
        Card {
            HStack {
                Text("Fichiers").font(.headline)
                Spacer()
                if !model.files.isEmpty {
                    Text(verbatim: "\(model.files.count) · \(formatSize(model.totalSize))")
                        .font(.subheadline.monospacedDigit())
                        .foregroundStyle(Theme.muted)
                }
            }
            ForEach(model.files) { file in
                HStack(spacing: 12) {
                    Image(systemName: file.symbol)
                        .frame(width: 38, height: 38)
                        .background(Theme.gradient.opacity(0.9), in: RoundedRectangle(cornerRadius: 11))
                        .foregroundStyle(Theme.background)
                    VStack(alignment: .leading, spacing: 2) {
                        Text(file.name).lineLimit(1).truncationMode(.middle)
                        Text(formatSize(file.size)).font(.caption).foregroundStyle(Theme.muted)
                    }
                    Spacer()
                    Button {
                        model.files.removeAll { $0.id == file.id }
                    } label: {
                        Image(systemName: "xmark.circle.fill").foregroundStyle(Theme.muted)
                    }
                    .accessibilityLabel(Text("Retirer"))
                    .disabled(model.busy)
                }
                .transition(.asymmetric(insertion: .move(edge: .leading).combined(with: .opacity), removal: .scale.combined(with: .opacity)))
            }
            HStack(spacing: 10) {
                PhotosPicker(selection: $photoItems, maxSelectionCount: 20, matching: .any(of: [.images, .videos])) {
                    Label("Photos", systemImage: "photo.on.rectangle")
                }
                .buttonStyle(SecondaryButtonStyle())
                Button {
                    importing = true
                } label: {
                    Label("Fichiers", systemImage: "folder")
                }
                .buttonStyle(SecondaryButtonStyle())
            }
            .disabled(model.busy)
            if loadingPhotos {
                ProgressView().frame(maxWidth: .infinity)
            }
        }
    }

    @ViewBuilder
    private var action: some View {
        switch (model.distance, model.phase) {
        case (.nearby, .choosing), (.nearby, .failed):
            Card {
                CodeEntryView(title: "Le code du téléphone qui reçoit") { code in
                    model.sendNearby(to: code)
                }
                Text("Sur l'autre téléphone : LOCALINK › Recevoir › À côté.")
                    .font(.footnote)
                    .foregroundStyle(Theme.muted)
            }
        case (.remote, .choosing), (.remote, .failed):
            Card {
                if model.files.count > 1 {
                    StatusLine(text: String(localized: "À distance, un seul fichier à la fois : le premier sera partagé."), symbol: "exclamationmark.circle")
                }
                Text("Le fichier part sur le serveur LOCALINK. Vous obtenez un code et un QR code : votre contact a 2 minutes pour le récupérer, puis tout s'efface.")
                    .font(.subheadline)
                    .foregroundStyle(Theme.muted)
                Button {
                    model.shareRemotely()
                } label: {
                    Label("Partager pour 2 minutes", systemImage: "icloud.and.arrow.up")
                }
                .buttonStyle(PrimaryButtonStyle())
            }
        default:
            EmptyView()
        }
    }

    @ViewBuilder
    private var result: some View {
        switch model.phase {
        case .choosing:
            if model.files.isEmpty {
                Card {
                    Label("Choisissez des photos, des vidéos ou des fichiers.", systemImage: "hand.tap")
                        .foregroundStyle(Theme.muted)
                }
            }
        case .connecting:
            Card {
                HStack(spacing: 12) {
                    ProgressView()
                    Text("Recherche du téléphone…")
                }
                Button("Annuler", role: .cancel, action: model.cancel).buttonStyle(SecondaryButtonStyle())
            }
        case .sending(let fraction, let name):
            Card {
                Text("Envoi de \(name)").lineLimit(1).truncationMode(.middle)
                ProgressBar(value: fraction)
                Text(fraction, format: .percent.precision(.fractionLength(0)))
                    .font(.system(.title2, design: .monospaced).bold())
                    .contentTransition(.numericText())
                Button("Annuler", role: .cancel, action: model.cancel).buttonStyle(SecondaryButtonStyle())
            }
        case .sent(let count):
            Card {
                Label("Envoyé ! \(count) fichier(s) reçu(s) de l'autre côté.", systemImage: "checkmark.circle.fill")
                    .font(.headline)
                    .foregroundStyle(Theme.green)
                    .symbolEffect(.bounce, value: count)
                Button("Envoyer d'autres fichiers", action: model.reset).buttonStyle(PrimaryButtonStyle())
            }
            .sensoryFeedback(.success, trigger: count)
        case .uploading(let fraction):
            Card {
                Text("Envoi sur le serveur…")
                ProgressBar(value: fraction)
                Button("Annuler", role: .cancel, action: model.cancel).buttonStyle(SecondaryButtonStyle())
            }
        case .shared(let share, let expires):
            VStack(spacing: 18) {
                CodeCard(code: share.code, title: "Code de partage")
                CountdownRing(expires: expires)
                Text("Donnez ce code ou ce QR code : il fonctionne 2 minutes, puis le fichier est effacé du serveur.")
                    .font(.subheadline)
                    .multilineTextAlignment(.center)
                    .foregroundStyle(Theme.muted)
                Button(role: .destructive, action: model.deleteShare) {
                    Label("Effacer maintenant", systemImage: "trash")
                }
                .buttonStyle(SecondaryButtonStyle())
                Button("Partager autre chose", action: model.reset).buttonStyle(PrimaryButtonStyle())
            }
            .transition(.scale(scale: 0.92).combined(with: .opacity))
        case .failed(let message):
            StatusLine(text: message, symbol: "exclamationmark.triangle.fill", color: Theme.red)
        }
    }
}
