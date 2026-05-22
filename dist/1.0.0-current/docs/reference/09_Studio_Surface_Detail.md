# 09. Studio Surface Detail - Studio workspace 详细设计

## 窗口组织

- 宿主窗口：一个无装饰 eframe viewport，由 egui 自绘标题栏和主体
- 主 workspace：Dashboard（Workflow 列表、最近执行、索引状态）
- Workspace pane：Workflow Builder、Analysis Workbench、Memory Browser、Execution History、Plugin Manager
- Workspace pane 可以同时打开多个，但全部渲染在同一个宿主窗口内

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

- 主路径不使用 macOS / Windows / Linux 原生子窗口
- Workspace 使用 egui 内部状态对象和 pane 渲染
- 状态通过事件总线同步
- 复杂视图（关系图）初期使用 egui 自定义 Painter，后续可引入 egui_graphs 等成熟库

## 当前实现

- `std_studio::StudioApp` 维护 workspace pane 列表、焦点 pane 和去重打开逻辑
- 支持从主窗口打开 Dashboard pane、Workflow Builder、Analysis Workbench、Memory Browser、Execution History、Plugin Manager
- `std-studio` egui 入口在主窗口内渲染 workspace panes，不使用 macOS 原生子窗口作为主路径
- Workflow 创建、编辑、模拟、执行和历史读取保持在 `std-studio/src/workflow.rs`
- Workspace pane 状态由单元测试覆盖，确保打开、聚焦、关闭、标题生成和重复打开去重
- `std-studio --smoke` 使用临时 data_dir 运行 headless GUI state smoke，覆盖 workspace pane、Workflow、Batch、Analysis、Memory、Plugin 和 History

---

Studio 的目标是让「专业的事情有专业的界面去做」。
