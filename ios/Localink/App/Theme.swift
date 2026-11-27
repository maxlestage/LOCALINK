import SwiftUI

/// The colors of LOCALINK, the same as the website.
enum Theme {
    static let background = Color(red: 5 / 255, green: 6 / 255, blue: 15 / 255)
    static let backgroundRaised = Color(red: 16 / 255, green: 19 / 255, blue: 42 / 255)
    static let surface = Color.white.opacity(0.06)
    static let line = Color.white.opacity(0.1)
    static let muted = Color(red: 154 / 255, green: 163 / 255, blue: 199 / 255)
    static let cyan = Color(red: 45 / 255, green: 226 / 255, blue: 1)
    static let violet = Color(red: 124 / 255, green: 92 / 255, blue: 1)
    static let pink = Color(red: 1, green: 95 / 255, blue: 210 / 255)
    static let green = Color(red: 60 / 255, green: 242 / 255, blue: 166 / 255)
    static let red = Color(red: 1, green: 107 / 255, blue: 129 / 255)

    static let gradient = LinearGradient(
        colors: [cyan, violet, pink], startPoint: .topLeading, endPoint: .bottomTrailing)
}

/// The big gradient button.
struct PrimaryButtonStyle: ButtonStyle {
    @Environment(\.isEnabled) private var isEnabled

    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.headline)
            .foregroundStyle(Theme.background)
            .frame(maxWidth: .infinity, minHeight: 54)
            .padding(.horizontal, 20)
            .background(Theme.gradient, in: Capsule())
            .shadow(color: Theme.cyan.opacity(0.4), radius: configuration.isPressed ? 6 : 16, y: 8)
            .scaleEffect(configuration.isPressed ? 0.97 : 1)
            .opacity(isEnabled ? 1 : 0.5)
            .animation(.spring(response: 0.3, dampingFraction: 0.6), value: configuration.isPressed)
    }
}

/// The outlined button.
struct SecondaryButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .font(.headline)
            .foregroundStyle(.white)
            .frame(maxWidth: .infinity, minHeight: 50)
            .padding(.horizontal, 18)
            .background(Theme.surface, in: Capsule())
            .overlay(Capsule().strokeBorder(Theme.line))
            .scaleEffect(configuration.isPressed ? 0.97 : 1)
            .animation(.spring(response: 0.3, dampingFraction: 0.6), value: configuration.isPressed)
    }
}

/// A rounded panel.
struct Card<Content: View>: View {
    @ViewBuilder var content: Content

    var body: some View {
        VStack(alignment: .leading, spacing: 14) { content }
            .padding(18)
            .frame(maxWidth: .infinity, alignment: .leading)
            .background(Theme.surface, in: RoundedRectangle(cornerRadius: 24, style: .continuous))
            .overlay(RoundedRectangle(cornerRadius: 24, style: .continuous).strokeBorder(Theme.line))
    }
}

/// Slow blurred blobs behind every screen (still for those who ask for less motion).
struct AnimatedBackground: View {
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    var body: some View {
        TimelineView(.animation(paused: reduceMotion)) { timeline in
            let t = timeline.date.timeIntervalSinceReferenceDate
            Canvas { context, size in
                let blobs: [(Color, Double, Double)] = [
                    (Theme.violet, 0.15, 0.0), (Theme.cyan, 0.85, 2.1), (Theme.pink, 0.5, 4.2),
                ]
                for (color, x, phase) in blobs {
                    let dx = sin(t / 7 + phase) * size.width * 0.12
                    let dy = cos(t / 9 + phase) * size.height * 0.08
                    let radius = max(size.width, size.height) * 0.45
                    let center = CGPoint(x: size.width * x + dx, y: size.height * (0.2 + x * 0.5) + dy)
                    let rect = CGRect(
                        x: center.x - radius, y: center.y - radius, width: radius * 2, height: radius * 2)
                    context.fill(
                        Path(ellipseIn: rect),
                        with: .radialGradient(
                            Gradient(colors: [color.opacity(0.28), .clear]), center: center,
                            startRadius: 0, endRadius: radius))
                }
            }
        }
        .background(Theme.background)
        .ignoresSafeArea()
    }
}

/// The logo: two devices linked by a wave, drawn like on the website; the wave flows.
struct LogoMark: View {
    var size: CGFloat = 44
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    var body: some View {
        TimelineView(.animation(paused: reduceMotion)) { timeline in
            let phase = -CGFloat(timeline.date.timeIntervalSinceReferenceDate.truncatingRemainder(dividingBy: 1.6) / 1.6) * size * 0.28
            ZStack {
                RoundedRectangle(cornerRadius: size * 0.28, style: .continuous)
                    .fill(LinearGradient(colors: [Theme.cyan, Theme.violet], startPoint: .topLeading, endPoint: .bottomTrailing))
                Path { path in
                    path.move(to: CGPoint(x: size * 0.3, y: size * 0.62))
                    path.addCurve(
                        to: CGPoint(x: size * 0.7, y: size * 0.37),
                        control1: CGPoint(x: size * 0.38, y: size * 0.44),
                        control2: CGPoint(x: size * 0.62, y: size * 0.44))
                }
                .stroke(
                    Theme.background,
                    style: StrokeStyle(lineWidth: size * 0.07, lineCap: .round, dash: [size * 0.03, size * 0.11], dashPhase: phase))
                Circle().fill(Theme.background).frame(width: size * 0.22).position(x: size * 0.28, y: size * 0.66)
                Circle().fill(Theme.background).frame(width: size * 0.22).position(x: size * 0.72, y: size * 0.34)
            }
        }
        .frame(width: size, height: size)
        .shadow(color: Theme.cyan.opacity(0.4), radius: size * 0.3)
        .accessibilityHidden(true)
    }
}

/// A file size for people: `4,8 Mo`.
func formatSize(_ bytes: Int64) -> String {
    ByteCountFormatter.string(fromByteCount: bytes, countStyle: .file)
}
