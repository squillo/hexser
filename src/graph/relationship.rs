//! Relationship enum for component relationships in the graph.
//!
//! The Relationship enum defines the types of relationships that can exist
//! between components in the hexagonal architecture graph. Relationships
//! capture dependencies, data flow, and architectural patterns, enabling
//! intent inference and validation of architectural rules.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Relationship enum definition for graph edges.

/// Enum representing relationship types between components.
///
/// Relationships define how components interact and depend on each other,
/// enabling graph analysis and pattern detection.
///
/// # Example
///
/// ```rust
/// use hex::graph::Relationship;
///
/// let rel = Relationship::Implements;
/// assert!(matches!(rel, Relationship::Implements));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relationship {
    /// Adapter implements a port interface.
    Implements,

    /// Component depends on another component.
    Depends,

    /// Mapper transforms data between representations.
    Transforms,

    /// Aggregate contains entities or value objects.
    Aggregates,

    /// Application invokes a use case.
    Invokes,

    /// Component produces domain events.
    Produces,

    /// Component consumes domain events.
    Consumes,

    /// Component validates another component.
    Validates,

    /// Component provides configuration.
    Configures,

    /// Unknown or unclassified relationship.
    Unknown,
}

impl Relationship {
    /// Returns the name of the relationship as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Implements => "Implements",
            Self::Depends => "Depends",
            Self::Transforms => "Transforms",
            Self::Aggregates => "Aggregates",
            Self::Invokes => "Invokes",
            Self::Produces => "Produces",
            Self::Consumes => "Consumes",
            Self::Validates => "Validates",
            Self::Configures => "Configures",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_equality() {
        assert_eq!(Relationship::Implements, Relationship::Implements);
        assert_ne!(Relationship::Implements, Relationship::Depends);
    }

    #[test]
    fn test_relationship_as_str() {
        assert_eq!(Relationship::Implements.as_str(), "Implements");
        assert_eq!(Relationship::Depends.as_str(), "Depends");
    }

    #[test]
    fn test_relationship_display() {
        let rel = Relationship::Implements;
        assert_eq!(format!("{}", rel), "Implements");
    }
}
