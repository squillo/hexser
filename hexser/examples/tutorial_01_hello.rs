//! Tutorial 01: Hello Hex
//!
//! This is the complete code from Tutorial 01 that demonstrates
//! the basics of hex with automatic component registration.
//!
//! Run with: cargo run --example tutorial_01_hello
//!
//! Revision History
//! - 2026-11-17T18:11:31Z @AI: HEXSER-0.7 — clippy --all-targets clean: allow print at the top (a CLI example); `#[allow(dead_code)]` on the registered-only entity.

// why: an example is a CLI program that reports to stdout; clippy.toml allows print in CLI code.
#![allow(clippy::disallowed_macros)]

use hexser::prelude::*;

// why: illustrative fields in a teaching example; the type is only registered, never built.
#[allow(dead_code)]
#[derive(HexDomain, HexEntity)]
struct TodoMore {
  id: String,
  title: String,
  done: bool,
}

fn main() {
  println!("🎓 Tutorial 01: Hello Hex\n");
  println!("{}", "=".repeat(50));

  let graph = HexGraph::current();

  println!("\n✅ Component registered automatically!");
  graph.pretty_print();

  println!("\n📊 What we created:");
  println!("  - 1 Domain entity (Todo)");
  println!("  - Automatic registration via derive macro");
  println!("  - Zero boilerplate!");

  println!("\n🎉 You just created your first hexagonal architecture!");
  println!("{}", "=".repeat(50));

  println!("\nNext: Try Tutorial 02 - Adding Ports");
  println!("  cargo run --example tutorial_02_ports");
}
