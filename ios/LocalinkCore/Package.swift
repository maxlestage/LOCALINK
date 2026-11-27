// swift-tools-version: 6.0
// The logic of LOCALINK that does not depend on iOS: connection codes, the transfer
// protocol between two phones and the client of the share API. Tested on macOS and
// on Linux (`swift test`).

import PackageDescription

let package = Package(
    name: "LocalinkCore",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "LocalinkCore", targets: ["LocalinkCore"])
    ],
    targets: [
        .target(name: "LocalinkCore"),
        .testTarget(name: "LocalinkCoreTests", dependencies: ["LocalinkCore"]),
    ]
)
