import SwiftUI

/// LOCALINK: files from one phone to another, side by side (directly, on the same
/// network) or far apart (through the server, erased after 2 minutes). A QR code or a
/// code, the user's choice, and no account.
@main
struct LocalinkApp: App {
    @State private var received = ReceivedFiles()

    var body: some Scene {
        WindowGroup {
            RootView()
                .environment(received)
                .preferredColorScheme(.dark)
                .tint(Theme.cyan)
        }
    }
}
