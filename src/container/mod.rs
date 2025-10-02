//! Dependency injection container for hexagonal architecture components.
//!
//! Provides zero-boilerplate dependency management with lifetime scoping,
//! thread-safe service resolution, and compile-time circular dependency detection.
//! Follows hexagonal architecture principles where the container itself is
//! an infrastructure concern that manages domain, port, and adapter instances.
//!
//! Revision History
//! - 2025-10-02T20:30:00Z @AI: Add async provider support for Phase 6.2.
//! - 2025-10-02T20:00:00Z @AI: Initial Phase 6 container module implementation.

pub mod scope;
pub mod provider;
pub mod container;
pub mod container_error;

#[cfg(feature = "container")]
pub mod async_provider;

pub use self::scope::Scope;
pub use self::provider::Provider;
pub use self::container::Container;
pub use self::container_error::ContainerError;

#[cfg(feature = "container")]
pub use self::async_provider::AsyncProvider;
