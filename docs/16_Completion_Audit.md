# 16. Completion Audit - v1.0 完成审计

## 当前结论
v1.0 completion 未完成。当前 UI 完成状态全部作废，功能 smoke 和后端能力不能作为 UI 完成证据。完成状态不能从单元测试、smoke 路径存在或 UI 代码存在直接推出。每个门槛都必须有当前运行证据。

当前证据规则：

- 历史 target/ui-evidence 路径不能作为完成证据
- 历史 /tmp 截图不能作为完成证据
- 真实截图必须来自本轮 `STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix` 输出
- 真实截图 manifest 必须是 `artifacts/ui/manual-acceptance/manifest.txt`
- 真实截图 manifest 必须包含中心与边缘 pixel evidence
- 真实截图 doctor 必须拒绝 `single-color`、`dominant-black`、`dominant-white`、`edge-black`、`edge-white-carrier`
- 真实截图 acceptance rule 必须是 `explicit-opt-in+current-run-manifest+pid-process-title+png-files+center-edge-pixel-evidence+carrier-reject`
- 安装版 GUI 验证必须来自本轮显式 desktop opt-in 输出
- 后台 UI 验收必须来自本轮 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance` 输出
- 后台 UI manifest 必须是 `artifacts/ui/background-acceptance/manifest.txt`
- 后台 UI doctor 必须使用 `STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt std doctor` 校验
- 后台 UI acceptance rule 必须是 `isolated-harness-only+frontmost-preserved+doctor-validated`
- 默认测试不得触碰 Terminal、iTerm2、1Password、WeChat、weixin、wechat、微信、System Settings 或用户当前 frontmost app
- Launcher capture state required: light-collapsed
- Launcher capture state required: dark-collapsed
- Launcher capture state required: light-empty
- Launcher capture state required: dark-empty
- Launcher capture state required: light-results
- Launcher capture state required: dark-results
- Launcher capture state required: light-no-results
- Launcher capture state required: dark-no-results
- Launcher capture state required: light-searching
- Launcher capture state required: dark-searching
- Launcher capture state required: light-loading
- Launcher capture state required: dark-loading
- Launcher capture state required: light-executing
- Launcher capture state required: dark-executing
- Launcher capture state required: light-defer
- Launcher capture state required: dark-defer
- Launcher capture state required: light-error
- Launcher capture state required: dark-error
- Launcher capture state required: light-ime
- Launcher capture state required: dark-ime
- Launcher capture state required: light-action-panel
- Launcher capture state required: dark-action-panel
- Studio capture state required: light-dashboard
- Studio capture state required: dark-dashboard
- Studio capture state required: light-workflow
- Studio capture state required: dark-workflow
- Studio capture state required: light-workflow-error
- Studio capture state required: dark-workflow-error
- Studio capture state required: light-analysis
- Studio capture state required: dark-analysis
- Studio capture state required: light-plugins
- Studio capture state required: dark-plugins
- Studio capture state required: light-plugin-permission
- Studio capture state required: dark-plugin-permission
- Studio capture state required: light-operations
- Studio capture state required: dark-operations
- Studio capture state required: light-settings
- Studio capture state required: dark-settings
- Studio capture state required: light-panes
- Studio capture state required: dark-panes

## 已验证证据

### Launcher runtime

安装版 `std-launcher` 已验证真实 GUI 全局热键序列，但 Launcher UI 仍需要按 docs/18-21 完整视觉审计：

```text
STD_ALLOW_DESKTOP_AUTOMATION=1 /tmp/std-cli-install-current/bin/std-launcher --gui-hotkey-smoke Alt+Space 7000
launcher_gui_hotkey_smoke PASS
registered=true
input_sent=true
event_received=true
commands=Visible(true),Focus
visible_after_close=false
resident_after_close=true
second_input_sent=true
second_event_received=true
close_commands=Visible(false)
second_commands=Visible(true),Focus
error=none
```

该证据覆盖默认隐藏、真实全局快捷键、关闭只隐藏、隐藏后 hotkey runtime 仍注册、二次唤起。

当前截图证据：未完成。旧 `target/ui-evidence` 截图不得列入已验证证据；下一次验收必须来自本轮 `STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix` 的 manifest，并由 `std doctor` 校验 PNG 文件、字节数和 capture matrix 完整性。

### macOS app 多语言名称

安装版 `std` 已验证真实 `/Applications/WeChat.app` 多语言搜索：

```text
/tmp/std-cli-install-current/bin/std search 微信
Open App: WeChat    Launch macOS app at /Applications/WeChat.app

