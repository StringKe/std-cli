# 14. Code Quality - Rustfmt、Clippy 与 Dylint

## 目标

代码结构必须支持长期快速迭代。质量规则优先使用 Rust 生态工具，不在 `std` CLI 里自研 Rust lint。

## Workspace 配置

- `rustfmt.toml`：统一格式规则
- `clippy.toml`：统一 Clippy 阈值和测试宽松规则
- `Cargo.toml` `[workspace.lints.clippy]`：统一 lint 等级
- `Cargo.toml` `[workspace.metadata.dylint]`：加载本地 Dylint 文件规模和视觉 token lint
- 每个 crate 的 `Cargo.toml` 通过 `[lints] workspace = true` 继承 workspace lint
- `crates/file_too_long`：本地 Dylint lint crate，不作为业务 workspace member 编译，包含 `file_too_long` 和 `no_inline_visual_values`

## Release 质量门禁

Release verify gate 使用 `mise` 作为任务入口，内部仍只调用 Rust 生态工具：

```bash
mise run quality
```

`mise run test` 会设置 `STD_TEST_MODE=1`。该模式下 release/debug binary 即使被测试间接调用，也会阻断 `open`、`osascript` 和其他 external runner。CLI、batch、workflow 的外部执行必须同时满足 `--allow-external` 和 `STD_ALLOW_DESKTOP_AUTOMATION=1`，只给其中一个条件一律返回 `NeedsExternalRunner`。Launcher 真实用户 Enter 使用独立入口打开 App / File，但该入口在 `STD_TEST_MODE=1` 下同样返回 `NeedsExternalRunner`。默认测试严禁唤起 Terminal、App、1Password、WeChat、menu bar resident、全局热键、AX 或 CGEvent 后台 UI 自动化。

测试代码禁止设置 `STD_ALLOW_DESKTOP_AUTOMATION`、`STD_ALLOW_UI_PREVIEW` 或 `STD_ALLOW_BACKGROUND_UI_AUTOMATION`。任何测试子进程必须设置 `STD_TEST_MODE=1`，并显式关闭这三个 opt-in 环境变量。

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

