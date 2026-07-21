//! Main error type for the hex crate.
//!
//! Hexserror provides comprehensive error information following ERRORS_PROMPT.md
//! guidelines. Wraps layer-specific error structs with full error chaining support.
//! All errors include error codes, descriptive messages, actionable next steps,
//! and suggestions for remediation. Designed for both humans and AI agents.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: Add Hexserror::with_source forwarding to layer variants (makes the documented source-chaining builder real).
//! - 2026-07-21T00:00:00Z @AI: Forward with_next_step(s)/with_suggestion(s) to Validation/NotFound/Conflict variants (was silently dropped on those).
//! - 2026-07-20T00:00:00Z @AI: Box variant payloads to shrink Hexserror (fixes clippy::result_large_err across the crate); enum is now pointer-sized.
//! - 2025-10-09T21:22:00Z @AI: Add Serde support for rich errors.
//! - 2025-10-06T00:00:00Z @AI: Refactor to wrap layer-specific error structs for Phase 1.
//! - 2025-10-01T00:00:00Z @AI: Initial Hexserror enum with rich error information.

use crate::error::RichError;

/// Main error type for the hex crate
///
/// Wraps layer-specific error types with full error chaining support.
/// Implements std::error::Error for seamless integration with Rust ecosystem.
///
/// # Example
///
/// ```rust
/// use hexser::error::domain_error::DomainError;
/// use hexser::error::hex_error::Hexserror;
/// use hexser::error::RichError;
///
/// fn validate_order() -> Result<(), Hexserror> {
///     let err = DomainError::new("E_HEX_001", "Order cannot be empty")
///         .with_next_step("Add at least one item")
///         .with_suggestion("order.add_item(item)");
///     Err(Hexserror::Domain(Box::new(err)))
/// }
/// ```
///
/// # Layout
///
/// Each variant payload is boxed. The rich per-layer error structs are large
/// (`LayerError` alone is ~176 bytes), and `Hexserror` is the `Err` half of
/// nearly every `HexResult<T>` in the crate. Boxing keeps `Hexserror` pointer-sized
/// so success paths do not pay for the error payload's stack footprint, and
/// resolves `clippy::result_large_err` at every call site.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Hexserror {
  /// Domain layer error
  Domain(std::boxed::Box<crate::error::domain_error::DomainError>),
  /// Port layer error
  Port(std::boxed::Box<crate::error::port_error::PortError>),
  /// Adapter layer error
  Adapter(std::boxed::Box<crate::error::adapter_error::AdapterError>),
  /// Validation error
  Validation(std::boxed::Box<crate::error::validation_error::ValidationError>),
  /// Resource not found error
  NotFound(std::boxed::Box<crate::error::not_found_error::NotFoundError>),
  /// Conflict error
  Conflict(std::boxed::Box<crate::error::conflict_error::ConflictError>),
}

impl Hexserror {
  /// Create domain error with code and message
  pub fn domain(code: &str, message: &str) -> Self {
    Self::Domain(std::boxed::Box::new(
      crate::error::domain_error::DomainError::new(code, message),
    ))
  }

  /// Create port error with code and message
  pub fn port(code: &str, message: &str) -> Self {
    Self::Port(std::boxed::Box::new(
      crate::error::port_error::PortError::new(code, message),
    ))
  }

  /// Create adapter error with code and message
  pub fn adapter(code: &str, message: &str) -> Self {
    Self::Adapter(std::boxed::Box::new(
      crate::error::adapter_error::AdapterError::new(code, message),
    ))
  }

  /// Create validation error
  pub fn validation(message: &str) -> Self {
    Self::Validation(std::boxed::Box::new(
      crate::error::validation_error::ValidationError::new(
        crate::error::codes::validation::INVALID_FORMAT,
        message,
      ),
    ))
  }

  /// Create validation error for specific field
  pub fn validation_field(message: &str, field: &str) -> Self {
    Self::Validation(std::boxed::Box::new(
      crate::error::validation_error::ValidationError::new(
        crate::error::codes::validation::REQUIRED_FIELD,
        message,
      )
      .with_field(field),
    ))
  }

  /// Create not found error
  pub fn not_found(resource: &str, id: &str) -> Self {
    Self::NotFound(std::boxed::Box::new(
      crate::error::not_found_error::NotFoundError::new(resource, id),
    ))
  }

  /// Create conflict error
  pub fn conflict(message: &str) -> Self {
    Self::Conflict(std::boxed::Box::new(
      crate::error::conflict_error::ConflictError::new(message),
    ))
  }

  /// Add next step (builder pattern).
  ///
  /// Works on every variant — the guidance is attached to whichever layer/resource error is
  /// wrapped, so it is never silently dropped.
  pub fn with_next_step(self, step: &str) -> Self {
    match self {
      Self::Domain(err) => Self::Domain(std::boxed::Box::new(err.with_next_step(step))),
      Self::Port(err) => Self::Port(std::boxed::Box::new(err.with_next_step(step))),
      Self::Adapter(err) => Self::Adapter(std::boxed::Box::new(err.with_next_step(step))),
      Self::Validation(err) => Self::Validation(std::boxed::Box::new(err.with_next_step(step))),
      Self::NotFound(err) => Self::NotFound(std::boxed::Box::new(err.with_next_step(step))),
      Self::Conflict(err) => Self::Conflict(std::boxed::Box::new(err.with_next_step(step))),
    }
  }

