//! Adapters for visualization formats.
//!
//! Concrete implementations of format exporters.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial adapters module.

pub mod dot_exporter;
pub mod mermaid_exporter;

#[cfg(feature = "visualization")]
pub mod json_exporter;
