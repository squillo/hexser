//! HexGraph is the core immutable graph structure.
//!
//! HexGraph represents the entire hexagonal architecture as an immutable,
//! thread-safe graph. It uses Arc for zero-cost sharing across threads
//! and provides methods for querying nodes and edges. Graphs are constructed
//! using GraphBuilder and cannot be modified after creation.
//!
//! Revision History
//! - 2025-10-02T14:00:00Z @AI: Rename nodes_in_layer to nodes_by_layer and nodes_by_role to nodes_by_role for better API naming.
//! - 2025-10-01T00:03:00Z @AI: Initial immutable HexGraph implementation for Phase 2.

/// Immutable graph representing hexagonal architecture components.
///
/// HexGraph is thread-safe and uses Arc internally for efficient sharing.
/// Once created, it cannot be modified - use GraphBuilder to create new graphs.
///
/// # Example
///
/// ```rust
/// use hexer::graph::{HexGraph, HexNode, NodeId, Layer, Role};
///
/// let graph = HexGraph::builder()
///     .with_node(HexNode::new(
///         NodeId::from_name("Entity"),
///         Layer::Domain,
///         Role::Entity,
///         "MyEntity",
///         "domain",
///     ))
///     .build();
///
/// assert_eq!(graph.node_count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct HexGraph {
    pub(crate) inner: std::sync::Arc<GraphInner>,
}

#[derive(Debug)]
pub(crate) struct GraphInner {
    pub(crate) nodes: std::collections::HashMap<crate::graph::node_id::NodeId, crate::graph::hex_node::HexNode>,
    pub(crate) edges: Vec<crate::graph::hex_edge::HexEdge>,
    pub(crate) metadata: crate::graph::metadata::GraphMetadata,
}

impl HexGraph {
    /// Get the current graph built from registered components
    pub fn current() -> std::sync::Arc<Self> {
        std::sync::Arc::new(crate::registry::component_registry::ComponentRegistry::build_graph())
    }

    /// Create a new empty graph.
    pub fn new() -> Self {
        Self {
            inner: std::sync::Arc::new(GraphInner {
                nodes: std::collections::HashMap::new(),
                edges: Vec::new(),
                metadata: crate::graph::metadata::GraphMetadata::default(),
            }),
        }
    }

    /// Export to DOT format
    #[cfg(feature = "visualization")]
    pub fn to_dot(&self) -> crate::result::hex_result::HexResult<String> {
        let exporter = crate::graph::visualization::adapters::dot_exporter::DotExporter::new();
        let use_case = crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
        use_case.execute(
            self,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        )
    }

    /// Export to Mermaid format
    #[cfg(feature = "visualization")]
    pub fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String> {
        let exporter = crate::graph::visualization::adapters::mermaid_exporter::MermaidExporter::new();
        let use_case = crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
        use_case.execute(
            self,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        )
    }

    /// Export to JSON format
    #[cfg(feature = "visualization")]
    pub fn to_json(&self) -> crate::result::hex_result::HexResult<String> {
        let exporter = crate::graph::visualization::adapters::json_exporter::JsonExporter::new();
        let use_case = crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
        use_case.execute(
            self,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        )
    }

    /// Save visualization to file
    #[cfg(feature = "visualization")]
    pub fn save_visualization(
        &self,
        path: &std::path::Path,
        exporter: &dyn crate::graph::visualization::ports::format_exporter::FormatExporter,
    ) -> crate::result::hex_result::HexResult<()> {
        let use_case = crate::graph::visualization::application::export_graph::ExportGraph::new(exporter);
        let content = use_case.execute(
            self,
            crate::graph::visualization::domain::visual_style::VisualStyle::default(),
        )?;

        std::fs::write(path, content).map_err(|e| {
            crate::error::hex_error::HexError::io(
                "E_HEX_IO_001",
                format!("Failed to write file: {}", e),
            )
        })
    }

    /// Create a new graph builder.
    pub fn builder() -> crate::graph::builder::GraphBuilder {
        crate::graph::builder::GraphBuilder::new()
    }

    /// Get count of distinct layers in graph
    pub fn layer_count(&self) -> usize {
        let mut layers = std::collections::HashSet::new();
        for node in self.nodes() {
            layers.insert(node.layer());
        }
        layers.len()
    }

    /// Get the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.inner.nodes.len()
    }

    /// Get the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.inner.edges.len()
    }

        /// Export architecture context for AI agent consumption
        #[cfg(feature = "ai")]
        pub fn to_ai_context(&self) -> crate::result::hex_result::HexResult<crate::ai::AIContext> {
            crate::ai::ContextBuilder::new(self).build()
        }

    /// Get a node by its ID.
    pub fn get_node(&self, id: &crate::graph::node_id::NodeId) -> Option<&crate::graph::hex_node::HexNode> {
        self.inner.nodes.get(id)
    }

    /// Print human-readable summary of graph
    pub fn pretty_print(&self) {
        println!("Hexagonal Architecture Graph:");
        println!("  Nodes: {}", self.node_count());
        println!("  Edges: {}", self.edge_count());
        println!("\nBy Layer:");
        for layer in [
            crate::graph::layer::Layer::Domain,
            crate::graph::layer::Layer::Port,
            crate::graph::layer::Layer::Adapter,
            crate::graph::layer::Layer::Application,
            crate::graph::layer::Layer::Infrastructure,
        ] {
            let count = self.nodes_by_layer(layer).len();
            if count > 0 {
                println!("  {:?}: {}", layer, count);
            }
        }
    }

    /// Get all nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &crate::graph::hex_node::HexNode> {
        self.inner.nodes.values()
    }

    /// Get all edges in the graph.
    pub fn edges(&self) -> &[crate::graph::hex_edge::HexEdge] {
        &self.inner.edges
    }

    /// Get nodes by layer.
    pub fn nodes_by_layer(&self, layer: crate::graph::layer::Layer) -> Vec<&crate::graph::hex_node::HexNode> {
        self.inner.nodes.values()
            .filter(|n| n.layer() == layer)
            .collect()
    }

    /// Get nodes by role.
    pub fn nodes_by_role(&self, role: crate::graph::role::Role) -> Vec<&crate::graph::hex_node::HexNode> {
        self.inner.nodes.values()
            .filter(|n| n.role() == role)
            .collect()
    }

    /// Get edges from a specific node.
    pub fn edges_from(&self, source: &crate::graph::node_id::NodeId) -> Vec<&crate::graph::hex_edge::HexEdge> {
        self.inner.edges.iter()
            .filter(|e| e.source() == source)
            .collect()
    }

    /// Get edges to a specific node.
    pub fn edges_to(&self, target: &crate::graph::node_id::NodeId) -> Vec<&crate::graph::hex_edge::HexEdge> {
        self.inner.edges.iter()
            .filter(|e| e.target() == target)
            .collect()
    }

    /// Get graph metadata.
    pub fn metadata(&self) -> &crate::graph::metadata::GraphMetadata {
        &self.inner.metadata
    }

    /// Check if graph is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.nodes.is_empty()
    }
}

impl Default for HexGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph = HexGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_graph_thread_safety() {
        let graph = HexGraph::new();
        let graph_clone = graph.clone();

        std::thread::spawn(move || {
            assert_eq!(graph_clone.node_count(), 0);
        }).join().unwrap();
    }

    #[test]
    fn test_graph_default() {
        let graph = HexGraph::default();
        assert!(graph.is_empty());
    }
}
