//! Describable trait for components that can describe themselves.
//!
//! This trait enables components to provide human-readable descriptions
//! of their purpose and structure. It's primarily used for documentation,
//! debugging, and visualization purposes.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Update ArcGraphExt to use renamed query methods.
//! - 2025-10-02T12:30:00Z @AI: Remove duplicate implementations, simplify trait.
//! - 2025-10-02T12:00:00Z @AI: Initial Describable trait implementation.

/// Trait for components that can describe themselves.
///
/// Provides methods for self-description useful in documentation,
/// debugging, and visualization scenarios.
///
/// # Example
///
/// ```rust
/// use hex::showcase::Describable;
///
/// struct MyComponent {
///     name: String,
/// }
///
/// impl Describable for MyComponent {
///     fn describe(&self) -> String {
///         format!("Component: {}", self.name)
///     }
///
///     fn short_name(&self) -> &str {
///         &self.name
///     }
/// }
/// ```
pub trait Describable {
    /// Returns a detailed description of this component.
    fn describe(&self) -> String;

    /// Returns a short name for this component.
    fn short_name(&self) -> &str;
}

impl Describable for crate::graph::hex_node::HexNode {
    fn describe(&self) -> String {
        format!(
            "{} ({} in {}) - defined in {}",
            self.type_name(),
            self.role(),
            self.layer(),
            self.module_path()
        )
    }

    fn short_name(&self) -> &str {
        self.type_name()
    }
}

impl Describable for crate::graph::hex_graph::HexGraph {
    fn describe(&self) -> String {
        format!(
            "Hexagonal Architecture Graph with {} nodes and {} edges",
            self.node_count(),
            self.edge_count()
        )
    }

    fn short_name(&self) -> &str {
        "HexGraph"
    }
}

impl Describable for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
    fn describe(&self) -> String {
        (**self).describe()
    }

    fn short_name(&self) -> &str {
        (**self).short_name()
    }
}

/// Extension trait for pretty printing
pub trait PrettyPrint {
    fn pretty_print(&self);
}

impl PrettyPrint for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
    fn pretty_print(&self) {
        println!("{}", self.describe());
    }
}

impl PrettyPrint for crate::graph::hex_graph::HexGraph {
    fn pretty_print(&self) {
        println!("{}", self.describe());
    }
}

/// Extension methods for Arc<HexGraph> to provide graph query capabilities
pub trait ArcGraphExt {
    fn nodes_by_layer(&self, layer: crate::graph::layer::Layer)
        -> Vec<&crate::graph::hex_node::HexNode>;

    fn nodes_by_role(&self, role: crate::graph::role::Role)
        -> Vec<&crate::graph::hex_node::HexNode>;
}

impl ArcGraphExt for std::sync::Arc<crate::graph::hex_graph::HexGraph> {
    fn nodes_by_layer(&self, layer: crate::graph::layer::Layer)
        -> Vec<&crate::graph::hex_node::HexNode> {
        (**self).nodes_by_layer(layer)
    }

    fn nodes_by_role(&self, role: crate::graph::role::Role)
        -> Vec<&crate::graph::hex_node::HexNode> {
        (**self).nodes_by_role(role)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_describable() {
        let node = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("Test"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "test::domain",
        );

        let description = node.describe();
        assert!(description.contains("TestEntity"));
        assert!(description.contains("Entity"));
        assert!(description.contains("Domain"));
        assert_eq!(node.short_name(), "TestEntity");
    }

    #[test]
    fn test_graph_describable() {
        let graph = crate::graph::hex_graph::HexGraph::new();
        let description = graph.describe();
        assert!(description.contains("0 nodes"));
        assert!(description.contains("0 edges"));
        assert_eq!(graph.short_name(), "HexGraph");
    }
}
