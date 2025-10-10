//! Web adapter module declarations.
//!
//! This module contains all web/HTTP adapter implementations for the RealWorld API,
//! including authentication, routing, and REST API endpoint handlers.
//!
//! Revision History
//! - 2025-10-10T10:05:00Z @AI: Add tests module with comprehensive HTTP endpoint integration tests.
//! - 2025-10-10T08:28:00Z @AI: Initial web adapter module declarations.

pub mod auth;
pub mod user_routes;
pub mod article_routes;
pub mod comment_routes;
pub mod profile_routes;
pub mod tag_routes;
pub mod routes;

#[cfg(test)]
mod tests;
