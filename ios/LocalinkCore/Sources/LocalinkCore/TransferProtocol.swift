import Foundation

/// The protocol between two phones on the same network.
///
/// Every message is a frame: one byte for its kind, four bytes (big endian) for the
/// length of its payload, then the payload. A transfer is:
///
/// ```text
/// hello(Hello) · offer(FileOffer) · chunk(bytes)… · end · offer… · done
/// ```
///
/// and the receiver answers `ack` once everything is saved.
public enum FrameKind: UInt8, Sendable, CaseIterable {
    case hello = 1
    case offer = 2
    case chunk = 3
    case end = 4
    case done = 5
    case ack = 6
    case cancel = 7
}

/// A decoded frame.
public struct Frame: Equatable, Sendable {
    public var kind: FrameKind
    public var payload: Data

    public init(_ kind: FrameKind, _ payload: Data = Data()) {
        self.kind = kind
        self.payload = payload
    }

    public static let headerSize = 5
    /// The largest payload accepted: chunks are smaller, so a frame announcing more is
    /// garbage or an attack.
    public static let maxPayload = 4 * 1024 * 1024
    /// The size of the chunks a file is cut into.
    public static let chunkSize = 256 * 1024

    /// The bytes of the frame, ready to send.
    public var encoded: Data {
        var data = Data(capacity: Self.headerSize + payload.count)
        data.append(kind.rawValue)
        let length = UInt32(payload.count)
        data.append(contentsOf: [
            UInt8(truncatingIfNeeded: length >> 24), UInt8(truncatingIfNeeded: length >> 16),
            UInt8(truncatingIfNeeded: length >> 8), UInt8(truncatingIfNeeded: length),
        ])
        data.append(payload)
        return data
    }

    /// A frame carrying `value` in JSON.
    public static func json<T: Encodable>(_ kind: FrameKind, _ value: T) throws -> Frame {
        Frame(kind, try JSONEncoder().encode(value))
    }

    /// The JSON payload, decoded.
    public func decode<T: Decodable>(_ type: T.Type) throws -> T {
        try JSONDecoder().decode(type, from: payload)
    }
}

/// What the sender says first.
public struct Hello: Codable, Equatable, Sendable {
    public var version: Int
    public var device: String
    public var files: Int
    public var totalSize: Int64

    public init(version: Int = 1, device: String, files: Int, totalSize: Int64) {
        self.version = version
        self.device = device
        self.files = files
        self.totalSize = totalSize
    }
}

/// A file about to come.
public struct FileOffer: Codable, Equatable, Sendable {
    public var name: String
    public var size: Int64
    public var type: String?

    public init(name: String, size: Int64, type: String? = nil) {
        self.name = name
        self.size = size
        self.type = type
    }
}

public enum ProtocolError: Error, Equatable, Sendable {
    case unknownKind(UInt8)
    case tooLarge(Int)
}

/// Cuts a stream of bytes, received in pieces of any size, into frames.
public struct FrameDecoder: Sendable {
    private var buffer = Data()

    public init() {}

    /// Adds received bytes; returns the frames they complete.
    public mutating func feed(_ data: Data) throws -> [Frame] {
        buffer.append(data)
        var frames: [Frame] = []
        while buffer.count >= Frame.headerSize {
            let start = buffer.startIndex
            let rawKind = buffer[start]
            guard let kind = FrameKind(rawValue: rawKind) else {
                throw ProtocolError.unknownKind(rawKind)
            }
            let length =
                Int(buffer[start + 1]) << 24 | Int(buffer[start + 2]) << 16
                | Int(buffer[start + 3]) << 8 | Int(buffer[start + 4])
            guard length <= Frame.maxPayload else { throw ProtocolError.tooLarge(length) }
            guard buffer.count >= Frame.headerSize + length else { break }
            let payloadStart = start + Frame.headerSize
            let payload = Data(buffer[payloadStart..<payloadStart + length])
            buffer = Data(buffer[(payloadStart + length)...])
            frames.append(Frame(kind, payload))
        }
        return frames
    }

    /// Bytes received that do not make a whole frame yet.
    public var pending: Int { buffer.count }
}

/// A file name safe to save: no folders, no control characters, at most 120
/// characters, `fichier` when nothing is left. The same rules as the server.
public func cleanFileName(_ raw: String) -> String {
    let base = raw.split(whereSeparator: { $0 == "/" || $0 == "\\" }).last.map(String.init) ?? ""
    let forbidden: Set<Character> = ["\"", "<", ">", "|", ":", "*", "?"]
    var name = String(base.filter { !$0.isASCII || (!$0.isNewline && $0.asciiValue! >= 0x20 && !forbidden.contains($0)) }.prefix(120))
    name = name.trimmingCharacters(in: .whitespaces)
    while name.hasPrefix(".") { name.removeFirst() }
    name = name.trimmingCharacters(in: .whitespaces)
    return name.isEmpty ? "fichier" : name
}

/// `photo.jpg`, then `photo 2.jpg`, `photo 3.jpg`… : the first name `exists` refuses.
public func uniqueFileName(_ name: String, exists: (String) -> Bool) -> String {
    guard exists(name) else { return name }
    let ext = (name as NSString).pathExtension
    let stem = (name as NSString).deletingPathExtension
    var n = 2
    while true {
        let candidate = ext.isEmpty ? "\(stem) \(n)" : "\(stem) \(n).\(ext)"
        if !exists(candidate) { return candidate }
        n += 1
    }
}
