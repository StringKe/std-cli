# std-cli 文档索引

本目录是 std-cli 的单一真相源文档。

## 必读

- [01_Product_Vision.md](01_Product_Vision.md) - 产品愿景与长期定位
- [02_Design_Principles.md](02_Design_Principles.md) - 核心设计宪法（所有决策的最高准则）
- [03_Surfaces.md](03_Surfaces.md) - 三个表面（Launcher / Studio / Terminal）的职责划分
- [04_Core_Abstractions.md](04_Core_Abstractions.md) - Action / Skill / Command / Workflow / Memory 等核心抽象
- [05_Architecture.md](05_Architecture.md) - 整体技术架构（全部 egui + 强 Core）

## 完整文档列表

- [06_Workflow_System.md](06_Workflow_System.md) - Workflow 定义、执行、AI 集成
- [07_Personal_RAG_and_Analysis.md](07_Personal_RAG_and_Analysis.md) - 个人环境索引与结构化理解（Eney 方向）
- [08_Launcher_Surface_Detail.md](08_Launcher_Surface_Detail.md) - Launcher（cli 入口）详细设计
- [09_Studio_Surface_Detail.md](09_Studio_Surface_Detail.md) - Studio 多窗口与编辑分析界面
- [10_Tool_and_Plugin_System.md](10_Tool_and_Plugin_System.md) - Tool Registry 与插件机制
- [11_Event_Protocol.md](11_Event_Protocol.md) - 事件驱动与表面通信协议
- [12_Configuration_and_Storage.md](12_Configuration_and_Storage.md) - 配置、存储和权限边界
- [13_Implementation_Roadmap.md](13_Implementation_Roadmap.md) - 分阶段实现路线图
- [14_Code_Quality.md](14_Code_Quality.md) - Rustfmt、Clippy 与 release 质量门禁
- [15_Terminal_Automation.md](15_Terminal_Automation.md) - 终端自动化与 batch 计划
- [16_Completion_Audit.md](16_Completion_Audit.md) - v1.0 完成审计和当前运行证据
- [17_Final_Completion_Matrix.md](17_Final_Completion_Matrix.md) - v1.0 逐项完成矩阵

## UI 与体验文档（Launcher + Studio）

- [18_UI_Philosophy_and_Visual_Language.md](18_UI_Philosophy_and_Visual_Language.md) - UI 哲学与视觉 token 体系（字号 / 颜色 / 间距 / 圆角 / 阴影 / 表面层级）
- [19_Motion_and_Interaction_Rhythm.md](19_Motion_and_Interaction_Rhythm.md) - 动效曲线、时长档位、场景动效表、Spring 映射、Reduce Motion 降级
- [20_Keyboard_Focus_and_Input.md](20_Keyboard_Focus_and_Input.md) - 键盘优先、焦点环、跨平台修饰键、全局热键、IME
- [21_Launcher_UX_Spec.md](21_Launcher_UX_Spec.md) - Launcher 浮层从唤起到执行的全链路 UX 细则
- [22_Studio_UX_Spec.md](22_Studio_UX_Spec.md) - Studio 多窗口布局、Workflow Builder、Analysis Workbench 等 UX 细则
- [23_Accessibility_and_Localization.md](23_Accessibility_and_Localization.md) - WCAG AA、屏幕阅读器、HC、Reduce Motion、i18n zh-CN / en-US
- [24_egui_Implementation_Constraints.md](24_egui_Implementation_Constraints.md) - egui 落地约束、Material 替代策略、自定义 widget 规范

所有文档均以当前 workspace 的代码、配置和验证证据为基准。

## 写作规范

- 所有文档使用简体中文 + 必要英文术语
- 遵循 Design Principles 中的所有约束
- 新增文档需更新本索引
- 重大架构变更必须先更新 Design Principles

---

**StringKe std-cli 启动文档**
