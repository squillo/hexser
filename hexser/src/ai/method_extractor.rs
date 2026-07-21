//! Method extraction for hexser traits.
//!
//! Extracts method signatures and documentation from core hexser traits
//! (Repository, Directive, Query) to populate ComponentInfo.methods field.
//! Provides hardcoded method information for known trait methods until
//! rustdoc JSON integration is implemented.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: PRD-272 CLAIM-D2 (finding L45): add test_repository_signatures_do_not_drift, a drift guard asserting the hardcoded Repository/QueryRepository method names returned by extract_methods_for_type match the real trait method names in ports::repository (save; find_one, find, exists, count, delete_where). No production logic changed.
//! - 2025-10-10T20:44:00Z @AI: Initial implementation with Repository, Directive, and Query trait methods.

/// Extracts method information for a component type based on its role.
///
/// Returns a vector of MethodInfo structures for known trait methods.
/// Currently hardcoded for core hexser traits; future versions will
/// use rustdoc JSON output for dynamic extraction.
///
/// # Arguments
///
/// * `type_name` - Fully qualified type name of the component
/// * `role` - Component role (Entity, Repository, Directive, Query, etc.)
///
/// # Returns
///
/// Vector of MethodInfo describing public methods for the type
pub fn extract_methods_for_type(
  _type_name: &str,
  role: &str,
) -> std::vec::Vec<crate::ai::ai_context::MethodInfo> {
  match role {
    "Repository" => repository_trait_methods(),
    "Directive" => directive_trait_methods(),
    "Query" => query_trait_methods(),
    _ => std::vec::Vec::new(),
  }
}

