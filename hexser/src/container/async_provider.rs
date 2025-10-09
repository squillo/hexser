//! Async provider trait for creating service instances asynchronously.
//!
//! Defines interface for async service factories in the dependency injection
//! container. Async providers support asynchronous initialization like
//! database connections, HTTP clients, and other I/O-bound resources.
//! Uses async-trait for ergonomic async trait methods.
//!
//! Revision History
//! - 2025-10-02T20:30:00Z @AI: Initial async provider trait for Phase 6.2.
//! - 2025-10-06T17:22:00Z @AI: Tests: add justifications; remove super import; qualify paths per no-use rule.

#[cfg(feature = "container")]
/// Async provider for creating service instances
///
/// Implementors define how to asynchronously construct instances of type `T`,
/// allowing for async initialization like database connections.
#[async_trait::async_trait]
pub trait AsyncProvider<T>: Send + Sync {
  /// Create new instance of service asynchronously
  ///
  /// Called by container when resolving a service asynchronously.
  /// Supports async operations like network calls, file I/O, etc.
  ///
  /// # Returns
  /// New instance of the service
  ///
  /// # Errors
  /// Returns Hexserror if instance creation fails
  async fn provide_async(&self) -> crate::result::hex_result::HexResult<T>;
}

#[cfg(all(test, feature = "container"))]
mod tests {
  // Note: Per NO `use` STATEMENTS rule, tests reference items via fully qualified paths.
  // This ensures clarity and avoids ambiguous imports.

  struct AsyncTestService {
    value: i32,
  }

  struct AsyncTestProvider {
    value: i32,
  }

  #[async_trait::async_trait]
  impl crate::container::async_provider::AsyncProvider<AsyncTestService> for AsyncTestProvider {
    async fn provide_async(&self) -> crate::result::hex_result::HexResult<AsyncTestService> {
      // Simulate async work
      tokio::time::sleep(std::time::Duration::from_millis(10)).await;
      Ok(AsyncTestService { value: self.value })
    }
  }

  #[tokio::test]
  async fn test_async_provider_creates_instance() {
    // Test: Async provider constructs a service instance with configured value.
    // Justification: Verifies core contract of AsyncProvider::provide_async and async behavior.
    let provider = AsyncTestProvider { value: 42 };
    let service = <AsyncTestProvider as crate::container::async_provider::AsyncProvider<
      AsyncTestService,
    >>::provide_async(&provider)
    .await
    .unwrap();
    assert_eq!(service.value, 42);
  }

  #[tokio::test]
  async fn test_async_provider_concurrent_creation() {
    // Test: Concurrent async creation yields correct values across tasks.
    // Justification: Ensures thread-safety assumptions and absence of shared mutable state in provider.
    let handle1 = tokio::spawn({
      let p = AsyncTestProvider { value: 10 };
      async move {
        <AsyncTestProvider as crate::container::async_provider::AsyncProvider<AsyncTestService>>::provide_async(&p).await
      }
    });

    let handle2 = tokio::spawn({
      let p = AsyncTestProvider { value: 10 };
      async move {
        <AsyncTestProvider as crate::container::async_provider::AsyncProvider<AsyncTestService>>::provide_async(&p).await
      }
    });

    let result1 = handle1.await.unwrap().unwrap();
    let result2 = handle2.await.unwrap().unwrap();

    assert_eq!(result1.value, 10);
    assert_eq!(result2.value, 10);
  }
}
