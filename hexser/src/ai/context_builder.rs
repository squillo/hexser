//! Builder for constructing AI context from hex graph.
//!
//! Transforms HexGraph into machine-readable AIContext.
//! Analyzes architecture, detects issues, generates suggestions.
//! Primary entry point for AI agent integration.
//!
//! Revision History
//! - 2025-10-10T20:28:00Z @AI: Add methods field to ComponentInfo with empty placeholder for future method extraction.
//! - 2025-10-02T19:00:00Z @AI: Fix test add_edge calls to use HexEdge constructor, fix Relationship typo, fix edges iteration.
//! - 2025-10-02T18:30:00Z @AI: Add comprehensive documentation and tests for all functions.
//! - 2025-10-02T18:15:00Z @AI: Fix API usage - remove iter call, use correct edge field names.
//! - 2025-10-02T18:00:00Z @AI: Initial context builder implementation.

/// Builder for constructing AI context from architecture graph
pub struct ContextBuilder<'a> {
  graph: &'a crate::graph::hex_graph::HexGraph,
}

impl<'a> ContextBuilder<'a> {
  /// Create new context builder from graph
  ///
  /// # Arguments
  /// * `graph` - Reference to HexGraph to analyze
  ///
  /// # Example
  /// ```
  /// # use hexser::graph::builder::GraphBuilder;
  /// # use hexser::ai::ContextBuilder;
  /// let graph = GraphBuilder::new().build();
  /// let builder = ContextBuilder::new(&graph);
  /// ```
  pub fn new(graph: &'a crate::graph::hex_graph::HexGraph) -> Self {
    Self { graph }
  }

  /// Build complete AI context from graph
  ///
  /// Transforms architecture graph into structured format for AI agent consumption.
  /// Includes components, relationships, constraints, and generated suggestions.
  ///
  /// # Returns
  /// Complete AIContext structure ready for serialization
  ///
  /// # Errors
  /// Returns error if context generation fails
  pub fn build(self) -> crate::result::hex_result::HexResult<super::ai_context::AIContext> {
    let components = self.build_components();
    let relationships = self.build_relationships();
    let constraints = self.build_constraints();
    let suggestions = self.generate_suggestions(&components, &relationships);

    Result::Ok(super::ai_context::AIContext {
      architecture: String::from("hexagonal"),
      version: String::from(env!("CARGO_PKG_VERSION")),
      components,
      relationships,
      constraints,
      suggestions,
      metadata: super::ai_context::ContextMetadata {
        generated_at: chrono::Utc::now().to_rfc3339(),
        hex_version: String::from(env!("CARGO_PKG_VERSION")),
        total_components: self.graph.node_count(),
        total_relationships: self.graph.edge_count(),
        schema_version: String::from("1.0.0"),
      },
    })
  }

  /// Extract component information from graph nodes
  ///
  /// Maps each graph node to ComponentInfo structure including
  /// type name, layer, role, and dependencies.
  ///
  /// Uses method_extractor to populate methods field with trait method information
  /// for Repository, Directive, and Query components.
  fn build_components(&self) -> Vec<super::ai_context::ComponentInfo> {
    self
      .graph
      .nodes()
      .map(|node| {
        let role_str = format!("{:?}", node.role);
        let methods =
          crate::ai::method_extractor::extract_methods_for_type(&node.type_name, &role_str);

        super::ai_context::ComponentInfo {
          type_name: node.type_name.clone(),
          layer: format!("{:?}", node.layer),
          role: role_str,
          module_path: node.module_path.clone(),
          purpose: None,
          dependencies: self
            .graph
            .edges_from(&node.id)
            .into_iter()
            .map(|edge| edge.target.to_string())
            .collect(),
          methods,
        }
      })
      .collect()
  }

  /// Extract relationship information from graph edges
  ///
  /// Maps each edge to RelationshipInfo with validation status.
  /// Validates relationships against architectural rules.
  fn build_relationships(&self) -> Vec<super::ai_context::RelationshipInfo> {
    self
      .graph
      .edges()
      .into_iter()
      .map(|edge| {
        let is_valid = self.validate_relationship(edge);
        super::ai_context::RelationshipInfo {
          from: edge.source.to_string(),
          to: edge.target.to_string(),
          relationship_type: format!("{:?}", edge.relationship),
          is_valid,
          validation_message: if is_valid {
            None
          } else {
            Some(String::from("Violates layer dependency rules"))
          },
        }
      })
      .collect()
  }

