# 23. Accessibility and Localization - 无障碍与本地化

本文件定义 std-cli 的无障碍（A11y）与本地化（i18n / l10n）规范。目标是让有视障、动作障碍、认知障碍的开发者依然能高效使用 std-cli，并让中英双语用户得到等价体验。本文件优先级 = 02_Design_Principles 同级，不可在「赶进度」时跳过。

## 01. 哲学

**A-01 A11y 不是可选项**
无障碍是产品质量的一部分。任何「先做主流程，A11y 后补」的提案都拒绝。

**A-02 内建 > 后期适配**
键盘可达、对比度、reduce motion 在组件设计阶段就必须考虑，不在最后做「无障碍 review」补救。

**A-03 系统级偏好优先**
用户已经在 OS 里告诉过系统「我要 reduce motion」「我要 high contrast」「我要 large text」，std-cli 必须先尊重，再提供自己的 override。

## 02. 标准与目标

- **键盘**：100% 可达。任何 mouse 可触发的操作必须有键盘等价。详见 20。
- **对比度**：所有文本对背景 ≥ WCAG 2.2 AA（正文 4.5:1，大文本 3:1）。`accent/base` 与 `bg/surface-0/1` 的对比度也满足。
- **焦点指示**：永远可见，不可被装饰覆盖。详见 20-02。
- **动效**：Reduce Motion 下降级。详见 19-07。
- **透明度**：尽管 std-cli 不依赖透明（详见 24-04），仍然在自定义渲染中尊重 Reduce Transparency。
- **字号**：尊重系统 large text 偏好；提供应用内 zoom（`Mod+=` / `Mod+-`）。
- **截图模式**：智能脱敏（详见 09），不在本文件展开。

## 03. 屏幕阅读器支持

egui 通过 `accesskit` 提供跨平台 A11y 接入（macOS VoiceOver、Windows Narrator、Linux Orca）。std-cli 必须确保：

- 每个 interactive widget 暴露：role（button / list / listitem / textbox / dialog）、name、value、state
- 装饰性图标设置 `aria-hidden` 等价（accesskit 的 `Role::Unknown` + 不暴露 name）
- 列表 / 表格在 accesskit 树中保留层级（list > listitem，table > row > cell）
- 选中 / 焦点 / 禁用状态准确传达
- Modal 打开时，背景视图标记为 `inert`，焦点 trap 在 modal 内

**Launcher 特殊处理**：

- 主面板打开时朗读：「Launcher, search field, {placeholder or current query}」
- 列表选中变化朗读：「{title}, {subtitle}, {N of M}, press Enter to {primary action}」
- Action Panel 打开朗读：「Actions for {selected item}, list of {count}」
- 执行中朗读：「Running {action title}」；完成朗读 toast 内容
- 「最近 / 建议」分组标题在朗读时作为 group label

**Studio 特殊处理**：

- Sidebar 树节点朗读：`{title}, group {n}, level {depth}, {N of M}`
- DnD：开始朗读「Picked up {item}, use Alt+Up Alt+Down to move」，放下朗读结果
- 长任务进度（Batch Debug）每 5% 更新一次朗读（避免 spam）

**禁止**：

- 仅靠颜色传达信息（status 必须配 icon + 文本）
- 仅靠位置传达信息（如「右上角红点」必须有 `notification badge with N unread` 朗读）
- aria-live 滥用：仅关键状态变化使用 `polite`，错误用 `assertive`

## 04. Reduce Motion / Reduce Transparency / Increase Contrast / Bold Text

**Reduce Motion**：

- 触发：系统设置（macOS `NSWorkspace accessibilityDisplayShouldReduceMotion`、Windows `SPI_GETCLIENTAREAANIMATION`、Linux 一期默认 false）+ std-cli Settings `appearance.reduce_motion`
- 行为：详见 19-07
- 强制：环境变量 `STD_REDUCE_MOTION=1`

**Reduce Transparency**：

- std-cli 不依赖系统级 vibrancy，但 Tooltip / Popover / Toast 的 elevation 阴影会模糊看起来「带透明感」
- 触发：系统设置（macOS `accessibilityDisplayShouldReduceTransparency`）+ std-cli Settings
- 行为：所有 `elev/2 elev/3` 阴影模糊半径 -> 4px（更硬边）；Toast / Tooltip 背景 alpha 由 0.96 -> 1.0

**Increase Contrast**：

- 触发：系统设置（macOS `accessibilityDisplayShouldIncreaseContrast`）
- 行为：自动切换到更高对比度 token 子集（详见 09-高对比度变体）
  - `fg/secondary` 接近 `fg/primary`
  - `stroke/divider` 加深一档到 `stroke/border`
  - selected 行底色对比度提升至 6:1
  - 焦点环宽度由 2px -> 3px

**Bold Text**：

- 触发：系统设置（macOS `NSWorkspace accessibilityDisplayShouldUseBoldText`）
- 行为：默认字重由 Regular (400) -> Medium (500)；Semibold 不动
- 不影响代码字体（JetBrains Mono 在 bold 下可读性下降）

**实现**：

- `std-egui::a11y::AccessibilityContext` 在每帧读取系统偏好
- 所有视觉 token 通过 `accessibility(ctx)` 函数动态返回值，而非静态 const

## 05. 高对比度（HC）变体

| Token | 默认 dark | HC dark | 默认 light | HC light |
| --- | --- | --- | --- | --- |
| `fg/secondary` | #B5BAC1 | #DCDFE3 | #4B5057 | #2A2D32 |
| `fg/tertiary` | #7A8089 | #B5BAC1 | #7A8089 | #4B5057 |
| `stroke/divider` | #262A30 | #34393F | #E3E6EA | #D0D4D9 |
| `accent/weak` | rgba(78,156,255,0.18) | rgba(78,156,255,0.32) | rgba(10,107,255,0.12) | rgba(10,107,255,0.22) |
| 焦点环宽度 | 2px | 3px | 2px | 3px |

