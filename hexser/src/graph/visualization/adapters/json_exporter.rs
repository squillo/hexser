//! JSON format exporter adapter.
//!
//! Exports graphs to JSON format compatible with D3.js.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial JSON exporter implementation.

/// JSON format exporter
pub struct JsonExporter;

impl JsonExporter {
  /// Create new JSON exporter
  pub fn new() -> Self {
    Self
  }
}

#[derive(serde::Serialize)]
struct D3Graph {
  nodes: Vec<D3Node>,
  links: Vec<D3Link>,
}

#[derive(serde::Serialize)]
struct D3Node {
  id: String,
  name: String,
  group: String,
}

#[derive(serde::Serialize)]
struct D3Link {
  source: String,
  target: String,
  value: usize,
}

impl crate::graph::visualization::ports::format_exporter::FormatExporter for JsonExporter {
  fn export(
    &self,
    visual_graph: &crate::graph::visualization::domain::visual_graph::VisualGraph,
  ) -> crate::result::hex_result::HexResult<String> {
    let d3_nodes = visual_graph
      .nodes
      .iter()
      .map(|node| D3Node {
        id: node.id.clone(),
        name: node.label.clone(),
        group: node.layer.clone(),
      })
      .collect();

    let d3_links = visual_graph
      .edges
      .iter()
      .map(|edge| D3Link {
        source: edge.source.clone(),
        target: edge.target.clone(),
        value: 1,
      })
      .collect();

    let d3_graph = D3Graph {
      nodes: d3_nodes,
      links: d3_links,
    };

    serde_json::to_string_pretty(&d3_graph).map_err(move |e| {
      let msg = format!("JSON serialization failed: {}", e);
      crate::error::hex_error::Hexserror::adapter("E_HEX_VIZ_001", msg.as_str())
    })
  }

  fn format_name(&self) -> &str {
    "JSON (D3.js)"
  }

  fn file_extension(&self) -> &str {
    "json"
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::graph::visualization::ports::format_exporter::FormatExporter;

  #[test]
  fn test_json_export() {
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

    let exporter = JsonExporter::new();
    let result = exporter.export(&visual);

    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("nodes"));
    assert!(json.contains("links"));
    assert!(json.contains("Test"));
  }
}
