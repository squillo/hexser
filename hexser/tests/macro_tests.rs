//! Integration tests for procedural macros.
//!
//! Tests derive macros by actually using them in compiled code.
//! This is the correct way to test proc macros since they require
//! the full compiler infrastructure to function properly.

#[cfg(feature = "macros")]
mod macro_tests {
  use hexser::prelude::*;
  use hexser_macros::HexAggregate;

  #[test]
  fn test_hex_aggregate_derive() {
    // Aggregate requires Entity trait
    #[derive(HexAggregate, Entity)]
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

    #[derive(Entity)]
    struct EntityItem {
      id: String,
    }

    // If this test compiles, macros work
    assert!(true);
  }
}
