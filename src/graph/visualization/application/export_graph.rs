//! Export graph use case.
//!
//! Orchestrates graph export using format exporters.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial ExportGraph use case.

/// Export graph use case
pub struct ExportGraph<'a> {
    exporter: &'a dyn crate::graph::visualization::ports::format_exporter::FormatExporter,
}

impl<'a> ExportGraph<'a> {
    /// Create new export use case
    pub fn new(
        exporter: &'a dyn crate::graph::visualization::ports::format_exporter::FormatExporter,
    ) -> Self {
        Self { exporter }
    }

    /// Execute export
    pub fn execute(
        &self,
        graph: &crate::graph::hex_graph::HexGraph,
        style: crate::graph::visualization::domain::visual_style::VisualStyle,
    ) -> crate::result::hex_result::HexResult<String> {
        let visual_graph = crate::graph::visualization::domain::visual_graph::VisualGraph::from_hex_graph(graph, style);
        self.exporter.export(&visual_graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_use_case() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("Test"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Test",
                "test",
            ))
            .build();

        let exporter = crate::graph::visualization::adapters::dot_exporter::DotExporter::new();
        let use_case = ExportGraph::new(&exporter);
        let result = use_case.execute(
            &graph,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        );

        assert!(result.is_ok());
    }
}
