//! Hex Potions: ready-to-mix patterns and examples for the hexer crate.
//!
//! Potions are small, focused, and copy-friendly examples that show how to
//! assemble common hexagonal architecture operations with `hexer`.
//!
//! Goals:
//! - Demonstrate idiomatic component shapes for common scenarios
//! - Stay dependency-free and simple
//! - Serve as living documentation you can paste into your app
//!
//! Featured potions:
//! - auth::signup - Minimal signup flow with a repository and directive
//! - crud::simple - Simple in-memory CRUD repository example
//!
//! Usage:
//! - Browse modules, copy what you need, and adapt to your domain
//! - All examples compile and have tests

pub mod auth;
pub mod crud;

pub use hexer as core; // convenient alias if you want to explore in docs
