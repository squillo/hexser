//! Intent inference for detecting architectural patterns.
//!
//! Analyzes graph structure to identify common patterns like Repository, CQRS, etc.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Initial intent inference implementation for Phase 4.

/// Intent inference engine
pub struct IntentInference<'g> {
    graph: &'g crate::graph::hex_graph::HexGraph,
}

/// Detected architectural pattern
#[derive(Debug, Clone, PartialEq)]
pub enum ArchitecturalPattern {
    Repository {
        count: usize,
        repositories: Vec<crate::graph::node_id::NodeId>,
    },
    CQRS {
        directive_count: usize,
        query_count: usize,
    },
    EventSourcing {
        event_count: usize,
        aggregates: Vec<crate::graph::node_id::NodeId>,
    },
}

impl<'g> IntentInference<'g> {
    /// Create intent inference for graph
    pub fn new(graph: &'g crate::graph::hex_graph::HexGraph) -> Self {
        Self { graph }
    }

    /// Identify all architectural patterns in the graph
    pub fn identify_patterns(&self) -> Vec<ArchitecturalPattern> {
        let mut patterns = Vec::new();

        if let Some(pattern) = self.detect_repository_pattern() {
            patterns.push(pattern);
        }

        if let Some(pattern) = self.detect_cqrs_pattern() {
            patterns.push(pattern);
        }

        if let Some(pattern) = self.detect_event_sourcing_pattern() {
            patterns.push(pattern);
        }

        patterns
    }

    fn detect_repository_pattern(&self) -> Option<ArchitecturalPattern> {
        let repositories: Vec<_> = self
            .graph
            .query()
            .role(crate::graph::role::Role::Repository)
            .execute()
            .into_iter()
            .map(|node| node.id)
            .collect();

        if !repositories.is_empty() {
            Some(ArchitecturalPattern::Repository {
                count: repositories.len(),
                repositories,
            })
        } else {
            None
        }
    }

    fn detect_cqrs_pattern(&self) -> Option<ArchitecturalPattern> {
        let directive_count = self
            .graph
            .query()
            .role(crate::graph::role::Role::Directive)
            .count();

        let query_count = self
            .graph
            .query()
            .role(crate::graph::role::Role::Query)
            .count();

        if directive_count > 0 || query_count > 0 {
            Some(ArchitecturalPattern::CQRS {
                directive_count,
                query_count,
            })
        } else {
            None
        }
    }

    fn detect_event_sourcing_pattern(&self) -> Option<ArchitecturalPattern> {
        let event_count = self
            .graph
            .query()
            .role(crate::graph::role::Role::DomainEvent)
            .count();

        let aggregates: Vec<_> = self
            .graph
            .query()
            .role(crate::graph::role::Role::Aggregate)
            .execute()
            .into_iter()
            .map(|node| node.id)
            .collect();

        if event_count > 0 && !aggregates.is_empty() {
            Some(ArchitecturalPattern::EventSourcing {
                event_count,
                aggregates,
            })
        } else {
            None
        }
    }
}

impl crate::graph::hex_graph::HexGraph {
    /// Create intent inference for this graph
    pub fn intent(&self) -> IntentInference {
        IntentInference::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_repository_pattern() {
        let repo_id = crate::graph::node_id::NodeId::from_name("UserRepo");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                repo_id,
                crate::graph::layer::Layer::Port,
                crate::graph::role::Role::Repository,
                "UserRepo",
                "test",
            ))
            .build();

        let patterns = graph.intent().identify_patterns();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            ArchitecturalPattern::Repository { count, .. } => {
                assert_eq!(*count, 1);
            }
            _ => panic!("Expected Repository pattern"),
        }
    }

    #[test]
    fn test_detect_cqrs_pattern() {
        let directive_id = crate::graph::node_id::NodeId::from_name("CreateUser");
        let query_id = crate::graph::node_id::NodeId::from_name("GetUser");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                directive_id,
                crate::graph::layer::Layer::Application,
                crate::graph::role::Role::Directive,
                "CreateUser",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                query_id,
                crate::graph::layer::Layer::Application,
                crate::graph::role::Role::Query,
                "GetUser",
                "test",
            ))
            .build();

        let patterns = graph.intent().identify_patterns();
        assert_eq!(patterns.len(), 1);

        match &patterns[0] {
            ArchitecturalPattern::CQRS {
                directive_count,
                query_count,
            } => {
                assert_eq!(*directive_count, 1);
                assert_eq!(*query_count, 1);
            }
            _ => panic!("Expected CQRS pattern"),
        }
    }
}
