//! Async provider trait for creating service instances asynchronously.
//!
//! Defines interface for async service factories in the dependency injection
//! container. Async providers support asynchronous initialization like
//! database connections, HTTP clients, and other I/O-bound resources.
//! Uses async-trait for ergonomic async trait methods.
//!
//! Revision History
//! - 2025-10-02T20:30:00Z @AI: Initial async provider trait for Phase 6.2.

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
    /// Returns HexError if instance creation fails
    async fn provide_async(&self) -> crate::result::hex_result::HexResult<T>;
}

#[cfg(all(test, feature = "container"))]
mod tests {
    use super::*;

    struct AsyncTestService {
        value: i32,
    }

    struct AsyncTestProvider {
        value: i32,
    }

    #[async_trait::async_trait]
    impl AsyncProvider<AsyncTestService> for AsyncTestProvider {
        async fn provide_async(&self) -> crate::result::hex_result::HexResult<AsyncTestService> {
            // Simulate async work
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            Ok(AsyncTestService { value: self.value })
        }
    }

    #[tokio::test]
    async fn test_async_provider_creates_instance() {
        let provider = AsyncTestProvider { value: 42 };
        let service = provider.provide_async().await.unwrap();
        assert_eq!(service.value, 42);
    }

    #[tokio::test]
    async fn test_async_provider_concurrent_creation() {

        let handle1 = tokio::spawn({
            let p = AsyncTestProvider { value: 10 };
            async move { p.provide_async().await }
        });

        let handle2 = tokio::spawn({
            let p = AsyncTestProvider { value: 10 };
            async move { p.provide_async().await }
        });

        let result1 = handle1.await.unwrap().unwrap();
        let result2 = handle2.await.unwrap().unwrap();

        assert_eq!(result1.value, 10);
        assert_eq!(result2.value, 10);
    }
}
