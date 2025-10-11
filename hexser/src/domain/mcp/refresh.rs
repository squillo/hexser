//! Refresh request and response types for MCP server.
//!
//! Defines types for the hexser/refresh method which triggers recompilation
//! and reloads the architecture graph by restarting the MCP server process.
//!
//! Revision History
//! - 2025-10-10T19:48:00Z @AI: Initial implementation of refresh domain types.

/// Request to refresh a project's architecture graph.
///
/// Triggers recompilation and server restart to clear old inventory cache.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RefreshRequest {
  /// Name of the project to refresh
  pub project: std::string::String,
}

/// Result of a refresh operation.
///
/// Contains compilation status and component change statistics.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RefreshResult {
  /// Status of the refresh operation ("success", "error", "compiling")
  pub status: std::string::String,

  /// Whether compilation was successful
  pub compiled: bool,

  /// Number of components added in new graph (0 if restart pending)
  pub components_added: usize,

  /// Number of components removed in new graph (0 if restart pending)
  pub components_removed: usize,

  /// Error message if compilation failed
  pub error: std::option::Option<std::string::String>,
}

impl RefreshResult {
  /// Creates a successful refresh result.
  pub fn success(added: usize, removed: usize) -> Self {
    Self {
      status: std::string::String::from("success"),
      compiled: true,
      components_added: added,
      components_removed: removed,
      error: std::option::Option::None,
    }
  }

  /// Creates a compilation error result.
  pub fn compilation_error(error_msg: std::string::String) -> Self {
    Self {
      status: std::string::String::from("error"),
      compiled: false,
      components_added: 0,
      components_removed: 0,
      error: std::option::Option::Some(error_msg),
    }
  }

  /// Creates a pending restart result.
  pub fn restart_required() -> Self {
    Self {
      status: std::string::String::from("restart_required"),
      compiled: true,
      components_added: 0,
      components_removed: 0,
      error: std::option::Option::Some(std::string::String::from(
        "Compilation successful. Server restart required to load new graph.",
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_refresh_request_creation() {
    // Test: Validates RefreshRequest can be created
    // Justification: Basic type validation
    let request = super::RefreshRequest {
      project: std::string::String::from("test_project"),
    };
    std::assert_eq!(request.project, "test_project");
  }

  #[test]
  fn test_refresh_result_success() {
    // Test: Validates successful refresh result
    // Justification: Verifies success case construction
    let result = super::RefreshResult::success(3, 1);
    std::assert_eq!(result.status, "success");
    std::assert!(result.compiled);
    std::assert_eq!(result.components_added, 3);
    std::assert_eq!(result.components_removed, 1);
    std::assert!(result.error.is_none());
  }

  #[test]
  fn test_refresh_result_compilation_error() {
    // Test: Validates compilation error result
    // Justification: Verifies error case construction
    let result = super::RefreshResult::compilation_error(std::string::String::from("syntax error"));
    std::assert_eq!(result.status, "error");
    std::assert!(!result.compiled);
    std::assert!(result.error.is_some());
  }

  #[test]
  fn test_refresh_result_restart_required() {
    // Test: Validates restart required result
    // Justification: Verifies restart case construction
    let result = super::RefreshResult::restart_required();
    std::assert_eq!(result.status, "restart_required");
    std::assert!(result.compiled);
    std::assert!(result.error.is_some());
  }
}
