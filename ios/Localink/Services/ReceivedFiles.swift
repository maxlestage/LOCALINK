import Foundation
import LocalinkCore
import Observation

/// A file received, kept in the app's Documents folder (visible in the Files app, under
/// "On my iPhone › LOCALINK").
struct ReceivedFile: Identifiable, Hashable, Sendable {
    var id: URL { url }
    let url: URL
    let size: Int64
    let date: Date

    var name: String { url.lastPathComponent }
}

/// The received files.
@MainActor
@Observable
final class ReceivedFiles {
    private(set) var files: [ReceivedFile] = []

    /// Where files are saved.
    nonisolated static var folder: URL {
        FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }

    init() {
        reload()
    }

    func reload() {
        let keys: [URLResourceKey] = [.fileSizeKey, .creationDateKey, .isRegularFileKey]
        let urls =
            (try? FileManager.default.contentsOfDirectory(
                at: Self.folder, includingPropertiesForKeys: keys, options: [.skipsHiddenFiles])) ?? []
        files = urls.compactMap { url in
            guard let values = try? url.resourceValues(forKeys: Set(keys)), values.isRegularFile == true else {
                return nil
            }
            return ReceivedFile(
                url: url, size: Int64(values.fileSize ?? 0), date: values.creationDate ?? .distantPast)
        }
        .sorted { $0.date > $1.date }
    }

    func delete(_ file: ReceivedFile) {
        try? FileManager.default.removeItem(at: file.url)
        reload()
    }

    /// A free place for a file named `name`.
    nonisolated static func destination(for name: String) -> URL {
        let clean = cleanFileName(name)
        let unique = uniqueFileName(clean) { candidate in
            FileManager.default.fileExists(atPath: folder.appendingPathComponent(candidate).path)
        }
        return folder.appendingPathComponent(unique)
    }
}
