# 20. Keyboard, Focus and Input - 键盘、焦点与输入

本文件定义 std-cli 在键盘、焦点、IME、快捷键上的规范。目标用户是「双手不离键盘的开发者」，键盘可达率必须 100%。与 02_Design_Principles 02 项「Launcher 永远克制」呼应：键盘是 Launcher 的唯一一等输入方式。

## 01. 键盘优先原则

**K-01 任何 mouse 可达的操作必须 keyboard 可达**
没有例外。鼠标只是辅助。

**K-02 关键操作展示快捷键 hint**
所有可触发 action 在 UI 上以等宽 keycap 形式 inline 展示其 shortcut。Raycast 验证过，shortcut 不在 tooltip 里、不在文档里、不在菜单里翻才能看到。

**K-03 默认聚焦最高频字段**
任何窗口、Popover、Modal 打开时焦点必须落在最常用的字段上。Launcher 浮层打开 -> 焦点在搜索框；Workflow Builder 打开 -> 焦点在「步骤列表」第一项。

**K-04 输入实时响应（< 16ms / keystroke）**
搜索、表单、命令面板必须在用户敲下每个键的同一帧内更新过滤结果。

**K-05 不重写系统保留快捷键**
macOS 的 Cmd+W、Cmd+Q、Cmd+H、Cmd+,、Cmd+Tab，Windows 的 Alt+Tab、Alt+F4，Linux 桌面环境保留快捷键，全部不覆盖。详见 03 节。

## 02. 焦点环规范

**视觉**：

- 默认 2px `accent/base` 外边框
- 圆角随被聚焦元素的 `radius/*` 取相同档位
- 与元素本体之间留 1px 透明 gap（防止内描边混淆）
- 不使用 box-shadow 模拟（egui 渲染开销不必要）

**显示规则**：

- 任意时刻**有且只有一个**元素拥有焦点环
- 来源区分：
  - **键盘聚焦**（Tab / 箭头 / 程序聚焦）-> 显示焦点环
  - **鼠标点击**聚焦 -> 不显示焦点环（参考 macOS 行为：纯鼠标用户不需要被焦点环干扰）
  - **Touch / Pen** -> 一期不支持
- 实现：跟踪「最近一次焦点变更的来源」flag，仅在 keyboard 来源时绘制

**禁用**：

- 焦点环颜色覆盖 accent（如绿色、红色焦点环表示状态）
- 同时多个焦点环（确实需要多焦点的场景重新设计组件，或使用 selected 视觉而非焦点视觉）
- focus 时元素 scale / position 移动

## 03. 跨平台修饰键

**Primary 修饰键** = macOS `Cmd` / Windows / Linux `Ctrl`

**Alt** 在三个平台都叫 `Opt`（macOS）或 `Alt`（Win / Linux），文档统一写 `Alt`，UI 在 macOS 上渲染 `⌥`。

**Shift / Ctrl** 在三平台一致。

**显示符号映射**（UI 渲染时根据平台切换）：

| 含义 | macOS UI | Win / Linux UI | 文档内 |
| --- | --- | --- | --- |
| Primary | `⌘` | `Ctrl` | `Mod` |
| Shift | `⇧` | `Shift` | `Shift` |
| Alt / Option | `⌥` | `Alt` | `Alt` |
| Control（macOS 第二修饰） | `⌃` | `Ctrl-Alt` 不映射 | `Ctrl` (macOS only) |
| Enter | `↵` | `Enter` | `Enter` |
| Esc | `Esc` | `Esc` | `Esc` |
| Tab | `⇥` | `Tab` | `Tab` |
| Space | `␣` | `Space` | `Space` |
| 箭头 | `↑` `↓` `←` `→` | `↑` `↓` `←` `→` | `Up` 等 |
| Backspace | `⌫` | `Backspace` | `Backspace` |
| Delete | `⌦` | `Del` | `Delete` |

**禁止**：

- 在 macOS 上使用 `Ctrl` 作为 primary（与 macOS 习惯背离）
- 在 Win / Linux 上要求用户按 `Cmd`（不存在）

## 04. 全局热键

**默认热键**：

- macOS：`Opt+Space`（`Alt+Space`）
- Windows：`Alt+Space`
- Linux：`Alt+Space`（注意与 GNOME Terminal、KDE 系统菜单冲突，安装时检测并提示）

**冲突处理**：

