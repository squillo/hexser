//! Hex Potions: ready-to-mix patterns and examples for the hexser crate.
//!
//! Potions are small, focused, and copy-friendly examples that show how to
//! assemble common hexagonal architecture operations with `hexser`.
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
//!
//! Revision History
//! - 2026-11-17T16:20:28Z @AI: HEXSER-0.7 D9 — `#![forbid(unsafe_code)]` at the crate root, pinned by `forbid_unsafe_tests`.

#![forbid(unsafe_code)]

pub mod auth;
pub mod crud;

pub use hexser as core; // convenient alias if you want to explore in docs

#[cfg(test)]
mod forbid_unsafe_tests {
  /// why: the crate promises no `unsafe` (house rule; ISO 27001 A.8.28 evidence row E1 in
  /// PRD HEXSER-0.7 §13). The compiler enforces it only while this attribute stands at the
  /// crate root, so the attribute itself is pinned: deleting it turns this red.
  #[test]
  fn this_crate_forbids_unsafe_code() {
    let source = include_str!("lib.rs");
    std::assert!(
      source
        .lines()
        .any(|line| line.trim() == "#![forbid(unsafe_code)]"),
      "src/lib.rs must carry `#![forbid(unsafe_code)]` at the crate root"
    );
  }
}
