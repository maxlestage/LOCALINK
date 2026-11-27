import Foundation
import LocalinkCore
@preconcurrency import Network
import Security

/// Transfers between two phones of the same network (Wi‑Fi, a hotspot, or Apple's
/// peer-to-peer Wi‑Fi): the receiver announces itself on the network under the name of
/// its code (Bonjour), the sender finds it with the code alone. The connection is TLS
/// with a key derived from the code, so only the phone that knows the code can send.
enum NearbyNetwork {
    static let serviceType = "_localink._tcp"

    /// TCP + TLS with the pre-shared key of `code`, peer-to-peer Wi‑Fi allowed.
    static func parameters(for code: ConnectionCode) -> NWParameters {
        let tcp = NWProtocolTCP.Options()
        tcp.enableKeepalive = true
        tcp.keepaliveIdle = 2
        tcp.noDelay = true

        let tls = NWProtocolTLS.Options()
        let options = tls.securityProtocolOptions
        let key = code.sharedSecret.withUnsafeBytes { DispatchData(bytes: $0) }
        let identity = Data("LOCALINK".utf8).withUnsafeBytes { DispatchData(bytes: $0) }
        sec_protocol_options_add_pre_shared_key(options, key as __DispatchData, identity as __DispatchData)
        // TLS_PSK_WITH_AES_128_GCM_SHA256 (0x00A8).
        let suite: tls_ciphersuite_t? = tls_ciphersuite_t(rawValue: 0x00A8)
        if let suite {
            sec_protocol_options_append_tls_ciphersuite(options, suite)
        }
        sec_protocol_options_set_min_tls_protocol_version(options, .TLSv12)
        sec_protocol_options_set_max_tls_protocol_version(options, .TLSv12)

        let parameters = NWParameters(tls: tls, tcp: tcp)
        parameters.includePeerToPeer = true
        return parameters
    }
}

enum NearbyError: LocalizedError {
    case notFound
    case interrupted
    case refused

    var errorDescription: String? {
        switch self {
        case .notFound:
            String(localized: "Aucun téléphone avec ce code à proximité. Vérifiez le code et que l'autre téléphone est sur l'écran Recevoir.")
        case .interrupted:
            String(localized: "Le transfert a été interrompu.")
        case .refused:
            String(localized: "La connexion a été refusée.")
        }
    }
}

/// Resumes a continuation once, whatever calls it first (a result, a timeout…).
final class Once<T>: @unchecked Sendable {
    private let lock = NSLock()
    private var continuation: CheckedContinuation<T, Error>?

    init(_ continuation: CheckedContinuation<T, Error>) {
        self.continuation = continuation
    }

    func resume(_ result: Result<T, Error>) {
        lock.lock()
        let continuation = self.continuation
        self.continuation = nil
        lock.unlock()
        continuation?.resume(with: result)
    }
}

// MARK: - Sending

/// Sends files to the phone that shows `code`.
enum NearbySender {
    /// Finds the receiver, connects and sends `files`; `progress` gets the fraction
    /// sent and the name of the current file.
    static func send(
        _ files: [PickedFile], to code: ConnectionCode, device: String,
        progress: @escaping @Sendable (Double, String) -> Void
    ) async throws {
        let parameters = NearbyNetwork.parameters(for: code)
        let endpoint = try await find(code, parameters: parameters)
        let queue = DispatchQueue(label: "localink.send")
        let connection = NWConnection(to: endpoint, using: parameters)
        defer { connection.cancel() }
        try await ready(connection, queue: queue)

        let total = max(1, files.reduce(Int64(0)) { $0 + $1.size })
        try await send(
            try Frame.json(.hello, Hello(device: device, files: files.count, totalSize: total)), on: connection)
        var sent: Int64 = 0
        for file in files {
            try Task.checkCancellation()
            try await send(try Frame.json(.offer, FileOffer(name: file.name, size: file.size, type: file.type)), on: connection)
            let handle = try FileHandle(forReadingFrom: file.url)
            defer { try? handle.close() }
            while let chunk = try handle.read(upToCount: Frame.chunkSize), !chunk.isEmpty {
                try Task.checkCancellation()
                try await send(Frame(.chunk, chunk), on: connection)
                sent += Int64(chunk.count)
                progress(Double(sent) / Double(total), file.name)
            }
            try await send(Frame(.end), on: connection)
        }
        try await send(Frame(.done), on: connection)
        try await waitForAck(connection)
        progress(1, files.last?.name ?? "")
    }

