import CoreGraphics
import Foundation
import ImageIO

let args = CommandLine.arguments
guard args.count == 2 else {
    fputs("usage: cg-sample-pixels.swift <png>\n", stderr)
    exit(2)
}

let url = URL(fileURLWithPath: args[1])
guard let source = CGImageSourceCreateWithURL(url as CFURL, nil),
      let image = CGImageSourceCreateImageAtIndex(source, 0, nil) else {
    fputs("unable to read image: \(args[1])\n", stderr)
    exit(1)
}

let width = image.width
let height = image.height
let bytesPerPixel = 4
let bytesPerRow = width * bytesPerPixel
var pixels = [UInt8](repeating: 0, count: height * bytesPerRow)

guard let context = CGContext(
    data: &pixels,
    width: width,
    height: height,
    bitsPerComponent: 8,
    bytesPerRow: bytesPerRow,
    space: CGColorSpaceCreateDeviceRGB(),
    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue
) else {
    fputs("unable to create bitmap context\n", stderr)
    exit(1)
}

context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

let xPercents = [25, 50, 75]
let yPercents = [25, 50, 75]
var colors = Set<String>()
var samples = 0
var blackPixels = 0
var whitePixels = 0

for xPercent in xPercents {
    for yPercent in yPercents {
        let x = min(width - 1, max(0, width * xPercent / 100))
        let y = min(height - 1, max(0, height * yPercent / 100))
        let offset = y * bytesPerRow + x * bytesPerPixel
        let r = pixels[offset]
        let g = pixels[offset + 1]
        let b = pixels[offset + 2]
        let color = String(format: "%02X%02X%02X", r, g, b)
        colors.insert(color)
        samples += 1
        if r == 0 && g == 0 && b == 0 {
            blackPixels += 1
        }
        if r == 255 && g == 255 && b == 255 {
            whitePixels += 1
        }
    }
}

print("samples=\(samples) unique_colors=\(colors.count) black_pixels=\(blackPixels) white_pixels=\(whitePixels)")