/tmp/std-cli-install-current/bin/std search weixin
Open App: WeChat    Launch macOS app at /Applications/WeChat.app

/tmp/std-cli-install-current/bin/std search wechat
Open App: WeChat    Launch macOS app at /Applications/WeChat.app
```

该证据覆盖 `微信`、`weixin`、`wechat`、macOS `Info.plist`、localized `InfoPlist.strings`、URL scheme、派生别名路径。

### Release 和 install

当前 release/install 证据已于 2026-05-22 08:03 UTC 用 `mise` 任务入口重跑，全部命令均在 `STD_TEST_MODE=1`、`STD_ALLOW_DESKTOP_AUTOMATION=0`、`STD_ALLOW_UI_PREVIEW=0`、`STD_ALLOW_BACKGROUND_UI_AUTOMATION=0` 下执行：

```text
mise run release-build
PASS

mise run release-package
PASS
dist_dir=dist/1.0.0-current
binaries=3
app_bundles=2
quality=PASS

mise run release-verify
PASS
version=1.0.0
binaries=3
app_bundles=2
docs=26
examples=9
quality=6
checksums=46
metadata=PASS
install_command=PASS

mise run install-run
PASS
prefix=.std-cli/install-check
binaries=3
app_bundles=2

mise run install-verify
PASS
prefix=.std-cli/install-check
binaries=3
app_bundles=2
storage=PASS
```

### Quality

当前 `mise run quality` 证据：

```text
cargo fmt --all --check
PASS

cargo clippy --workspace --all-targets -- -D warnings
PASS

DYLINT_RUSTFLAGS="-D warnings" cargo dylint --workspace --all -- --all-targets
PASS

cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml
cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib
PASS

cargo run -p std-egui --example a11y-audit
PASS

cargo test --workspace -- --test-threads=1
PASS

cargo deny check
PASS

cargo machete
PASS
```

`cargo deny check` 仍输出 duplicate warnings，但退出码为 0，当前门槛判定为 PASS。

### Studio runtime

安装版 `.std-cli/install-check/bin/std-studio` 已验证 headless 核心工作流，但该证据只覆盖内部 workspace pane 状态，不证明 UI 已完成：

```text
.std-cli/install-check/bin/std-studio --smoke
studio_smoke PASS
workspace_panes=10
focused_pane=11
pane_opened=true
pane_focus_switched=true
pane_closed=true
pane_focus_restored=true
pane_state_preserved=true
workflow_status=Completed
batch_status=NeedsExternalRunner
analysis_coverage_layers=overview:PASS,components:PASS,relations:PASS,history:PASS
memory_count=1
plugin_js_status=Completed
plugin_ts_status=Completed
operations_release_result=release verify evidence 7/7 present
operations_install_result=install verify evidence 5/5 present
history_count=1
```

该证据覆盖：

- Workflow 创建、编辑、模拟、运行
- Batch 中外部 runner 默认 defer
- 执行历史写入
- Memory 写入和搜索
- Plugin manifest 加载、检查和运行
- Index 分析和 coverage
- 10 个 Studio workspace pane 模型入口

当前 Studio 截图证据：未完成。旧 `/tmp` 截图不得列入已验证证据；下一次验收必须来自本轮 `STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix` 的 manifest，并覆盖 light / dark、Dashboard、Workflow、Analysis、Plugin、Operations、Settings、panes 等场景。

### Terminal CLI runtime

安装版 `std` 已在隔离数据目录验证脚本友好命令面：

```text
STDCLI_CONFIG=/tmp/std-cli-smoke-config.json
STDCLI_DATA_DIR=/tmp/std-cli-smoke-data
```

命令面覆盖：

```text
/tmp/std-cli-install-current/bin/std --help
Commands:
config install release doctor search preview trigger run batch workflow index analyze plan tool plugin app memory skill command clipboard files events
```

隔离配置和 doctor：

```text
std config list
data_dir=/tmp/std-cli-smoke-data

