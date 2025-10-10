//! Error types module with rich, actionable error information.
//!
//! Provides comprehensive error types following ERRORS_PROMPT.md guidelines.
//! All errors include error codes, error chaining, rich context, and actionable
//! guidance. Uses trait-based approach for layer errors to eliminate duplication.
//! Structured for both human and AI consumption.
//!
//! # Macros
//!
//! Error construction macros automatically capture source location:
//!
//! ```rust,ignore
//! use hexser::{hex_domain_error, error::codes};
//!
//! let err = hex_domain_error!(
//!     codes::domain::INVARIANT_VIOLATION,
//!     "Order must have items",
//!     next_steps: ["Add at least one item"],
//!     suggestions: ["order.add_item(item)"]
//! );
//! ```
//!
//! Revision History
//! - 2025-10-09T21:51:00Z @AI: Add env_control module for conditional source location serialization.
//! - 2025-10-06T03:00:00Z @AI: Add error construction macros for Phase 2.
//! - 2025-10-06T01:00:00Z @AI: Add RichError trait and LayerError generic for Phase 1.
//! - 2025-10-06T00:00:00Z @AI: Add new error structs and source location for Phase 1.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 error module structure.

pub mod adapter_error;
pub mod codes;
pub mod conflict_error;
pub mod domain_error;
pub mod env_control;
pub mod hex_error;
pub mod layer_error;
pub mod not_found_error;
pub mod port_error;
pub mod rich_error;
pub mod source_location;
pub mod validation_error;

pub use codes as error_codes;
pub use hex_error::Hexserror;
pub use rich_error::RichError;
