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

manifest="${STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST:-artifacts/ui/background-acceptance/manifest.txt}"
launcher_bin=""
timeout_ms=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --manifest)
      manifest="$2"
      shift 2
      ;;
    --launcher-bin)
      launcher_bin="$2"
      shift 2
      ;;
    --timeout-ms)
      timeout_ms="$2"
      shift 2
      ;;
    *)
      echo "usage: background-ui-acceptance.sh [--manifest <path>] [--launcher-bin <path>] [--timeout-ms <ms>]" >&2
      exit 2
      ;;
  esac
done

manifest_dir=$(dirname -- "$manifest")
mkdir -p "$manifest_dir"
: >"$manifest"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
harness_pid=""
cleanup_recorded="false"

cleanup_harness() {
  if [ "$cleanup_recorded" = "true" ]; then
    return
  fi
  cleanup_recorded="true"
  if [ -n "$harness_pid" ]; then
    kill "$harness_pid" 2>/dev/null || true
    {
      echo "cleanup_attempted=true"
      echo "cleanup_target_pid=$harness_pid"
      echo "cleanup_signal=TERM"
      echo "cleanup_scope=validated_background_ui_harness_pid_only"
    } >>"$manifest"
  else
    {
      echo "cleanup_attempted=false"
      echo "cleanup_target_pid=MISSING"
      echo "cleanup_signal=NONE"
      echo "cleanup_scope=no_validated_harness_pid"
    } >>"$manifest"
  fi
}

trap cleanup_harness EXIT

record_manifest_header() {
  {
    echo "background-ui-acceptance manifest"
    echo "created_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "run_id=$run_id"
    echo "target=isolated_background_ui_harness_only"
    echo "identity_rule=pid+window-id+bundle-id+window-title+harness-token"
    echo "completion_rule=background-ui-smoke-PASS-and-frontmost-preserved"
    echo "default_gate=manual-opt-in-only"
    echo "forbidden_targets=frontmost_app,Terminal,1Password,WeChat,weixin,wechat,微信,System_Settings"
    echo "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click"
    echo "fallback=never_frontmost_desktop_click"
    echo "frontmost_policy=previous_app_never_targeted"
    echo "frontmost_user_app_policy=identify_and_preserve_current_frontmost_app"
  } >>"$manifest"
}

record_manifest_header

set --
if [ -n "$launcher_bin" ]; then
  set -- "$@" --launcher-bin "$launcher_bin"
fi
if [ -n "$timeout_ms" ]; then
  set -- "$@" --timeout-ms "$timeout_ms"
fi

output=$(STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-harness.sh "$@")
printf '%s\n' "$output"

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

{
  echo "harness_pid=$harness_pid"
  echo "window_id=$window_id"
  echo "bundle_id=$bundle_id"
  echo "window_title=$window_title"
  echo "harness_token=$harness_token"
  echo "harness_run_id=$run_id"
  echo "smoke_command=$smoke_command"
} >>"$manifest"

smoke_output=$(STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke \
  --harness-pid "$harness_pid" \
  --window-id "$window_id" \
  --bundle-id "$bundle_id" \
  --window-title "$window_title" \
  --harness-token "$harness_token")
printf '%s\n' "$smoke_output"

smoke_status="FAIL"
if printf '%s\n' "$smoke_output" | grep -q '^background_ui_smoke PASS$'; then
  smoke_status="PASS"
fi
driver_stdout=""
driver_frontmost_preserved="false"
while IFS= read -r line; do
  case "$line" in
    runner_stdout=*) driver_stdout=${line#runner_stdout=} ;;
  esac
done <<EOF
$smoke_output
EOF
if printf '%s\n' "$driver_stdout" | grep -q 'background_driver PASS' &&
  printf '%s\n' "$driver_stdout" | grep -q "target_pid=$harness_pid" &&
  printf '%s\n' "$driver_stdout" | grep -q "window_id=$window_id" &&
  printf '%s\n' "$driver_stdout" | grep -q 'event_route=postToPid_target_pid_only' &&
  printf '%s\n' "$driver_stdout" | grep -q 'frontmost_preserved=true' &&
  printf '%s\n' "$driver_stdout" | grep -q 'frontmost_before=' &&
  printf '%s\n' "$driver_stdout" | grep -q 'frontmost_after='; then
  driver_frontmost_preserved="true"
fi
{
  echo "smoke_status=$smoke_status"
  echo "smoke_run_id=$run_id"
  echo "driver_stdout=$driver_stdout"
  echo "driver_identity=target-pid-window-id-and-frontmost-pid"
  echo "frontmost_preservation=required"
  echo "frontmost_preserved=$driver_frontmost_preserved"
  echo "frontmost_before_equals_after=required"
  echo "frontmost_evidence_source=background_driver_stdout"
  echo "target_not_frontmost=required"
  echo "previous_app_policy=event_tap_only_no_input_delivery"
  echo "frontmost_user_app_policy=identify_and_preserve_current_frontmost_app"
  echo "real_app_policy=deny_user_apps_by_bundle_pid_window_title_mismatch"
  echo "harness_origin=spawned_by_scripts_background_ui_harness_only"
  echo "manifest=$manifest"
} >>"$manifest"

if [ "$smoke_status" != "PASS" ]; then
  echo "background_ui_acceptance FAIL reason=background smoke did not PASS manifest=$manifest" >&2
  exit 1
fi
if [ "$driver_frontmost_preserved" != "true" ]; then
  echo "background_ui_acceptance FAIL reason=frontmost preservation missing from driver stdout manifest=$manifest" >&2
  exit 1
fi

echo "background_ui_acceptance PASS manifest=$manifest"
