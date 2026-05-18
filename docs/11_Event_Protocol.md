# 11. Event Protocol — 事件协议

## 目的

让 Core 和多个表面之间实现松耦合通信，同时支持审计和调试。

## 事件分类

- **Orchestration Events**：Workflow 开始、步骤完成、失败、需要人工确认等
- **Index Events**：实体索引完成、更新
- **UI Events**：用户在 Launcher/Studio 中的操作（跨表面同步）
- **AI Events**：Planner 产生计划、工具调用、结果返回

## 实现方式

- Core 内部使用 tokio broadcast 或 flume channel
- 表面通过 `EventBus` trait 订阅感兴趣的事件
- 所有事件都可序列化，便于未来支持远程调试或日志持久化

## 设计原则

- 事件是通知，不是命令（避免循环依赖）
- 关键操作必须有对应的 Command（通过 Registry 调用），事件仅用于状态同步和 UI 更新

---

良好的事件协议是多表面架构能够长期演进的基础。