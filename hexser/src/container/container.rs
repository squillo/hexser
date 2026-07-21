//! Dependency injection container implementation with async support.
//!
//! Provides thread-safe service management with lifetime scoping and
//! dependency resolution. Container uses Arc internally for zero-cost
//! cloning and sharing across threads. Services are registered with
//! providers and scopes, then resolved on demand with automatic caching
//! for Singleton instances.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Never hold container locks across provider execution; singletons use a per-entry OnceCell (lock-free reads, single init, no deadlock on concurrent/nested resolution of distinct services).
//! - 2025-10-02T20:45:00Z @AI: Clean async-only implementation with tokio::sync::RwLock.
//! - 2025-10-02T20:40:00Z @AI: Simplify to tokio::sync::RwLock when container feature enabled.
//! - 2025-10-02T20:35:00Z @AI: Fix async compatibility by using tokio::sync::RwLock.
//! - 2025-10-02T20:30:00Z @AI: Add async resolution support for Phase 6.2.
//! - 2025-10-02T20:00:00Z @AI: Initial container implementation for Phase 6.

/// Dependency injection container
///
/// Thread-safe container for managing service lifecycles and dependencies.
/// Uses Arc internally for efficient cloning and sharing across threads.
/// Uses tokio::sync::RwLock for async compatibility.
pub struct Container {
  inner: std::sync::Arc<ContainerInner>,
}

struct ContainerInner {
  services: tokio::sync::RwLock<std::collections::HashMap<String, ServiceEntry>>,
}

struct ServiceEntry {
  scope: crate::container::scope::Scope,
  factory: std::sync::Arc<dyn std::any::Any + Send + Sync>,
  /// Per-service singleton cache. Wrapped in `Arc` so it can be cloned out of the services map
  /// (dropping the map lock) before the provider runs; `OnceCell` guarantees exactly one
  /// initialization, has lock-free reads once populated, and never holds a container-wide lock.
  singleton_cache:
    std::sync::Arc<tokio::sync::OnceCell<std::sync::Arc<dyn std::any::Any + Send + Sync>>>,
}

impl Container {
  /// Create new empty container
  ///
  /// # Example
  /// ```
  /// # use hexser::container::Container;
  /// let container = Container::new();
  /// ```
  pub fn new() -> Self {
    Self {
      inner: std::sync::Arc::new(ContainerInner {
        services: tokio::sync::RwLock::const_new(std::collections::HashMap::new()),
      }),
    }
  }

