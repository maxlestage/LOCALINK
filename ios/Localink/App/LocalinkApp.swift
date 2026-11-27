import SwiftUI

/// LOCALINK: files from one phone to another, side by side (directly, on the same
/// network) or far apart (through the server, erased after 2 minutes). A QR code or a
/// code, the user's choice, and no account.
@main
struct LocalinkApp: App {
    @State private var received = ReceivedFiles()
    @State private var splash = true
    @AppStorage(Preferences.onboarded) private var onboarded = false

    var body: some Scene {
        WindowGroup {
            ZStack {
                RootView()
                if splash {
                    SplashView {
                        withAnimation(.easeOut(duration: 0.45)) { splash = false }
                    }
                    .transition(.opacity.combined(with: .scale(scale: 1.08)))
                    .zIndex(1)
                }
            }
            .environment(received)
            .preferredColorScheme(.dark)
            .tint(Theme.cyan)
            .fullScreenCover(
                isPresented: Binding(get: { !splash && !onboarded }, set: { if !$0 { onboarded = true } })
            ) {
                OnboardingView { onboarded = true }
                    .preferredColorScheme(.dark)
            }
        }
    }
}
