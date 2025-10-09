//! Node information for component registration.
//!
//! Contains metadata about a component for graph node construction.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial NodeInfo implementation.

/// Metadata about a component for registration
#[derive(Debug, Clone)]
pub struct NodeInfo {
  pub layer: crate::graph::layer::Layer,
  pub role: crate::graph::role::Role,
  pub type_name: &'static str,
  pub module_path: &'static str,
}

impl NodeInfo {
  /// Create new NodeInfo
  pub fn new(
    layer: crate::graph::layer::Layer,
    role: crate::graph::role::Role,
    type_name: &'static str,
    module_path: &'static str,
  ) -> Self {
    Self {
      layer,
      role,
      type_name,
      module_path,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_info_creation() {
    let info = NodeInfo::new(
      crate::graph::layer::Layer::Domain,
      crate::graph::role::Role::Entity,
      "TestType",
      "test::module",
    );

    assert_eq!(info.type_name, "TestType");
    assert_eq!(info.module_path, "test::module");
  }
}
