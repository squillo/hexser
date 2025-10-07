//! Visual styling configuration.
//!
//! Defines colors, shapes, and other visual properties.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial VisualStyle implementation.

/// Visual styling configuration
#[derive(Clone, Debug)]
pub struct VisualStyle {
    pub color_scheme: ColorScheme,
}

/// Color scheme for layers
#[derive(Clone, Debug)]
pub enum ColorScheme {
    Default,
}

impl VisualStyle {
    /// Get color for layer
    pub fn color_for_layer(&self, layer: &crate::graph::layer::Layer) -> String {
        match layer {
            crate::graph::layer::Layer::Domain => String::from("lightblue"),
            crate::graph::layer::Layer::Port => String::from("lightgreen"),
            crate::graph::layer::Layer::Adapter => String::from("lightyellow"),
            crate::graph::layer::Layer::Application => String::from("lightcoral"),
            crate::graph::layer::Layer::Infrastructure => String::from("lightgray"),
            crate::graph::layer::Layer::Unknown => String::from("red"),
        }
    }
}

impl Default for VisualStyle {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::Default,
        }
    }
}
