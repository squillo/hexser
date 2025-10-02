//! Visualizable trait for graph visualization showcase.
//!
//! Demonstrates visualization capabilities.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial Visualizable trait.

/// Trait for visualizable components
pub trait Visualizable {
    /// Export to DOT format
    fn to_dot(&self) -> crate::result::hex_result::HexResult<String>;

    /// Export to Mermaid format
    fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String>;

    /// Export to JSON format
    fn to_json(&self) -> crate::result::hex_result::HexResult<String>;
}

impl Visualizable for crate::graph::hex_graph::HexGraph {
    fn to_dot(&self) -> crate::result::hex_result::HexResult<String> {
        self.to_dot()
    }

    fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String> {
        self.to_mermaid()
    }

    fn to_json(&self) -> crate::result::hex_result::HexResult<String> {
        self.to_json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizable_trait() {
        let graph = crate::graph::builder::GraphBuilder::new()
            .add_node(crate::graph::hex_node::HexNode::new(
                crate::graph::node_id::NodeId::from_name("Test"),
                crate::graph::layer::Layer::Domain,
                crate::graph::role::Role::Entity,
                "Test",
                "test",
            ))
            .build();

        assert!(graph.to_dot().is_ok());
        assert!(graph.to_mermaid().is_ok());
        assert!(graph.to_json().is_ok());
    }
}
