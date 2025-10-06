//! Directive trait for write operations (CQRS pattern).
//!
//! Directives represent write operations that modify system state. They capture
//! the intent to change something and are processed by directive handlers.
//! Directives support the write side of CQRS, separating writes from reads
//! and enabling patterns like event sourcing and directive validation.
//!
//! Revision History
//! - 2025-10-01T00:01:00Z @AI: Renamed from Command to Directive to better reflect intent.
//! - 2025-10-01T00:00:00Z @AI: Initial Command trait definition for CQRS write operations.

/// Trait for directives that represent write operations.
///
/// Directives express the intent to modify system state. They should be
/// immutable and contain all data needed to execute the operation.
///
/// # Example
///
/// ```rust
/// use hexer::application::Directive;
/// use hexer::HexResult;
///
/// struct CreateUserDirective {
///     email: String,
///     password: String,
/// }
///
/// impl Directive for CreateUserDirective {
///     fn validate(&self) -> HexResult<()> {
///         if self.email.contains('@') {
///             Ok(())
///         } else {
///             Err(hexer::HexError::validation("Invalid email"))
///         }
///     }
/// }
/// ```
pub trait Directive {
    /// Validate the directive before execution.
    ///
    /// Returns `Ok(())` if the directive is valid, or an error describing
    /// validation failures.
    fn validate(&self) -> crate::result::hex_result::HexResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDirective {
        value: i32,
    }

    impl Directive for TestDirective {
        fn validate(&self) -> crate::result::hex_result::HexResult<()> {
            if self.value > 0 {
                Result::Ok(())
            } else {
                    Result::Err(
                        crate::error::hex_error::HexError::validation_field(
                            "Value must be positive",
                            "value"
                        )
                    )
            }
        }
    }

    #[test]
    fn test_directive_validation_success() {
        let directive = TestDirective { value: 10 };
        assert!(directive.validate().is_ok());
    }

    #[test]
    fn test_directive_validation_failure() {
        let directive = TestDirective { value: -5 };
        assert!(directive.validate().is_err());
    }
}
