//! Service lifetime scope enumeration.
//!
//! Defines how service instances are created and shared across the application.
//! Singleton instances are created once and reused, while Transient instances
//! are created fresh for each resolution. This enables fine-grained control
//! over resource management and object lifecycle.
//!
//! Revision History
//! - 2025-10-02T20:00:00Z @AI: Initial scope implementation for Phase 6.
//! - 2025-10-06T17:11:00Z @AI: Finalize scope: remove test wildcard use; confirm container integration.
//! - 2025-10-06T17:22:00Z @AI: Tests: add justifications for clarity and coverage rationale.

/// Service lifetime scope
///
/// Determines how instances are created and cached in the container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
  /// Single instance shared across entire application
  ///
  /// The service is created once on first resolution and reused
  /// for all subsequent resolutions. Ideal for stateless services
  /// or services managing shared state.
  Singleton,

  /// New instance created for each resolution
  ///
  /// A fresh service instance is created every time it's resolved
  /// from the container. Ideal for stateful services or when
  /// isolation between consumers is required.
  Transient,
}

impl Scope {
  /// Check if scope is Singleton
  pub fn is_singleton(&self) -> bool {
    matches!(self, Scope::Singleton)
  }

  /// Check if scope is Transient
  pub fn is_transient(&self) -> bool {
    matches!(self, Scope::Transient)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_scope_singleton_check() {
    // Test: Verifies Scope::Singleton helper methods.
    // Justification: Ensures API clarity for container logic relying on these checks.
    assert!(crate::container::scope::Scope::Singleton.is_singleton());
    assert!(!crate::container::scope::Scope::Singleton.is_transient());
  }

  #[test]
  fn test_scope_transient_check() {
    // Test: Verifies Scope::Transient helper methods.
    // Justification: Ensures API clarity and prevents logic inversion bugs in consumers.
    assert!(crate::container::scope::Scope::Transient.is_transient());
    assert!(!crate::container::scope::Scope::Transient.is_singleton());
  }

  #[test]
  fn test_scope_equality() {
    // Test: Validates equality semantics for Scope enum.
    // Justification: Prevents regressions affecting HashMap keys and matching logic.
    assert_eq!(
      crate::container::scope::Scope::Singleton,
      crate::container::scope::Scope::Singleton
    );
    assert_eq!(
      crate::container::scope::Scope::Transient,
      crate::container::scope::Scope::Transient
    );
    assert_ne!(
      crate::container::scope::Scope::Singleton,
      crate::container::scope::Scope::Transient
    );
  }
}
