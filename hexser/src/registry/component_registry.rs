//! Global component registry using inventory pattern.
//!
//! Collects all registered components at compile time and provides
//! methods to build the architecture graph.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 (SB 15/203) — call the canonical `with_node`/`with_edge`/`NodeId::from_name`; their aliases are deprecated.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D5 — `build_graph` adds one `Implements` edge per submitted `ImplementsEntry`.
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
      let node_id = crate::graph::node_id::NodeId::from_name(info.type_name);

      let metadata = std::collections::HashMap::new();

      let node = crate::graph::hex_node::HexNode::with_metadata(
        node_id,
        info.layer,
        info.role,
        info.type_name,
        info.module_path,
        metadata,
      );

      builder = builder.with_node(node);

      for dep_id in entry.dependencies() {
        let edge = crate::graph::hex_edge::HexEdge::new(
          node_id,
          dep_id,
          crate::graph::relationship::Relationship::Depends,
        );
        builder = builder.with_edge(edge);
      }
    }

    // Proved claims (HEXSER-0.7 D5): each `implements(..)` the compiler checked is one
    // `Implements` edge, so `Implements` is never a declaration (SB 15/380).
    for claim in inventory::iter::<crate::registry::implements_entry::ImplementsEntry> {
      builder = builder.with_edge(crate::graph::hex_edge::HexEdge::new(
        claim.adapter(),
        claim.port(),
        crate::graph::relationship::Relationship::Implements,
      ));
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

  /// why: component_count() must return without panicking and agree with the number of
  /// entries build_graph() produces, so the registry count is a trustworthy signal. (The old
  /// assertion `count >= 0` was vacuous on a usize.)
  #[test]
  fn test_registry_operations() {
    let count = ComponentRegistry::component_count();
    let graph = ComponentRegistry::build_graph();
    assert_eq!(count, graph.node_count());
  }
}
