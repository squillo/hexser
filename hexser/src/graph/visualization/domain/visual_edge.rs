//! Visual edge domain model.
//!
//! Represents an edge in the visual graph.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial VisualEdge implementation.

/// Visual edge
#[derive(Clone, Debug)]
pub struct VisualEdge {
  pub source: String,
  pub target: String,
  pub relationship: String,
}

impl VisualEdge {
  /// Create from HexEdge
  pub fn from_hex_edge(edge: &crate::graph::hex_edge::HexEdge) -> Self {
    Self {
      source: edge.source.to_string(),
      target: edge.target.to_string(),
      relationship: format!("{:?}", edge.relationship),
    }
  }
}
