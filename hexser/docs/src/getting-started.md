# Getting Started

This chapter helps you add hexser to a project and build your first hexagonal component.

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
hexser = { version = "0.1.0", features = ["macros"] }
```

Enable the macros feature to use the derive macros provided by the companion crate `hexser_macros`.

## Your first components

The goal: mark your domain types, define ports (traits), and implement adapters.

```rust
use hexser::prelude::*;

#[derive(Entity, HexDomain)]
struct User {
    id: String,
    email: String,
}

// A port is a trait describing what the domain needs
#[derive(HexPort)]
trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

// An adapter implements one or more ports
#[derive(HexAdapter)]
struct InMemoryUserRepository {
    users: Vec<User>,
}

impl Repository<User> for InMemoryUserRepository {
    fn find_by_id(&self, id: &String) -> HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| &u.id == id).cloned())
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| u.email == email).cloned())
    }
}
```

If this compiles, the macros worked. For a minimal compile test, see tests/macro_tests.rs.

## Next steps

- Read Core Concepts to understand layers
- Explore Tutorials for step-by-step projects
- See Architecture for the big picture and design rationale
