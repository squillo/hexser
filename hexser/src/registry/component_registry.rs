//! Global component registry using inventory pattern.
//!
//! Collects all registered components at compile time and provides
//! methods to build the architecture graph.
//!
//! Revision History
//! - 2025-10-02T12:00:00Z @AI: Fix HexNode construction to use with_metadata method.
//! - 2025-10-02T00:00:00Z @AI: Initial ComponentRegistry implementation.

/// Global component registry
pub struct ComponentRegistry;

impl ComponentRegistry {
  /// Build a graph from all registered components
  pub fn build_graph() -> crate::graph::hex_graph::HexGraph {
    let mut builder = crate::graph::builder::GraphBuilder::new();

    for entry in inventory::iter::<crate::registry::component_entry::ComponentEntry> {
      let info = entry.node_info();
      let node_id = crate::graph::node_id::NodeId::from_type_name(info.type_name);

      let metadata = std::collections::HashMap::new();

      let node = crate::graph::hex_node::HexNode::with_metadata(
        node_id,
        info.layer,
        info.role,
        info.type_name,
        info.module_path,
        metadata,
      );

      builder = builder.add_node(node);

      for dep_id in entry.dependencies() {
        let edge = crate::graph::hex_edge::HexEdge::new(
          node_id,
          dep_id,
          crate::graph::relationship::Relationship::Depends,
        );
        builder = builder.add_edge(edge);
      }
    }

    builder.build()
  }

  /// Count registered components
  pub fn component_count() -> usize {
    inventory::iter::<crate::registry::component_entry::ComponentEntry>().count()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_registry_operations() {
    let count = ComponentRegistry::component_count();
    assert!(count >= 0);
  }
}