- 注册失败时不静默失败：Launcher 启动时输出 `hotkey_registration FAIL`，并在 macOS menu bar / 系统托盘提示
- 提供 `std-launcher --hotkey-smoke <combo>` 用于非 GUI 注册验证
- 真实桌面验证必须使用 `STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke <combo>`
- 提供 `std doctor` 检测系统已占用快捷键并建议替代

**用户自定义**：

- Settings > Hotkeys 提供 Capture UI（按下要绑定的组合）
- 唯一一处全局热键，不提供「次要全局热键」
- 用户更改后立即热更新，无需重启

**反弹规则**：

- 唤起 Launcher 后再次按下相同热键 -> 关闭 Launcher（toggle）
- Esc 始终关闭 Launcher，不可禁用

**后台 UI 验收**：

- 默认测试禁止 AX、CGEvent、postToPid、System Events 和真实全局热键
- 真实焦点、Enter 打开、窗口 toggle、面板管理验收只能走显式 opt-in
- 推荐使用隔离 harness 后台验证，不操作用户正在使用的窗口
- harness 目标必须由固定 bundle id、pid、window id、window title 四重匹配确认
- 验收命令固定为 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title "std-cli Background UI Harness"`
- driver 顺序固定为 per-process event tap -> appKitDefined primer -> center primer -> postToPid 定向输入
- event tap 只订阅 raw value 13、19、20 三类 focus message，只拦截 previous app deactivation，target activation 必须放行
- previous app 永远不能作为输入目标；真实 App 名称不能作为 harness 选择条件
- 浮动光标只是状态可视化，不是输入机制
- 禁止向当前 frontmost app、Terminal、1Password、WeChat、系统设置或用户既有窗口投递事件
- 该后台路径不进入默认质量门禁，只能人工 opt-in 运行

## 05. Launcher 内快捷键

**导航**：

| 快捷键 | 行为 |
| --- | --- |
| `Up` / `Down` | 上下移动选中项 |
| `Mod+Up` / `Mod+Down` | 跳到列表顶 / 底 |
| `Tab` / `Shift+Tab` | 在「搜索框 -> 结果列表 -> Action Panel」三区切换 |
| `Enter` | 触发选中项的主 action |
| `Mod+Enter` | 触发选中项的次 action（保留扩展） |
| `Mod+K` | 打开 Action Panel（次级 action 集合） |
| `Esc` | 一级：清空 query 时关闭 Launcher，二级：清空 query 但不关闭，三级：返回上层 |
| `Mod+1` ... `Mod+9` | 直接触发结果列表第 1..9 项 |
| `Mod+Shift+Enter` | 触发并保持 Launcher 打开（pin 模式） |
| `Mod+Backspace` | 在 query 中删除上一个 token |

**Esc 行为分级**：

- 有 query 文本时：清空 query，焦点回搜索框
- 已经空 query 时：关闭 Launcher
- 在 Action Panel 内时：先关 Action Panel，再 Esc 才返回上一级
- 在二级页面（plugin 子视图）时：先返回上一级，再 Esc 才关 Launcher

**Action Panel（Mod+K）**：

- Raycast 风格的次级动作列表
- 内部本身是一个 keyboard-navigable list
- 自带搜索框，可继续 type 过滤
- Esc 返回主结果列表

## 06. Studio 内快捷键

**全局（Studio 主窗口）**：

| 快捷键 | 行为 |
| --- | --- |
| `Mod+N` | 新建 Workflow |
| `Mod+T` | 新建 tab（如适用） |
| `Mod+W` | 关闭当前 tab / 窗口（遵循平台习惯） |
| `Mod+S` | 保存当前文档 |
| `Mod+,` | 打开 Settings |
| `Mod+/` | 打开内嵌命令面板 |
| `Mod+P` | 快速跳转（Workflow / Skill / Memory / Plugin） |
| `Mod+Shift+P` | 跳转到 Command（与 VS Code 习惯一致） |
| `Mod+B` | 切换主 Sidebar 显示 |
| `Mod+J` | 切换底部 Panel（Batch Debug / Logs） |
| `Mod+0` | 重置 zoom |
| `Mod+=` / `Mod+-` | zoom in / out |
| `F1` | 上下文帮助 |

**Workflow Builder 内**：

| 快捷键 | 行为 |
| --- | --- |
| `Mod+Enter` | 测试执行当前 Workflow |
| `Mod+Shift+Enter` | 模拟执行（不写副作用） |
| `Mod+D` | 复制当前步骤 |
| `Mod+Backspace` | 删除当前步骤（需要 confirm） |
| `Alt+Up` / `Alt+Down` | 步骤上移 / 下移 |
| `Mod+Shift+H` | 显示版本历史 |

**Analysis Workbench 内**：

| 快捷键 | 行为 |
| --- | --- |
| `Mod+F` | 在当前实体内全文搜索 |
| `Mod+L` | 切换关系图 / 列表视图 |
| `Mod+I` | 切换 Inspector 显隐 |
| `?` | 自然语言问答 focus |

**禁止**：

- 抢占系统级 `Mod+Q` `Mod+H` `Mod+M` `Mod+Tab` `Mod+\``
- 抢占 macOS 文本编辑通用快捷键（`Mod+A` 全选、`Mod+C` 复制、`Mod+V` 粘贴、`Mod+Z` 撤销、`Mod+Shift+Z` 重做）
- 自定义剪贴板 paste 行为覆盖系统 paste

