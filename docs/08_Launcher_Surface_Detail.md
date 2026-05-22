# 08. Launcher Surface Detail - Launcher（cli 入口）详细设计

## 界面约束

- 极简、克制、高信息密度
- 类似 Spotlight / Raycast 的搜索框 + 结果列表
- 支持键盘全流程操作

## 核心功能

1. **统一搜索**
   - 本地 App / 文件
   - Workflow
   - 剪切板历史（结构化搜索）
   - Memory
   - 已注册 Action / Skill

2. **语音入口**
   - 全局热键按住 -> 语音输入 -> 自然语言转 Action / Workflow 执行
   - 支持 filler word 清理（类似 Wispr Flow）

3. **快速预览与执行**
   - 结果支持 Action Panel（类似 Raycast）
   - 常见操作一键执行

4. **轻量结果展示**
   - 执行中显示进度
   - 结果用 toast 或小面板展示，不抢占焦点

## 性能要求

- 热键唤起 < 80ms
- 搜索输入实时响应（< 16ms per keystroke）
- 后台索引更新不影响交互
- Launcher 状态层输出 `LauncherPerformanceReport`，包含 search / preview / trigger / hotkey 预算和最近一次实测耗时
- UI 底部显示 `launcher_perf PASS/FAIL`，测试会断言搜索、预览、触发路径满足预算

## 技术实现要点

- `std-launcher` 使用 egui / eframe 渲染 Launcher 面板
- `GlobalHotkeyRuntime` 使用 `global-hotkey` 注册和匹配热键事件
- `std-launcher --hotkey-smoke Alt+Space` 只注册并释放全局热键，输出注册耗时和 PASS/FAIL，不打开窗口
- `std-launcher --window-smoke` 验证隐藏窗口发送 `Visible(false)`，热键唤起发送 `Visible(true)` 和 `Focus`
- `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance` 是真实焦点、Enter 打开、窗口 toggle 的首选人工验收：启动固定 bundle id 的隔离 harness，用 per-process event tap、appKitDefined primer、center primer 和 postToPid 定向输入验证后台交互
- `STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space 5000` 只保留为人工安装包热键补充验收
- 未设置 `STD_ALLOW_DESKTOP_AUTOMATION=1` 时，GUI hotkey smoke 返回 `SKIP`，不触碰当前桌面
- 搜索使用 `std-core` Registry scoring
- 结果渲染使用稳定尺寸列表，避免搜索、预览、触发时抖动

---

Launcher 的唯一使命：**让用户在最短时间内把意图转化为行动**。
