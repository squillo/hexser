# Macros

Hexer provides derive macros (via the hex_macros crate) to remove boilerplate and automatically register your components for analysis.

Available derives:

- HexDomain: Mark a struct or enum as part of the Domain layer
- HexPort: Mark a trait as a Port
- HexAdapter: Mark a struct/impl as an Adapter that implements one or more ports
- HexAggregate: Mark an aggregate root type
- Entity: Implement the Entity trait, enabling invariants
- HexRepository: Mark repository-style ports
- HexDirective: Mark directive (command) handlers for CQRS
- HexQuery: Mark query handlers for CQRS

Example usage:

```rust
use hexer::prelude::*;

#[derive(Entity, HexAggregate)]
struct Order {
    id: String,
    total_cents: i64,
}

#[derive(HexPort)]
trait OrderRepository: Repository<Order> {}

#[derive(HexAdapter)]
struct InMemoryOrders;

impl Repository<Order> for InMemoryOrders {}
impl OrderRepository for InMemoryOrders {}
```

The tests/macro_tests.rs demonstrates basic compile-time checks for these derives.
