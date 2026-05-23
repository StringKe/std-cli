use crate::{AnalysisWorkbenchTab, AnalysisWorkbenchViewModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisVisibilityState {
    pub tabs: Vec<AnalysisVisibleTab>,
    pub coverage_layers: Vec<AnalysisVisibleCoverageLayer>,
    pub search_hits: usize,
    pub answer_sources: usize,
    pub components: usize,
    pub relations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisVisibleTab {
    pub key: &'static str,
    pub label: &'static str,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisVisibleCoverageLayer {
    pub key: &'static str,
    pub status: &'static str,
    pub count: usize,
}

impl AnalysisVisibilityState {
    pub fn from_workbench(model: &AnalysisWorkbenchViewModel) -> Self {
        Self {
            tabs: model
                .tabs
                .iter()
                .map(|tab| AnalysisVisibleTab {
                    key: tab.tab.key(),
                    label: tab.tab.label(),
                    count: tab.count,
                })
                .collect(),
            coverage_layers: model
                .coverage_layers
                .iter()
                .map(|layer| AnalysisVisibleCoverageLayer {
                    key: layer.key,
                    status: layer.status_label(),
                    count: layer.count,
                })
                .collect(),
            search_hits: model.search_hits.len(),
            answer_sources: model.answer_sources.len(),
            components: tab_count(model, AnalysisWorkbenchTab::Components),
            relations: tab_count(model, AnalysisWorkbenchTab::Relations),
        }
    }

    pub fn docs_22_core_tabs_visible(&self) -> bool {
        self.tabs.iter().map(|tab| tab.label).eq([
            "Overview",
            "Components",
            "Symbols",
            "Relations",
            "Q&A",
        ])
    }

    pub fn four_layer_coverage_visible(&self) -> bool {
        self.coverage_layers.iter().map(|layer| layer.key).eq([
            "overview",
            "components",
            "relations",
            "history",
        ])
    }

    pub fn status_text_is_explicit(&self) -> bool {
        self.coverage_layers
            .iter()
            .all(|layer| matches!(layer.status, "PASS" | "FAIL"))
    }

    pub fn visual_contract(&self) -> String {
        let coverage = self
            .coverage_layers
            .iter()
            .map(|layer| format!("{}:{}", layer.key, layer.status))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "toolbar=target-path|re-index|qa-input;tabs={};overview=target|index|activity;coverage={};symbols=search-hits:{};qa=sources:{};components={};relations={}",
            self.tab_labels().join("|"),
            coverage,
            self.search_hits,
            self.answer_sources,
            self.components,
            self.relations
        )
    }

    fn tab_labels(&self) -> Vec<&'static str> {
        self.tabs.iter().map(|tab| tab.label).collect()
    }
}

fn tab_count(model: &AnalysisWorkbenchViewModel, tab: AnalysisWorkbenchTab) -> usize {
    model
        .tabs
        .iter()
        .find(|item| item.tab == tab)
        .map(|item| item.count)
        .unwrap_or(0)
}
