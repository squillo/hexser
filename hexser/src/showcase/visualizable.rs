//! Visualizable trait for components that can be visualized.
//!
//! Provides multiple visualization formats for architecture
//! components, enabling documentation and analysis.
//!
//! Revision History
//! - 2025-10-02T17:00:00Z @AI: Initial Visualizable trait implementation.

/// Trait for components that can be visualized
pub trait Visualizable {
  /// Export to DOT format (GraphViz)
  fn to_dot(&self) -> crate::result::hex_result::HexResult<String>;

  /// Export to Mermaid format
  fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String>;

  /// Generate ASCII art representation
  fn to_ascii_art(&self) -> String;
}

impl Visualizable for crate::graph::hex_graph::HexGraph {
  fn to_dot(&self) -> crate::result::hex_result::HexResult<String> {
    self.to_dot()
  }

  fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String> {
    self.to_mermaid()
  }

  fn to_ascii_art(&self) -> String {
    let mut output = String::new();
    output.push_str("Architecture:\n");

    for layer in [
      crate::graph::layer::Layer::Application,
      crate::graph::layer::Layer::Port,
      crate::graph::layer::Layer::Adapter,
      crate::graph::layer::Layer::Domain,
      crate::graph::layer::Layer::Infrastructure,
    ] {
      let nodes = self.nodes_by_layer(layer);
      if !nodes.is_empty() {
        output.push_str(&format!("\n{:?} Layer:\n", layer));
        for node in nodes {
          output.push_str(&format!("  └─ {}\n", node.type_name));
        }
      }
    }

    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[cfg(feature = "visualization")]
  fn test_visualizable_trait() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("Test"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "Test",
        "test",
      ))
      .build();

    let ascii = graph.to_ascii_art();
    assert!(ascii.contains("Domain Layer"));
    assert!(ascii.contains("Test"));

    assert!(graph.to_dot().is_ok());
    assert!(graph.to_mermaid().is_ok());
    assert!(graph.to_json().is_ok());
  }
}
