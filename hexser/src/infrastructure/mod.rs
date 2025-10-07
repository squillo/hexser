//! Infrastructure layer module for external concerns.
//!
//! The infrastructure layer contains technology-specific implementations
//! and configuration for external systems like databases, message queues,
//! web servers, and third-party APIs. This module provides traits for
//! infrastructure configuration and setup.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 infrastructure module structure.

pub mod config;

pub use config::Config;
