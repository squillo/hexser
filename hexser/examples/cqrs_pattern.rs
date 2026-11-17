//! CQRS pattern example with hex.
//!
//! This example demonstrates Command Query Responsibility Segregation (CQRS)
//! with hexser's `Actor` (the single writer) and `QueryHandler`. It shows how to
//! separate write operations (directives) from read operations (queries).
//!
//! Run with: `cargo run --example cqrs_pattern`
//!
//! Revision History
//! - 2026-11-17T18:11:31Z @AI: HEXSER-0.7 — clippy --all-targets clean: allow print at the top (a CLI example); print the saved user's id and the found view's id.
//! - 2026-11-17T16:59:59Z @AI: HEXSER-0.7 — the write side is an `Actor` that owns its repository (`&mut self`) and refuses with a `HexRefusal`, in place of the deprecated `DirectiveHandler`, whose `&self` left the save unwritten.
//! - 2025-10-07T13:02:00Z @AI: Migrate to v0.4 Repository by removing id-centric methods; no behavior change.

// why: an example is a CLI program that reports to stdout; clippy.toml allows print in CLI code.
#![allow(clippy::disallowed_macros)]

use hexser::QueryHandler;

fn main() -> hexser::HexResult<()> {
  println!("=== CQRS Pattern Example ===\n");

  // Create repositories
  let write_repo = InMemoryUserRepository::new();
  let query_repo = InMemoryUserQueryRepository::new();

  // Write side: the single writer owns its repository and decides each directive.
  let mut writer = UserWriter {
    repository: write_repo,
    next_id: 1,
  };

  let create_directive = CreateUserDirective {
    email: String::from("alice@example.com"),
    name: String::from("Alice"),
  };

  println!("Creating user via directive...");
  let created = hexser::application::Actor::handle(&mut writer, create_directive, ())?;
  println!("✓ User created: id {}", created.id);

  // A refused directive: the code reads back to the closed refusal.
  let refused = hexser::application::Actor::handle(
    &mut writer,
    CreateUserDirective {
      email: String::from("bob"),
      name: String::from("Bob"),
    },
    (),
  )
  .unwrap_err();
  println!("✓ Refused: {refused}");
  assert_eq!(
    CreateUserRefusal::from_code(refused.code()),
    Some(CreateUserRefusal::InvalidEmail)
  );
  assert_eq!(writer.repository.users.len(), 1);

  // Read side: Query user
  let query_handler = FindUserByEmailHandler {
    repository: &query_repo,
  };

  let find_query = FindUserByEmailQuery {
    email: String::from("alice@example.com"),
  };

  println!("\nQuerying user...");
  if let Some(user_view) = query_handler.handle(find_query)? {
    println!(
      "✓ Found user: {} ({}), id {}",
      user_view.name, user_view.email, user_view.id
    );
  }

  println!("\n✅ CQRS example completed!");

  Ok(())
}

// Domain: User entity
#[derive(Clone)]
struct User {
  id: String,
  email: String,
  name: String,
}

impl hexser::domain::HexEntity for User {
  type Id = String;
}

// Write side: Directive
struct CreateUserDirective {
  email: String,
  name: String,
}

/// Why a create is refused: a closed list, each with its guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum CreateUserRefusal {
  #[hex(next_step = "Give an email address that contains '@'")]
  InvalidEmail,
  #[hex(next_step = "Give a name")]
  EmptyName,
}

// Write side: the event the writer decided.
#[derive(Debug)]
struct UserCreated {
  id: String,
}

// Write side: the single writer. `&mut self` lets it save without interior mutability.
struct UserWriter {
  repository: InMemoryUserRepository,
  next_id: u64,
}

impl hexser::application::Actor for UserWriter {
  type Command = CreateUserDirective;
  type Context<'a> = ();
  type Outcome = hexser::HexResult<UserCreated>;

  fn handle(&mut self, directive: CreateUserDirective, _context: ()) -> Self::Outcome {
    if !directive.email.contains('@') {
      return Err(hexser::Hexserror::refused(
        &CreateUserRefusal::InvalidEmail,
        "Invalid email format",
      ));
    }
    if directive.name.is_empty() {
      return Err(hexser::Hexserror::refused(
        &CreateUserRefusal::EmptyName,
        "Name cannot be empty",
      ));
    }

    let id = self.next_id.to_string();
    self.next_id += 1;
    let user = User {
      id: id.clone(),
      email: directive.email,
      name: directive.name,
    };
    println!("Saving user {}: {} <{}>", user.id, user.name, user.email);
    hexser::ports::Repository::save(&mut self.repository, user)?;
    Ok(UserCreated { id })
  }
}

// Read side: Query
struct FindUserByEmailQuery {
  email: String,
}

// Read side: View model
#[derive(Clone)]
struct UserView {
  id: String,
  email: String,
  name: String,
}

// Read side: Query handler
struct FindUserByEmailHandler<'a> {
  repository: &'a InMemoryUserQueryRepository,
}

impl<'a> hexser::application::QueryHandler<FindUserByEmailQuery, Option<UserView>>
  for FindUserByEmailHandler<'a>
{
  fn handle(&self, query: FindUserByEmailQuery) -> hexser::HexResult<Option<UserView>> {
    self.repository.find_by_email(&query.email)
  }
}

// Write repository (for commands)
struct InMemoryUserRepository {
  users: Vec<User>,
}

impl InMemoryUserRepository {
  fn new() -> Self {
    Self { users: Vec::new() }
  }
}

impl hexser::adapters::Adapter for InMemoryUserRepository {}

impl hexser::ports::Repository<User> for InMemoryUserRepository {
  fn save(&mut self, user: User) -> hexser::HexResult<()> {
    self.users.push(user);
    Ok(())
  }
}

// Read repository (for queries)
struct InMemoryUserQueryRepository {
  users: Vec<UserView>,
}

impl InMemoryUserQueryRepository {
  fn new() -> Self {
    // Preload with sample data
    Self {
      users: vec![UserView {
        id: String::from("1"),
        email: String::from("alice@example.com"),
        name: String::from("Alice"),
      }],
    }
  }

  fn find_by_email(&self, email: &str) -> hexser::HexResult<Option<UserView>> {
    Ok(self.users.iter().find(|u| u.email == email).cloned())
  }
}
