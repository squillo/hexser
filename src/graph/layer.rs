//! Layer enum for hexagonal architecture layers.
//!
//! The Layer enum identifies which architectural layer a component belongs to.
//! This is used for graph-based analysis, visualization, and validation of
//! architectural boundaries. Each layer has specific responsibilities and
//! dependency rules in hexagonal architecture.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Layer enum definition for architecture layers.

/// Enum representing architectural layers in hexagonal architecture.
///
/// Layers define the organizational structure of the application, with
/// each layer having specific responsibilities and dependency rules.
///
/// # Example
///
/// ```rust
/// use hexer::graph::Layer;
///
/// let layer = Layer::Domain;
/// assert!(matches!(layer, Layer::Domain));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    /// Core business logic layer, no infrastructure dependencies.
    Domain,

    /// Interface definitions for external interactions.
    Port,

    /// Concrete implementations of ports using specific technologies.
    Adapter,

    /// Use case orchestration layer.
    Application,

    /// External concerns like databases and APIs.
    Infrastructure,

    /// Unknown or unclassified layer.
    Unknown,
}

impl Layer {
    /// Returns the name of the layer as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Domain => "Domain",
            Self::Port => "Port",
            Self::Adapter => "Adapter",
            Self::Application => "Application",
            Self::Infrastructure => "Infrastructure",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_equality() {
        assert_eq!(Layer::Domain, Layer::Domain);
        assert_ne!(Layer::Domain, Layer::Port);
    }

    #[test]
    fn test_layer_as_str() {
        assert_eq!(Layer::Domain.as_str(), "Domain");
        assert_eq!(Layer::Port.as_str(), "Port");
    }

    #[test]
    fn test_layer_display() {
        let layer = Layer::Domain;
        assert_eq!(format!("{}", layer), "Domain");
    }
}
