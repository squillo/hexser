//! Graph analysis algorithms for architectural insights.
//!
//! Provides cycle detection, coupling metrics, and component analysis.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Initial analysis implementation for Phase 4.

/// Graph analyzer with algorithmic analysis
pub struct GraphAnalysis<'g> {
    graph: &'g crate::graph::hex_graph::HexGraph,
}

/// Coupling metrics for a component
#[derive(Debug, Clone, PartialEq)]
pub struct CouplingMetrics {
    pub afferent: usize,
    pub efferent: usize,
    pub instability: f64,
}

impl<'g> GraphAnalysis<'g> {
    /// Create analyzer for graph
    pub fn new(graph: &'g crate::graph::hex_graph::HexGraph) -> Self {
        Self { graph }
    }

    /// Detect circular dependencies using DFS
    pub fn detect_cycles(&self) -> Vec<Vec<crate::graph::node_id::NodeId>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut path = Vec::new();

        for node in self.graph.nodes() {
            if !visited.contains(&node.id) {
                self.dfs_cycle_detect(
                    &node.id,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    fn dfs_cycle_detect(
        &self,
        node_id: &crate::graph::node_id::NodeId,
        visited: &mut std::collections::HashSet<crate::graph::node_id::NodeId>,
        rec_stack: &mut std::collections::HashSet<crate::graph::node_id::NodeId>,
        path: &mut Vec<crate::graph::node_id::NodeId>,
        cycles: &mut Vec<Vec<crate::graph::node_id::NodeId>>,
    ) {
        visited.insert(*node_id);
        rec_stack.insert(*node_id);
        path.push(*node_id);

        for edge in self.graph.edges_from(*node_id) {
            if !visited.contains(&edge.target) {
                self.dfs_cycle_detect(&edge.target, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(&edge.target) {
                let cycle_start = path.iter().position(|&id| id == edge.target).unwrap();
                cycles.push(path[cycle_start..].to_vec());
            }
        }

        path.pop();
        rec_stack.remove(node_id);
    }

    /// Calculate coupling metrics for a node
    pub fn calculate_coupling(
        &self,
        node_id: crate::graph::node_id::NodeId,
    ) -> Option<CouplingMetrics> {
        if self.graph.get_node(node_id).is_none() {
            return None;
        }

        let afferent = self.graph.edges_to(node_id).count();
        let efferent = self.graph.edges_from(node_id).count();
        let total = afferent + efferent;

        let instability = if total == 0 {
            0.0
        } else {
            efferent as f64 / total as f64
        };

        Some(CouplingMetrics {
            afferent,
            efferent,
            instability,
        })
    }

    /// Find leaf nodes (no outgoing edges)
    pub fn find_leaf_nodes(&self) -> Vec<&crate::graph::hex_node::HexNode> {
        self.graph
            .nodes()
            .filter(|node| self.graph.edges_from(node.id).count() == 0)
            .collect()
    }

    /// Find root nodes (no incoming edges)
    pub fn find_root_nodes(&self) -> Vec<&crate::graph::hex_node::HexNode> {
        self.graph
            .nodes()
            .filter(|node| self.graph.edges_to(node.id).count() == 0)
            .collect()
    }
}

impl crate::graph::hex_graph::HexGraph {
    /// Create analyzer for this graph
    pub fn analysis(&self) -> GraphAnalysis {
        GraphAnalysis::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_no_cycles() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("A"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "A",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("B"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "B",
                "test",
            ))
            .build();

        let cycles = graph.analysis().detect_cycles();
        assert_eq!(cycles.len(), 0);
    }

    #[test]
    fn test_coupling_metrics() {
        let node_a = crate::graph::node_id::NodeId::from_name("A");
        let node_b = crate::graph::node_id::NodeId::from_name("B");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                node_a,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "A",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                node_b,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "B",
                "test",
            ))
            .add_edge(crate::graph::hex_edge::HexEdge::new(
                node_a,
                node_b,
                crate::graph::relationship::Relationship::Depends,
            ))
            .build();

        let metrics = graph.analysis().calculate_coupling(node_a).unwrap();
        assert_eq!(metrics.efferent, 1);
        assert_eq!(metrics.afferent, 0);
    }

    #[test]
    fn test_find_leaf_nodes() {
        let node_a = crate::graph::node_id::NodeId::from_name("A");
        let node_b = crate::graph::node_id::NodeId::from_name("B");

        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                node_a,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "A",
                "test",
            ))
            .add_node(crate::graph::hex_node::HexNode::new(
                node_b,
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "B",
                "test",
            ))
            .add_edge(crate::graph::hex_edge::HexEdge::HexEdge::new(
                node_a,
                node_b,
                crate::graph::relationship::Relationship::Depends,
            ))
            .build();

        let leaves = graph.analysis().find_leaf_nodes();
        assert_eq!(leaves.len(), 1);
    }
}
