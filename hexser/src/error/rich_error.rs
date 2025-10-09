//! Rich error trait for errors with context and guidance.
//!
//! Defines common behavior for all rich errors in the system.
//! Rich errors include error codes, messages, next steps, suggestions,
//! source locations, documentation links, and error chaining.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict duplicates.
//! - 2025-10-06T01:00:00Z @AI: Initial RichError trait for Phase 1 refactor.

/// Trait for errors with rich context and actionable guidance
pub trait RichError: std::error::Error + std::fmt::Debug + std::fmt::Display {
  /// Get error code
  fn code(&self) -> &str;

  /// Get error message
  fn message(&self) -> &str;

  /// Get next steps for resolution
  fn next_steps(&self) -> &[String];

  /// Get suggestions for fixing the error
  fn suggestions(&self) -> &[String];

  /// Get source code location if available
  fn location(&self) -> Option<&crate::error::source_location::SourceLocation>;

  /// Get documentation URL if available
  fn more_info_url(&self) -> Option<&str>;

  /// Add next step (builder pattern)
  fn with_next_step(self, step: impl Into<String>) -> Self
  where
    Self: Sized;

  /// Add multiple next steps (builder pattern)
  fn with_next_steps(self, steps: &[&str]) -> Self
  where
    Self: Sized;

  /// Add suggestion (builder pattern)
  fn with_suggestion(self, suggestion: impl Into<String>) -> Self
  where
    Self: Sized;

  /// Add multiple suggestions (builder pattern)
  fn with_suggestions(self, suggestions: &[&str]) -> Self
  where
    Self: Sized;

  /// Add source location (builder pattern)
  fn with_location(self, location: crate::error::source_location::SourceLocation) -> Self
  where
    Self: Sized;

  /// Add documentation URL (builder pattern)
  fn with_more_info(self, url: impl Into<String>) -> Self
  where
    Self: Sized;

  /// Add underlying error source (builder pattern)
  fn with_source(self, source: impl std::error::Error + Send + Sync + 'static) -> Self
  where
    Self: Sized;
}
