//! Visual graph domain model.
//!
//! Abstract representation of architecture graph for visualization,
//! independent of output format.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial VisualGraph implementation.

/// Visual representation of architecture graph
#[derive(Clone, Debug)]
pub struct VisualGraph {
  pub nodes: Vec<crate::graph::visualization::domain::visual_node::VisualNode>,
  pub edges: Vec<crate::graph::visualization::domain::visual_edge::VisualEdge>,
  pub style: crate::graph::visualization::domain::visual_style::VisualStyle,
}

impl VisualGraph {
  /// Create from HexGraph
  pub fn from_hex_graph(
    graph: &crate::graph::hex_graph::HexGraph,
    style: crate::graph::visualization::domain::visual_style::VisualStyle,
  ) -> Self {
    let nodes = graph
      .nodes()
      .map(|node| {
        crate::graph::visualization::domain::visual_node::VisualNode::from_hex_node(node, &style)
      })
      .collect();

    let edges = graph
      .edges()
      .into_iter()
      .map(|edge| crate::graph::visualization::domain::visual_edge::VisualEdge::from_hex_edge(edge))
      .collect();

    Self {
      nodes,
      edges,
      style,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_visual_graph() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("Test"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "Test",
        "test",
      ))
      .build();

    let visual = VisualGraph::from_hex_graph(
      &graph,
      crate::graph::visualization::domain::visual_style::VisualStyle::default(),
    );

    assert_eq!(visual.nodes.len(), 1);
  }
}
