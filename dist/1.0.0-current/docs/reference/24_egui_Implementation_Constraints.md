# 24. egui Implementation Constraints - egui 落地约束

本文件是 18-23 视觉、动效、键盘、UX、A11y 文档的「工程对应面」。egui 是 immediate mode GUI，与 Apple SwiftUI / AppKit 在 state 模型、布局、绘制、动画上差异巨大。本文件列出 egui 在 std-cli 场景下的能力边界、推荐做法、明确禁止的反模式。

## 01. egui 是什么，不是什么

**是**：

- Immediate mode：每帧重建 UI 树，没有持久 widget 对象
- Painter 接口：直接驱动 GPU primitive（mesh + texture）
- 跨平台：通过 winit + glow / wgpu 在 macOS / Win / Linux 一致
- 已落地版本：egui 0.33.2（详见 Cargo.toml workspace）

**不是**：

- 不是 retained mode，没有 view 生命周期可以 attach 长 effect
- 不是 native，不会自动拿到 NSVisualEffectView / Acrylic 一类系统材质
- 不是布局引擎丰富的工具：layout 受限于 horizontal / vertical / grid，复杂 layout 需自计算
- 不是无成本：每帧逻辑跑全树，需要避免在 update 内做重 IO / 重计算

## 02. State 管理边界

**M-01 UI state 留在 egui，业务 state 留在 std-core**

- 滚动位置、当前 selected index、动画 in-flight progress -> egui `Memory` 或本地 struct
- 当前 Workflow 列表、Memory 内容、配置 -> std-core，通过事件订阅同步

**M-02 跨帧持久状态用 `Id` + `Memory`**

```rust
let id = egui::Id::new("launcher.results.selected");
let mut idx: usize = ctx.data(|d| d.get_temp(id).unwrap_or(0));
// ... user input mutates idx ...
ctx.data_mut(|d| d.insert_temp(id, idx));
```

**禁止**：

- 把业务大对象（如全部 Workflow）拷贝进 egui Memory
- 在 update 内 lock std-core 全局 mutex（应通过事件队列异步同步）
- 用 `static mut` 或 `lazy_static` 存 UI state

## 03. 布局策略

egui 提供的核心布局：

- `horizontal` / `vertical`
- `grid::Grid`
- `Layout::with_main_align` / `with_cross_align`
- `Window` / `Area`（用于 Tooltip / Popover）
- `Viewport`（用于 detached window，已在 Studio 使用）

**std-cli 落地的布局规则**：

- 主窗口分区（Sidebar / Canvas / Inspector / Bottom Panel）使用 `SidePanel` + `TopBottomPanel` + `CentralPanel`，不要自己算
- 列表使用 `ScrollArea` + `show_rows` virtual 渲染（≥ 100 行时强制）
- 表格使用 `egui_extras::Table`
- 自定义复杂 layout 必须包装为可复用 widget 放到 `std-egui::widgets`

**禁止**：

- 任意 widget 在 update 中 measure 自身再决定位置（egui 是 immediate mode，下一帧才知道实际尺寸）
- 用 hover-only 显示的元素承担键盘可达任务（详见 20-K-01）

## 04. 模拟 Apple Material / Vibrancy 的策略

Apple 的 Liquid Glass、Material（ultraThin / thin / regular / thick / ultraThick）依赖 OS 级 backdrop blur + 内容色穿透。egui 没有等价能力且自实现成本高，**std-cli 不复刻**。

**替代策略**：

| Apple 需求 | std-cli 做法 |
| --- | --- |
| `regularMaterial` 背景（Sidebar、Tooltip） | 纯色 `bg/surface-1`，不做模糊 |
| `thickMaterial`（Modal） | 纯色 `bg/surface-0` + `elev/3` |
| `ultraThinMaterial`（覆盖在内容上的薄层） | 纯色 `bg/surface-2` 不透明，配 `stroke/divider` |
| Vibrancy（动态色彩穿透） | 不实现 |
| Lensing edges（玻璃边缘折射） | 不实现 |

**理由**：

- backdrop blur 需要把窗口下方屏幕内容采样到 texture，跨平台一致实现复杂
- Liquid Glass 折射 / lensing 是 metal shader 级别效果，egui 默认 painter 不开放
- 不引入「视觉差异化但跨平台不一致」的特性
- 用更高对比的纯色 + 明确层级 + 阴影代替

