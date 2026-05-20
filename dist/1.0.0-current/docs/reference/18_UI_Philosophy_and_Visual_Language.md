# 18. UI Philosophy and Visual Language - UI 哲学与视觉语言

本文件是 std-cli 所有可见界面（Launcher、Studio）视觉决策的唯一基准。任何 widget、面板、组件改动必须先对照本文件的 token 表与原则。与 02_Design_Principles 冲突时，以 02 为准。

## 01. 灵感来源与边界

**借鉴**：

- Apple Human Interface Guidelines（macOS 26 Tahoe / Liquid Glass）的层级语言、字号节奏、间距 grid、动效节奏
- SF Pro / SF Mono 的字体度量比例（用同等度量的开源替代字体）
- SF Symbols 7 的线性图标语言（用开源等价物 + 自绘）
- macOS 系统级 Materials 的 thick / regular / thin 三档思路

**不抄**：

- Liquid Glass 折射、镜面反射、边缘 lensing：egui 无法低成本复现，强行模拟会增加渲染负担且效果劣化
- iOS / iPadOS 风格的圆角卡片堆叠：std-cli 是开发者工具，工具感优先于内容感
- 系统级 Vibrancy（NSVisualEffectView）的桌面色彩穿透：跨平台一致性优先
- Apple 商品化光泽、渐变玻璃质感

**底线**：

- 看起来必须像「专业开发者会信任的工具」，而不是消费级 App
- 一切视觉决策服从两个表面的核心目标：Launcher 极速、Studio 高密度
- 在 macOS、Windows、Linux 上视觉一致，不做 native-look 的平台适配

## 02. 五条哲学

**P-01 信息密度优先于留白**
开发者每秒处理的信息远多于普通用户。留白服务于扫描效率（分组、对齐），不服务于呼吸感。

**P-02 一切层级靠对比，不靠装饰**
层级通过字重、字号、颜色对比、间距建立，禁止用边框、阴影、渐变去「美化」层级。

**P-03 不引入第二色相**
全局只用一个 accent 色（默认 macOS system blue 等价），其余全部中性灰阶。Status 色仅在确实需要时使用（成功、警告、错误、信息），不参与品牌表达。

**P-04 动效服从可读性**
动效只用于：状态切换连续性、焦点位置追踪、长任务进度反馈。装饰性动画一律不做。详见 19_Motion_and_Interaction_Rhythm。

**P-05 文档先于像素**
任何新组件的视觉规则在引入前必须落到 token 表或本文件，禁止「先做出来再补 token」。

## 03. 字体

**字族**：

- UI sans：`Inter Variable`（开源、与 SF Pro 度量接近）
- 代码 / 路径 / 命令：`JetBrains Mono`（开源、与 SF Mono 等价）
- 不引入第三种字族

**字号 token**（单位 logical px，DPI 自适应由 egui 处理）：

| Token | px | 行距 | 用途 |
| --- | --- | --- | --- |
| `text/caption` | 11 | 14 | 状态标签、shortcut hint、辅助说明 |
| `text/footnote` | 12 | 16 | 次要 metadata、列表分组标题 |
| `text/body` | 13 | 18 | 默认正文、列表项主标题、表单 label |
| `text/body-strong` | 13 | 18 | 选中项标题、强调正文（仅字重切换） |
| `text/title` | 15 | 20 | 面板/卡片标题 |
| `text/headline` | 18 | 24 | Studio 窗口标题、Launcher 大型空态 |
| `text/display` | 24 | 30 | 仅用于 Onboarding 引导首屏 |

**字重**：仅使用 Regular (400)、Medium (500)、Semibold (600)。禁止 Light、Bold、Heavy、Black。

**代码字号**：固定 `text/code = 12px / 16 line-height`，单一档位，禁止可调（用户缩放走整体 zoom，不改 token）。

**禁用**：

- 全大写标题（除 shortcut 显示 `⌘K` 这类 ASCII 符号）
- 斜体 UI 文本
- 三种以上字号在同一面板共存

## 04. 颜色

**模式**：light + dark 双轨。系统跟随 + 用户强制覆盖。

**中性阶**（dark mode token / light mode token）：

