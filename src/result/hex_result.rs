//! HexResult type alias for consistent error handling.
//!
//! This module provides the standard Result type used throughout the hex crate.
//! HexResult is a type alias for Result<T, HexError>, providing consistent
//! error handling and reducing boilerplate in function signatures.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial HexResult type alias definition.

/// Standard Result type for hex crate operations.
///
/// This is a type alias for `Result<T, HexError>`, providing consistent
/// error handling throughout the crate.
///
/// # Example
///
/// ```rust
/// use hex::HexResult;
///
/// fn do_something() -> HexResult<String> {
///     Ok(String::from("success"))
/// }
/// ```
pub type HexResult<T> = Result<T, crate::error::hex_error::HexError>;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_function() -> HexResult<i32> {
        Result::Ok(42)
    }

    #[test]
    fn test_hex_result_ok() {
        let result = test_function();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_hex_result_err() {
        let result: HexResult<i32> = Result::Err(crate::error::hex_error::HexError::validation("test"));
        assert!(result.is_err());
    }
}
