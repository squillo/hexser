//! In-memory database adapter module declarations.
//!
//! This module contains in-memory implementations of all repository ports
//! for the RealWorld API, providing thread-safe storage for testing and demonstration.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Add tag_adapter for tag repository implementation.
//! - 2025-10-09T22:14:00Z @AI: Initial in-memory database adapter declarations.

pub mod user_adapter;
pub mod article_adapter;
pub mod comment_adapter;
pub mod tag_adapter;
