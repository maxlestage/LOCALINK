import SwiftUI

/// Three tabs: send, receive, and the files received.
struct RootView: View {
    /// The tab to open: the first one, or `-tab 1` at launch (the CI's screenshots).
    @State private var tab = UserDefaults.standard.integer(forKey: "tab")

    var body: some View {
        TabView(selection: $tab) {
            SendView()
                .tabItem { Label("Envoyer", systemImage: "paperplane.fill") }
                .tag(0)
            ReceiveView()
                .tabItem { Label("Recevoir", systemImage: "tray.and.arrow.down.fill") }
                .tag(1)
            ReceivedListView()
                .tabItem { Label("Reçus", systemImage: "folder.fill") }
                .tag(2)
        }
    }
}

/// The title of a screen, with the logo.
struct ScreenHeader: View {
    let title: LocalizedStringKey
    let subtitle: LocalizedStringKey

    var body: some View {
        HStack(alignment: .center, spacing: 14) {
            LogoMark(size: 46)
            VStack(alignment: .leading, spacing: 2) {
                Text(title).font(.largeTitle.bold())
                Text(subtitle).font(.subheadline).foregroundStyle(Theme.muted)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

/// Side by side, or far apart.
enum Distance: Hashable {
    case nearby
    case remote
}

/// The choice between the two ways of sending.
struct DistancePicker: View {
    @Binding var distance: Distance

    var body: some View {
        HStack(spacing: 10) {
            option(.nearby, title: "À côté", detail: "Même réseau, en direct", symbol: "wifi")
            option(.remote, title: "À distance", detail: "Effacé après 2 min", symbol: "timer")
        }
    }

    private func option(_ value: Distance, title: LocalizedStringKey, detail: LocalizedStringKey, symbol: String) -> some View {
        Button {
            withAnimation(.spring(response: 0.4, dampingFraction: 0.75)) { distance = value }
        } label: {
            VStack(alignment: .leading, spacing: 6) {
                Image(systemName: symbol)
                    .font(.title2)
                    .symbolEffect(.bounce, value: distance == value)
                Text(title).font(.headline)
                Text(detail).font(.caption).foregroundStyle(Theme.muted)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(14)
            .background(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .fill(distance == value ? Theme.cyan.opacity(0.14) : Theme.surface))
            .overlay(
                RoundedRectangle(cornerRadius: 20, style: .continuous)
                    .strokeBorder(distance == value ? Theme.cyan : Theme.line, lineWidth: distance == value ? 1.5 : 1))
        }
        .buttonStyle(.plain)
        .accessibilityAddTraits(distance == value ? .isSelected : [])
    }
}

/// A bar that fills, with a sheen moving across.
struct ProgressBar: View {
    let value: Double

    var body: some View {
        GeometryReader { proxy in
            ZStack(alignment: .leading) {
                Capsule().fill(Theme.surface)
                Capsule()
                    .fill(Theme.gradient)
                    .frame(width: max(8, proxy.size.width * min(1, max(0, value))))
                    .animation(.easeOut(duration: 0.25), value: value)
            }
        }
        .frame(height: 10)
        .accessibilityElement()
        .accessibilityValue(Text(value, format: .percent.precision(.fractionLength(0))))
    }
}

/// The ring of the 2 minutes of a share.
struct CountdownRing: View {
    let expires: Date
    var lifetime: TimeInterval = 120

    var body: some View {
        TimelineView(.periodic(from: .now, by: 0.5)) { timeline in
            let left = max(0, expires.timeIntervalSince(timeline.date))
            let ending = left <= 20
            ZStack {
                Circle().stroke(Theme.surface, lineWidth: 8)
                Circle()
                    .trim(from: 0, to: left / lifetime)
                    .stroke(ending ? Theme.red : Theme.cyan, style: StrokeStyle(lineWidth: 8, lineCap: .round))
                    .rotationEffect(.degrees(-90))
                    .animation(.linear(duration: 0.5), value: left)
                VStack(spacing: 0) {
                    Text(left > 0 ? LocalizedStringKey("S'efface dans") : LocalizedStringKey("Effacé du serveur"))
                        .font(.caption2)
                        .foregroundStyle(left > 0 ? Theme.muted : Theme.green)
                    Text(clockText(left))
                        .font(.system(.title, design: .monospaced).bold())
                        .foregroundStyle(ending ? Theme.red : .white)
                        .contentTransition(.numericText(countsDown: true))
                }
            }
            .frame(width: 140, height: 140)
        }
    }

    private func clockText(_ left: TimeInterval) -> String {
        let seconds = Int(left.rounded(.up))
        return "\(seconds / 60):" + String(format: "%02d", seconds % 60)
    }
}

/// A message under an action.
struct StatusLine: View {
    let text: String
    var symbol = "info.circle"
    var color = Theme.muted

    var body: some View {
        Label(text, systemImage: symbol)
            .font(.subheadline)
            .foregroundStyle(color)
            .frame(maxWidth: .infinity, alignment: .leading)
            .transition(.move(edge: .bottom).combined(with: .opacity))
    }
}
