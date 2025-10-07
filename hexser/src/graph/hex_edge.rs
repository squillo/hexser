//! HexEdge represents a relationship between components in the graph.
//!
//! Edges are directed connections between nodes that represent dependencies,
//! implementations, data flow, or other relationships. Each edge has a
//! source node, target node, relationship type, and optional metadata.
//! Edges are immutable once created.
//!
//! Revision History
//! - 2025-10-01T00:03:00Z @AI: Initial HexEdge implementation for Phase 2.

/// Represents a directed edge between two nodes in the graph.
///
/// Edges capture relationships between components such as "implements",
/// "depends on", "transforms", etc. They are immutable and contain
/// metadata about the relationship.
///
/// # Example
///
/// ```rust
/// use hexser::graph::{HexEdge, NodeId, Relationship};
///
/// let edge = HexEdge::new(
///     NodeId::from_name("PostgresRepo"),
///     NodeId::from_name("UserRepository"),
///     Relationship::Implements,
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexEdge {
    /// Source node of this edge.
    pub source: crate::graph::node_id::NodeId,

    /// Target node of this edge.
    pub target: crate::graph::node_id::NodeId,

    /// Type of relationship this edge represents.
    pub relationship: crate::graph::relationship::Relationship,

    /// Additional metadata about this edge.
    pub metadata: std::collections::HashMap<String, String>,
}

impl HexEdge {
    /// Create a new HexEdge between two nodes.
    pub fn new(
        source: crate::graph::node_id::NodeId,
        target: crate::graph::node_id::NodeId,
        relationship: crate::graph::relationship::Relationship,
    ) -> Self {
        Self {
            source,
            target,
            relationship,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create a new HexEdge with metadata.
    pub fn with_metadata(
        source: crate::graph::node_id::NodeId,
        target: crate::graph::node_id::NodeId,
        relationship: crate::graph::relationship::Relationship,
        metadata: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            source,
            target,
            relationship,
            metadata,
        }
    }

    /// Get the source node ID.
    pub fn source(&self) -> &crate::graph::node_id::NodeId {
        &self.source
    }

    /// Get the target node ID.
    pub fn target(&self) -> &crate::graph::node_id::NodeId {
        &self.target
    }

    /// Get the relationship type.
    pub fn relationship(&self) -> crate::graph::relationship::Relationship {
        self.relationship
    }

    /// Get a metadata value by key.
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if this edge has a specific relationship type.
    pub fn has_relationship(&self, rel: crate::graph::relationship::Relationship) -> bool {
        self.relationship == rel
    }

    /// Check if this edge connects the given nodes (directional).
    pub fn connects(
        &self,
        from: &crate::graph::node_id::NodeId,
        to: &crate::graph::node_id::NodeId,
    ) -> bool {
        &self.source == from && &self.target == to
    }
}

impl std::fmt::Display for HexEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --[{}]--> {}",
            self.source,
            self.relationship,
            self.target
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_edge_creation() {
        let source = crate::graph::node_id::NodeId::from_name("Source");
        let target = crate::graph::node_id::NodeId::from_name("Target");

        let edge = HexEdge::new(
            source.clone(),
            target.clone(),
            crate::graph::relationship::Relationship::Depends,
        );

        assert_eq!(edge.source(), &source);
        assert_eq!(edge.target(), &target);
        assert_eq!(edge.relationship(), crate::graph::relationship::Relationship::Depends);
    }

    #[test]
    fn test_hex_edge_with_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(String::from("strength"), String::from("strong"));

        let edge = HexEdge::with_metadata(
            crate::graph::node_id::NodeId::from_name("A"),
            crate::graph::node_id::NodeId::from_name("B"),
            crate::graph::relationship::Relationship::Depends,
            metadata,
        );

        assert_eq!(edge.get_metadata("strength"), Some(&String::from("strong")));
    }

    #[test]
    fn test_edge_predicates() {
        let source = crate::graph::node_id::NodeId::from_name("Source");
        let target = crate::graph::node_id::NodeId::from_name("Target");

        let edge = HexEdge::new(
            source.clone(),
            target.clone(),
            crate::graph::relationship::Relationship::Implements,
        );

        assert!(edge.has_relationship(crate::graph::relationship::Relationship::Implements));
        assert!(!edge.has_relationship(crate::graph::relationship::Relationship::Depends));

        assert!(edge.connects(&source, &target));
        assert!(!edge.connects(&target, &source));
    }

    #[test]
    fn test_edge_display() {
        let edge = HexEdge::new(
            crate::graph::node_id::NodeId::from_name("A"),
            crate::graph::node_id::NodeId::from_name("B"),
            crate::graph::relationship::Relationship::Implements,
        );

        let display = format!("{}", edge);
        assert!(display.contains("Implements"));
        assert!(display.contains("-->"));
    }
}
