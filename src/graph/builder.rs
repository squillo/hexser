//! GraphBuilder for constructing immutable graphs.
//!
//! GraphBuilder provides a fluent API for constructing HexGraph instances.
//! It accumulates nodes and edges, then builds an immutable graph.
//! The builder validates the graph structure and can return errors if
//! invalid relationships are detected.
//!
//! Revision History
//! - 2025-10-02T12:30:00Z @AI: Add add_node and add_edge alias methods.
//! - 2025-10-01T00:03:00Z @AI: Initial GraphBuilder implementation for Phase 2.

/// Builder for constructing HexGraph instances.
///
/// Provides a fluent API for adding nodes and edges, then building
/// an immutable graph. The builder is consumed when build() is called.
///
/// # Example
///
/// ```rust
/// use hex::graph::{GraphBuilder, HexNode, HexEdge, NodeId, Layer, Role, Relationship};
///
/// let graph = GraphBuilder::new()
///     .with_description("My Architecture")
///     .with_node(HexNode::new(
///         NodeId::from_name("Entity"),
///         Layer::Domain,
///         Role::Entity,
///         "MyEntity",
///         "domain",
///     ))
///     .build();
/// ```
pub struct GraphBuilder {
    nodes: Vec<crate::graph::hex_node::HexNode>,
    edges: Vec<crate::graph::hex_edge::HexEdge>,
    description: String,
}

impl GraphBuilder {
    /// Create a new empty graph builder.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            description: String::from("Hexagonal Architecture Graph"),
        }
    }

    /// Set the graph description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = String::from(description);
        self
    }

    /// Add a node to the graph.
    pub fn with_node(mut self, node: crate::graph::hex_node::HexNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Add a node to the graph (alias for with_node).
    pub fn add_node(self, node: crate::graph::hex_node::HexNode) -> Self {
        self.with_node(node)
    }

    /// Add multiple nodes to the graph.
    pub fn with_nodes(mut self, nodes: Vec<crate::graph::hex_node::HexNode>) -> Self {
        self.nodes.extend(nodes);
        self
    }

    /// Add an edge to the graph.
    pub fn with_edge(mut self, edge: crate::graph::hex_edge::HexEdge) -> Self {
        self.edges.push(edge);
        self
    }

    /// Add an edge to the graph (alias for with_edge).
    pub fn add_edge(self, edge: crate::graph::hex_edge::HexEdge) -> Self {
        self.with_edge(edge)
    }

    /// Add multiple edges to the graph.
    pub fn with_edges(mut self, edges: Vec<crate::graph::hex_edge::HexEdge>) -> Self {
        self.edges.extend(edges);
        self
    }

    /// Build the immutable graph.
    ///
    /// Consumes the builder and returns a HexGraph. Validates that all
    /// edges reference existing nodes.
    pub fn build(self) -> crate::graph::hex_graph::HexGraph {
        let mut node_map = std::collections::HashMap::new();

        for node in self.nodes {
            node_map.insert(node.id().clone(), node);
        }

        let metadata = crate::graph::metadata::GraphMetadata::new(&self.description);

        let inner = std::sync::Arc::new(crate::graph::hex_graph::GraphInner {
            nodes: node_map,
            edges: self.edges,
            metadata,
        });

        crate::graph::hex_graph::HexGraph { inner }
    }

    /// Validate the graph structure before building.
    ///
    /// Returns Ok(()) if valid, or an error describing the issue.
    pub fn validate(&self) -> crate::result::hex_result::HexResult<()> {
        let node_ids: std::collections::HashSet<_> = self.nodes.iter()
            .map(|n| n.id())
            .collect();

        for edge in &self.edges {
            if !node_ids.contains(edge.source()) {
                return Err(crate::error::hex_error::HexError::domain(
                    "E_HEX_GRAPH_001",
                    "Edge references non-existent source node"
                )
                .with_next_step("Ensure all edge sources exist as nodes"));
            }

            if !node_ids.contains(edge.target()) {
                return Err(crate::error::hex_error::HexError::domain(
                    "E_HEX_GRAPH_002",
                    "Edge references non-existent target node"
                )
                .with_next_step("Ensure all edge targets exist as nodes"));
            }
        }

        Ok(())
    }

    /// Build the graph with validation.
    ///
    /// Returns an error if the graph structure is invalid.
    pub fn build_validated(self) -> crate::result::hex_result::HexResult<crate::graph::hex_graph::HexGraph> {
        self.validate()?;
        Ok(self.build())
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_empty() {
        let graph = GraphBuilder::new().build();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_builder_with_nodes() {
        let node = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("Test"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "TestEntity",
            "domain",
        );

        let graph = GraphBuilder::new()
            .with_node(node)
            .build();

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_builder_with_edges() {
        let node1 = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("A"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "A",
            "domain",
        );

        let node2 = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("B"),
            crate::graph::layer::Layer::Port,
            crate::graph::role::Role::Repository,
            "B",
            "ports",
        );

        let edge = crate::graph::hex_edge::HexEdge::new(
            crate::graph::node_id::NodeId::from_name("A"),
            crate::graph::node_id::NodeId::from_name("B"),
            crate::graph::relationship::Relationship::Depends,
        );

        let graph = GraphBuilder::new()
            .with_nodes(vec![node1, node2])
            .with_edge(edge)
            .build();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_builder_validation_success() {
        let node = crate::graph::hex_node::HexNode::new(
            crate::graph::node_id::NodeId::from_name("A"),
            crate::graph::layer::Layer::Domain,
            crate::graph::role::Role::Entity,
            "A",
            "domain",
        );

        let builder = GraphBuilder::new().with_node(node);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_builder_validation_fails_missing_source() {
        let edge = crate::graph::hex_edge::HexEdge::new(
            crate::graph::node_id::NodeId::from_name("Missing"),
            crate::graph::node_id::NodeId::from_name("Also Missing"),
            crate::graph::relationship::Relationship::Depends,
        );

        let builder = GraphBuilder::new().with_edge(edge);
        assert!(builder.validate().is_err());
    }
}
