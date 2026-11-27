import Foundation
import Testing

@testable import LocalinkCore

@Suite struct ConnectionCodeTests {
    @Test func readsWhatPeopleType() {
        #expect(ConnectionCode("7kq4 m9xp")?.value == "7KQ4-M9XP")
        #expect(ConnectionCode("7KQ4M9XP")?.value == "7KQ4-M9XP")
        #expect(ConnectionCode("LOCALINK:7KQ4-M9XP")?.value == "7KQ4-M9XP")
        #expect(ConnectionCode("https://localink.example/c/7KQ4M9XP")?.value == "7KQ4-M9XP")
        #expect(ConnectionCode("7KQ4-M9X") == nil)
        #expect(ConnectionCode("0KQ4-M9XP") == nil, "0 is not in the alphabet")
        #expect(ConnectionCode("") == nil)
    }

    @Test func randomCodesAreValidAndDifferent() {
        var seen = Set<String>()
        for _ in 0..<500 {
            let code = ConnectionCode.random()
            #expect(ConnectionCode(code.value) == code)
            #expect(ConnectionCode(code.qrPayload) == code)
            #expect(ConnectionCode(serviceName: code.serviceName) == code)
            seen.insert(code.value)
        }
        #expect(seen.count > 495)
    }

    @Test func theAlphabetMatchesTheServer() {
        #expect(String(ConnectionCode.alphabet) == "23456789ABCDEFGHJKMNPQRSTUVWXYZ")
        for c in "0O1IL" { #expect(!ConnectionCode.alphabet.contains(c)) }
    }

    @Test func clocks() {
        #expect(clock(120) == "2:00")
        #expect(clock(65) == "1:05")
        #expect(clock(7) == "0:07")
        #expect(clock(-3) == "0:00")
    }
}

@Suite struct ProtocolTests {
    @Test func framesSurviveAnyCutting() throws {
        let offer = FileOffer(name: "vacances été.jpg", size: 5, type: "image/jpeg")
        let frames = [
            try Frame.json(.hello, Hello(device: "iPhone de Léa", files: 1, totalSize: 5)),
            try Frame.json(.offer, offer),
            Frame(.chunk, Data([1, 2, 3, 4, 5])),
            Frame(.end),
            Frame(.done),
        ]
        let stream = frames.reduce(Data()) { $0 + $1.encoded }
        for size in [1, 2, 3, 7, 64, stream.count] {
            var decoder = FrameDecoder()
            var out: [Frame] = []
            var index = 0
            while index < stream.count {
                let end = min(index + size, stream.count)
                out += try decoder.feed(stream.subdata(in: index..<end))
                index = end
            }
            #expect(out == frames, "pieces of \(size)")
            #expect(decoder.pending == 0)
        }
        #expect(try frames[1].decode(FileOffer.self) == offer)
    }

    @Test func garbageIsRefused() {
        var decoder = FrameDecoder()
        #expect(throws: ProtocolError.unknownKind(99)) { try decoder.feed(Data([99, 0, 0, 0, 0])) }
        var other = FrameDecoder()
        #expect(throws: ProtocolError.tooLarge(0x7fff_ffff)) {
            try other.feed(Data([3, 0x7f, 0xff, 0xff, 0xff]))
        }
    }

    @Test func fileNamesAreSafe() {
        #expect(cleanFileName("../../etc/passwd") == "passwd")
        #expect(cleanFileName("..\\x\\note.txt") == "note.txt")
        #expect(cleanFileName(".env") == "env")
        #expect(cleanFileName("a\u{0}b\"c.txt") == "abc.txt")
        #expect(cleanFileName("") == "fichier")
        #expect(cleanFileName("été 2026.jpg") == "été 2026.jpg")
        let taken: Set<String> = ["photo.jpg", "photo 2.jpg", "notes"]
        #expect(uniqueFileName("photo.jpg", exists: taken.contains) == "photo 3.jpg")
        #expect(uniqueFileName("notes", exists: taken.contains) == "notes 2")
        #expect(uniqueFileName("new.pdf", exists: taken.contains) == "new.pdf")
    }
}

@Suite struct ShareAPITests {
    let api = ShareAPI(baseURL: URL(string: "https://localink.example")!)

    @Test func requests() throws {
        let request = api.createRequest(fileName: "vacances été.jpg", mimeType: "image/jpeg")
        #expect(request.url?.absoluteString == "https://localink.example/api/shares")
        #expect(request.httpMethod == "POST")
        #expect(request.value(forHTTPHeaderField: "X-File-Name") == "vacances%20%C3%A9t%C3%A9.jpg")
        #expect(request.value(forHTTPHeaderField: "Content-Type") == "image/jpeg")
        let code = try #require(ConnectionCode("7KQ4M9XP"))
        #expect(api.fileURL(code).absoluteString == "https://localink.example/api/shares/7KQ4-M9XP")
        #expect(api.infoURL(code).absoluteString == "https://localink.example/api/shares/7KQ4-M9XP/info")
    }

    @Test func answersAreDecoded() throws {
        let json = #"{"code":"7KQ4-M9XP","token":"ab12","name":"vacances été.jpg","size":200000,"expires_in":120}"#
        let share = try JSONDecoder().decode(CreatedShare.self, from: Data(json.utf8))
        #expect(share.code.value == "7KQ4-M9XP")
        #expect(share.expiresIn == 120)
        let delete = api.deleteRequest(share)
        #expect(delete.httpMethod == "DELETE")
        #expect(delete.value(forHTTPHeaderField: "X-Localink-Token") == "ab12")
        let info = try JSONDecoder().decode(
            ShareInfo.self,
            from: Data(#"{"code":"7KQ4-M9XP","name":"a.txt","size":3,"type":"text/plain","expires_in":95}"#.utf8))
        #expect(info.expiresIn == 95)
        #expect(ShareError(status: 404) == .notFound)
        #expect(ShareError(status: 413) == .tooLarge)
        #expect(ShareError(status: 507) == .serverFull)
        #expect(ShareError(status: 201) == nil)
    }

    @Test func namesRoundTripLikeTheServer() {
        #expect(percentEncode("été 1.jpg") == "%C3%A9t%C3%A9%201.jpg")
        #expect(decodeFileName("vacances%20%C3%A9t%C3%A9.jpg") == "vacances été.jpg")
        #expect(decodeFileName("..%2F..%2Fpasswd") == "passwd")
        #expect(decodeFileName(nil) == nil)
    }
}
