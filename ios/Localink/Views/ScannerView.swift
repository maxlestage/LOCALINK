import LocalinkCore
import SwiftUI
import VisionKit

/// The camera, looking for the QR code of LOCALINK; a message where the device cannot scan.
struct ScannerPanel: View {
    let onCode: (ConnectionCode) -> Void
    @State private var found = false

    var body: some View {
        Group {
            if ScannerView.isAvailable {
                ScannerView { code in
                    guard !found else { return }
                    found = true
                    onCode(code)
                }
                .overlay(alignment: .center) {
                    RoundedRectangle(cornerRadius: 24, style: .continuous)
                        .strokeBorder(Theme.cyan, style: StrokeStyle(lineWidth: 3, dash: [28, 14]))
                        .padding(36)
                        .allowsHitTesting(false)
                }
            } else {
                VStack(spacing: 10) {
                    Image(systemName: "camera.metering.unknown").font(.largeTitle)
                    Text("Le scanner n'est pas disponible : autorisez l'appareil photo dans Réglages, ou tapez le code.")
                        .multilineTextAlignment(.center)
                        .foregroundStyle(Theme.muted)
                }
                .padding()
            }
        }
        .frame(height: 300)
        .frame(maxWidth: .infinity)
        .background(Color.black, in: RoundedRectangle(cornerRadius: 24, style: .continuous))
        .clipShape(RoundedRectangle(cornerRadius: 24, style: .continuous))
        .sensoryFeedback(.success, trigger: found)
    }
}

/// VisionKit's live scanner, for QR codes only.
struct ScannerView: UIViewControllerRepresentable {
    let onCode: (ConnectionCode) -> Void

    static var isAvailable: Bool {
        DataScannerViewController.isSupported && DataScannerViewController.isAvailable
    }

    func makeUIViewController(context: Context) -> DataScannerViewController {
        let scanner = DataScannerViewController(
            recognizedDataTypes: [.barcode(symbologies: [.qr])],
            qualityLevel: .balanced,
            recognizesMultipleItems: false,
            isHighFrameRateTrackingEnabled: false,
            isHighlightingEnabled: true)
        scanner.delegate = context.coordinator
        DispatchQueue.main.async { try? scanner.startScanning() }
        return scanner
    }

    func updateUIViewController(_ controller: DataScannerViewController, context: Context) {}

    static func dismantleUIViewController(_ controller: DataScannerViewController, coordinator: Coordinator) {
        controller.stopScanning()
    }

    func makeCoordinator() -> Coordinator { Coordinator(onCode: onCode) }

    @MainActor
    final class Coordinator: NSObject, DataScannerViewControllerDelegate {
        let onCode: (ConnectionCode) -> Void

        init(onCode: @escaping (ConnectionCode) -> Void) {
            self.onCode = onCode
        }

        func dataScanner(
            _ dataScanner: DataScannerViewController, didAdd addedItems: [RecognizedItem], allItems: [RecognizedItem]
        ) {
            for item in addedItems {
                if case .barcode(let barcode) = item, let payload = barcode.payloadStringValue,
                    let code = ConnectionCode(payload)
                {
                    onCode(code)
                    return
                }
            }
        }
    }
}
