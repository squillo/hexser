//! Tutorial 02: Adding Ports
//!
//! Demonstrates how to define port interfaces that separate
//! domain logic from infrastructure concerns.
//!
//! Run with: cargo run --example tutorial_02_ports

use hexser::prelude::*;

// Domain Layer - from Tutorial 01
#[derive(HexDomain, Entity)]
struct Todo {
  id: String,
  title: String,
  done: bool,
}

// Port Layer - NEW!
#[derive(HexPort, HexRepository)]
struct TodoRepository;

fn main() {
  println!("🎓 Tutorial 02: Adding Ports\n");
  println!("{}", "=".repeat(50));

  let graph = HexGraph::current();

  println!("\n✅ Both domain and port layers registered!");
  graph.pretty_print();

  println!("\n📊 Architecture layers:");
  println!(
    "  Domain: {} components",
    graph.nodes_by_layer(Layer::Domain).len()
  );
  println!(
    "  Ports: {} components",
    graph.nodes_by_layer(Layer::Port).len()
  );

  println!("\n💡 Key concept:");
  println!("  Ports define WHAT your application needs");
  println!("  Adapters (next tutorial) define HOW it works");

  println!("\n🎉 You've separated domain from infrastructure!");
  println!("{}", "=".repeat(50));

  println!("\nNext: Try Tutorial 03 - Implementing Adapters");
  println!("  cargo run --example tutorial_03_adapters");
}
