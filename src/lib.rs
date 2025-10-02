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
//! - **Domain**: Core business logic (`Entity`, `ValueObject`, `Aggregate`)
//! - **Ports**: Interface definitions (`Repository`, `UseCase`, `Query`)
//! - **Adapters**: Port implementations (`Adapter`, `Mapper`)
//! - **Application**: Use case orchestration (`Directive`, `DirectiveHandler`)
//! - **Infrastructure**: External concerns (`Config`)
//!
//! # Quick Start
//!
//! ```rust
//! use hex::prelude::*;
//!
//! // Define a domain entity
//! struct User {
//!     id: String,
//!     email: String,
//! }
//!
//! impl Entity for User {
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

pub mod domain;
pub mod ports;
pub mod adapters;
pub mod application;
pub mod infrastructure;
pub mod error;
pub mod result;
pub mod showcase;
pub mod graph;
pub mod registry;

// Re-export commonly used items at crate root for convenience
pub use crate::error::hex_error::HexError;
pub use crate::result::hex_result::HexResult;

// Re-export all domain traits
pub use crate::domain::{
    Aggregate,
    DomainEvent,
    DomainService,
    Entity,
    ValueObject,
};

// Re-export all port traits
pub use crate::ports::{
    InputPort,
    OutputPort,
    Query,
    Repository,
    UseCase,
};

// Re-export all adapter traits
pub use crate::adapters::{
    Adapter,
    Mapper,
};

// Re-export all application traits
pub use crate::application::{
    Directive,
    DirectiveHandler,
    QueryHandler,
};

// Re-export infrastructure traits
pub use crate::infrastructure::Config;

// Re-export inventory for proc macros
pub use inventory;

// Re-export error codes module
pub use crate::error::codes as error_codes;

// Re-export graph types (Phase 2)
pub use crate::graph::{
    GraphBuilder,
    GraphMetadata,
    HexEdge,
    HexGraph,
    HexNode,
    Layer,
    NodeId,
    Relationship,
    Role,
};

// Re-export showcase traits
pub use crate::showcase::{
    ArcGraphExt,
    Describable,
    Inspectable,
    PrettyPrint,
};

/// Prelude module for convenient imports.
///
/// Import everything you need with a single use statement:
///
/// ```rust
/// use hex::prelude::*;
/// ```
pub mod prelude {
    pub use crate::HexError;
    pub use crate::HexResult;

    pub use crate::domain::{
        Aggregate,
        DomainEvent,
        DomainService,
        Entity,
        ValueObject,
    };

    pub use crate::ports::{
        InputPort,
        OutputPort,
        Query,
        Repository,
        UseCase,
    };

    pub use crate::adapters::{
        Adapter,
        Mapper,
    };

    pub use crate::application::{
        Directive,
        DirectiveHandler,
        QueryHandler,
    };

    pub use crate::infrastructure::Config;

    pub use crate::graph::{
        GraphBuilder,
        GraphMetadata,
        HexEdge,
        HexGraph,
        HexNode,
        Layer,
        NodeId,
        Relationship,
        Role,
    };

    pub use crate::showcase::{
        ArcGraphExt,
        Describable,
        Inspectable,
        PrettyPrint,
    };

    // Phase 3: Registry and derive macro support
    #[cfg(feature = "macros")]
    pub use hex_macros::{
        HexDomain,
        HexPort,
        HexAdapter,
        Entity,
        HexRepository,
        HexDirective,
        HexQuery,
    };

    pub use crate::registry::{
        ComponentRegistry,
        Registrable,
        ComponentEntry,
        NodeInfo,
    };
}
