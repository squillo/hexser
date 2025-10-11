//! AI context structure for machine-readable architecture representation.
//!
//! Defines structured format for exporting architecture metadata to AI agents.
//! Includes components, relationships, constraints, and suggestions.
//! Follows JSON Schema for validation and tooling integration.
//!
//! Revision History
//! - 2025-10-10T20:28:00Z @AI: Add MethodInfo to ComponentInfo for capturing method signatures and documentation.
//! - 2025-10-02T18:00:00Z @AI: Initial AI context structure.
//! - 2025-10-06T17:59:00Z @AI: Add to_json() serializer and tests; ensure ai feature includes serde.

/// Machine-readable architecture context for AI agents
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AIContext {
  /// Architecture pattern used
  pub architecture: String,

  /// Crate version
  pub version: String,

  /// All components in the architecture
  pub components: Vec<ComponentInfo>,

  /// Relationships between components
  pub relationships: Vec<RelationshipInfo>,

  /// Architectural constraints and rules
  pub constraints: ConstraintSet,

  /// AI suggestions for improvements
  pub suggestions: Vec<Suggestion>,

  /// Metadata about the export
  pub metadata: ContextMetadata,
}

/// Information about a single component
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ComponentInfo {
  /// Fully qualified type name
  pub type_name: String,

  /// Architectural layer
  pub layer: String,

  /// Component role
  pub role: String,

  /// Module path
  pub module_path: String,

  /// Brief description of purpose
  pub purpose: Option<String>,

  /// Dependencies on other components
  pub dependencies: Vec<String>,

  /// Public methods and their documentation
  pub methods: Vec<MethodInfo>,
}

/// Information about a method within a component
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MethodInfo {
  /// Method name
  pub name: String,

  /// Method signature (full declaration)
  pub signature: String,

  /// Documentation comment for the method
  pub documentation: Option<String>,

  /// Method parameters with types and descriptions
  pub parameters: Vec<ParameterInfo>,

  /// Return type information
  pub return_type: Option<String>,

  /// Whether this method is public
  pub is_public: bool,

  /// Whether this method is async
  pub is_async: bool,
}

/// Information about a method parameter
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ParameterInfo {
  /// Parameter name
  pub name: String,

  /// Parameter type
  pub param_type: String,

  /// Parameter description from documentation
  pub description: Option<String>,
}

/// Information about component relationships
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RelationshipInfo {
  /// Source component
  pub from: String,

  /// Target component
  pub to: String,

  /// Relationship type
  pub relationship_type: String,

  /// Whether this relationship is valid per architecture rules
  pub is_valid: bool,

  /// Explanation if invalid
  pub validation_message: Option<String>,
}

/// Set of architectural constraints
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ConstraintSet {
  /// Dependency rules between layers
  pub dependency_rules: Vec<DependencyRule>,

  /// Layer boundary rules
  pub layer_boundaries: Vec<LayerBoundary>,

  /// Naming conventions
  pub naming_conventions: Vec<NamingConvention>,

  /// Required patterns
  pub required_patterns: Vec<String>,
}

/// Rule about layer dependencies
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DependencyRule {
  /// Source layer
  pub from_layer: String,

  /// Target layer
  pub to_layer: String,

  /// Whether dependency is allowed
  pub allowed: bool,

  /// Explanation
  pub reason: String,
}

/// Layer boundary definition
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LayerBoundary {
  /// Layer name
  pub layer: String,

  /// What this layer can depend on
  pub can_depend_on: Vec<String>,

  /// What can depend on this layer
  pub dependents_allowed: Vec<String>,

  /// Purpose of this layer
  pub purpose: String,
}

/// Naming convention rule
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NamingConvention {
  /// What the convention applies to
  pub applies_to: String,

  /// Pattern or rule
  pub pattern: String,

  /// Example
  pub example: String,
}

/// AI suggestion for improvement
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Suggestion {
  /// Suggestion type
  pub suggestion_type: SuggestionType,

  /// Component this applies to
  pub component: Option<String>,

  /// Description
  pub description: String,

  /// Priority
  pub priority: Priority,

  /// Code example if applicable
  pub code_example: Option<String>,
}

/// Type of suggestion
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
  /// Missing implementation
  MissingImplementation,

  /// Architectural violation
  ArchitecturalViolation,

  /// Improvement opportunity
  Improvement,

  /// Best practice recommendation
  BestPractice,

  /// Potential issue
  PotentialIssue,
}

/// Suggestion priority
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
  Low,
  Medium,
  High,
  Critical,
}