  /// Register service with provider and scope
  ///
  /// # Arguments
  /// * `name` - Unique service identifier
  /// * `provider` - Factory for creating instances
  /// * `scope` - Lifetime scope for instances
  ///
  /// # Errors
  /// Returns error if service already exists
  pub async fn register<T: 'static + Send + Sync>(
    &self,
    name: impl Into<String>,
    provider: impl crate::container::provider::Provider<T> + 'static,
    scope: crate::container::scope::Scope,
  ) -> crate::result::hex_result::HexResult<()> {
    let name = name.into();
    let mut services = self.inner.services.write().await;

    if services.contains_key(&name) {
      return Err(
        crate::error::hex_error::Hexserror::validation(&format!(
          "Service {name} already registered"
        ))
        .with_next_step("Use different service name or remove existing registration"),
      );
    }

    let boxed_provider: Box<dyn crate::container::provider::Provider<T>> = Box::new(provider);
    services.insert(
      name,
      ServiceEntry {
        scope,
        factory: std::sync::Arc::new(boxed_provider),
        singleton_cache: std::sync::Arc::new(tokio::sync::OnceCell::new()),
      },
    );

    Ok(())
  }

  /// Resolve service instance by name
  ///
  /// For Singleton scope, returns cached instance if available.
  /// For Transient scope, creates new instance on every call.
  ///
  /// # Errors
  /// Returns error if service not registered or creation fails
  pub async fn resolve<T: 'static + Send + Sync>(
    &self,
    name: &str,
  ) -> crate::result::hex_result::HexResult<std::sync::Arc<T>> {
    // Hold the services read lock only long enough to clone the factory, scope, and singleton
    // cache out of the map. The provider is invoked *after* the lock is dropped, so provider
    // code that resolves other services (the natural DI pattern) can re-acquire the map lock
    // without deadlocking, and a slow provider never blocks registration.
    let (scope, factory, cache) = {
      let services = self.inner.services.read().await;
      let entry = services
        .get(name)
        .ok_or_else(|| crate::error::hex_error::Hexserror::not_found("Service", name))?;
      (
        entry.scope,
        std::sync::Arc::clone(&entry.factory),
        std::sync::Arc::clone(&entry.singleton_cache),
      )
    };

    match scope {
      crate::container::scope::Scope::Singleton => {
        let cached = cache
          .get_or_try_init(|| async {
            let provider = factory
              .downcast_ref::<Box<dyn crate::container::provider::Provider<T>>>()
              .ok_or_else(|| {
                crate::error::hex_error::Hexserror::adapter("E_CNT_005", "Provider type mismatch")
              })?;
            let instance = provider.provide()?;
            std::result::Result::Ok::<_, crate::error::hex_error::Hexserror>(std::sync::Arc::new(
              instance,
            )
              as std::sync::Arc<dyn std::any::Any + Send + Sync>)
          })
          .await?;
        cached
          .clone()
          .downcast::<T>()
          .map_err(|_| crate::error::hex_error::Hexserror::adapter("E_CNT_004", "Type mismatch"))
      }
      crate::container::scope::Scope::Transient => {
        let provider = factory
          .downcast_ref::<Box<dyn crate::container::provider::Provider<T>>>()
          .ok_or_else(|| {
            crate::error::hex_error::Hexserror::adapter("E_CNT_006", "Provider type mismatch")
          })?;

        let instance = provider.provide()?;
        Ok(std::sync::Arc::new(instance))
      }
    }
  }

  /// Check if service is registered
  pub async fn contains(&self, name: &str) -> bool {
    self.inner.services.read().await.contains_key(name)
  }

  /// Get count of registered services
  pub async fn service_count(&self) -> usize {
    self.inner.services.read().await.len()
  }

  #[cfg(feature = "container")]
  /// Register async service with provider and scope
  ///
  /// # Arguments
  /// * `name` - Unique service identifier
  /// * `provider` - Async factory for creating instances
  /// * `scope` - Lifetime scope for instances
  ///
  /// # Errors
  /// Returns error if service already exists
  pub async fn register_async<T: 'static + Send + Sync>(
    &self,
    name: impl Into<String>,
    provider: impl crate::container::async_provider::AsyncProvider<T> + 'static,
    scope: crate::container::scope::Scope,
  ) -> crate::result::hex_result::HexResult<()> {
    let name = name.into();
    let mut services = self.inner.services.write().await;

    if services.contains_key(&name) {
      return Err(
        crate::error::hex_error::Hexserror::validation(&format!(
          "Service {name} already registered"
        ))
        .with_next_step("Use different service name or remove existing registration"),
      );
    }

    let boxed_provider: Box<dyn crate::container::async_provider::AsyncProvider<T>> =
      Box::new(provider);
    services.insert(
      name,
      ServiceEntry {
        scope,
        factory: std::sync::Arc::new(boxed_provider),
        singleton_cache: std::sync::Arc::new(tokio::sync::OnceCell::new()),
      },
    );

    Ok(())
  }

  #[cfg(feature = "container")]
  /// Resolve service instance asynchronously using async provider
  ///
  /// For Singleton scope, returns cached instance if available.
  /// For Transient scope, creates new instance on every call.
  /// Uses AsyncProvider for true async service creation.
  ///
  /// # Errors
  /// Returns error if service not registered or creation fails
  pub async fn resolve_async<T: 'static + Send + Sync>(
    &self,
    name: &str,
  ) -> crate::result::hex_result::HexResult<std::sync::Arc<T>> {
    // See `resolve`: clone what is needed out of the map under a brief read lock, then run the
    // async provider with no container lock held. This is what makes nested async resolution
    // (a provider awaiting `resolve_async` for a dependency) safe from deadlock.
    let (scope, factory, cache) = {
      let services = self.inner.services.read().await;
      let entry = services
        .get(name)
        .ok_or_else(|| crate::error::hex_error::Hexserror::not_found("Service", name))?;
      (
        entry.scope,
        std::sync::Arc::clone(&entry.factory),
        std::sync::Arc::clone(&entry.singleton_cache),
      )
    };

    match scope {
      crate::container::scope::Scope::Singleton => {
        let cached = cache
          .get_or_try_init(|| async {
            let provider = factory
              .downcast_ref::<Box<dyn crate::container::async_provider::AsyncProvider<T>>>()
              .ok_or_else(|| {
                crate::error::hex_error::Hexserror::adapter(
                  "E_CNT_007",
                  "Async provider type mismatch",
                )
              })?;
            let instance = provider.provide_async().await?;
            std::result::Result::Ok::<_, crate::error::hex_error::Hexserror>(std::sync::Arc::new(
              instance,
            )
              as std::sync::Arc<dyn std::any::Any + Send + Sync>)
          })
          .await?;
        cached
          .clone()
          .downcast::<T>()
          .map_err(|_| crate::error::hex_error::Hexserror::adapter("E_CNT_004", "Type mismatch"))
      }
      crate::container::scope::Scope::Transient => {
        let provider = factory
          .downcast_ref::<Box<dyn crate::container::async_provider::AsyncProvider<T>>>()
          .ok_or_else(|| {
            crate::error::hex_error::Hexserror::adapter("E_CNT_008", "Async provider type mismatch")
          })?;

        let instance = provider.provide_async().await?;
        Ok(std::sync::Arc::new(instance))
      }
    }
  }
}

