//! Domain layer module containing core business logic types and traits.
//!
//! The domain layer is the heart of hexagonal architecture, containing all
//! business logic without any dependencies on infrastructure or frameworks.
//! This module provides traits for entities, value objects, aggregates,
//! domain events, and domain services.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 domain module structure.

pub mod entity;
pub mod value_object;
pub mod aggregate;
pub mod domain_event;
pub mod domain_service;

pub use entity::Entity;
pub use value_object::ValueObject;
pub use aggregate::Aggregate;
pub use domain_event::DomainEvent;
pub use domain_service::DomainService;
