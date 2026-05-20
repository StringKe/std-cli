# 09. Studio Surface Detail - Studio 多窗口详细设计

## 窗口组织

- 主窗口：Dashboard（Workflow 列表、最近执行、索引状态）
- 编辑器窗口：Workflow Builder（可多开）
- Analysis 窗口：实体分析工作台
- 辅助窗口：Memory Browser、Execution History、Plugin Manager

## Workflow Builder（MVP 版本）

- 左侧：步骤列表（可拖拽排序、添加/删除）
- 右侧：选中步骤的属性面板（基于 schema 自动生成表单）
- 底部：AI 辅助面板（描述需求 -> 生成/优化步骤）
- 顶部工具栏：保存、测试执行、模拟、版本历史
- Batch Debug：用 `std-orchestration::BatchExecutor` 执行 JSON batch plan，展示每个 action / workflow step 的状态、defer 和错误结果

## Analysis Workbench

- 支持拖入或选择一个实体（项目目录、Workflow 文件、App bundle）
- 触发索引后展示多层视图：
  - 概览
  - 组件树
  - 关系图（轻量）
  - 自然语言问答界面

## 技术实现

- 多窗口使用 winit + egui 的 Viewport 能力
- 状态通过事件总线同步
- 复杂视图（关系图）初期使用 egui 自定义 Painter，后续可引入 egui_graphs 等成熟库

## 当前实现

- `std_studio::StudioApp` 维护 `StudioWindow` 列表、焦点窗口和去重打开逻辑
- 支持从主窗口打开 Dashboard pane、Workflow Builder、Analysis Workbench、Memory Browser、Execution History、Plugin Manager
- `std-studio` egui 入口在主窗口内渲染 workspace windows，不使用 macOS 原生子窗口作为主路径
- Workflow 创建、编辑、模拟、执行和历史读取保持在 `std-studio/src/workflow.rs`
- 多窗口状态由单元测试覆盖，确保打开、聚焦、关闭、标题生成和重复打开去重
- `std-studio --smoke` 使用临时 data_dir 运行 headless GUI state smoke，覆盖多窗口、Workflow、Batch、Analysis、Memory、Plugin 和 History

---

Studio 的目标是让「专业的事情有专业的界面去做」。
