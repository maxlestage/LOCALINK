import CoreTransferable
import Foundation
import LocalinkCore
import UniformTypeIdentifiers

/// A file chosen to be sent, copied into the app's temporary folder (so it stays
/// readable for the whole transfer, wherever it came from).
struct PickedFile: Identifiable, Hashable, Sendable {
    let id = UUID()
    let url: URL
    let name: String
    let size: Int64
    let type: String?

    /// Copies `source` (a file of the Files app, of Photos…) for the transfer.
    static func copy(from source: URL, name: String? = nil) throws -> PickedFile {
        let folder = FileManager.default.temporaryDirectory
            .appendingPathComponent("outgoing", isDirectory: true)
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try FileManager.default.createDirectory(at: folder, withIntermediateDirectories: true)
        let destination = folder.appendingPathComponent(cleanFileName(name ?? source.lastPathComponent))
        try FileManager.default.copyItem(at: source, to: destination)
        return try PickedFile(at: destination)
    }

    init(at url: URL) throws {
        self.url = url
        name = url.lastPathComponent
        let attributes = try FileManager.default.attributesOfItem(atPath: url.path)
        size = (attributes[.size] as? NSNumber)?.int64Value ?? 0
        type = UTType(filenameExtension: url.pathExtension)?.preferredMIMEType
    }

    /// The icon of its kind.
    var symbol: String { fileSymbol(forExtension: url.pathExtension) }
}

/// The SF Symbol of a kind of file.
func fileSymbol(forExtension ext: String) -> String {
    guard let type = UTType(filenameExtension: ext) else { return "doc" }
    if type.conforms(to: .image) { return "photo" }
    if type.conforms(to: .movie) || type.conforms(to: .video) { return "film" }
    if type.conforms(to: .audio) { return "music.note" }
    if type.conforms(to: .pdf) { return "doc.richtext" }
    if type.conforms(to: .archive) { return "doc.zipper" }
    if type.conforms(to: .text) { return "doc.text" }
    return "doc"
}

/// A photo or a video of the Photos library, received as a file.
struct TransferableFile: Transferable, Sendable {
    let file: PickedFile

    static var transferRepresentation: some TransferRepresentation {
        FileRepresentation(importedContentType: .movie) { received in
            TransferableFile(file: try PickedFile.copy(from: received.file))
        }
        FileRepresentation(importedContentType: .image) { received in
            TransferableFile(file: try PickedFile.copy(from: received.file))
        }
        FileRepresentation(importedContentType: .data) { received in
            TransferableFile(file: try PickedFile.copy(from: received.file))
        }
    }
}
