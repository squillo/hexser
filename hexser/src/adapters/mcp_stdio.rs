//! MCP server adapter using stdio transport.
//!
//! Implements the McpServer port using standard input/output for JSON-RPC
//! communication. Reads line-delimited JSON requests from stdin and writes
//! JSON-RPC responses to stdout. Integrates with ProjectRegistry for
//! multi-project architecture data serving.
//!
//! Revision History
//! - 2025-10-10T20:16:00Z @AI: Add Default impl and fix clippy warnings (needless borrows in cargo args).
//! - 2025-10-10T19:48:00Z @AI: Implement hexser/refresh method for triggering recompilation and clearing inventory cache.
//! - 2025-10-10T18:37:00Z @AI: Replace single graph with ProjectRegistry for multi-project support.
//! - 2025-10-08T23:35:00Z @AI: Initial MCP stdio adapter implementation.

/// MCP server implementation using stdio transport.
///
/// Processes MCP requests via JSON-RPC over standard input/output.
/// Uses ProjectRegistry to provide architecture context for multiple projects.
/// Each line on stdin should be a complete JSON-RPC request.
pub struct McpStdioServer {
  /// Registry managing multiple project configurations
  registry: crate::domain::mcp::ProjectRegistry,
}

impl McpStdioServer {
  /// Creates a new MCP stdio server with current HexGraph as single project.
  ///
  /// Provides backward compatibility by wrapping current graph in registry.
  ///
  /// # Returns
  ///
  /// New McpStdioServer instance with single "hexser" project
  pub fn new() -> Self {
    McpStdioServer {
      registry: crate::domain::mcp::ProjectRegistry::from_current_graph(),
    }
  }

  /// Creates a new MCP stdio server with a specific project registry.
  ///
  /// # Arguments
  ///
  /// * `registry` - ProjectRegistry containing projects to serve
  ///
  /// # Returns
  ///
  /// New McpStdioServer instance
  pub fn with_registry(registry: crate::domain::mcp::ProjectRegistry) -> Self {
    McpStdioServer { registry }
  }

  /// Creates a new MCP stdio server with a specific graph (backward compatibility).
  ///
  /// # Arguments
  ///
  /// * `graph` - Arc-wrapped HexGraph to use
  ///
  /// # Returns
  ///
  /// New McpStdioServer instance
  #[deprecated(since = "0.4.6", note = "Use with_registry instead")]
  pub fn with_graph(graph: std::sync::Arc<crate::graph::hex_graph::HexGraph>) -> Self {
    let mut registry = crate::domain::mcp::ProjectRegistry::new();
    let config = crate::domain::mcp::ProjectConfig::new(
      std::string::String::from("hexser"),
      std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
      graph,
    );
    registry.register(config);
    McpStdioServer { registry }
  }

  /// Parses project name from URI.
  ///
  /// Supports both legacy flat URIs (hexser://context) and new project-scoped
  /// URIs (hexser://project_name/context).
  ///
  /// # Arguments
  ///
  /// * `uri` - Resource URI to parse
  ///
  /// # Returns
  ///
  /// Tuple of (project_name, resource_type) or None if invalid
  fn parse_uri(uri: &str) -> std::option::Option<(std::string::String, std::string::String)> {
    if !uri.starts_with("hexser://") {
      return std::option::Option::None;
    }

    let path = &uri[9..]; // Remove "hexser://" prefix

    if path.is_empty() {
      return std::option::Option::None;
    }

    if path.contains('/') {
      // New format: hexser://project/resource
      let parts: std::vec::Vec<&str> = path.splitn(2, '/').collect();
      if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
        return std::option::Option::Some((
          std::string::String::from(parts[0]),
          std::string::String::from(parts[1]),
        ));
      }
    } else {
      // Legacy format: hexser://resource (assumes "hexser" project)
      return std::option::Option::Some((
        std::string::String::from("hexser"),
        std::string::String::from(path),
      ));
    }

    std::option::Option::None
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

impl std::default::Default for McpStdioServer {
  fn default() -> Self {
    Self::new()
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
    let mut resources = std::vec::Vec::new();

    for (project_name, _config) in self.registry.iter() {
      // Add context resource for this project
      resources.push(crate::domain::mcp::Resource {
        uri: std::format!("hexser://{}/context", project_name),
        name: std::format!("{} Architecture Context", project_name),
        description: std::option::Option::Some(std::format!(
          "Machine-readable architecture context for {} project",
          project_name
        )),
        mime_type: std::option::Option::Some(std::string::String::from("application/json")),
      });

      // Add pack resource for this project
      resources.push(crate::domain::mcp::Resource {
        uri: std::format!("hexser://{}/pack", project_name),
        name: std::format!("{} Agent Pack", project_name),
        description: std::option::Option::Some(std::format!(
          "Comprehensive agent pack (architecture + guidelines + docs) for {} project",
          project_name
        )),
        mime_type: std::option::Option::Some(std::string::String::from("application/json")),
      });
    }

    std::result::Result::Ok(crate::domain::mcp::ResourceList { resources })
  }

