# 15. Terminal Automation - 终端自动化

`std` CLI 是脚本友好的自动化表面。它不拥有业务逻辑，只通过 `std-core` 的 Registry、Workflow、Memory、Index、Plugin 和事件能力工作。

## 单步命令

常用入口：

```bash
std search terminal
std preview terminal
std trigger terminal
std run smoke
std workflow check smoke
std index rebuild .
std files index .
std memory remember "Workflow rule" "Use std run for workflows" --tags workflow,cli
std plugin run plugin-hello-js
std doctor
```

默认安全边界：

- `std trigger <query>` 不会直接唤起外部 runner
- `std run <workflow>` 中的外部 runner step 默认返回 `NeedsExternalRunner`
- 需要真正触发外部 runner 时必须显式加 `--allow-external`

## 批量计划

`std batch <path>` 顺序执行 JSON 计划。每个 step 可以是 `action` 或 `workflow`。

```json
{
  "stop_on_error": true,
  "allow_external": false,
  "steps": [
    {
      "name": "rebuild",
      "kind": "action",
      "target": "index"
    },
    {
      "name": "smoke",
      "kind": "workflow",
      "target": "smoke"
    },
    {
      "name": "terminal",
      "kind": "action",
      "target": "terminal"
    }
  ]
}
```

运行：

```bash
std batch examples/batch/smoke.batch.json
std batch examples/batch/smoke.batch.json --stop-on-error
std batch examples/batch/smoke.batch.json --allow-external
```

输出是结构化 JSON：

- `status=Completed`：所有 step 完成
- `status=NeedsExternalRunner`：至少一个 step 被 defer，没有失败
- `status=Failed`：至少一个 step 失败
- `steps[].execution`：统一的 `ActionExecution` 结果
- `steps[].error`：解析、搜索或执行错误

`--stop-on-error` 会在第一个 `Failed` step 后停止。`NeedsExternalRunner` 不算错误，它是安全 defer 状态。

## 自动化约束

- batch 不引入新的执行通道，由 `std-orchestration::BatchExecutor` 复用 Registry action 和 Workflow executor
- batch 默认继承外部 runner defer 策略
- batch 结果适合被 shell、CI、本地脚本和 Studio 调试视图消费
- batch 文件应保持声明式，不写 shell 控制流
