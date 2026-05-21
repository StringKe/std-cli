import ApplicationServices
import AppKit
import CoreGraphics
import Foundation

struct Config {
    let harnessPid: pid_t
    let windowId: Int
    let bundleId: String
    let windowTitle: String
}

let requiredBundleId = "dev.std-cli.background-ui-harness"
let requiredWindowTitle = "std-cli Background UI Harness"

guard ProcessInfo.processInfo.environment["STD_TEST_MODE"] != "1" else {
    fail("STD_TEST_MODE blocks background UI automation")
}
guard ProcessInfo.processInfo.environment["STD_ALLOW_BACKGROUND_UI_AUTOMATION"] == "1" else {
    fail("STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required")
}

let config = parseConfig()
guard config.bundleId == requiredBundleId else {
    fail("bundle_id outside whitelist")
}
guard config.windowTitle == requiredWindowTitle else {
    fail("window_title outside whitelist")
}
guard let app = NSRunningApplication(processIdentifier: config.harnessPid),
      app.bundleIdentifier == requiredBundleId else {
    fail("pid bundle_id outside whitelist")
}
guard let window = findWindow(config) else {
    fail("harness window not found")
}
guard window.ownerPid == config.harnessPid else {
    fail("window pid mismatch")
}

let previousPid = NSWorkspace.shared.frontmostApplication?.processIdentifier ?? 0
guard previousPid != config.harnessPid else {
    fail("harness is frontmost; refusing to target active user window")
}
var session = BackgroundActivationSession(previousPid: previousPid, targetPid: config.harnessPid)
session.start()
sendAppKitActivation(to: config.harnessPid, windowId: config.windowId, subtype: 1)
postCenterPrimer(to: config.harnessPid, windowId: config.windowId, window: window)
postKeySmoke(to: config.harnessPid, windowId: config.windowId)
sendAppKitActivation(to: config.harnessPid, windowId: config.windowId, subtype: 2)
session.stop()

print("background_driver PASS target_pid=\(config.harnessPid) window_id=\(config.windowId)")

struct WindowInfo {
    let ownerPid: pid_t
    let bounds: CGRect
    let title: String
}

final class BackgroundActivationSession {
    private let previousPid: pid_t
    private let targetPid: pid_t
    private var previousTap: CFMachPort?
    private var targetTap: CFMachPort?

    init(previousPid: pid_t, targetPid: pid_t) {
        self.previousPid = previousPid
        self.targetPid = targetPid
    }

    func start() {
        previousTap = createTap(pid: previousPid, dropFocusMessages: true)
        targetTap = createTap(pid: targetPid, dropFocusMessages: false)
    }

    func stop() {
        if let previousTap {
            CFMachPortInvalidate(previousTap)
        }
        if let targetTap {
            CFMachPortInvalidate(targetTap)
        }
    }

    private func createTap(pid: pid_t, dropFocusMessages: Bool) -> CFMachPort? {
        guard pid > 0 else {
            return nil
        }
        let info = UnsafeMutableRawPointer(Unmanaged.passRetained(TapPolicy(drop: dropFocusMessages)).toOpaque())
        guard let tap = CGEvent.tapCreateForPid(
            pid: pid,
            place: .headInsertEventTap,
            options: .defaultTap,
            eventsOfInterest: focusEventMask(),
            callback: eventTapCallback,
            userInfo: info
        ) else {
            return nil
        }
        let source = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0)
        CFRunLoopAddSource(CFRunLoopGetCurrent(), source, .commonModes)
        CGEvent.tapEnable(tap: tap, enable: true)
        return tap
    }
}

final class TapPolicy {
    let drop: Bool

    init(drop: Bool) {
        self.drop = drop
    }
}

let eventTapCallback: CGEventTapCallBack = { _, type, event, userInfo in
    guard let userInfo else {
        return Unmanaged.passUnretained(event)
    }
    let policy = Unmanaged<TapPolicy>.fromOpaque(userInfo).takeUnretainedValue()
    if policy.drop && isFocusMessage(type) {
        return nil
    }
    return Unmanaged.passUnretained(event)
}

func focusEventMask() -> CGEventMask {
    [CGEventType(rawValue: 13), CGEventType(rawValue: 19), CGEventType(rawValue: 20)]
        .compactMap { $0 }
        .reduce(CGEventMask(0)) { mask, type in
            mask | (1 << type.rawValue)
        }
}

func isFocusMessage(_ type: CGEventType) -> Bool {
    let raw = type.rawValue
    return raw == 13 || raw == 19 || raw == 20
}

