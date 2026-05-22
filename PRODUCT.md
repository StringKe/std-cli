# PRODUCT.md - std-cli 产品上下文

## Register

product

## Product Purpose

std-cli 是面向重度开发者的本地优先、AI 原生个人自动化与理解层。产品把快速触发、Workflow 自动化、项目与工具链理解整合到一个可安装、可验证、跨平台一致的桌面工具中。

## Primary Users

- 每天长时间使用 macOS、Terminal、IDE、浏览器和开发者工具的重度开发者
- 需要快速触发工作流、搜索本地上下文、审计插件、理解项目结构的个人开发者
- 愿意 review、配置和管理自己工具链的高级用户

## Core Surfaces

### Launcher

全局热键唤起的浮动面板。职责是发现、搜索、预览和触发，不承载复杂编辑。

Launcher 必须：

- 极快，热键到首屏 <= 80ms，输入到结果 <= 16ms
- 克制，固定搜索、结果、Action Panel、反馈几类结构
- 键盘优先，99% 操作不离键盘
- 默认隐藏，关闭只隐藏，二次热键可再次唤起
- 外部 runner 默认 defer，显式 opt-in 才执行

### Studio

单个无装饰宿主窗口内的专业 workspace 应用。职责是构建、分析、管理和验证。

Studio 必须：

- 使用 egui 内部 workspace pane，不把原生子窗口混入主路径
- 覆盖 Dashboard、Workflow Builder、Analysis Workbench、Plugin Manager、Memory Browser、History、Operations、Settings
- 让 Workflow 创建、编辑、模拟、运行、trace 查看成为同一套可用流程
- 让 Plugin、Index、QA、Doctor、Release、Install 状态可视化
- 支持 light / dark、键盘焦点、A11y、空态、错误态和状态反馈

### Terminal

命令行是辅助表面，服务于脚本、验证、release、install、doctor 和 smoke evidence。Terminal 不能替代 Launcher / Studio 的真实 UI 完成证据。

## Strategic Principles

- Core 强于 GUI。业务逻辑在 Core，Launcher 和 Studio 只是渲染表面。
- Launcher 永远克制。复杂编辑、深度分析和配置进入 Studio。
- Studio 提供专业能力。信息密度和操作效率优先，不追求消费级花哨。
- 本地优先。索引、记忆、Workflow、执行历史默认本地保存。
- 文档先于像素。UI 改动必须对齐 docs/18-24。
- 质量必须可证明。完成状态只能来自当前运行证据、截图、交互验证和质量门禁。

## Tone

- 专业、安静、可靠
- 偏工程工具而非营销产品
- 信息密度高，但层级清晰
- 文案简洁，按钮用动词，错误用“无法 / Unable to”

## Anti References

- 黑色或白色整窗底板作为可见 carrier
- macOS 原生 chrome 和自绘 detached window 混用
- 用 smoke 或单元测试冒充 UI 完成证据
- 用装饰性渐变、玻璃拟态、消费级大卡片、英雄指标模板装饰产品 UI
- 在业务 UI 中硬编码颜色、字号、间距或非 token 视觉值

## Completion Standard

v1 UI 完成必须同时具备：

- `mise run quality` PASS
- release build/package/verify PASS
- install run/verify PASS
- Launcher 和 Studio light / dark 真实截图 PASS
- Launcher 结果、无结果、defer、错误状态真实截图 PASS
- Launcher 键盘导航、IME、真实 hotkey 或隔离 background UI acceptance PASS
- Studio workspace pane 打开、聚焦、切换、关闭、恢复真实验证 PASS
- Plugin JS/TS 二进制级运行 PASS
- Index 四层 coverage PASS
- completion audit 覆盖 UI docs 18-24、Launcher、Studio、Core、Terminal、Plugin、Index、Workflow、Release、Install、Quality
