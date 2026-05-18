# 06. Workflow System — Workflow 系统

## 核心定位

Workflow 是 std-cli 的最高生产力抽象，是用户把重复劳动固化、让 AI 参与规划和执行的主要载体。

## Workflow 定义方式（双轨制）

### 1. Declarative Workflow（推荐首选）

位置：`~/.std-cli/workflows/<name>/`

结构示例：
```
my-deploy/
├── workflow.md          # frontmatter + 描述
├── steps/
│   ├── build.yml
│   └── deploy.yml
└── references/
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
- 支持顺序、并行、条件分支、循环、错误处理、重试策略
- 步骤之间通过上下文（Context）传递数据
- 每个步骤都有明确的输入输出 schema，便于 AI 规划

## 与 AI 的深度集成

- Workflow 定义自动暴露为 tool
- AI Planner 可以将用户自然语言需求拆解成 Workflow 执行计划
- 支持 “生成 Workflow”、“优化现有 Workflow”、“解释这个 Workflow 在干什么” 等能力

## 状态与持久化

- 执行过程产生 TaskAttempt / StepResult
- 支持暂停、恢复、人工确认节点
- 完整审计日志

---

**本系统设计目标**：让开发者既能轻松写出可靠的自动化，又能让 AI 真正理解和协助 Workflow 的全生命周期。