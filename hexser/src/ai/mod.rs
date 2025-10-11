//! AI agent integration module for machine-readable architecture.
//!
//! Provides structured export of architecture context for AI consumption.
//! Enables AI agents to understand architectural constraints, suggest improvements,
//! and generate compliant code. All functionality behind `ai` feature flag.
//!
//! Revision History
//! - 2025-10-10T20:44:00Z @AI: Add method_extractor module for trait method extraction.
//! - 2025-10-06T18:14:00Z @AI: Add AgentPack module and re-exports for comprehensive AI interop.
//! - 2025-10-02T18:00:00Z @AI: Initial AI context export implementation.

#[cfg(feature = "ai")]
pub mod ai_context;

#[cfg(feature = "ai")]
pub mod context_builder;

#[cfg(feature = "ai")]
pub mod agent_pack;

#[cfg(feature = "ai")]
pub mod method_extractor;

#[cfg(feature = "ai")]
pub use self::ai_context::AIContext;

#[cfg(feature = "ai")]
pub use self::context_builder::ContextBuilder;

#[cfg(feature = "ai")]
pub use self::agent_pack::AgentPack;
