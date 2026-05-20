# 19. Motion and Interaction Rhythm - 动效与交互节奏

本文件定义 std-cli 所有 UI 动效的时长、曲线、触发条件、降级策略。任何引入动画的 PR 必须先映射到本文件的某条规则。与 18_UI_Philosophy 冲突时以 18 为准。

## 01. 动效哲学

**M-01 动效是因果叙述**
每个动画必须回答用户的一个具体问题：「我点的东西去哪了？」「这个新出现的东西从哪来？」「系统还在处理还是已经完成？」。无叙述价值的动画禁止存在。

**M-02 优先于装饰：响应感**
Launcher 永远优先「立刻有响应」，宁可截断动画的中段也不让用户感觉系统迟钝。任何动效在用户下一次输入到来时必须可被打断或丢弃。

**M-03 模仿物理但不模拟物理**
借鉴 Apple Spring 的「自然停止」感受，但用 egui 原生 `CubicBezier` 与 ease-out 函数近似，避免在每帧做 mass / stiffness 解算。

**M-04 全局曲线统一**
全局只允许 4 条曲线 + 1 个 linear（仅 progress 用）。组件 PR 不允许引入自定义 `cubic-bezier(...)`。

**M-05 Reduce Motion 是一等公民**
所有非功能必要的动效在系统 Reduce Motion 或用户在 Settings 关闭动效时，**必须降级为瞬时切换**。功能必要的（如 progress、focus indicator）允许保留，但需简化。

## 02. 时长档位

| Token | ms | 适用 |
| --- | --- | --- |
| `dur/instant` | 0 | Reduce Motion 降级、状态恢复 |
| `dur/micro` | 80 | Hover、Pressed、Color tint |
| `dur/short` | 140 | Selected 切换、列表行高亮 |
| `dur/base` | 220 | Popover / Tooltip 进出、Sidebar 收展 |
| `dur/medium` | 320 | Launcher 主面板进出、Modal |
| `dur/long` | 480 | 大窗口 viewport 切换（罕用） |
| `dur/progress` | 不定 | 真实进度跟随，禁止用 ease 模拟「假进度」 |

**禁用**：

- > 480ms 的常规动画
- 同一交互内出现两个 ≥ `dur/medium` 的并行动画
- 用 `dur/long` 做 hover 一类高频微交互

## 03. 曲线档位

全局曲线表，组件代码必须直接引用这 5 个常量之一。

| Token | egui 表达 | 等价 cubic-bezier | 用途 |
| --- | --- | --- | --- |
| `ease/linear` | `Ease::Linear` | linear | 进度条、loader、文本 marquee |
| `ease/out-standard` | `CubicBezier(0.2, 0.0, 0.0, 1.0)` | 0.2, 0, 0, 1 | **默认曲线**（与 egui 2024+ 默认 ease-out 一致），所有出现、selected、focus |
| `ease/in-standard` | `CubicBezier(0.4, 0.0, 1.0, 1.0)` | 0.4, 0, 1, 1 | 离场（消失、关闭、退出） |
| `ease/in-out` | `CubicBezier(0.4, 0.0, 0.2, 1.0)` | 0.4, 0, 0.2, 1 | 双向往返（折叠、抽屉） |
| `ease/snappy` | `CubicBezier(0.18, 1.0, 0.22, 1.0)` | 0.18, 1, 0.22, 1 | 近似 Apple `.snappy` 的轻弹感，用于按下后弹起、success 反馈 |

**禁用**：

- `cubic-bezier` 自定义控制点
- `Ease::Equation(fn)` 自定义函数（除非性能或视觉无替代方案，需文档化说明）
- `ease/snappy` 用在面板进出（过于跳跃，仅限按钮、Tag、indicator）

## 04. Spring 映射

为了与 Apple HIG 节奏保持「感官同步」但避免实时解 spring ODE，我们把 Apple 常用 spring preset **静态映射**为本文件的 cubic-bezier + 时长组合。

| Apple 等价 | std-cli 替代 | 时长 | 曲线 |
| --- | --- | --- | --- |
| `.smooth` | 主面板进出 | `dur/medium` 320ms | `ease/out-standard` |
| `.snappy` | 按钮反馈 | `dur/short` 140ms | `ease/snappy` |
| `.bouncy(0.5, 0.3)` | 成功 toast | `dur/base` 220ms | `ease/snappy` + 1 次 6% scale overshoot（仅 toast，禁止在常规组件用） |
| `.interactiveSpring` | 拖拽吸附 | `dur/short` 140ms | `ease/out-standard` |

**bounce overshoot 实施限制**：

