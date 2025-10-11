//! Model Context Protocol (MCP) domain types.
//!
//! Defines core MCP protocol types following JSON-RPC 2.0 specification.
//! Enables AI agents to query project architecture via standardized protocol.
//! All types behind `mcp` feature flag.
//!
//! Revision History
//! - 2025-10-10T19:48:00Z @AI: Add refresh module with RefreshRequest and RefreshResult types.
//! - 2025-10-10T18:37:00Z @AI: Add project_config and project_registry for multi-project support.
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
pub mod project_config;

#[cfg(feature = "mcp")]
pub mod project_registry;

#[cfg(feature = "mcp")]
pub mod refresh;

#[cfg(feature = "mcp")]
pub use self::json_rpc::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

#[cfg(feature = "mcp")]
pub use self::capabilities::{ResourceCapability, ServerCapabilities};

#[cfg(feature = "mcp")]
pub use self::resource::{Resource, ResourceContent, ResourceList};

#[cfg(feature = "mcp")]
pub use self::initialize::{InitializeRequest, InitializeResult};

#[cfg(feature = "mcp")]
pub use self::project_config::ProjectConfig;

#[cfg(feature = "mcp")]
pub use self::project_registry::ProjectRegistry;

#[cfg(feature = "mcp")]
pub use self::refresh::{RefreshRequest, RefreshResult};
