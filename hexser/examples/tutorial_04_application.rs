//! Tutorial 04: Application Layer
//!
//! Demonstrates the application layer that orchestrates domain logic
//! and coordinates port interactions through directives and queries.
//!
//! Run with: cargo run --example tutorial_04_application
//!
//! Revision History
//! - 2026-11-17T18:11:31Z @AI: HEXSER-0.7 — clippy --all-targets clean: allow print at the top (a CLI example); list the active todos the query returns.
//! - 2026-11-17T16:59:59Z @AI: HEXSER-0.7 — the write side is an `Actor` (`&mut self`, so the handler really assigns ids) with a `HexRefusal` outcome, in place of the deprecated `DirectiveHandler`.
//! - 2025-10-07T11:55:00Z @AI: Align with v0.4 Repository by removing id-centric methods; no behavior change otherwise.

// why: an example is a CLI program that reports to stdout; clippy.toml allows print in CLI code.
#![allow(clippy::disallowed_macros)]

use hexser::prelude::*;

// Domain Layer
#[derive(HexDomain, HexEntity, Clone)]
struct Todo {
  id: String,
  title: String,
  done: bool,
}

// Port Layer
trait TodoRepository: Repository<Todo> {
  fn find_active(&self) -> HexResult<Vec<Todo>>;
}

// Adapter Layer
#[derive(HexAdapter)]
struct InMemoryTodoRepository {
  todos: Vec<Todo>,
}

impl InMemoryTodoRepository {
  fn new() -> Self {
    Self { todos: Vec::new() }
  }
}

impl Repository<Todo> for InMemoryTodoRepository {
  fn save(&mut self, todo: Todo) -> HexResult<()> {
    self.todos.push(todo);
    Ok(())
  }
}

impl TodoRepository for InMemoryTodoRepository {
  fn find_active(&self) -> HexResult<Vec<Todo>> {
    Ok(self.todos.iter().filter(|t| !t.done).cloned().collect())
  }
}

// Application Layer - NEW!
#[derive(HexDirective)]
struct CreateTodoDirective {
  title: String,
}

/// Why the handler can refuse a directive: a closed list, each with its guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, HexRefusal)]
enum CreateTodoRefusal {
  #[hex(next_step = "Give the todo a title")]
  EmptyTitle,
}

/// The single writer for todos: `&mut self`, so it can assign ids without a lock or a `Cell`.
#[derive(HexActor)]
struct CreateTodoHandler {
  next_id: u32,
}

impl CreateTodoHandler {
  fn new() -> Self {
    Self { next_id: 1 }
  }
}

impl hexser::application::Actor for CreateTodoHandler {
  type Command = CreateTodoDirective;
  type Context<'a> = ();
  /// The id assigned, or why not.
  type Outcome = Result<u32, CreateTodoRefusal>;

  fn handle(&mut self, directive: CreateTodoDirective, _context: ()) -> Self::Outcome {
    if directive.title.trim().is_empty() {
      return Err(CreateTodoRefusal::EmptyTitle);
    }

    let id = self.next_id;
    self.next_id += 1;
    println!("Creating todo: {}", directive.title);
    println!("  Assigned ID: {id}");
    Ok(id)
  }
}

#[derive(HexQuery)]
struct ListActiveTodosQuery;

struct ListActiveTodosHandler<'a> {
  repository: &'a InMemoryTodoRepository,
}

impl<'a> QueryHandler<ListActiveTodosQuery, Vec<Todo>> for ListActiveTodosHandler<'a> {
  fn handle(&self, _query: ListActiveTodosQuery) -> HexResult<Vec<Todo>> {
    self.repository.find_active()
  }
}

fn main() {
  println!("Tutorial 04: Application Layer\n");
  println!("{}", "=".repeat(50));

  let mut handler = CreateTodoHandler::new();

  let directive = CreateTodoDirective {
    title: String::from("Complete tutorial series"),
  };

  println!("\nExecuting directive:");
  hexser::application::Actor::handle(&mut handler, directive, ()).unwrap();

  // A refusal is a value; it becomes a `Hexserror` only at the edge, and its code reads back.
  let refused = hexser::application::Actor::handle(
    &mut handler,
    CreateTodoDirective {
      title: String::from("  "),
    },
    (),
  )
  .unwrap_err();
  let at_the_edge = Hexserror::refused(&refused, "A todo needs a title");
  println!("  Refused: {at_the_edge} (code {})", at_the_edge.code());

  let repo = InMemoryTodoRepository::new();
  let query_handler = ListActiveTodosHandler { repository: &repo };

  println!("\nExecuting query:");
  let active = query_handler.handle(ListActiveTodosQuery).unwrap();
  println!("  Found {} active todos", active.len());
  for todo in &active {
    println!("    {}: {}", todo.id, todo.title);
  }

  let graph = HexGraph::current();

  println!("\nComplete hexagonal architecture:");
  graph.pretty_print();

  println!("\nArchitecture summary:");
  println!(
    "  Domain: {} components",
    graph.nodes_by_layer(Layer::Domain).len()
  );
  println!(
    "  Ports: {} components",
    graph.nodes_by_layer(Layer::Port).len()
  );
  println!(
    "  Adapters: {} components",
    graph.nodes_by_layer(Layer::Adapter).len()
  );
  println!(
    "  Application: {} components",
    graph.nodes_by_layer(Layer::Application).len()
  );

  println!("\nKey concepts:");
  println!("  An Actor is the single writer: it decides each directive (CQRS)");
  println!("  Queries handle read operations");
  println!("  Application layer orchestrates domain and ports");
  println!("  Business logic stays in domain layer");

  println!("\nYou've built a complete hexagonal application!");
  println!("{}", "=".repeat(50));

  println!("\nNext steps:");
  println!("  Check architecture_visualization.rs for advanced features");
  println!("  Review cqrs_pattern.rs for CQRS best practices");
  println!("  See ../docs/events.md for CloudEvents v1.0 event publishing");
  println!("  Learn how Directives can emit DomainEvents for event-driven CQRS");
}
