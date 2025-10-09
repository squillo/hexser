//! Conflict error type for state conflicts.
//!
//! Provides ConflictError struct for resource state conflicts.
//! Conflict errors occur when operations conflict with current resource state.
//! Includes context about the conflicting resource.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict duplicates.
//! - 2025-10-06T00:00:00Z @AI: Initial ConflictError struct for Phase 1.

/// Conflict error for state conflicts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictError {
  /// Error code from codes::resource module
  pub code: String,
  /// Human-readable conflict description
  pub message: String,
  /// Optional ID of existing conflicting resource
  pub existing_id: Option<String>,
  /// Optional source code location
  pub location: Option<crate::error::source_location::SourceLocation>,
}

impl ConflictError {
  /// Create new conflict error
  pub fn new(message: impl Into<String>) -> Self {
    Self {
      code: String::from(crate::error::codes::resource::CONFLICT),
      message: message.into(),
      existing_id: None,
      location: None,
    }
  }

  /// Add existing resource ID (builder pattern)
  pub fn with_existing_id(mut self, id: impl Into<String>) -> Self {
    self.existing_id = Some(id.into());
    self
  }

  /// Add source location (builder pattern)
  pub fn with_location(mut self, location: crate::error::source_location::SourceLocation) -> Self {
    self.location = Some(location);
    self
  }
}

impl std::fmt::Display for ConflictError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error [{}]: {}", self.code, self.message)?;

    if let Some(ref id) = self.existing_id {
      write!(f, " (existing ID: {})", id)?;
    }

    write!(
      f,
      "\nNext Steps: Resolve conflict or use different identifier"
    )?;

    if let Some(ref location) = self.location {
      write!(f, "\nSource: {}", location)?;
    }

    Ok(())
  }
}

impl std::error::Error for ConflictError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_conflict_error_creation() {
    let err = ConflictError::new("Resource already exists");
    assert_eq!(err.message, "Resource already exists");
    assert_eq!(err.code, crate::error::codes::resource::CONFLICT);
    assert_eq!(err.existing_id, None);
  }

  #[test]
  fn test_conflict_error_with_existing_id() {
    let err = ConflictError::new("Duplicate user").with_existing_id("user-456");

    assert_eq!(err.existing_id, Some(String::from("user-456")));
  }

  #[test]
  fn test_conflict_error_display() {
    let err = ConflictError::new("Email already registered").with_existing_id("123");

    let display = format!("{}", err);
    assert!(display.contains("Email already registered"));
    assert!(display.contains("123"));
  }
}
