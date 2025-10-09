//! Component registry for compile-time registration.
//!
//! Provides infrastructure for registering hexagonal architecture components
//! at compile time using the inventory pattern, enabling automatic graph construction.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial Phase 3 registry implementation.

pub mod component_entry;
pub mod component_registry;
pub mod inventory_integration;
pub mod node_builder;
pub mod node_info;
pub mod registrable;

pub use component_entry::ComponentEntry;
pub use component_registry::ComponentRegistry;
pub use node_info::NodeInfo;
pub use registrable::Registrable;