- 仅 Toast 成功态、Onboarding 关键步骤切换两个场景可用
- overshoot 幅度 ≤ 6% scale，回弹 1 次后停止
- 用 egui `animate_value_with_time` 分两段实现：0..1.06 用 `ease/snappy`，1.06..1.0 用 `ease/out-standard`

## 05. 场景动效规范

下表是所有允许的动效组合。表外组合需先更新本文件。

| 场景 | 触发 | 动画属性 | 时长 | 曲线 | Reduce Motion |
| --- | --- | --- | --- | --- | --- |
| Hover 状态 | 指针进入 | bg color | `dur/micro` 80 | `ease/out-standard` | 保留 color 切换，无过渡 |
| Pressed | 按下 | bg color + 0.97 scale | `dur/micro` 80 | `ease/snappy` | 仅保留 color，无 scale |
| Selected 列表行 | 键盘移动 / 点击 | bg + 文本色 | `dur/short` 140 | `ease/out-standard` | 瞬时切换 |
| Focus 环 | Tab / 程序聚焦 | 2px accent 边框 alpha 0->1 | `dur/short` 140 | `ease/out-standard` | 直接显示 |
| Tooltip 进 | hover 500ms 后 | opacity 0->1 + Y -4 -> 0 | `dur/base` 220 | `ease/out-standard` | opacity 瞬时，无位移 |
| Tooltip 出 | 离开 / Esc | opacity 1->0 | `dur/micro` 80 | `ease/in-standard` | 瞬时 |
| Popover / Action Panel 进 | 触发 | opacity 0->1 + scale 0.96->1 | `dur/base` 220 | `ease/out-standard` | opacity 瞬时 |
| Popover / Action Panel 出 | dismiss | opacity + scale 1->0.98 | `dur/short` 140 | `ease/in-standard` | 瞬时 |
| Sidebar 收 / 展 | 拖拽 / 快捷键 | width | `dur/base` 220 | `ease/in-out` | 瞬时 |
| Launcher 主面板进 | 全局热键 | opacity 0->1 + Y -8 -> 0 | `dur/medium` 320 | `ease/out-standard` | opacity 瞬时，无位移 |
| Launcher 主面板出 | Esc / blur | opacity + scale 1->0.98 | `dur/short` 140 | `ease/in-standard` | 瞬时 |
| Modal 进 | open | overlay opacity 0->1 (220ms)，内容 scale 0.97->1 + opacity (220ms) | `dur/base` 220 | `ease/out-standard` | overlay 瞬时，内容直接 |
| Modal 出 | dismiss | 全部 reverse | `dur/short` 140 | `ease/in-standard` | 瞬时 |
| Toast 进 | 操作完成 | Y 16->0 + opacity 0->1 | `dur/base` 220 | `ease/snappy` | opacity 瞬时 |
| Toast 出 | 计时到期 | opacity 1->0 | `dur/micro` 80 | `ease/in-standard` | 瞬时 |
| Collapsing Header | 点击 | height + arrow rotate | `dur/base` 220 | `ease/in-out` | 瞬时切换，箭头直接换向 |
| Tab 切换 | 点击 / Cmd+数字 | 选中下划线 X 位移 | `dur/short` 140 | `ease/out-standard` | 瞬时 |
| 列表内容替换 | 搜索 query 变化 | 整列 opacity 0.6->1（防止抖动） | `dur/short` 140 | `ease/out-standard` | 瞬时 |
| Progress 不定态 | 长任务运行 | linear 旋转 1200ms / 圈 | `dur/progress` | `ease/linear` | 替换为单帧静态指示器 |
| Progress 定量 | 已知进度 | 真实百分比映射 | 跟随 | `ease/linear` | 保留（功能） |
| 拖拽吸附 | 释放在 snap 点 | 位置 | `dur/short` 140 | `ease/out-standard` | 瞬时 |
| 报错 shake | 验证失败 | X 平移 ±6 px 三次 | 总 240ms | `ease/in-out` | 替换为红色 1 帧 flash |

**完全禁止的动效**：

- 整窗淡入淡出（mode 切换 / 主题切换）
- Parallax / 视差滚动
- 装饰性 hover 缩放（按钮放大 1.05 等）
- 装饰性 hover 倾斜 / rotate
- 装饰性 marquee（除非真有滚动需求且文本被截断）
- 数字 odometer 翻牌（除非用户在 Settings 显式打开）
- Loading skeleton 渐变 shimmer（用静态灰底替代，更克制）

## 06. 中断与丢弃语义

**M-06 所有动画必须可被打断**
egui 的 `animate_value_with_time` 在再次调用时会自动从当前值继续动画到新目标。组件代码必须始终把目标值传给 egui，而不是「锁住一个动画跑完」。

**M-07 用户输入到达即可丢弃**
如果用户在 Launcher 主面板进场动画过程中开始输入，输入必须立即生效，动画自然完成或被新的输入触发的动画覆盖。

