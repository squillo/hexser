//! Metadata types for graph components.
//!
//! This module provides metadata structures for storing additional information
//! about graph components. Metadata is stored as key-value pairs and can be
//! attached to nodes, edges, or the graph itself. Metadata is immutable and
//! copied when graphs are constructed.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: current_timestamp no longer unwraps duration_since (best-effort 0 on a pre-epoch clock) — removes a panic path from the universal construction path.
//! - 2026-07-20T00:00:00Z @AI: Add add_warning/warnings to record non-fatal construction warnings (e.g. NodeId collisions) in attributes.
//! - 2025-10-01T00:03:00Z @AI: Initial metadata types for Phase 2.

/// Metadata for the entire graph.
///
/// Contains information about the graph as a whole, such as when it was
/// created, version information, and custom attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphMetadata {
  /// When this graph was created (Unix timestamp).
  pub created_at: u64,

  /// Version identifier for this graph.
  pub version: u64,

  /// Description of this graph.
  pub description: String,

  /// Additional custom metadata.
  pub attributes: std::collections::HashMap<String, String>,
}

impl GraphMetadata {
  /// Create new graph metadata with current timestamp.
  pub fn new(description: &str) -> Self {
    Self {
      created_at: Self::current_timestamp(),
      version: 1,
      description: String::from(description),
      attributes: std::collections::HashMap::new(),
    }
  }

  /// Create graph metadata with specific version.
  pub fn with_version(description: &str, version: u64) -> Self {
    Self {
      created_at: Self::current_timestamp(),
      version,
      description: String::from(description),
      attributes: std::collections::HashMap::new(),
    }
  }

  /// Get current Unix timestamp (best-effort; returns 0 if the clock predates the epoch rather
  /// than panicking, since this runs on the universal graph-construction path).
  fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_secs())
      .unwrap_or(0)
  }

  /// Get an attribute value.
  pub fn get_attribute(&self, key: &str) -> Option<&String> {
    self.attributes.get(key)
  }

  /// Record a non-fatal construction warning (stored in `attributes` under a `warning.N` key).
  ///
  /// Used, for example, to surface a `NodeId` hash collision during graph construction rather
  /// than silently dropping one of the colliding components.
  pub fn add_warning(&mut self, message: String) {
    let index = self
      .attributes
      .keys()
      .filter(|k| k.starts_with("warning."))
      .count();
    self
      .attributes
      .insert(std::format!("warning.{index}"), message);
  }

  /// Collect the construction warnings recorded via [`Self::add_warning`], in insertion order.
  pub fn warnings(&self) -> std::vec::Vec<&String> {
    let mut keyed: std::vec::Vec<(usize, &String)> = self
      .attributes
      .iter()
      .filter_map(|(k, v)| {
        k.strip_prefix("warning.")
          .and_then(|n| n.parse::<usize>().ok())
          .map(|n| (n, v))
      })
      .collect();
    keyed.sort_by_key(|(n, _)| *n);
    keyed.into_iter().map(|(_, v)| v).collect()
  }
}

impl Default for GraphMetadata {
  fn default() -> Self {
    Self::new("Hexagonal Architecture Graph")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_graph_metadata_creation() {
    let metadata = GraphMetadata::new("Test Graph");
    assert_eq!(metadata.description, "Test Graph");
    assert_eq!(metadata.version, 1);
    assert!(metadata.created_at > 0);
  }

  #[test]
  fn test_graph_metadata_with_version() {
    let metadata = GraphMetadata::with_version("Test", 42);
    assert_eq!(metadata.version, 42);
  }

  #[test]
  fn test_graph_metadata_default() {
    let metadata = GraphMetadata::default();
    assert!(metadata.description.contains("Hexagonal"));
  }

  /// why: warnings recorded via add_warning must be retrievable in insertion order, so
  /// construction diagnostics (e.g. NodeId collisions) survive on the built graph.
  #[test]
  fn test_add_and_read_warnings_in_order() {
    let mut metadata = GraphMetadata::new("Test");
    assert!(metadata.warnings().is_empty());
    metadata.add_warning(String::from("first"));
    metadata.add_warning(String::from("second"));
    let warnings = metadata.warnings();
    assert_eq!(warnings.len(), 2);
    assert_eq!(warnings[0], "first");
    assert_eq!(warnings[1], "second");
  }
}
