//! Tutorial 02: Adding Ports
//!
//! Learn to define port interfaces that separate domain from infrastructure.

use hexser::prelude::*;

#[derive(HexDomain, Entity, Clone)]
struct User {
    id: String,
    email: String,
    name: String,
}

trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
    fn count(&self) -> HexResult<usize>;
}

fn main() {
    println!("Tutorial 02: Adding Ports\n");
    println!("{}", "=".repeat(50));

    let graph = HexGraph::current();

    println!("\nRegistered components:");
    graph.pretty_print();

    println!("\nArchitecture layers:");
    println!("  Domain: {} components",
        graph.nodes_by_layer(Layer::Domain).len());
    println!("  Ports: {} components",
        graph.nodes_by_layer(Layer::Port).len());

    println!("\nWhat we built:");
    println!("  - User entity (Domain layer)");
    println!("  - UserRepository port (Port layer)");
    println!("  - Custom query methods (find_by_email, count)");

    println!("\nKey insight:");
    println!("  Ports are interfaces. They define WHAT, not HOW.");
    println!("  Adapters (next tutorial) provide the HOW.");

    println!("\n{}", "=".repeat(50));
    println!("Next: Tutorial 03 - Implementing Adapters");
}