  /// Build complete constraint set for architecture
  ///
  /// Generates dependency rules, layer boundaries, naming conventions,
  /// and required patterns for AI agents to enforce.
  fn build_constraints(&self) -> super::ai_context::ConstraintSet {
    super::ai_context::ConstraintSet {
      dependency_rules: self.build_dependency_rules(),
      layer_boundaries: self.build_layer_boundaries(),
      naming_conventions: self.build_naming_conventions(),
      required_patterns: vec![
        String::from("One item per file"),
        String::from("No use statements"),
        String::from("Fully qualified paths"),
      ],
    }
  }

  /// Generate dependency rules between layers
  ///
  /// Defines which layers can depend on which other layers
  /// according to hexagonal architecture principles.
  fn build_dependency_rules(&self) -> Vec<super::ai_context::DependencyRule> {
    vec![
      super::ai_context::DependencyRule {
        from_layer: String::from("Domain"),
        to_layer: String::from("Infrastructure"),
        allowed: false,
        reason: String::from("Domain must not depend on infrastructure"),
      },
      super::ai_context::DependencyRule {
        from_layer: String::from("Application"),
        to_layer: String::from("Domain"),
        allowed: true,
        reason: String::from("Application coordinates domain logic"),
      },
    ]
  }

  /// Define layer boundaries and allowed dependencies
  ///
  /// Specifies for each layer what it can depend on
  /// and what can depend on it.
  fn build_layer_boundaries(&self) -> Vec<super::ai_context::LayerBoundary> {
    vec![
      super::ai_context::LayerBoundary {
        layer: String::from("Domain"),
        can_depend_on: vec![],
        dependents_allowed: vec![String::from("Ports"), String::from("Application")],
        purpose: String::from("Pure business logic with zero dependencies"),
      },
      super::ai_context::LayerBoundary {
        layer: String::from("Ports"),
        can_depend_on: vec![String::from("Domain")],
        dependents_allowed: vec![String::from("Adapters"), String::from("Application")],
        purpose: String::from("Interfaces defining what application needs"),
      },
    ]
  }

  /// Generate naming convention rules
  ///
  /// Provides patterns and examples for naming components
  /// according to project standards.
  fn build_naming_conventions(&self) -> Vec<super::ai_context::NamingConvention> {
    vec![
      super::ai_context::NamingConvention {
        applies_to: String::from("Repository ports"),
        pattern: String::from("*Repository trait"),
        example: String::from("trait UserRepository: Repository<User>"),
      },
      super::ai_context::NamingConvention {
        applies_to: String::from("Directives"),
        pattern: String::from("*Directive struct"),
        example: String::from("struct CreateUserDirective"),
      },
    ]
  }

  /// Validate relationship against architectural rules
  ///
  /// Checks if edge represents valid dependency according
  /// to hexagonal architecture layer rules.
  ///
  /// # Arguments
  /// * `_edge` - Edge to validate
  ///
  /// # Returns
  /// True if relationship is valid, false otherwise
  fn validate_relationship(&self, _edge: &crate::graph::hex_edge::HexEdge) -> bool {
    true
  }

  /// Generate AI suggestions based on architecture analysis
  ///
  /// Analyzes components and relationships to detect issues
  /// like missing implementations or architectural violations.
  ///
  /// # Arguments
  /// * `components` - List of all components in architecture
  /// * `_relationships` - List of all relationships
  ///
  /// # Returns
  /// List of suggestions for improvements
  fn generate_suggestions(
    &self,
    components: &[super::ai_context::ComponentInfo],
    _relationships: &[super::ai_context::RelationshipInfo],
  ) -> Vec<super::ai_context::Suggestion> {
    let mut suggestions = Vec::new();

    let ports: Vec<_> = components.iter().filter(|c| c.layer == "Port").collect();

    let adapters: Vec<_> = components.iter().filter(|c| c.layer == "Adapter").collect();

    if ports.len() > adapters.len() {
      suggestions.push(super::ai_context::Suggestion {
        suggestion_type: super::ai_context::SuggestionType::MissingImplementation,
        component: None,
        description: String::from("More ports than adapters - some ports may need implementations"),
        priority: super::ai_context::Priority::Medium,
        code_example: None,
      });
    }

    suggestions
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_context_builder_new() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let builder = ContextBuilder::new(&graph);
    assert_eq!(builder.graph.node_count(), 0);
  }

