//! Manual verification that Serde works for rich errors.
//!
//! This example demonstrates that all error types can be serialized
//! and deserialized using Serde when the serde feature is enabled.

#[cfg(feature = "serde")]
fn main() {
  use hexser::error::*;

  // Test SourceLocation
  let loc = source_location::SourceLocation::new("test.rs", 42, 10);
  let json = serde_json::to_string_pretty(&loc).unwrap();
  println!("SourceLocation JSON:\n{}\n", json);
  let _deserialized: source_location::SourceLocation = serde_json::from_str(&json).unwrap();
  println!("✓ SourceLocation serialization works\n");

  // Test ValidationError
  let err =
    validation_error::ValidationError::new("E_VAL_001", "Invalid email format").with_field("email");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("ValidationError JSON:\n{}\n", json);
  let _deserialized: validation_error::ValidationError = serde_json::from_str(&json).unwrap();
  println!("✓ ValidationError serialization works\n");

  // Test NotFoundError
  let err = not_found_error::NotFoundError::new("User", "user-123");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("NotFoundError JSON:\n{}\n", json);
  let _deserialized: not_found_error::NotFoundError = serde_json::from_str(&json).unwrap();
  println!("✓ NotFoundError serialization works\n");

  // Test ConflictError
  let err = conflict_error::ConflictError::new("Email already exists").with_existing_id("user-456");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("ConflictError JSON:\n{}\n", json);
  let _deserialized: conflict_error::ConflictError = serde_json::from_str(&json).unwrap();
  println!("✓ ConflictError serialization works\n");

  // Test DomainError (LayerError<DomainLayer>)
  let err = domain_error::DomainError::new("E_DOM_001", "Order cannot be empty")
    .with_next_step("Add at least one item")
    .with_suggestion("order.add_item(item)");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("DomainError JSON:\n{}\n", json);
  let _deserialized: domain_error::DomainError = serde_json::from_str(&json).unwrap();
  println!("✓ DomainError serialization works\n");

  // Test Hexserror enum with different variants
  let err = hex_error::Hexserror::validation("Invalid input data");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("Hexserror::Validation JSON:\n{}\n", json);
  let _deserialized: hex_error::Hexserror = serde_json::from_str(&json).unwrap();
  println!("✓ Hexserror::Validation serialization works\n");

  let err = hex_error::Hexserror::not_found("Order", "order-789");
  let json = serde_json::to_string_pretty(&err).unwrap();
  println!("Hexserror::NotFound JSON:\n{}\n", json);
  let _deserialized: hex_error::Hexserror = serde_json::from_str(&json).unwrap();
  println!("✓ Hexserror::NotFound serialization works\n");

  println!("==========================================");
  println!("✓ All rich error types support Serde!");
  println!("==========================================");
}

#[cfg(not(feature = "serde"))]
fn main() {
  println!("This example requires the 'serde' feature to be enabled.");
  println!("Run with: cargo run --example test_serde_errors --features serde");
}
