//! Error types module with rich, actionable error information.
//!
//! This module provides comprehensive error types following ERRORS_PROMPT.md
//! guidelines. All errors are designed to be helpful, empathetic, and actionable,
//! with clear next steps and suggestions for remediation. Errors are structured
//! for both human and AI consumption.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 error module structure.

pub mod hex_error;
pub mod domain_error;
pub mod port_error;
pub mod adapter_error;
pub mod codes;

pub use hex_error::HexError;
pub use codes as error_codes;
