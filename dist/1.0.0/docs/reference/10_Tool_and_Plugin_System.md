# 10. Tool and Plugin System - 工具与插件系统

## 当前实现

`std-core` 是工具和插件的唯一业务中心。插件 manifest 会被加载为 `PluginTool`，再注册成 `ActionRegistry` 中的 `Command` action。CLI、Launcher、Studio 都通过同一套 Registry 搜索和执行插件能力。

已实现两类插件 action：

- `shell`：通过 `sh -c` 执行命令，必须声明 `shell` 权限
- `javascript` / `typescript`：通过内嵌 `deno_core` 执行脚本，必须声明 `code` 权限

所有插件 action 支持 `timeout_ms`。超时后 host 会终止命令或 V8 isolate，并返回 `timed_out=true`。

## Manifest

插件目录必须包含 `plugin.json`。`std-core` 会扫描 `$STDCLI_DATA_DIR/plugins/*/plugin.json`。

```json
{
  "name": "hello-js",
  "description": "Minimal deno_core JavaScript plugin example",
  "version": "0.1.0",
  "permissions": ["code"],
  "actions": [
    {
      "name": "Plugin Hello JS",
      "description": "Run a minimal JavaScript plugin",
      "when_to_use": "When validating std-cli JavaScript plugin execution",
      "kind": "javascript",
      "script": "main.js",
      "timeout_ms": 1000,
      "tags": ["plugin-hello-js", "example"]
    }
  ]
}
```

字段说明：

- `name`：插件名，会作为 Registry tag
- `description`：插件说明
- `version`：插件版本
- `permissions`：插件权限列表
- `fs_scopes`：相对插件目录或绝对路径的文件访问范围
- `network_hosts`：允许访问的 `host:port`
- `actions`：插件暴露的 action 列表
- `actions[].kind`：`shell`、`javascript` 或 `typescript`
- `actions[].command`：shell action 的命令
- `actions[].script`：code action 的脚本路径，相对插件目录
- `actions[].timeout_ms`：执行超时
- `actions[].tags`：搜索标签

## 权限

权限是显式白名单：

- `shell`：允许 shell action 执行 `command`
- `code`：允许 JavaScript / TypeScript action 运行脚本
- `fs_scoped`：允许脚本调用 scoped file API
- `network`：允许脚本调用 HTTP API
- `clipboard`：允许脚本读取 host 提供的 clipboard history
- `read_only`：占位权限，不授予 shell、code、fs、network、clipboard 能力

`fs_scoped` 默认允许插件目录本身。额外路径通过 `fs_scopes` 声明。相对路径按插件目录解析。路径必须能 canonicalize，超出 scope 会失败。

`network` 只支持 `http://`，并且 host 必须命中 `network_hosts`。未写端口时按 `:80` 处理。

## JavaScript Host API

内嵌 runtime 会向脚本注入 `globalThis.std`：

```javascript
const input = std.args();
const pluginDir = std.pluginDir();
std.print("plain text");
std.error("stderr text");
std.emit({ ok: true });
const body = std.readTextFile(pluginDir + "/data/input.txt");
std.writeTextFile(pluginDir + "/data/output.txt", body);
const response = std.httpGet("http://127.0.0.1:8080/status");
const records = std.clipboardHistory(5);
```

API 说明：

- `std.args()`：读取执行参数 JSON
- `std.pluginDir()`：读取当前插件目录的 canonical path
- `std.print(value)`：写 stdout
- `std.error(value)`：写 stderr
- `std.emit(value)`：把 JSON 序列化后写 stdout
- `std.readTextFile(path)`：读取 scoped text file，需要 `fs_scoped`
- `std.writeTextFile(path, body)`：写入 scoped text file，需要 `fs_scoped`
- `std.httpGet(url)`：执行 HTTP GET，需要 `network`
- `std.clipboardHistory(limit)`：读取剪切板历史，需要 `clipboard`

## 示例插件

仓库提供三个可运行示例：

- `examples/plugins/hello-js`
- `examples/plugins/scoped-fs`
- `examples/plugins/typed-ts`

安装到临时 data dir：

```bash
mkdir -p .smoke/data/plugins
cp -R examples/plugins/hello-js .smoke/data/plugins/
cp -R examples/plugins/scoped-fs .smoke/data/plugins/
cp -R examples/plugins/typed-ts .smoke/data/plugins/
printf '{"data_dir":".smoke/data"}' > .smoke/std-cli.json
```

运行 smoke：

```bash
STDCLI_CONFIG=.smoke/std-cli.json cargo run -p std-cli -- plugin list
STDCLI_CONFIG=.smoke/std-cli.json cargo run -p std-cli -- search plugin-hello-js
STDCLI_CONFIG=.smoke/std-cli.json cargo run -p std-cli -- plugin run plugin-hello-js
STDCLI_CONFIG=.smoke/std-cli.json cargo run -p std-cli -- plugin run plugin-scoped-fs
STDCLI_CONFIG=.smoke/std-cli.json cargo run -p std-cli -- plugin run plugin-typed-ts
```

预期结果：

- `plugin list` 输出三个 `plugin.json`
- `search plugin-hello-js` 命中 `Plugin Hello JS`
- `plugin run plugin-hello-js` 返回 `runtime=deno_core`，stdout 包含 `hello-js`
- `plugin run plugin-scoped-fs` 返回 `runtime=deno_core`，stdout 包含 `processed by scoped-fs`
- `plugin run plugin-typed-ts` 返回 `runtime=deno_core`，stdout 包含 `typed-ts`

清理 smoke：

```bash
rm -rf .smoke
```
