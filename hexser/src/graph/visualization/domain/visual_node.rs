//! Visual node domain model.
//!
//! Represents a node in the visual graph with styling information.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial VisualNode implementation.

/// Visual node with styling
#[derive(Clone, Debug)]
pub struct VisualNode {
    pub id: String,
    pub label: String,
    pub layer: String,
    pub role: String,
    pub color: String,
    pub shape: String,
}

impl VisualNode {
    /// Create from HexNode
    pub fn from_hex_node(
        node: &crate::graph::hex_node::HexNode,
        style: &crate::graph::visualization::domain::visual_style::VisualStyle,
    ) -> Self {
        let color = style.color_for_layer(&node.layer);
        let shape = String::from("box");

        Self {
            id: node.id.to_string(),
            label: node.type_name.to_string(),
            layer: format!("{:?}", node.layer),
            role: format!("{:?}", node.role),
            color,
            shape,
        }
    }
}
