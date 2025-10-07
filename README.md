# N Lang Hex Workspace

This repository contains the Hexser ecosystem — a pragmatic, zero‑boilerplate take on Hexagonal Architecture in Rust — plus companion crates, docs, and examples.

Workspace crates:
- hexser: Core crate with traits, types, errors, graph model, and optional derive macros.
- hexser_macros: Procedural macro crate (feature = "macros") powering derive convenience.
- hexser_potions: Potions — a set of small, copy‑friendly examples for common app operations (formerly “blueprints”).

Why “Potions”? They are lightweight, mixable recipes you can pour into your app — perfect for experimenting, learning, and scaffolding.

Quick start with hexser:

```toml
[dependencies]
hexser = { path = "./hex", version = "0.3.0", features = ["macros"] }
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

Explore Potions (examples):

- Authentication: Minimal sign‑up flow with a Directive and an in‑memory repository.
- CRUD: Simple in‑memory Repository example, ready to paste.

Use directly from the workspace:

```toml
[dependencies]
hexser_potions = { path = "./hex/hexser_potions", version = "0.3.0" }
```

Then in your code:

```rust
use hexser_potions::auth::{SignUpUser, InMemoryUserRepository, execute_signup};

let mut repo = InMemoryUserRepository::default();
let user = execute_signup(&mut repo, SignUpUser { email: "a@b.com".into() })?;
```

Documentation:
- hexser crate README: ./hex/README.md
- Concepts: ./hex/docs/src/core-concepts.md
- Architecture: ./hex/docs/src/architecture.md

License: MIT or Apache‑2.0
