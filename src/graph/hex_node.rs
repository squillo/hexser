//! HexNode represents a component in the hexagonal architecture graph.
//!
//! Each node represents a single component (entity, port, adapter, etc.)
//! with its associated metadata including layer, role, type information,
//! and custom metadata. Nodes are immutable once created and are identified
//! by their unique NodeId.
//!
//! Revision History
//! - 2025-10-01T00:03:00Z @AI: Initial HexNode implementation for Phase 2.

/// Represents a component node in the hexagonal architecture graph.
///
/// Nodes are immutable and contain all metadata about a component including
/// its architectural layer, role, type information, and custom metadata.
///
/// # Example
///
/// ```rust
/// use hex::graph::{HexNode, NodeId, Layer, Role};
///
/// let node = HexNode::new(
///     NodeId::from_name("MyEntity"),
///     Layer::Domain,
///     Role::Entity,
///     "MyEntity",
///     "my_crate::domain",
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexNode {
    /// Unique identifier for this node.
    pub id: crate::graph::node_id::NodeId,

    /// Architectural layer this component belongs to.
    pub layer: crate::graph::layer::Layer,

    /// Role this component plays within its layer.
    pub role: crate::graph::role::Role,

    /// Type name of the component (e.g., "UserRepository").
    pub type_name: String,

    /// Module path where the component is defined.
    pub module_path: String,

    /// Additional metadata about this node.
    pub metadata: std::collections::HashMap<String, String>,
}

impl HexNode {
    /// Create a new HexNode with the specified properties.
    pub fn new(
        id: crate::graph::node_id::NodeId,
        layer: crate::graph::layer::Layer,
        role: crate::graph::role::Role,
        type_name: &str,
        module_path: &str,
    ) -> Self {
        Self {
            id,
            layer,
            role,
            type_name: String::from(type_name),
            module_path: String::from(module_path),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a new HexNode with metadata.
    pub fn with_metadata(
        id: crate::graph::node_id::NodeId,
        layer: crate::graph::layer::Layer,
        role: crate::graph::role::Role,
        type_name: &str,
        module_path: &str,
        metadata: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            layer,
            role,
            type_name: String::from(type_name),
            module_path: String::from(module_path),
            metadata,
        }
    }

    /// Get the node's unique identifier.
    pub fn id(&self) -> &crate::graph::node_id::NodeId {
        &self.id
    }

    /// Get the architectural layer.
    pub fn layer(&self) -> crate::graph::layer::Layer {
        self.layer
    }

    /// Get the component role.
    pub fn role(&self) -> crate::graph::role::Role {
        self.role
    }

    /// Get the type name.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Get the module path.
    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    /// Get a metadata value by key.
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if this node is in a specific layer.
    pub fn is_in_layer(&self, layer: crate::graph::layer::Layer) -> bool {
        self.layer == layer
    }

    /// Check if this node has a specific role.
    pub fn has_role(&self, role: crate::graph::role::Role) -> bool {
        self.role == role
    }
}

impl std::fmt::Display for HexNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}::{} ({} in {})",
            self.type_name,
            self.role,
            self.layer,
            self.module_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_node_creation() {
        let node = HexNode::new(
            crate::graph::node_id::NodeId::from_name("TestEntity"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
        );

        assert_eq!(node.type_name(), "TestEntity");
        assert_eq!(node.module_path(), "test::domain");
        assert_eq!(node.layer(), crate::graph::layer::Layer::Domain);
        assert_eq!(node.role(), crate::graph::role::Role::Entity);
    }

    #[test]
    fn test_hex_node_with_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(String::from("version"), String::from("1.0"));

        let node = HexNode::with_metadata(
            crate::graph::node_id::NodeId::from_name("TestEntity"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
            metadata,
        );

        assert_eq!(node.get_metadata("version"), Some(&String::from("1.0")));
        assert_eq!(node.get_metadata("missing"), None);
    }

    #[test]
    fn test_node_predicates() {
        let node = HexNode::new(
            crate::graph::node_id::NodeId::from_name("TestEntity"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
        );

        assert!(node.is_in_layer(crate::graph::layer::Layer::Domain));
        assert!(!node.is_in_layer(crate::graph::layer::Layer::Port));

        assert!(node.has_role(crate::graph::role::Role::Entity));
        assert!(!node.has_role(crate::graph::role::Role::Repository));
    }

    #[test]
    fn test_node_display() {
        let node = HexNode::new(
            crate::graph::node_id::NodeId::from_name("TestEntity"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
        );

        let display = format!("{}", node);
        assert!(display.contains("TestEntity"));
        assert!(display.contains("Entity"));
        assert!(display.contains("Domain"));
    }
}
