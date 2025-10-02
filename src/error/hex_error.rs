//! Main error type for the hex crate.
//!
//! HexError provides comprehensive error information following ERRORS_PROMPT.md
//! guidelines. Each error variant includes error codes, descriptive messages,
//! actionable next steps, and suggestions for remediation. Errors are designed
//! to be empathetic, accessible, and helpful for both humans and AI agents.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial HexError enum with rich error information.

/// Main error type for the hex crate.
///
/// Provides rich error information including codes, messages, next steps,
/// and suggestions following ERRORS_PROMPT.md guidelines.
///
/// # Example
///
/// ```rust
/// use hex::HexError;
///
/// fn do_something() -> Result<(), HexError> {
///     Err(HexError::Domain {
///         code: String::from("E_HEX_001"),
///         message: String::from("Invalid operation"),
///         next_steps: vec![String::from("Check input values")],
///         suggestions: vec![String::from("Ensure all required fields are present")],
///     })
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexError {
    /// Domain layer errors representing business rule violations.
    Domain {
        code: String,
        message: String,
        next_steps: Vec<String>,
        suggestions: Vec<String>,
    },

    /// Port layer errors from interface communication failures.
    Port {
        code: String,
        message: String,
        next_steps: Vec<String>,
        suggestions: Vec<String>,
    },

    /// Adapter layer errors from infrastructure failures.
    Adapter {
        code: String,
        message: String,
        next_steps: Vec<String>,
        suggestions: Vec<String>,
    },

    /// Validation errors for input data.
    Validation {
        message: String,
        field: Option<String>,
    },

    /// Resource not found errors.
    NotFound {
        resource: String,
        id: String,
    },

    /// Conflict errors for state conflicts.
    Conflict {
        message: String,
        existing_id: Option<String>,
    },
}

