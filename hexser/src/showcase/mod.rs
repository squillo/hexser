//! Showcase module for developer-experience traits.
//!
//! Provides convenience traits that demonstrate the introspection and
//! self-documentation capabilities of the crate's graph-based approach:
//! `Describable`/`PrettyPrint` for human-readable output, `Inspectable` for
//! programmatic introspection, and `Visualizable` for diagram export. These
//! showcase how components can describe themselves and be inspected.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Consolidate the accidentally-tripled module doc header into one block (fixes doc_lazy_continuation).
//! - 2025-10-02T17:00:00Z @AI: Showcase module creation.
//! - 2025-10-02T12:00:00Z @AI: Add Describable and Inspectable.
//! - 2025-10-01T00:04:00Z @AI: Initial showcase module for DX improvements.

pub mod describable;
pub mod inspectable;
pub mod visualizable;

pub use describable::{ArcGraphExt, Describable, PrettyPrint};
pub use inspectable::Inspectable;
