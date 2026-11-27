import CoreImage
import CoreImage.CIFilterBuiltins
import UIKit

/// QR codes drawn by Core Image.
enum QRCodeImage {
    /// The QR code of `text`, sharp at any size (draw it with `.interpolation(.none)`).
    static func make(_ text: String) -> UIImage? {
        let context = CIContext()
        let filter = CIFilter.qrCodeGenerator()
        filter.message = Data(text.utf8)
        filter.correctionLevel = "M"
        guard let output = filter.outputImage?.transformed(by: CGAffineTransform(scaleX: 10, y: 10)),
            let image = context.createCGImage(output, from: output.extent)
        else { return nil }
        return UIImage(cgImage: image)
    }
}
