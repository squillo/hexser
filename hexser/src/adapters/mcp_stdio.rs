//! MCP server adapter using stdio transport.
//!
//! Implements the McpServer port using standard input/output for JSON-RPC
//! communication. Reads line-delimited JSON requests from stdin and writes
//! JSON-RPC responses to stdout. Integrates with HexGraph and existing AI
//! infrastructure to serve architecture data.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP stdio adapter implementation.

/// MCP server implementation using stdio transport.
///
/// Processes MCP requests via JSON-RPC over standard input/output.
/// Uses HexGraph to provide architecture context and agent pack resources.
/// Each line on stdin should be a complete JSON-RPC request.
pub struct McpStdioServer {
  /// Reference to the HexGraph for architecture data
  graph: std::sync::Arc<crate::graph::hex_graph::HexGraph>,
}

impl McpStdioServer {
  /// Creates a new MCP stdio server with the current HexGraph.
  ///
  /// # Returns
  ///
  /// New McpStdioServer instance
  pub fn new() -> Self {
    McpStdioServer {
      graph: crate::HexGraph::current(),
    }
  }

  /// Creates a new MCP stdio server with a specific graph.
  ///
  /// # Arguments
  ///
  /// * `graph` - Arc-wrapped HexGraph to use
  ///
  /// # Returns
  ///
  /// New McpStdioServer instance
  pub fn with_graph(graph: std::sync::Arc<crate::graph::hex_graph::HexGraph>) -> Self {
    McpStdioServer { graph }
  }

  /// Runs the MCP server loop, reading from stdin and writing to stdout.
  ///
  /// Reads line-delimited JSON-RPC requests from stdin, processes each
  /// request, and writes responses to stdout. Exits on EOF or IO error.
  ///
  /// # Returns
  ///
  /// Result indicating success or IO error
  pub fn run(&self) -> crate::HexResult<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    for line_result in stdin.lines() {
      let line = match line_result {
        std::result::Result::Ok(l) => l,
        std::result::Result::Err(e) => {
          return std::result::Result::Err(crate::Hexserror::adapter(
            "E_MCP_STDIN",
            &format!("Failed to read from stdin: {}", e),
          ));
        }
      };

      if line.trim().is_empty() {
        continue;
      }

      let request: crate::domain::mcp::JsonRpcRequest = match serde_json::from_str(&line) {
        std::result::Result::Ok(req) => req,
        std::result::Result::Err(e) => {
          let error_response = crate::domain::mcp::JsonRpcResponse::error(
            serde_json::Value::Null,
            crate::domain::mcp::JsonRpcError::parse_error(format!("Invalid JSON: {}", e)),
          );
          self.write_response(&mut stdout, &error_response)?;
          continue;
        }
      };

      let response = <Self as crate::ports::mcp_server::McpServer>::handle_request(self, request);
      self.write_response(&mut stdout, &response)?;
    }

    std::result::Result::Ok(())
  }

  fn write_response(
    &self,
    stdout: &mut std::io::Stdout,
    response: &crate::domain::mcp::JsonRpcResponse,
  ) -> crate::HexResult<()> {
    let json = match serde_json::to_string(response) {
      std::result::Result::Ok(j) => j,
      std::result::Result::Err(e) => {
        return std::result::Result::Err(crate::Hexserror::adapter(
          "E_MCP_SERIALIZE",
          &format!("Failed to serialize response: {}", e),
        ));
      }
    };

    use std::io::Write;
    if let std::result::Result::Err(e) = writeln!(stdout, "{}", json) {
      return std::result::Result::Err(crate::Hexserror::adapter(
        "E_MCP_STDOUT",
        &format!("Failed to write to stdout: {}", e),
      ));
    }

    if let std::result::Result::Err(e) = stdout.flush() {
      return std::result::Result::Err(crate::Hexserror::adapter(
        "E_MCP_FLUSH",
        &format!("Failed to flush stdout: {}", e),
      ));
    }

    std::result::Result::Ok(())
  }
}