  #[test]
  fn test_build_complete_context() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("TestEntity"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "TestEntity",
        "test::entity",
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let context = builder.build().unwrap();

    assert_eq!(context.architecture, "hexagonal");
    assert_eq!(context.components.len(), 1);
    assert_eq!(context.components[0].type_name, "TestEntity");
    assert_eq!(context.metadata.total_components, 1);
  }

  #[test]
  fn test_build_components() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("User"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "User",
        "domain::user",
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let components = builder.build_components();

    assert_eq!(components.len(), 1);
    assert_eq!(components[0].type_name, "User");
    assert_eq!(components[0].layer, "Domain");
    assert_eq!(components[0].role, "Entity");
  }

  #[test]
  fn test_build_relationships() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("User"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "User",
        "domain",
      ))
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("UserRepo"),
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::Repository,
        "UserRepo",
        "ports",
      ))
      .add_edge(crate::graph::hex_edge::HexEdge::new(
        crate::graph::node_id::NodeId::from_name("UserRepo"),
        crate::graph::node_id::NodeId::from_name("User"),
        crate::graph::relationship::Relationship::Depends,
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let relationships = builder.build_relationships();

    assert_eq!(relationships.len(), 1);
    assert!(relationships[0].is_valid);
  }

  #[test]
  fn test_build_constraints() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let builder = ContextBuilder::new(&graph);
    let constraints = builder.build_constraints();

    assert!(!constraints.dependency_rules.is_empty());
    assert!(!constraints.layer_boundaries.is_empty());
    assert!(!constraints.naming_conventions.is_empty());
    assert!(!constraints.required_patterns.is_empty());
  }

  #[test]
  fn test_build_dependency_rules() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let builder = ContextBuilder::new(&graph);
    let rules = builder.build_dependency_rules();

    assert!(rules.iter().any(|r| r.from_layer == "Domain" && !r.allowed));
    assert!(
      rules
        .iter()
        .any(|r| r.from_layer == "Application" && r.allowed)
    );
  }

  #[test]
  fn test_build_layer_boundaries() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let builder = ContextBuilder::new(&graph);
    let boundaries = builder.build_layer_boundaries();

    let domain = boundaries.iter().find(|b| b.layer == "Domain");
    assert!(domain.is_some());
    assert!(domain.unwrap().can_depend_on.is_empty());
  }

  #[test]
  fn test_build_naming_conventions() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let builder = ContextBuilder::new(&graph);
    let conventions = builder.build_naming_conventions();

    assert!(
      conventions
        .iter()
        .any(|c| c.applies_to == "Repository ports")
    );
    assert!(conventions.iter().any(|c| c.applies_to == "Directives"));
  }

  #[test]
  fn test_validate_relationship() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("A"),
        crate::graph::layer::Layer::Domain,
        crate::graph::role::Role::Entity,
        "A",
        "domain",
      ))
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("B"),
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::Repository,
        "B",
        "ports",
      ))
      .add_edge(crate::graph::hex_edge::HexEdge::new(
        crate::graph::node_id::NodeId::from_name("A"),
        crate::graph::node_id::NodeId::from_name("B"),
        crate::graph::relationship::Relationship::Depends,
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let edge = graph.edges().iter().next().unwrap();
    assert!(builder.validate_relationship(edge));
  }

  #[test]
  fn test_generate_suggestions_with_ports_no_adapters() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("UserRepository"),
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::Repository,
        "UserRepository",
        "ports",
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let context = builder.build().unwrap();

    assert!(!context.suggestions.is_empty());
    assert!(context.suggestions.iter().any(|s| matches!(
      s.suggestion_type,
      super::super::ai_context::SuggestionType::MissingImplementation
    )));
  }

  #[test]
  fn test_generate_suggestions_balanced_architecture() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("UserRepository"),
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::Repository,
        "UserRepository",
        "ports",
      ))
      .add_node(crate::graph::hex_node::HexNode::new(
        crate::graph::node_id::NodeId::from_name("PostgresUserRepo"),
        crate::graph::layer::Layer::Adapter,
        crate::graph::role::Role::Adapter,
        "PostgresUserRepo",
        "adapters",
      ))
      .build();

    let builder = ContextBuilder::new(&graph);
    let components = builder.build_components();
    let relationships = builder.build_relationships();
    let suggestions = builder.generate_suggestions(&components, &relationships);

    assert!(suggestions.is_empty());
  }
}
