//! ValueObject trait for domain objects defined by their values.
//!
//! Value objects are immutable objects that are defined entirely by their
//! attribute values. Two value objects with the same attribute values are
//! considered identical. Value objects have no identity and should be immutable.
//! They are used to model descriptive aspects of the domain with no conceptual identity.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial ValueObject trait definition with validation.

/// Trait for domain value objects.
///
/// Value objects are defined by their values, not by identity. They should be
/// immutable and two value objects with the same values are considered equal.
///
/// # Example
///
/// ```rust
/// use hexer::domain::ValueObject;
/// use hexer::HexResult;
///
/// #[derive(Clone, PartialEq, Eq)]
/// struct Email(String);
///
/// impl ValueObject for Email {
///     fn validate(&self) -> HexResult<()> {
///         if self.0.contains('@') {
///             Ok(())
///         } else {
///             Err(hexer::HexError::validation("Invalid email format"))
///         }
///     }
/// }
/// ```
pub trait ValueObject {
    /// Validate the value object's invariants.
    ///
    /// Returns `Ok(())` if the value object is valid, or an error describing
    /// what validation rules were violated.
    fn validate(&self) -> crate::result::hex_result::HexResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestEmail(String);

    impl ValueObject for TestEmail {
        fn validate(&self) -> crate::result::hex_result::HexResult<()> {
            if self.0.contains('@') {
                Result::Ok(())
            } else {
                    Result::Err(
                        crate::error::hex_error::HexError::validation_field(
                            "Email must contain @",
                            "email"
                        )
                    )
            }
        }
    }

    #[test]
    fn test_value_object_validation_success() {
        let email = TestEmail(String::from("test@example.com"));
        assert!(email.validate().is_ok());
    }

    #[test]
    fn test_value_object_validation_failure() {
        let email = TestEmail(String::from("invalid"));
        assert!(email.validate().is_err());
    }
}
