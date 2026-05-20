# std-cli

**开发者个人 AI 自动化与理解层**（Personal Developer Intelligence & Automation Layer）

全局热键入口（Launcher） + 专业构建与分析环境（Studio） + 强 Rust Core，全程使用 egui 实现。

目标：让重度开发者拥有一个本地、可控、能真正「理解」自己工具链和项目的 AI 增强操作系统层。

## 定位

- 不是另一个通用 Raycast / Spotlight 替代品
- 不是纯终端 CLI 工具
- 而是一个**分表面**的开发者基础设施：
  - Launcher（你口中的 cli 入口）：全局热键浮动面板，极致快速
  - Studio：真正的桌面窗口，用于 Workflow 构建 + 应用/项目结构深度分析（Eney 方向）
  - 终端命令行：辅助脚本用途

## 核心能力方向

- 极快全局入口（热键 + 语音）
- Workflow 自动化（可被 AI 深度参与）
- 个人知识与结构化理解（对 App、项目、Workflow、工具链的深度分析与搜索）
- 剪切板增强、长期记忆、AI 编排

## 技术原则

- 全部使用 Rust + egui
- 强 Core、GUI 中立
- 先 declarative 再 code
- 文档先行

## 状态

当前处于 v1.0 convergence 阶段。Workspace 已具备：

- `std` Terminal：配置、Workflow、Batch、Index、Files、Memory、Skill、Command、Plugin、Release、Install、Doctor
- `std-launcher`：全局热键计划、搜索、预览、键盘移动、语音 transcript、外部 runner defer、smoke/perf 报告
- `std-studio`：多窗口工作台、Workflow Builder、Batch Debug、Plugin Manager、Memory Browser、Analysis Workbench、Execution History、Settings
- `std-core`：GUI 中立业务中心，承载 Registry、事件总线、配置、本地存储、审计日志、Action、Memory、Plugin、Index 接入
- `std-index`：Entity Overview、Component Digest、Symbol / Relation Index、Historical Context 四层索引
- Release gate：rustfmt、Clippy、Dylint、cargo-deny、cargo-machete、workspace tests、launcher/index/plugin/workflow smoke evidence

更多信息请查看 `docs/` 目录。

## 相关项目

- [std-ai](https://github.com/StringKe/std-ai)：AI 配置标准化
- [claudex](https://github.com/StringKe/claudex)：Claude Code 多提供商代理

---

**StringKe 2026**
