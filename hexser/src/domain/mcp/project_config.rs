//! Project configuration for multi-project MCP support.
//!
//! Defines ProjectConfig which represents a single project's metadata and
//! associated HexGraph for architecture introspection. Used by ProjectRegistry
//! to manage multiple projects simultaneously.
//!
//! Revision History
//! - 2025-10-10T18:37:00Z @AI: Initial implementation for multi-project MCP support.

/// Configuration for a single project in the MCP server.
///
/// Each ProjectConfig contains a unique identifier, filesystem path,
/// and architecture graph for that project.
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ProjectConfig {
  /// Unique project identifier (workspace member name or custom ID)
  pub name: std::string::String,

  /// Root directory path for the project
  pub root_path: std::path::PathBuf,

  /// Architecture graph for this project
  pub graph: std::sync::Arc<crate::graph::hex_graph::HexGraph>,
}

impl ProjectConfig {
  /// Creates a new ProjectConfig.
  ///
  /// # Arguments
  ///
  /// * `name` - Unique project identifier
  /// * `root_path` - Project root directory
  /// * `graph` - Architecture graph for the project
  ///
  /// # Returns
  ///
  /// New ProjectConfig instance
  pub fn new(
    name: std::string::String,
    root_path: std::path::PathBuf,
    graph: std::sync::Arc<crate::graph::hex_graph::HexGraph>,
  ) -> Self {
    Self {
      name,
      root_path,
      graph,
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_project_config_creation() {
    // Test: Validates ProjectConfig can be created with basic fields
    // Justification: Core functionality for multi-project support
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let config = super::ProjectConfig::new(
      std::string::String::from("test_project"),
      std::path::PathBuf::from("/path/to/project"),
      std::sync::Arc::new(graph),
    );

    std::assert_eq!(config.name, "test_project");
    std::assert_eq!(
      config.root_path,
      std::path::PathBuf::from("/path/to/project")
    );
  }

  #[test]
  fn test_project_config_clone() {
    // Test: Validates ProjectConfig implements Clone
    // Justification: Required for sharing configs across threads
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let config = super::ProjectConfig::new(
      std::string::String::from("test"),
      std::path::PathBuf::from("/path"),
      std::sync::Arc::new(graph),
    );

    let cloned = config.clone();
    std::assert_eq!(cloned.name, config.name);
  }
}
