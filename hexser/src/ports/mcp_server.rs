//! MCP server port trait.
//!
//! Defines the abstract interface for MCP server operations. Adapters
//! implement this trait to handle JSON-RPC requests over different transports
//! (stdio, HTTP, etc.). The port abstracts MCP protocol details from transport.
//!
//! Revision History
//! - 2025-10-10T19:48:00Z @AI: Add refresh_project method for triggering recompilation and cache clearing.
//! - 2025-10-08T23:35:00Z @AI: Initial MCP server port trait.

/// MCP server operations port.
///
/// Trait defining the core MCP server operations that must be implemented
/// by any adapter. Handles initialization handshake and resource operations.
/// Implementation should integrate with HexGraph to provide architecture data.
pub trait McpServer {
  /// Handles the initialize method.
  ///
  /// First method called by client. Performs capability negotiation
  /// and returns server information.
  ///
  /// # Arguments
  ///
  /// * `request` - Client's initialization request
  ///
  /// # Returns
  ///
  /// Result containing initialization response or error
  fn initialize(
    &self,
    request: crate::domain::mcp::InitializeRequest,
  ) -> crate::HexResult<crate::domain::mcp::InitializeResult>;

  /// Lists available resources.
  ///
  /// Returns all resources that clients can query. For Hexser, this
  /// includes architecture context and agent pack resources.
  ///
  /// # Returns
  ///
  /// Result containing resource list or error
  fn list_resources(&self) -> crate::HexResult<crate::domain::mcp::ResourceList>;

  /// Reads a specific resource by URI.
  ///
  /// Retrieves the content of the requested resource. Hexser supports
  /// hexser://context (AIContext JSON) and hexser://pack (AgentPack JSON).
  ///
  /// # Arguments
  ///
  /// * `uri` - Resource URI to read
  ///
  /// # Returns
  ///
  /// Result containing resource content or error
  fn read_resource(&self, uri: &str) -> crate::HexResult<crate::domain::mcp::ResourceContent>;

  /// Refreshes a project's architecture graph.
  ///
  /// Triggers recompilation of the project and restarts the MCP server
  /// to clear the old inventory static cache and load the updated graph.
  /// This allows AI agents to see newly added components after code changes.
  ///
  /// # Arguments
  ///
  /// * `request` - Refresh request with project name
  ///
  /// # Returns
  ///
  /// Result containing refresh status or error
  fn refresh_project(
    &mut self,
    request: crate::domain::mcp::RefreshRequest,
  ) -> crate::HexResult<crate::domain::mcp::RefreshResult>;

  /// Processes a JSON-RPC request and returns a JSON-RPC response.
  ///
  /// Main entry point for handling MCP requests. Routes to appropriate
  /// handler based on method name.
  ///
  /// # Arguments
  ///
  /// * `request` - JSON-RPC request
  ///
  /// # Returns
  ///
  /// JSON-RPC response (success or error)
  fn handle_request(
    &self,
    request: crate::domain::mcp::JsonRpcRequest,
  ) -> crate::domain::mcp::JsonRpcResponse;
}
