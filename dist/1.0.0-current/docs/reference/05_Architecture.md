# 05. Architecture - std-cli 技术架构

**最后更新**：2026-05-19
**原则**：本架构严格遵循 `02_Design_Principles.md`，以「GUI 中立 + 强 Core」为核心。

## 1. 整体设计哲学

std-cli 采用 **「一个进程 + 多表面 + 强 Core」** 的架构。

- 所有业务逻辑、执行引擎、索引、AI 能力集中在 `core` 层
- Launcher（全局热键面板）和 Studio（专业窗口）只是 Core 的不同渲染前端
- 全部使用 Rust + egui，不引入 WebView / Tauri

目标是在保证 Launcher 极致快速的前提下，同时支撑复杂的 Workflow 构建和 Eney 式的个人环境深度分析能力。

## 2. 高层架构图

```text
std-cli process
  egui layer
    Launcher window: floating hotkey panel
    Studio windows: desktop multi-window workspace
  egui integration layer
    shared widgets, view models, window coordination
  core layer
    std-types: Action, Skill, Command, Workflow, Memory, Event
    std-core: Registry, storage, execution, plugins, planner
    std-orchestration: workflow dry run, execute, trace, batch
    std-index: four-layer structural index and question answering
    std-cli: script-friendly terminal surface
```

## 3. Crate 结构（推荐，2026 最新实践）

采用适度拆分，避免 devkitx 过度工程化：

```toml
# Cargo.toml (workspace)
[workspace]
members = [
    "crates/std-types",
    "crates/std-core",
    "crates/std-orchestration",
    "crates/std-index",           # 个人 RAG / 分析引擎
    "crates/std-egui",             # egui 公共组件 + 窗口管理
    "crates/std-launcher",         # Launcher 专属入口
    "crates/std-studio",           # Studio 窗口集合
    "crates/std-cli",              # 终端命令行
]
```

**各 crate 职责**：

- `std-types`：所有公共数据结构（不可变优先）
- `std-core`：最核心的注册、执行、AI 规划能力
- `std-orchestration`：Workflow 状态机与调度
- `std-index`：多层个人索引 + 结构化搜索（Eney 能力核心）
- `std-egui`：跨表面共享的 egui 组件、主题、Viewport 管理
- `std-launcher` / `std-studio`：各自的窗口实现（只调用 Core + egui 组件）

## 4. 关键技术选型

| 领域               | 当前方案                          | 当前版本                  | 理由 |
|--------------------|-----------------------------------|---------------------------|------|
| GUI 框架           | `egui` + `eframe`                | 0.28                      | 纯 Rust、即时模式、适合本地工具 |
| 窗口事件           | `winit`                          | 0.30                      | 与 egui 表面协同 |
| 全局热键           | `global-hotkey`                  | 0.6                       | Launcher 注册和事件匹配 |
| 模糊搜索           | `nucleo`                         | 0.5                       | 快速搜索 App、Workflow、Memory、文件 |
| JS/TS 插件运行时   | `deno_core`                      | 0.400                     | 受控权限和 Rust host API |
| 结构化索引         | `std-index` 自有轻量解析          | workspace                 | Entity、Component、Symbol、History 四层输出 |
| 异步运行时         | `tokio`                          | 1.44                      | Workflow 和工具执行基础 |
| 配置解析           | `figment`                        | 0.10                      | 支持多层 TOML、JSON、YAML 配置 |

## 5. Launcher 窗口特殊实现要点

- 使用 `winit` 创建 `Window` 时设置 `decorations: false`、`transparent: true`、`always_on_top: true`
- 当前实现使用 egui + eframe 窗口和 `global-hotkey` 注册计划；不依赖 WebView
- 输入事件全部走 egui，搜索使用 `nucleo`
- Launcher 状态层支持热键切换、搜索、预览、键盘移动、语音 transcript 和执行反馈

## 6. Studio workspace pane 实现

- `std-studio` 维护 `StudioApp` 和 egui 内部 workspace pane model
- 每个窗口类型映射到真实 pane content snapshot
- Workflow builder、Analysis workbench、Plugin manager、App manager、Memory browser、Execution history 都复用共享 Core

## 7. Core 与 egui 的边界

**严格禁止**：
- 在 Core 里 import `egui`、`winit` 任何类型
- 在 Core 里直接操作窗口

**正确做法**：
- Core 暴露同步 Rust API 和可测试的执行结果
- egui 层通过 view model 把用户意图发给 Core
- Core 把状态变化写入审计事件，CLI、Launcher、Studio 可读取同一份事实

## 8. 演进路径（避免过度设计）

v1.0 已落地：
- Launcher + 基础 Core + Workflow 执行
- Studio workspace pane + Workflow 编辑器 + 执行轨迹
- `std-index` 四层结构化分析
- `deno_core` JavaScript / TypeScript 插件系统

## 9. 依赖管理建议

- 使用 `cargo` workspace + `[workspace.dependencies]`
- 关键依赖锁定最新 minor 版本（通过 `cargo update` 策略）
- 定期 review 依赖（尤其是 egui 生态更新较快）

---

**本架构以当前 workspace 的 Cargo 配置和测试证据为准。**
