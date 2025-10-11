//! Mermaid format exporter adapter.
//!
//! Exports graphs to Mermaid diagram format.
//!
//! Revision History
//! - 2025-10-10T17:33:00Z @AI: Fix node ID sanitization to remove NodeId() wrapper for valid Mermaid syntax.
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

  /// Sanitize node ID for Mermaid syntax
  ///
  /// Removes "NodeId(" prefix and ")" suffix, replaces "::" with "_"
  /// to create valid Mermaid node identifiers
  fn sanitize_node_id(id: &str) -> String {
    let cleaned = if id.starts_with("NodeId(") && id.ends_with(")") {
      // Extract the numeric part: "NodeId(12345)" -> "12345"
      &id[7..id.len() - 1]
    } else {
      id
    };
    // Also replace :: with _ for any remaining type names
    cleaned.replace("::", "_")
  }
}

impl crate::graph::visualization::ports::format_exporter::FormatExporter for MermaidExporter {
  fn export(
    &self,
    visual_graph: &crate::graph::visualization::domain::visual_graph::VisualGraph,
  ) -> crate::result::hex_result::HexResult<String> {
    let mut output = format!("graph {}\n", self.direction);

    for node in &visual_graph.nodes {
      let node_id = Self::sanitize_node_id(&node.id);
      output.push_str(&format!(
        "  {}[\"{}\\n({})\"]\n",
        node_id, node.label, node.role
      ));
    }

    output.push_str("\n");

    for edge in &visual_graph.edges {
      let source_id = Self::sanitize_node_id(&edge.source);
      let target_id = Self::sanitize_node_id(&edge.target);
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
  use super::*;
  use crate::graph::visualization::ports::format_exporter::FormatExporter;

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
