//! Describable trait for self-describing components.
//!
//! Provides human-readable descriptions for documentation and debugging.
//! Components can describe their purpose, name, and category.
//!
//! Revision History
//! - 2025-10-02T19:00:00Z @AI: Complete rewrite to fix merge conflicts.

/// Trait for self-describing components
pub trait Describable {
  /// Get detailed description
  fn describe(&self) -> String;

  /// Get short name
  fn short_name(&self) -> &str;

  /// Get category
  fn category(&self) -> &str;
}

impl Describable for crate::graph::hex_node::HexNode {
  fn describe(&self) -> String {
    format!(
      "{} ({:?} in {:?} layer)",
      self.type_name, self.role, self.layer
    )
  }

  fn short_name(&self) -> &str {
    &self.type_name
  }

  fn category(&self) -> &str {
    match self.layer {
      crate::graph::layer::Layer::Domain => "Domain Model",
      crate::graph::layer::Layer::Port => "Interface",
      crate::graph::layer::Layer::Adapter => "Implementation",
      crate::graph::layer::Layer::Application => "Use Case",
      crate::graph::layer::Layer::Infrastructure => "Infrastructure",
      crate::graph::layer::Layer::Unknown => "Unknown",
    }
  }
}

impl Describable for crate::graph::hex_graph::HexGraph {
  fn describe(&self) -> String {
    format!(
      "HexGraph with {} nodes and {} edges",
      self.node_count(),
      self.edge_count()
    )
  }

  fn short_name(&self) -> &str {
    "HexGraph"
  }

  fn category(&self) -> &str {
    "Architecture Graph"
  }
}

impl Describable for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
  fn describe(&self) -> String {
    (**self).describe()
  }

  fn short_name(&self) -> &str {
    (**self).short_name()
  }

  fn category(&self) -> &str {
    (**self).category()
  }
}

/// Extension trait for pretty printing
pub trait PrettyPrint {
  /// Print description
  fn pretty_print(&self);
}

impl PrettyPrint for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
  fn pretty_print(&self) {
    println!("{}", self.describe());
  }
}

/// Extension methods for Arc<HexGraph>
pub trait ArcGraphExt {
  /// Get nodes by layer
  fn nodes_by_layer(&self, layer: crate::graph::layer::Layer)
    -> Vec<&crate::graph::hex_node::HexNode>;

  /// Get nodes by role
  fn nodes_by_role(&self, role: crate::graph::role::Role)
    -> Vec<&crate::graph::hex_node::HexNode>;
}

impl ArcGraphExt for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
  fn nodes_by_layer(&self, layer: crate::graph::layer::Layer)
    -> Vec<&crate::graph::hex_node::HexNode>
  {
    (**self).nodes_by_layer(layer)
  }

  fn nodes_by_role(&self, role: crate::graph::role::Role)
    -> Vec<&crate::graph::hex_node::HexNode>
  {
    (**self).nodes_by_role(role)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_describable() {
    let node = crate::graph::hex_node::HexNode::new(
      crate::graph::node_id::NodeId::from_name("User"),
      crate::graph::layer::Layer::Domain,
      crate::graph::role::Role::Entity,
      "User",
      "domain::user",
    );

    assert_eq!(node.short_name(), "User");
    assert_eq!(node.category(), "Domain Model");
    assert!(node.describe().contains("Entity"));
  }

  #[test]
  fn test_graph_describable() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    assert_eq!(graph.short_name(), "HexGraph");
    assert_eq!(graph.category(), "Architecture Graph");
    assert!(graph.describe().contains("0 nodes"));
  }

  #[test]
  fn test_arc_graph_ext() {
    let graph = std::sync::Arc::new(
      crate::graph::builder::GraphBuilder::new()
        .add_node(crate::graph::hex_node::HexNode::new(
          crate::graph::node_id::NodeId::from_name("Test"),
          crate::graph::layer::Layer::Domain,
          crate::graph::role::Role::Entity,
          "Test",
          "test",
        ))
        .build()
    );

    let domain_nodes = graph.nodes_by_layer(crate::graph::layer::Layer::Domain);
    assert_eq!(domain_nodes.len(), 1);

    let entities = graph.nodes_by_role(crate::graph::role::Role::Entity);
    assert_eq!(entities.len(), 1);
  }
}

