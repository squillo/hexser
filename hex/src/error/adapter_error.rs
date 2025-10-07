//! Adapter-specific error types and utilities.
//!
//! Provides AdapterError type alias for infrastructure failures.
//! Adapter errors represent failures in concrete technology implementations
//! like databases, APIs, and external services.
//! Includes error chaining, rich context, and actionable guidance via RichError trait.
//!
//! Revision History
//! - 2025-10-06T01:00:00Z @AI: Refactor to use LayerError generic for Phase 1.
//! - 2025-10-06T00:00:00Z @AI: Implement AdapterError struct for Phase 1.
//! - 2025-10-01T00:00:00Z @AI: Initial placeholder for adapter error utilities.

use crate::error::RichError;

/// Adapter layer error type
pub type AdapterError = crate::error::layer_error::LayerError<
    crate::error::layer_error::layer_markers::AdapterLayer
>;

/// Create connection failure error for any driver
pub fn connection_failed(driver: impl Into<String>, details: impl Into<String>) -> AdapterError {
    AdapterError::new(
        crate::error::codes::adapter::DB_CONNECTION_FAILURE,
        format!("{} connection failed: {}", driver.into(), details.into())
    )
    .with_next_step("Check connection configuration")
    .with_suggestion("Verify connection string and credentials")
}

/// Create API failure error
pub fn api_failure(details: impl Into<String>) -> AdapterError {
    AdapterError::new(crate::error::codes::adapter::API_FAILURE, details)
        .with_next_step("Check API endpoint availability")
        .with_suggestion("Verify API credentials and network connectivity")
}

/// Create mapping failure error
pub fn mapping_failure(details: impl Into<String>) -> AdapterError {
    AdapterError::new(crate::error::codes::adapter::MAPPING_FAILURE, details)
        .with_next_step("Verify data structure compatibility")
        .with_suggestion("Check mapping definitions and field types")
}

/// Create IO failure error
pub fn io_failure(details: impl Into<String>) -> AdapterError {
    AdapterError::new(crate::error::codes::io::IO_FAILURE, details)
        .with_next_step("Check file path and permissions")
        .with_suggestion("Verify file exists and is accessible")
}

#[cfg(test)]
mod tests {
  use std::error::Error;
  use super::*;
    use crate::error::rich_error::RichError;

    #[test]
    fn test_adapter_error_creation() {
        let err = AdapterError::new("E_HEX_200", "Database connection failed");
        assert_eq!(err.code(), "E_HEX_200");
        assert_eq!(err.message(), "Database connection failed");
    }

    #[test]
    fn test_adapter_error_builder() {
        let err = AdapterError::new("E_HEX_201", "API failure")
            .with_next_step("Check API credentials")
            .with_suggestion("Verify endpoint URL");

        assert_eq!(err.next_steps().len(), 1);
        assert_eq!(err.suggestions().len(), 1);
    }

    #[test]
    fn test_adapter_error_with_source() {
        let inner = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let err = AdapterError::new("E_HEX_200", "Database connection failed")
            .with_source(inner);

        assert!(err.source().is_some());
    }
}
