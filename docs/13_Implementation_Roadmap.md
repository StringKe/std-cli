# 13. Implementation Roadmap — 实现路线图

**原则**：小步快跑、文档先行、避免 devkitx 式过度工程。

## Phase 0: 地基（0~4 周）—— 可用 Launcher

**目标**：拥有一个能全局热键弹出的极简 Launcher，能搜索本地内容并执行简单操作。

- 初始化 Cargo workspace（按 Architecture 推荐的 crate 结构）
- 搭建 `std-core` + `std-types` 基础
- 实现 Launcher 窗口（egui + winit，2026 最新方式）
- 全局热键集成 + macOS 浮动窗口行为打磨
- 基础模糊搜索（nucleo）
- 简单的 Action 注册与触发
- 终端 CLI 框架（clap）
- 完成第一个可运行的 release（能用热键弹出搜索面板）

**里程碑**：`v0.1.0` — “能用的全局入口”

## Phase 1: Workflow 闭环（5~10 周）

- 完善 Workflow 数据模型（declarative frontmatter 风格）
- 实现基础 Workflow 执行引擎（顺序步骤 + 错误处理）
- ToolRegistry 设计（参考 devkitx AgentTool trait）
- Launcher 支持搜索和触发 Workflow
- 简单 AI Planner（把自然语言转成可执行步骤）
- 基础执行历史记录

**里程碑**：`v0.3.0` — “Workflow 能跑，AI 可以参与规划”

## Phase 2: Studio 基础能力（11~18 周）

- 实现 Studio 主窗口框架（多窗口管理）
- Workflow 编辑器（列表 + 属性面板模式）
- AI 辅助生成/修复 Workflow 面板
- 简单执行模拟器与轨迹查看
- 剪切板历史增强（搜索、分类、语义）

**里程碑**：`v0.5.0` — “有真正的构建界面”

## Phase 3: 个人理解能力（19~28 周）—— Eney 方向起步

- 实现 `std-index` 层（多层索引框架）
  - 项目/Workflow 概览层
  - 文件与步骤摘要层
  - 符号/调用关系层
- Studio 中支持“分析一个项目/Workflow”
- AI 可以基于索引回答结构化问题
- Mise 工具链隔离集成

**里程碑**：`v0.8.0` — “开始能理解自己东西的内部结构”

## Phase 4: 生态与深化（29 周以后）

- deno_core 插件系统 + `@std-cli/sdk`
- 更强的可视化 Workflow 编辑（如果需要）
- 语音全程打通（高质量 STT + TTS）
- 个人 Memory 系统（长期 + 向量）
- 插件市场 / 远程 Workflow 来源（可选）
- 性能、主题、macOS 深度集成打磨

**长期目标**：`v1.0+` 接近 Eney 级个人开发者智能层，但保持克制和专注。

## 版本策略

- 0.x 阶段：快速迭代，每 2-4 周一个有价值的 milestone
- 每个 Phase 结束时更新 Architecture 和 Roadmap
- 严格遵守 Design Principles

**当前状态**：Phase 0 启动中。

---

**维护者**：StringKe