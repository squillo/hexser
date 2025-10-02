//! Config marker trait for infrastructure configuration.
//!
//! Configuration components handle setup and initialization of infrastructure
//! concerns like database connections, message queue clients, HTTP servers,
//! and external service integrations. They provide a clear separation between
//! configuration concerns and business logic.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Config marker trait definition.

/// Marker trait for infrastructure configuration components.
///
/// Config components handle initialization and setup of infrastructure
/// such as databases, message brokers, or external services.
///
/// # Example
///
/// ```rust
/// use hex::infrastructure::Config;
///
/// struct DatabaseConfig {
///     connection_string: String,
///     pool_size: u32,
/// }
///
/// impl Config for DatabaseConfig {}
///
/// impl DatabaseConfig {
///     fn new(connection_string: String) -> Self {
///         Self {
///             connection_string,
///             pool_size: 10,
///         }
///     }
/// }
/// ```
pub trait Config {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig {
        value: String,
    }

    impl Config for TestConfig {}

    #[test]
    fn test_config_marker_compiles() {
        let _config = TestConfig {
            value: String::from("test"),
        };
    }
}
