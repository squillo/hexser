//! Error code registry for hex crate errors.
//!
//! This module provides a centralized registry of all error codes used
//! throughout the hex crate. Each error code is documented with its meaning,
//! when it occurs, and how to resolve it. Error codes follow the format
//! E_HEX_XXX where XXX is a three-digit number.
//!
//! Revision History
//! - 2025-10-01T00:02:00Z @AI: Initial error code registry for Phase 1.

/// Domain layer error codes (E_HEX_001 - E_HEX_099).
pub mod domain {
    /// Occurs when a container must contain at least one item.
    ///
    /// Occurs when attempting to create or save a container with no items.
    /// Resolution: Add items to the container before saving.
    pub const INVARIANT_EMPTY: &str = "E_HEX_001";

    /// Aggregate invariant violation.
    ///
    /// Occurs when an aggregate's business rules are violated.
    /// Resolution: Check aggregate documentation for required invariants.
    pub const INVARIANT_VIOLATION: &str = "E_HEX_002";

    /// Invalid entity state transition.
    ///
    /// Occurs when attempting an illegal state transition.
    /// Resolution: Verify the entity lifecycle and valid transitions.
    pub const INVALID_STATE_TRANSITION: &str = "E_HEX_003";
}

/// Port layer error codes (E_HEX_100 - E_HEX_199).
pub mod port {
    /// Port communication failure.
    ///
    /// Occurs when communication with a port fails.
    /// Resolution: Check port implementation and connectivity.
    pub const COMMUNICATION_FAILURE: &str = "E_HEX_100";

    /// Port not found.
    ///
    /// Occurs when a required port is not available.
    /// Resolution: Ensure the port is properly registered.
    pub const PORT_NOT_FOUND: &str = "E_HEX_101";

    /// Port timeout.
    ///
    /// Occurs when a port operation times out.
    /// Resolution: Increase timeout or check port responsiveness.
    pub const PORT_TIMEOUT: &str = "E_HEX_102";
}

/// Adapter layer error codes (E_HEX_200 - E_HEX_299).
pub mod adapter {
    /// Database connection failure.
    ///
    /// Occurs when unable to connect to database.
    /// Resolution: Check database configuration and availability.
    pub const DB_CONNECTION_FAILURE: &str = "E_HEX_200";

    /// External API failure.
    ///
    /// Occurs when external API call fails.
    /// Resolution: Check API endpoint, credentials, and network.
    pub const API_FAILURE: &str = "E_HEX_201";

    /// Data mapping failure.
    ///
    /// Occurs when mapping between representations fails.
    /// Resolution: Verify data structure compatibility.
    pub const MAPPING_FAILURE: &str = "E_HEX_202";
}

/// Validation error codes (E_HEX_300 - E_HEX_399).
pub mod validation {
    /// Required field missing.
    ///
    /// Occurs when a required field is not provided.
    /// Resolution: Provide all required fields.
    pub const REQUIRED_FIELD: &str = "E_HEX_300";

    /// Invalid format.
    ///
    /// Occurs when field value has invalid format.
    /// Resolution: Check format requirements and examples.
    pub const INVALID_FORMAT: &str = "E_HEX_301";

    /// Value out of range.
    ///
    /// Occurs when value exceeds allowed range.
    /// Resolution: Provide value within valid range.
    pub const OUT_OF_RANGE: &str = "E_HEX_302";
}

/// Resource error codes (E_HEX_400 - E_HEX_499).
pub mod resource {
    /// Resource not found.
    ///
    /// Occurs when requested resource doesn't exist.
    /// Resolution: Verify resource ID and existence.
    pub const NOT_FOUND: &str = "E_HEX_400";

    /// Resource already exists.
    ///
    /// Occurs when attempting to create duplicate resource.
    /// Resolution: Use update instead of create, or use different ID.
    pub const ALREADY_EXISTS: &str = "E_HEX_401";

    /// Resource conflict.
    ///
    /// Occurs when resource state conflicts with operation.
    /// Resolution: Resolve conflict or retry operation.
    pub const CONFLICT: &str = "E_HEX_402";
}

/// IO error codes (E_HEX_500 - E_HEX_599).
pub mod io {
    /// File not found.
    ///
    /// Occurs when attempting to access non-existent file.
    /// Resolution: Verify file path and existence.
    pub const FILE_NOT_FOUND: &str = "E_HEX_500";

    /// Permission denied.
    ///
    /// Occurs when lacking permissions for file operation.
    /// Resolution: Check file permissions and user access rights.
    pub const PERMISSION_DENIED: &str = "E_HEX_501";

    /// IO operation failed.
    ///
    /// Occurs when general IO operation fails.
    /// Resolution: Check system resources and retry.
    pub const IO_FAILURE: &str = "E_HEX_502";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes_unique() {
        let codes = vec![
          domain::INVARIANT_EMPTY,
          domain::INVARIANT_VIOLATION,
          domain::INVALID_STATE_TRANSITION,
          port::COMMUNICATION_FAILURE,
          port::PORT_NOT_FOUND,
          port::PORT_TIMEOUT,
          adapter::DB_CONNECTION_FAILURE,
          adapter::API_FAILURE,
          adapter::MAPPING_FAILURE,
          validation::REQUIRED_FIELD,
          validation::INVALID_FORMAT,
          validation::OUT_OF_RANGE,
          resource::NOT_FOUND,
          resource::ALREADY_EXISTS,
          resource::CONFLICT,
          io::FILE_NOT_FOUND,
          io::PERMISSION_DENIED,
          io::IO_FAILURE,
        ];

        let unique_codes: std::collections::HashSet<_> = codes.iter().collect();
        assert_eq!(codes.len(), unique_codes.len(), "All error codes must be unique");
    }

    #[test]
    fn test_error_code_format() {
        let codes = vec![
          domain::INVARIANT_EMPTY,
          port::COMMUNICATION_FAILURE,
          adapter::DB_CONNECTION_FAILURE,
          io::FILE_NOT_FOUND,
        ];

        for code in codes {
            assert!(code.starts_with("E_HEX_"), "Error code must start with E_HEX_");
            assert_eq!(code.len(), 9, "Error code must be 9 characters");
        }
    }
}