## 06. 字号缩放

**应用内 zoom**：

- `Mod+=` / `Mod+-` / `Mod+0`
- 范围 0.85x .. 1.5x，步进 0.05
- 等比缩放：字号、间距、行高、icon、圆角、阴影
- 持久化到 Settings

**系统 large text**：

- macOS 没有 system-wide text scale（仅 Display 缩放）
- Windows / Linux 的 system text scale 会被 winit 自动报告给 egui
- 系统 scale 优先于应用内 zoom；两者乘积是最终 scale

## 07. 颜色辨识无障碍

- 不依赖红 / 绿区分（色盲友好）
- status 配色配 icon：success 用对勾、warning 用三角、danger 用八边形、info 用圆点
- 表格 / 图表中区分多类用「颜色 + 图案 + 标签」三重，不单靠颜色

## 08. 键盘可达 self-check

详见 20-09。任何无法通过键盘 self-check 的 PR 不允许合入。

## 09. 本地化

**一期支持语言**：

- `zh-CN`（默认）
- `en-US`

**字符串 ID 规则**：

- 所有面向用户的字符串走 i18n key，禁止 inline literal
- key 命名：`<surface>.<area>.<purpose>`，例如 `launcher.empty_state.no_match.title`
- 中英双语必须同时落地，禁止「先写英文，后补中文」或反之
- 短语优先于句子，避免长句子在窄宽度断行

**字符串文件**：

- 位置：`crates/std-egui/src/i18n/zh-CN.toml`、`en-US.toml`
- 格式：TOML，扁平 key
- 引用：`use std_egui::i18n::t;  t!("launcher.empty_state.no_match.title")`
- 编译期检查：缺失 key 编译失败（参考 `rust-i18n` crate 模式）

**日期 / 数字 / 时间**：

- 走 `chrono` + locale-aware format
- `zh-CN`：`2026年5月19日 14:23`
- `en-US`：`May 19, 2026 2:23 PM`
- 相对时间：`5 minutes ago` / `5 分钟前`，> 7 天显示绝对时间

**键盘符号本地化**：

- 详见 20-03 跨平台修饰键表
- 不翻译 `Cmd` `Ctrl` `Shift` `Alt`（开发者习惯英文 + 符号）

**RTL（右到左）**：

- 一期不支持
- 但布局代码使用 `start` / `end` 而非 `left` / `right`，为后续 RTL 留接口
- 不阻断 v1.0 release

**输入法（IME）**：

- 详见 20-08
- zh-CN 默认 IME 测试覆盖至少：拼音、五笔、Sogou
- en-US 默认 IME 测试覆盖：标准键盘 + dvorak

## 10. 文案规范（双语共享）

- **简洁**：标题 < 6 词 / 中文 < 12 字；正文 < 3 行
- **动词开头**：button 文案用动词（`Save` / `保存`），不用名词
- **不威胁**：错误提示用「无法 / Unable to」，避免「失败 / Failed」一类负面词作为标题
- **避免缩写**：除非是行业通用（PR、CI、CLI）
- **避免感叹号**
- **数字 ≥ 10 用阿拉伯数字**，< 10 中文场景用中文「三个步骤」，英文用 `three steps`（参考 Apple Style Guide）
- **不使用 emoji**（与 HARD RULES 一致）

**Launcher 文案样本**：

| 场景 | zh-CN | en-US |
| --- | --- | --- |
| placeholder | 搜索 Workflow、应用、剪切板... | Search Workflows, apps, clipboard... |
| 空结果 | 没有匹配项 | No matches |
| 空结果副 | 换个关键词，或按 ? 询问 | Try a different keyword or press ? to ask |
| 执行中 | 正在运行 {action} | Running {action} |
| 错误 | 无法执行 {action} | Unable to run {action} |

## 11. 截图与脱敏

- Studio 提供「Capture Screenshot」action，可选「Pixelate sensitive areas」
- 脱敏规则：自动模糊 `Memory` 面板个人内容、`.env` 类文件路径
- 用户也可手动框选区域脱敏
- 不上传到任何远程服务（详见 12_Configuration_and_Storage）

## 12. 验收清单（任何 PR 必查）

- [ ] 所有 UI 字符串走 i18n key，zh-CN 和 en-US 同步
- [ ] 屏幕阅读器在 VoiceOver / Narrator 下能完整读出新组件
- [ ] 键盘 self-check（20-09）通过
- [ ] Reduce Motion 下行为符合 19-07
- [ ] Reduce Transparency 下视觉降级正常
- [ ] Increase Contrast 下 HC token 自动生效
- [ ] WCAG 2.2 AA 对比度自动断言通过（基于 token 计算）
- [ ] Bold Text 下不破坏布局
- [ ] App-zoom 0.85x .. 1.5x 不破坏布局
- [ ] 中英文长度差异下不出现 overflow（中文短、英文长是常见 trap）

## 13. 实现要求

- `std-egui::a11y` 模块封装系统偏好读取（accesskit + 平台 API）
- `std-egui::i18n` 模块封装 t! 宏 + locale-aware 格式化
- CI 增加 `cargo run -p std-egui --example a11y-audit` 静态检查：扫描代码中是否存在硬编码字符串
- `std-launcher --smoke` 和 `std-studio --smoke` 必须能在 `STD_LOCALE=en-US` 和 `STD_REDUCE_MOTION=1` 下正常运行并产出 evidence

---

**维护**：StringKe

**关联文档**：18 视觉（HC 变体来源）、19 动效（Reduce Motion）、20 键盘（A11y 主路径）、24 egui A11y 实现细节
