//! Component entry for inventory-based registration.
//!
//! Represents a single registered component in the inventory system.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial ComponentEntry implementation.

/// Entry for a registered component
pub struct ComponentEntry {
    pub node_info_fn: fn() -> crate::registry::node_info::NodeInfo,
    pub dependencies_fn: fn() -> Vec<crate::graph::node_id::NodeId>,
}

impl ComponentEntry {
    /// Create a new component entry for a type
    pub const fn new<T: crate::registry::registrable::Registrable>() -> Self {
        Self {
            node_info_fn: T::node_info,
            dependencies_fn: T::dependencies,
        }
    }

    /// Get node info from this entry
    pub fn node_info(&self) -> crate::registry::node_info::NodeInfo {
        (self.node_info_fn)()
    }

    /// Get dependencies from this entry
    pub fn dependencies(&self) -> Vec<crate::graph::node_id::NodeId> {
        (self.dependencies_fn)()
    }
}

inventory::collect!(ComponentEntry);

#[cfg(test)]
mod tests {
    use super::*;

    struct TestType;

    impl crate::registry::registrable::Registrable for TestType {
        fn node_info() -> crate::registry::node_info::NodeInfo {
            crate::registry::node_info::NodeInfo {
                layer: crate::graph::layer::Layer::Domain,
                role: crate::graph::role::Role::Entity,
                type_name: "TestType",
                module_path: "test",
            }
        }

        fn dependencies() -> Vec<crate::graph::node_id::NodeId> {
            Vec::new()
        }
    }

    #[test]
    fn test_component_entry_creation() {
        let entry = ComponentEntry::new::<TestType>();
        let info = entry.node_info();
        assert_eq!(info.type_name, "TestType");
    }
}