impl crate::ports::mcp_server::McpServer for McpStdioServer {
  fn initialize(
    &self,
    _request: crate::domain::mcp::InitializeRequest,
  ) -> crate::HexResult<crate::domain::mcp::InitializeResult> {
    std::result::Result::Ok(crate::domain::mcp::InitializeResult::hexser_default())
  }

  fn list_resources(&self) -> crate::HexResult<crate::domain::mcp::ResourceList> {
    std::result::Result::Ok(crate::domain::mcp::ResourceList::hexser_default())
  }

  fn read_resource(&self, uri: &str) -> crate::HexResult<crate::domain::mcp::ResourceContent> {
    match uri {
      "hexser://context" => {
        let builder = crate::ai::ContextBuilder::new(std::sync::Arc::as_ref(&self.graph));
        let context = builder.build()?;
        let json = match context.to_json() {
          std::result::Result::Ok(j) => j,
          std::result::Result::Err(e) => {
            return std::result::Result::Err(crate::Hexserror::adapter(
              "E_MCP_CONTEXT_SERIALIZE",
              &e,
            ));
          }
        };
        std::result::Result::Ok(crate::domain::mcp::ResourceContent::text(
          String::from(uri),
          json,
          Some(String::from("application/json")),
        ))
      }
      "hexser://pack" => {
        let pack =
          crate::ai::AgentPack::from_graph_with_defaults(std::sync::Arc::as_ref(&self.graph))?;
        let json = match pack.to_json() {
          std::result::Result::Ok(j) => j,
          std::result::Result::Err(e) => {
            return std::result::Result::Err(crate::Hexserror::adapter("E_MCP_PACK_SERIALIZE", &e));
          }
        };
        std::result::Result::Ok(crate::domain::mcp::ResourceContent::text(
          String::from(uri),
          json,
          Some(String::from("application/json")),
        ))
      }
      _ => std::result::Result::Err(crate::Hexserror::adapter(
        "E_MCP_RESOURCE_NOT_FOUND",
        &format!("Unknown resource URI: {}", uri),
      )),
    }
  }

