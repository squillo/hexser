//! Validation error type for input validation failures.
//!
//! Provides ValidationError struct for input data validation failures.
//! Validation errors occur when user input or data doesn't meet requirements.
//! Includes field-specific context and actionable guidance.
//!
//! Revision History
//! - 2025-10-06T00:00:00Z @AI: Initial ValidationError struct for Phase 1.

use std::fmt::{Display, Formatter};

/// Validation error with field-specific context
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Error code from codes::validation module
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Optional field name that failed validation
    pub field: Option<String>,
    /// Optional source code location
    pub location: Option<crate::error::source_location::SourceLocation>,
}


impl ValidationError {
    /// Create new validation error
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
            location: None,
        }
    }

    /// Add field name (builder pattern)
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Add source location (builder pattern)
    pub fn with_location(mut self, location: crate::error::source_location::SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}


impl std::fmt::Display for ValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(ref field_name) = self.field {
      write!(f, "Error [{}]: Validation failed on field '{}': {}", self.code, field_name, self.message)?;
    } else {
      write!(f, "Error [{}]: {}", self.code, self.message)?;
    }

    if let Some(ref location) = self.location {
      write!(f, "\nSource: {}", location)?;
    }

    Ok(())
  }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let err = ValidationError::new("E_HEX_300", "Invalid input");
        assert_eq!(err.code, "E_HEX_300");
        assert_eq!(err.message, "Invalid input");
        assert_eq!(err.field, None);
    }

    #[test]
    fn test_validation_error_with_field() {
        let err = ValidationError::new("E_HEX_301", "Invalid format")
            .with_field("email");

        assert_eq!(err.field, Some(String::from("email")));
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new("E_HEX_300", "Required field missing")
            .with_field("username");

        let display = format!("{}", err);
        assert!(display.contains("username"));
        assert!(display.contains("E_HEX_300"));
    }
}
