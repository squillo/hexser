//! Tutorial 03: Implementing Adapters
//!
//! Demonstrates how to implement adapters that provide concrete
//! implementations for ports using specific technologies.
//!
//! Run with: cargo run --example tutorial_03_adapters
//!
//! Revision History
//! - 2025-10-07T11:54:00Z @AI: Migrate to v0.4 QueryRepository API; remove id-centric Repository methods; update example usage.

use hexser::prelude::*;

// Domain Layer - from Tutorial 01
#[derive(HexDomain, Entity, Clone)]
struct Todo {
  id: String,
  title: String,
  done: bool,
}

// Port Layer - from Tutorial 02
trait TodoRepository: Repository<Todo> {
  fn find_active(&self) -> HexResult<Vec<Todo>>;
  fn count_completed(&self) -> HexResult<usize>;
}

// Adapter Layer - NEW!
#[derive(HexAdapter)]
struct InMemoryTodoRepository {
  todos: Vec<Todo>,
}

impl InMemoryTodoRepository {
  fn new() -> Self {
    Self { todos: Vec::new() }
  }

  fn with_todos(todos: Vec<Todo>) -> Self {
    Self { todos }
  }
}

impl Repository<Todo> for InMemoryTodoRepository {
  fn save(&mut self, todo: Todo) -> HexResult<()> {
    if let Some(existing) = self.todos.iter_mut().find(|t| t.id == todo.id) {
      *existing = todo;
    } else {
      self.todos.push(todo);
    }
    Ok(())
  }
}

#[derive(Clone)]
enum TodoFilter {
  All,
  ById(String),
}
#[derive(Clone, Copy)]
enum TodoSortKey {
  Id,
}

impl hexser::ports::repository::QueryRepository<Todo> for InMemoryTodoRepository {
  type Filter = TodoFilter;
  type SortKey = TodoSortKey;

  fn find_one(&self, filter: &TodoFilter) -> HexResult<Option<Todo>> {
    let found = match filter {
      TodoFilter::All => self.todos.first().cloned(),
      TodoFilter::ById(id) => self.todos.iter().find(|t| &t.id == id).cloned(),
    };
    Ok(found)
  }

  fn find(
    &self,
    filter: &TodoFilter,
    _opts: hexser::ports::repository::FindOptions<TodoSortKey>,
  ) -> HexResult<Vec<Todo>> {
    let items: Vec<Todo> = match filter {
      TodoFilter::All => self.todos.clone(),
      TodoFilter::ById(id) => self.todos.iter().filter(|t| &t.id == id).cloned().collect(),
    };
    Ok(items)
  }

  fn delete_where(&mut self, filter: &TodoFilter) -> HexResult<u64> {
    let before = self.todos.len();
    match filter {
      TodoFilter::All => self.todos.clear(),
      TodoFilter::ById(id) => self.todos.retain(|t| &t.id != id),
    }
    Ok((before.saturating_sub(self.todos.len())) as u64)
  }
}

impl TodoRepository for InMemoryTodoRepository {
  fn find_active(&self) -> HexResult<Vec<Todo>> {
    Ok(self.todos.iter().filter(|t| !t.done).cloned().collect())
  }

  fn count_completed(&self) -> HexResult<usize> {
    Ok(self.todos.iter().filter(|t| t.done).count())
  }
}

fn main() {
  println!("Tutorial 03: Implementing Adapters\n");
  println!("{}", "=".repeat(50));

  let repo = InMemoryTodoRepository::with_todos(vec![
    Todo {
      id: String::from("1"),
      title: String::from("Learn hexagonal architecture"),
      done: false,
    },
    Todo {
      id: String::from("2"),
      title: String::from("Read documentation"),
      done: true,
    },
  ]);

  println!("\nUsing adapter methods:");
  let total = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find(
    &repo,
    &TodoFilter::All,
    hexser::ports::repository::FindOptions::default(),
  )
  .unwrap()
  .len();
  println!("  Total todos: {}", total);
  println!("  Active todos: {}", repo.find_active().unwrap().len());
  println!("  Completed: {}", repo.count_completed().unwrap());

  let graph = HexGraph::current();

  println!("\nAll three layers registered:");
  graph.pretty_print();

  println!("\nArchitecture layers:");
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

  println!("\nKey concepts:");
  println!("  Adapters implement ports using specific technology");
  println!("  InMemoryTodoRepository uses Vec (in-memory storage)");
  println!("  Could swap with PostgresAdapter, SqliteAdapter, etc.");
  println!("  Domain and ports remain unchanged");

  println!("\nYou've completed the core hexagonal pattern!");
  println!("{}", "=".repeat(50));

  println!("\nNext: Try Tutorial 04 - Application Layer");
  println!("  cargo run --example tutorial_04_application");
}
