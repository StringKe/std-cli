#!/bin/sh
set -eu

if [ "${STD_TEST_MODE:-}" = "1" ]; then
  echo "background_ui_acceptance SKIP reason=STD_TEST_MODE blocks background UI automation" >&2
  exit 1
fi

if [ "${STD_ALLOW_BACKGROUND_UI_AUTOMATION:-}" != "1" ]; then
  echo "background_ui_acceptance SKIP reason=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required" >&2
  exit 1
fi

output=$(STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-harness.sh "$@")
printf '%s\n' "$output"

harness_pid=""
window_id=""
bundle_id=""
window_title=""
harness_token=""
smoke_command=""

while IFS= read -r line; do
  case "$line" in
    harness_pid=*) harness_pid=${line#harness_pid=} ;;
    window_id=*) window_id=${line#window_id=} ;;
    bundle_id=*) bundle_id=${line#bundle_id=} ;;
    window_title=*) window_title=${line#window_title=} ;;
    harness_token=*) harness_token=${line#harness_token=} ;;
    smoke_command=*) smoke_command=${line#smoke_command=} ;;
  esac
done <<EOF
$output
EOF

if [ -z "$harness_pid" ] || [ -z "$window_id" ]; then
  echo "background_ui_acceptance FAIL reason=harness identity missing" >&2
  exit 1
fi

if [ "$bundle_id" != "dev.std-cli.background-ui-harness" ]; then
  echo "background_ui_acceptance FAIL reason=bundle_id outside whitelist" >&2
  exit 1
fi

if [ -z "$harness_token" ]; then
  echo "background_ui_acceptance FAIL reason=harness_token missing" >&2
  exit 1
fi

if [ "$window_title" != "std-cli Background UI Harness $harness_token" ]; then
  echo "background_ui_acceptance FAIL reason=window_title outside whitelist" >&2
  exit 1
fi

expected_smoke_command="STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid $harness_pid --window-id $window_id --bundle-id $bundle_id --window-title \"$window_title\" --harness-token $harness_token"
if [ "$smoke_command" != "$expected_smoke_command" ]; then
  echo "background_ui_acceptance FAIL reason=smoke_command identity mismatch" >&2
  exit 1
fi

STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke \
  --harness-pid "$harness_pid" \
  --window-id "$window_id" \
  --bundle-id "$bundle_id" \
  --window-title "$window_title" \
  --harness-token "$harness_token"
