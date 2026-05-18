# 12. Configuration and Storage — 配置与存储

## 配置发现顺序（与 claudex 保持一致）

1. `$STDCLI_CONFIG` 环境变量指定文件
2. 当前目录及向上查找 `std-cli.toml` / `std-cli.yaml`
3. `~/.config/std-cli/config.toml`（全局推荐）
4. `~/.std-cli/config.toml`

支持 TOML 和 YAML。

## 关键存储路径

- `~/.std-cli/workflows/` — Workflow 定义（单一真相源）
- `~/.std-cli/index/` — 个人索引数据
- `~/.std-cli/memory/` — 长期记忆
- `~/.std-cli/history/` — 执行历史与审计日志
- `~/.std-cli/mise.toml` — 工具链隔离配置（Mise scope）

## Mise 集成

Workflow 执行和代码分析时，优先使用 `mise` 管理的隔离环境，保证可复现性。

## 敏感信息

- API Key、OAuth Token 使用系统 keyring 或加密存储
- 绝不把敏感信息写入普通配置文件

---

**配置系统要做到“简单、发现性强、与现有工具链（mise、std-ai）协同良好”。**