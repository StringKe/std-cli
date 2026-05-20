#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: scripts/capture-window.sh <process-name> <window-title-fragment> <output-png>" >&2
  exit 2
fi

process_name="$1"
title_fragment="$2"
output_png="$3"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "capture-window requires macOS" >&2
  exit 2
fi

attempts="${STD_CAPTURE_ATTEMPTS:-30}"
attempt=1
while [ "$attempt" -le "$attempts" ]; do
  if /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/swift \
    scripts/cg-capture-window.swift "$process_name" "$title_fragment" "$output_png" 2>/dev/null; then
    test -s "$output_png"
    exit 0
  fi
  /bin/sleep 0.5
  attempt=$((attempt + 1))
done

echo "window not found: $process_name / $title_fragment" >&2
exit 1
