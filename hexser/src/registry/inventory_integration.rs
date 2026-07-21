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

  /// why: iterating the inventory-registered components must not panic and must be repeatable
  /// (the registry is link-time-fixed), so two iterations yield the same count. (The old
  /// assertion `len() >= 0` was vacuous.)
  #[test]
  fn test_component_iteration() {
    let first: Vec<_> = iter_components().collect();
    let second: Vec<_> = iter_components().collect();
    assert_eq!(first.len(), second.len());
  }
}
