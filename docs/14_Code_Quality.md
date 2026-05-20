# 14. Code Quality - Rustfmt、Clippy 与 Dylint

## 目标

代码结构必须支持长期快速迭代。质量规则优先使用 Rust 生态工具，不在 `std` CLI 里自研 Rust lint。

## Workspace 配置

- `rustfmt.toml`：统一格式规则
- `clippy.toml`：统一 Clippy 阈值和测试宽松规则
- `Cargo.toml` `[workspace.lints.clippy]`：统一 lint 等级
- `Cargo.toml` `[workspace.metadata.dylint]`：加载本地 Dylint 文件规模 lint
- 每个 crate 的 `Cargo.toml` 通过 `[lints] workspace = true` 继承 workspace lint
- `crates/file_too_long`：本地 Dylint lint crate，不作为业务 workspace member 编译

## Release 质量门禁

Release verify gate 使用 `mise` 作为任务入口，内部仍只调用 Rust 生态工具：

```bash
mise run quality
```

`mise run test` 会设置 `STD_TEST_MODE=1`。该模式下 release/debug binary 即使被测试间接调用，也会阻断 `open`、`osascript` 和其他 external runner。外部 runner 还必须同时满足 `--allow-external` 和 `STD_ALLOW_DESKTOP_AUTOMATION=1`，只给其中一个条件一律返回 `NeedsExternalRunner`。默认测试严禁唤起 Terminal、App、1Password、WeChat、menu bar resident 或全局热键。

`quality` 任务展开为：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
DYLINT_RUSTFLAGS="-D warnings" cargo dylint --workspace --all -- --all-targets
cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml
cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib
cargo test --workspace -- --test-threads=1
cargo deny check
cargo machete
```

`clippy.toml` 负责函数行数、参数数量、认知复杂度和测试宽松规则。`crates/file_too_long` 通过 Dylint 强制 Rust 源文件不超过 500 行。`std-cli` 的 file limit gate 扫描配置文件，强制不超过 300 行。Markdown 文档不做行数限制。`deny.toml` 负责依赖安全、许可证和来源策略。`cargo machete` 负责未使用依赖。

Release 包还必须附带 v1.0 表面 smoke 证据：

```bash
std doctor
std-launcher --smoke "rebuild index"
std-launcher --window-smoke
std-launcher --theme-smoke
std-launcher --surface-smoke
std-launcher --ui-semantics-smoke index
std-launcher --keyboard-smoke index
std-launcher --preview-smoke
std-studio --smoke
std-studio --workspace-policy-smoke
std-studio --theme-smoke
std-studio --preview-smoke
std workflow trace --limit 5
std index coverage
std plugin check examples/plugins/hello-js
```

真实桌面热键验证属于人工桌面验收，必须单独显式 opt-in，不能进入默认 `mise run quality` 或 release smoke gate：

```bash
STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space 5000
```

未设置 `STD_ALLOW_DESKTOP_AUTOMATION=1` 时，`--gui-hotkey-smoke` 返回 `SKIP`，不创建窗口、不注册全局热键、不发送 System Events。

Launcher 和 Studio 截图预览同样属于人工 UI 验收。`std-launcher --preview-smoke` 与 `std-studio --preview-smoke` 只输出状态矩阵和待执行命令，不创建窗口。真正打开可见预览窗口必须显式设置：

```bash
STD_ALLOW_UI_PREVIEW=1 std-launcher --ui-preview light defer 8000
STD_ALLOW_UI_PREVIEW=1 std-studio --ui-preview light panes 8000
```

未设置 `STD_ALLOW_UI_PREVIEW=1` 时，`--ui-preview` 返回 `SKIP`，不创建可见窗口。

## 拆分策略

Clippy 的 `too_many_lines` 管函数规模。Dylint 的 `file_too_long` 管 Rust 源文件规模，超过 500 行时质量门禁失败。新功能优先放在已有领域模块，没有合适模块时新增小模块。禁止为了绕过工具做无意义切片，拆分后的模块名必须表达业务边界。
