//! Compile-and-run verification of the README / crate-doc "Quick Start" example.
//!
//! The Quick Start is the first code a crates.io visitor copies, so it must compile against the
//! real API. Keeping a copy here as a test means drift becomes a build failure (the README
//! itself is not doctested). If you edit the Quick Start in README.md or lib.rs, update this to
//! match.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: Add as a guard for the rewritten Quick Start (was using the removed Entity derive and an illegal derive-on-trait).

#![cfg(feature = "macros")]

use hexser::prelude::*;

#[derive(Clone, HexEntity, HexDomain)]
struct User {
  id: String,
  email: String,
  name: String,
}

trait UserRepository: Repository<User> {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

#[derive(HexAdapter)]
struct InMemoryUserRepository {
  users: Vec<User>,
}

impl Repository<User> for InMemoryUserRepository {
  fn save(&mut self, user: User) -> HexResult<()> {
    if let Some(existing) = self.users.iter_mut().find(|u| u.id == user.id) {
      *existing = user;
    } else {
      self.users.push(user);
    }
    Ok(())
  }
}

impl UserRepository for InMemoryUserRepository {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
    Ok(self.users.iter().find(|u| u.email == email).cloned())
  }
}

/// why: guards that the published Quick Start compiles and runs against the real API — a
/// non-compiling landing-page example is the highest-impact usability defect for a new user.
#[test]
fn readme_quickstart_runs() {
  let mut repo = InMemoryUserRepository { users: Vec::new() };
  repo
    .save(User {
      id: String::from("1"),
      email: String::from("alice@example.com"),
      name: String::from("Alice"),
    })
    .unwrap();

  let found = repo.find_by_email("alice@example.com").unwrap();
  assert_eq!(found.map(|u| u.name), Some(String::from("Alice")));
}
