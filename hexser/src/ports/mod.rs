//! Ports layer module containing interface definitions for external interactions.
//!
//! Ports define the interfaces through which the application interacts with
//! the outside world. They are abstract contracts that adapters will implement.
//! This module provides traits for input ports, output ports, repositories,
//! use cases, and queries (CQRS pattern).
//!
//! Revision History
//! - 2025-10-08T22:54:00Z @AI: Remove weather_port module (moved to examples).
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 ports module structure.

pub mod input_port;
pub mod output_port;
pub mod repository;
pub mod use_case;
pub mod query;

pub use input_port::InputPort;
pub use output_port::OutputPort;
pub use repository::Repository;
pub use use_case::UseCase;
pub use query::Query;
