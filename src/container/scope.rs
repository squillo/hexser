//! Service lifetime scope enumeration.
//!
//! Defines how service instances are created and shared across the application.
//! Singleton instances are created once and reused, while Transient instances
//! are created fresh for each resolution. This enables fine-grained control
//! over resource management and object lifecycle.
//!
//! Revision History
//! - 2025-10-02T20:00:00Z @AI: Initial scope implementation for Phase 6.

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
    use super::*;

    #[test]
    fn test_scope_singleton_check() {
        assert!(Scope::Singleton.is_singleton());
        assert!(!Scope::Singleton.is_transient());
    }

    #[test]
    fn test_scope_transient_check() {
        assert!(Scope::Transient.is_transient());
        assert!(!Scope::Transient.is_singleton());
    }

    #[test]
    fn test_scope_equality() {
        assert_eq!(Scope::Singleton, Scope::Singleton);
        assert_eq!(Scope::Transient, Scope::Transient);
        assert_ne!(Scope::Singleton, Scope::Transient);
    }
}
