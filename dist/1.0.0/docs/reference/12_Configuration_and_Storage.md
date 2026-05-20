# 12. Configuration and Storage - 配置与存储

## 配置优先级（从高到低，与 claudex 保持一致）

1. `$STDCLI_CONFIG` 环境变量指定文件
2. 当前目录及向上查找 `std-cli.toml` / `std-cli.json` / `std-cli.yaml`，近目录覆盖父目录
3. `~/.config/std-cli/config.toml` / `~/.config/std-cli/config.yaml`（全局推荐）
4. `~/.std-cli/config.toml` / `~/.std-cli/config.yaml`
5. 内置默认值

字段级环境变量优先级最高，支持：

- `STDCLI_LAUNCHER_HOTKEY` / `STD_LAUNCHER_HOTKEY`
- `STDCLI_DATA_DIR` / `STD_DATA_DIR`
- `STDCLI_ENABLE_AI` / `STD_ENABLE_AI`
- `STDCLI_THEME` / `STD_THEME`

支持 TOML、JSON 和 YAML。

## 关键存储路径

- `~/.std-cli/workflows/` - Workflow 定义（单一真相源）
- `~/.std-cli/index/` - 个人索引数据
- `~/.std-cli/memory/` - 长期记忆
- `~/.std-cli/history/` - 执行历史与审计日志

## 工具链边界

当前 v1.0 不直接管理 `mise` 环境。Workflow、插件和外部命令都通过显式 Action、权限和 `allow_external` 边界运行。

## 敏感信息

- API Key、OAuth Token 使用系统 keyring 或加密存储
- 绝不把敏感信息写入普通配置文件

---

**配置系统要做到「简单、发现性强、与现有工具链协同良好」。**