impl HexError {
    /// Create a domain error with code and message.
    pub fn domain(code: &str, message: &str) -> Self {
        Self::Domain {
            code: String::from(code),
            message: String::from(message),
            next_steps: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a port error with code and message.
    pub fn port(code: &str, message: &str) -> Self {
        Self::Port {
            code: String::from(code),
            message: String::from(message),
            next_steps: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create an adapter error with code and message.
    pub fn adapter(code: &str, message: &str) -> Self {
        Self::Adapter {
            code: String::from(code),
            message: String::from(message),
            next_steps: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a validation error.
    pub fn validation(message: &str) -> Self {
        Self::Validation {
            message: String::from(message),
            field: None,
        }
    }

    /// Create a validation error for a specific field.
    pub fn validation_field(message: &str, field: &str) -> Self {
        Self::Validation {
            message: String::from(message),
            field: Some(String::from(field)),
        }
    }

    /// Create a not found error.
    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::NotFound {
            resource: String::from(resource),
            id: String::from(id),
        }
    }

    /// Create a conflict error.
    pub fn conflict(message: &str) -> Self {
        Self::Conflict {
            message: String::from(message),
            existing_id: None,
        }
    }

    /// Add a next step to the error (builder pattern).
    pub fn with_next_step(mut self, step: &str) -> Self {
        match &mut self {
            Self::Domain { next_steps, .. }
            | Self::Port { next_steps, .. }
            | Self::Adapter { next_steps, .. } => {
                next_steps.push(String::from(step));
            }
            _ => {}
        }
        self
    }

    /// Add multiple next steps to the error (builder pattern).
    pub fn with_next_steps(mut self, steps: &[&str]) -> Self {
        match &mut self {
            Self::Domain { next_steps, .. }
            | Self::Port { next_steps, .. }
            | Self::Adapter { next_steps, .. } => {
                next_steps.extend(steps.iter().map(|s| String::from(*s)));
            }
            _ => {}
        }
        self
    }

    /// Add a suggestion to the error (builder pattern).
    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        match &mut self {
            Self::Domain { suggestions, .. }
            | Self::Port { suggestions, .. }
            | Self::Adapter { suggestions, .. } => {
                suggestions.push(String::from(suggestion));
            }
            _ => {}
        }
        self
    }

    /// Add multiple suggestions to the error (builder pattern).
    pub fn with_suggestions(mut self, suggestions_list: &[&str]) -> Self {
        match &mut self {
            Self::Domain { suggestions, .. }
            | Self::Port { suggestions, .. }
            | Self::Adapter { suggestions, .. } => {
                suggestions.extend(suggestions_list.iter().map(|s| String::from(*s)));
            }
            _ => {}
        }
        self
    }

    /// Add field information to validation error (builder pattern).
    pub fn with_field(mut self, field: &str) -> Self {
        if let Self::Validation { field: field_opt, .. } = &mut self {
            *field_opt = Some(String::from(field));
        }
        self
    }

    /// Add existing ID to conflict error (builder pattern).
    pub fn with_existing_id(mut self, id: &str) -> Self {
        if let Self::Conflict { existing_id, .. } = &mut self {
            *existing_id = Some(String::from(id));
        }
        self
    }
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain { code, message, next_steps, suggestions } => {
                write!(f, "Error: {} - {}", code, message)?;
                if !next_steps.is_empty() {
                    write!(f, "\nNext Steps:")?;
                    for step in next_steps {
                        write!(f, "\n  - {}", step)?;
                    }
                }
                if !suggestions.is_empty() {
                    write!(f, "\nSuggestions:")?;
                    for suggestion in suggestions {
                        write!(f, "\n  - {}", suggestion)?;
                    }
                }
                Result::Ok(())
            }
            Self::Port { code, message, .. } => {
                write!(f, "Port Error: {} - {}", code, message)
            }
            Self::Adapter { code, message, .. } => {
                write!(f, "Adapter Error: {} - {}", code, message)
            }
            Self::Validation { message, field } => {
                if let Some(field_name) = field {
                    write!(f, "Validation Error on '{}': {}", field_name, message)
                } else {
                    write!(f, "Validation Error: {}", message)
                }
            }
            Self::NotFound { resource, id } => {
                write!(f, "Not Found: {} with id '{}'", resource, id)
            }
            Self::Conflict { message, .. } => {
                write!(f, "Conflict: {}", message)
            }
        }
    }
}

impl std::error::Error for HexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_error_creation() {
        let err = HexError::domain("E_HEX_001", "Test error");
        assert!(matches!(err, HexError::Domain { .. }));
    }

    #[test]
    fn test_validation_error_creation() {
        let err = HexError::validation("Invalid input");
        assert!(matches!(err, HexError::Validation { .. }));
    }

    #[test]
    fn test_not_found_error_creation() {
        let err = HexError::not_found("User", "123");
        assert!(matches!(err, HexError::NotFound { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = HexError::validation("Test message");
        let display = format!("{}", err);
        assert!(display.contains("Test message"));
    }

    #[test]
    fn test_builder_next_step() {
        let err = HexError::domain("E_TEST", "Test error")
            .with_next_step("Do this first");

        if let HexError::Domain { next_steps, .. } = err {
            assert_eq!(next_steps.len(), 1);
            assert_eq!(next_steps[0], "Do this first");
        } else {
            panic!("Expected Domain error");
        }
    }

    #[test]
    fn test_builder_multiple_steps() {
        let err = HexError::domain("E_TEST", "Test error")
            .with_next_steps(&["Step 1", "Step 2"]);

        if let HexError::Domain { next_steps, .. } = err {
            assert_eq!(next_steps.len(), 2);
        } else {
            panic!("Expected Domain error");
        }
    }

    #[test]
    fn test_builder_suggestions() {
        let err = HexError::domain("E_TEST", "Test error")
            .with_suggestion("Try this")
            .with_suggestions(&["Or this", "Or that"]);

        if let HexError::Domain { suggestions, .. } = err {
            assert_eq!(suggestions.len(), 3);
        } else {
            panic!("Expected Domain error");
        }
    }

    #[test]
    fn test_validation_with_field() {
        let err = HexError::validation("Invalid value")
            .with_field("email");

        if let HexError::Validation { field, .. } = err {
            assert_eq!(field, Some(String::from("email")));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[test]
    fn test_conflict_with_existing_id() {
        let err = HexError::conflict("Resource exists")
            .with_existing_id("123");

        if let HexError::Conflict { existing_id, .. } = err {
            assert_eq!(existing_id, Some(String::from("123")));
        } else {
            panic!("Expected Conflict error");
        }
    }
}
