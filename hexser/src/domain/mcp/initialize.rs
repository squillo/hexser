//! MCP initialization protocol types.
//!
//! Handles the initialization handshake between MCP client and server.
//! The initialize method is the first call made by clients, establishing
//! protocol version and exchanging capability information.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP initialization types.

/// Initialize request from client to server.
///
/// First message in MCP handshake. Client declares its protocol version
/// and capabilities. Server responds with its own capabilities and
/// server information.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct InitializeRequest {
    /// MCP protocol version the client supports
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    /// Client capabilities (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<ClientCapabilities>,

    /// Client information (optional)
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
}

/// Client capability declarations.
///
/// Informs server of what the client supports. Currently a placeholder
/// as Hexser server doesn't require specific client capabilities.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct ClientCapabilities {
    /// Experimental features (placeholder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}

/// Client information for identification.
///
/// Allows client to identify itself to server. Useful for logging
/// and debugging MCP interactions.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ClientInfo {
    /// Client name
    pub name: String,

    /// Client version
    pub version: String,
}

/// Initialize result from server to client.
///
/// Server response to initialization request. Declares supported
/// protocol version, server capabilities, and server information.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct InitializeResult {
    /// MCP protocol version the server supports
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    /// Server capabilities declaration
    pub capabilities: super::capabilities::ServerCapabilities,

    /// Server information for identification
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

impl InitializeResult {
    /// Creates Hexser MCP server initialization result.
    ///
    /// Uses default Hexser capabilities and current version information.
    ///
    /// # Returns
    ///
    /// InitializeResult for Hexser MCP server
    pub fn hexser_default() -> Self {
        InitializeResult {
            protocol_version: String::from("2024-11-05"),
            capabilities: super::capabilities::ServerCapabilities::hexser_default(),
            server_info: ServerInfo::hexser_default(),
        }
    }
}

/// Server information for identification.
///
/// Provides server name and version to client. Helps clients
/// understand what server implementation they are connected to.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ServerInfo {
    /// Server name
    pub name: String,

    /// Server version
    pub version: String,
}

impl ServerInfo {
    /// Creates Hexser server information.
    ///
    /// # Returns
    ///
    /// ServerInfo for Hexser MCP server
    pub fn hexser_default() -> Self {
        ServerInfo {
            name: String::from("hexser-mcp-server"),
            version: String::from(env!("CARGO_PKG_VERSION")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_request_deserialization() {
        let json = r#"{
            "protocolVersion": "2024-11-05",
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        }"#;

        let req: InitializeRequest = serde_json::from_str(json).unwrap();
        std::assert_eq!(req.protocol_version, "2024-11-05");
        std::assert!(req.client_info.is_some());
    }

    #[test]
    fn test_initialize_result_serialization() {
        let result = InitializeResult::hexser_default();
        let json = serde_json::to_string(&result).unwrap();

        std::assert!(json.contains("\"protocolVersion\":\"2024-11-05\""));
        std::assert!(json.contains("\"serverInfo\""));
        std::assert!(json.contains("\"capabilities\""));
    }

    #[test]
    fn test_server_info_default() {
        let info = ServerInfo::hexser_default();

        std::assert_eq!(info.name, "hexser-mcp-server");
        std::assert!(!info.version.is_empty());
    }
}
