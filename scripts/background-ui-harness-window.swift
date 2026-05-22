import AppKit
import CoreGraphics
import Foundation

let requiredBundleId = "dev.std-cli.background-ui-harness"
let requiredWindowTitlePrefix = "std-cli Background UI Harness"
let harnessToken = parseHarnessToken()
let requiredWindowTitle = "\(requiredWindowTitlePrefix) \(harnessToken)"
let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
let list = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []

for item in list {
    let ownerPid = item[kCGWindowOwnerPID as String] as? pid_t ?? 0
    let number = item[kCGWindowNumber as String] as? Int ?? 0
    let title = item[kCGWindowName as String] as? String ?? ""
    guard ownerPid > 0,
          number > 0,
          title == requiredWindowTitle,
          let app = NSRunningApplication(processIdentifier: ownerPid),
          app.bundleIdentifier == requiredBundleId else {
        continue
    }
    print("harness_pid=\(ownerPid)")
    print("window_id=\(number)")
    print("bundle_id=\(requiredBundleId)")
    print("window_title=\(requiredWindowTitle)")
    print("harness_token=\(harnessToken)")
    print("smoke_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid \(ownerPid) --window-id \(number) --bundle-id \(requiredBundleId) --window-title \"\(requiredWindowTitle)\" --harness-token \(harnessToken)")
    exit(0)
}

fputs("background_ui_harness_window FAIL reason=harness window not found\n", stderr)
exit(1)

func parseHarnessToken() -> String {
    let args = CommandLine.arguments.dropFirst()
    var index = args.startIndex
    while index < args.endIndex {
        let key = args[index]
        let next = args.index(after: index)
        if key == "--harness-token", next < args.endIndex {
            return args[next]
        }
        index = args.index(after: index)
    }
    fputs("background_ui_harness_window FAIL reason=harness token required\n", stderr)
    exit(1)
}
