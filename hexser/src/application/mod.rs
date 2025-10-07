//! Application layer module for use case orchestration.
//!
//! The application layer coordinates between the domain and ports layers,
//! orchestrating use cases without containing business logic itself.
//! This module provides traits for directives, directive handlers, and query
//! handlers, supporting the CQRS (Command Query Responsibility Segregation) pattern.
//!
//! Revision History
//! - 2025-10-01T00:01:00Z @AI: Renamed Command to Directive for better intent representation.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 application module structure.

pub mod directive;
pub mod directive_handler;
pub mod query_handler;

pub use directive::Directive;
pub use directive_handler::DirectiveHandler;
pub use query_handler::QueryHandler;
