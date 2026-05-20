import AppKit
import CoreGraphics
import Foundation
import UniformTypeIdentifiers

let args = CommandLine.arguments
guard args.count == 4 else {
    fputs("usage: cg-capture-window.swift <owner-name> <title-fragment> <output-png>\n", stderr)
    exit(2)
}

let ownerName = args[1]
let titleFragment = args[2]
let outputURL = URL(fileURLWithPath: args[3])
let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let infoList = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []
var fallback: (number: CGWindowID, area: Int)?

for info in infoList {
    let owner = info[kCGWindowOwnerName as String] as? String ?? ""
    let title = info[kCGWindowName as String] as? String ?? ""
    guard owner == ownerName else {
        continue
    }
    guard let rawNumber = info[kCGWindowNumber as String] as? Int,
          let bounds = info[kCGWindowBounds as String] as? [String: Any],
          let width = bounds["Width"] as? Int,
          let height = bounds["Height"] as? Int else {
        continue
    }
    let number = CGWindowID(rawNumber)
    let area = width * height
    if fallback == nil || area > fallback!.area {
        fallback = (number, area)
    }
    if title.contains(titleFragment) {
        capture(window: number, to: outputURL)
    }
}

if let fallback {
    capture(window: fallback.number, to: outputURL)
}

fputs("window not found: \(ownerName) / \(titleFragment)\n", stderr)
exit(1)

func capture(window: CGWindowID, to outputURL: URL) -> Never {
    guard let image = CGWindowListCreateImage(
        .null,
        .optionIncludingWindow,
        window,
        [.boundsIgnoreFraming, .bestResolution]
    ) else {
        fputs("failed to capture window: \(window)\n", stderr)
        exit(1)
    }
    let bitmap = NSBitmapImageRep(cgImage: image)
    guard let data = bitmap.representation(using: .png, properties: [:]) else {
        fputs("failed to encode png: \(outputURL.path)\n", stderr)
        exit(1)
    }
    do {
        try FileManager.default.createDirectory(
            at: outputURL.deletingLastPathComponent(),
            withIntermediateDirectories: true
        )
        try data.write(to: outputURL)
    } catch {
        fputs("failed to write png: \(error)\n", stderr)
        exit(1)
    }
    exit(0)
}
