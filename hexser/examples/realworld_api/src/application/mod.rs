//! Application layer module declarations.
//!
//! This module contains all application use cases organized by domain area.
//! Each subdirectory represents a feature area with its directives and queries.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Add tag module with get_all operation.
//! - 2025-10-09T23:49:00Z @AI: Add profile module with get, follow, unfollow operations.
//! - 2025-10-09T23:49:00Z @AI: Add comment module with add, get, delete operations.
//! - 2025-10-09T22:14:00Z @AI: Initial application module declarations.

pub mod user;
pub mod article;
pub mod comment;
pub mod profile;
pub mod tag;
