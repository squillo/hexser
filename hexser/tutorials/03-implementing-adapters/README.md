# Tutorial 03: Implementing Adapters (15 minutes)

## Goal
Create concrete adapter implementations for ports using specific technologies.

## Key Concepts
- Adapters implement ports using real technologies
- Multiple implementations can coexist
- Dependency injection selects adapter at runtime
- Swap implementations without changing domain

## Prerequisites
- Completed Tutorials 01–02

---

## v0.4 Note: Filter-based reads with QueryRepository
As of v0.4, repositories focus on saving aggregates. Reads and deletions-by-criteria use the generic QueryRepository trait with domain-owned filters. See a complete, runnable example in:
- examples/tutorial_03_adapters.rs

Minimal snippet using the fully qualified paths (no `use` statements):

```rust
// Domain-owned types
#[derive(Clone)]
struct Todo { id: String, title: String, done: bool }
impl hexser::domain::Entity for Todo { type Id = String; }

#[derive(Clone)]
enum TodoFilter { All, ById(String) }
#[derive(Clone, Copy)]
enum TodoSortKey { Id }

// Adapter implements write via Repository and read via QueryRepository
struct InMemoryTodoRepository { todos: std::vec::Vec<Todo> }
impl hexser::ports::Repository<Todo> for InMemoryTodoRepository {
  fn save(&mut self, t: Todo) -> hexser::HexResult<()> { self.todos.push(t); Ok(()) }
}
impl hexser::ports::repository::QueryRepository<Todo> for InMemoryTodoRepository {
  type Filter = TodoFilter;
  type SortKey = TodoSortKey;
  fn find_one(&self, f: &TodoFilter) -> hexser::HexResult<std::option::Option<Todo>> {
    let found = match f {
      TodoFilter::All => self.todos.first().cloned(),
      TodoFilter::ById(id) => self.todos.iter().find(|t| &t.id == id).cloned(),
    }; Ok(found)
  }
  fn find(&self, f: &TodoFilter, _o: hexser::ports::repository::FindOptions<TodoSortKey>) -> hexser::HexResult<std::vec::Vec<Todo>> {
    let v = match f {
      TodoFilter::All => self.todos.clone(),
      TodoFilter::ById(id) => self.todos.iter().filter(|t| &t.id == id).cloned().collect(),
    }; Ok(v)
  }
}

// Read all with defaults
let v = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find(
  &repo,
  &TodoFilter::All,
  hexser::ports::repository::FindOptions::default(),
)?;
```

---

## How to Run
This tutorial is documentation-first and pairs with a runnable example that exercises adapters and prints the architecture graph:

```bash
# From repository root
cargo run --example tutorial_03_adapters
```

You should see counts per layer and a printed graph.

---

## The Pattern
Adapters are thin, technology-specific implementations of your ports:
- Keep domain-owned filters and sort keys simple (e.g., ById, ByEmail, CreatedAt)
- Translate filters inside adapters to your storage (SQL, HTTP, files)
- Keep Repository for writes; use QueryRepository for reads/deletes-by-criteria

Next: Continue with Tutorial 04 – CQRS Basics.
