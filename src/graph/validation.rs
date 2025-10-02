//! Architectural validation rules for hexagonal architecture.
//!
//! Validates layer dependencies, port implementations, and detects architectural smells.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Initial validation implementation for Phase 4.

/// Architectural validator
pub struct ArchitecturalValidator<'g> {
    graph: &'g crate::graph::hex_graph::HexGraph,
}

/// Layer dependency violation
#[derive(Debug, Clone, PartialEq)]
pub struct LayerViolation {
    pub from: crate::graph::node_id::NodeId,
    pub to: crate::graph::node_id::NodeId,
    pub reason: String,
}

/// Unimplemented port
#[derive(Debug, Clone, PartialEq)]
pub struct UnimplementedPort {
    pub port_id: crate::graph::node_id::NodeId,
    pub port_name: String,
}

/// Architectural smell
#[derive(Debug, Clone, PartialEq)]
pub enum ArchitecturalSmell {
    GodComponent {
        node_id: crate::graph::node_id::NodeId,
        connection_count: usize,
    },
    CircularDependency {
        cycle: Vec<crate::graph::node_id::NodeId>,
    },
    OrphanedComponent {
        node_id: crate::graph::node_id::NodeId,
    },
}

impl<'g> ArchitecturalValidator<'g> {
    /// Create validator for graph
    pub fn new(graph: &'g crate::graph::hex_graph::HexGraph) -> Self {
        Self { graph }
    }

    /// Validate layer dependencies follow hexagonal rules
    pub fn validate_layer_dependencies(&self) -> Result<(), Vec<LayerViolation>> {
        let mut violations = Vec::new();

        for node in self.graph.nodes() {
            for edge in self.graph.edges_from(node.id) {
                if let Some(target) = self.graph.get_node(edge.target) {
                    if !self.is_valid_layer_dependency(node.layer, target.layer) {
                        violations.push(LayerViolation {
                            from: node.id,
                            to: target.id,
                            reason: format!(
                                "{:?} layer cannot depend on {:?} layer",
                                node.layer, target.layer
                            ),
                        });
                    }
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    fn is_valid_layer_dependency(
        &self,
        from: crate::graph::layer::Layer,
        to: crate::graph::layer::Layer,
    ) -> bool {
        match (from, to) {
            (crate::graph::layer::Layer::Domain, crate::graph::layer::Layer::Domain) => true,
            (crate::graph::layer::Layer::Port, crate::graph::layer::Layer::Domain) => true,
            (crate::graph::layer::Layer::Port, crate::graph::layer::Layer::Port) => true,
            (crate::graph::layer::Layer::Adapter, crate::graph::layer::Layer::Port) => true,
            (crate::graph::layer::Layer::Adapter, crate::graph::layer::Layer::Domain) => true,
            (crate::graph::layer::Layer::Application, _) => true,
            (crate::graph::layer::Layer::Infrastructure, _) => true,
            _ => false,
        }
    }

    /// Validate all ports have implementations
    pub fn validate_port_implementations(&self) -> Result<(), Vec<UnimplementedPort>> {
        let mut unimplemented = Vec::new();

        let ports: Vec<_> = self
            .graph
            .query()
            .layer(crate::graph::layer::Layer::Port)
            .execute();

        for port in ports {
            let has_adapter = self
                .graph
                .edges_to(port.id)
                .any(|edge| edge.relationship == crate::graph::relationship::Relationship::Implements);

            if !has_adapter {
                unimplemented.push(UnimplementedPort {
                    port_id: port.id,
                    port_name: port.type_name.to_string(),
                });
            }
        }

        if unimplemented.is_empty() {
            Ok(())
        } else {
            Err(unimplemented)
        }
    }

    /// Detect architectural smells
    pub fn detect_smells(&self) -> Vec<ArchitecturalSmell> {
        let mut smells = Vec::new();

        smells.extend(self.detect_god_components());
        smells.extend(self.detect_circular_dependencies());
        smells.extend(self.detect_orphaned_components());

        smells
    }

    fn detect_god_components(&self) -> Vec<ArchitecturalSmell> {
        const THRESHOLD: usize = 10;

        self.graph
            .nodes()
            .filter_map(|node| {
                let in_count = self.graph.edges_to(node.id).count();
                let out_count = self.graph.edges_from(node.id).count();
                let total = in_count + out_count;

                if total > THRESHOLD {
                    Some(ArchitecturalSmell::GodComponent {
                        node_id: node.id,
                        connection_count: total,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn detect_circular_dependencies(&self) -> Vec<ArchitecturalSmell> {
        self.graph
            .analysis()
            .detect_cycles()
            .into_iter()
            .map(|cycle| ArchitecturalSmell::CircularDependency { cycle })
            .collect()
    }

    fn detect_orphaned_components(&self) -> Vec<ArchitecturalSmell> {
        self.graph
            .nodes()
            .filter_map(|node| {
                let has_connections = self.graph.edges_to(node.id).count() > 0
                    || self.graph.edges_from(node.id).count() > 0;

                if !has_connections {
                    Some(ArchitecturalSmell::OrphanedComponent { node_id: node.id })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl crate::graph::hex_graph::HexGraph {
    /// Create validator for this graph
    pub fn validation(&self) -> ArchitecturalValidator {
        ArchitecturalValidator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_layer_dependencies() {
        let domain_id = crate::graph::node_id::NodeId::from_name("Domain");
        let port_id = crate::graph::node_id::NodeId::from_name("Port");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                domain_id,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Domain",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                port_id,
                crate::graph::layer::Layer::Port,
                crate::graph::role::Role::Repository,
                "Port",
                "test",
            ))
            .add_edge(crate::graph::hex_edge::HexEdge::new(
                port_id,
                domain_id,
                crate::graph::relationship::Relationship::Depends,
            ))
            .build();

        let result = graph.validation().validate_layer_dependencies();
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_orphaned_components() {
        let orphan_id = crate::graph::node_id::NodeId::from_name("Orphan");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                orphan_id,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Orphan",
                "test",
            ))
            .build();

        let smells = graph.validation().detect_smells();
        assert_eq!(smells.len(), 1);
    }
}
