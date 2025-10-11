//! Project registry for managing multiple projects in MCP server.
//!
//! ProjectRegistry maintains a collection of ProjectConfig instances, enabling
//! the MCP server to serve architecture data for multiple projects simultaneously.
//! Supports lookup by project name and iteration over all registered projects.
//!
//! Revision History
//! - 2025-10-10T18:37:00Z @AI: Initial implementation for multi-project MCP support.

/// Registry managing multiple project configurations.
///
/// ProjectRegistry provides centralized access to all projects available
/// in the MCP server, enabling multi-project architecture queries.
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ProjectRegistry {
  projects: std::collections::HashMap<
    std::string::String,
    crate::domain::mcp::project_config::ProjectConfig,
  >,
}

impl ProjectRegistry {
  /// Creates a new empty ProjectRegistry.
  ///
  /// # Returns
  ///
  /// New ProjectRegistry with no projects
  pub fn new() -> Self {
    Self {
      projects: std::collections::HashMap::new(),
    }
  }

  /// Registers a project in the registry.
  ///
  /// # Arguments
  ///
  /// * `config` - ProjectConfig to register
  ///
  /// # Returns
  ///
  /// Previous ProjectConfig with same name, if any
  pub fn register(
    &mut self,
    config: crate::domain::mcp::project_config::ProjectConfig,
  ) -> std::option::Option<crate::domain::mcp::project_config::ProjectConfig> {
    self.projects.insert(config.name.clone(), config)
  }

  /// Retrieves a project by name.
  ///
  /// # Arguments
  ///
  /// * `name` - Project identifier
  ///
  /// # Returns
  ///
  /// Reference to ProjectConfig if found
  pub fn get(
    &self,
    name: &str,
  ) -> std::option::Option<&crate::domain::mcp::project_config::ProjectConfig> {
    self.projects.get(name)
  }

  /// Returns all registered project names.
  ///
  /// # Returns
  ///
  /// Vector of project names
  pub fn project_names(&self) -> std::vec::Vec<std::string::String> {
    self.projects.keys().cloned().collect()
  }

  /// Returns number of registered projects.
  ///
  /// # Returns
  ///
  /// Count of projects
  pub fn len(&self) -> usize {
    self.projects.len()
  }

  /// Checks if registry is empty.
  ///
  /// # Returns
  ///
  /// True if no projects registered
  pub fn is_empty(&self) -> bool {
    self.projects.is_empty()
  }

  /// Iterates over all projects.
  ///
  /// # Returns
  ///
  /// Iterator over (name, ProjectConfig) pairs
  pub fn iter(
    &self,
  ) -> impl std::iter::Iterator<
    Item = (
      &std::string::String,
      &crate::domain::mcp::project_config::ProjectConfig,
    ),
  > {
    self.projects.iter()
  }

  /// Creates registry with current HexGraph as single project.
  ///
  /// Provides backward compatibility with single-project mode.
  /// Uses "hexser" as default project name.
  ///
  /// # Returns
  ///
  /// ProjectRegistry with current graph as single project
  pub fn from_current_graph() -> Self {
    let mut registry = Self::new();
    let graph = crate::graph::hex_graph::HexGraph::current();
    let config = crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("hexser"),
      std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
      graph,
    );
    registry.register(config);
    registry
  }
}

impl std::default::Default for ProjectRegistry {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_registry_creation() {
    // Test: Validates empty registry creation
    // Justification: Basic registry initialization
    let registry = super::ProjectRegistry::new();
    std::assert_eq!(registry.len(), 0);
    std::assert!(registry.is_empty());
  }

  #[test]
  fn test_registry_register_and_get() {
    // Test: Validates project registration and retrieval
    // Justification: Core registry functionality
    let mut registry = super::ProjectRegistry::new();
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let config = crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("test_project"),
      std::path::PathBuf::from("/path"),
      std::sync::Arc::new(graph),
    );

    registry.register(config);
    std::assert_eq!(registry.len(), 1);
    std::assert!(!registry.is_empty());

    let retrieved = registry.get("test_project");
    std::assert!(retrieved.is_some());
    std::assert_eq!(retrieved.unwrap().name, "test_project");
  }

  #[test]
  fn test_registry_get_nonexistent() {
    // Test: Validates retrieval of non-existent project returns None
    // Justification: Error handling verification
    let registry = super::ProjectRegistry::new();
    std::assert!(registry.get("nonexistent").is_none());
  }

  #[test]
  fn test_registry_project_names() {
    // Test: Validates project name listing
    // Justification: Required for resource enumeration
    let mut registry = super::ProjectRegistry::new();
    let graph = crate::graph::builder::GraphBuilder::new().build();

    registry.register(crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("project1"),
      std::path::PathBuf::from("/path1"),
      std::sync::Arc::new(graph.clone()),
    ));
    registry.register(crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("project2"),
      std::path::PathBuf::from("/path2"),
      std::sync::Arc::new(graph),
    ));

    let names = registry.project_names();
    std::assert_eq!(names.len(), 2);
    std::assert!(names.contains(&std::string::String::from("project1")));
    std::assert!(names.contains(&std::string::String::from("project2")));
  }

  #[test]
  fn test_registry_from_current_graph() {
    // Test: Validates backward compatibility with single-project mode
    // Justification: Ensures existing code continues to work
    let registry = super::ProjectRegistry::from_current_graph();
    std::assert_eq!(registry.len(), 1);
    std::assert!(registry.get("hexser").is_some());
  }

  #[test]
  fn test_registry_replace_project() {
    // Test: Validates replacing existing project returns old config
    // Justification: Verifies update semantics
    let mut registry = super::ProjectRegistry::new();
    let graph1 = crate::graph::builder::GraphBuilder::new().build();
    let graph2 = crate::graph::builder::GraphBuilder::new().build();

    let config1 = crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("test"),
      std::path::PathBuf::from("/path1"),
      std::sync::Arc::new(graph1),
    );
    let config2 = crate::domain::mcp::project_config::ProjectConfig::new(
      std::string::String::from("test"),
      std::path::PathBuf::from("/path2"),
      std::sync::Arc::new(graph2),
    );

    let old = registry.register(config1);
    std::assert!(old.is_none());

    let old = registry.register(config2);
    std::assert!(old.is_some());
    std::assert_eq!(old.unwrap().root_path, std::path::PathBuf::from("/path1"));
  }
}
