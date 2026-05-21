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

pids=""
cleanup() {
  for pid in $pids; do
    kill "$pid" 2>/dev/null || true
  done
}
trap cleanup EXIT

capture_launcher() {
  theme="$1"
  scenario="$2"
  output="$out_dir/launcher-$theme-$scenario.png"
  STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher -- --ui-preview "$theme" "$scenario" 7000 &
  pid="$!"
  pids="$pids $pid"
  STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh std-launcher "std-cli Launcher" "$output"
  wait "$pid" || true
  test -s "$output"
  echo "$output"
}

capture_studio() {
  theme="$1"
  scenario="$2"
  output="$out_dir/studio-$theme-$scenario.png"
  STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview "$theme" "$scenario" 7000 &
  pid="$!"
  pids="$pids $pid"
  STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh std-studio "std-cli Studio" "$output"
  wait "$pid" || true
  test -s "$output"
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
capture_studio light analysis
capture_studio dark analysis
capture_studio light plugins
capture_studio dark plugins
capture_studio light operations
capture_studio dark operations
capture_studio light settings
capture_studio dark settings
capture_studio light panes
capture_studio dark panes

echo "capture-ui-matrix PASS out_dir=$out_dir"
