//! Ports layer module containing interface definitions for external interactions.
//!
//! Ports define the interfaces through which the application interacts with
//! the outside world. They are abstract contracts that adapters will implement.
//! This module provides traits for input ports, output ports, repositories,
//! use cases, queries (CQRS pattern), and CloudEvents v1.0-compliant event ports.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Add events module with CloudEvents v1.0 ports.
//! - 2025-10-08T23:35:00Z @AI: Add mcp_server port for Model Context Protocol support.
//! - 2025-10-08T22:54:00Z @AI: Remove weather_port module (moved to examples).
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 ports module structure.

pub mod events;
pub mod input_port;
pub mod output_port;
pub mod query;
pub mod repository;
pub mod use_case;

#[cfg(feature = "mcp")]
pub mod mcp_server;

pub use input_port::InputPort;
pub use output_port::OutputPort;
pub use query::Query;
pub use repository::Repository;
pub use use_case::UseCase;

// Re-export CloudEvents v1.0 types and traits
pub use events::{
  CLOUDEVENTS_SPEC_VERSION, CloudEventsEnvelope, EventCodec, EventPublisher, EventRouter,
  EventSubscriber,
};
