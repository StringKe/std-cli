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
let edgePoints = [
    (1, 1),
    (width / 2, 1),
    (width - 2, 1),
    (1, height / 2),
    (width - 2, height / 2),
    (1, height - 2),
    (width / 2, height - 2),
    (width - 2, height - 2),
]
var colors = Set<String>()
var samples = 0
var opaqueSamples = 0
var blackPixels = 0
var whitePixels = 0
var transparentPixels = 0
var edgeSamples = 0
var edgeTransparentPixels = 0
var edgeBlackPixels = 0
var edgeWhitePixels = 0

for xPercent in xPercents {
    for yPercent in yPercents {
        let x = min(width - 1, max(0, width * xPercent / 100))
        let y = min(height - 1, max(0, height * yPercent / 100))
        let offset = y * bytesPerRow + x * bytesPerPixel
        let r = pixels[offset]
        let g = pixels[offset + 1]
        let b = pixels[offset + 2]
        let a = pixels[offset + 3]
        samples += 1
        if a == 0 {
            transparentPixels += 1
            continue
        }
        opaqueSamples += 1
        let color = String(format: "%02X%02X%02X", r, g, b)
        colors.insert(color)
        if r == 0 && g == 0 && b == 0 {
            blackPixels += 1
        }
        if r == 255 && g == 255 && b == 255 {
            whitePixels += 1
        }
    }
}

for point in edgePoints {
    let x = min(width - 1, max(0, point.0))
    let y = min(height - 1, max(0, point.1))
    let offset = y * bytesPerRow + x * bytesPerPixel
    let r = pixels[offset]
    let g = pixels[offset + 1]
    let b = pixels[offset + 2]
    let a = pixels[offset + 3]
    edgeSamples += 1
    if a == 0 {
        edgeTransparentPixels += 1
    }
    if a != 0 && r == 0 && g == 0 && b == 0 {
        edgeBlackPixels += 1
    }
    if a != 0 && r == 255 && g == 255 && b == 255 {
        edgeWhitePixels += 1
    }
}

print("samples=\(samples) opaque_samples=\(opaqueSamples) unique_colors=\(colors.count) black_pixels=\(blackPixels) white_pixels=\(whitePixels) transparent_pixels=\(transparentPixels) edge_samples=\(edgeSamples) edge_transparent_pixels=\(edgeTransparentPixels) edge_black_pixels=\(edgeBlackPixels) edge_white_pixels=\(edgeWhitePixels)")
