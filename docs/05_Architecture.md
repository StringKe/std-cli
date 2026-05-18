# 05. Architecture — std-cli 技术架构（2026 最新生态版）

**最后更新**：2026-05-18  
**原则**：本架构严格遵循 `02_Design_Principles.md`，以「GUI 中立 + 强 Core」为核心。

## 1. 整体设计哲学

std-cli 采用 **「一个进程 + 多表面 + 强 Core」** 的架构。

- 所有业务逻辑、执行引擎、索引、AI 能力集中在 `core` 层
- Launcher（全局热键面板）和 Studio（专业窗口）只是 Core 的不同渲染前端
- 全部使用 Rust + egui（2026 最新生态），不引入 WebView / Tauri

目标是在保证 Launcher 极致快速的前提下，同时支撑复杂的 Workflow 构建和 Eney 式的个人环境深度分析能力。

## 2. 高层架构图

```
┌─────────────────────────────────────────────────────────────┐
│                        std-cli 进程                          │
├─────────────────────────────────────────────────────────────┤
│                        egui Layer                            │
│  ┌──────────────────────┐      ┌──────────────────────────┐ │
│  │   Launcher Window    │      │      Studio Windows      │ │
│  │  (浮动、无边框、热键)  │      │  (普通桌面多窗口)          │ │
│  │   egui + winit       │      │   egui + winit           │ │
│  └──────────┬───────────┘      └──────────┬───────────────┘ │
│             │                               │                 │
│             └───────────────┬───────────────┘                 │
│                             │                                 │
├─────────────────────────────▼─────────────────────────────────┤
│                       egui Integration Layer                   │
│  (窗口管理、主题、事件桥接、Viewport 协调，不包含业务逻辑)        │
├───────────────────────────────────────────────────────────────┤
│                         Core Layer (纯 Rust)                   │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
│  │   Types     │  │   Registry   │  │   Orchestration      │ │
│  │ (Action/    │  │ (Tool/Skill/ │  │ (Workflow 调度)       │ │
│  │  Workflow)  │  │  Workflow)   │  │                      │ │
│  └─────────────┘  └──────────────┘  └──────────────────────┘ │
│  ┌──────────────────────────────┐  ┌──────────────────────┐ │
│  │      Personal Index / RAG     │  │     AI Layer         │ │
│  │  (多层结构化索引 + 语义搜索)    │  │ (Planner + Context)  │ │
│  └──────────────────────────────┘  └──────────────────────┘ │
│  ┌──────────────────────────────┐  ┌──────────────────────┐ │
│  │       Execution Engine       │  │   Deno Runtime       │ │
│  │   (步骤执行、权限、隔离)       │  │  (deno_core 嵌入)     │ │
│  └──────────────────────────────┘  └──────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                    Infra & Services                     │ │
│  │  (存储、Mise 环境、审计日志、global-hotkey 抽象等)         │ │
│  └────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────┘
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
- `std-index`：多层个人索引 + 语义搜索（Eney 能力核心）
- `std-egui`：跨表面共享的 egui 组件、主题、Viewport 管理
- `std-launcher` / `std-studio`：各自的窗口实现（只调用 Core + egui 组件）

## 4. 关键技术选型（2026 最新生态）

| 领域               | 推荐方案                          | 版本建议（2026）          | 理由 |
|--------------------|-----------------------------------|---------------------------|------|
| GUI 框架           | egui + winit + wgpu              | egui 0.31+ / winit 0.31+ | 纯 Rust、即时模式、2026 年多窗口支持已成熟 |
| 多窗口管理         | winit + egui-winit + 自定义 ViewportCoordinator | 最新 stable              | 能精细控制 Launcher 的浮动行为 |
| 全局热键           | `global-hotkey` crate            | 最新                       | 跨平台，macOS 支持较好 |
| 模糊搜索           | `nucleo`                         | 最新                       | 目前 Rust 里最快的模糊匹配库 |
| JS/TS 插件运行时   | `deno_core`                      | 最新 0.3xx 系列            | 成熟、可控权限、Rust 互操作极佳 |
| 本地模型 / 嵌入    | `candle` 或 `llama.cpp` Rust 绑定 | 最新稳定版                 | 本地优先 |
| 全文/向量索引      | `tantivy` + `sqlite-vec` 或 Lance | 最新                       | 平衡性能与易用性 |
| 异步运行时         | `tokio`                          | 1.4x+                      | 工业标准 |
| 热键 + 窗口激活    | `winit` + `global-hotkey` + macOS objc2 少量原生代码 | 最新 | 实现真正“点击外部隐藏”的浮动面板 |
| 配置解析           | `figment`                        | 最新                       | 支持多层配置（与 claudex 一致） |

**重要说明**：
- 坚决使用 2026 年时的最新稳定版本，不要使用 2024~2025 年的旧版 egui / winit。
- egui 在 2025-2026 年对多 Viewport / 多窗口的支持已经大幅改善，可以在一个进程内优雅地管理 Launcher（特殊浮动窗口）和多个 Studio 窗口。

## 5. Launcher 窗口特殊实现要点

- 使用 `winit` 创建 `Window` 时设置 `decorations: false`、`transparent: true`、`always_on_top: true`
- 通过 macOS `objc2-app-kit` 少量代码实现 vibrancy（液态玻璃效果）和正确的 activation policy
- 输入事件全部走 egui，搜索使用 `nucleo`
- 窗口失去焦点时自动隐藏（通过 `winit` 的 `WindowEvent::Focused` 处理）

## 6. Studio 多窗口实现

- 推荐在启动时创建一个主 `StudioApp`，后续通过 `std::sync::mpsc` 或 `tokio` channel 向主事件循环发送“打开新窗口”请求
- 每个 Studio 窗口拥有独立的 `egui::Context`，但共享同一个 `AppState`（通过 Arc<Mutex<>> 或 Actor 模型）
- 窗口之间通过事件总线通信，避免直接持有对方引用

## 7. Core 与 egui 的边界

**严格禁止**：
- 在 Core 里 import `egui`、`winit` 任何类型
- 在 Core 里直接操作窗口

**正确做法**：
- Core 只暴露 `trait` 和 `async fn`
- egui 层通过 channel / callback 把用户意图发给 Core
- Core 把状态变化通过事件广播给所有已注册的表面

## 8. 演进路径（避免过度设计）

Phase 1：Launcher + 基础 Core + 简单 Workflow 执行
Phase 2：Studio 基础窗口 + Workflow 编辑器（列表+属性面板）
Phase 3：`std-index` 层 + 初步的个人 RAG 分析能力
Phase 4：deno_core 插件系统 + 更强的结构化理解

## 9. 依赖管理建议

- 使用 `cargo` workspace + `[workspace.dependencies]`
- 关键依赖锁定最新 minor 版本（通过 `cargo update` 策略）
- 定期 review 依赖（尤其是 egui 生态更新较快）

---

**本架构以「2026 年 5 月最新稳定生态」为基准。**

后续如有重大 Rust / egui 生态变化，需更新本文件并重新评估。