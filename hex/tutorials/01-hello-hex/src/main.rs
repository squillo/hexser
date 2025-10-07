//! Tutorial 01: Hello Hex
//!
//! Your first hexagonal architecture component with automatic registration.

use hexser::prelude::*;

#[derive(HexDomain, Entity)]
struct Todo {
    id: String,
    title: String,
    done: bool,
}

fn main() {
    println!("🎓 Tutorial 01: Hello Hex\n");

    // The graph is automatically built at compile time!
    let graph = HexGraph::current();

    // Let's see what we have
    graph.pretty_print();

    println!("\n🎉 You just created your first hexagonal architecture!");
    println!("\n📚 What happened:");
    println!("  1. #[derive(HexDomain)] registered Todo in the Domain layer");
    println!("  2. #[derive(Entity)] implemented the Entity trait");
    println!("  3. Graph was built automatically at compile time");
    println!("  4. Zero boilerplate required!");

    println!("\n✨ Try next: Add more entities and see them appear!");
    println!("   Then move to Tutorial 02 to learn about Ports.");
}