**保留**：

- 整窗 `Color32::TRANSPARENT` 设置（Launcher 浮层圆角外部为透明）以保持非矩形外观
- `egui_extras` 一类已有 lib，不自造

## 05. 动画落地

详见 19 全表。实现层细则：

**使用 ctx.animate_***：

```rust
let hovered = response.hovered();
let t = ctx.animate_bool_with_easing(
    response.id,
    hovered,
    egui::emath::easing::cubic_out,  // 0.2, 0, 0, 1 对应曲线
);
let bg = lerp_color(bg_idle, bg_hover, t);
```

**自定义曲线封装**：

```rust
// crates/std-egui/src/motion.rs
pub mod curves {
    use egui::CubicBezier;
    pub const OUT_STANDARD: CubicBezier = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
    pub const IN_STANDARD:  CubicBezier = CubicBezier::new(0.4, 0.0, 1.0, 1.0);
    pub const IN_OUT:       CubicBezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
    pub const SNAPPY:       CubicBezier = CubicBezier::new(0.18, 1.0, 0.22, 1.0);
}
```

**关键 API**：

- `Context::animate_bool_with_easing(id, target, ease)` -> 0..1
- `Context::animate_value_with_time(id, target_value, duration_seconds)` -> 当前插值
- `Context::request_repaint_after(remaining)` 精确 schedule 下一帧（不要 `request_repaint` 永久 spin）

**禁止**：

- 在 update 内自己启 `std::thread::spawn` 跑动画
- 用 `std::time::Instant` 自己算 delta（egui 提供 `ctx.input(|i| i.time)`）
- 在动画期间持续 `request_repaint()`（应使用 `request_repaint_after`）

**Spring overshoot 实现示例**（19-04 toast 唯一允许场景）：

```rust
// 第一段 0..1.06，使用 SNAPPY
let t1 = ctx.animate_bool_with_easing(stage1_id, true, curves::SNAPPY);
let s1 = lerp(0.0, 1.06, t1);

// 第二段达到 1.06 后开始 1.06..1.0，使用 OUT_STANDARD
if t1 >= 1.0 {
    let t2 = ctx.animate_bool_with_easing(stage2_id, true, curves::OUT_STANDARD);
    return lerp(1.06, 1.0, t2);
}
return s1;
```

## 06. 渲染性能

**预算**（与 19-08 一致）：

- 每帧 layout + paint ≤ 4ms（95p）
- 同时 in-flight 动画 ≤ 8

**最佳实践**：

- ScrollArea 必须用 `show_rows(ui, row_height, total_rows, |ui, row_range| ...)` 而非渲染整个列表
- 计算昂贵的字符串高亮 / Markdown 渲染使用 `egui::cache::FrameCache`
- icon 渲染：preload 到 `egui::TextureHandle`，不每帧 from_rgba
- 大表格：`egui_extras::Table` + `striped(true)` + 固定 column 宽度
- 不在 update 内做：磁盘 IO、网络、JSON parse、SQL query

**热点诊断**：

- `egui::debug::PaintStats`
- macOS Instruments / Xcode CPU profiler
- 每个 release 必须 attach 一次 `std-studio --smoke` 期间的 `frame_time_p95` 数据

## 07. 自定义 widget 规范

**位置**：所有可复用 widget 在 `crates/std-egui/src/widgets/`

**契约**：

- 单文件 ≤ 500 行（详见 14 file_too_long lint）
- 接受 `&mut Ui`，返回 `Response`
- 内部不持有可变全局状态
- 接受 token 而不是字面值（颜色 / 字号 / 间距）

**通用骨架**：

```rust
pub fn list_row(ui: &mut Ui, item: &ResultItem, selected: bool) -> Response {
    let height = Space::ROW_HEIGHT;
    let (rect, resp) = ui.allocate_at_least(vec2(ui.available_width(), height), Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        let bg = if selected {
            Color::accent_weak(ui.ctx())
        } else if resp.hovered() {
            Color::bg_surface_2(ui.ctx())
        } else {
            Color::TRANSPARENT
        };
        painter.rect_filled(rect, Radius::MD, bg);
        // ... draw icon, title, subtitle, shortcut
    }
    resp
}
```

**禁止**：

