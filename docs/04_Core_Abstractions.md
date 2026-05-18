# 04. Core Abstractions — 核心抽象

## 设计目标

所有可执行内容都必须是第一等、可被 AI 发现和参与的抽象。

## 核心抽象层级（从低到高）

### 1. Action
用户在 Launcher 或 Studio 中可见的最小可触发单元。

例如：
- 启动某个 App
- 执行某个 Workflow
- 粘贴某段 clipboard 内容
- 调用某个 Skill

### 2. Skill
有明确 `description` 和 `when_to_use`，可被 AI Planner **主动发现和调用**的原子能力。

推荐使用 declarative 形式（目录 + frontmatter + Markdown），与 std-ai 风格保持一致。

### 3. Command
用户显式输入 `/command-name` 触发的模板。

### 4. Workflow（最高抽象）
多步骤、可状态、可分支、可被 AI 完整规划和生成的自动化单元。

Workflow 是 std-cli 的核心生产力载体。

一个 Workflow 可以包含：
- 对 Skill / Command 的调用
- 条件分支与循环
- AI 子任务
- 用户交互步骤
- 外部工具调用（通过 deno_core）

### 5. Memory
长期可被 recall 的结构化上下文。

包括但不限于：
- 执行历史
- 用户偏好
- 项目特定知识
- 工具链使用模式

## 统一注册机制

所有以上抽象最终都要注册到 **Registry** 中，供：

- Launcher 搜索
- AI Planner 发现和规划
- Studio 可视化与编辑

Registry 设计参考 devkitx 的 `AgentTool` trait 思路，但要更适应 Workflow 场景。

## AI 友好性要求

每个抽象都必须提供：

- `name`
- `description`（高质量、自然语言）
- `when_to_use`
- `input_schema`（JSON Schema）
- `output_schema`
- `examples`

这样 AI 才能高质量地进行 tool calling 和任务规划。

---

**本文件为后续 Tool System 和 Workflow Engine 设计的基础。**