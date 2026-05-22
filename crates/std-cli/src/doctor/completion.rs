use crate::{
    doctor::workspace::{check_text, find_workspace_root, read_required},
    CliError,
};

const REQUIRED_AREAS: [&str; 11] = [
    "UI docs 18-24",
    "Launcher",
    "Studio",
    "Core",
    "Terminal",
    "Plugin",
    "Index",
    "Workflow",
    "Release",
    "Install",
    "Quality",
];

const MANUAL_BLOCKERS: [&str; 6] = [
    "Launcher 截图仍需按 docs/18-21 做像素级审计",
    "真实全局热键安装包验收仍需单独显式运行",
    "焦点环、IME、A11y、reduce motion 和安装版 UI 需要真实证据",
    "Studio UI 仍需按 docs/18-24 重新验收",
    "light / dark、workspace pane 打开聚焦关闭恢复、焦点、A11y、Operations 真实证据截图需要重新证明",
    "完成前必须重跑并保留当前证据",
];

const CURRENT_EVIDENCE_RULES: [&str; 6] = [
    "历史 target/ui-evidence 路径不能作为完成证据",
    "历史 /tmp 截图不能作为完成证据",
    "真实截图必须来自本轮 STD_ALLOW_UI_PREVIEW=1 capture-ui-matrix 输出",
    "真实截图 manifest 必须包含 samples、unique_colors、black_pixels、white_pixels",
    "真实截图 doctor 必须拒绝 single-color、dominant-black、dominant-white carrier",
    "安装版 GUI 验证必须来自本轮显式 desktop opt-in 输出",
];

const STALE_EVIDENCE_PATTERNS: [&str; 4] = [
    "target/ui-evidence/",
    "/tmp/std-studio-installed-ui.png",
    "screencapture -x /tmp/",
    "PNG image data, 3840 x 2160",
];

pub(crate) struct CompletionDoctor {
    pub(crate) audit: &'static str,
    pub(crate) matrix: &'static str,
    pub(crate) areas: Vec<&'static str>,
    pub(crate) blockers: Vec<&'static str>,
    pub(crate) evidence_rules: Vec<&'static str>,
    pub(crate) final_completion: &'static str,
}

pub(crate) fn check_completion_gate() -> Result<CompletionDoctor, CliError> {
    let root = find_workspace_root()?;
    let audit = read_required(&root.join("docs/16_Completion_Audit.md"))?;
    let matrix = read_required(&root.join("docs/17_Final_Completion_Matrix.md"))?;

    check_audit_doc(&audit)?;
    check_matrix_doc(&matrix)?;

    Ok(CompletionDoctor {
        audit: "PASS",
        matrix: "PASS",
        areas: REQUIRED_AREAS.to_vec(),
        blockers: MANUAL_BLOCKERS.to_vec(),
        evidence_rules: CURRENT_EVIDENCE_RULES.to_vec(),
        final_completion: "INCOMPLETE_REAL_GUI_REQUIRED",
    })
}

fn check_audit_doc(audit: &str) -> Result<(), CliError> {
    for required in [
        "v1.0 completion 未完成",
        "当前 UI 完成状态全部作废",
        "功能 smoke 和后端能力不能作为 UI 完成证据",
        "每个门槛都必须有当前运行证据",
        "UI docs 18-24、Launcher、Studio、Core、Terminal、Plugin、Index、Workflow、Release、Install、Quality",
        "默认测试和 smoke 不得唤起 Terminal、App、文件或外部 runner",
        "只有显式 opt-in 才执行真实 GUI hotkey 或外部 runner 行为",
    ] {
        check_text(audit, required)?;
    }
    for required in REQUIRED_AREAS {
        check_text(audit, required)?;
    }
    for required in CURRENT_EVIDENCE_RULES {
        check_text(audit, required)?;
    }
    reject_stale_evidence_paths(audit)?;
    Ok(())
}

fn check_matrix_doc(matrix: &str) -> Result<(), CliError> {
    for required in [
        "当前矩阵用于最终完成判断",
        "UI 完成状态全部作废",
        "不能用代码存在、测试存在或历史印象替代",
        "## Core",
        "## Launcher",
        "## Studio",
        "## Terminal",
        "## Plugin",
        "## Index",
        "## Workflow",
        "## Release",
        "## Install",
        "## Quality",
        "## 最终门槛",
        "状态：未完成",
        "状态：PASS",
    ] {
        check_text(matrix, required)?;
    }
    for blocker in MANUAL_BLOCKERS {
        check_text(matrix, blocker)?;
    }
    for required in CURRENT_EVIDENCE_RULES {
        check_text(matrix, required)?;
    }
    reject_stale_evidence_paths(matrix)?;
    Ok(())
}

fn reject_stale_evidence_paths(text: &str) -> Result<(), CliError> {
    for stale in STALE_EVIDENCE_PATTERNS {
        if text.contains(stale) {
            return Err(CliError::Doctor(format!(
                "stale UI evidence path must not appear in completion evidence: {stale}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_gate_keeps_final_goal_incomplete_until_manual_gui_evidence() {
        let report = check_completion_gate().unwrap();

        assert_eq!(report.audit, "PASS");
        assert_eq!(report.matrix, "PASS");
        assert_eq!(report.final_completion, "INCOMPLETE_REAL_GUI_REQUIRED");
        assert_eq!(report.areas, REQUIRED_AREAS);
        assert_eq!(report.evidence_rules, CURRENT_EVIDENCE_RULES);
        assert!(report
            .blockers
            .contains(&"Studio UI 仍需按 docs/18-24 重新验收"));
    }

    #[test]
    fn completion_gate_rejects_stale_ui_evidence_paths() {
        let error = reject_stale_evidence_paths(
            "evidence=target/ui-evidence/launcher-light-results-refined.png",
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("stale UI evidence path must not appear"));
    }
}
