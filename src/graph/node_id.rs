//! NodeId struct for type-safe unique component identification.
//!
//! NodeId provides type-safe unique identification for components in the
//! hexagonal architecture graph. It uses type-based identification via
//! std::any::type_name() to generate consistent, deterministic IDs at
//! compile time, with fallback to string-based IDs for dynamic cases.
//!
//! Revision History
//! - 2025-10-02T12:00:00Z @AI: Add from_type_name method for registry compatibility.
//! - 2025-10-01T00:00:00Z @AI: Initial NodeId struct with type-based identification.

/// Type-safe unique identifier for graph nodes.
///
/// NodeId provides deterministic identification based on Rust types,
/// enabling compile-time component registration and graph construction.
///
/// # Example
///
/// ```rust
/// use hexer::graph::NodeId;
///
/// struct MyComponent;
///
/// let id1 = NodeId::of::<MyComponent>();
/// let id2 = NodeId::of::<MyComponent>();
/// assert_eq!(id1, id2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u64);

impl NodeId {
    /// Create a NodeId from a type parameter.
    ///
    /// Uses the type's fully qualified name to generate a deterministic ID.
    pub fn of<T: 'static>() -> Self {
        let type_name = std::any::type_name::<T>();
        Self::from_name(type_name)
    }

    /// Create a NodeId from a string name.
    ///
    /// Useful for dynamic cases where type information is not available.
    pub fn from_name(name: &str) -> Self {
        let hash = Self::hash_string(name);
        Self(hash)
    }

    /// Create a NodeId from a type name string.
    ///
    /// This is equivalent to from_name but provides clearer semantics
    /// when working with type names from NodeInfo.
    pub fn from_type_name(type_name: &str) -> Self {
        Self::from_name(type_name)
    }

    /// Get the raw u64 value of this NodeId.
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    fn hash_string(s: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
        }
        hash
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestType;
    struct OtherType;

    #[test]
    fn test_node_id_of_same_type() {
        let id1 = NodeId::of::<TestType>();
        let id2 = NodeId::of::<TestType>();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_node_id_of_different_types() {
        let id1 = NodeId::of::<TestType>();
        let id2 = NodeId::of::<OtherType>();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_node_id_from_name() {
        let id1 = NodeId::from_name("test");
        let id2 = NodeId::from_name("test");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_node_id_from_different_names() {
        let id1 = NodeId::from_name("test1");
        let id2 = NodeId::from_name("test2");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_node_id_as_u64() {
        let id = NodeId::from_name("test");
        let value = id.as_u64();
        assert!(value > 0);
    }

    #[test]
    fn test_node_id_display() {
        let id = NodeId::from_name("test");
        let display = format!("{}", id);
        assert!(display.starts_with("NodeId("));
    }
}
