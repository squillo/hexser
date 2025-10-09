# Hexser Workspace

__From the founders:__ Hi all, thank you for checking out Hexser.

We are the authors of [N Lang,](https://squillo.io/nlang), and we strongly believe that Rust's and N Lang's robust type systems and meaningful designs are the future of software engineering.  We wrote this library to allow people to create incredible applications in Rust quickly while also giving them best-in-class generative code support and good design.

"Good Design is Good Business" - Thomas J. Watson Jr., the former CEO of IBM.

Hexser allows you to build incredibly small and incredibly large Rust apps that fit perfectly into the [Squillo](https://squillo.io) ecosystem but also for professional teams. We are proud to Open-Sourcing this library under MIT or Apache and can't wait to see how you use it. 

Enjoy!

---

Build real-world, hexagonal Rust apps without the ceremony. Hexser is a pragmatic, zero‑boilerplate take on Hexagonal Architecture with a clear Domain–Ports–Adapters flow, fast iteration, automatically graphed inference, and copy‑pasteable examples.

---

## Crates

Main crates (jump in):
- [hexser](./hexser) — Core traits, types, errors, graph model, and opt‑in derive macros. Also available on crates.io: https://crates.io/crates/hexser
- [hexser_macros](./hexser_macros) — Procedural macros (feature = "macros") to reduce repetition.
- [hexser_potions](./hexser_potions) — “Potions”: small, mixable examples for common app operations (formerly “blueprints”).

## Documentation:
- hexser crate README: ./hexser/README.md
- hexser_potions crate README: ./hexser_potions/README.md
- hexser_macros crate README: ./hexser_macros/README.md

---

## Why Hexser?

Why teams choose Hexser:
- Zero boilerplate: write traits for your Ports and small impls for Adapters — that’s it.
- Query‑first repositories: lightweight domain filter/sort types keep adapters simple and testable.
- First‑class use‑cases: model application logic as Directives (clear, explicit, testable).
- Fast feedback: in‑memory adapters and ready‑to‑paste Potions get you shipping quickly.
- Opt‑in macros: reduce repetition without hiding control flow; nothing “magic” at runtime.

## Quick start

Quick start with hexser:

```toml
[dependencies]
hexser = { path = "./hexser", version = "0.4.4", features = ["macros"] }
```

```rust
use hexser::prelude::*;

// Domain entity
struct User { id: String }
impl Entity for User { type Id = String; }

// Repository port (query-oriented)
// Define lightweight filter/sort types in your domain
#[derive(Debug, Clone)]
enum UserFilter { ByEmail(String), ById(String), All }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserSortKey { Email }

trait UserRepository: QueryRepository<User> {}

// Usage (adapter decides how to match filters)
// let found = <YourRepo as QueryRepository<User>>::find_one(&repo, &UserFilter::ByEmail("a@b.com".into()))?;
```

## CloudEvents v1.0 Support

Hexser includes built-in support for CloudEvents v1.0 specification, enabling standards-compliant, transport-agnostic domain event publishing and consumption:

- **Standards-compliant**: Full CloudEvents v1.0 specification compliance
- **Transport-agnostic**: Works with HTTP, Kafka, AMQP, and other transports
- **Hexagonal design**: Clear separation between domain events, ports, and adapters
- **Zero dependencies**: Native implementation without external CloudEvents crate
- **CQRS integration**: Seamless integration with Directives (write) and Queries (read)

```rust
use hexser::prelude::*;
use hexser::ports::events::{CloudEventsEnvelope, EventPublisher};

// Define domain event
struct UserCreated {
    user_id: String,
    email: String,
}

impl DomainEvent for UserCreated {
    fn event_type(&self) -> &str { "com.example.user.created" }
    fn aggregate_id(&self) -> String { self.user_id.clone() }
}

// Wrap in CloudEvents envelope
let event = UserCreated {
    user_id: String::from("user-123"),
    email: String::from("user@example.com"),
};

let envelope = CloudEventsEnvelope::from_domain_event(
    String::from("evt-001"),
    String::from("/services/user-service"),
    event,
);

// Publish via any transport adapter
// publisher.publish(&envelope)?;
```

**Documentation**: See [hexser/docs/events.md](./hexser/docs/events.md) for comprehensive guide including:
- CloudEvents v1.0 attribute mappings (required: id, source, specversion, type)
- Transport bindings (HTTP binary/structured, Kafka, AMQP)
- CQRS integration patterns
- Security and reliability considerations

## Potions Ecosystem

Why “Potions”? They are lightweight, composable recipes you can pour into your app — perfect for experimenting, learning, and scaffolding.

Explore Potions (examples):

- Authentication: Minimal sign‑up flow with a Directive and an in‑memory repository.
- CRUD: Simple in‑memory Repository example, ready to paste.

Use directly from the workspace:

```toml
[dependencies]
hexser_potions = { path = "./hexser_potions", version = "0.4.4" }
```

Then in your code:

```rust
use hexser_potions::auth::{SignUpUser, InMemoryUserRepository, execute_signup};

let mut repo = InMemoryUserRepository::default();
let user = execute_signup(&mut repo, SignUpUser { email: "a@b.com".into() })?;
```

## Licensing
License: MIT or Apache‑2.0
