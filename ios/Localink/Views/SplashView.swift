import SwiftUI

/// The start of the app: the logo arrives, the letters of LOCALINK rise one after the
/// other, a line links them, then everything fades into the app.
struct SplashView: View {
    let onFinish: () -> Void
    @State private var phase = 0
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    private let letters = Array("LOCALINK")

    var body: some View {
        ZStack {
            AnimatedBackground()
            VStack(spacing: 26) {
                ZStack {
                    ForEach(0..<3) { ring in
                        Circle()
                            .stroke(Theme.cyan.opacity(0.5), lineWidth: 1.5)
                            .frame(width: 112, height: 112)
                            .scaleEffect(phase >= 1 ? 1.6 + CGFloat(ring) * 0.45 : 0.6)
                            .opacity(phase >= 1 ? 0 : 0.8)
                            .animation(.easeOut(duration: 1.3).delay(Double(ring) * 0.18), value: phase)
                    }
                    LogoMark(size: 112)
                        .scaleEffect(phase >= 1 ? 1 : 0.2)
                        .rotationEffect(.degrees(phase >= 1 ? 0 : -40))
                        .opacity(phase >= 1 ? 1 : 0)
                        .animation(.spring(response: 0.6, dampingFraction: 0.6), value: phase)
                }
                HStack(spacing: 1) {
                    ForEach(letters.indices, id: \.self) { index in
                        Text(String(letters[index]))
                            .font(.system(size: 40, weight: .heavy, design: .rounded))
                            .foregroundStyle(Theme.gradient)
                            .offset(y: phase >= 2 ? 0 : 34)
                            .rotationEffect(.degrees(phase >= 2 ? 0 : 12))
                            .opacity(phase >= 2 ? 1 : 0)
                            .animation(
                                .spring(response: 0.5, dampingFraction: 0.65).delay(Double(index) * 0.05),
                                value: phase)
                    }
                }
                .accessibilityElement()
                .accessibilityLabel(Text(verbatim: "LOCALINK"))
                Capsule()
                    .fill(Theme.gradient)
                    .frame(width: phase >= 3 ? 170 : 0, height: 4)
                    .shadow(color: Theme.cyan, radius: 8)
                    .animation(.easeInOut(duration: 0.55), value: phase)
                Text("Près ou loin, sans compte")
                    .font(.subheadline)
                    .foregroundStyle(Theme.muted)
                    .opacity(phase >= 3 ? 1 : 0)
                    .animation(.easeIn(duration: 0.4), value: phase)
            }
        }
        .task {
            if reduceMotion {
                phase = 3
                try? await Task.sleep(for: .milliseconds(500))
                onFinish()
                return
            }
            try? await Task.sleep(for: .milliseconds(120))
            phase = 1
            try? await Task.sleep(for: .milliseconds(380))
            phase = 2
            try? await Task.sleep(for: .milliseconds(520))
            phase = 3
            try? await Task.sleep(for: .milliseconds(750))
            // `-holdSplash YES` at launch keeps it on screen (the CI's screenshot).
            if UserDefaults.standard.bool(forKey: "holdSplash") { return }
            onFinish()
        }
    }
}
