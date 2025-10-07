//! Fluent query API for exploring the hexagonal architecture graph.
//!
//! Provides builder pattern for filtering and traversing nodes.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Initial query API implementation for Phase 4.

/// Fluent query builder for graph exploration
pub struct GraphQuery<'g> {
    graph: &'g crate::graph::hex_graph::HexGraph,
    filters: Vec<QueryFilter>,
}

/// Query filters
#[derive(Clone)]
pub enum QueryFilter {
    Layer(crate::graph::layer::Layer),
    Role(crate::graph::role::Role),
    TypeNameContains(String),
    ModulePathContains(String),
}

impl<'g> GraphQuery<'g> {
    /// Create query for graph
    pub fn new(graph: &'g crate::graph::hex_graph::HexGraph) -> Self {
        Self {
            graph,
            filters: Vec::new(),
        }
    }

    /// Filter by layer
    pub fn layer(mut self, layer: crate::graph::layer::Layer) -> Self {
        self.filters.push(QueryFilter::Layer(layer));
        self
    }

    /// Filter by role
    pub fn role(mut self, role: crate::graph::role::Role) -> Self {
        self.filters.push(QueryFilter::Role(role));
        self
    }

    /// Filter by type name substring
    pub fn type_name_contains(mut self, substring: impl Into<String>) -> Self {
        self.filters.push(QueryFilter::TypeNameContains(substring.into()));
        self
    }

    /// Filter by module path substring
    pub fn module_path_contains(mut self, substring: impl Into<String>) -> Self {
        self.filters.push(QueryFilter::ModulePathContains(substring.into()));
        self
    }

    /// Execute query and return matching nodes
    pub fn execute(&self) -> Vec<&crate::graph::hex_node::HexNode> {
        self.graph
            .nodes()
            .filter(|node| self.matches_all_filters(node))
            .collect()
    }

    /// Count matching nodes without collecting
    pub fn count(&self) -> usize {
        self.graph
            .nodes()
            .filter(|node| self.matches_all_filters(node))
            .count()
    }

    /// Get first matching node
    pub fn first(&self) -> Option<&crate::graph::hex_node::HexNode> {
        self.graph
            .nodes()
            .find(|node| self.matches_all_filters(node))
    }

    fn matches_all_filters(&self, node: &crate::graph::hex_node::HexNode) -> bool {
        self.filters.iter().all(|filter| self.matches_filter(node, filter))
    }

    fn matches_filter(&self, node: &crate::graph::hex_node::HexNode, filter: &QueryFilter) -> bool {
        match filter {
            QueryFilter::Layer(layer) => node.layer == *layer,
            QueryFilter::Role(role) => node.role == *role,
            QueryFilter::TypeNameContains(substring) => node.type_name.contains(substring),
            QueryFilter::ModulePathContains(substring) => node.module_path.contains(substring),
        }
    }
}

impl crate::graph::hex_graph::HexGraph {
    /// Create query for this graph
    pub fn query(&self) -> GraphQuery {
        GraphQuery::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_by_layer() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("TestEntity"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "TestEntity",
                "test::domain",
            ))
            .build();

        let results = graph.query()
            .layer(crate::graph::layer::Layer::Domain)
            .execute();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_query_by_role() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("TestRepo"),
                crate::graph::layer::Layer::Port,
                crate::graph::role::Role::Repository,
                "TestRepo",
                "test::ports",
            ))
            .build();

        let results = graph.query()
            .role(crate::graph::role::Role::Repository)
            .execute();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_query_count() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("Test1"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Test1",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("Test2"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Test2",
                "test",
            ))
            .build();

        let count = graph.query()
            .layer(crate::graph::layer::Layer::Domain)
            .count();

        assert_eq!(count, 2);
    }
}