| Token | dark | light | 用途 |
| --- | --- | --- | --- |
| `bg/surface-0` | #0E0F11 | #FFFFFF | Launcher 浮层底、Studio 窗口底 |
| `bg/surface-1` | #16181B | #F7F8FA | 默认面板背景 |
| `bg/surface-2` | #1E2126 | #EEF0F3 | 嵌套面板、Hover 行 |
| `bg/surface-3` | #262A30 | #E3E6EA | Selected 行、Inspector 区分 |
| `bg/overlay` | rgba(0,0,0,0.36) | rgba(0,0,0,0.18) | Modal 遮罩 |
| `fg/primary` | #ECEEF1 | #1A1C20 | 主文字 |
| `fg/secondary` | #B5BAC1 | #4B5057 | 次要文字 |
| `fg/tertiary` | #7A8089 | #7A8089 | 辅助文字、占位 |
| `fg/disabled` | #4A4F56 | #B8BCC2 | 禁用态 |
| `stroke/divider` | #262A30 | #E3E6EA | 分隔线 |
| `stroke/border` | #34393F | #D0D4D9 | 输入框、卡片边 |

**Accent**（单一品牌色，跟随 dark/light 微调）：

| Token | dark | light | 用途 |
| --- | --- | --- | --- |
| `accent/base` | #4E9CFF | #0A6BFF | 焦点环、主按钮、selected 强调 |
| `accent/weak` | rgba(78,156,255,0.18) | rgba(10,107,255,0.12) | Selected 行底色 |
| `accent/hover` | #6AAEFF | #2680FF | Accent 元素 hover |

**状态色**（仅在功能必要时出现，不参与排版）：

| Token | dark | light | 含义 |
| --- | --- | --- | --- |
| `status/success` | #3DCB7C | #138750 | PASS、执行成功 |
| `status/warning` | #F5B643 | #B27500 | 警告、SKIP |
| `status/danger` | #FF6A6A | #C8312B | FAIL、删除确认 |
| `status/info` | #4E9CFF | #0A6BFF | 中性提示（复用 accent） |

**禁用**：

- 多色渐变背景（仅允许同色相极浅渐变在大面积空态使用，必须配 motion-off fallback）
- 任何彩色 emoji 风格的图标着色
- 同一界面出现两个以上非中性色块
- 在 dark mode 用纯黑 #000 或纯白 #FFF（眼睛过曝）

## 05. 间距

**8pt grid 强制**。所有 padding / margin / gap 必须是 `4 / 8 / 12 / 16 / 20 / 24 / 32 / 48` 之一。

**语义 token**：

| Token | px | 用途 |
| --- | --- | --- |
| `space/2xs` | 4 | 图标与文字间距、行内 inline gap |
| `space/xs` | 8 | 列表项内部 padding、按钮内 padding |
| `space/sm` | 12 | 列表项之间、表单字段之间 |
| `space/md` | 16 | 面板内 padding（默认） |
| `space/lg` | 24 | 区块之间、Inspector 分组之间 |
| `space/xl` | 32 | 主区域之间（Studio Canvas 与 sidebar） |
| `space/2xl` | 48 | 空态居中区上下边距 |

**禁用**：

- 5、7、10、14、15、18、22 等非 grid 值
- 用 `space/lg` 解决两个紧密相关元素的间距问题（应该用 `xs` 或 `sm`）

## 06. 圆角

| Token | px | 用途 |
| --- | --- | --- |
| `radius/none` | 0 | 表格、Studio Canvas 内部网格 |
| `radius/sm` | 4 | 按钮、输入框、Tag、shortcut 键帽 |
| `radius/md` | 8 | 列表项 selected 高亮背景、卡片 |
| `radius/lg` | 12 | Launcher 主面板、Modal、Toast |
| `radius/xl` | 16 | 仅 Launcher 整体浮层外框 |

**禁用**：

- 完全圆形（pill）按钮，除非用于头像或 status dot
- 同一组件内不同子元素使用不同圆角档位

## 07. 阴影与 Elevation

不模拟物理光源。阴影**仅用于将悬浮层与底面拉开层次**，参数固定。

| Token | dark 参数 | light 参数 | 用途 |
| --- | --- | --- | --- |
| `elev/0` | 无 | 无 | 嵌入式（面板、列表行） |
| `elev/1` | `0 1 2 rgba(0,0,0,0.4)` | `0 1 2 rgba(0,0,0,0.06)` | Hover 卡片、Tooltip |
| `elev/2` | `0 8 24 rgba(0,0,0,0.5)` | `0 8 24 rgba(0,0,0,0.10)` | Popover、Action Panel |
| `elev/3` | `0 16 48 rgba(0,0,0,0.6)` | `0 16 48 rgba(0,0,0,0.16)` | Launcher 浮层、Modal |

**禁用**：

- 模糊半径 > 64 的阴影
- inner shadow（egui 渲染代价高且无收益）
- 多层阴影叠加营造光感

## 08. 边框与分隔

- 默认 1px `stroke/divider`
- 输入框、卡片用 1px `stroke/border`
- 焦点态用 2px `accent/base`（详见 20_Keyboard_Focus_and_Input）
- 禁止双线、虚线分隔（虚线仅用于「dashed drop zone」一类语义场景）

