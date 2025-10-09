//! MCP server capability declarations.
//!
//! Defines what capabilities this MCP server supports. During initialization,
//! the server declares its capabilities to the client, enabling feature negotiation.
//! Hexser MCP server supports resources (architecture queries).
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP capability types.

/// Server capabilities declaration.
///
/// Declares which MCP features this server supports. Sent during
/// initialization handshake to inform client of available operations.
/// Hexser supports resources for querying architecture data.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ServerCapabilities {
  /// Resource capability (reading architecture data)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub resources: Option<ResourceCapability>,

  /// Tool capability (not currently supported)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<ToolCapability>,

  /// Prompt capability (not currently supported)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prompts: Option<PromptCapability>,
}

impl ServerCapabilities {
  /// Creates default Hexser MCP server capabilities.
  ///
  /// Enables resources with no subscription or list change support.
  /// Tools and prompts are disabled for this initial implementation.
  ///
  /// # Returns
  ///
  /// ServerCapabilities with resources enabled
  pub fn hexser_default() -> Self {
    ServerCapabilities {
      resources: Some(ResourceCapability::default()),
      tools: None,
      prompts: None,
    }
  }
}

/// Resource capability configuration.
///
/// Indicates whether the server supports resource subscriptions
/// and list change notifications. Hexser starts with basic support
/// (no subscriptions or list changes).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct ResourceCapability {
  /// Whether clients can subscribe to resource changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscribe: Option<bool>,

  /// Whether server emits notifications when resource list changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub list_changed: Option<bool>,
}

/// Tool capability configuration (placeholder).
///
/// Tools allow servers to expose callable functions. Not currently
/// implemented in Hexser MCP server but reserved for future use.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct ToolCapability {
  /// Placeholder for tool features
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

/// Prompt capability configuration (placeholder).
///
/// Prompts expose reusable prompt templates. Not currently
/// implemented in Hexser MCP server but reserved for future use.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct PromptCapability {
  /// Placeholder for prompt features
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_capabilities_serialization() {
    let caps = ServerCapabilities::hexser_default();
    let json = serde_json::to_string(&caps).unwrap();

    std::assert!(json.contains("\"resources\""));
    std::assert!(!json.contains("\"tools\""));
    std::assert!(!json.contains("\"prompts\""));
  }

  #[test]
  fn test_resource_capability_default() {
    let res_cap = ResourceCapability::default();
    let json = serde_json::to_string(&res_cap).unwrap();

    // Empty object since all fields are optional and None by default
    std::assert_eq!(json, "{}");
  }
}
