//! HexEntity trait for domain objects with unique identity.
//!
//! Entities are objects that have a unique identity that persists over time,
//! even if their attributes change. Two entities are considered equal if they
//! have the same identity, regardless of their attribute values.
//! Entities are mutable by nature as their state can change while maintaining identity.
//! The identity type is defined via an associated type for maximum flexibility.
//!
//! Revision History
//! - 2025-10-09T09:43:00Z @AI: Rename Entity to HexEntity for consistency.
//! - 2025-10-01T00:00:00Z @AI: Initial Entity trait definition with associated type Id.

/// Trait for domain entities with unique identity.
///
/// Entities have an identity that persists over time. Two entities with the same
/// identity are considered the same entity, even if their attributes differ.
///
/// # Example
///
/// ```rust
/// use hexser::domain::HexEntity;
///
/// struct User {
///     id: String,
///     email: String,
/// }
///
/// impl HexEntity for User {
///     type Id = String;
/// }
/// ```
pub trait HexEntity {
  /// The type used to uniquely identify this entity.
  type Id;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestUser {
    id: u64,
    name: String,
  }

  impl HexEntity for TestUser {
    type Id = u64;
  }

  #[test]
  fn test_entity_trait_compiles() {
    let user = TestUser {
      id: 1,
      name: String::from("Test"),
    };
    let _id_type: <TestUser as HexEntity>::Id = user.id;
  }
}
