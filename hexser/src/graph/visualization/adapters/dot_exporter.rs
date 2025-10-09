//! DOT format exporter adapter.
//!
//! Exports graphs to GraphViz DOT format.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial DOT exporter implementation.

/// DOT format exporter
pub struct DotExporter {
  pub rankdir: String,
}

impl DotExporter {
  /// Create new DOT exporter
  pub fn new() -> Self {
    Self {
      rankdir: String::from("TB"),
    }
  }
}

impl crate::graph::visualization::ports::format_exporter::FormatExporter for DotExporter {
  fn export(
    &self,
    visual_graph: &crate::graph::visualization::domain::visual_graph::VisualGraph,
  ) -> crate::result::hex_result::HexResult<String> {
    let mut output = format!("digraph hex_architecture {{\n  rankdir={};\n", self.rankdir);
    output.push_str("  node [shape=box, style=rounded];\n\n");

    for node in &visual_graph.nodes {
      output.push_str(&format!(
        "  \"{}\" [label=\"{}\\n({})\", fillcolor={}, style=filled];\n",
        node.id, node.label, node.role, node.color
      ));
    }

    output.push_str("\n");

    for edge in &visual_graph.edges {
      output.push_str(&format!(
        "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
        edge.source, edge.target, edge.relationship
      ));
    }

    output.push_str("}\n");
    Ok(output)
  }

  fn format_name(&self) -> &str {
    "DOT (GraphViz)"
  }

  fn file_extension(&self) -> &str {
    "dot"
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::graph::visualization::ports::format_exporter::FormatExporter;

  #[test]
  fn test_dot_export() {
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

    let exporter = DotExporter::new();
    let result = exporter.export(&visual);

    assert!(result.is_ok());
    let dot = result.unwrap();
    assert!(dot.contains("digraph hex_architecture"));
    assert!(dot.contains("Test"));
  }
}
