//! MCP server binary for serving hexser architecture data.
//!
//! Implements Model Context Protocol server over stdio transport. AI agents
//! can connect via JSON-RPC 2.0 to query architecture context, agent packs,
//! and other project metadata. Requires both `ai` and `mcp` features.
//!
//! Usage: Run this binary and communicate via stdin/stdout with line-delimited JSON-RPC.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP server binary implementation.

fn main() -> hexser::HexResult<()> {
  let server = hexser::adapters::mcp_stdio::McpStdioServer::new();
  server.run()
}
