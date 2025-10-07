//! Main error type for the hex crate.
//!
//! Hexserror provides comprehensive error information following ERRORS_PROMPT.md
//! guidelines. Wraps layer-specific error structs with full error chaining support.
//! All errors include error codes, descriptive messages, actionable next steps,
//! and suggestions for remediation. Designed for both humans and AI agents.
//!
//! Revision History
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
///     Err(Hexserror::Domain(err))
/// }
/// ```
#[derive(Debug)]
pub enum Hexserror {
    /// Domain layer error
    Domain(crate::error::domain_error::DomainError),
    /// Port layer error
    Port(crate::error::port_error::PortError),
    /// Adapter layer error
    Adapter(crate::error::adapter_error::AdapterError),
    /// Validation error
    Validation(crate::error::validation_error::ValidationError),
    /// Resource not found error
    NotFound(crate::error::not_found_error::NotFoundError),
    /// Conflict error
    Conflict(crate::error::conflict_error::ConflictError),
}

impl Hexserror {
    /// Create domain error with code and message
    pub fn domain(code: &str, message: &str) -> Self {
        Self::Domain(crate::error::domain_error::DomainError::new(code, message))
    }

    /// Create port error with code and message
    pub fn port(code: &str, message: &str) -> Self {
        Self::Port(crate::error::port_error::PortError::new(code, message))
    }

    /// Create adapter error with code and message
    pub fn adapter(code: &str, message: &str) -> Self {
        Self::Adapter(crate::error::adapter_error::AdapterError::new(code, message))
    }

    /// Create validation error
    pub fn validation(message: &str) -> Self {
        Self::Validation(
            crate::error::validation_error::ValidationError::new(
                crate::error::codes::validation::INVALID_FORMAT,
                message
            )
        )
    }

    /// Create validation error for specific field
    pub fn validation_field(message: &str, field: &str) -> Self {
        Self::Validation(
            crate::error::validation_error::ValidationError::new(
                crate::error::codes::validation::REQUIRED_FIELD,
                message
            ).with_field(field)
        )
    }

    /// Create not found error
    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::NotFound(crate::error::not_found_error::NotFoundError::new(resource, id))
    }

    /// Create conflict error
    pub fn conflict(message: &str) -> Self {
        Self::Conflict(crate::error::conflict_error::ConflictError::new(message))
    }

    /// Add next step (builder pattern)
    pub fn with_next_step(self, step: &str) -> Self {
        match self {
            Self::Domain(err) => Self::Domain(err.with_next_step(step)),
            Self::Port(err) => Self::Port(err.with_next_step(step)),
            Self::Adapter(err) => Self::Adapter(err.with_next_step(step)),
            other => other,
        }
    }

    /// Add multiple next steps (builder pattern)
    pub fn with_next_steps(self, steps: &[&str]) -> Self {
        match self {
            Self::Domain(err) => Self::Domain(err.with_next_steps(steps)),
            Self::Port(err) => Self::Port(err.with_next_steps(steps)),
            Self::Adapter(err) => Self::Adapter(err.with_next_steps(steps)),
            other => other,
        }
    }

    /// Add suggestion (builder pattern)
    pub fn with_suggestion(self, suggestion: &str) -> Self {
        match self {
            Self::Domain(err) => Self::Domain(err.with_suggestion(suggestion)),
            Self::Port(err) => Self::Port(err.with_suggestion(suggestion)),
            Self::Adapter(err) => Self::Adapter(err.with_suggestion(suggestion)),
            other => other,
        }
    }

    /// Add multiple suggestions (builder pattern)
    pub fn with_suggestions(self, suggestions: &[&str]) -> Self {
        match self {
            Self::Domain(err) => Self::Domain(err.with_suggestions(suggestions)),
            Self::Port(err) => Self::Port(err.with_suggestions(suggestions)),
            Self::Adapter(err) => Self::Adapter(err.with_suggestions(suggestions)),
            other => other,
        }
    }

    /// Add field to validation error (builder pattern)
    pub fn with_field(self, field: &str) -> Self {
        match self {
            Self::Validation(err) => Self::Validation(err.with_field(field)),
            other => other,
        }
    }

    /// Add existing ID to conflict error (builder pattern)
    pub fn with_existing_id(self, id: &str) -> Self {
        match self {
            Self::Conflict(err) => Self::Conflict(err.with_existing_id(id)),
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
  use std::error::Error;
  use super::*;

    #[test]
    fn test_domain_error_creation() {
        let err = Hexserror::domain("E_HEX_001", "Test error");
        assert!(matches!(err, Hexserror::Domain(_)));
    }

    #[test]
    fn test_validation_error_creation() {
        let err = Hexserror::validation("Invalid input");
        assert!(matches!(err, Hexserror::Validation(_)));
    }

    #[test]
    fn test_not_found_error_creation() {
        let err = Hexserror::not_found("User", "123");
        assert!(matches!(err, Hexserror::NotFound(_)));
    }

    #[test]
    fn test_error_display() {
        let err = Hexserror::validation("Test message");
        let display = format!("{}", err);
        assert!(display.contains("Test message"));
    }

    #[test]
    fn test_builder_next_step() {
        let err = Hexserror::domain("E_TEST", "Test error")
            .with_next_step("Do this first");

        if let Hexserror::Domain(domain_err) = err {
            assert_eq!(domain_err.next_steps.len(), 1);
            assert_eq!(domain_err.next_steps[0], "Do this first");
        } else {
            panic!("Expected Domain error");
        }
    }

    #[test]
    fn test_builder_multiple_steps() {
        let err = Hexserror::domain("E_TEST", "Test error")
            .with_next_steps(&["Step 1", "Step 2"]);

        if let Hexserror::Domain(domain_err) = err {
            assert_eq!(domain_err.next_steps.len(), 2);
        } else {
            panic!("Expected Domain error");
        }
    }

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

    #[test]
    fn test_validation_with_field() {
        let err = Hexserror::validation("Invalid value")
            .with_field("email");

        if let Hexserror::Validation(val_err) = err {
            assert_eq!(val_err.field, Some(String::from("email")));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[test]
    fn test_conflict_with_existing_id() {
        let err = Hexserror::conflict("Resource exists")
            .with_existing_id("123");

        if let Hexserror::Conflict(conf_err) = err {
            assert_eq!(conf_err.existing_id, Some(String::from("123")));
        } else {
            panic!("Expected Conflict error");
        }
    }

    #[test]
    fn test_error_source_chaining() {
        let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let domain_err = crate::error::domain_error::DomainError::new("E_HEX_001", "Failed to load")
            .with_source(inner);
        let err = Hexserror::Domain(domain_err);

        assert!(err.source().is_some());
    }
}