func sendAppKitActivation(to pid: pid_t, windowId: Int, subtype: Int16) {
    let nsEvent = NSEvent.otherEvent(
        with: .appKitDefined,
        location: .zero,
        modifierFlags: [],
        timestamp: ProcessInfo.processInfo.systemUptime,
        windowNumber: windowId,
        context: nil,
        subtype: subtype,
        data1: 0,
        data2: 0
    )
    guard let event = nsEvent?.cgEvent else {
        return
    }
    event.setIntegerValueField(CGEventField.eventTargetUnixProcessID, value: Int64(pid))
    setWindowAddressing(event, windowId: windowId)
    event.postToPid(pid)
}

func postCenterPrimer(to pid: pid_t, windowId: Int, window: WindowInfo) {
    let point = CGPoint(x: window.bounds.midX, y: window.bounds.midY)
    postMouse(type: .leftMouseDown, to: pid, windowId: windowId, point: point, pressure: 1)
    Thread.sleep(forTimeInterval: 0.03)
    postMouse(type: .leftMouseUp, to: pid, windowId: windowId, point: point, pressure: 0)
}

func postMouse(type: CGEventType, to pid: pid_t, windowId: Int, point: CGPoint, pressure: Int64) {
    guard let event = CGEvent(mouseEventSource: nil, mouseType: type, mouseCursorPosition: point, mouseButton: .left) else {
        return
    }
    event.setIntegerValueField(.eventTargetUnixProcessID, value: Int64(pid))
    event.setIntegerValueField(.mouseEventClickState, value: 1)
    event.setIntegerValueField(.mouseEventPressure, value: pressure)
    setWindowAddressing(event, windowId: windowId)
    event.postToPid(pid)
}

func postKeySmoke(to pid: pid_t, windowId: Int) {
    guard let down = CGEvent(keyboardEventSource: nil, virtualKey: 53, keyDown: true),
          let up = CGEvent(keyboardEventSource: nil, virtualKey: 53, keyDown: false) else {
        return
    }
    for event in [down, up] {
        event.setIntegerValueField(.eventTargetUnixProcessID, value: Int64(pid))
        setWindowAddressing(event, windowId: windowId)
        event.postToPid(pid)
    }
}

func setWindowAddressing(_ event: CGEvent, windowId: Int) {
    event.setIntegerValueField(.mouseEventWindowUnderMousePointer, value: Int64(windowId))
    event.setIntegerValueField(.mouseEventWindowUnderMousePointerThatCanHandleThisEvent, value: Int64(windowId))
    event.setIntegerValueField(CGEventField(rawValue: 51)!, value: Int64(windowId))
    event.setIntegerValueField(CGEventField(rawValue: 58)!, value: 1)
}

func findWindow(_ config: Config) -> WindowInfo? {
    let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
    let list = CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? []
    for item in list {
        let ownerPid = item[kCGWindowOwnerPID as String] as? pid_t ?? 0
        let number = item[kCGWindowNumber as String] as? Int ?? 0
        let title = item[kCGWindowName as String] as? String ?? ""
        guard ownerPid == config.harnessPid,
              number == config.windowId,
              title == config.windowTitle else {
            continue
        }
        guard let boundsDict = item[kCGWindowBounds as String] as? [String: Any],
              let bounds = CGRect(dictionaryRepresentation: boundsDict as CFDictionary) else {
            return nil
        }
        return WindowInfo(ownerPid: ownerPid, bounds: bounds, title: title)
    }
    return nil
}

func parseConfig() -> Config {
    let args = CommandLine.arguments.dropFirst()
    var values: [String: String] = [:]
    var index = args.startIndex
    while index < args.endIndex {
        let key = args[index]
        let next = args.index(after: index)
        guard next < args.endIndex else {
            fail("missing value for \(key)")
        }
        values[key] = args[next]
        index = args.index(after: next)
    }
    guard let pidText = values["--harness-pid"],
          let pid = pid_t(pidText),
          let windowText = values["--window-id"],
          let windowId = Int(windowText),
          let bundleId = values["--bundle-id"],
          let windowTitle = values["--window-title"] else {
        fail("usage: background-ui-smoke.swift --harness-pid <pid> --window-id <id> --bundle-id <bundle> --window-title <title>")
    }
    return Config(harnessPid: pid, windowId: windowId, bundleId: bundleId, windowTitle: windowTitle)
}

func fail(_ message: String) -> Never {
    fputs("background_driver FAIL reason=\(message)\n", stderr)
    exit(1)
}
