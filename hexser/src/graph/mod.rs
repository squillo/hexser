//! Graph module for hexagonal architecture introspection.
//!
//! This module provides the graph-based introspection system for analyzing
//! hexagonal architecture. It includes immutable graph structures, nodes,
//! edges, builders, and metadata. Future phases will add query APIs,
//! analysis algorithms, and visualization capabilities.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D6 — `finding` module and the `GraphFinding` re-export.
//! - 2025-10-01T00:03:00Z @AI: Phase 2 implementation with graph core.
//! - 2025-10-01T00:00:00Z @AI: Initial placeholder for graph module structure.

pub mod builder;
pub mod finding;
pub mod hex_edge;
pub mod hex_graph;
pub mod hex_node;
pub mod layer;
pub mod metadata;
pub mod node_id;
pub mod relationship;
pub mod role;
pub mod visualization;

pub use builder::GraphBuilder;
pub use finding::GraphFinding;
pub use hex_edge::HexEdge;
pub use hex_graph::HexGraph;
pub use hex_node::HexNode;
pub use layer::Layer;
pub use metadata::GraphMetadata;
pub use node_id::NodeId;
pub use relationship::Relationship;
pub use role::Role;
