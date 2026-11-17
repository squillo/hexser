//! GraphBuilder for constructing immutable graphs.
//!
//! GraphBuilder provides a fluent API for constructing HexGraph instances.
//! It accumulates nodes and edges, then builds an immutable graph.
//! The builder validates the graph structure and can return errors if
//! invalid relationships are detected.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D6 (F-A) — a collision is two registrations that differ in type name OR module path, so two template nodes with one short name in two modules are no longer merged in silence; each is recorded as a typed `GraphFinding::ShortNameCollision`, and the 0.6 string warning is rendered from it with both module paths.
//! - 2026-07-21T00:00:00Z @AI: PRD-272 §3.H — adjacency indices use IndexMap for deterministic iteration.
//! - 2026-07-20T00:00:00Z @AI: build() now uses BTreeMap, precomputes adjacency indices, and records NodeId collisions in metadata instead of silently dropping a node.
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
/// use hexser::graph::{GraphBuilder, HexNode, HexEdge, NodeId, Layer, Role, Relationship};
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
  /// Consumes the builder and returns a HexGraph, precomputing the outgoing/incoming adjacency
  /// indices so edge queries are O(degree). If two distinct registrations produce the same
  /// `NodeId` (two template nodes with one short name in two modules, or a hash collision),
  /// the later one is kept and the merge is recorded as a typed
  /// [`GraphFinding::ShortNameCollision`](crate::graph::finding::GraphFinding::ShortNameCollision),
  /// read through [`HexGraph::findings`](crate::graph::hex_graph::HexGraph::findings), with
  /// its words also written to the metadata warnings. One registration seen twice (same type
  /// name, same module) is not a collision.
  pub fn build(self) -> crate::graph::hex_graph::HexGraph {
    let mut node_map = std::collections::BTreeMap::new();
    let mut collisions: std::vec::Vec<crate::graph::finding::GraphFinding> = std::vec::Vec::new();

    for node in self.nodes {
      let id = *node.id();
      let type_name = std::string::String::from(node.type_name());
      let module_path = std::string::String::from(node.module_path());
      if let std::option::Option::Some(prev) = node_map.insert(id, node) {
        // Two registrations hashed to the same NodeId; keep the later one but record it so the
        // merge is observable instead of a silently missing component. Templates key by the
        // SHORT name, so the module path is what tells two of them apart (F-A).
        if prev.type_name() != type_name || prev.module_path() != module_path {
          collisions.push(crate::graph::finding::GraphFinding::ShortNameCollision {
            id,
            kept_type_name: type_name,
            kept_module_path: module_path,
            dropped_type_name: std::string::String::from(prev.type_name()),
            dropped_module_path: std::string::String::from(prev.module_path()),
          });
        }
      }
    }

    // Build adjacency indices. `IndexMap` (PRD-272 §3.H) keeps deterministic iteration order.
    let mut outgoing: indexmap::IndexMap<crate::graph::node_id::NodeId, std::vec::Vec<usize>> =
      indexmap::IndexMap::new();
    let mut incoming: indexmap::IndexMap<crate::graph::node_id::NodeId, std::vec::Vec<usize>> =
      indexmap::IndexMap::new();
    for (index, edge) in self.edges.iter().enumerate() {
      outgoing.entry(*edge.source()).or_default().push(index);
      incoming.entry(*edge.target()).or_default().push(index);
    }

    let mut metadata = crate::graph::metadata::GraphMetadata::new(&self.description);
    for collision in &collisions {
      if let crate::graph::finding::GraphFinding::ShortNameCollision {
        kept_type_name,
        kept_module_path,
        dropped_type_name,
        dropped_module_path,
        ..
      } = collision
      {
        metadata.add_warning(std::format!(
          "NodeId collision between `{}` and `{}`",
          qualified(dropped_module_path, dropped_type_name),
          qualified(kept_module_path, kept_type_name)
        ));
      }
    }

    let inner = std::sync::Arc::new(crate::graph::hex_graph::GraphInner {
      nodes: node_map,
      edges: self.edges,
      outgoing,
      incoming,
      metadata,
      collisions,
    });

    crate::graph::hex_graph::HexGraph { inner }
  }

  /// Validate the graph structure before building.
  ///
  /// Returns Ok(()) if valid, or an error describing the issue.
  pub fn validate(&self) -> crate::result::hex_result::HexResult<()> {
    let node_ids: std::collections::HashSet<_> = self.nodes.iter().map(|n| n.id()).collect();

    for edge in &self.edges {
      if !node_ids.contains(edge.source()) {
        return Err(
          crate::error::hex_error::Hexserror::domain(
            "E_HEX_GRAPH_001",
            "Edge references non-existent source node",
          )
          .with_next_step("Ensure all edge sources exist as nodes"),
        );
      }

      if !node_ids.contains(edge.target()) {
        return Err(
          crate::error::hex_error::Hexserror::domain(
            "E_HEX_GRAPH_002",
            "Edge references non-existent target node",
          )
          .with_next_step("Ensure all edge targets exist as nodes"),
        );
      }
    }

    Ok(())
  }

  /// Build the graph with validation.
  ///
  /// Returns an error if the graph structure is invalid.
  pub fn build_validated(
    self,
  ) -> crate::result::hex_result::HexResult<crate::graph::hex_graph::HexGraph> {
    self.validate()?;
    Ok(self.build())
  }
}

/// A registration's name as a reader needs it: a derive's type name is already a full path; a
/// template's is a short name, so its module path is put in front.
fn qualified(module_path: &str, type_name: &str) -> std::string::String {
  if type_name.contains("::") || module_path.is_empty() {
    std::string::String::from(type_name)
  } else {
    std::format!("{module_path}::{type_name}")
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

    let graph = GraphBuilder::new().with_node(node).build();

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

  /// why: when two distinct types map to the same NodeId, build() must keep one node AND record
  /// a metadata warning, rather than silently dropping a component with no trace. Forced here by
  /// giving two nodes with different type_names the same explicit NodeId.
  #[test]
  fn test_build_records_nodeid_collision_warning() {
    let shared_id = crate::graph::node_id::NodeId::from_name("Shared");
    let node_a = crate::graph::hex_node::HexNode::new(
      shared_id,
      crate::graph::layer::Layer::Domain,
      crate::graph::role::Role::Entity,
      "TypeA",
      "domain",
    );
    let node_b = crate::graph::hex_node::HexNode::new(
      shared_id,
      crate::graph::layer::Layer::Domain,
      crate::graph::role::Role::Entity,
      "TypeB",
      "domain",
    );

    let graph = GraphBuilder::new().with_nodes(vec![node_a, node_b]).build();

    // Only one survives the id collision, but the collision is observable in metadata.
    assert_eq!(graph.node_count(), 1);
    let warnings = graph.metadata().warnings();
    assert_eq!(warnings.len(), 1, "collision must be recorded once");
    assert!(warnings[0].contains("TypeA") && warnings[0].contains("TypeB"));
  }

  /// why: identical type_names sharing an id (the same component re-registered) is NOT a real
  /// collision and must not produce a spurious warning.
  #[test]
  fn test_build_no_warning_for_same_type_reinsertion() {
    let id = crate::graph::node_id::NodeId::from_name("Same");
    let mk = || {
      crate::graph::hex_node::HexNode::new(
        id,
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "SameType",
        "domain",
      )
    };
    let graph = GraphBuilder::new().with_nodes(vec![mk(), mk()]).build();
    assert_eq!(graph.node_count(), 1);
    assert!(graph.metadata().warnings().is_empty());
  }
}
