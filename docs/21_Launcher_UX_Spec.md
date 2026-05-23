# 21. Launcher UX Spec - Launcher 表面 UX 详规

本文件是 Launcher 浮层从「热键唤起」到「执行完成」全链路的 UX 合订本，覆盖布局、状态机、输入、列表、Action Panel、空态、错误态、性能预算。与 08_Launcher_Surface_Detail 是「设计 + 实现要点」的总览，本文件是「像素级与交互级」的细则。视觉 token、动效来自 18、19、20。

## 01. 设计目标

- 用户从「想到要做的事」到「事情发生」的时间 < 2 秒
- 99% 操作不离键盘
- 输入到反馈 < 16ms / keystroke，热键到首屏 < 80ms（v1.0 已落地的预算）
- 不抢占焦点：执行完成后默认收回到上游应用，不打断当前任务

## 02. 布局规范

**整体尺寸**：

- 宽度：固定 `min(720px, 视口宽度 x 0.55)`
- 高度：未输入时 64px（仅搜索条），输入后 `64 + min(items x 行高, 视口高度 x 0.6)`
- 圆角：`radius/xl` = 16
- 阴影：`elev/3`
- 背景：`bg/surface-0` 100% 不透明（egui 没有等价 NSVisualEffectView，不做半透模拟）

**屏幕位置**：

- 水平居中
- 垂直位置：屏幕高度 0.28 处（黄金分割上方，便于眼动）
- 多显示器：跟随当前鼠标所在屏，热键唤起时根据鼠标坐标决定

**内部结构**（自上而下）：

| 区域 | 高度 | 内容 |
| --- | --- | --- |
| Search Bar | 64 | 左 `icon/md` 搜索图标 + 输入框 + 右侧 mode tag（可选） |
| Result List | 自适应 | 列表项 + 分组标题 |
| Action Bar | 36 | 左：当前选中项的 path / breadcrumb；右：主 action + Action Panel hint (`Actions ⌘K`) |

**列表项内部结构**（单行高 36，分组标题行 24）：

```
[ icon 20 ]  Title (text/body-strong)        Subtitle (text/footnote, fg/secondary)        [shortcut keycap]  [action hint]
   16px           填充                                           固定右对齐
```

**Search Bar 高度 vs 字号**：

- 输入框字号 `text/headline` 18px（与 Raycast 2022 改版后保持大字号搜索栏一致，强调中心地位）
- placeholder 用 `fg/tertiary`
- 左 icon 颜色 `fg/secondary`

## 03. 状态机

```
[Hidden] --hotkey--> [Open: empty]
[Open: empty] --type--> [Open: searching]
[Open: searching] --results--> [Open: with-results]
[Open: searching] --no-match--> [Open: empty-state]
[Open: with-results] --Mod+K--> [Open: action-panel]
[Open: with-results] --Enter--> [Executing: progress]
[Executing: progress] --done--> [Toast & Hidden]  // 默认
[Executing: progress] --done & pinned--> [Open: result-view]
[Executing: progress] --error--> [Open: error-toast]
[Open: *] --Esc level chain--> [Hidden]
[Open: *] --blur --> [Hidden]                       // 失焦关闭
```

**状态切换的视觉细节**：

- `Hidden -> Open`：详见 19 表格「Launcher 主面板进」
- `Open: empty -> Open: searching`：列表区直接 expand（无动画展开），spinner 在搜索图标位置 fade in 80ms
- `Open: searching -> Open: with-results`：整列 opacity 0.6 -> 1，时长 `dur/short` 140ms，避免抖动
- `Open: with-results -> Executing`：主面板**不关闭**，按钮区显示 progress（详见 06）
- `Executing -> done`：默认 `dur/medium` 320ms 关闭主面板 + toast 显示 1.8s

## 04. 输入与建议

**Query 解析顺序**（每次 keystroke 触发，全程 < 16ms）：

1. **前缀命令**：`/` `>` `?` 三种 prefix
   - `/`：执行注册命令（如 `/workflow new`）
   - `>`：纯 Action 过滤（不搜索文件、App）
   - `?`：自然语言问答（路由到 AI Planner，详见 06）
2. **fuzzy match**：使用 `nucleo` crate（已在 Cargo workspace 中）
3. **scoring**：std-core Registry 已提供 scoring
4. **recency boost**：最近 7 天使用过的项 score x 1.2（详见 09）

**输入辅助**：

- `Tab` 在搜索框内：自动补全选中项的关键词到 query
- `Mod+Backspace`：删除 query 中上一个空格分隔的 token
- query 中显示高亮：匹配字符 `fg/primary`、其他字符 `fg/tertiary`
- query 字符串经过 `trim()`，多空格归一

**禁止**：

