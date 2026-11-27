import LocalinkCore
import SwiftUI

/// How a code is given or taken: as text or as a QR code, the user's choice.
enum CodeMethod: Hashable {
    case code
    case qr
}

/// The choice "Code / QR code".
struct CodeMethodPicker: View {
    @Binding var method: CodeMethod
    var codeLabel: LocalizedStringKey = "Code"
    var qrLabel: LocalizedStringKey = "QR code"

    var body: some View {
        Picker("Se connecter avec", selection: $method.animation(.spring(response: 0.4, dampingFraction: 0.8))) {
            Label(codeLabel, systemImage: "key.horizontal").tag(CodeMethod.code)
            Label(qrLabel, systemImage: "qrcode").tag(CodeMethod.qr)
        }
        .pickerStyle(.segmented)
    }
}

/// A code shown to another phone: its text or its QR code.
struct CodeCard: View {
    let code: ConnectionCode
    var title: LocalizedStringKey = "Mon code LOCALINK"
    @State private var method = CodeMethod.code
    @State private var copied = false

    var body: some View {
        VStack(spacing: 16) {
            Text(title).font(.subheadline).foregroundStyle(Color(white: 0.35))
            CodeMethodPicker(method: $method)
            Group {
                switch method {
                case .code:
                    VStack(spacing: 10) {
                        Text(code.value)
                            .font(.system(size: 40, weight: .bold, design: .monospaced))
                            .kerning(4)
                            .foregroundStyle(Color(red: 11 / 255, green: 127 / 255, blue: 149 / 255))
                            .minimumScaleFactor(0.6)
                            .lineLimit(1)
                            .padding(.vertical, 16)
                            .frame(maxWidth: .infinity)
                            .background(
                                Color(red: 227 / 255, green: 246 / 255, blue: 250 / 255),
                                in: RoundedRectangle(cornerRadius: 18, style: .continuous))
                            .contentTransition(.numericText())
                            .accessibilityLabel(Text(code.value.map(String.init).joined(separator: " ")))
                        Text(verbatim: "localink · \(code.compact)")
                            .font(.system(.footnote, design: .monospaced))
                            .foregroundStyle(Color(white: 0.4))
                    }
                    .frame(minHeight: 220)
                case .qr:
                    QRView(text: code.qrPayload)
                        .frame(width: 220, height: 220)
                        .accessibilityLabel(Text("QR code de \(code.value)"))
                }
            }
            .transition(.asymmetric(insertion: .scale(scale: 0.9).combined(with: .opacity), removal: .opacity))
            Button {
                UIPasteboard.general.string = code.value
                withAnimation { copied = true }
                Task {
                    try? await Task.sleep(for: .seconds(1.6))
                    withAnimation { copied = false }
                }
            } label: {
                Label(copied ? LocalizedStringKey("Code copié") : LocalizedStringKey("Copier le code"), systemImage: copied ? "checkmark" : "doc.on.doc")
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 16)
                    .padding(.vertical, 10)
                    .background(Color.white, in: Capsule())
                    .overlay(Capsule().strokeBorder(Color(white: 0.85)))
                    .foregroundStyle(Color(red: 16 / 255, green: 19 / 255, blue: 42 / 255))
            }
            .sensoryFeedback(.success, trigger: copied)
        }
        .padding(20)
        .frame(maxWidth: .infinity)
        // A light card in a dark app: its controls take the light style, readable on white.
        .environment(\.colorScheme, .light)
        .background(Color(red: 245 / 255, green: 247 / 255, blue: 1), in: RoundedRectangle(cornerRadius: 28, style: .continuous))
        .shadow(color: Theme.cyan.opacity(0.3), radius: 30, y: 12)
    }
}

/// A QR code, with a scan line moving over it.
struct QRView: View {
    let text: String
    @State private var scanning = false
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 18, style: .continuous).fill(.white)
            if let image = QRCodeImage.make(text) {
                Image(uiImage: image)
                    .interpolation(.none)
                    .resizable()
                    .scaledToFit()
                    .padding(14)
            }
            GeometryReader { proxy in
                Rectangle()
                    .fill(Theme.cyan.opacity(0.8))
                    .frame(height: 3)
                    .shadow(color: Theme.cyan, radius: 6)
                    .offset(y: scanning ? proxy.size.height - 20 : 14)
                    .padding(.horizontal, 10)
            }
            .opacity(reduceMotion ? 0 : 1)
        }
        .onAppear {
            withAnimation(.easeInOut(duration: 1.6).repeatForever(autoreverses: true)) { scanning = true }
        }
    }
}

/// Takes a code from another phone: typed or scanned, the user's choice.
struct CodeEntryView: View {
    let title: LocalizedStringKey
    let onCode: (ConnectionCode) -> Void
    @State private var method = CodeMethod.code
    @State private var text = ""
    @FocusState private var focused: Bool

    private var code: ConnectionCode? { ConnectionCode(text) }

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text(title).font(.headline)
            CodeMethodPicker(method: $method, codeLabel: "Taper le code", qrLabel: "Scanner le QR code")
            switch method {
            case .code:
                VStack(spacing: 12) {
                    TextField("XXXX-XXXX", text: $text)
                        .font(.system(size: 32, weight: .bold, design: .monospaced))
                        .multilineTextAlignment(.center)
                        .textInputAutocapitalization(.characters)
                        .autocorrectionDisabled()
                        .keyboardType(.asciiCapable)
                        .submitLabel(.go)
                        .focused($focused)
                        .padding(.vertical, 14)
                        .background(Theme.surface, in: RoundedRectangle(cornerRadius: 18, style: .continuous))
                        .overlay(
                            RoundedRectangle(cornerRadius: 18, style: .continuous)
                                .strokeBorder(code == nil ? Theme.line : Theme.green, lineWidth: 1.5))
                        .onChange(of: text) { _, new in text = format(new) }
                        .onSubmit(submit)
                    Button("Valider", action: submit)
                        .buttonStyle(PrimaryButtonStyle())
                        .disabled(code == nil)
                        .sensoryFeedback(.impact, trigger: code != nil)
                }
                .transition(.opacity)
            case .qr:
                ScannerPanel { scanned in
                    text = scanned.value
                    onCode(scanned)
                }
                .transition(.opacity)
            }
        }
    }

    private func submit() {
        guard let code else { return }
        focused = false
        onCode(code)
    }

    /// Upper case, the alphabet only, a dash after four characters.
    private func format(_ raw: String) -> String {
        let chars = raw.uppercased().filter { ConnectionCode.alphabet.contains($0) }.prefix(8)
        let value = String(chars)
        return value.count > 4 ? String(value.prefix(4)) + "-" + String(value.dropFirst(4)) : value
    }
}