  /// Add multiple next steps (builder pattern). Works on every variant.
  pub fn with_next_steps(self, steps: &[&str]) -> Self {
    match self {
      Self::Domain(err) => Self::Domain(std::boxed::Box::new(err.with_next_steps(steps))),
      Self::Port(err) => Self::Port(std::boxed::Box::new(err.with_next_steps(steps))),
      Self::Adapter(err) => Self::Adapter(std::boxed::Box::new(err.with_next_steps(steps))),
      Self::Validation(err) => Self::Validation(std::boxed::Box::new(err.with_next_steps(steps))),
      Self::NotFound(err) => Self::NotFound(std::boxed::Box::new(err.with_next_steps(steps))),
      Self::Conflict(err) => Self::Conflict(std::boxed::Box::new(err.with_next_steps(steps))),
    }
  }

  /// Add suggestion (builder pattern). Works on every variant.
  pub fn with_suggestion(self, suggestion: &str) -> Self {
    match self {
      Self::Domain(err) => Self::Domain(std::boxed::Box::new(err.with_suggestion(suggestion))),
      Self::Port(err) => Self::Port(std::boxed::Box::new(err.with_suggestion(suggestion))),
      Self::Adapter(err) => Self::Adapter(std::boxed::Box::new(err.with_suggestion(suggestion))),
      Self::Validation(err) => {
        Self::Validation(std::boxed::Box::new(err.with_suggestion(suggestion)))
      }
      Self::NotFound(err) => Self::NotFound(std::boxed::Box::new(err.with_suggestion(suggestion))),
      Self::Conflict(err) => Self::Conflict(std::boxed::Box::new(err.with_suggestion(suggestion))),
    }
  }

  /// Add multiple suggestions (builder pattern). Works on every variant.
  pub fn with_suggestions(self, suggestions: &[&str]) -> Self {
    match self {
      Self::Domain(err) => Self::Domain(std::boxed::Box::new(err.with_suggestions(suggestions))),
      Self::Port(err) => Self::Port(std::boxed::Box::new(err.with_suggestions(suggestions))),
      Self::Adapter(err) => Self::Adapter(std::boxed::Box::new(err.with_suggestions(suggestions))),
      Self::Validation(err) => {
        Self::Validation(std::boxed::Box::new(err.with_suggestions(suggestions)))
      }
      Self::NotFound(err) => {
        Self::NotFound(std::boxed::Box::new(err.with_suggestions(suggestions)))
      }
      Self::Conflict(err) => {
        Self::Conflict(std::boxed::Box::new(err.with_suggestions(suggestions)))
      }
    }
  }

  /// Add field to validation error (builder pattern)
  pub fn with_field(self, field: &str) -> Self {
    match self {
      Self::Validation(err) => Self::Validation(std::boxed::Box::new(err.with_field(field))),
      other => other,
    }
  }

  /// Add existing ID to conflict error (builder pattern)
  pub fn with_existing_id(self, id: &str) -> Self {
    match self {
      Self::Conflict(err) => Self::Conflict(std::boxed::Box::new(err.with_existing_id(id))),
      other => other,
    }
  }

  /// Attach an underlying cause (builder pattern).
  ///
  /// Applies to the layer variants (Domain/Port/Adapter), which carry a source chain. The
  /// Validation/NotFound/Conflict variants have no source field and are returned unchanged.
  pub fn with_source(self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
    match self {
      Self::Domain(err) => Self::Domain(std::boxed::Box::new(err.with_source(source))),
      Self::Port(err) => Self::Port(std::boxed::Box::new(err.with_source(source))),
      Self::Adapter(err) => Self::Adapter(std::boxed::Box::new(err.with_source(source))),
      other => other,
    }
  }
}

