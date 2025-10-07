//! Provider trait for creating service instances.
//!
//! Defines the interface for service factories in the dependency injection
//! container. Providers are responsible for constructing instances of services
//! with their dependencies resolved. This trait enables polymorphic service
//! creation and supports both manual and automatic dependency resolution.
//!
//! Revision History
//! - 2025-10-02T20:00:00Z @AI: Initial provider trait for Phase 6.
//! - 2025-10-06T17:22:00Z @AI: Tests: add justifications; remove super import; qualify paths per no-use rule.

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
    // Note: Per NO `use` STATEMENTS rule, tests reference items via fully qualified paths.
    // This ensures clarity and avoids ambiguous imports.

    struct TestService {
        value: i32,
    }

    struct TestProvider {
        value: i32,
    }

    impl crate::container::provider::Provider<TestService> for TestProvider {
        fn provide(&self) -> crate::result::hex_result::HexResult<TestService> {
            Ok(TestService { value: self.value })
        }
    }

    #[test]
    fn test_provider_creates_instance() {
        // Test: Provider constructs a service instance with configured value.
        // Justification: Verifies core contract of Provider::provide for correctness.
        let provider = TestProvider { value: 42 };
        let service = <TestProvider as crate::container::provider::Provider<TestService>>::provide(&provider).unwrap();
        assert_eq!(service.value, 42);
    }

    #[test]
    fn test_provider_creates_multiple_instances() {
        // Test: Provider produces independent instances on each call.
        // Justification: Ensures no hidden caching at the Provider level; caching is a Container concern.
        let provider = TestProvider { value: 10 };
        let service1 = <TestProvider as crate::container::provider::Provider<TestService>>::provide(&provider).unwrap();
        let service2 = <TestProvider as crate::container::provider::Provider<TestService>>::provide(&provider).unwrap();

        assert_eq!(service1.value, 10);
        assert_eq!(service2.value, 10);
    }
}