## 09. 表面层级（Surface Hierarchy）

**目标**：用户在 Studio / Launcher 任意位置都能立刻分清自己在第几层。

| 层级 | 物理含义 | token | 典型组件 |
| --- | --- | --- | --- |
| L0 桌面 | 系统桌面 | 系统提供 | 不归我们管 |
| L1 表面底 | Launcher 浮层、Studio 主窗口 | `bg/surface-0` | 整窗背景 |
| L2 主分区 | 内容主区、sidebar | `bg/surface-1` | List 容器、Canvas 底 |
| L3 内嵌面板 | Inspector、Action Panel、子卡片 | `bg/surface-2` | Right Inspector |
| L4 高亮态 | Selected 行、Pressed | `bg/surface-3` 或 `accent/weak` | Hover、Selected |
| L5 浮层 | Popover、Modal、Toast | `bg/surface-1` + `elev/2 或 3` | 上下文菜单 |

**规则**：

- 任意两个相邻层级颜色对比 ≥ 4%（dark）或 ≥ 3%（light），否则用 `stroke/divider` 加 1px 线补足
- L5 浮层必须配 `elev/2` 以上 + `radius/md` 以上
- 同一面板内不允许出现三层及以上嵌套（视觉 noise）

## 10. 图标系统

**风格**：单色线性图标，stroke width 1.5px，corner radius 内部矩形 1.5px。

**尺寸 token**：

| Token | px | 用途 |
| --- | --- | --- |
| `icon/xs` | 12 | 行内 inline、shortcut hint |
| `icon/sm` | 16 | 列表项、按钮内、tab |
| `icon/md` | 20 | Sidebar 主导航、空态辅助 |
| `icon/lg` | 32 | 空态主图、App 头像 |
| `icon/xl` | 48 | Onboarding |

**颜色**：

- 默认 `fg/secondary`
- 选中态 `accent/base`
- 禁用态 `fg/disabled`
- 禁止任何彩色图标（status indicator 例外，但只用 `status/*` token，不上彩色 emoji）

**来源**：

- 一期使用 `lucide` 开源图标集（与 Apple SF Symbols 视觉接近，许可证 ISC）
- 二期自绘约 20 个 std-cli 专属图标（Workflow / Skill / Memory / Action / Index 等核心抽象图标）
- 禁止从设计稿直接复制 SF Symbols（许可证不允许商业重发布）

## 11. 暗黑模式

- 默认跟随系统
- 用户可在 Settings 强制 dark / light
- 模式切换无过渡动画（避免每次系统切换都触发整窗 flash，详见 19）
- Dark mode 是默认运行模式（开发者主要工作环境），所有截图、smoke evidence 以 dark 为基线

## 12. 命名与一致性

- 同一 UI 概念全局一个词。Launcher 里叫 Result，Studio 里也叫 Result，不要在 Studio 里改叫 Entry / Match / Item
- token 命名空间：`text` `bg` `fg` `stroke` `space` `radius` `elev` `icon` `accent` `status`，禁止扩张
- 新 token 必须由本文件维护者确认后加入，禁止在组件本地定义魔法值

## 13. 写代码时如何引用

所有 token 通过 `std-egui::tokens` 模块导出：

```rust
use std_egui::tokens::{Space, Radius, Color, Text};

let frame = egui::Frame::new()
    .fill(Color::bg_surface_1(ctx))
    .inner_margin(Space::Md)
    .corner_radius(Radius::Md);
```

**禁止**：

- 在业务代码里写 `Color32::from_rgb(78, 156, 255)`
- 在业务代码里写 `egui::vec2(15.0, 17.0)` 一类非 grid 值
- 在业务代码里硬编码 `12.0` 字号

任何硬编码视觉值都会被 Code Quality 中的 grep 检查拦截（详见 14_Code_Quality 后续补丁）。

## 14. 验收清单（任何 PR 触及 UI 必须自查）

- [ ] 字号、字重均来自 token 表，无硬编码
- [ ] 颜色全部来自 token，无 RGB 字面量
- [ ] 间距全部是 8pt grid 值
- [ ] dark + light 双模式截图自查无破洞
- [ ] 焦点态、disabled 态、hover 态、selected 态可视化无歧义
- [ ] 任意两层相邻表面有可辨识的层级差
- [ ] 未引入新的色相、字族、阴影档位
- [ ] 与 19、20、21、22 文档不冲突

---

**维护**：StringKe

**更新**：本文件每次变更必须同步更新 `std-egui::tokens` 模块的实现并跑 `std-studio --smoke` `std-launcher --smoke`。