`clippy.toml` 负责函数行数、参数数量、认知复杂度和测试宽松规则。`crates/file_too_long` 通过 Dylint 强制 Rust 源文件不超过 500 行，并用 `no_inline_visual_values` 禁止业务 UI 代码直接调用 `Color32::from_rgb`、`Color32::from_rgba_*`、`Color32::from_black_alpha` 或 `Color32::from_white_alpha`。视觉颜色必须通过 `std-egui::tokens` 暴露，token palette 自身是唯一允许定义 Color32 构造的位置。`std-cli` 的 file limit gate 扫描配置文件，强制不超过 300 行。Markdown 文档不做行数限制。`deny.toml` 负责依赖安全、许可证和来源策略。`cargo machete` 负责未使用依赖。

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
STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher -- --ui-preview light defer 8000
STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview light panes 8000
```

未设置 `STD_ALLOW_UI_PREVIEW=1` 时，`--ui-preview` 返回 `SKIP`，不创建可见窗口。

后台 UI 自动化验收可以使用 macOS AX / CGEvent / postToPid 方案，但只能作为人工 UI 验收 runner。推荐模型是先装 per-process event tap，再发送 appKitDefined primer 和 center primer，然后只向目标 PID 投递点击或键盘事件。该方案只用于隔离测试窗口，禁止把用户已经打开的真实 App 当成验收目标。该 runner 必须同时满足：

- `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1`
- `STD_TEST_MODE` 未启用
- 目标进程限定为测试命令启动的隔离 harness，不复用用户已打开的真实窗口
- harness 必须有可验证的 bundle id、pid、window id 和 window title 白名单
- runner 必须用 pid 反查真实 bundle identifier，不能只信任命令行传入的 bundle id 字符串
- target identity 必须是固定 bundle id、pid、window id、window title 四重匹配，缺任一项直接 `SKIP` 或 `FAIL`
- 启动 harness 使用 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-harness.sh`，该脚本只创建 `dev.std-cli.background-ui-harness` 测试 app，并用 `open -g` 避免抢占前台
- 验收命令必须完整写作 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title "std-cli Background UI Harness"`
- `cargo run -p std-cli -- ui background-smoke` 必须收到 `--harness-pid`、`--window-id`、`--bundle-id dev.std-cli.background-ui-harness`、`--window-title "std-cli Background UI Harness"` 才能进入真实 driver
- 浮动光标不是输入机制，只能作为可视化状态；driver 不依赖系统鼠标位置
- driver 只能使用 `postToPid` 定向投递到 harness pid，不能使用全局 HID、System Events、前台点击或用户当前 frontmost app
- 激活前先安装 previous 和 target 两个 per-process event tap，然后再发 appKitDefined primer 和 center primer
- per-process event tap 只订阅 raw value 13、19、20 三类 focus message，只允许拦截 previous app deactivation，target activation 必须放行
- activation start 使用 `appKitDefined` subtype 1 `applicationActivated`，结束使用 subtype 2 `applicationDeactivated`
- center primer 只能投递到 harness window center，用于窗口激活，不触发用户行为
- CGEvent 必须写入 `windowUnderMouse`、`windowThatCanHandle` 和 field 51/58，事件路由必须保持 window id 与 harness 匹配
- previous app 永远不能作为输入目标；它只允许被安装 event tap，用来丢弃 deactivation focus message
- 不向用户当前 frontmost app、Terminal、1Password、WeChat、weixin、wechat、微信或系统设置发送事件
- 不用真实 App 名称、进程名或窗口标题作为 harness 选择条件。macOS App 名称和窗口标题存在多语言别名，WeChat、weixin、wechat、微信这类名称都不能作为允许条件。harness 只能来自固定 bundle id、pid、window id 和 window title 四重匹配
- 失败时返回 `SKIP` 或 `FAIL`，不能 fallback 到前台点击真实桌面

当前人工 runner 为 `scripts/background-ui-smoke.swift`，由 `cargo run -p std-cli -- ui background-smoke` 在通过全部 harness 白名单后调用 `/usr/bin/swift` 执行。脚本自身再次检查 `STD_ALLOW_BACKGROUND_UI_AUTOMATION=1` 和 `STD_TEST_MODE`，避免绕过 CLI 直接运行时触碰桌面。runner 使用 `CGEvent.tapCreateForPid` 创建 per-process event tap，tap mask 只包含 raw value 13、19、20 三类 focus message，使用 `NSEvent.otherEvent` 生成 `appKitDefined` activation primer，并只对传入的 harness pid/window id 调用 `postToPid`。

完整人工验收流程：

```bash
cargo build -p std-launcher
STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-harness.sh
STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title "std-cli Background UI Harness"
```

该路径不能进入 `mise run quality`、release smoke gate、默认质量门禁或默认测试。它只用于后续真实截图、键盘焦点、窗口或面板管理验收。

截图采集脚本同样必须显式 opt-in，未设置 `STD_ALLOW_UI_PREVIEW=1` 时直接返回 `SKIP`，不调用 macOS `screencapture`：

```bash
STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh std-launcher "std-cli Launcher" artifacts/ui/launcher-light-results.png
STD_ALLOW_UI_PREVIEW=1 scripts/capture-window.sh std-studio "std-cli Studio" artifacts/ui/studio-light-dashboard.png
```

成组截图使用 `scripts/capture-ui-matrix.sh`。该脚本同样默认 `SKIP`，并且在 `STD_TEST_MODE=1` 下拒绝运行，不能进入默认测试或质量门禁：

```bash
STD_ALLOW_UI_PREVIEW=1 scripts/capture-ui-matrix.sh artifacts/ui/manual-acceptance
```

## 拆分策略

Clippy 的 `too_many_lines` 管函数规模。Dylint 的 `file_too_long` 管 Rust 源文件规模，超过 500 行时质量门禁失败。新功能优先放在已有领域模块，没有合适模块时新增小模块。禁止为了绕过工具做无意义切片，拆分后的模块名必须表达业务边界。