- widget 内部直接读写 std-core 状态
- widget 内 spawn tokio task
- widget 内做 unwrap / expect（应返回 Result 或安全 fallback）

## 08. 多 workspace window

- 已落地：Studio 主路径使用 egui 内部 dock/pane 渲染 workspace window
- 每个 workspace window 是 Studio 内部状态对象，不使用原生子窗口 chrome
- 跨窗口同步通过 std-core 事件总线
- workspace window 之间不直接互相调用 UI API

**closeguard**：

- 主窗口关闭时先记录 pending state，再关闭当前实例内 workspace windows
- 详见 11_Event_Protocol

## 09. Accessibility 接入

egui 集成 `accesskit`：

- 每个 widget 在创建时通过 `Ui::response.id` 注册 a11y node
- 标准 widget（`Label` `Button` `TextEdit`）已自带 role
- 自定义 widget 必须显式声明：

```rust
let response = ui.allocate_response(size, Sense::click());
response.widget_info(|| WidgetInfo::labeled(
    WidgetType::Button,
    ui.is_enabled(),
    "Launch Workflow",
));
```

**额外要求**：

- 列表 / 表格使用 `accesskit::Role::List` `Row` `Cell`，确保 VoiceOver 能朗读「N of M」
- 焦点变化时主动告知 a11y：`ctx.memory_mut(|m| m.request_focus(id))`
- Modal 打开时 set inert flag 在外部树

**当前 egui 限制**：

- DnD 的 a11y 提示需要自实现（egui 0.33 没有内置 announce）
- 关系图 / Canvas 一类自定义图形需自己暴露语义节点

## 10. 输入与 IME

- 全局 key handler 通过 `Context::input(|i| i.events.iter()...)` 读取
- IME 通过 `Event::Ime` 系列：`ImePreedit` `ImeCommit` `ImeCancel`
- composing 状态判定（详见 20-08）：

```rust
let composing = ctx.input(|i| {
    i.events.iter().any(|e| matches!(e, Event::Ime(ImeEvent::Preedit(_))))
});
```

- 输入框使用 `TextEdit::singleline()` 默认已处理 IME；自定义键盘逻辑必须显式判断 composing

## 11. 拖拽（DnD）

- egui 0.33 的 DnD 通过 `egui::DragValue` 或自实现 `Sense::drag()` + 状态机
- std-cli 在 `std-egui::widgets::dnd` 提供统一封装
- 必须配键盘等价（详见 20、22-12）

## 12. 错误处理与 panic 安全

- update 内禁止 panic：任何 unwrap 在 release build 都视为缺陷
- widget 在拿不到 token / state 时 fallback 到 safe default 并打日志，不 panic
- panic catch：eframe 顶层 `set_panic_hook` 写入 audit log，弹出 recovery dialog
- 测试覆盖：smoke 路径不允许 panic 出现

## 13. 与 Code Quality 文档的对齐

- 视觉值（颜色、字号、间距）的硬编码检测：在 `crates/file_too_long` 同目录新增 lint `no_inline_visual_values`（dylint），grep 形式扫描 `Color32::from_rgb`、`vec2(*.*, *.*)` 含非 grid 数字等模式
- 任何新自定义 widget 必须配 unit test（最小 1 个 happy path + 1 个 disabled state）
- 详见 14_Code_Quality

## 14. 未来扩展点

不在 v1.0 范围，但保留接口：

- wgpu backend 切换（当前 glow）：为后续启用更复杂的 shader-based 效果
- Custom Painter 扩展：为 Relations 图、Memory timeline 等场景预留
- 平台原生菜单（macOS NSMenu / Win Menu）的混入：通过 `eframe::NativeOptions::with_menubar`
- Accessibility tree 的更深节点：等 accesskit 在 egui 中完善

## 15. 验收清单

- [ ] 不在组件代码中硬编码视觉值
- [ ] 不在 update 内 IO / 阻塞
- [ ] 动画使用 ctx.animate_* 系列
- [ ] 自定义 widget 暴露 `WidgetInfo`
- [ ] 列表 ≥ 100 行使用 virtual render
- [ ] Reduce Motion 路径已 smoke 验证
- [ ] 单文件 ≤ 500 行
- [ ] 视觉值硬编码 lint 通过
**关联文档**：14 Code Quality、18 token、19 motion、20 input、23 A11y、09 Studio、08 Launcher
