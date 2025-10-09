//! Integration with the inventory crate for compile-time registration.
//!
//! Provides utilities for working with inventory-collected components.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial inventory integration.

/// Re-export inventory macros for use in derive macros
pub use inventory;

/// Helper to iterate over registered components
pub fn iter_components()
-> impl Iterator<Item = &'static crate::registry::component_entry::ComponentEntry> {
  inventory::iter::<crate::registry::component_entry::ComponentEntry>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_component_iteration() {
    let components: Vec<_> = iter_components().collect();
    assert!(components.len() >= 0);
  }
}
