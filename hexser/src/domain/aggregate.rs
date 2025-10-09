//! Aggregate trait for consistency boundaries in the domain.
//!
//! An aggregate is a cluster of domain objects (entities and value objects) that
//! can be treated as a single unit for data changes. Each aggregate has a root entity
//! (the aggregate root) and a boundary that defines what is inside the aggregate.
//! External objects can only reference the aggregate root, ensuring consistency.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial Aggregate trait definition extending Entity.

/// Trait for aggregate roots that define transactional consistency boundaries.
///
/// Aggregates ensure that business rules and invariants are consistently enforced
/// within a transactional boundary. Only the aggregate root can be referenced
/// from outside the aggregate.
///
/// # Example
///
/// ```rust
/// use hexser::domain::{Entity, Aggregate};
/// use hexser::HexResult;
///
/// struct Order {
///     id: String,
///     items: Vec<OrderItem>,
/// }
///
/// struct OrderItem {
///     product_id: String,
///     quantity: u32,
/// }
///
/// impl Entity for Order {
///     type Id = String;
/// }
///
/// impl Aggregate for Order {
///     fn check_invariants(&self) -> HexResult<()> {
///         if self.items.is_empty() {
///             Err(hexser::Hexserror::domain("E_HEX_001", "Order must have items"))
///         } else {
///             Ok(())
///         }
///     }
/// }
/// ```
pub trait Aggregate: crate::domain::entity::Entity {
  /// Check the aggregate's invariants.
  ///
  /// This method should validate that all business rules and invariants
  /// within the aggregate boundary are satisfied.
  fn check_invariants(&self) -> crate::result::hex_result::HexResult<()>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestOrder {
    id: u64,
    item_count: usize,
  }

  impl crate::domain::entity::Entity for TestOrder {
    type Id = u64;
  }

  impl Aggregate for TestOrder {
    fn check_invariants(&self) -> crate::result::hex_result::HexResult<()> {
      if self.item_count > 0 {
        Result::Ok(())
      } else {
        Result::Err(
          crate::error::hex_error::Hexserror::domain("E_HEX_001", "Order must have items")
            .with_next_step("Add at least one item")
            .with_suggestion("order.add_item(item)"),
        )
      }
    }
  }

  #[test]
  fn test_aggregate_invariants_valid() {
    let order = TestOrder {
      id: 1,
      item_count: 5,
    };
    assert!(order.check_invariants().is_ok());
  }

  #[test]
  fn test_aggregate_invariants_invalid() {
    let order = TestOrder {
      id: 1,
      item_count: 0,
    };
    assert!(order.check_invariants().is_err());
  }
}
