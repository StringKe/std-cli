use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.workflow_builder.title") => Some("Workflow Builder"),
        (Locale::EnUs, "studio.workflow_builder.title") => Some("Workflow Builder"),
        (Locale::ZhCn, "studio.workflow_builder.detail") => Some("步骤、属性、AI assist"),
        (Locale::EnUs, "studio.workflow_builder.detail") => Some("steps, properties, AI assist"),
        (Locale::ZhCn, "studio.workflow_builder.goal.hint") => Some("描述 workflow 目标"),
        (Locale::EnUs, "studio.workflow_builder.goal.hint") => Some("Describe workflow goal"),
        (Locale::ZhCn, "studio.workflow_builder.plan") => Some("Plan"),
        (Locale::EnUs, "studio.workflow_builder.plan") => Some("Plan"),
        (Locale::ZhCn, "studio.workflow_builder.simulate") => Some("Simulate"),
        (Locale::EnUs, "studio.workflow_builder.simulate") => Some("Simulate"),
        (Locale::ZhCn, "studio.workflow_builder.run") => Some("Run"),
        (Locale::EnUs, "studio.workflow_builder.run") => Some("Run"),
        (Locale::ZhCn, "studio.workflow_builder.save") => Some("Save"),
        (Locale::EnUs, "studio.workflow_builder.save") => Some("Save"),
        (Locale::ZhCn, "studio.workflow_builder.steps.title") => Some("Steps"),
        (Locale::EnUs, "studio.workflow_builder.steps.title") => Some("Steps"),
        (Locale::ZhCn, "studio.workflow_builder.steps.detail") => Some("Alt+Up Alt+Down"),
        (Locale::EnUs, "studio.workflow_builder.steps.detail") => Some("Alt+Up Alt+Down"),
        (Locale::ZhCn, "studio.workflow_builder.steps.empty") => Some("选择或规划一个 workflow"),
        (Locale::EnUs, "studio.workflow_builder.steps.empty") => Some("Select or plan a workflow"),
        (Locale::ZhCn, "studio.workflow_builder.preview.empty") => Some("还没有 preview"),
        (Locale::EnUs, "studio.workflow_builder.preview.empty") => Some("No preview yet"),
        (Locale::ZhCn, "studio.workflow_builder.properties.title") => Some("Step Properties"),
        (Locale::EnUs, "studio.workflow_builder.properties.title") => Some("Step Properties"),
        (Locale::ZhCn, "studio.workflow_builder.properties.detail") => Some("schema JSON"),
        (Locale::EnUs, "studio.workflow_builder.properties.detail") => Some("schema JSON"),
        (Locale::ZhCn, "studio.workflow_builder.properties.empty") => {
            Some("选择已保存 workflow 以编辑步骤")
        }
        (Locale::EnUs, "studio.workflow_builder.properties.empty") => {
            Some("Select a saved workflow to edit steps")
        }
        (Locale::ZhCn, "studio.workflow_builder.step_name") => Some("Step name"),
        (Locale::EnUs, "studio.workflow_builder.step_name") => Some("Step name"),
        (Locale::ZhCn, "studio.workflow_builder.parameters") => Some("Parameters JSON"),
        (Locale::EnUs, "studio.workflow_builder.parameters") => Some("Parameters JSON"),
        (Locale::ZhCn, "studio.workflow_builder.index") => Some("Index"),
        (Locale::EnUs, "studio.workflow_builder.index") => Some("Index"),
        (Locale::ZhCn, "studio.workflow_builder.add") => Some("Add"),
        (Locale::EnUs, "studio.workflow_builder.add") => Some("Add"),
        (Locale::ZhCn, "studio.workflow_builder.update") => Some("Update"),
        (Locale::EnUs, "studio.workflow_builder.update") => Some("Update"),
        (Locale::ZhCn, "studio.workflow_builder.move_up") => Some("Move Up"),
        (Locale::EnUs, "studio.workflow_builder.move_up") => Some("Move Up"),
        (Locale::ZhCn, "studio.workflow_builder.move_down") => Some("Move Down"),
        (Locale::EnUs, "studio.workflow_builder.move_down") => Some("Move Down"),
        (Locale::ZhCn, "studio.workflow_builder.remove") => Some("Remove"),
        (Locale::EnUs, "studio.workflow_builder.remove") => Some("Remove"),
        (Locale::ZhCn, "studio.workflow_builder.ai.title") => Some("AI Assist"),
        (Locale::EnUs, "studio.workflow_builder.ai.title") => Some("AI Assist"),
        (Locale::ZhCn, "studio.workflow_builder.ai.detail") => Some("从目标生成计划"),
        (Locale::EnUs, "studio.workflow_builder.ai.detail") => Some("plan from goal"),
        (Locale::ZhCn, "studio.workflow_builder.ai.prompt") => Some("描述这个 workflow 应该做什么"),
        (Locale::EnUs, "studio.workflow_builder.ai.prompt") => {
            Some("Describe what this workflow should do")
        }
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.workflow_builder.title" => Some("Workflow Builder"),
        "studio.workflow_builder.detail" => Some("steps, properties, AI assist"),
        "studio.workflow_builder.goal.hint" => Some("Describe workflow goal"),
        "studio.workflow_builder.plan" => Some("Plan"),
        "studio.workflow_builder.simulate" => Some("Simulate"),
        "studio.workflow_builder.run" => Some("Run"),
        "studio.workflow_builder.save" => Some("Save"),
        "studio.workflow_builder.steps.title" => Some("Steps"),
        "studio.workflow_builder.steps.detail" => Some("Alt+Up Alt+Down"),
        "studio.workflow_builder.steps.empty" => Some("Select or plan a workflow"),
        "studio.workflow_builder.preview.empty" => Some("No preview yet"),
        "studio.workflow_builder.properties.title" => Some("Step Properties"),
        "studio.workflow_builder.properties.detail" => Some("schema JSON"),
        "studio.workflow_builder.properties.empty" => Some("Select a saved workflow to edit steps"),
        "studio.workflow_builder.step_name" => Some("Step name"),
        "studio.workflow_builder.parameters" => Some("Parameters JSON"),
        "studio.workflow_builder.index" => Some("Index"),
        "studio.workflow_builder.add" => Some("Add"),
        "studio.workflow_builder.update" => Some("Update"),
        "studio.workflow_builder.move_up" => Some("Move Up"),
        "studio.workflow_builder.move_down" => Some("Move Down"),
        "studio.workflow_builder.remove" => Some("Remove"),
        "studio.workflow_builder.ai.title" => Some("AI Assist"),
        "studio.workflow_builder.ai.detail" => Some("plan from goal"),
        "studio.workflow_builder.ai.prompt" => Some("Describe what this workflow should do"),
        _ => None,
    }
}
