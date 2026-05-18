# 03. Surfaces — std-cli 的三个表面

## 概述

std-cli 采用「一个强 Core + 多个表面」的架构。不同使用场景对应不同的表面。

## 1. Launcher（cli 入口）

**这是用户口中的「cli」**。

- 全局热键（推荐 Option/Alt + Space）唤起的浮动面板
- 类似 Raycast / Spotlight / Wox 的主界面，但更克制
- 必须极致快速、极简、可靠

**职责边界（严格）**：
- 搜索本地 App、文件、Workflow、剪切板历史、Memory
- 语音输入（按住说话 → 转文字并执行）
- 快速触发已定义的 Workflow
- 简单 AI 操作（改写、总结、执行简单指令）
- 显示执行结果（轻量 toast / 面板）

**坚决不做**：
- 复杂 Workflow 的多步编辑
- 应用/项目的深度结构分析
- 大量配置界面

## 2. Studio（构建与分析环境）

真正的专业桌面应用，支持多窗口。

**主要功能区**：
- Workflow Builder（创建、编辑、调试 Workflow）
- Analysis Workbench（指向一个 App / 项目 / Workflow，进行结构分析和功能理解）
- Personal Knowledge Explorer（浏览和搜索个人索引）
- Clipboard & History 高级管理
- Memory 可视化与管理
- AI 深度辅助面板

Studio 是 Eney 式「分析并理解」能力的主要承载界面。

## 3. Terminal（辅助命令行）

`std` 二进制提供的子命令。

主要用于：
- 脚本 / CI 调用（`std run <workflow-name>`）
- 索引重建、配置管理
- 批量操作
- 开发者调试

不是主要使用界面。

## 表面之间的关系

- 三个表面共享同一个 Rust Core
- Launcher 追求「毫秒级响应」
- Studio 追求「专业能力」
- Terminal 追求「脚本友好」

---

**更新日期**：2026-05

**维护者**：StringKe