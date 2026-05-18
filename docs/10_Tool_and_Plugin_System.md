# 10. Tool and Plugin System — 工具与插件系统

## 设计目标

提供统一的方式，让系统、开发者、AI 都能方便地扩展能力，同时保持安全和可控。

## 核心抽象：AgentTool-like Trait

受 devkitx 优秀设计启发，定义统一的 `StdAction` / `Tool` trait：

```rust
pub trait StdAction: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> &str;
    fn parameters_schema(&self) -> Value;   // JSON Schema
    fn is_read_only(&self) -> bool;
    fn execute(&self, args: Value, ctx: &ExecutionContext) -> Result<Value>;
}
```

所有 Skill、Command、Workflow 步骤、内置工具最终都实现此 trait 并注册到 `ActionRegistry`。

## 两种扩展方式

### 1. Declarative Tools（首选）
- 纯 Markdown + frontmatter 定义
- 系统自动注册为 Action
- AI 最容易生成

### 2. Imperative Plugins（通过 deno_core）
- 用户编写 TypeScript / JavaScript
- 通过 `@std-cli/sdk` 调用 Host 提供的受控能力
- 细粒度权限声明（类似 Deno）

## 插件安全

- declarative 默认安全
- code 插件必须声明权限（clipboard、fs scoped、network、shell 等）
- 执行时有资源限制和超时看门狗

## 内置工具分类

- 系统原语（paste、notification、window management）
- 文件与搜索
- Workflow 相关（子 Workflow 调用、状态查询）
- AI 相关（planner、memory recall）
- 第三方集成（通过插件）

---

**Registry 是整个系统能力发现和 AI 规划的基石。**