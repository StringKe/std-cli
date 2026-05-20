# 22. Studio UX Spec - Studio 表面 UX 详规

本文件是 Studio 单宿主窗口、workspace pane、Canvas、Inspector、Workflow Builder、Analysis Workbench 的 UX 合订本。与 09_Studio_Surface_Detail 互补：09 偏「workspace 与功能区拓扑」，本文件偏「像素与交互细则」。视觉 token、动效、键盘逻辑来自 18、19、20。

## 01. 设计目标

- **专业感**：界面密度可与 Xcode、JetBrains IDE 对标，开发者不会觉得「这只是个 launcher 的辅助 GUI」
- **可发现 + 可深挖**：每个抽象（Workflow / Skill / Memory / Action / Plugin）都有专门面板，但不需要先读说明书
- **Workspace 协作**：用户可以同时盯一个 Workflow Builder 和一个 Analysis Workbench，互不打扰
- **AI 内建**：所有面板都能在不切换上下文的前提下调用 AI 辅助

## 02. Workspace 拓扑

**主窗口**（每个 Studio 实例一个）：

- 名称：Dashboard
- 标题栏文本：`std-cli Studio`
- 默认尺寸：1280 x 800
- 最小尺寸：1080 x 640
- 内容：左 Sidebar + 中 Canvas + 右 Inspector + 底部 Panel

**Workspace pane**（应用内可多开）：

- Workflow Builder：每个 Workflow 一个 workspace pane
- Analysis Workbench：每个分析目标一个 workspace pane
- Memory Browser：单例
- Execution History：单例
- Plugin Manager：单例

**多 workspace pane 行为**：

- 使用 egui 内部 dock/pane 渲染，禁止 macOS 原生子窗口作为主路径
- 同一对象（同一 Workflow id、同一 Analysis target）只允许一个 pane（去重，详见 09 当前实现）
- Pane 间状态通过 std-core 事件总线同步
- 关闭主窗口即关闭当前 Studio 实例内的 workspace panes

**禁止**：

- 嵌入「Studio 内的 Launcher 复刻」（命令面板除外，详见 20-07）
- 任何无主窗口的 detached panel（迷失状态太重）
- macOS / Windows / Linux 原生子窗口混入 Studio 工作台主路径
- 用 modal 打断式 dialog 做日常操作（仅 destructive confirm 允许 modal）

## 03. 主窗口布局

```
┌────────────────────────────────────────────────────────────┐
│  Host Chrome (egui, borderless host window)                │  52
├──────┬──────────────────────────────────────────┬──────────┤
│      │                                          │          │
│ Side │              Canvas / Content            │ Inspector│
│ bar  │                                          │ (toggle) │
│ 240  │            自适应                         │   320    │
│      │                                          │          │
│      │                                          │          │
├──────┴──────────────────────────────────────────┴──────────┤
│ Bottom Panel (toggle): Batch Debug / Logs / Problems       │ 240
├────────────────────────────────────────────────────────────┤
│ Status Bar: 24                                             │
└────────────────────────────────────────────────────────────┘
```

**Sidebar**：

- 宽度默认 240，可拖拽 200-360
- 背景 `bg/surface-1`，无右边框（用 1px `stroke/divider` 隔开）
- 内容分组：`Workspace`（Workflows / Skills / Memory），`Tools`（Plugins / Index / Settings），`Recent`
- 每行 28 高度，左 padding 12，icon `icon/sm` + 标题 `text/body`
- selected 行 `accent/weak`，hover 行 `bg/surface-2`
- 可折叠（`Mod+B`），折叠后宽度 48 仅显示 icon

**Canvas**：

- 默认 `bg/surface-0`（与自绘 host chrome 相连，制造「整窗主背景」错觉）
- 内部组件自己负责更高层 surface 嵌套
- 顶部可挂可选的 Toolbar（高度 44），不强制

**Inspector**：

- 宽度默认 320，可拖拽 280-480
- 默认折叠
- 触发：选中某个可深挖对象时自动展开 + 可手动 `Mod+I` 切换
- 内容来自当前 Canvas 上下文，schema-driven 自动生成表单
- 背景 `bg/surface-1`，左 1px divider

**Bottom Panel**：

- 默认折叠
- 触发：`Mod+J`，或 Batch Debug 启动时自动展开
- 高度默认 240，可拖拽 160-480
- tabs：`Batch Debug` / `Logs` / `Problems` / `Performance`
- 背景 `bg/surface-1`

**Status Bar**：

- 高度 24，背景 `bg/surface-1`
- 左：当前 workspace 状态（index 状态、connection 状态）
- 右：分析进度、当前 AI provider、版本号
- 字号 `text/caption` 11，颜色 `fg/secondary`
- 不可承担操作（status bar 不是 toolbar，避免误点）