std doctor --json
status=PASS
storage=PASS
planner=PASS
workflow_dry_run=PASS
quality=PASS
quality_tools=rustfmt,clippy,dylint,cargo-deny,cargo-machete
source_file_limit=500
config_file_limit=300
plugins=2
```

默认外部行为 defer：

```text
std trigger terminal
status=NeedsExternalRunner
deferred=true
reason=external runner action requires explicit user trigger
```

Workflow、batch、memory、analyze、search 和 trace 已用安装版验证：

```text
std workflow new cli-smoke --description "CLI smoke workflow"
workflow created

std workflow step add /tmp/std-cli-smoke-data/workflows/cli-smoke/workflow.md "Collect smoke context" --json {"target":"doctor"}
step_type=Action

std workflow check /tmp/std-cli-smoke-data/workflows/cli-smoke/workflow.md
status=Completed

std run /tmp/std-cli-smoke-data/workflows/cli-smoke/workflow.md
status=Completed

std batch /tmp/std-cli-smoke-batch/batch.json
status=NeedsExternalRunner

std memory remember "CLI smoke memory" "Installed CLI memory write" --scope project --tags cli,smoke
PASS

std memory recall smoke
title=CLI smoke memory

std analyze /tmp/std-cli-smoke-project
kind=Project
components=1
relations=5

std workflow trace --limit 5
workflow_name=cli-smoke
status=Completed
audit_events=WorkflowStarted,WorkflowStepCompleted,WorkflowCompleted
```

### Plugin runtime

安装版 `.std-cli/install-check/bin/std` 已验证 JavaScript 和 TypeScript 插件均通过 `deno_core` 执行：

```text
.std-cli/install-check/bin/std plugin check examples/plugins/hello-js
status=PASS
plugin_name=hello-js

.std-cli/install-check/bin/std plugin check examples/plugins/typed-ts
status=PASS
plugin_name=typed-ts

.std-cli/install-check/bin/std plugin run hello-js
status=Completed
runtime=deno_core
script=/Users/chen/.std-cli/plugins/hello-js/main.js
stdout={"plugin":"hello-js","greeting":"hello from std-cli","input":{}}

.std-cli/install-check/bin/std plugin run plugin-typed-ts
status=Completed
runtime=deno_core
script=/Users/chen/.std-cli/plugins/typed-ts/main.ts
stdout={"plugin":"typed-ts","greeting":"hello std-cli"}
```

### Index runtime

安装版 `.std-cli/install-check/bin/std` 已验证四层 index coverage：

```text
.std-cli/install-check/bin/std index coverage
total=5
complete=5
incomplete=0
entity_overview=true
component_digest=true
symbol_relation_index=true
historical_context=true
```

## 最终审计项

以下项必须在 `17_Final_Completion_Matrix.md` 中逐项用当前运行证据重新证明：

- UI docs 18-24、Launcher、Studio、Core、Terminal、Plugin、Index、Workflow、Release、Install、Quality 的 requirement-by-requirement completion audit

## 审计规则
未验证即未完成。默认测试和 smoke 不得唤起 Terminal、App、文件或外部 runner。默认测试不得触碰 Terminal、iTerm2、1Password、WeChat、weixin、wechat、微信、System Settings 或用户当前 frontmost app。外部行为默认 `NeedsExternalRunner`。只有显式 opt-in 才执行真实 GUI hotkey 或外部 runner 行为。完成声明必须引用当前运行证据。
