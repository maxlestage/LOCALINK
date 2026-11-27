import QuickLook
import SwiftUI

/// The files received, to open, share or delete.
struct ReceivedListView: View {
    @Environment(ReceivedFiles.self) private var received
    @State private var preview: URL?

    var body: some View {
        NavigationStack {
            List {
                ForEach(received.files) { file in
                    Button {
                        preview = file.url
                    } label: {
                        HStack(spacing: 12) {
                            Image(systemName: fileSymbol(forExtension: file.url.pathExtension))
                                .frame(width: 40, height: 40)
                                .background(Theme.gradient, in: RoundedRectangle(cornerRadius: 12))
                                .foregroundStyle(Theme.background)
                            VStack(alignment: .leading, spacing: 2) {
                                Text(file.name).lineLimit(1).truncationMode(.middle)
                                Text(verbatim: "\(formatSize(file.size)) · \(file.date.formatted(.relative(presentation: .named)))")
                                    .font(.caption)
                                    .foregroundStyle(Theme.muted)
                            }
                        }
                    }
                    .swipeActions {
                        Button(role: .destructive) {
                            withAnimation { received.delete(file) }
                        } label: {
                            Label("Supprimer", systemImage: "trash")
                        }
                        ShareLink(item: file.url) {
                            Label("Partager", systemImage: "square.and.arrow.up")
                        }
                        .tint(Theme.violet)
                    }
                    .listRowBackground(Theme.surface)
                }
            }
            .scrollContentBackground(.hidden)
            .background(AnimatedBackground())
            .overlay {
                if received.files.isEmpty {
                    ContentUnavailableView(
                        "Aucun fichier reçu", systemImage: "tray",
                        description: Text("Les fichiers reçus apparaissent ici, et dans l'app Fichiers."))
                }
            }
            .navigationTitle("Reçus")
            .refreshable { received.reload() }
            .onAppear { received.reload() }
            .quickLookPreview($preview)
        }
    }
}