## 04. Host Chrome

- 根窗口使用无装饰 eframe viewport，Studio 自绘 host chrome
- macOS / Windows / Linux 只保留一个系统宿主窗口，不使用原生子窗口
- 关闭 / 最小化 / 最大化是中性工具栏按钮，使用 egui `ViewportCommand`
- 禁止模拟 macOS 交通灯按钮，避免看起来像原生窗口 chrome
- host chrome 中允许放置当前 workspace、pane 数量、刷新和打开当前视图
- drag region 覆盖非交互区域，按钮和输入控件不得抢拖拽行为

## 05. Workflow Builder

```
┌────────────────────────────────────────────────────────────┐
│ Toolbar: [Save] [Test ▶] [Simulate] [History]   AI · zoom  │ 44
├──────────────────┬─────────────────────────────────────────┤
│                  │                                         │
│  Steps           │  Step Properties (selected step)        │
│  (list, DnD)     │  (schema-driven form)                   │
│                  │                                         │
│  + Add Step      │                                         │
│                  │                                         │
├──────────────────┴─────────────────────────────────────────┤
│ AI Assist Panel (collapsible)                              │
│ "Describe what this workflow should do ..."                │
└────────────────────────────────────────────────────────────┘
```

**Steps 列表**：

- 行高 48，左 icon + 主标题 + 副标题（步骤类型）
- selected 状态用 `bg/surface-3` + 左 4px `accent/base` 竖条
- DnD：行级 grabber（左侧 6 像素拖把），拖动期间整行 `elev/2` 浮起 + 半透明，释放位置显示 2px `accent/base` 横线 indicator
- 拖动期间禁用 hover 高亮（避免视觉冲突）
- `Alt+Up` / `Alt+Down` 键盘移动（详见 20）

**Step Properties**：

- schema-driven form，每个 field 占一整行
- field 标签 `text/body-strong`，描述 `text/footnote` `fg/secondary`
- 输入控件：text input、textarea、select、number、boolean、JSON editor（用 egui_extras 表格）
- 复杂参数（如 prompt）支持「展开为大编辑器 modal」

**AI Assist Panel**：

- 默认折叠为单行 input 框 + chevron
- 展开后约 200 高度，显示「最近建议 + 自由输入」
- 建议项以卡片形式展示，可一键 Apply / Insert / Replace selected step

**Test / Simulate**：

- Test：跑真 action（写 audit log）
- Simulate：dry run（不写副作用）
- 启动后底部 Panel `Batch Debug` 自动展开，按行实时显示每步状态：`pending` / `running` / `success` / `error` / `skipped`
- 错误行可点击展开 stack + payload
- Test 过程中可 `Cancel`

## 06. Analysis Workbench

