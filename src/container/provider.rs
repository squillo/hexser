//! Provider trait for creating service instances.
//!
//! Defines the interface for service factories in the dependency injection
//! container. Providers are responsible for constructing instances of services
//! with their dependencies resolved. This trait enables polymorphic service
//! creation and supports both manual and automatic dependency resolution.
//!
//! Revision History
//! - 2025-10-02T20:00:00Z @AI: Initial provider trait for Phase 6.

/// Provider for creating service instances
///
/// Implementors define how to construct instances of type `T`,
/// potentially using other services from the container.
pub trait Provider<T>: Send + Sync {
    /// Create new instance of service
    ///
    /// Called by container when resolving a service. For Singleton scope,
    /// this is called once and cached. For Transient scope, called on
    /// every resolution.
    ///
    /// # Returns
    /// New instance of the service
    ///
    /// # Errors
    /// Returns ContainerError if instance creation fails
    fn provide(&self) -> crate::result::hex_result::HexResult<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        value: i32,
    }

    struct TestProvider {
        value: i32,
    }

    impl Provider<TestService> for TestProvider {
        fn provide(&self) -> crate::result::hex_result::HexResult<TestService> {
            Ok(TestService { value: self.value })
        }
    }

    #[test]
    fn test_provider_creates_instance() {
        let provider = TestProvider { value: 42 };
        let service = provider.provide().unwrap();
        assert_eq!(service.value, 42);
    }

    #[test]
    fn test_provider_creates_multiple_instances() {
        let provider = TestProvider { value: 10 };
        let service1 = provider.provide().unwrap();
        let service2 = provider.provide().unwrap();

        assert_eq!(service1.value, 10);
        assert_eq!(service2.value, 10);
    }
}