  fn handle_request(
    &self,
    request: crate::domain::mcp::JsonRpcRequest,
  ) -> crate::domain::mcp::JsonRpcResponse {
    let id = request.id.clone();

    match request.method.as_str() {
      "initialize" => {
        let init_request: crate::domain::mcp::InitializeRequest = match request.params {
          Some(p) => match serde_json::from_value(p) {
            std::result::Result::Ok(r) => r,
            std::result::Result::Err(e) => {
              return crate::domain::mcp::JsonRpcResponse::error(
                id,
                crate::domain::mcp::JsonRpcError::invalid_request(format!(
                  "Invalid initialize params: {}",
                  e
                )),
              );
            }
          },
          None => {
            return crate::domain::mcp::JsonRpcResponse::error(
              id,
              crate::domain::mcp::JsonRpcError::invalid_request(String::from(
                "Missing initialize params",
              )),
            );
          }
        };

        match self.initialize(init_request) {
          std::result::Result::Ok(result) => {
            let result_value = match serde_json::to_value(result) {
              std::result::Result::Ok(v) => v,
              std::result::Result::Err(e) => {
                return crate::domain::mcp::JsonRpcResponse::error(
                  id,
                  crate::domain::mcp::JsonRpcError::internal_error(format!(
                    "Serialization error: {}",
                    e
                  )),
                );
              }
            };
            crate::domain::mcp::JsonRpcResponse::success(id, result_value)
          }
          std::result::Result::Err(e) => crate::domain::mcp::JsonRpcResponse::error(
            id,
            crate::domain::mcp::JsonRpcError::internal_error(format!("{}", e)),
          ),
        }
      }
      "resources/list" => match self.list_resources() {
        std::result::Result::Ok(list) => {
          let list_value = match serde_json::to_value(list) {
            std::result::Result::Ok(v) => v,
            std::result::Result::Err(e) => {
              return crate::domain::mcp::JsonRpcResponse::error(
                id,
                crate::domain::mcp::JsonRpcError::internal_error(format!(
                  "Serialization error: {}",
                  e
                )),
              );
            }
          };
          crate::domain::mcp::JsonRpcResponse::success(id, list_value)
        }
        std::result::Result::Err(e) => crate::domain::mcp::JsonRpcResponse::error(
          id,
          crate::domain::mcp::JsonRpcError::internal_error(format!("{}", e)),
        ),
      },
      "resources/read" => {
        let uri: String = match request.params {
          Some(p) => match p.get("uri") {
            Some(u) => match u.as_str() {
              Some(s) => String::from(s),
              None => {
                return crate::domain::mcp::JsonRpcResponse::error(
                  id,
                  crate::domain::mcp::JsonRpcError::invalid_request(String::from(
                    "URI must be a string",
                  )),
                );
              }
            },
            None => {
              return crate::domain::mcp::JsonRpcResponse::error(
                id,
                crate::domain::mcp::JsonRpcError::invalid_request(String::from(
                  "Missing uri parameter",
                )),
              );
            }
          },
          None => {
            return crate::domain::mcp::JsonRpcResponse::error(
              id,
              crate::domain::mcp::JsonRpcError::invalid_request(String::from("Missing params")),
            );
          }
        };

        match self.read_resource(&uri) {
          std::result::Result::Ok(content) => {
            let content_value = match serde_json::to_value(content) {
              std::result::Result::Ok(v) => v,
              std::result::Result::Err(e) => {
                return crate::domain::mcp::JsonRpcResponse::error(
                  id,
                  crate::domain::mcp::JsonRpcError::internal_error(format!(
                    "Serialization error: {}",
                    e
                  )),
                );
              }
            };
            crate::domain::mcp::JsonRpcResponse::success(id, content_value)
          }
          std::result::Result::Err(e) => crate::domain::mcp::JsonRpcResponse::error(
            id,
            crate::domain::mcp::JsonRpcError::internal_error(format!("{}", e)),
          ),
        }
      }
      _ => crate::domain::mcp::JsonRpcResponse::error(
        id,
        crate::domain::mcp::JsonRpcError::method_not_found(request.method),
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ports::mcp_server::McpServer;

  #[test]
  fn test_mcp_server_initialization() {
    let server = McpStdioServer::new();
    let request = crate::domain::mcp::InitializeRequest {
      protocol_version: String::from("2024-11-05"),
      capabilities: None,
      client_info: None,
    };

    let result = server.initialize(request);
    std::assert!(result.is_ok());
  }

  #[test]
  fn test_list_resources() {
    let server = McpStdioServer::new();
    let list = server.list_resources().unwrap();

    std::assert_eq!(list.resources.len(), 2);
    std::assert_eq!(list.resources[0].uri, "hexser://context");
    std::assert_eq!(list.resources[1].uri, "hexser://pack");
  }

  #[test]
  fn test_handle_initialize_request() {
    let server = McpStdioServer::new();
    let request = crate::domain::mcp::JsonRpcRequest::new(
      serde_json::Value::Number(serde_json::Number::from(1)),
      String::from("initialize"),
      Some(serde_json::json!({"protocolVersion": "2024-11-05"})),
    );

    let response = server.handle_request(request);
    std::assert!(response.result.is_some());
    std::assert!(response.error.is_none());
  }

  #[test]
  fn test_handle_unknown_method() {
    let server = McpStdioServer::new();
    let request = crate::domain::mcp::JsonRpcRequest::new(
      serde_json::Value::Number(serde_json::Number::from(1)),
      String::from("unknown/method"),
      None,
    );

    let response = server.handle_request(request);
    std::assert!(response.result.is_none());
    std::assert!(response.error.is_some());
    std::assert_eq!(response.error.unwrap().code, -32601);
  }
}
