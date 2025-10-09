//! Model Context Protocol (MCP) domain types.
//!
//! Defines core MCP protocol types following JSON-RPC 2.0 specification.
//! Enables AI agents to query project architecture via standardized protocol.
//! All types behind `mcp` feature flag.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP domain module structure.

#[cfg(feature = "mcp")]
pub mod json_rpc;

#[cfg(feature = "mcp")]
pub mod capabilities;

#[cfg(feature = "mcp")]
pub mod resource;

#[cfg(feature = "mcp")]
pub mod initialize;

#[cfg(feature = "mcp")]
pub use self::json_rpc::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

#[cfg(feature = "mcp")]
pub use self::capabilities::{ResourceCapability, ServerCapabilities};

#[cfg(feature = "mcp")]
pub use self::resource::{Resource, ResourceContent, ResourceList};

#[cfg(feature = "mcp")]
pub use self::initialize::{InitializeRequest, InitializeResult};
