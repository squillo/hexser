//! Graph module for hexagonal architecture introspection.
//!
//! This module provides the graph-based introspection system for analyzing
//! hexagonal architecture. It includes immutable graph structures, nodes,
//! edges, builders, and metadata. Future phases will add query APIs,
//! analysis algorithms, and visualization capabilities.
//!
//! Revision History
//! - 2025-10-01T00:03:00Z @AI: Phase 2 implementation with graph core.
//! - 2025-10-01T00:00:00Z @AI: Initial placeholder for graph module structure.

pub mod layer;
pub mod role;
pub mod relationship;
pub mod node_id;
pub mod hex_node;
pub mod hex_edge;
pub mod metadata;
pub mod hex_graph;
pub mod builder;

pub use layer::Layer;
pub use role::Role;
pub use relationship::Relationship;
pub use node_id::NodeId;
pub use hex_node::HexNode;
pub use hex_edge::HexEdge;
pub use metadata::GraphMetadata;
pub use hex_graph::HexGraph;
pub use builder::GraphBuilder;
