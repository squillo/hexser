//! Visualizable trait for components that can be visualized.
//!
//! Provides multiple visualization formats for architecture
//! components, enabling documentation and analysis.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Fix infinite recursion (stack overflow) when the `visualization` feature is off: delegate to the inherent methods via UFCS when enabled, return an actionable error when not.
//! - 2025-10-02T17:00:00Z @AI: Initial Visualizable trait implementation.

/// Trait for components that can be visualized.
///
/// `to_dot`/`to_mermaid` require the `visualization` feature. When it is disabled they return
/// an error explaining how to enable it, rather than failing to build or (previously) aborting
/// with a stack overflow. `to_ascii_art` is always available.
pub trait Visualizable {
  /// Export to DOT format (GraphViz). Requires the `visualization` feature.
  fn to_dot(&self) -> crate::result::hex_result::HexResult<String>;

  /// Export to Mermaid format. Requires the `visualization` feature.
  fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String>;

  /// Generate ASCII art representation. Always available.
  fn to_ascii_art(&self) -> String;
}

impl Visualizable for crate::graph::hex_graph::HexGraph {
  fn to_dot(&self) -> crate::result::hex_result::HexResult<String> {
    // Call the inherent method explicitly (UFCS) so this never resolves back to the trait
    // method. The inherent method only exists under the `visualization` feature.
    #[cfg(feature = "visualization")]
    {
      crate::graph::hex_graph::HexGraph::to_dot(self)
    }
    #[cfg(not(feature = "visualization"))]
    {
      std::result::Result::Err(
        crate::error::hex_error::Hexserror::port(
          "E_HEX_VIZ_001",
          "DOT export requires the `visualization` feature, which is not enabled",
        )
        .with_next_step("Enable the `visualization` feature on the hexser dependency"),
      )
    }
  }

  fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String> {
    #[cfg(feature = "visualization")]
    {
      crate::graph::hex_graph::HexGraph::to_mermaid(self)
    }
    #[cfg(not(feature = "visualization"))]
    {
      std::result::Result::Err(
        crate::error::hex_error::Hexserror::port(
          "E_HEX_VIZ_002",
          "Mermaid export requires the `visualization` feature, which is not enabled",
        )
        .with_next_step("Enable the `visualization` feature on the hexser dependency"),
      )
    }
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

  fn sample_graph() -> crate::graph::hex_graph::HexGraph {
    crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("Test"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "Test",
        "test",
      ))
      .build()
  }

  /// why: with the feature enabled, the trait methods must delegate to the real inherent
  /// exporters and succeed; also guards that ascii art (always available) renders the layer.
  #[test]
  #[cfg(feature = "visualization")]
  fn test_visualizable_trait_enabled() {
    let graph = sample_graph();

    let ascii = graph.to_ascii_art();
    assert!(ascii.contains("Domain Layer"));
    assert!(ascii.contains("Test"));

    assert!(Visualizable::to_dot(&graph).is_ok());
    assert!(Visualizable::to_mermaid(&graph).is_ok());
    assert!(graph.to_json().is_ok());
  }

  /// why: with the feature DISABLED (the default), the trait methods must return an error
  /// instead of recursing into themselves and aborting with a stack overflow (H1). Regression
  /// guard for the exact defect: before the fix this test would crash the process.
  #[test]
  #[cfg(not(feature = "visualization"))]
  fn test_visualizable_trait_disabled_returns_error_not_stack_overflow() {
    let graph = sample_graph();

    // ascii art is always available.
    assert!(graph.to_ascii_art().contains("Domain Layer"));

    let dot = Visualizable::to_dot(&graph);
    let mermaid = Visualizable::to_mermaid(&graph);
    assert!(
      dot.is_err(),
      "to_dot must error, not recurse, when feature is off"
    );
    assert!(
      mermaid.is_err(),
      "to_mermaid must error, not recurse, when feature is off"
    );
  }
}
