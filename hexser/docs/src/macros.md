# Macros

Hexser provides derive macros (via the hexser_macros crate) to remove boilerplate and automatically register your components for analysis.

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
use hexser::prelude::*;

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

## Roles: `#[hex(role = "…")]`

The five **registration** derives (`HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`,
`HexQuery`) each emit `impl Registrable` plus an `inventory::submit!`, and each honours
`#[hex(role = "…")]` with any `Role` variant. Only the default differs: `Entity`,
`Repository`, `Adapter`, `Directive`, `Query` respectively.

```rust
#[derive(HexDomain)]
#[hex(role = "ValueObject")]   // a domain layer is not made only of entities
struct Email(String);
```

⚠ Until 2026-09-04 `HexDomain` declared the attribute and ignored it, so a type marked
`ValueObject` registered as `Role::Entity` and compiled clean.

## Generics: a registration derive on a generic type is a compile error

`inventory` submits one entry per component and `type_name::<Self>()` on a generic names a
monomorphization chosen by a consumer crate, so there is no single honest node for `Foo<T>`.
The derive says so. Derive on a non-generic marker struct that stands for the component, or
hand-write `impl Registrable` when you need `node_info()` on the generic itself.

⚠ Until 2026-09-04 this compiled and silently skipped the submission: the type implemented
`Registrable`, answered `node_info()`, and appeared in no graph query.

## Registering without a derive

`hexser::hex_register_component!` and its per-layer wrappers (`hex_register_domain!`,
`hex_register_port!`, `hex_register_adapter!`, `hex_register_application!`,
`hex_register_infrastructure!`) emit the same impl + submission pair for a type you cannot
annotate, or in a build without the `macros` feature. They too emitted no submission until
2026-09-04.
