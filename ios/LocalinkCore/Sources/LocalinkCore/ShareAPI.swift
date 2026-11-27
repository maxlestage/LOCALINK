import Foundation

#if canImport(FoundationNetworking)
    import FoundationNetworking
#endif

/// The server of LOCALINK, for the shares between phones far apart: a file stays there
/// 2 minutes, then it is erased.
public struct ShareAPI: Sendable {
    /// How long a share lives on the server.
    public static let lifetime: TimeInterval = 120
    /// The largest file the server accepts.
    public static let maxFileSize: Int64 = 25 * 1024 * 1024

    public var baseURL: URL

    public init(baseURL: URL) {
        self.baseURL = baseURL
    }

    var sharesURL: URL { baseURL.appendingPathComponent("api").appendingPathComponent("shares") }

    /// The request that uploads a file (its bytes are the body).
    public func createRequest(fileName: String, mimeType: String?) -> URLRequest {
        var request = URLRequest(url: sharesURL)
        request.httpMethod = "POST"
        request.setValue(mimeType ?? "application/octet-stream", forHTTPHeaderField: "Content-Type")
        request.setValue(percentEncode(fileName), forHTTPHeaderField: "X-File-Name")
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        request.timeoutInterval = 60
        return request
    }

    /// Where the file of a code is downloaded.
    public func fileURL(_ code: ConnectionCode) -> URL {
        sharesURL.appendingPathComponent(code.value)
    }

    /// What a code holds (name, size, time left), without downloading it.
    public func infoURL(_ code: ConnectionCode) -> URL {
        fileURL(code).appendingPathComponent("info")
    }

    /// The request that erases a share before its 2 minutes are over.
    public func deleteRequest(_ share: CreatedShare) -> URLRequest {
        var request = URLRequest(url: fileURL(share.code))
        request.httpMethod = "DELETE"
        request.setValue(share.token, forHTTPHeaderField: "X-Localink-Token")
        return request
    }
}

/// The answer to an upload.
public struct CreatedShare: Decodable, Equatable, Sendable {
    public var code: ConnectionCode
    public var token: String
    public var name: String
    public var size: Int64
    public var expiresIn: Int

    enum CodingKeys: String, CodingKey {
        case code, token, name, size
        case expiresIn = "expires_in"
    }

    public init(from decoder: Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        let raw = try c.decode(String.self, forKey: .code)
        guard let code = ConnectionCode(raw) else {
            throw DecodingError.dataCorruptedError(forKey: .code, in: c, debugDescription: raw)
        }
        self.code = code
        token = try c.decode(String.self, forKey: .token)
        name = try c.decode(String.self, forKey: .name)
        size = try c.decode(Int64.self, forKey: .size)
        expiresIn = try c.decode(Int.self, forKey: .expiresIn)
    }
}

/// What a code holds.
public struct ShareInfo: Decodable, Equatable, Sendable {
    public var name: String
    public var size: Int64
    public var type: String
    public var expiresIn: Int

    enum CodingKeys: String, CodingKey {
        case name, size, type
        case expiresIn = "expires_in"
    }
}

/// What can go wrong with the server, by status.
public enum ShareError: Error, Equatable, Sendable {
    case notFound
    case tooLarge
    case serverFull
    case empty
    case server(Int)

    public init?(status: Int) {
        switch status {
        case 200..<300: return nil
        case 400: self = .empty
        case 404: self = .notFound
        case 413: self = .tooLarge
        case 507: self = .serverFull
        default: self = .server(status)
        }
    }
}

/// Percent-encodes everything but unreserved characters (RFC 3986), like the server.
public func percentEncode(_ text: String) -> String {
    var out = ""
    for byte in text.utf8 {
        let c = Character(UnicodeScalar(byte))
        if (byte < 0x80 && (c.isLetter || c.isNumber)) || "-._~".contains(c) {
            out.append(c)
        } else {
            out += String(format: "%%%02X", byte)
        }
    }
    return out
}

/// The name the server gives back (`X-Localink-Name`), decoded.
public func decodeFileName(_ header: String?) -> String? {
    guard let header, let decoded = header.removingPercentEncoding else { return nil }
    return cleanFileName(decoded)
}
