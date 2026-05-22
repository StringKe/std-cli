#!/bin/sh
set -eu

if [ "${STD_TEST_MODE:-}" = "1" ]; then
  echo "background_ui_harness SKIP reason=STD_TEST_MODE blocks background UI automation" >&2
  exit 1
fi

if [ "${STD_ALLOW_BACKGROUND_UI_AUTOMATION:-}" != "1" ]; then
  echo "background_ui_harness SKIP reason=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required" >&2
  exit 1
fi

launcher_bin="target/debug/std-launcher"
timeout_ms="30000"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --launcher-bin)
      launcher_bin="$2"
      shift 2
      ;;
    --timeout-ms)
      timeout_ms="$2"
      shift 2
      ;;
    *)
      echo "usage: background-ui-harness.sh [--launcher-bin <path>] [--timeout-ms <ms>]" >&2
      exit 2
      ;;
  esac
done

if [ ! -x "$launcher_bin" ]; then
  echo "background_ui_harness FAIL reason=launcher binary not executable: $launcher_bin" >&2
  exit 1
fi

root_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
app_dir="$root_dir/.std-cli/background-ui-harness/StdCliBackgroundUiHarness.app"
macos_dir="$app_dir/Contents/MacOS"
plist="$app_dir/Contents/Info.plist"
executable="$macos_dir/std-launcher"
launcher_abs="$(CDPATH= cd -- "$(dirname -- "$launcher_bin")" && pwd)/$(basename -- "$launcher_bin")"
harness_token="run-$$"

rm -rf "$app_dir"
mkdir -p "$macos_dir"
cat >"$executable" <<WRAPPER
#!/bin/sh
export STD_ALLOW_BACKGROUND_UI_AUTOMATION=1
export STD_BACKGROUND_UI_HARNESS_TOKEN="$harness_token"
unset STD_TEST_MODE
exec "$launcher_abs" --background-ui-harness "$timeout_ms"
WRAPPER
chmod +x "$executable"
cat >"$plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>std-launcher</string>
  <key>CFBundleIdentifier</key>
  <string>dev.std-cli.background-ui-harness</string>
  <key>CFBundleName</key>
  <string>std-cli Background UI Harness</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
</dict>
</plist>
PLIST

/usr/bin/open -n -g "$app_dir"

index=0
while [ "$index" -lt 50 ]; do
  if /usr/bin/swift "$root_dir/scripts/background-ui-harness-window.swift" --harness-token "$harness_token"; then
    exit 0
  fi
  index=$((index + 1))
  sleep 0.1
done

echo "background_ui_harness FAIL reason=harness window not found after launch" >&2
exit 1
