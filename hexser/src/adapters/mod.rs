//! Adapters layer module containing port implementations.
//!
//! Adapters provide concrete implementations of ports using specific technologies.
//! They translate between the domain layer and external systems, databases, APIs,
//! or user interfaces. This module provides traits for marking adapters and
//! mapping data between different representations, including CloudEvents v1.0
//! event bus implementations.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Add in_memory_event_bus adapter for CloudEvents v1.0 support.
//! - 2025-10-08T23:35:00Z @AI: Add mcp_stdio adapter for Model Context Protocol support.
//! - 2025-10-08T22:54:00Z @AI: Remove rest_weather_adapter module (moved to examples).
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 adapters module structure.

pub mod adapter;
pub mod in_memory_event_bus;
pub mod mapper;

#[cfg(feature = "mcp")]
pub mod mcp_stdio;

pub use adapter::Adapter;
pub use in_memory_event_bus::InMemoryEventBus;
pub use mapper::Mapper;
