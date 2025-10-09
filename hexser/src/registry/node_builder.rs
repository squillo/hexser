//! Utilities for building HexNode instances from registered components.
//!
//! Converts NodeInfo into HexNode for graph construction.
//!
//! Revision History
//! - 2025-10-02T12:30:00Z @AI: Fix HexNode construction to use with_metadata.
//! - 2025-10-02T00:00:00Z @AI: Initial node builder implementation.

/// Build a HexNode from NodeInfo
pub fn build_node_from_info(
  info: crate::registry::node_info::NodeInfo,
) -> crate::graph::hex_node::HexNode {
  let node_id = crate::graph::node_id::NodeId::from_type_name(info.type_name);
  let metadata = std::collections::HashMap::new();

  crate::graph::hex_node::HexNode::with_metadata(
    node_id,
    info.layer,
    info.role,
    info.type_name,
    info.module_path,
    metadata,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_build_node_from_info() {
    let info = crate::registry::node_info::NodeInfo {
      layer: crate::graph::layer::Layer::Domain,
      role: crate::graph::role::Role::Entity,
      type_name: "TestNode",
      module_path: "test",
    };

    let node = build_node_from_info(info);
    assert_eq!(node.type_name(), "TestNode");
  }
}
