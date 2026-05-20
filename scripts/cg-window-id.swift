import CoreGraphics
import Foundation

let args = CommandLine.arguments
guard args.count == 3 else {
    fputs("usage: cg-window-id.swift <owner-name> <title-fragment>\n", stderr)
    exit(2)
}

let ownerName = args[1]
let titleFragment = args[2]
let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let infoList = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []
var fallback: (number: Int, area: Int)?

for info in infoList {
    let owner = info[kCGWindowOwnerName as String] as? String ?? ""
    let title = info[kCGWindowName as String] as? String ?? ""
    guard owner == ownerName else {
        continue
    }
    if let number = info[kCGWindowNumber as String] as? Int,
       let bounds = info[kCGWindowBounds as String] as? [String: Any],
       let width = bounds["Width"] as? Int,
       let height = bounds["Height"] as? Int {
        let area = width * height
        if fallback == nil || area > fallback!.area {
            fallback = (number, area)
        }
    }
    guard title.contains(titleFragment) else {
        continue
    }
    if let number = info[kCGWindowNumber as String] as? Int {
        print(number)
        exit(0)
    }
}

if let fallback {
    print(fallback.number)
    exit(0)
}

fputs("window not found: \(ownerName) / \(titleFragment)\n", stderr)
exit(1)
