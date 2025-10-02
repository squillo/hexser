//! Inspectable trait for components that support introspection.
//!
//! This trait enables components to expose their internal structure and
//! relationships for analysis, debugging, and visualization. It provides
//! access to dependencies, metadata, and architectural context.
//!
//! Revision History
//! - 2025-10-02T12:30:00Z @AI: Remove duplicate implementations, simplify trait.
//! - 2025-10-02T12:00:00Z @AI: Initial Inspectable trait implementation.

/// Trait for components that support introspection.
///
/// Provides methods for examining component structure, dependencies,
/// and architectural context.
///
/// # Example
///
/// ```rust
/// use hex::showcase::Inspectable;
/// use hex::graph::{HexNode, NodeId, Layer, Role};
///
/// let node = HexNode::new(
///     NodeId::from_name("Test"),
///     Layer::Domain,
///     Role::Entity,
///     "TestEntity",
///     "test::domain",
/// );
///
/// // Inspect the node
/// let layer_info = node.layer_info();
/// assert_eq!(layer_info.layer, Layer::Domain);
/// ```
pub trait Inspectable {
    /// Returns the layer this component belongs to.
    fn layer_info(&self) -> LayerInfo;

    /// Returns a list of component IDs this depends on.
    fn dependencies(&self) -> Vec<crate::graph::node_id::NodeId>;

    /// Returns a list of component IDs that depend on this.
    fn dependents(&self) -> Vec<crate::graph::node_id::NodeId>;
}

/// Information about a component's layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerInfo {
    /// The architectural layer.
    pub layer: crate::graph::layer::Layer,

    /// The role within the layer.
    pub role: crate::graph::role::Role,

    /// Type name of the component.
    pub type_name: String,
}

impl Inspectable for crate::graph::hex_node::HexNode {
    fn layer_info(&self) -> LayerInfo {
        LayerInfo {
            layer: self.layer(),
            role: self.role(),
            type_name: String::from(self.type_name()),
        }
    }

    fn dependencies(&self) -> Vec<crate::graph::node_id::NodeId> {
        Vec::new()
    }

    fn dependents(&self) -> Vec<crate::graph::node_id::NodeId> {
        Vec::new()
    }
}

impl Inspectable for crate::graph::hex_graph::HexGraph {
    fn layer_info(&self) -> LayerInfo {
        LayerInfo {
            layer: crate::graph::layer::Layer::Unknown,
            role: crate::graph::role::Role::Unknown,
            type_name: String::from("HexGraph"),
        }
    }

    fn dependencies(&self) -> Vec<crate::graph::node_id::NodeId> {
        Vec::new()
    }

    fn dependents(&self) -> Vec<crate::graph::node_id::NodeId> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_inspectable() {
        let node = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("Test"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
        );

        let layer_info = node.layer_info();
        assert_eq!(layer_info.layer, crate::graph::layer::Layer::Domain);
        assert_eq!(layer_info.role, crate::graph::role::Role::Entity);
        assert_eq!(layer_info.type_name, "TestEntity");

        let deps = node.dependencies();
        assert_eq!(deps.len(), 0);
    }

    #[test]
    fn test_graph_inspectable() {
        let graph = crate::graph::hex_graph::HexGraph::new();
        let layer_info = graph.layer_info();
        assert_eq!(layer_info.type_name, "HexGraph");
    }
}
