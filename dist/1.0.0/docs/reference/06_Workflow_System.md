# 06. Workflow System - Workflow 系统

## 核心定位

Workflow 是 std-cli 的最高生产力抽象，是用户把重复劳动固化、让 AI 参与规划和执行的主要载体。

## Workflow 定义方式（双轨制）

### 1. Declarative Workflow（推荐首选）

位置：`~/.std-cli/workflows/<name>/`

结构示例：
```
my-deploy/
- workflow.md          # frontmatter + 描述
- steps/
  - build.yml
  - deploy.yml
- references/
```

`workflow.md` 使用 YAML Frontmatter + Markdown，与 std-ai 风格保持一致。

优势：
- AI 极易生成和理解
- 版本控制友好
- 声明式优先，符合 Design Principles

### 2. Code-enhanced Workflow

当 declarative 无法满足复杂逻辑时，允许在 `steps/` 下放置 `.ts` / `.js` 文件，由 deno_core 执行。

## 执行模型

- Workflow 由 Orchestration 引擎调度
- 支持顺序步骤、条件分支、循环、AI 子任务和人工输入步骤
- 外部 runner 默认不执行，返回 `NeedsExternalRunner`，只有显式 `allow_external` 才执行
- 步骤之间通过上下文（Context）传递数据
- Action 可以声明 `input_schema` 与 `output_schema`
- Workflow dry run 会把已解析 Action 的 schema 暴露到每个 `StepDryRun`
- Workflow execute 会在运行 Action 前校验 step parameters，不符合 `input_schema` 时失败并阻止实际执行
- Studio Workflow Debug 会显示已解析步骤的 schema 摘要，支撑属性面板和 AI 规划

## 与 AI 的深度集成

- Workflow 定义自动暴露为 tool
- AI Planner 可以将用户自然语言需求拆解成 Workflow 执行计划
- AI Planner 会读取本地上下文并写入每个 `PlanStep.parameters.context`：
  - `memory_titles`
  - `clipboard_items`
  - `indexed_entities`
  - `workflow_actions`
- `PlanStep.reason` 包含 Registry 命中字段和上下文计数，便于 Launcher、Studio、Terminal 解释计划来源
- 支持 `std plan <goal>` 生成计划，并可保存为 Workflow

## 状态与持久化

- 执行过程产生 Workflow execution history、StepResult 和 trace summary
- 支持人工输入节点，执行前可通过 dry run 校验响应
- 完整审计日志

---

**本系统设计目标**：让开发者既能轻松写出可靠的自动化，又能让 AI 真正理解和协助 Workflow 的全生命周期。
