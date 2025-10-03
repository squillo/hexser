//! Adapter marker trait for port implementations.
//!
//! Adapters implement ports using specific technologies or frameworks.
//! They act as a bridge between the application core and external systems,
//! translating between domain concepts and infrastructure concerns.
//! This trait serves as a marker to identify adapter components in the architecture.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Adapter marker trait definition.

/// Marker trait for adapters that implement ports.
///
/// Adapters provide concrete implementations of port interfaces using
/// specific technologies, frameworks, or external systems.
///
/// # Example
///
/// ```rust
/// use hexer::adapters::Adapter;
/// use hexer::ports::Repository;
/// use hexer::domain::Entity;
/// use hexer::HexResult;
///
/// struct User {
///     id: String,
/// }
///
/// impl Entity for User {
///     type Id = String;
/// }
///
/// struct PostgresUserRepository {
///     // Database connection
/// }
///
/// impl Adapter for PostgresUserRepository {}
///
/// // Would also implement Repository<User> for PostgresUserRepository
/// ```
pub trait Adapter {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAdapter;

    impl Adapter for TestAdapter {}

    #[test]
    fn test_adapter_marker_compiles() {
        let _adapter = TestAdapter;
    }
}
