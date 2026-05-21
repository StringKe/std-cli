use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.plugins.title") => Some("Plugin Manager"),
        (Locale::EnUs, "studio.plugins.title") => Some("Plugin Manager"),
        (Locale::ZhCn, "studio.plugins.detail") => Some("manifest 检查、权限边界、JS/TS 执行"),
        (Locale::EnUs, "studio.plugins.detail") => {
            Some("manifest checks, scoped permissions, JS/TS execution")
        }
        (Locale::ZhCn, "studio.plugins.search.hint") => Some("plugin action、tag、manifest"),
        (Locale::EnUs, "studio.plugins.search.hint") => Some("plugin action, tag, manifest"),
        (Locale::ZhCn, "studio.plugins.search") => Some("搜索"),
        (Locale::EnUs, "studio.plugins.search") => Some("Search"),
        (Locale::ZhCn, "studio.plugins.reload") => Some("重载"),
        (Locale::EnUs, "studio.plugins.reload") => Some("Reload"),
        (Locale::ZhCn, "studio.plugins.run") => Some("运行"),
        (Locale::EnUs, "studio.plugins.run") => Some("Run"),
        (Locale::ZhCn, "studio.plugins.status.title") => Some("Runtime Status"),
        (Locale::EnUs, "studio.plugins.status.title") => Some("Runtime Status"),
        (Locale::ZhCn, "studio.plugins.status.detail") => {
            Some("manifest、preview、runtime、permission、boundary")
        }
        (Locale::EnUs, "studio.plugins.status.detail") => {
            Some("manifest, preview, runtime, permission, boundary")
        }
        (Locale::ZhCn, "studio.plugins.status.no_preview") => Some("未选择预览"),
        (Locale::EnUs, "studio.plugins.status.no_preview") => Some("No preview"),
        (Locale::ZhCn, "studio.plugins.status.no_run") => Some("尚未运行"),
        (Locale::EnUs, "studio.plugins.status.no_run") => Some("No run"),
        (Locale::ZhCn, "studio.plugins.runtime.completed") => Some("已完成"),
        (Locale::EnUs, "studio.plugins.runtime.completed") => Some("Completed"),
        (Locale::ZhCn, "studio.plugins.runtime.failed") => Some("无法执行"),
        (Locale::EnUs, "studio.plugins.runtime.failed") => Some("Unable to run"),
        (Locale::ZhCn, "studio.plugins.runtime.deferred") => Some("需要确认"),
        (Locale::EnUs, "studio.plugins.runtime.deferred") => Some("Needs review"),
        (Locale::ZhCn, "studio.plugins.manifests.title") => Some("Manifests"),
        (Locale::EnUs, "studio.plugins.manifests.title") => Some("Manifests"),
        (Locale::ZhCn, "studio.plugins.manifests.detail") => Some("已发现 plugin.json"),
        (Locale::EnUs, "studio.plugins.manifests.detail") => Some("discovered plugin.json"),
        (Locale::ZhCn, "studio.plugins.manifests.empty") => Some("没有 plugin manifest"),
        (Locale::EnUs, "studio.plugins.manifests.empty") => Some("No plugin manifests"),
        (Locale::ZhCn, "studio.plugins.actions.title") => Some("Actions"),
        (Locale::EnUs, "studio.plugins.actions.title") => Some("Actions"),
        (Locale::ZhCn, "studio.plugins.actions.detail") => Some("执行前预览"),
        (Locale::EnUs, "studio.plugins.actions.detail") => Some("preview before execution"),
        (Locale::ZhCn, "studio.plugins.actions.empty") => Some("没有 plugin action"),
        (Locale::EnUs, "studio.plugins.actions.empty") => Some("No plugin actions"),
        (Locale::ZhCn, "studio.plugins.checks.title") => Some("Checks"),
        (Locale::EnUs, "studio.plugins.checks.title") => Some("Checks"),
        (Locale::ZhCn, "studio.plugins.checks.detail") => Some("权限与 scope"),
        (Locale::EnUs, "studio.plugins.checks.detail") => Some("permissions and scopes"),
        (Locale::ZhCn, "studio.plugins.checks.empty") => Some("没有 manifest check report"),
        (Locale::EnUs, "studio.plugins.checks.empty") => Some("No manifest check reports"),
        (Locale::ZhCn, "studio.plugins.security.title") => Some("Security Boundary"),
        (Locale::EnUs, "studio.plugins.security.title") => Some("Security Boundary"),
        (Locale::ZhCn, "studio.plugins.security.detail") => Some("权限、文件、网络、action"),
        (Locale::EnUs, "studio.plugins.security.detail") => {
            Some("permission, file, network, action")
        }
        (Locale::ZhCn, "studio.plugins.preview.title") => Some("Preview"),
        (Locale::EnUs, "studio.plugins.preview.title") => Some("Preview"),
        (Locale::ZhCn, "studio.plugins.preview.detail") => Some("已选择 action"),
        (Locale::EnUs, "studio.plugins.preview.detail") => Some("selected action"),
        (Locale::ZhCn, "studio.plugins.preview.empty") => Some("没有选择 action"),
        (Locale::EnUs, "studio.plugins.preview.empty") => Some("No action selected"),
        (Locale::ZhCn, "studio.plugins.execution.title") => Some("Execution"),
        (Locale::EnUs, "studio.plugins.execution.title") => Some("Execution"),
        (Locale::ZhCn, "studio.plugins.execution.detail") => Some("最近受控运行"),
        (Locale::EnUs, "studio.plugins.execution.detail") => Some("last controlled run"),
        (Locale::ZhCn, "studio.plugins.execution.empty") => Some("还没有执行记录"),
        (Locale::EnUs, "studio.plugins.execution.empty") => Some("No execution yet"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.plugins.title" => Some("Plugin Manager"),
        "studio.plugins.detail" => Some("manifest checks, scoped permissions, JS/TS execution"),
        "studio.plugins.search.hint" => Some("plugin action, tag, manifest"),
        "studio.plugins.search" => Some("Search"),
        "studio.plugins.reload" => Some("Reload"),
        "studio.plugins.run" => Some("Run"),
        "studio.plugins.status.title" => Some("Runtime Status"),
        "studio.plugins.status.detail" => Some("manifest, preview, runtime, permission, boundary"),
        "studio.plugins.status.no_preview" => Some("No preview"),
        "studio.plugins.status.no_run" => Some("No run"),
        "studio.plugins.runtime.completed" => Some("Completed"),
        "studio.plugins.runtime.failed" => Some("Unable to run"),
        "studio.plugins.runtime.deferred" => Some("Needs review"),
        "studio.plugins.manifests.title" => Some("Manifests"),
        "studio.plugins.manifests.detail" => Some("discovered plugin.json"),
        "studio.plugins.manifests.empty" => Some("No plugin manifests"),
        "studio.plugins.actions.title" => Some("Actions"),
        "studio.plugins.actions.detail" => Some("preview before execution"),
        "studio.plugins.actions.empty" => Some("No plugin actions"),
        "studio.plugins.checks.title" => Some("Checks"),
        "studio.plugins.checks.detail" => Some("permissions and scopes"),
        "studio.plugins.checks.empty" => Some("No manifest check reports"),
        "studio.plugins.security.title" => Some("Security Boundary"),
        "studio.plugins.security.detail" => Some("permission, file, network, action"),
        "studio.plugins.preview.title" => Some("Preview"),
        "studio.plugins.preview.detail" => Some("selected action"),
        "studio.plugins.preview.empty" => Some("No action selected"),
        "studio.plugins.execution.title" => Some("Execution"),
        "studio.plugins.execution.detail" => Some("last controlled run"),
        "studio.plugins.execution.empty" => Some("No execution yet"),
        _ => None,
    }
}
