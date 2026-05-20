import CoreGraphics
import Foundation

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
        capture(bounds: bounds, to: outputURL)
    }
}

if let fallback {
    let bounds = infoList.compactMap { info -> [String: Any]? in
        guard let rawNumber = info[kCGWindowNumber as String] as? Int,
              CGWindowID(rawNumber) == fallback.number else {
            return nil
        }
        return info[kCGWindowBounds as String] as? [String: Any]
    }.first
    if let bounds {
        capture(bounds: bounds, to: outputURL)
    }
}

fputs("window not found: \(ownerName) / \(titleFragment)\n", stderr)
exit(1)

func capture(bounds: [String: Any], to outputURL: URL) -> Never {
    guard let x = bounds["X"] as? Int,
          let y = bounds["Y"] as? Int,
          let width = bounds["Width"] as? Int,
          let height = bounds["Height"] as? Int else {
        fputs("invalid window bounds\n", stderr)
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

    let task = Process()
    task.executableURL = URL(fileURLWithPath: "/usr/sbin/screencapture")
    task.arguments = ["-x", "-R\(x),\(y),\(width),\(height)", outputURL.path]
    do {
        try task.run()
        task.waitUntilExit()
    } catch {
        fputs("failed to run screencapture: \(error)\n", stderr)
        exit(1)
    }
    exit(task.terminationStatus)
}
