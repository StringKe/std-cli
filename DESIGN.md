# DESIGN.md - std-cli 设计上下文

## Design Register

product

## Visual Strategy

std-cli 是高频开发者工具，视觉策略采用 restrained product UI：中性冷灰 surface、单一蓝色 accent、语义 status 色只在状态需要时出现。视觉目标是让熟悉 Raycast、Linear、Figma、JetBrains、Xcode 的用户能立即信任，而不是被装饰吸引。

## Theme

同时支持 light / dark。Dark 是开发者主要工作环境下的基线，Light 必须同等完成，不能只是反色或临时 fallback。

物理场景：开发者在 27 寸或笔记本屏幕上连续工作，白天和夜间都会频繁热键唤起 Launcher，并在 Studio 中长时间编辑 Workflow、查看执行状态和分析项目结构。界面必须在暗光和亮光中都保持低疲劳、高可读。

## Tokens

视觉 token 由 `std-egui::tokens` 提供，业务 UI 禁止直接写颜色、字号、间距和圆角魔法值。

### Typography

- UI sans：Inter Variable
- Code：JetBrains Mono
- 字号：caption 11、footnote 12、body 13、title 15、headline 18、display 24
- 字重：Regular 400、Medium 500、Semibold 600
- 禁止 display font、斜体、过多字号、全大写标题

### Colors

- `bg/surface-0`：dark `#1C1E22`，light `#FAFBFD`
- `bg/surface-1`：dark `#24272C`，light `#F2F5F8`
- `bg/surface-2`：dark `#2D3138`，light `#E8ECF1`
- `bg/surface-3`：dark `#373C44`，light `#DBE1E8`
- `fg/primary`：dark `#ECEEF1`，light `#1A1C20`
- `fg/secondary`：dark `#B5BAC1`，light `#4B5057`
- `accent/base`：dark `#4E9CFF`，light `#0A6BFF`
- 禁止纯黑 `#000` 和纯白 `#FFF` 出现在产品 UI 视觉值中

### Spacing

8pt grid。允许值：4、8、12、16、20、24、32、48。

### Radius

- `radius/sm` 4：按钮、输入框、keycap
- `radius/md` 8：列表选中、卡片
- `radius/lg` 12：popover、modal、toast
- `radius/xl` 16：Launcher 外框

### Elevation

不模拟物理光源。阴影只用于分离浮层和底面。Launcher 浮层用 `elev/3`，Action Panel 用 `elev/2`。

## Layout Principles

- 信息密度优先于留白，留白只服务扫描效率
- 层级靠字号、字重、颜色对比、间距和 surface，不靠装饰
- 禁止 nested cards
- Launcher 是固定浮层结构，不做大窗口 carrier
- Studio 是单宿主窗口和内部 workspace pane，不使用原生子窗口做主交互

## Launcher Design Contract

- 宽度固定 `min(720px, viewport x 0.55)`
- 未输入高度 64px，输入后按结果扩展
- 搜索栏使用 `text/headline`，结果行 36px，分组标题 24px，Action Bar 36px
- 状态必须覆盖 empty、searching、results、no-results、action-panel、executing、defer、error
- Search、Results、Action Panel、Feedback 必须都有键盘路径和焦点证据
- 外部行为默认 `NeedsExternalRunner`
- 截图验收必须覆盖 light、dark、results、no-results、defer、error

## Studio Design Contract

- 主窗口默认 1280 x 800，最小 1080 x 640
- Host Chrome 52，Status Bar 24，Sidebar 240，Inspector 320，Bottom Panel 240
- 主路径是 single borderless egui host viewport + internal workspace panes
- 视图必须覆盖 Dashboard、Workflow Builder、Analysis Workbench、Plugin Manager、Memory Browser、History、Operations、Settings
- Workflow Builder 必须覆盖 create、edit、simulate、run、trace
- Plugin Manager 必须展示 manifest、runtime、permissions、audit log
- Analysis Workbench 必须展示 overview、components、symbols、relations、Q&A 和四层 coverage
- Operations 必须展示 QA、Doctor、Release、Install、Runtime 的真实命令和结果

## Motion

动效用于解释状态，不用于装饰。

- Hover：80ms
- Selected：140ms
- Popover：220ms
- Launcher enter：320ms
- Reduce Motion 下非功能动效瞬时切换
- 禁止长于 480ms 的常规动画
- 禁止自定义曲线，必须使用 docs/19 定义曲线

## Keyboard And Focus

- 任何 mouse 可达操作必须 keyboard 可达
- Launcher 打开焦点在搜索框
- Studio Workflow Builder 打开焦点在步骤列表
- 任意时刻只有一个可见焦点环
- 键盘来源显示焦点环，鼠标来源不显示焦点环
- IME composing 期间 Enter、Esc、箭头不触发 action
- 全局热键默认 Alt+Space

## Accessibility And Localization

- WCAG 2.2 AA
- High contrast 提升 secondary text、divider 和 focus ring
- Reduce Motion、Reduce Transparency、Bold Text、UI scale 都是一等输入
- 所有面向用户字符串走 i18n，zh-CN 和 en-US 同步
- 状态不能只靠颜色，必须有 icon 或文本

## egui Constraints

- 业务 state 在 Core，UI state 在 egui
- 大列表用 virtual rows
- 表格用 `egui_extras::Table`
- 不实现 Liquid Glass、Vibrancy、backdrop blur、lensing
- 不在 `update()` 做磁盘 IO、网络、JSON parse 或 SQL query
- 不用 `egui::Window` 或 detached viewport 做 Studio 主路径

## Verification

UI 完成需要真实 evidence：

- `mise run quality`
- release build/package/verify
- install run/verify
- Launcher / Studio 截图矩阵
- Launcher keyboard、IME、hotkey 或 background UI acceptance
- Studio workspace pane open/focus/switch/close/restore
- Plugin JS/TS runtime
- Index coverage