```
┌────────────────────────────────────────────────────────────┐
│ Target Path / Bundle ID / Repo URL    [Re-Index]  Q&A 输入  │ 44
├────────────────────────────────────────────────────────────┤
│ Tabs: [Overview] [Components] [Symbols] [Relations] [Q&A]  │ 36
├────────────────────────────────────────────────────────────┤
│                                                            │
│  Tab content                                               │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

**Overview**：

- 三栏卡片：基本信息（路径 / 大小 / 语言 / 框架）、Index 状态（四层覆盖、最近 re-index）、最近活动
- 每张卡片 `bg/surface-1` `radius/md` `space/md` padding

**Components**：

- 左侧树（component digest），右侧选中组件的描述 + 关键 symbol 列表
- 树最大深度 5 层，超过折叠
- 节点 hover 显示 tooltip：路径 + 文件数 + symbol 数

**Symbols**：

- 表格视图（egui_extras Table），列：symbol / kind / file / line / score
- 顶部 sticky 搜索框（`Mod+F`）
- 行右键菜单：`Open in Editor` / `Find References` / `Copy Path`

**Relations**：

- 一期：列表呈现 inbound / outbound 关系，按数量降序
- 二期：egui 自绘有向图，节点 ≤ 200 时使用力导布局，> 200 时退化为分层视图
- `Mod+L` 列表 / 图视图切换

**Q&A**：

- 自然语言输入框 + 历史对话列表
- 回答区支持 Markdown + 代码块 syntax highlight（用 syntect 或类似）
- 每条回答下方显示「引用来源」（4 层索引中的具体文件 / symbol，可点击跳转）

## 07. Memory Browser

- 左 Sidebar：分类（执行历史 / 用户偏好 / 项目知识 / 工具链模式）
- 中 List：每项 + 时间戳 + tag
- 右 Inspector：详情 + 关联事件
- 顶部全文搜索 + 时间范围过滤
- 任意项可一键「Pin to Workflow」生成 memory recall step

## 08. Execution History

- 表格：时间 / Workflow / 状态 / 耗时 / 触发来源（Launcher / Studio / CLI / Plugin）
- 双击行展开 timeline 视图：每个 step 的开始 / 结束 / payload
- 失败行红色左条 `status/danger`
- 顶部过滤器（时间范围、状态、Workflow name）

## 09. Plugin Manager

- 列表：插件 name / version / status / source / `Enable` switch
- 选中行右侧 Inspector：description、permissions、command list、最近 audit log
- 顶部 `Install from path` `Install from registry`（registry 二期）
- 安全提示：未审计 plugin 启用前显示 dialog 列出权限范围

## 10. Settings

- 单独窗口（`Mod+,`）
- 左 Sidebar 分类：Appearance / Hotkeys / AI Provider / Index / Plugins / Privacy / About
- 右内容区表单
- 任何 destructive 操作（重置全部 / 删除索引）有 modal 二次确认 + typed-confirm（输入「DELETE」）

## 11. Workspace 协作行为

- Workflow 在 Launcher 触发执行 -> Studio 自动 toast 显示「Execution started」+ 可点击跳到 Execution History
- 在某个 Workflow Builder 中保存 -> 其他窗口监听到 `WorkflowChanged` 事件并刷新本地视图
- 不同 workspace 的 Selection 不共享（避免「我在 Builder 选了 Step 3，结果 Inspector 跳到别的区域」）
- Workspace 焦点切换无动画，保持专业工作台的稳定感

## 12. 拖拽（DnD）

**允许的 drop targets**：

- Workflow Builder Steps 列表：拖入步骤排序
- Analysis Workbench Target：拖入文件夹 / 单文件触发索引
- Memory Browser：拖入文本 / 文件创建 Memory 条目
- Plugin Manager：拖入 .zip 触发安装提示

**视觉**：

- drop zone 上 1px dashed `accent/base` 边框 + `accent/weak` 背景
- 不可 drop 区域 hover 时显示 `not-allowed` cursor
- 拖入时整窗背景不变化（避免大面积 flash）

**键盘等价**：

- 所有 DnD 必须有键盘等价（菜单 / 按钮 / 命令面板），否则不实现
- 详见 20-K-01

## 13. 表单与输入

- 输入框高 28，padding 内 8/12，圆角 `radius/sm` 4
- placeholder `fg/tertiary`
- 校验失败：边框 `status/danger` + 下方 `text/footnote` `status/danger` 错误说明
- 校验成功不显示绿色（避免过度反馈）
- 必填字段不在 label 旁加 `*`，而是用 placeholder「required」或下方说明文本

**JSON / YAML / Code 编辑**：

- 内嵌 `egui_code_editor` 或类似 lib
- 默认主题与 18 配色对齐（提供两套主题：dark / light，禁止内置 monokai 等异色）
- 行号 `fg/tertiary`，当前行背景 `bg/surface-2`

## 14. 信息密度上限

| 区域 | 上限 | 超过时 |
| --- | --- | --- |
| Sidebar 单层 item | 20 | 折叠分组 + 全文搜索 |
| Steps 列表 | 100 | 提示拆分 Workflow |
| Symbols 表格 | 5000 | 分页 + virtual list |
| Relations 图节点 | 500 | 分层视图 |
| Bottom Panel logs | 10000 行 | 滚动 buffer 截断 + 提示 |

## 15. 通知与反馈

- Toast：右下角，最多堆叠 3 条，每条 4s 自动消失，可点击 dismiss
- 关键完成（Workflow 成功）：toast 主标题 + 「Open History」副 action
- 错误：toast 红色左条，永不自动消失，必须用户 dismiss
- 系统级通知：仅在 Studio 失焦时发送，避免抢焦点

## 16. Performance 与验收

- `StudioPerformanceReport`：开窗时间、Canvas 重绘 95p、列表渲染时长；`std-studio --smoke` 必须覆盖 workspace 打开 / 关闭
- [ ] 所有面板使用 18 token，无新增颜色 / 间距 / 字号
- [ ] 所有动效在 19 表内
- [ ] 所有快捷键不冲突 20-06 与系统保留
- [ ] DnD 都有键盘等价
- [ ] dark + light 双模式验证
- [ ] Reduce Motion 下面板切换瞬时无伪动画
- [ ] Workspace pane 同时打开 3 个：Workflow Builder + Analysis + Memory，CPU < 10%（空闲）
- [ ] `std-studio --smoke` PASS