  fn read_resource(&self, uri: &str) -> crate::HexResult<crate::domain::mcp::ResourceContent> {
    let (project_name, resource_type) = Self::parse_uri(uri).ok_or_else(|| {
      crate::Hexserror::adapter("E_MCP_INVALID_URI", &format!("Invalid URI format: {}", uri))
    })?;

    let project = self.registry.get(&project_name).ok_or_else(|| {
      crate::Hexserror::adapter(
        "E_MCP_PROJECT_NOT_FOUND",
        &format!("Project not found: {}", project_name),
      )
    })?;

    match resource_type.as_str() {
      "context" => {
        let builder = crate::ai::ContextBuilder::new(std::sync::Arc::as_ref(&project.graph));
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
          std::string::String::from(uri),
          json,
          std::option::Option::Some(std::string::String::from("application/json")),
        ))
      }
      "pack" => {
        let pack =
          crate::ai::AgentPack::from_graph_with_defaults(std::sync::Arc::as_ref(&project.graph))?;
        let json = match pack.to_json() {
          std::result::Result::Ok(j) => j,
          std::result::Result::Err(e) => {
            return std::result::Result::Err(crate::Hexserror::adapter("E_MCP_PACK_SERIALIZE", &e));
          }
        };
        std::result::Result::Ok(crate::domain::mcp::ResourceContent::text(
          std::string::String::from(uri),
          json,
          std::option::Option::Some(std::string::String::from("application/json")),
        ))
      }
      _ => std::result::Result::Err(crate::Hexserror::adapter(
        "E_MCP_RESOURCE_NOT_FOUND",
        &format!("Unknown resource type: {}", resource_type),
      )),
    }
  }

  fn refresh_project(
    &mut self,
    request: crate::domain::mcp::RefreshRequest,
  ) -> crate::HexResult<crate::domain::mcp::RefreshResult> {
    let project = self.registry.get(&request.project).ok_or_else(|| {
      crate::Hexserror::adapter(
        "E_MCP_PROJECT_NOT_FOUND",
        &format!("Project not found: {}", request.project),
      )
    })?;

    let output = std::process::Command::new("cargo")
      .args(["build", "-p", &request.project, "--features", "macros"])
      .current_dir(&project.root_path)
      .output()
      .map_err(|e| {
        crate::Hexserror::adapter(
          "E_MCP_COMPILE",
          &format!("Failed to execute cargo build: {}", e),
        )
      })?;

    if !output.status.success() {
      let error_msg = std::string::String::from_utf8_lossy(&output.stderr).to_string();
      return std::result::Result::Ok(crate::domain::mcp::RefreshResult::compilation_error(
        error_msg,
      ));
    }

    std::result::Result::Ok(crate::domain::mcp::RefreshResult::restart_required())
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
      "hexser/refresh" => {
        let refresh_request: crate::domain::mcp::RefreshRequest = match request.params {
          Some(p) => match serde_json::from_value(p) {
            std::result::Result::Ok(r) => r,
            std::result::Result::Err(e) => {
              return crate::domain::mcp::JsonRpcResponse::error(
                id,
                crate::domain::mcp::JsonRpcError::invalid_request(format!(
                  "Invalid refresh params: {}",
                  e
                )),
              );
            }
          },
          None => {
            return crate::domain::mcp::JsonRpcResponse::error(
              id,
              crate::domain::mcp::JsonRpcError::invalid_request(String::from(
                "Missing refresh params",
              )),
            );
          }
        };

        let project = match self.registry.get(&refresh_request.project) {
          Some(p) => p,
          None => {
            let error_result = crate::domain::mcp::RefreshResult::compilation_error(format!(
              "Project not found: {}",
              refresh_request.project
            ));
            let result_value = match serde_json::to_value(error_result) {
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
            return crate::domain::mcp::JsonRpcResponse::success(id, result_value);
          }
        };

        let output = match std::process::Command::new("cargo")
          .args([
            "build",
            "-p",
            &refresh_request.project,
            "--features",
            "macros",
          ])
          .current_dir(&project.root_path)
          .output()
        {
          std::result::Result::Ok(o) => o,
          std::result::Result::Err(e) => {
            let error_result = crate::domain::mcp::RefreshResult::compilation_error(format!(
              "Failed to execute cargo build: {}",
              e
            ));
            let result_value = match serde_json::to_value(error_result) {
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
            return crate::domain::mcp::JsonRpcResponse::success(id, result_value);
          }
        };

        let result = if !output.status.success() {
          let error_msg = std::string::String::from_utf8_lossy(&output.stderr).to_string();
          crate::domain::mcp::RefreshResult::compilation_error(error_msg)
        } else {
          crate::domain::mcp::RefreshResult::restart_required()
        };

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
    // Test: Validates resource listing returns project-scoped URIs
    // Justification: Core functionality for resource discovery
    let server = McpStdioServer::new();
    let list = server.list_resources().unwrap();

    std::assert_eq!(list.resources.len(), 2);
    std::assert_eq!(list.resources[0].uri, "hexser://hexser/context");
    std::assert_eq!(list.resources[1].uri, "hexser://hexser/pack");
  }

  #[test]
  fn test_parse_uri_legacy_format() {
    // Test: Validates backward compatibility with legacy URI format
    // Justification: Ensures existing integrations continue to work
    let result = McpStdioServer::parse_uri("hexser://context");
    std::assert!(result.is_some());
    let (project, resource) = result.unwrap();
    std::assert_eq!(project, "hexser");
    std::assert_eq!(resource, "context");
  }

  #[test]
  fn test_parse_uri_project_scoped_format() {
    // Test: Validates new project-scoped URI format
    // Justification: Core functionality for multi-project support
    let result = McpStdioServer::parse_uri("hexser://myproject/pack");
    std::assert!(result.is_some());
    let (project, resource) = result.unwrap();
    std::assert_eq!(project, "myproject");
    std::assert_eq!(resource, "pack");
  }

  #[test]
  fn test_parse_uri_invalid() {
    // Test: Validates invalid URIs are rejected
    // Justification: Error handling verification
    std::assert!(McpStdioServer::parse_uri("invalid://uri").is_none());
    std::assert!(McpStdioServer::parse_uri("hexser://").is_none());
  }

  #[test]
  fn test_multi_project_registry() {
    // Test: Validates multiple projects can be registered and served
    // Justification: Core multi-project functionality
    let mut registry = crate::domain::mcp::ProjectRegistry::new();
    let graph1 = crate::graph::builder::GraphBuilder::new().build();
    let graph2 = crate::graph::builder::GraphBuilder::new().build();

    registry.register(crate::domain::mcp::ProjectConfig::new(
      std::string::String::from("project1"),
      std::path::PathBuf::from("/path1"),
      std::sync::Arc::new(graph1),
    ));
    registry.register(crate::domain::mcp::ProjectConfig::new(
      std::string::String::from("project2"),
      std::path::PathBuf::from("/path2"),
      std::sync::Arc::new(graph2),
    ));

    let server = McpStdioServer::with_registry(registry);
    let list = server.list_resources().unwrap();

    std::assert_eq!(list.resources.len(), 4);
    std::assert!(
      list
        .resources
        .iter()
        .any(|r| r.uri == "hexser://project1/context")
    );
    std::assert!(
      list
        .resources
        .iter()
        .any(|r| r.uri == "hexser://project1/pack")
    );
    std::assert!(
      list
        .resources
        .iter()
        .any(|r| r.uri == "hexser://project2/context")
    );
    std::assert!(
      list
        .resources
        .iter()
        .any(|r| r.uri == "hexser://project2/pack")
    );
  }

  #[test]
  fn test_read_resource_legacy_uri() {
    // Test: Validates backward compatibility for reading resources
    // Justification: Ensures existing code continues to work
    let server = McpStdioServer::new();
    let result = server.read_resource("hexser://context");
    std::assert!(result.is_ok());
  }

  #[test]
  fn test_read_resource_project_not_found() {
    // Test: Validates error when project doesn't exist
    // Justification: Error handling verification
    let server = McpStdioServer::new();
    let result = server.read_resource("hexser://nonexistent/context");
    std::assert!(result.is_err());
  }

  #[test]
  fn test_read_resource_invalid_resource_type() {
    // Test: Validates error for unknown resource types
    // Justification: Error handling verification
    let server = McpStdioServer::new();
    let result = server.read_resource("hexser://hexser/unknown");
    std::assert!(result.is_err());
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

  #[test]
  fn test_handle_refresh_project_not_found() {
    // Test: Validates refresh returns error for nonexistent project
    // Justification: Error handling verification for refresh endpoint
    let server = McpStdioServer::new();
    let request = crate::domain::mcp::JsonRpcRequest::new(
      serde_json::Value::Number(serde_json::Number::from(1)),
      String::from("hexser/refresh"),
      Some(serde_json::json!({"project": "nonexistent"})),
    );

    let response = server.handle_request(request);
    std::assert!(response.result.is_some());
    std::assert!(response.error.is_none());

    let result: crate::domain::mcp::RefreshResult =
      serde_json::from_value(response.result.unwrap()).unwrap();
    std::assert_eq!(result.status, "error");
    std::assert!(!result.compiled);
  }

  #[test]
  fn test_handle_refresh_missing_params() {
    // Test: Validates refresh returns error when params missing
    // Justification: Validates parameter validation
    let server = McpStdioServer::new();
    let request = crate::domain::mcp::JsonRpcRequest::new(
      serde_json::Value::Number(serde_json::Number::from(1)),
      String::from("hexser/refresh"),
      None,
    );

    let response = server.handle_request(request);
    std::assert!(response.result.is_none());
    std::assert!(response.error.is_some());
    std::assert_eq!(response.error.unwrap().code, -32600);
  }
}
