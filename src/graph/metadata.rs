//! Metadata types for graph components.
//!
//! This module provides metadata structures for storing additional information
//! about graph components. Metadata is stored as key-value pairs and can be
//! attached to nodes, edges, or the graph itself. Metadata is immutable and
//! copied when graphs are constructed.
//!
//! Revision History
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

    /// Get current Unix timestamp.
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Get an attribute value.
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
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
}
