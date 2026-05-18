# 08. Launcher Surface Detail — Launcher（cli 入口）详细设计

## 界面约束

- 极简、克制、高信息密度
- 类似 Spotlight / Raycast 的搜索框 + 结果列表
- 支持键盘全流程操作

## 核心功能

1. **统一搜索**
   - 本地 App / 文件
   - Workflow
   - 剪切板历史（支持语义）
   - Memory
   - 已注册 Action / Skill

2. **语音入口**
   - 全局热键按住 → 语音输入 → 自然语言转 Action / Workflow 执行
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

## 技术实现要点（egui 2026）

- 自定义 winit Window（无边框、透明、圆角）
- macOS vibrancy 使用 objc2-app-kit
- 搜索使用 nucleo + 自定义 scoring
- 结果渲染使用虚拟列表

---

Launcher 的唯一使命：**让用户在最短时间内把意图转化为行动**。