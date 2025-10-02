//! Derive macro implementations.
//!
//! Each derive macro (HexDomain, HexPort, HexAdapter, etc.) has its own
//! module with the implementation logic.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial derive implementations module.

pub mod hex_domain;
pub mod hex_port;
pub mod hex_adapter;
pub mod entity;
pub mod repository;
pub mod directive;
pub mod query;