**M-08 长任务进度不可造假**
禁止用 `ease` 模拟 0% -> 90% 的「假进度」。要么显示真实进度，要么显示不定态 spinner。

## 07. Reduce Motion

**触发条件**（任一即生效）：

- macOS / Windows / Linux 系统级 Reduce Motion 设置
- std-cli Settings `appearance.reduce_motion = true`
- `STD_REDUCE_MOTION=1` 环境变量（用于 CI / smoke test）

**降级规则**：

- 所有「opacity 渐变 + 位移」改为：opacity 直接 1，位移省略
- 所有 scale 改为：直接 1
- 所有 spring overshoot 改为：直接到位
- Progress 不定态旋转 spinner 改为静态点状指示器
- shake 改为单帧红色 flash

**实现要求**：

- `std-egui::motion` 模块导出 `MotionContext`，提供 `is_reduced(ctx) -> bool`
- 所有动效封装函数必须接受 `ctx`，内部根据 `is_reduced` 选择路径
- smoke test 必须覆盖 `STD_REDUCE_MOTION=1` 路径

## 08. 性能预算

**P-01 每帧渲染 < 4ms**
egui immediate mode 下，包含动画的帧整体渲染 + 布局 ≤ 4ms（240Hz 显示器留 buffer）。

**P-02 同时活跃动画 ≤ 8**
单 viewport 内同时跑的 `animate_*` 调用 ≤ 8 个。Launcher 列表 500 行的渲染中，行级动画必须复用同一 animation id，不允许每行各自跑动画。

**P-03 重渲染范围最小化**
动画期间通过 `Context::request_repaint_after(remaining)` 精确告知最近一次重绘时刻，避免持续 `request_repaint()` 引发持续 60Hz 重绘消耗。

**P-04 长尾监测**
Launcher / Studio 启动时挂钩 `Context::frame_time` 上报到 `LauncherPerformanceReport` 和 `StudioPerformanceReport`：若动画期间 95p 帧时间 > 8ms，标记 motion budget FAIL。

## 09. egui 实现要点

```rust
use egui::Context;
use std_egui::motion::{Curves, Durations, MotionContext};

let hovered = response.hovered();
let t = ctx.animate_bool_with_easing(
    response.id,
    hovered,
    Curves::OUT_STANDARD,         // CubicBezier(0.2, 0, 0, 1)
);
let bg = Color32::lerp(
    Color::bg_surface_1(ctx),
    Color::bg_surface_2(ctx),
    t,
);
```

**禁止**：

- 在组件代码中调用 `Context::request_repaint()` 用作动画驱动（应使用 `animate_*` 系列函数 + `request_repaint_after`）
- 在 `update()` 内用 `std::time::Instant` 自行计时
- 在动画 closure 内做 IO / 阻塞调用

**统一封装**（强制）：

- 任何场景动效在第一次落地时必须封装为 `std_egui::motion::*` 的 named function，例如 `motion::popover_enter(ctx, ...)`，组件直接调用，不允许各自抄 cubic-bezier 系数

## 10. 测试与验证

**smoke 路径**：

- `std-launcher --smoke` 必须断言主面板进入动画时长 ≤ `dur/medium` 上限 + 30ms tolerance
- `std-studio --smoke` 必须断言 Modal 进入动画在 Reduce Motion 下时长 = 0

**视觉 regression**：

- 一期不引入 pixel diff（成本高）
- 改为对 `Context::frame_time` 采样 + animation id 数量上限断言

**手测清单**：

- [ ] 打开 / 关闭 Launcher 主面板，1 秒内重复 5 次无视觉撕裂
- [ ] 系统 Reduce Motion 打开后，主面板瞬时显示，无 320ms 等待
- [ ] 拖动 Sidebar 在 60Hz / 120Hz / 240Hz 下顺滑无掉帧
- [ ] Hover 列表 100 行无 FPS 下降
- [ ] CPU profile 中无 spring solver hotspot

## 11. 与其他文档的引用

- 颜色 / 表面层级：详见 18_UI_Philosophy_and_Visual_Language
- 焦点动画的可见性：详见 20_Keyboard_Focus_and_Input
- Launcher 主面板进出节奏：详见 21_Launcher_UX_Spec
- 拖拽 / Sidebar 收展节奏：详见 22_Studio_UX_Spec
- Reduce Motion 与系统设置联动：详见 23_Accessibility_and_Localization
- egui API 落地：详见 24_egui_Implementation_Constraints

---

**维护**：StringKe

**变更须知**：本文件新增任何曲线档位、时长档位、场景行均视为接口变更，需同步更新 `std-egui::motion` 模块并跑 smoke。