- 在 query 输入框中显示 placeholder 之外的「教学性提示文字」
- 自动展开建议（如 Spotlight 旧版的 spelling correction）一律不做
- query 期间预加载结果之外的资源（图片、metadata 详情）

## 05. 结果列表

**分组规则**：

- 默认分组顺序：Action / Workflow > App / File > Clipboard > Memory > Skill > Other
- 用户可在 Settings 自定义顺序
- 同分组内按 score 降序
- 分组标题 `text/footnote` `fg/tertiary` 保留可读标题大小写 + 1px 上 divider，**不参与键盘选中**

**项渲染**：

- icon 优先来源：
  1. 注册时 declared icon（`icon/sm` 16px，单色）
  2. App / File 类目使用系统提供 icon（macOS：`NSWorkspace icon for URL`，Win / Linux 一期 fallback 到默认）
  3. 都没有时使用类目默认图标
- Title 行：`text/body` 普通态、`text/body-strong` selected 态
- Subtitle 行：可选，`text/footnote` `fg/secondary`，超过单行省略号截断
- 右侧 shortcut 一律展示 `Mod+1..9` 中对应 index 的 keycap（前 9 项），第 10 项起不展示
- 右侧 action hint：仅在 selected 时显示主 action 文本 + `Enter` keycap

**Selected 行**：

- 背景 `accent/weak`
- Title 强化为 `text/body-strong`
- 不使用焦点环（焦点逻辑在搜索框，列表项是 selected 而非 focused，详见 20-02）

**键盘**：

- 详见 20-05
- 列表始终至少 1 项 selected（除非 query 无结果）
- 移动到顶 / 底时不 wrap（Raycast 习惯：硬到底，按 `Down` 没反应明确表示边界）

**渲染**：

- 行高固定 36，分组标题 24
- 使用 virtual list（仅渲染可视 + 上下各 5 行）
- 同 query 同 result set 的渲染必须像素级稳定，禁止帧抖动
- 列表最多展示 200 项，超过部分提示 `… 200+ matches, refine your query`

## 06. Action Panel（Mod+K）

**触发**：

- selected 项后按 `Mod+K`
- 也可点击 Action Bar 右侧 `Actions ⌘K`

**视觉**：

- 在主面板右下角弹出 popover
- 宽度 320，最大高 360（超过滚动）
- 圆角 `radius/lg` 12，elev `elev/2`
- 与主面板对齐：右下角对齐 Action Bar 的 `Actions` 按钮

**内容**：

- 当前 selected 项注册的所有次级 actions
- 每项包含 icon + 标题 + 自身 shortcut
- 顶部自带独立搜索框，支持继续过滤
- 默认 selected 第一项
- 外部执行类 action 的顺序固定为 `Review First`、`Run`、`Defer`、`Open in Studio`、`Copy`。默认 Enter 只走 `Review First`，不会打开 app / 文件 / 外部 runner；`Run` 必须由用户在 Action Panel 中显式选中后触发

**键盘**：

- `Up` / `Down` 选中
- `Enter` 触发
- `Esc` 关闭，焦点回主面板搜索框

## 07. 自然语言入口（`?` 前缀 + 语音）

**文本入口**：

- query 以 `?` 开头时，进入 NL mode
- 整个列表区替换为「单条建议卡 + 选项 actions」
- AI Planner 返回 `Suggestion { intent, action_id, params, confidence }`
- confidence ≥ 0.8 时高亮主 action；< 0.8 时显示「2-3 个候选 actions」

**语音入口**：

- 长按全局热键（详见 08_Launcher_Surface_Detail 已落地）
- 持续按住期间显示「录音中」状态：搜索图标位置替换为脉冲指示器，搜索栏背景轻微红色 hint `rgba(status/danger, 0.06)`
- 松开后转写 + 自动走 `?` 前缀路径
- 失败时 toast 显示「转写失败 + retry」

**禁止**：

- AI 路径的延迟超过 `dur/medium`：超过后必须显示 progress 指示器
- 静默调用 AI（语音转写、NL parse 都必须可见反馈）

## 08. 空态 / 错误态 / 加载态

**空 query 时**（主面板刚唤起）：

- 列表区显示「最近使用」前 5 项（如果有）
- 没有历史时显示「Suggested Workflows」3-5 项
- 都没有时显示空白 + 居中 `text/footnote` `fg/tertiary`：`Press / for commands · ? to ask · ↓ for recent`

**搜索无结果**：

- 居中显示
- `icon/lg` 32px 灰色搜索图标
- 主文本 `text/body-strong` `fg/primary`：`No matches`
- 副文本 `text/footnote` `fg/secondary`：`Try a different keyword or press ? to ask`
- 提供 fallback 行（可点击 / 可 Enter）：`Ask AI about "{query}"`

