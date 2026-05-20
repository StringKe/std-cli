# 01. Product Vision - std-cli 产品愿景

## 一句话定义

**std-cli 是为重度开发者打造的本地优先、AI 原生的个人自动化与理解层（Personal Developer Intelligence & Automation Layer）。**

它通过一个全局热键图形化入口（Launcher）和一个专业的桌面构建分析环境（Studio），让开发者能够极快地触发操作、固化自动化流程，并真正「理解」自己 Mac 上各种应用、项目、Workflow 和工具链的内部结构。

## 为什么要做这个项目

当前开发者面临的核心痛点：

- 全局入口工具（Raycast、Spotlight、Alfred）功能强大但理解深度不足
- Workflow / 自动化工具要么太简单，要么编辑体验极差
- AI 助手大多是「聊天」，很难真正读懂开发者自己的代码库、工具链和历史操作
- 多个工具之间割裂，没有统一的个人知识与执行层

std-cli 的目标是把「快速触发 + 自动化编排 + 深度理解」三件事，在一个本地可控、体验一致的系统中解决。

## 长期愿景（Eney 方向）

最终希望达到类似 Eney 的能力，但更聚焦开发者：

- 你可以指向一个应用、一个项目、一个复杂的 Workflow，让系统分析其文件结构、调用关系、配置、历史执行轨迹
- 通过自然语言或搜索，就能快速理解「这个东西到底是怎么工作的」
- 同时还能把理解到的知识直接用于 Workflow 自动化和 AI 决策

这不是通用 AI 助手，而是**开发者自己的第二大脑 + 第二双手**。

## 产品形态（分表面设计）

- **Launcher（cli 入口）**：全局热键浮动面板（类似 Raycast / Spotlight，但更克制）
  - 极致快速、极简
  - 支持语音输入
  - 快速搜索与触发 Workflow、App、功能
  - 简单 AI 操作

- **Studio**：真正的桌面多窗口应用
  - Workflow 的专业构建与调试
  - 应用 / 项目 / 工具的结构化分析与可视化
  - 个人知识库浏览与管理
  - AI 深度辅助

- **Terminal**：辅助命令行工具（次要）

## 核心约束

- 全部使用 Rust + egui 实现
- 强 Core、GUI 中立
- Launcher 必须永远保持克制
- Studio 提供专业能力，但不追求消费级花哨
- 本地优先，数据可控

## 与相关项目的协同

- std-ai：负责 AI 配置的标准化（rules/skills/commands）
- claudex：负责 Claude Code 的多提供商智能代理
- std-cli：负责全局入口 + Workflow 自动化 + 个人环境深度理解

三者共同构成开发者 AI 工具链的重要基础设施。

## 当前阶段

当前处于 v1.0 convergence 阶段。重点是把已经落地的强 Core、Launcher、Studio、Terminal、Index、Plugin、Release 和质量门禁收敛到可安装、可验证、可发布的软件状态。继续保持克制，不扩大无证据范围。

---

**StringKe 2026**
