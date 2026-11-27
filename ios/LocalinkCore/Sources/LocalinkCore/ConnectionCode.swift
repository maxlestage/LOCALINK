import Foundation

/// A LOCALINK code, `XXXX-XXXX`: what a phone shows (as text or as a QR code) so that
/// another one can find it, nearby or through the server.
///
/// The alphabet leaves out 0/O and 1/I/L, which are easy to mix up when a code is
/// typed; it is the same on the website and on the server.
public struct ConnectionCode: Hashable, Sendable, CustomStringConvertible, Codable {
    public static let alphabet: [Character] = Array("23456789ABCDEFGHJKMNPQRSTUVWXYZ")
    /// What a QR code of LOCALINK contains before the code.
    public static let qrPrefix = "LOCALINK:"

    /// `7KQ4-M9XP`.
    public let value: String

    /// A code as typed or scanned: `7kq4 m9xp`, `7KQ4M9XP`, `LOCALINK:7KQ4-M9XP`, or a
    /// link that ends with it. `nil` when it cannot be one.
    public init?(_ text: String) {
        var text = text.trimmingCharacters(in: .whitespacesAndNewlines)
        if text.uppercased().hasPrefix(Self.qrPrefix) {
            text = String(text.dropFirst(Self.qrPrefix.count))
        } else if let last = text.split(separator: "/").last, text.contains("://") {
            text = String(last)
        }
        let chars = text.uppercased().filter { !$0.isWhitespace && $0 != "-" }
        guard chars.count == 8, chars.allSatisfy(Self.alphabet.contains) else {
            return nil
        }
        value = String(chars.prefix(4)) + "-" + String(chars.suffix(4))
    }

    /// A new random code.
    public static func random() -> ConnectionCode {
        var generator = SystemRandomNumberGenerator()
        return random(using: &generator)
    }

    public static func random<G: RandomNumberGenerator>(using generator: inout G) -> ConnectionCode {
        let chars = (0..<8).map { _ in alphabet.randomElement(using: &generator)! }
        return ConnectionCode(String(chars))!
    }

    public var description: String { value }

    /// The code without its dash, `7KQ4M9XP`.
    public var compact: String { value.replacingOccurrences(of: "-", with: "") }

    /// What its QR code contains, as on the website: `LOCALINK:7KQ4-M9XP`.
    public var qrPayload: String { Self.qrPrefix + value }

    /// The name of the phone on the local network (Bonjour), found by the code alone.
    public var serviceName: String { "LOCALINK-" + compact }

    /// The secret both phones derive from the code: the transfer only works between
    /// the two phones that know it (TLS with a pre-shared key).
    public var sharedSecret: Data { Data(("localink/v1/" + compact).utf8) }

    /// The code of a service name found on the network, if it is one of LOCALINK.
    public init?(serviceName: String) {
        guard serviceName.hasPrefix("LOCALINK-") else { return nil }
        self.init(String(serviceName.dropFirst("LOCALINK-".count)))
    }
}

/// `2:00`, `0:07`: the time left before a share is erased.
public func clock(_ seconds: Int) -> String {
    let seconds = max(0, seconds)
    return "\(seconds / 60):" + (seconds % 60 < 10 ? "0" : "") + "\(seconds % 60)"
}