**Loading**：

- 列表区上方 2px 进度条，颜色 `accent/base`，间歇 indeterminate 动画（详见 19）
- 单项异步加载（如 file preview metadata）使用 inline 灰色 shimmer 替代图标，不阻塞整列
- > 200ms 仍未返回结果：search icon 位置叠加旋转 spinner

**Executing**（用户已 Enter 一个 action，正在执行）：

- 搜索栏 placeholder 替换为「Running: {action title}」
- Action Bar 左侧显示当前 action progress
- 不可输入；提供 `Cancel ⌃C` shortcut
- 任务超过 1.5s 仍未完成：建议「移到后台 ⏎」，回车后主面板关闭 + 系统托盘红点提示后续完成

**错误态**：

- 主面板不关闭
- toast 在主面板底部内部显示（而不是系统级 toast），背景 `status/danger` weak alpha
- 错误文本最多 2 行 + `Copy` button + `Retry` button
- 严重错误（如 plugin crash）路由到 Studio Execution History（点击「Open Studio」打开）

## 09. 个人化与排序

**recency / frequency 排序**：

- 每次执行后写 `LauncherUsage` 事件到 std-core 审计日志
- score = base_score x recency_boost x frequency_boost
- recency: 7 日内 x 1.2，30 日内 x 1.05，更早 x 1.0
- frequency: log10(usage_count + 1) x 0.5 加到 score
- 排序对用户透明，不暴露 score 给 UI

**pin / unpin**：

- 用户右键（或 `Mod+P`）任意项 -> pin 到顶部
- 最多 pin 8 项；超过提示并要求 unpin

## 10. 性能预算

| 指标 | 预算 | 度量方式 | 当前 |
| --- | --- | --- | --- |
| 热键 -> 主面板可见 | ≤ 80ms | `LauncherPerformanceReport.hotkey_to_paint` | 已落地 |
| keystroke -> 列表更新 | ≤ 16ms (95p) | 同 report | 已落地 |
| Mod+K -> Action Panel 可见 | ≤ 50ms | `std-launcher --action-panel-smoke` | 已落地 |
| 主面板关闭 -> 完全消失 | ≤ 200ms | `std-launcher --close-smoke` | 已落地 |
| Cold start（first launch） | ≤ 600ms | std doctor | 已落地 |

**回归门禁**：

- 任何 Launcher 改动 PR 必须跑 `std-launcher --smoke`、`std-launcher --window-smoke`、`std-launcher --keyboard-smoke index` 并附输出
- 真实焦点、Enter 打开、窗口 toggle 验证必须优先使用 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance`
- 后台 UI 验收只能操作 `dev.std-cli.background-ui-harness` 隔离窗口，必须验证 bundle id、pid、window id、window title 四重匹配
- 后台 UI harness 必须带本轮 `harness_token`，窗口标题必须是 `std-cli Background UI Harness <token>`，禁止复用旧 harness 或用户已有窗口
- 后台 UI runner 必须输出 `frontmost_preserved=true`，并证明 `frontmost_before` 等于 `frontmost_after`
- 后台 UI runner 只能验证隔离 harness 的后台事件路由，不得把 Terminal、iTerm2、1Password、WeChat、weixin、wechat、微信、System Settings 或用户当前工作窗口作为目标
- `STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space` 只保留为人工安装包热键补充验收，不进入默认回归门禁
- 95p keystroke 时间退化 > 4ms 视为 P0 阻塞

## 11. 与上下游 surface 的关系

- Launcher 不内嵌任何「编辑大对象」体验。需要编辑 Workflow / Skill / Memory 必须打开 Studio
- 提供 `Open in Studio` 作为多数项的次级 action（在 Action Panel）
- Launcher 自己不持久化复杂状态：所有 user data 写入 std-core 存储，跨进程一致
- Studio 命令面板（`Mod+/`）与 Launcher 浮层完全独立，互不调用

## 12. 验收清单（Launcher 改动 PR 自查）

- [ ] 18 视觉 token 全部沿用，无新增 token
- [ ] 所有动效在 19 表内，无新增曲线 / 时长
- [ ] 所有快捷键不冲突系统保留（详见 20-03、20-06）
- [ ] IME composing 期间 Enter 不误触发（详见 20-08）
- [ ] dark + light 双模式空态、错误态、加载态截图自查
- [ ] Reduce Motion 下主面板瞬时显示、列表瞬时切换
- [ ] `LauncherPerformanceReport` 各项 PASS
- [ ] 至少一条新功能的 smoke evidence 加入到 release gate

---

**维护**：StringKe

**关联文档**：03_Surfaces、08_Launcher_Surface_Detail（实现）、11_Event_Protocol（事件）、12_Configuration_and_Storage（存储）