    /// The receiver announced under the name of `code`, within 20 seconds.
    private static func find(_ code: ConnectionCode, parameters: NWParameters) async throws -> NWEndpoint {
        let browsing = NWParameters()
        browsing.includePeerToPeer = true
        let browser = NWBrowser(for: .bonjour(type: NearbyNetwork.serviceType, domain: nil), using: browsing)
        let queue = DispatchQueue(label: "localink.browse")
        defer { browser.cancel() }
        return try await withCheckedThrowingContinuation { continuation in
            let once = Once(continuation)
            browser.browseResultsChangedHandler = { results, _ in
                for result in results {
                    if case .service(let name, _, _, _) = result.endpoint, name == code.serviceName {
                        once.resume(.success(result.endpoint))
                    }
                }
            }
            browser.stateUpdateHandler = { state in
                if case .failed(let error) = state { once.resume(.failure(error)) }
            }
            browser.start(queue: queue)
            queue.asyncAfter(deadline: .now() + 20) { once.resume(.failure(NearbyError.notFound)) }
        }
    }

    private static func ready(_ connection: NWConnection, queue: DispatchQueue) async throws {
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            let once = Once(continuation)
            connection.stateUpdateHandler = { state in
                switch state {
                case .ready: once.resume(.success(()))
                case .failed(let error), .waiting(let error): once.resume(.failure(error))
                case .cancelled: once.resume(.failure(NearbyError.interrupted))
                default: break
                }
            }
            connection.start(queue: queue)
            queue.asyncAfter(deadline: .now() + 20) { once.resume(.failure(NearbyError.refused)) }
        }
    }

    private static func send(_ frame: Frame, on connection: NWConnection) async throws {
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            connection.send(
                content: frame.encoded,
                completion: .contentProcessed { error in
                    if let error {
                        continuation.resume(throwing: error)
                    } else {
                        continuation.resume()
                    }
                })
        }
    }

    /// The receiver says everything is saved.
    private static func waitForAck(_ connection: NWConnection) async throws {
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            let once = Once(continuation)
            connection.receive(minimumIncompleteLength: Frame.headerSize, maximumLength: 64) { data, _, _, error in
                if let data, data.first == FrameKind.ack.rawValue {
                    once.resume(.success(()))
                } else {
                    once.resume(.failure(error ?? NearbyError.interrupted))
                }
            }
            DispatchQueue.global().asyncAfter(deadline: .now() + 30) {
                once.resume(.failure(NearbyError.interrupted))
            }
        }
    }
}

// MARK: - Receiving

/// What the receiver tells the screen.
enum IncomingEvent: Sendable {
    case started(device: String, files: Int, total: Int64)
    case progress(Double, file: String)
    case finished([URL])
    case failed(String)
}

/// The receiving side of one connection, confined to its own queue.
final class IncomingSession: @unchecked Sendable {
    private let connection: NWConnection
    private let queue = DispatchQueue(label: "localink.receive")
    private let onEvent: @Sendable (IncomingEvent) -> Void
    private var decoder = FrameDecoder()
    private var handle: FileHandle?
    private var current: URL?
    private var saved: [URL] = []
    private var total: Int64 = 1
    private var received: Int64 = 0
    private var lastReport = Date.distantPast
    private var finished = false

