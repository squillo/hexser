//! Conflict error type for state conflicts.
//!
//! Provides ConflictError struct for resource state conflicts.
//! Conflict errors occur when operations conflict with current resource state.
//! Includes context about the conflicting resource.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: Add next_steps/suggestions storage + builders so Hexserror guidance builders no longer silently drop input on this variant.
//! - 2025-10-09T21:51:00Z @AI: Add conditional source location serialization via env_control.
//! - 2025-10-09T21:22:00Z @AI: Add Serde support for rich errors.
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict duplicates.
//! - 2025-10-06T00:00:00Z @AI: Initial ConflictError struct for Phase 1.

/// Conflict error for state conflicts
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConflictError {
  /// Error code from codes::resource module
  pub code: String,
  /// Human-readable conflict description
  pub message: String,
  /// Optional ID of existing conflicting resource
  pub existing_id: Option<String>,
  /// Actionable next steps for resolving the error
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Vec::is_empty")
  )]
  pub next_steps: Vec<String>,
  /// Concrete suggestions (e.g. example fixes)
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Vec::is_empty")
  )]
  pub suggestions: Vec<String>,
  /// Optional source code location
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "crate::error::env_control::should_skip_location")
  )]
  pub location: Option<crate::error::source_location::SourceLocation>,
}

impl ConflictError {
  /// Create new conflict error
  pub fn new(message: impl Into<String>) -> Self {
    Self {
      code: String::from(crate::error::codes::resource::CONFLICT),
      message: message.into(),
      existing_id: None,
      next_steps: Vec::new(),
      suggestions: Vec::new(),
      location: None,
    }
  }

  /// Add existing resource ID (builder pattern)
  pub fn with_existing_id(mut self, id: impl Into<String>) -> Self {
    self.existing_id = Some(id.into());
    self
  }

  /// Add an actionable next step (builder pattern)
  pub fn with_next_step(mut self, step: impl Into<String>) -> Self {
    self.next_steps.push(step.into());
    self
  }

  /// Add multiple next steps (builder pattern)
  pub fn with_next_steps(mut self, steps: &[&str]) -> Self {
    self
      .next_steps
      .extend(steps.iter().map(|s| String::from(*s)));
    self
  }

  /// Add a suggestion (builder pattern)
  pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
    self.suggestions.push(suggestion.into());
    self
  }

  /// Add multiple suggestions (builder pattern)
  pub fn with_suggestions(mut self, suggestions: &[&str]) -> Self {
    self
      .suggestions
      .extend(suggestions.iter().map(|s| String::from(*s)));
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

    if self.next_steps.is_empty() {
      write!(
        f,
        "\nNext Step: Resolve conflict or use different identifier"
      )?;
    } else {
      for step in &self.next_steps {
        write!(f, "\nNext Step: {}", step)?;
      }
    }
    for suggestion in &self.suggestions {
      write!(f, "\nSuggestion: {}", suggestion)?;
    }

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
