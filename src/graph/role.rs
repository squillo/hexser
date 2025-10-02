//! Role enum for component roles in hexagonal architecture.
//!
//! The Role enum identifies what role a component plays within its layer.
//! Roles provide finer-grained classification than layers and are used
//! for pattern detection, intent inference, and architectural analysis.
//! Each role represents a specific responsibility or pattern in the architecture.
//!
//! Revision History
//! - 2025-10-01T00:01:00Z @AI: Renamed Command to Directive for consistency.
//! - 2025-10-01T00:00:00Z @AI: Initial Role enum definition for component classification.

/// Enum representing component roles in hexagonal architecture.
///
/// Roles identify the specific responsibility of a component within its layer,
/// enabling pattern detection and architectural analysis.
///
/// # Example
///
/// ```rust
/// use hex::graph::Role;
///
/// let role = Role::Entity;
/// assert!(matches!(role, Role::Entity));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// Domain entity with unique identity.
    Entity,

    /// Value object defined by its values.
    ValueObject,

    /// Aggregate root defining consistency boundary.
    Aggregate,

    /// Domain event representing significant occurrence.
    DomainEvent,

    /// Domain service for domain operations.
    DomainService,

    /// Input port for use case entry points.
    InputPort,

    /// Output port for external system interfaces.
    OutputPort,

    /// Repository for persistence abstraction.
    Repository,

    /// Use case for business operations.
    UseCase,

    /// Query for read-only operations.
    Query,

    /// Adapter implementing ports.
    Adapter,

    /// Mapper for data transformation.
    Mapper,

    /// Directive for write operations.
    Directive,

    /// Directive handler for executing directives.
    DirectiveHandler,

    /// Query handler for executing queries.
    QueryHandler,

    /// Configuration component.
    Config,

    /// Unknown or unclassified role.
    Unknown,
}

impl Role {
    /// Returns the name of the role as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Entity => "Entity",
            Self::ValueObject => "ValueObject",
            Self::Aggregate => "Aggregate",
            Self::DomainEvent => "DomainEvent",
            Self::DomainService => "DomainService",
            Self::InputPort => "InputPort",
            Self::OutputPort => "OutputPort",
            Self::Repository => "Repository",
            Self::UseCase => "UseCase",
            Self::Query => "Query",
            Self::Adapter => "Adapter",
            Self::Mapper => "Mapper",
            Self::Directive => "Directive",
            Self::DirectiveHandler => "DirectiveHandler",
            Self::QueryHandler => "QueryHandler",
            Self::Config => "Config",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_equality() {
        assert_eq!(Role::Entity, Role::Entity);
        assert_ne!(Role::Entity, Role::ValueObject);
    }

    #[test]
    fn test_role_as_str() {
        assert_eq!(Role::Entity.as_str(), "Entity");
        assert_eq!(Role::Repository.as_str(), "Repository");
        assert_eq!(Role::Directive.as_str(), "Directive");
    }

    #[test]
    fn test_role_display() {
        let role = Role::Entity;
        assert_eq!(format!("{}", role), "Entity");
    }
}