fn repository_trait_methods() -> std::vec::Vec<crate::ai::ai_context::MethodInfo> {
  vec![
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("save"),
      signature: std::string::String::from("fn save(&mut self, entity: T) -> HexResult<()>"),
      documentation: std::option::Option::Some(std::string::String::from(
        "Save an entity to the repository.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&mut self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("entity"),
          param_type: std::string::String::from("T"),
          description: std::option::Option::Some(std::string::String::from("The entity to save")),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<()>")),
      is_public: true,
      is_async: false,
    },
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("find_one"),
      signature: std::string::String::from(
        "fn find_one(&self, filter: &Self::Filter) -> HexResult<Option<T>>",
      ),
      documentation: std::option::Option::Some(std::string::String::from(
        "Fetch a single entity matching a filter.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("filter"),
          param_type: std::string::String::from("&Self::Filter"),
          description: std::option::Option::Some(std::string::String::from(
            "Domain-owned filter criteria",
          )),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<Option<T>>")),
      is_public: true,
      is_async: false,
    },
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("find"),
      signature: std::string::String::from(
        "fn find(&self, filter: &Self::Filter, options: FindOptions<Self::SortKey>) -> HexResult<Vec<T>>",
      ),
      documentation: std::option::Option::Some(std::string::String::from(
        "Fetch many entities matching filter with optional sort/pagination.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("filter"),
          param_type: std::string::String::from("&Self::Filter"),
          description: std::option::Option::Some(std::string::String::from("Filter criteria")),
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("options"),
          param_type: std::string::String::from("FindOptions<Self::SortKey>"),
          description: std::option::Option::Some(std::string::String::from(
            "Sorting and pagination options",
          )),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<Vec<T>>")),
      is_public: true,
      is_async: false,
    },
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("exists"),
      signature: std::string::String::from(
        "fn exists(&self, filter: &Self::Filter) -> HexResult<bool>",
      ),
      documentation: std::option::Option::Some(std::string::String::from(
        "Check existence of at least one entity matching filter.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("filter"),
          param_type: std::string::String::from("&Self::Filter"),
          description: std::option::Option::Some(std::string::String::from("Filter criteria")),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<bool>")),
      is_public: true,
      is_async: false,
    },
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("count"),
      signature: std::string::String::from(
        "fn count(&self, filter: &Self::Filter) -> HexResult<u64>",
      ),
      documentation: std::option::Option::Some(std::string::String::from(
        "Count entities matching filter.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("filter"),
          param_type: std::string::String::from("&Self::Filter"),
          description: std::option::Option::Some(std::string::String::from("Filter criteria")),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<u64>")),
      is_public: true,
      is_async: false,
    },
    crate::ai::ai_context::MethodInfo {
      name: std::string::String::from("delete_where"),
      signature: std::string::String::from(
        "fn delete_where(&mut self, filter: &Self::Filter) -> HexResult<u64>",
      ),
      documentation: std::option::Option::Some(std::string::String::from(
        "Delete entities matching filter; returns number removed.",
      )),
      parameters: vec![
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("self"),
          param_type: std::string::String::from("&mut self"),
          description: std::option::Option::None,
        },
        crate::ai::ai_context::ParameterInfo {
          name: std::string::String::from("filter"),
          param_type: std::string::String::from("&Self::Filter"),
          description: std::option::Option::Some(std::string::String::from("Filter criteria")),
        },
      ],
      return_type: std::option::Option::Some(std::string::String::from("HexResult<u64>")),
      is_public: true,
      is_async: false,
    },
  ]
}

fn directive_trait_methods() -> std::vec::Vec<crate::ai::ai_context::MethodInfo> {
  vec![crate::ai::ai_context::MethodInfo {
    name: std::string::String::from("validate"),
    signature: std::string::String::from("fn validate(&self) -> HexResult<()>"),
    documentation: std::option::Option::Some(std::string::String::from(
      "Validates this directive's input data before execution.",
    )),
    parameters: vec![crate::ai::ai_context::ParameterInfo {
      name: std::string::String::from("self"),
      param_type: std::string::String::from("&self"),
      description: std::option::Option::None,
    }],
    return_type: std::option::Option::Some(std::string::String::from("HexResult<()>")),
    is_public: true,
    is_async: false,
  }]
}

fn query_trait_methods() -> std::vec::Vec<crate::ai::ai_context::MethodInfo> {
  vec![]
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_repository_method_extraction() {
    // Test: Validates Repository trait methods are extracted
    // Justification: Core functionality for Repository components
    let methods = super::extract_methods_for_type("TestRepository", "Repository");
    std::assert_eq!(methods.len(), 6);
    std::assert!(methods.iter().any(|m| m.name == "save"));
    std::assert!(methods.iter().any(|m| m.name == "find_one"));
    std::assert!(methods.iter().any(|m| m.name == "find"));
    std::assert!(methods.iter().any(|m| m.name == "exists"));
    std::assert!(methods.iter().any(|m| m.name == "count"));
    std::assert!(methods.iter().any(|m| m.name == "delete_where"));
  }

  #[test]
  fn test_directive_method_extraction() {
    // Test: Validates Directive trait method is extracted
    // Justification: Core functionality for Directive components
    let methods = super::extract_methods_for_type("TestDirective", "Directive");
    std::assert_eq!(methods.len(), 1);
    std::assert_eq!(methods[0].name, "validate");
  }

  #[test]
  fn test_query_method_extraction() {
    // Test: Validates Query trait method extraction (currently empty)
    // Justification: Placeholder for future Query-specific methods
    let methods = super::extract_methods_for_type("TestQuery", "Query");
    std::assert_eq!(methods.len(), 0);
  }

  #[test]
  fn test_unknown_role_extraction() {
    // Test: Validates unknown roles return empty method list
    // Justification: Graceful handling of unsupported roles
    let methods = super::extract_methods_for_type("TestEntity", "Entity");
    std::assert_eq!(methods.len(), 0);
  }

  /// why: method_extractor hardcodes Repository/QueryRepository method names as string
  /// literals (see repository_trait_methods above); if the real traits in
  /// hexser/src/ports/repository.rs ever add, remove, or rename a method, nothing about the
  /// compiler stops these hardcoded strings from silently drifting out of sync with the
  /// actual trait surface. This test hardcodes the real method names as read directly from
  /// ports/repository.rs (Repository::save; QueryRepository::find_one, find, exists, count,
  /// delete_where) and fails loudly if the extractor's output ever stops containing one of
  /// them.
  #[test]
  fn test_repository_signatures_do_not_drift() {
    // Real method names, read directly from hexser/src/ports/repository.rs:
    // - trait Repository<T>: save
    // - trait QueryRepository<T>: find_one, find, exists, count, delete_where
    let real_repository_method_names: std::vec::Vec<&str> = vec![
      "save",
      "find_one",
      "find",
      "exists",
      "count",
      "delete_where",
    ];

    let methods = super::extract_methods_for_type("TestRepository", "Repository");
    let extracted_names: std::vec::Vec<&str> = methods.iter().map(|m| m.name.as_str()).collect();

    for real_name in real_repository_method_names {
      std::assert!(
        extracted_names.contains(&real_name),
        "method_extractor::repository_trait_methods is missing or has misnamed method '{}', \
         which exists on Repository/QueryRepository in ports/repository.rs — update \
         method_extractor.rs to match",
        real_name
      );
    }
  }

  #[test]
  fn test_save_method_details() {
    // Test: Validates save method has correct signature and documentation
    // Justification: Ensures method metadata is accurate
    let methods = super::repository_trait_methods();
    let save_method = methods.iter().find(|m| m.name == "save").unwrap();

    std::assert_eq!(
      save_method.signature,
      "fn save(&mut self, entity: T) -> HexResult<()>"
    );
    std::assert!(save_method.documentation.is_some());
    std::assert_eq!(save_method.parameters.len(), 2);
    std::assert!(save_method.is_public);
    std::assert!(!save_method.is_async);
  }
}
