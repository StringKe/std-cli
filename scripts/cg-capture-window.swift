import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

let args = CommandLine.arguments
guard args.count == 5 else {
    fputs("usage: cg-capture-window.swift <owner-pid> <owner-name> <title-fragment> <output-png>\n", stderr)
    exit(2)
}

guard let ownerPid = Int(args[1]) else {
    fputs("invalid owner pid: \(args[1])\n", stderr)
    exit(2)
}
let ownerName = args[2]
let titleFragment = args[3]
let outputURL = URL(fileURLWithPath: args[4])
let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let infoList = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []

for info in infoList {
    let owner = info[kCGWindowOwnerName as String] as? String ?? ""
    let pid = info[kCGWindowOwnerPID as String] as? Int ?? 0
    let title = info[kCGWindowName as String] as? String ?? ""
    guard pid == ownerPid else {
        continue
    }
    guard owner == ownerName else {
        continue
    }
    guard let rawNumber = info[kCGWindowNumber as String] as? Int,
          let bounds = info[kCGWindowBounds as String] as? [String: Any] else {
        continue
    }
    if title.contains(titleFragment) {
        capture(windowId: CGWindowID(rawNumber), bounds: bounds, to: outputURL)
    }
}

fputs("window not found: \(ownerPid) / \(ownerName) / \(titleFragment)\n", stderr)
exit(1)

func capture(windowId: CGWindowID, bounds: [String: Any], to outputURL: URL) -> Never {
    guard let width = bounds["Width"] as? Int,
          let height = bounds["Height"] as? Int else {
        fputs("invalid window bounds\n", stderr)
        exit(1)
    }
    guard width > 0 && height > 0 else {
        fputs("invalid window size\n", stderr)
        exit(1)
    }
    do {
        try FileManager.default.createDirectory(
            at: outputURL.deletingLastPathComponent(),
            withIntermediateDirectories: true
        )
    } catch {
        fputs("failed to create output directory: \(error)\n", stderr)
        exit(1)
    }

    guard let image = CGWindowListCreateImage(
        .null,
        [.optionIncludingWindow],
        windowId,
        [.boundsIgnoreFraming, .bestResolution]
    ) else {
        fputs("failed to capture window image\n", stderr)
        exit(1)
    }
    guard let destination = CGImageDestinationCreateWithURL(
        outputURL as CFURL,
        UTType.png.identifier as CFString,
        1,
        nil
    ) else {
        fputs("failed to create png destination\n", stderr)
        exit(1)
    }
    CGImageDestinationAddImage(destination, image, nil)
    if !CGImageDestinationFinalize(destination) {
        fputs("failed to write png\n", stderr)
        exit(1)
    }
    exit(0)
}
