#!/usr/bin/env bash
set -euo pipefail

if [ "${STD_ALLOW_UI_PREVIEW:-}" != "1" ]; then
  echo "capture-ui-matrix SKIP reason=STD_ALLOW_UI_PREVIEW=1 required" >&2
  exit 2
fi

if [ "${STD_TEST_MODE:-}" = "1" ]; then
  echo "capture-ui-matrix SKIP reason=STD_TEST_MODE blocks UI preview" >&2
  exit 2
fi

if [ "$(uname -s)" != "Darwin" ]; then
  echo "capture-ui-matrix requires macOS" >&2
  exit 2
fi

out_dir="${1:-artifacts/ui/$(date +%Y%m%d-%H%M%S)}"
mkdir -p "$out_dir"
manifest="$out_dir/manifest.txt"
: >"$manifest"

record_manifest_header() {
  {
    echo "capture-ui-matrix manifest"
    echo "created_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "out_dir=$out_dir"
    echo "opt_in=STD_ALLOW_UI_PREVIEW=1"
    echo "test_mode=STD_TEST_MODE must not be 1"
    echo "capture_rule=pid+process-name+window-title"
    echo "completion_rule=current-run-png-only"
  } >>"$manifest"
}

record_capture() {
  surface="$1"
  theme="$2"
  scenario="$3"
  output="$4"
  bytes=$(wc -c <"$output" | tr -d ' ')
  width=$(/usr/bin/sips -g pixelWidth "$output" 2>/dev/null | /usr/bin/awk '/pixelWidth/ {print $2}')
  height=$(/usr/bin/sips -g pixelHeight "$output" 2>/dev/null | /usr/bin/awk '/pixelHeight/ {print $2}')
  pixel_evidence=$(/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift \
    scripts/cg-sample-pixels.swift "$output")
  echo "$surface theme=$theme scenario=$scenario path=$output bytes=$bytes width=$width height=$height $pixel_evidence" >>"$manifest"
}

pids=""
cleanup() {
  for pid in $pids; do
    kill "$pid" 2>/dev/null || true
  done
}
trap cleanup EXIT
record_manifest_header

capture_launcher() {
  theme="$1"
  scenario="$2"
  output="$out_dir/launcher-$theme-$scenario.png"
  STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher -- --ui-preview "$theme" "$scenario" 7000 &
  pid="$!"
  pids="$pids $pid"
  STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh "$pid" std-launcher "std-cli Launcher" "$output"
  wait "$pid" || true
  test -s "$output"
  record_capture launcher "$theme" "$scenario" "$output"
  echo "$output"
}

capture_studio() {
  theme="$1"
  scenario="$2"
  output="$out_dir/studio-$theme-$scenario.png"
  STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview "$theme" "$scenario" 7000 &
  pid="$!"
  pids="$pids $pid"
  STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh "$pid" std-studio "std-cli Studio" "$output"
  wait "$pid" || true
  test -s "$output"
  record_capture studio "$theme" "$scenario" "$output"
  echo "$output"
}

capture_launcher light collapsed
capture_launcher dark collapsed
capture_launcher light empty
capture_launcher dark empty
capture_launcher light results
capture_launcher dark results
capture_launcher light no-results
capture_launcher dark no-results
capture_launcher light searching
capture_launcher dark searching
capture_launcher light loading
capture_launcher dark loading
capture_launcher light executing
capture_launcher dark executing
capture_launcher light defer
capture_launcher dark defer
capture_launcher light error
capture_launcher dark error
capture_launcher light action-panel
capture_launcher dark action-panel

capture_studio light dashboard
capture_studio dark dashboard
capture_studio light workflow
capture_studio dark workflow
capture_studio light workflow-error
capture_studio dark workflow-error
capture_studio light analysis
capture_studio dark analysis
capture_studio light plugins
capture_studio dark plugins
capture_studio light plugin-permission
capture_studio dark plugin-permission
capture_studio light operations
capture_studio dark operations
capture_studio light settings
capture_studio dark settings
capture_studio light panes
capture_studio dark panes

echo "capture-ui-matrix PASS out_dir=$out_dir manifest=$manifest"
