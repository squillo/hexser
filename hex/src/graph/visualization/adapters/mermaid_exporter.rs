//! Mermaid format exporter adapter.
//!
//! Exports graphs to Mermaid diagram format.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial Mermaid exporter implementation.

/// Mermaid format exporter
pub struct MermaidExporter {
    pub direction: String,
}

impl MermaidExporter {
    /// Create new Mermaid exporter
    pub fn new() -> Self {
        Self {
            direction: String::from("TD"),
        }
    }
}

impl crate::graph::visualization::ports::format_exporter::FormatExporter for MermaidExporter {
    fn export(
        &self,
        visual_graph: &crate::graph::visualization::domain::visual_graph::VisualGraph,
    ) -> crate::result::hex_result::HexResult<String> {
        let mut output = format!("graph {}\n", self.direction);

        for node in &visual_graph.nodes {
            let node_id = node.id.replace("::", "_");
            output.push_str(&format!("  {}[\"{}\\n({})\"]\n", node_id, node.label, node.role));
        }

        output.push_str("\n");

        for edge in &visual_graph.edges {
            let source_id = edge.source.replace("::", "_");
            let target_id = edge.target.replace("::", "_");
            output.push_str(&format!(
                "  {} -->|{}| {}\n",
                source_id, edge.relationship, target_id
            ));
        }

        Ok(output)
    }

    fn format_name(&self) -> &str {
        "Mermaid"
    }

    fn file_extension(&self) -> &str {
        "mmd"
    }
}

#[cfg(test)]
mod tests {
  use crate::graph::visualization::ports::format_exporter::FormatExporter;
  use super::*;

    #[test]
    fn test_mermaid_export() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("Test"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Test",
                "test",
            ))
            .build();

        let visual = crate::graph::visualization::domain::visual_graph::VisualGraph::from_hex_graph(
            &graph,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        );

        let exporter = MermaidExporter::new();
        let result = exporter.export(&visual);

        assert!(result.is_ok());
        let mermaid = result.unwrap();
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("Test"));
    }
}
