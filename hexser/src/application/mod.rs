//! Application layer module for use case orchestration.
//!
//! The application layer coordinates between the domain and ports layers,
//! orchestrating use cases without containing business logic itself.
//! This module provides traits for directives, directive handlers, and query
//! handlers, supporting the CQRS (Command Query Responsibility Segregation) pattern.
//! It also provides the Application trait for marking top-level entry points and
//! coordinating system lifecycle.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D7 — the deprecated `DirectiveHandler` stays re-exported under `#[allow(deprecated)]`.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D1 — `actor` module and the `Actor` re-export.
//! - 2025-10-12T17:48:00Z @AI: Add Application trait for marking entry points.
//! - 2025-10-01T00:01:00Z @AI: Renamed Command to Directive for better intent representation.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 application module structure.

pub mod actor;
// One-concept-per-file layout: the `Application` trait lives in application.rs, so the
// submodule shares the layer module's name by design.
#[allow(clippy::module_inception)]
pub mod application;
pub mod directive;
pub mod directive_handler;
pub mod query_handler;

pub use actor::Actor;
pub use application::Application;
pub use directive::Directive;
#[allow(deprecated)] // re-exported until removal (HEXSER-0.7 D7)
pub use directive_handler::DirectiveHandler;
pub use query_handler::QueryHandler;
