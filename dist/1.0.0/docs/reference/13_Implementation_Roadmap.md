# 13. Implementation Roadmap - 实现路线图

## 当前状态

当前实现进入 v1.0 convergence。目标不是继续扩大概念范围，而是把已经落地的 Launcher、Studio、Terminal、Core、Index、Plugin、Release 和质量门禁收敛成可安装、可验证、可发布的软件。

## 已落地能力

### Core

- `std-core` 是 GUI 中立业务中心
- Registry、Action、Skill、Command、Memory、Plugin、Tool、Event、Storage、Config 都从 core 暴露
- Launcher、Studio、Terminal 共享同一套 core 行为，不各自实现业务分支
- 外部 runner 默认 defer，显式 opt-in 才执行

### Terminal

- `std config`
- `std run`
- `std batch`
- `std workflow new/list/check/history/trace/step`
- `std index rebuild/list/show/inspect/coverage/search/ask`
- `std files index/search`
- `std plugin check/list/search/run`
- `std memory`
- `std skill`
- `std command`
- `std app`
- `std release plan/package/verify`
- `std install plan/run/verify`
- `std doctor`

### Launcher

- 热键注册计划和运行时封装
- 搜索、预览、键盘移动、触发反馈
- App、文件、Workflow、Clipboard、Memory、Action 搜索
- 语音 transcript 输入和清洗
- `std-launcher --smoke "rebuild index"` 输出 smoke/perf 证据

### Studio

- Dashboard、Workflow、Apps、Memory、Plugins、Analysis、History、Settings pane
- 多窗口打开、聚焦、关闭、重复窗口去重
- Workflow 创建、编辑、模拟、执行、trace 查看
- Batch Debug 复用 `std-orchestration::BatchExecutor`
- Plugin Manager 读取 manifest check report
- Analysis Workbench 复用 `std-index`
- Memory Browser 复用 core storage

### Index

- Entity Overview
- Component Digest
- Symbol / Relation Index
- Historical Context
- Coverage、Inspect、Search、Ask
- Studio 和 Terminal 共用同一套 index storage

### Plugin

- `deno_core` JavaScript / TypeScript runtime
- scoped fs、network、clipboard、code permission
- manifest check 独立于 action 执行
- shell tool timeout 和 JS infinite loop timeout

### Release 和质量

- macOS app bundle packaging
- release manifest
- checksum verify
- install plan/run/verify
- packaged docs/examples/quality evidence
- rustfmt、Clippy、Dylint、cargo-deny、cargo-machete
- Rust 源文件 500 行 Dylint 硬门槛
- 配置文件 300 行 doctor 硬门槛

## 剩余收敛项

- 继续做 requirement-by-requirement completion audit，不用局部测试替代完整证明
- 让 docs 中所有设计声明和当前代码能力一致
- 对接真实 release build 产物执行 `std release package` 和 `std release verify`
- 在安装目录上执行真实 `std install run` 和 `std install verify`
- 在安装后的二进制上执行 smoke evidence：
  - `std doctor`
  - `std-launcher --smoke "rebuild index"`
  - `std workflow trace --limit 5`
  - `std index coverage`
  - `std plugin check examples/plugins/hello-js`
- 本机或 CI 环境安装 `cargo-deny` 后执行 `cargo deny check`

## v1.0 完成门槛

v1.0 只能在以下证据全部 PASS 后宣布完成：

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `DYLINT_RUSTFLAGS="-D warnings" cargo dylint --workspace --all -- --all-targets`
- `cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml`
- `cargo test --workspace -- --test-threads=1`
- `cargo deny check`
- `cargo machete`
- `std doctor`
- `std release package --version 1.0.0`
- `std release verify --dist <dist>`
- `std install run --prefix <prefix> --from <dist>/bin`
- `std install verify --prefix <prefix>`
- installed `std`, `std-launcher`, `std-studio` smoke evidence

## 维护原则

- 不恢复 `std quality` 自研命令
- Rust 质量管理优先使用 Rust 生态工具
- 新源码文件低于 500 行
- 配置文件低于 300 行
- 测试和默认命令不得唤起 Terminal、App 或外部 runner
- 外部行为必须显式 `--allow-external` 或同等 opt-in

**维护者**：StringKe
