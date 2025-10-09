//! Integration tests for procedural macros.
//!
//! Tests derive macros by actually using them in compiled code.
//! This is the correct way to test proc macros since they require
//! the full compiler infrastructure to function properly.

#[cfg(feature = "macros")]
mod macro_tests {
  use hexser::prelude::*;

  #[test]
  fn test_hex_aggregate_derive() {
    // Aggregate requires Entity trait
    #[derive(HexAggregate, HexEntity)]
    struct TestAggregate {
      id: String,
      value: i32,
    }

    let agg = TestAggregate {
      id: String::from("test"),
      value: 42,
    };

    // Should compile and have default implementation
    assert!(agg.check_invariants().is_ok());
  }

  #[test]
  fn test_hex_domain_derive() {
    #[derive(HexDomain)]
    struct TestDomain {
      name: String,
    }

    // Should compile with Registrable trait
    let info = TestDomain::node_info();
    assert_eq!(info.layer, hexser::graph::Layer::Domain);
  }

  #[test]
  fn test_hex_adapter_derive() {
    #[derive(HexAdapter)]
    struct TestAdapter;

    // Should compile with registration
    let info = TestAdapter::node_info();
    assert_eq!(info.layer, hexser::graph::Layer::Adapter);
  }

  #[test]
  fn test_derive_macros_compile() {
    // Simple compilation test for all derive macros
    #[derive(HexDomain)]
    struct DomainItem;

    #[derive(HexAdapter)]
    struct AdapterItem;

    #[derive(HexEntity)]
    struct EntityItem {
      id: String,
    }

    // If this test compiles, macros work
    assert!(true);
  }

  #[test]
  fn test_hex_entity_derive_alias() {
    // Test that #[derive(HexEntity)] works as an alias to Entity
    #[derive(HexEntity)]
    struct User {
      id: String,
      email: String,
    }

    let user = User {
      id: String::from("user-001"),
      email: String::from("test@example.com"),
    };

    // Verify the HexEntity trait is implemented
    // The associated type Id should be String (detected from id field)
    let _id_type: <User as hexser::HexEntity>::Id = user.id;
    assert!(true);
  }

  #[test]
  fn test_hex_entity_derive_with_custom_id_type() {
    // Test that HexEntity correctly detects custom id types
    #[derive(HexEntity)]
    struct Order {
      id: u64,
      total: f64,
    }

    let order = Order {
      id: 12345,
      total: 99.99,
    };

    // Verify the associated type is u64
    let id: <Order as hexser::domain::HexEntity>::Id = order.id;
    assert_eq!(id, 12345);
  }

  #[test]
  fn test_hex_value_item_derive_default_validation() {
    // Test that #[derive(HexValueItem)] provides default validation
    #[derive(HexValueItem, Clone)]
    struct Email(String);

    let email = Email(String::from("test@example.com"));

    // Default validation should return Ok(())
    assert!(email.validate().is_ok());
  }

  #[test]
  fn test_hex_value_item_derive_custom_validation() {
    // Test that HexValueItem can be overridden with custom validation
    #[derive(Clone)]
    struct ValidatedEmail(String);

    // Derive provides default, but we override it
    impl hexser::domain::HexValueItem for ValidatedEmail {
      fn validate(&self) -> hexser::HexResult<()> {
        if self.0.contains('@') {
          Ok(())
        } else {
          Err(hexser::Hexserror::validation("Email must contain @"))
        }
      }
    }

    let valid = ValidatedEmail(String::from("test@example.com"));
    assert!(valid.validate().is_ok());

    let invalid = ValidatedEmail(String::from("invalid"));
    assert!(invalid.validate().is_err());
  }

  #[test]
  fn test_hex_entity_works_with_qualified_path() {
    // Test that HexEntity works and can be used with qualified paths
    #[derive(hexser::HexEntity)]
    struct QualifiedEntity {
      id: String,
    }

    #[derive(HexEntity)]
    struct PreludeEntity {
      id: String,
    }

    let e1 = QualifiedEntity {
      id: String::from("1"),
    };
    let e2 = PreludeEntity {
      id: String::from("2"),
    };

    // Both should have HexEntity trait implemented
    let _: <QualifiedEntity as hexser::domain::HexEntity>::Id = e1.id;
    let _: <PreludeEntity as hexser::domain::HexEntity>::Id = e2.id;

    assert!(true);
  }
}
