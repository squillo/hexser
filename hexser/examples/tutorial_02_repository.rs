//! Tutorial 02: Repository Port
//!
//! Learn to define ports (interfaces) for persistence.
//! Time: 10 minutes
//!
//! Run with: `cargo run --example tutorial_02_repository`
//!
//! Revision History
//! - 2026-11-17T18:11:31Z @AI: HEXSER-0.7 — clippy --all-targets clean: allow print at the top (a CLI example); `#[allow(dead_code)]` on the defined-not-used `User` and `UserRepository`.

// why: an example is a CLI program that reports to stdout; clippy.toml allows print in CLI code.
#![allow(clippy::disallowed_macros)]

fn main() -> hexser::HexResult<()> {
  println!("=== Tutorial 02: Repository Port ===\n");

  // From Tutorial 01: Our domain entity
  // why: the lesson is the port's shape; this tutorial defines the entity, it does not build one.
  #[allow(dead_code)]
  #[derive(Clone, Debug)]
  struct User {
    id: String,
    email: String,
    name: String,
  }

  impl hexser::domain::HexEntity for User {
    type Id = String;
  }

  // NEW: Define a Port (interface) for persistence
  // Ports define WHAT we need, not HOW to do it
  // why: the port is the lesson; Tutorial 03 implements it, so nothing here calls it.
  #[allow(dead_code)]
  trait UserRepository: hexser::ports::Repository<User> {
    fn find_by_email(&self, email: &str) -> hexser::HexResult<Option<User>>;
    fn count(&self) -> hexser::HexResult<usize>;
  }

  println!("✅ Defined UserRepository port");
  println!("   - Extends base Repository<User> trait");
  println!("   - Adds custom queries: find_by_email, count");
  println!("   - Lives in the Port layer (interfaces only!)");

  println!("\n🎯 Architecture so far:");
  println!("   ┌─────────────────┐");
  println!("   │     Domain      │  ← User entity");
  println!("   │     [User]      │");
  println!("   └────────┬────────┘");
  println!("            │ depends on");
  println!("            ↓");
  println!("   ┌─────────────────┐");
  println!("   │      Port       │  ← UserRepository interface");
  println!("   │ [UserRepository]│");
  println!("   └─────────────────┘");

  println!("\n📚 What you learned:");
  println!("   - Ports define interfaces (the WHAT)");
  println!("   - Domain depends on ports, not implementations");
  println!("   - Custom methods extend base functionality");

  println!("\n💡 Why this matters:");
  println!("   - You can now implement this with ANY database");
  println!("   - Domain code never changes when switching databases");
  println!("   - Easy to mock for testing");

  println!("\n✅ Tutorial 02 Complete!");
  println!("Next: `cargo run --example tutorial_03_adapter`");

  Ok(())
}
