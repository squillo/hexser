//! Error types module with rich, actionable error information.
//!
//! Provides comprehensive error types following ERRORS_PROMPT.md guidelines.
//! All errors include error codes, error chaining, rich context, and actionable
//! guidance. Uses trait-based approach for layer errors to eliminate duplication.
//! Structured for both human and AI consumption.
//!
//! # Constructing errors
//!
//! Build rich errors with the `Hexserror` constructors and builder methods:
//!
//! ```rust
//! use hexser::error::hex_error::Hexserror;
//! use hexser::error::codes;
//!
//! let err = Hexserror::domain(codes::domain::INVARIANT_VIOLATION, "Order must have items")
//!     .with_next_step("Add at least one item")
//!     .with_suggestion("order.add_item(item)");
//! ```
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Replace docs for the removed error macros with the real Hexserror constructor API.
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
