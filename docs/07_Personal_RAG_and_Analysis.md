# 07. Personal RAG and Analysis — 个人 RAG 与结构化分析（Eney 方向）

## 目标

让 std-cli 能够“真正读懂”开发者自己 Mac 上的世界：
- 项目代码结构
- 自定义 Workflow
- 常用工具和配置
- 剪切板历史、执行轨迹
- 甚至外部应用的可分析部分

最终实现用户可以说：“分析一下这个 App / 这个 Workflow / 这个项目，告诉我它的核心模块和调用关系。”

## 多层索引设计（参考 devkitx 4 层模型，适配开发者场景）

1. **Entity Overview**（实体概览）
   - 项目 / Workflow / 工具 / App 的高层摘要
   - 技术栈、主要入口、用途

2. **Component Digest**（组件摘要）
   - 文件、模块、步骤的用途总结
   - 关键导出和接口

3. **Symbol / Relation Index**（符号与关系索引）
   - 函数、类、Workflow 步骤、配置项的精确定义
   - 调用、依赖、数据流关系（使用 tree-sitter 或自定义解析器）

4. **Historical Context**（历史上下文）
   - 执行历史、修改记录、AI 对话片段
   - 用户操作模式

## 实现策略

- `std-index` crate 负责统一索引框架
- 支持可插拔的 Analyzer（针对不同实体类型：Rust 项目、Workflow 定义、macOS App bundle 等）
- 向量嵌入 + 结构化存储结合（tantivy + sqlite-vec）
- 索引更新采用增量 + 后台策略，避免阻塞 Launcher

## Studio 中的 Analysis Workbench

- “分析”按钮 → 触发索引 + 生成结构化报告
- 可视化：依赖图、调用链、关键路径
- AI 解释面板：自然语言提问 + 基于索引的回答

## 与 Workflow 的协同

分析结果可以直接被 Workflow 引用（例如“在部署前先分析目标项目结构”）。

---

**这是 std-cli 区别于普通 launcher 的核心差异化能力。**