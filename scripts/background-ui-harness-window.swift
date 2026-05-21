import AppKit
import CoreGraphics
import Foundation

let requiredBundleId = "dev.std-cli.background-ui-harness"
let requiredWindowTitle = "std-cli Background UI Harness"
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
    print("smoke_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 std ui background-smoke --harness-pid \(ownerPid) --window-id \(number) --bundle-id \(requiredBundleId) --window-title \"\(requiredWindowTitle)\"")
    exit(0)
}

fputs("background_ui_harness_window FAIL reason=harness window not found\n", stderr)
exit(1)
