# 17. Final Completion Matrix - v1.0 逐项完成矩阵

## 判定

当前矩阵用于最终完成判断。UI 完成状态全部作废，所有项目必须用当前运行证据重新证明，不能用代码存在、测试存在或历史印象替代。

## Core

状态：未完成

证据：

- `std doctor --json` 返回 `storage=PASS`、`planner=PASS`、`workflow_dry_run=PASS`
- `std workflow trace --limit 5` 返回 `WorkflowStarted`、`WorkflowStepCompleted`、`WorkflowCompleted`
- `std trigger terminal` 返回 `NeedsExternalRunner`

覆盖：

- GUI 中立业务中心
- Action、Registry、Event、Config、Storage、Audit
- AI Planner 本地计划
- 外部 runner 默认 defer

## Launcher

状态：未完成

证据：

- `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance` 返回 `PASS`
- `frontmost_preserved=true`
- `frontmost_before` 等于 `frontmost_after`
- target bundle id 为 `dev.std-cli.background-ui-harness`
- target window title 为 `std-cli Background UI Harness`
- target identity 通过 bundle id、pid、window id、window title 四重匹配
- `registered=true`
- `commands=Visible(true),Focus`
- `visible_after_close=false`
- `resident_after_close=true`
- `second_event_received=true`
- `target/ui-evidence/launcher-light-results-refined.png` 为 766 x 506 PNG
- `target/ui-evidence/launcher-dark-results-refined.png` 为 766 x 506 PNG
- `target/ui-evidence/launcher-light-no-results-refined.png` 为 766 x 506 PNG
- `target/ui-evidence/launcher-light-defer-refined.png` 为 766 x 506 PNG
- `target/ui-evidence/launcher-light-error-refined.png` 为 766 x 506 PNG

已覆盖：

- 常驻应用语义
- 默认隐藏
- 真实全局快捷键
- 关闭只隐藏
- 隐藏后再次唤起
- 外部行为默认 defer
- light / dark 截图
- 搜索结果、无结果、defer、错误状态截图

缺口：

- Launcher 截图仍需按 docs/18-21 做像素级审计，不得只用文件存在判定完成
- 真实全局热键安装包验收仍需单独显式运行，不进入默认回归门禁
- 焦点环、IME、A11y、reduce motion 和安装版 UI 需要真实证据

## Studio

状态：未完成

证据：

- `std-studio --smoke` 返回 `PASS`
- `workspace_panes=7`
- `workflow_status=Completed`
- `batch_status=NeedsExternalRunner`
- `analysis_coverage_complete=2`
- `plugin_status=Completed`
- `/tmp/std-studio-installed-ui.png` 为 3840 x 2160 非空截图
- System Events 窗口名为 `std-cli Studio`

已覆盖：

- Workflow 创建、编辑、模拟、运行
- 执行轨迹
- Plugin 管理和 manifest 检查
- Memory 写入和浏览
- Index 分析、搜索、问答、coverage
- QA、Doctor、Release、Install 状态面板
- Settings
- workspace pane 模型
- 真实 UI 渲染

缺口：

- Studio UI 仍需按 docs/18-24 重新验收，不得用 headless smoke 替代
- light / dark、workspace pane 打开聚焦关闭恢复、焦点、A11y、Operations 真实证据截图需要重新证明

## Terminal

状态：未完成

证据：

- `std --help` 覆盖 `workflow batch index analyze plugin memory config doctor release install`
- `std config list` 返回隔离 `data_dir=/tmp/std-cli-smoke-data`
- `std doctor --json` 返回 `status=PASS`
- `std workflow check` 返回 `status=Completed`
- `std run` 返回 `status=Completed`
- `std batch` 返回 `status=NeedsExternalRunner`
- `std analyze` 返回 `kind=Project`

覆盖：

- 脚本友好输出
- 所需命令面
- 默认 smoke 不唤起外部 runner

## Plugin

状态：PASS

证据：

- `std plugin check examples/plugins/hello-js` 返回 `status=PASS`
- `std plugin check examples/plugins/typed-ts` 返回 `status=PASS`
- `std plugin run hello-js` 返回 `status=Completed` 和 `runtime=deno_core`
- `std plugin run plugin-typed-ts` 返回 `status=Completed` 和 `runtime=deno_core`
- `cargo test --workspace -- --test-threads=1` 覆盖 scoped fs、network、clipboard 权限边界

覆盖：

- deno_core JS
- deno_core TS
- scoped fs
- network
- clipboard 权限

## Index

状态：PASS

证据：

- `std index rebuild /tmp/std-cli-smoke-project` 返回 `components=1`
- `std files index /tmp/std-cli-smoke-project` 返回 `entries=1`
- `std index coverage` 返回 `complete=1`、`incomplete=0`
- coverage 四项均为 true
- `std index ask SmokeComponent` 返回 defines_type 和 implements_type evidence

覆盖：

- Entity Overview
- Component Digest
- Symbol / Relation Index
- Historical Context
- Search
- Ask

## Workflow

状态：PASS

证据：

- `std workflow new cli-smoke` 返回 `workflow created`
- `std workflow step add` 返回 `step_type=Action`
- `std workflow check` 返回 `status=Completed`
- `std run` 返回 `status=Completed`
- `std workflow trace --limit 5` 返回 completed trace 和 audit events

覆盖：

- 创建
- 编辑
- 模拟
- 运行
- 执行轨迹

## Release

状态：PASS

证据：

- `cargo build --release --workspace` PASS
- `std release package --version 1.0.0 --from target/release --dist dist/1.0.0-current` PASS
- `std release verify --dist dist/1.0.0-current` PASS

覆盖：

- release manifest
- checksums
- app bundles
- docs/examples/quality evidence

## Install

状态：PASS

证据：

- `std install run --prefix /tmp/std-cli-install-current --from dist/1.0.0-current/bin` PASS
- `std install verify --prefix /tmp/std-cli-install-current` PASS
- installed `std`、`std-launcher`、`std-studio` smoke 均 PASS

覆盖：

- binaries
- macOS app bundles
- storage 初始化

## Quality

状态：PASS

证据：

- `mise run quality` PASS
- `cargo fmt --all --check` PASS
- `cargo clippy --workspace --all-targets -- -D warnings` PASS
- `cargo dylint --workspace --all -- --all-targets` PASS
- `cargo test --workspace -- --test-threads=1` PASS
- `cargo deny check` PASS
- `cargo machete` PASS
- `std doctor --json` 返回 `source_file_limit=500`、`config_file_limit=300`

覆盖：

- 只使用 Rust 生态质量工具
- 不恢复 `std quality`
- Rust 源文件低于 500 行
- 配置文件低于 300 行
- 默认测试不唤起 Terminal、App 或外部 runner

## 最终门槛

状态：PASS

完成前必须重跑并保留当前证据：

- `mise run quality`
- `cargo build --release --workspace`
- release package / verify
- install run / verify
- installed CLI smoke
- installed Launcher GUI hotkey smoke
- installed Studio smoke 和 UI 截图
- installed Plugin JS/TS smoke
- installed Index coverage
