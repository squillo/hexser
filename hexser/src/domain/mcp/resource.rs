//! MCP resource types for exposing architecture data.
//!
//! Resources represent queryable data sources in MCP. The Hexser MCP server
//! exposes architecture context as resources with URIs like hexser://context
//! and hexser://pack. Clients can list available resources and read their content.
//!
//! Revision History
//! - 2025-10-08T23:35:00Z @AI: Initial MCP resource types.

/// MCP resource descriptor.
///
/// Describes a single queryable resource with URI, name, description,
/// and MIME type. Resources are discovered via resources/list and
/// read via resources/read methods.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Resource {
  /// Unique resource URI (e.g., hexser://context)
  pub uri: String,

  /// Human-readable resource name
  pub name: String,

  /// Detailed description of the resource content
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,

  /// MIME type of the resource content
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mime_type: Option<String>,
}

impl Resource {
  /// Creates a new resource descriptor.
  ///
  /// # Arguments
  ///
  /// * `uri` - Unique resource URI
  /// * `name` - Human-readable name
  /// * `description` - Optional description
  /// * `mime_type` - Optional MIME type
  ///
  /// # Returns
  ///
  /// A new Resource instance
  pub fn new(
    uri: String,
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
  ) -> Self {
    Resource {
      uri,
      name,
      description,
      mime_type,
    }
  }

  /// Creates the AI context resource descriptor.
  ///
  /// Describes the hexser://context resource containing AIContext JSON.
  ///
  /// # Returns
  ///
  /// Resource descriptor for architecture context
  pub fn ai_context() -> Self {
    Resource::new(
      String::from("hexser://context"),
      String::from("Architecture Context"),
      Some(String::from(
        "Machine-readable architecture with components, relationships, and constraints",
      )),
      Some(String::from("application/json")),
    )
  }

  /// Creates the agent pack resource descriptor.
  ///
  /// Describes the hexser://pack resource containing complete AgentPack JSON.
  ///
  /// # Returns
  ///
  /// Resource descriptor for agent pack
  pub fn agent_pack() -> Self {
    Resource::new(
      String::from("hexser://pack"),
      String::from("Agent Pack"),
      Some(String::from(
        "Comprehensive package with architecture, guidelines, and documentation",
      )),
      Some(String::from("application/json")),
    )
  }
}

/// List of resources returned by resources/list.
///
/// Contains all resources available on the server. Clients use this
/// to discover what architecture data can be queried.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ResourceList {
  /// Available resources
  pub resources: Vec<Resource>,
}

impl ResourceList {
  /// Creates a new resource list.
  ///
  /// # Arguments
  ///
  /// * `resources` - Vector of available resources
  ///
  /// # Returns
  ///
  /// A new ResourceList instance
  pub fn new(resources: Vec<Resource>) -> Self {
    ResourceList { resources }
  }

  /// Creates default Hexser resource list.
  ///
  /// Includes AI context and agent pack resources.
  ///
  /// # Returns
  ///
  /// ResourceList with default Hexser resources
  pub fn hexser_default() -> Self {
    ResourceList::new(vec![Resource::ai_context(), Resource::agent_pack()])
  }
}

/// Resource content returned by resources/read.
///
/// Contains the actual content of a resource along with metadata.
/// Content can be text (JSON, markdown) or binary data.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ResourceContent {
  /// Resource URI that was read
  pub uri: String,

  /// Text content (for JSON, markdown, etc.)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,

  /// Binary content (not currently used)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blob: Option<String>,

  /// MIME type of the content
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mime_type: Option<String>,
}

impl ResourceContent {
  /// Creates resource content with text.
  ///
  /// # Arguments
  ///
  /// * `uri` - Resource URI
  /// * `text` - Text content
  /// * `mime_type` - Optional MIME type
  ///
  /// # Returns
  ///
  /// ResourceContent with text set
  pub fn text(uri: String, text: String, mime_type: Option<String>) -> Self {
    ResourceContent {
      uri,
      text: Some(text),
      blob: None,
      mime_type,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resource_serialization() {
    let resource = Resource::ai_context();
    let json = serde_json::to_string(&resource).unwrap();

    std::assert!(json.contains("\"uri\":\"hexser://context\""));
    std::assert!(json.contains("\"name\":\"Architecture Context\""));
  }

  #[test]
  fn test_resource_list_default() {
    let list = ResourceList::hexser_default();

    std::assert_eq!(list.resources.len(), 2);
    std::assert_eq!(list.resources[0].uri, "hexser://context");
    std::assert_eq!(list.resources[1].uri, "hexser://pack");
  }

  #[test]
  fn test_resource_content_text() {
    let content = ResourceContent::text(
      String::from("hexser://context"),
      String::from("{\"test\": true}"),
      Some(String::from("application/json")),
    );

    std::assert_eq!(content.uri, "hexser://context");
    std::assert!(content.text.is_some());
    std::assert!(content.blob.is_none());
  }
}