impl Clone for Container {
  fn clone(&self) -> Self {
    Self {
      inner: std::sync::Arc::clone(&self.inner),
    }
  }
}

impl Default for Container {
  fn default() -> Self {
    Self::new()
  }
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

  impl crate::container::provider::Provider<TestService> for TestProvider {
    fn provide(&self) -> crate::result::hex_result::HexResult<TestService> {
      Ok(TestService { value: self.value })
    }
  }

  #[tokio::test]
  async fn test_container_new() {
    let container = Container::new();
    assert_eq!(container.service_count().await, 0);
  }

  #[tokio::test]
  async fn test_container_register() {
    let container = Container::new();
    let provider = TestProvider { value: 42 };

    container
      .register(
        "test_service",
        provider,
        crate::container::scope::Scope::Singleton,
      )
      .await
      .unwrap();

    assert_eq!(container.service_count().await, 1);
    assert!(container.contains("test_service").await);
  }

  #[tokio::test]
  async fn test_container_duplicate_registration_fails() {
    let container = Container::new();
    let provider1 = TestProvider { value: 1 };
    let provider2 = TestProvider { value: 2 };

    container
      .register("test", provider1, crate::container::scope::Scope::Singleton)
      .await
      .unwrap();
    let result = container
      .register("test", provider2, crate::container::scope::Scope::Singleton)
      .await;

    assert!(result.is_err());
  }

  #[tokio::test]
  async fn test_container_clone_shares_services() {
    let container1 = Container::new();
    let provider = TestProvider { value: 10 };

    container1
      .register(
        "shared",
        provider,
        crate::container::scope::Scope::Singleton,
      )
      .await
      .unwrap();

    let container2 = container1.clone();
    assert!(container2.contains("shared").await);
    assert_eq!(container2.service_count().await, 1);
  }

  struct CountingProvider {
    calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
  }

  impl crate::container::provider::Provider<TestService> for CountingProvider {
    fn provide(&self) -> crate::result::hex_result::HexResult<TestService> {
      self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      Ok(TestService { value: 7 })
    }
  }

  /// why: a Singleton provider must run exactly once even when many tasks resolve it
  /// concurrently, and every caller must get the same instance. The previous code took an
  /// exclusive write lock on every resolve (M26); this guards both single-init and sharing.
  #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
  async fn test_singleton_provider_runs_exactly_once_under_concurrency() {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let container = Container::new();
    container
      .register(
        "svc",
        CountingProvider {
          calls: calls.clone(),
        },
        crate::container::scope::Scope::Singleton,
      )
      .await
      .unwrap();

    let mut handles = std::vec::Vec::new();
    for _ in 0..16 {
      let c = container.clone();
      handles.push(tokio::spawn(async move {
        c.resolve::<TestService>("svc").await.unwrap()
      }));
    }
    let mut arcs = std::vec::Vec::new();
    for h in handles {
      arcs.push(h.await.unwrap());
    }

    assert_eq!(
      calls.load(std::sync::atomic::Ordering::SeqCst),
      1,
      "singleton provider must run exactly once"
    );
    assert_eq!(arcs[0].value, 7);
    for a in &arcs {
      assert!(
        std::sync::Arc::ptr_eq(&arcs[0], a),
        "all resolves must return the same singleton instance"
      );
    }
  }

  struct RegisteringProvider {
    container: Container,
  }

  #[async_trait::async_trait]
  impl crate::container::async_provider::AsyncProvider<TestService> for RegisteringProvider {
    async fn provide_async(&self) -> crate::result::hex_result::HexResult<TestService> {
      // Register another service from within a provider. This re-acquires the services write
      // lock; before the fix the services read lock was still held across provider execution,
      // so this deadlocked. It must now complete.
      self
        .container
        .register(
          "late",
          TestProvider { value: 99 },
          crate::container::scope::Scope::Singleton,
        )
        .await?;
      Ok(TestService { value: 1 })
    }
  }

  /// why: a provider that touches the container (here, registering a service) must not deadlock,
  /// proving no container lock is held across provider execution (M25/M32). Bounded by a timeout
  /// so a regression fails fast instead of hanging the suite.
  #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
  async fn test_provider_touching_container_does_not_deadlock() {
    let container = Container::new();
    container
      .register_async(
        "early",
        RegisteringProvider {
          container: container.clone(),
        },
        crate::container::scope::Scope::Singleton,
      )
      .await
      .unwrap();

    let resolved = tokio::time::timeout(
      std::time::Duration::from_secs(5),
      container.resolve_async::<TestService>("early"),
    )
    .await;

    assert!(
      resolved.is_ok(),
      "resolve_async deadlocked: a provider re-entered the container"
    );
    assert!(resolved.unwrap().is_ok());
    assert!(container.contains("late").await);
  }
}
