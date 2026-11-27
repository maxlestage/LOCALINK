import Foundation
import LocalinkCore

/// The server of LOCALINK, for phones far apart: a file stays 2 minutes, then it is
/// erased.
struct RemoteShares: Sendable {
    let api: ShareAPI

    /// The server named in Info.plist (`LocalinkServerURL`).
    static var shared: RemoteShares {
        let raw = Bundle.main.object(forInfoDictionaryKey: "LocalinkServerURL") as? String
        let url = raw.flatMap(URL.init(string:)) ?? URL(string: "https://localink-cb296cd42039.herokuapp.com")!
        return RemoteShares(api: ShareAPI(baseURL: url))
    }

    /// Uploads `file`; `progress` gets the fraction sent.
    func upload(_ file: PickedFile, progress: @escaping @Sendable (Double) -> Void) async throws -> CreatedShare {
        guard file.size <= ShareAPI.maxFileSize else { throw ShareError.tooLarge }
        let request = api.createRequest(fileName: file.name, mimeType: file.type)
        let delegate = UploadProgress(progress)
        let (data, response) = try await URLSession.shared.upload(for: request, fromFile: file.url, delegate: delegate)
        try check(response)
        return try JSONDecoder().decode(CreatedShare.self, from: data)
    }

    /// What `code` holds.
    func info(_ code: ConnectionCode) async throws -> ShareInfo {
        let (data, response) = try await URLSession.shared.data(from: api.infoURL(code))
        try check(response)
        return try JSONDecoder().decode(ShareInfo.self, from: data)
    }

    /// Downloads the file of `code` into the received files.
    func download(_ code: ConnectionCode) async throws -> URL {
        let (temporary, response) = try await URLSession.shared.download(from: api.fileURL(code))
        try check(response)
        let header = (response as? HTTPURLResponse)?.value(forHTTPHeaderField: "X-Localink-Name")
        let destination = ReceivedFiles.destination(for: decodeFileName(header) ?? "fichier")
        try FileManager.default.moveItem(at: temporary, to: destination)
        return destination
    }

    /// Erases `share` before its 2 minutes are over.
    func delete(_ share: CreatedShare) async throws {
        let (_, response) = try await URLSession.shared.data(for: api.deleteRequest(share))
        try check(response)
    }

    private func check(_ response: URLResponse) throws {
        let status = (response as? HTTPURLResponse)?.statusCode ?? 0
        if let error = ShareError(status: status) { throw error }
    }
}

extension ShareError: @retroactive LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .notFound: String(localized: "Ce code ne mène à aucun partage, ou ses 2 minutes sont passées.")
        case .tooLarge: String(localized: "Le fichier dépasse 25 Mo, la limite du partage à distance.")
        case .serverFull: String(localized: "Le serveur est plein pour un moment : réessayez dans deux minutes.")
        case .empty: String(localized: "Le fichier est vide.")
        case .server(let status): String(localized: "Le serveur a répondu \(status). Réessayez.")
        }
    }
}

/// Reports the progress of an upload.
final class UploadProgress: NSObject, URLSessionTaskDelegate, @unchecked Sendable {
    private let onProgress: @Sendable (Double) -> Void

    init(_ onProgress: @escaping @Sendable (Double) -> Void) {
        self.onProgress = onProgress
    }

    func urlSession(
        _ session: URLSession, task: URLSessionTask, didSendBodyData bytesSent: Int64, totalBytesSent: Int64,
        totalBytesExpectedToSend: Int64
    ) {
        onProgress(Double(totalBytesSent) / Double(max(1, totalBytesExpectedToSend)))
    }
}