/// Metadata about the context export
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ContextMetadata {
  /// When context was generated
  pub generated_at: String,

  /// hex version used
  pub hex_version: String,

  /// Total component count
  pub total_components: usize,

  /// Total relationship count
  pub total_relationships: usize,

  /// Schema version
  pub schema_version: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ai_context_serialization() {
    let context = AIContext {
      architecture: String::from("hexagonal"),
      version: String::from("0.3.0"),
      components: vec![],
      relationships: vec![],
      constraints: ConstraintSet {
        dependency_rules: vec![],
        layer_boundaries: vec![],
        naming_conventions: vec![],
        required_patterns: vec![],
      },
      suggestions: vec![],
      metadata: ContextMetadata {
        generated_at: String::from("2025-10-02T18:00:00Z"),
        hex_version: String::from("0.3.0"),
        total_components: 0,
        total_relationships: 0,
        schema_version: String::from("1.0.0"),
      },
    };

    let json = serde_json::to_string(&context).unwrap();
    assert!(json.contains("hexagonal"));
    assert!(json.contains("schema_version"));
  }

  #[test]
  fn test_component_info_serialization() {
    let component = ComponentInfo {
      type_name: String::from("User"),
      layer: String::from("Domain"),
      role: String::from("Entity"),
      module_path: String::from("domain::user"),
      purpose: Some(String::from("Represents a user")),
      methods: vec![],
      dependencies: vec![],
    };

    let json = serde_json::to_string(&component).unwrap();
    assert!(json.contains("User"));
    assert!(json.contains("Domain"));
  }

  #[test]
  fn test_suggestion_serialization() {
    let suggestion = Suggestion {
      suggestion_type: SuggestionType::MissingImplementation,
      component: Some(String::from("UserRepository")),
      description: String::from("Port missing adapter implementation"),
      priority: Priority::High,
      code_example: Some(String::from("impl UserRepository for PostgresUserRepo")),
    };

    let json = serde_json::to_string(&suggestion).unwrap();
    assert!(json.contains("missing_implementation"));
    assert!(json.contains("high"));
  }

  #[test]
  fn test_method_info_serialization() {
    // Test: Validates MethodInfo structure serializes correctly with method details
    // Justification: New feature for capturing method-level documentation
    let method = MethodInfo {
      name: String::from("save"),
      signature: String::from("fn save(&mut self, entity: T) -> HexResult<()>"),
      documentation: Some(String::from("Saves an entity to the repository")),
      parameters: vec![
        ParameterInfo {
          name: String::from("self"),
          param_type: String::from("&mut self"),
          description: None,
        },
        ParameterInfo {
          name: String::from("entity"),
          param_type: String::from("T"),
          description: Some(String::from("The entity to save")),
        },
      ],
      return_type: Some(String::from("HexResult<()>")),
      is_public: true,
      is_async: false,
    };

    let json = serde_json::to_string(&method).unwrap();
    assert!(json.contains("save"));
    assert!(json.contains("HexResult"));
    assert!(json.contains("\"is_public\":true"));
  }

  #[test]
  fn test_component_with_methods() {
    // Test: Validates ComponentInfo with methods field serializes correctly
    // Justification: Integration test for new methods feature
    let component = ComponentInfo {
      type_name: String::from("UserRepository"),
      layer: String::from("Port"),
      role: String::from("Repository"),
      module_path: String::from("ports::user_repository"),
      purpose: Some(String::from("Manages user persistence")),
      methods: vec![MethodInfo {
        name: String::from("find_by_id"),
        signature: String::from("fn find_by_id(&self, id: &str) -> HexResult<Option<User>>"),
        documentation: Some(String::from("Finds a user by their ID")),
        parameters: vec![ParameterInfo {
          name: String::from("id"),
          param_type: String::from("&str"),
          description: Some(String::from("User identifier")),
        }],
        return_type: Some(String::from("HexResult<Option<User>>")),
        is_public: true,
        is_async: false,
      }],
      dependencies: vec![],
    };

    let json = serde_json::to_string(&component).unwrap();
    assert!(json.contains("UserRepository"));
    assert!(json.contains("find_by_id"));
    assert!(json.contains("Finds a user"));
  }
}

impl AIContext {
  /// Serialize this AIContext to a JSON string.
  ///
  /// Returns a String on success, or an error message on failure.
  /// Uses serde_json with a stable field order as defined by this struct.
  pub fn to_json(&self) -> Result<String, String> {
    match serde_json::to_string(self) {
      Ok(s) => Ok(s),
      Err(e) => Err(format!("Serialization error: {}", e)),
    }
  }
}

#[cfg(test)]
mod tests_to_json {
  #[test]
  fn test_ai_context_to_json_method() {
    let ctx = super::AIContext {
      architecture: String::from("hexagonal"),
      version: String::from("0.3.0"),
      components: Vec::new(),
      relationships: Vec::new(),
      constraints: super::ConstraintSet {
        dependency_rules: Vec::new(),
        layer_boundaries: Vec::new(),
        naming_conventions: Vec::new(),
        required_patterns: Vec::new(),
      },
      suggestions: Vec::new(),
      metadata: super::ContextMetadata {
        generated_at: String::from("2025-10-06T17:59:00Z"),
        hex_version: String::from("0.3.0"),
        total_components: 0,
        total_relationships: 0,
        schema_version: String::from("1.0.0"),
      },
    };

    let json = ctx.to_json().unwrap();
    assert!(json.contains("\"schema_version\""));
    assert!(json.contains("\"hexagonal\""));
  }
}
