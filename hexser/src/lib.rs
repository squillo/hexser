//! Zero-boilerplate hexagonal architecture with graph-based introspection.
//!
//! `hexser` provides reusable traits and types for building applications with Hexagonal
//! Architecture (Ports and Adapters). Components you derive are automatically registered into
//! an in-memory architecture graph you can query, validate, visualize, and export for AI
//! agents — no manual wiring.
//!
//! # Architecture Layers
//!
//! - **Domain**: Core business logic (`HexEntity`, `HexValueItem`, `Aggregate`)
//! - **Ports**: Interface definitions (`Repository`, `QueryRepository`, `UseCase`, `Query`)
//! - **Adapters**: Port implementations (`Adapter`, `Mapper`)
//! - **Application**: Use case orchestration (`Directive`, `DirectiveHandler`)
//! - **Infrastructure**: External concerns (`Config`)
//!
//! # Quick Start
//!
//! ```rust
//! use hexser::prelude::*;
//!
//! // A domain entity: derive HexEntity (Id is taken from the `id` field) and HexDomain to
//! // register it in the architecture graph.
//! #[derive(HexEntity, HexDomain)]
//! struct User {
//!     id: String,
//!     email: String,
//! }
//!
//! // A repository port. Repository is save-only; reads live on QueryRepository.
//! trait UserRepository: Repository<User> {
//!     fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
//! }
//!
//! // An adapter implementing the port. Deriving HexAdapter registers it in the graph and
//! // implements the `Adapter` marker trait for you.
//! #[derive(HexAdapter)]
//! struct InMemoryUserRepository {
//!     users: Vec<User>,
//! }
//! ```
//!
//! # Feature Flags
//!
//! `default = ["macros", "static-di"]`.
//!
//! - `macros`: derive macros (`HexEntity`, `HexDomain`, `HexPort`, …) — on by default.
//! - `static-di`: zero-cost, WASM-friendly static dependency injection — on by default.
//! - `serde`: `Serialize`/`Deserialize` for the rich error types (see also the
//!   `HEXSER_INCLUDE_SOURCE_LOCATION` env var, which gates whether source locations are
//!   included in serialized errors).
//! - `ai`: machine-readable architecture context export (`AIContext`, `ContextBuilder`).
//! - `mcp`: Model Context Protocol server (implies `ai`).
//! - `visualization`: DOT / Mermaid / JSON diagram export.
//! - `container`: dynamic (runtime) DI container (uses tokio).
//! - `async`: enables tokio / async-trait for downstream async adapters.
//! - `full`: all of the above.
//!
//! Revision History
//! - 2026-09-11T00:00:00Z @AI: Declare the private `clock` module — single guard for the `SystemTime::now()` panic on wasm32-unknown-unknown.
//! - 2026-07-21T00:00:00Z @AI: Rewrite crate header — correct feature list/default, drop the stale Phase-1/future-phases framing and nonexistent graph/analysis features; Quick Start now derives the real macros.
//! - 2025-10-09T14:14:00Z @AI: Remove Entity derive alias, expose HexEntity at crate root for qualified addressing.
//! - 2025-10-02T13:00:00Z @AI: Re-export inventory and error_codes for proc macros.
//! - 2025-10-02T12:00:00Z @AI: Add showcase module with Describable and Inspectable traits.
//! - 2025-10-01T00:01:00Z @AI: Added comprehensive re-exports and prelude module.
//! - 2025-10-01T00:00:00Z @AI: Initial Phase 1 implementation with core traits and types.

pub mod adapters;
pub mod application;
// Private: clockless-target-safe wall-clock reads, an implementation detail of the metadata
// timestamps rather than part of the public surface.
mod clock;
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
pub use crate::ports::{
  Direction, FindOptions, InputPort, OutputPort, Query, QueryRepository, Repository, Sort, UseCase,
};

// Re-export all adapter traits
pub use crate::adapters::{Adapter, Mapper};

// Re-export all application traits
pub use crate::application::{Application, Directive, DirectiveHandler, QueryHandler};

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

// Re-export derive macros at crate root for qualified addressing (e.g., hexser::HexEntity)
#[cfg(feature = "macros")]
pub use hexser_macros::{
  HexAdapter, HexAggregate, HexDirective, HexDomain, HexEntity, HexPort, HexQuery, HexRepository,
  HexValueItem,
};

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

  pub use crate::ports::{
    Direction, FindOptions, InputPort, OutputPort, Query, QueryRepository, Repository, Sort,
    UseCase,
  };

  pub use crate::adapters::{Adapter, Mapper};

  pub use crate::application::{Application, Directive, DirectiveHandler, QueryHandler};

  pub use crate::infrastructure::Config;

  pub use crate::graph::{
    GraphBuilder, GraphMetadata, HexEdge, HexGraph, HexNode, Layer, NodeId, Relationship, Role,
  };

  pub use crate::showcase::{ArcGraphExt, Describable, Inspectable, PrettyPrint};

  // Phase 3: Registry and derive macro support
  #[cfg(feature = "macros")]
  pub use hexser_macros::{
    HexAdapter, HexAggregate, HexDirective, HexDomain, HexEntity, HexPort, HexQuery, HexRepository,
    HexValueItem,
  };

  pub use crate::registry::{ComponentEntry, ComponentRegistry, NodeInfo, Registrable};

  #[cfg(feature = "ai")]
  pub use crate::ai::{AIContext, ContextBuilder};

  #[cfg(feature = "container")]
  pub use crate::container::{Container, Provider, Scope};

  #[cfg(feature = "static-di")]
  pub use crate::static_di::{StaticBuilder, StaticContainer};
}
