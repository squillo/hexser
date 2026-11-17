//! Common utilities shared across all derive macros.
//!
//! This module provides helper functions for parsing attributes,
//! extracting metadata, and validating derive targets.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 — `attrs`, the one strict `#[hex(...)]` reader.
//! - 2026-07-20T00:00:00Z @AI: Remove dead attributes/metadata modules; add shared codegen module.
//! - 2025-10-02T00:00:00Z @AI: Initial common utilities module.

pub mod attrs;
pub mod codegen;
pub mod validation;