impl std::fmt::Display for Hexserror {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Domain(err) => write!(f, "{}", err),
      Self::Port(err) => write!(f, "{}", err),
      Self::Adapter(err) => write!(f, "{}", err),
      Self::Validation(err) => write!(f, "{}", err),
      Self::NotFound(err) => write!(f, "{}", err),
      Self::Conflict(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Hexserror {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::Domain(err) => err.source(),
      Self::Port(err) => err.source(),
      Self::Adapter(err) => err.source(),
      Self::Validation(err) => err.source(),
      Self::NotFound(err) => err.source(),
      Self::Conflict(err) => err.source(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::error::Error;

  /// why: the `domain` constructor must produce the `Domain` variant so callers can
  /// match on layer; boxing the payload must not change which variant is built.
  #[test]
  fn test_domain_error_creation() {
    let err = Hexserror::domain("E_HEX_001", "Test error");
    assert!(matches!(err, Hexserror::Domain(_)));
  }

  /// why: guards that `validation` builds the `Validation` variant (not another layer).
  #[test]
  fn test_validation_error_creation() {
    let err = Hexserror::validation("Invalid input");
    assert!(matches!(err, Hexserror::Validation(_)));
  }

  /// why: guards that `not_found` builds the `NotFound` variant.
  #[test]
  fn test_not_found_error_creation() {
    let err = Hexserror::not_found("User", "123");
    assert!(matches!(err, Hexserror::NotFound(_)));
  }

  /// why: Display must surface the human message; boxing must not break the `Display`
  /// delegation through the boxed payload.
  #[test]
  fn test_error_display() {
    let err = Hexserror::validation("Test message");
    let display = format!("{}", err);
    assert!(display.contains("Test message"));
  }

  /// why: `with_next_step` on a Domain error must persist the step; verifies the boxed
  /// payload is unboxed, mutated, and re-boxed without loss.
  #[test]
  fn test_builder_next_step() {
    let err = Hexserror::domain("E_TEST", "Test error").with_next_step("Do this first");

    if let Hexserror::Domain(domain_err) = err {
      assert_eq!(domain_err.next_steps.len(), 1);
      assert_eq!(domain_err.next_steps[0], "Do this first");
    } else {
      panic!("Expected Domain error");
    }
  }

  /// why: `with_next_steps` must append all provided steps, not just the first.
  #[test]
  fn test_builder_multiple_steps() {
    let err = Hexserror::domain("E_TEST", "Test error").with_next_steps(&["Step 1", "Step 2"]);

    if let Hexserror::Domain(domain_err) = err {
      assert_eq!(domain_err.next_steps.len(), 2);
    } else {
      panic!("Expected Domain error");
    }
  }

  /// why: single and multiple suggestion builders must accumulate (1 + 2 = 3), confirming
  /// chained builders each round-trip through the box.
  #[test]
  fn test_builder_suggestions() {
    let err = Hexserror::domain("E_TEST", "Test error")
      .with_suggestion("Try this")
      .with_suggestions(&["Or this", "Or that"]);

    if let Hexserror::Domain(domain_err) = err {
      assert_eq!(domain_err.suggestions.len(), 3);
    } else {
      panic!("Expected Domain error");
    }
  }

  /// why: `with_field` must record the offending field on a Validation error for
  /// actionable diagnostics.
  #[test]
  fn test_validation_with_field() {
    let err = Hexserror::validation("Invalid value").with_field("email");

    if let Hexserror::Validation(val_err) = err {
      assert_eq!(val_err.field, Some(String::from("email")));
    } else {
      panic!("Expected Validation error");
    }
  }

  /// why: `with_existing_id` must record the conflicting id on a Conflict error.
  #[test]
  fn test_conflict_with_existing_id() {
    let err = Hexserror::conflict("Resource exists").with_existing_id("123");

    if let Hexserror::Conflict(conf_err) = err {
      assert_eq!(conf_err.existing_id, Some(String::from("123")));
    } else {
      panic!("Expected Conflict error");
    }
  }

  /// why: `std::error::Error::source` must expose the wrapped cause so `?`-chains and
  /// error reporters can walk the chain; boxing the variant must preserve this.
  #[test]
  fn test_error_source_chaining() {
    let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let domain_err = crate::error::domain_error::DomainError::new("E_HEX_001", "Failed to load")
      .with_source(inner);
    let err = Hexserror::Domain(std::boxed::Box::new(domain_err));

    assert!(err.source().is_some());
  }

  /// why: `with_next_step` on a NotFound error must actually retain the step (it was silently
  /// dropped before — the README even recommended this exact call). The guidance must appear in
  /// the Display output that users and AI agents read.
  #[test]
  fn test_not_found_retains_next_step() {
    let err = Hexserror::not_found("User", "123").with_next_step("Verify the ID and try again");
    if let Hexserror::NotFound(nf) = &err {
      assert_eq!(
        nf.next_steps,
        vec![String::from("Verify the ID and try again")]
      );
    } else {
      panic!("expected NotFound");
    }
    assert!(format!("{}", err).contains("Verify the ID and try again"));
  }

  /// why: Hexserror::with_source must attach the cause to layer variants so `source()` walks
  /// the chain — the README documents this pattern pervasively, so it must actually work.
  #[test]
  fn test_with_source_attaches_cause_on_layer_variant() {
    let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "boom");
    let err = Hexserror::adapter("E_HEX_200", "db down").with_source(inner);
    assert!(err.source().is_some());
  }

  /// why: guidance builders must retain input on Validation and Conflict variants too, closing
  /// the silent-drop gap for every non-layer variant.
  #[test]
  fn test_validation_and_conflict_retain_guidance() {
    let v = Hexserror::validation("bad").with_suggestion("use a valid email");
    if let Hexserror::Validation(ve) = &v {
      assert_eq!(ve.suggestions, vec![String::from("use a valid email")]);
    } else {
      panic!("expected Validation");
    }

    let c = Hexserror::conflict("dup").with_next_steps(&["merge", "rename"]);
    if let Hexserror::Conflict(ce) = &c {
      assert_eq!(ce.next_steps.len(), 2);
    } else {
      panic!("expected Conflict");
    }
  }
}
