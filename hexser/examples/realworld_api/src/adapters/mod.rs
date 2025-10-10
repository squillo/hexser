//! Adapters layer module declarations.
//!
//! This module contains all adapter implementations for the RealWorld API,
//! including in-memory database adapters and web API adapters.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Add web adapter module with axum REST API implementation.
//! - 2025-10-09T22:14:00Z @AI: Initial adapters module declarations.

pub mod in_memory_db;
pub mod web;
