//! Zero-boilerplate hexagonal architecture with graph-based introspection.
//!
//! The `hex` crate provides reusable types and traits for implementing
//! Hexagonal Architecture (Ports and Adapters) with automatic graph construction,
//! intent inference, and architectural validation. This is Phase 1: Core Foundation,
//! providing the foundational traits, types, and error handling.
//! Future phases will add graph construction, derive macros, and analysis capabilities.
//!
//! # Architecture Layers
//!
//! - **Domain**: Core business logic (`HexEntity`, `HexValueItem`, `Aggregate`)
//! - **Ports**: Interface definitions (`Repository`, `UseCase`, `Query`)
//! - **Adapters**: Port implementations (`Adapter`, `Mapper`)
//! - **Application**: Use case orchestration (`Directive`, `DirectiveHandler`)
//! - **Infrastructure**: External concerns (`Config`)
//!
//! # Quick Start
//!
//! ```rust
//! use hexser::prelude::*;
//!
//! // Define a domain entity
//! struct User {
//!     id: String,
//!     email: String,
//! }
//!
//! impl HexEntity for User {
//!     type Id = String;
//! }
//!
//! // Define a port (interface)
//! trait UserRepository: Repository<User> {
//!     fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
//! }
//!
//! // Implement an adapter
//! struct InMemoryUserRepository {
//!     users: Vec<User>,
//! }
//!
//! impl Adapter for InMemoryUserRepository {}
//! ```
//!
//! # Feature Flags
//!
//! - `default`: Core traits and types (zero dependencies)
//! - `graph`: Graph-based introspection (Phase 2+)
//! - `macros`: Derive macros for zero-boilerplate DX (Phase 3+)
//! - `analysis`: Architectural analysis and validation (Phase 4+)
//!
//! Revision History
//! - 2025-10-02T13:00:00Z @AI: Re-export inventory and error_codes for proc macros.
//! - 2025-10-02T12:00:00Z @AI: Add showcase module with Describable and Inspectable traits.
//! - 2025-10-01T00:01:00Z @AI: Added comprehensive re-exports and prelude module.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 implementation with core traits and types.

pub mod adapters;
pub mod application;
pub mod domain;
pub mod error;
pub mod graph;
pub mod infrastructure;
pub mod ports;
pub mod registry;
pub mod result;
pub mod showcase;
pub mod templates;

#[cfg(feature = "static-di")]
pub mod static_di;

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "container")]
pub mod container;

// Re-export commonly used items at crate root for convenience
pub use crate::{error::hex_error::Hexserror, result::hex_result::HexResult};

// Re-export all domain traits
pub use crate::domain::{Aggregate, DomainEvent, DomainService, HexEntity, HexValueItem};

// Re-export all port traits
pub use crate::ports::{InputPort, OutputPort, Query, Repository, UseCase};

// Re-export all adapter traits
pub use crate::adapters::{Adapter, Mapper};

// Re-export all application traits
pub use crate::application::{Directive, DirectiveHandler, QueryHandler};

// Re-export infrastructure traits
pub use crate::infrastructure::Config;

// Re-export inventory for proc macros
pub use inventory;

// Re-export error codes module
pub use crate::error::codes as error_codes;

// Re-export graph types (Phase 2)
pub use crate::graph::{
  GraphBuilder, GraphMetadata, HexEdge, HexGraph, HexNode, Layer, NodeId, Relationship, Role,
};

// Re-export showcase traits
pub use crate::showcase::{ArcGraphExt, Describable, Inspectable, PrettyPrint};

/// Prelude module for convenient imports.
///
/// Import everything you need with a single use statement:
///
/// ```rust
/// use hexser::prelude::*;
/// ```
pub mod prelude {
  pub use crate::{HexResult, Hexserror};

  pub use crate::domain::{Aggregate, DomainEvent, DomainService, HexEntity, HexValueItem};

  pub use crate::ports::{InputPort, OutputPort, Query, Repository, UseCase};

  pub use crate::adapters::{Adapter, Mapper};

  pub use crate::application::{Directive, DirectiveHandler, QueryHandler};

  pub use crate::infrastructure::Config;

  pub use crate::graph::{
    GraphBuilder, GraphMetadata, HexEdge, HexGraph, HexNode, Layer, NodeId, Relationship, Role,
  };

  pub use crate::showcase::{ArcGraphExt, Describable, Inspectable, PrettyPrint};

  // Phase 3: Registry and derive macro support
  #[cfg(feature = "macros")]
  pub use hexser_macros::{
    Entity, HexAdapter, HexAggregate, HexDirective, HexDomain, HexPort, HexQuery, HexRepository,
    HexValueItem,
  };

  // Alias Entity as HexEntity for consistency with trait name
  #[cfg(feature = "macros")]
  pub use hexser_macros::Entity as HexEntity;

  pub use crate::registry::{ComponentEntry, ComponentRegistry, NodeInfo, Registrable};

  #[cfg(feature = "ai")]
  pub use crate::ai::{AIContext, ContextBuilder};

  #[cfg(feature = "container")]
  pub use crate::container::{Container, Provider, Scope};

  #[cfg(feature = "static-di")]
  pub use crate::static_di::{StaticBuilder, StaticContainer};
}
