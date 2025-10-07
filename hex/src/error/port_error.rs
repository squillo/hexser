//! Port-specific error types and utilities.
//!
//! Provides PortError type alias for port communication failures.
//! Port errors represent failures in interfaces between domain and adapters.
//! Includes error chaining, rich context, and actionable guidance via RichError trait.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict, remove duplicates.
//! - 2025-10-06T02:00:00Z @AI: Add helper functions for Phase 2.
//! - 2025-10-06T01:00:00Z @AI: Refactor to use LayerError generic for Phase 1.
//! - 2025-10-06T00:00:00Z @AI: Implement PortError struct for Phase 1.
//! - 2025-10-01T00:00:00Z @AI: Initial placeholder for port error utilities.

use crate::error::RichError;

/// Port layer error type
pub type PortError = crate::error::layer_error::LayerError<
    crate::error::layer_error::layer_markers::PortLayer
>;

/// Create communication failure error
pub fn communication_failure(details: impl Into<String>) -> PortError {
    PortError::new(crate::error::codes::port::COMMUNICATION_FAILURE, details)
        .with_next_step("Check port implementation and connectivity")
        .with_suggestion("Verify port configuration and network status")
}

/// Create port not found error
pub fn port_not_found(port_name: impl Into<String>) -> PortError {
    PortError::new(
        crate::error::codes::port::PORT_NOT_FOUND,
        format!("Port not found: {}", port_name.into())
    )
    .with_next_step("Ensure the port is properly registered")
    .with_suggestion("Check port registration in container or registry")
}

/// Create port timeout error
pub fn port_timeout(port_name: impl Into<String>) -> PortError {
    PortError::new(
        crate::error::codes::port::PORT_TIMEOUT,
        format!("Port operation timed out: {}", port_name.into())
    )
    .with_next_step("Increase timeout or check port responsiveness")
    .with_suggestion("Verify network connectivity and port availability")
}

#[cfg(test)]
mod tests {
  use std::error::Error;
  use super::*;
    use crate::error::rich_error::RichError;

    #[test]
    fn test_port_error_creation() {
        let err = PortError::new("E_HEX_100", "Communication failure");
        assert_eq!(err.code(), "E_HEX_100");
        assert_eq!(err.message(), "Communication failure");
    }

    #[test]
    fn test_port_error_builder() {
        let err = PortError::new("E_HEX_101", "Port not found")
            .with_next_step("Register port")
            .with_suggestion("Check port configuration");

        assert_eq!(err.next_steps().len(), 1);
        assert_eq!(err.suggestions().len(), 1);
    }

    #[test]
    fn test_port_error_with_source() {
        let inner = std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout");
        let err = PortError::new("E_HEX_102", "Port timeout")
            .with_source(inner);

        assert!(err.source().is_some());
    }
}
