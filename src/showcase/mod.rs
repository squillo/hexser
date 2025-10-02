//! Showcase module for developer experience traits.
//!
//! This module provides traits that demonstrate the benefits of using hex
//! for hexagonal architecture. These traits offer convenience methods,
//! human-readable output, and introspection capabilities that showcase
//! the power of the graph-based approach.
//!
//! Revision History
//! - 2025-10-01T00:04:00Z @AI: Initial showcase module for DX improvements.
//! Showcase traits demonstrating developer experience benefits.
//!
//! This module provides traits that showcase the introspection and
//! self-documentation capabilities of the hex crate. These traits
//! demonstrate how components can describe themselves and be inspected.
//!
//! Revision History
//! - 2025-10-02T12:00:00Z @AI: Initial showcase module with Describable and Inspectable.

pub mod describable;
pub mod inspectable;

pub use describable::{Describable, PrettyPrint, ArcGraphExt};
pub use inspectable::Inspectable;
