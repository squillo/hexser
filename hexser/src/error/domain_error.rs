//! Domain-specific error types and utilities.
//!
//! Provides DomainError type alias for business rule violations in the domain layer.
//! Domain errors represent failures in core business logic and invariant violations.
//! Includes error chaining, rich context, and actionable guidance via RichError trait.
//!
//! Revision History
//! - 2025-10-06T01:00:00Z @AI: Refactor to use LayerError generic for Phase 1.
//! - 2025-10-06T00:00:00Z @AI: Implement DomainError struct for Phase 1.
//! - 2025-10-01T00:00:00Z @AI: Initial placeholder for domain error utilities.

use crate::error::RichError;

/// Domain layer error type
pub type DomainError =
  crate::error::layer_error::LayerError<crate::error::layer_error::layer_markers::DomainLayer>;

/// Create invariant violation error
pub fn invariant_violation(message: impl Into<String>) -> DomainError {
  DomainError::new(crate::error::codes::domain::INVARIANT_VIOLATION, message)
    .with_next_step("Check aggregate invariants")
    .with_suggestion("Verify business rules before operations")
}

/// Create invalid state transition error
pub fn invalid_state_transition(message: impl Into<String>) -> DomainError {
  DomainError::new(
    crate::error::codes::domain::INVALID_STATE_TRANSITION,
    message,
  )
  .with_next_step("Verify entity lifecycle")
  .with_suggestion("Check valid state transitions in documentation")
}

/// Create empty invariant error
pub fn invariant_empty(message: impl Into<String>) -> DomainError {
  DomainError::new(crate::error::codes::domain::INVARIANT_EMPTY, message)
    .with_next_step("Add required items")
    .with_suggestion("Ensure container is not empty before save")
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::error::rich_error::RichError;
  use std::error::Error;

  #[test]
  fn test_domain_error_creation() {
    let err = DomainError::new("E_HEX_001", "Invariant violation");
    assert_eq!(err.code(), "E_HEX_001");
    assert_eq!(err.message(), "Invariant violation");
  }

  #[test]
  fn test_domain_error_builder() {
    let err = DomainError::new("E_HEX_002", "Invalid state")
      .with_next_step("Check entity state")
      .with_suggestion("Validate before save");

    assert_eq!(err.next_steps().len(), 1);
    assert_eq!(err.suggestions().len(), 1);
  }

  #[test]
  fn test_domain_error_display() {
    let err = DomainError::new("E_HEX_001", "Test error").with_next_step("Do this");

    let display = format!("{}", err);
    assert!(display.contains("E_HEX_001"));
    assert!(display.contains("Test error"));
    assert!(display.contains("Next Steps"));
  }

  #[test]
  fn test_domain_error_with_source() {
    let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = DomainError::new("E_HEX_003", "Load failed").with_source(inner);

    assert!(err.source().is_some());
  }
}
