//! Domain layer module containing core business logic types and traits.
//!
//! The domain layer is the heart of hexagonal architecture, containing all
//! business logic without any dependencies on infrastructure or frameworks.
//! This module provides traits for entities, value objects, aggregates,
//! domain events, and domain services.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Add MCP domain module for Model Context Protocol support.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 domain module structure.

pub mod aggregate;
pub mod domain_event;
pub mod domain_service;
pub mod entity;
pub mod value_object;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use aggregate::Aggregate;
pub use domain_event::DomainEvent;
pub use domain_service::DomainService;
pub use entity::HexEntity;
pub use value_object::HexValueItem;