    init(_ connection: NWConnection, onEvent: @escaping @Sendable (IncomingEvent) -> Void) {
        self.connection = connection
        self.onEvent = onEvent
    }

    func start() {
        connection.stateUpdateHandler = { @Sendable [self] state in
            switch state {
            case .ready: receive()
            case .failed(let error): fail(error.localizedDescription)
            default: break
            }
        }
        connection.start(queue: queue)
    }

    func cancel() {
        queue.async { [self] in
            if !finished { fail(NearbyError.interrupted.localizedDescription) }
        }
    }

    private func receive() {
        connection.receive(minimumIncompleteLength: 1, maximumLength: 1 << 20) { @Sendable [self] data, _, isComplete, error in
            if let data, !data.isEmpty {
                do {
                    for frame in try decoder.feed(data) { try handle(frame) }
                } catch {
                    fail(error.localizedDescription)
                    return
                }
            }
            if finished { return }
            if let error {
                fail(error.localizedDescription)
            } else if isComplete {
                fail(NearbyError.interrupted.localizedDescription)
            } else {
                receive()
            }
        }
    }

    private func handle(_ frame: Frame) throws {
        switch frame.kind {
        case .hello:
            let hello = try frame.decode(Hello.self)
            total = max(1, hello.totalSize)
            onEvent(.started(device: hello.device, files: hello.files, total: hello.totalSize))
        case .offer:
            let offer = try frame.decode(FileOffer.self)
            let url = ReceivedFiles.destination(for: offer.name)
            FileManager.default.createFile(atPath: url.path, contents: nil)
            handle = try FileHandle(forWritingTo: url)
            current = url
        case .chunk:
            guard let handle else { throw NearbyError.interrupted }
            try handle.write(contentsOf: frame.payload)
            received += Int64(frame.payload.count)
            if Date().timeIntervalSince(lastReport) > 0.08 {
                lastReport = Date()
                onEvent(.progress(Double(received) / Double(total), file: current?.lastPathComponent ?? ""))
            }
        case .end:
            try handle?.close()
            handle = nil
            if let current { saved.append(current) }
            current = nil
        case .done:
            finished = true
            onEvent(.finished(saved))
            connection.send(
                content: Frame(.ack).encoded,
                completion: .contentProcessed { [connection] _ in connection.cancel() })
        case .cancel:
            fail(NearbyError.interrupted.localizedDescription)
        case .ack:
            break
        }
    }

    private func fail(_ message: String) {
        guard !finished else { return }
        finished = true
        try? handle?.close()
        handle = nil
        if let current { try? FileManager.default.removeItem(at: current) }
        onEvent(.failed(message))
        connection.cancel()
    }
}

/// Announces this phone under its code and receives what nearby phones send.
@MainActor
final class NearbyListener {
    private var listener: NWListener?
    private var sessions: [IncomingSession] = []
    private let queue = DispatchQueue(label: "localink.listen")

    /// Starts listening for `code`; `onEvent` is called on the main actor.
    func start(code: ConnectionCode, onEvent: @escaping @MainActor @Sendable (IncomingEvent) -> Void) throws {
        stop()
        let listener = try NWListener(using: NearbyNetwork.parameters(for: code))
        listener.service = NWListener.Service(name: code.serviceName, type: NearbyNetwork.serviceType)
        listener.newConnectionHandler = { @Sendable [weak self] connection in
            let session = IncomingSession(connection) { event in
                Task { @MainActor in onEvent(event) }
            }
            Task { @MainActor in self?.sessions.append(session) }
            session.start()
        }
        listener.stateUpdateHandler = { @Sendable state in
            if case .failed(let error) = state {
                Task { @MainActor in onEvent(.failed(error.localizedDescription)) }
            }
        }
        listener.start(queue: queue)
        self.listener = listener
    }

    func stop() {
        listener?.cancel()
        listener = nil
        sessions.forEach { $0.cancel() }
        sessions.removeAll()
    }
}
