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

pub mod container;
pub mod container_error;
pub mod provider;
pub mod scope;

#[cfg(feature = "container")]
pub mod async_provider;

pub use self::{
  container::Container, container_error::ContainerError, provider::Provider, scope::Scope,
};

#[cfg(feature = "container")]
pub use self::async_provider::AsyncProvider;
