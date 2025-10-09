//! Registrable trait for components that can be registered in the graph.
//!
//! Components implement this trait (usually via derive macros) to provide
//! metadata for automatic graph construction.
//!
//! Revision History
//! - 2025-10-02T12:30:00Z @AI: Add Sized bound to register_self method.
//! - 2025-10-02T00:00:00Z @AI: Initial Registrable trait implementation.

/// Trait for components that can be registered in the architecture graph
pub trait Registrable: 'static {
  /// Get node information for this component
  fn node_info() -> crate::registry::node_info::NodeInfo;

  /// Get IDs of components this depends on
  fn dependencies() -> Vec<crate::graph::node_id::NodeId>;

  /// Register this component (helper method)
  fn register_self() -> crate::graph::node_id::NodeId
  where
    Self: Sized,
  {
    crate::graph::node_id::NodeId::of::<Self>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestComponent;

  impl Registrable for TestComponent {
    fn node_info() -> crate::registry::node_info::NodeInfo {
      crate::registry::node_info::NodeInfo {
        layer: crate::graph::layer::Layer::Domain,
        role: crate::graph::role::Role::Entity,
        type_name: "TestComponent",
        module_path: module_path!(),
      }
    }

    fn dependencies() -> Vec<crate::graph::node_id::NodeId> {
      Vec::new()
    }
  }

  #[test]
  fn test_registrable_implementation() {
    let info = TestComponent::node_info();
    assert_eq!(info.type_name, "TestComponent");

    let deps = TestComponent::dependencies();
    assert_eq!(deps.len(), 0);
  }
}
