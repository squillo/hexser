//! Adapters layer module containing port implementations.
//!
//! Adapters provide concrete implementations of ports using specific technologies.
//! They translate between the domain layer and external systems, databases, APIs,
//! or user interfaces. This module provides traits for marking adapters and
//! mapping data between different representations.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 adapters module structure.

pub mod adapter;
pub mod mapper;

pub use adapter::Adapter;
pub use mapper::Mapper;