## 07. 命令面板（Mod+/ 或 Mod+Shift+P）

Studio 内嵌的次级 launcher，与 Launcher 浮层是不同实例。

- 触发：`Mod+/` 任何场景；`Mod+Shift+P` 命令专用
- 视觉：与 Launcher 同款 token，但宽度自适应当前 Studio 主窗口（最大 720px）
- 内容来源：Studio 内所有 Action + 当前打开文档的上下文 action
- 行为：与 Launcher 一致（详见 21）

**与 Launcher 的关系**：

- 命令面板是 Studio 内的小型 launcher，作用域是当前 Studio 实例
- 全局 Launcher 是系统级，作用域是整个 std-cli 数据
- 两者**不互相调用**，避免「打开 Launcher 然后再打开命令面板」的混乱

## 08. IME 输入

**M-01 IME 组合期间不触发任何 action**
中文 / 日文 / 韩文输入法 composing 状态下：

- `Enter` 由 IME 接管，不触发 Launcher action
- `Esc` 由 IME 接管，不关闭 Launcher
- 箭头键由 IME 接管，不切换选中项
- 只有 IME 提交（commit）后，后续按键才进入正常处理

**实现**：

- egui 提供 `Event::Ime` 路径，监听 `ImePreedit` 期间 set `ime_composing = true`
- 全局 key handler 收到 `Enter` / `Esc` / 箭头时，先检查 `ime_composing`
- smoke test 必须覆盖中文输入路径（断言 composing 期间 `Enter` 不触发）

**禁止**：

- 用 `event.text` 直接做命令解析（绕过 IME）
- 把 IME composing 字符当作 query 实时搜索（每次 IME 候选词变化都触发搜索 = 不必要 IO）

## 09. 全键盘流程 self-check

每个新 surface / window / popover 必须通过以下手测：

- [ ] 打开后焦点正确落在主要字段
- [ ] Tab / Shift+Tab 遍历顺序符合视觉左->右、上->下
- [ ] 所有可点击元素 Tab 可达
- [ ] Enter / Space 行为符合期望
- [ ] Esc 行为符合本文 05 节分级
- [ ] 系统保留快捷键未被劫持（在 macOS 上验证 `Mod+Q` `Mod+W` `Mod+H` 正常）
- [ ] IME 组合期间不会误触发
- [ ] Reduce Motion 下焦点环依然清晰可见

## 10. 实现要求

- `std-egui::input` 模块封装 `KeyBinding`，所有快捷键不在组件代码里硬编码字符串
- `std-egui::input::shortcut!(Mod+K)` 宏 / fn 自动处理跨平台映射
- 所有快捷键集中注册到 `std-core::shortcuts::Registry`，Settings UI 直接从 Registry 渲染列表
- 用户自定义覆盖默认时，Settings 显示 `default` / `user` 来源标记，可一键 reset

```rust
use std_egui::input::{shortcut, Mod, KeyBinding};

let binding = shortcut!(Mod+K);             // 跨平台
if ui.input(|i| binding.pressed(i)) {
    open_action_panel(ui);
}
```

## 11. 与其他文档的引用

- 焦点动画时长 / 曲线：详见 19_Motion_and_Interaction_Rhythm
- Launcher 完整快捷键交互链路：详见 21_Launcher_UX_Spec
- Studio 完整快捷键交互链路：详见 22_Studio_UX_Spec
- 系统级 Accessibility / Full Keyboard Access 联动：详见 23_Accessibility_and_Localization
- egui input 处理细节：详见 24_egui_Implementation_Constraints

---

**维护**：StringKe

**变更须知**：新增任何 shortcut 必须先确认不冲突 macOS / Windows / Linux 系统保留键，再更新本文件。
