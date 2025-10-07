//! Simple TODO application demonstrating hexagonal architecture.
//!
//! This example shows the minimal setup needed to build a hexagonal
//! application with hex. It demonstrates domain entities, ports, adapters,
//! and application layer working together.
//!
//! Run with: `cargo run --example simple_todo`
//!
//! Revision History
//! - 2025-10-07T11:56:00Z @AI: Migrate to v0.4 QueryRepository API; remove use statements and id-centric methods; update example flow.

fn main() -> hexser::HexResult<()> {
    println!("=== Simple TODO Application ===\n");

    // Create repository
    let mut repo = InMemoryTodoRepository::new();

    // Create a todo
    let todo = Todo {
        id: String::from("1"),
        title: String::from("Learn hexagonal architecture"),
        completed: false,
    };

    println!("Creating todo: {}", todo.title);
    <InMemoryTodoRepository as hexser::ports::Repository<Todo>>::save(&mut repo, todo)?;

    // Find all todos via QueryRepository
    let all = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find(
        &repo,
        &TodoFilter::All,
        hexser::ports::repository::FindOptions::default(),
    )?;
    println!("Total todos: {}", all.len());

    // Find specific todo by id via QueryRepository
    if let Some(found) = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find_one(
        &repo,
        &TodoFilter::ById(String::from("1")),
    )? {
        println!("Found todo: {} (completed: {})", found.title, found.completed);
    }

    // Complete the todo and save
    if let Some(mut todo) = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::find_one(
        &repo,
        &TodoFilter::ById(String::from("1")),
    )? {
        todo.completed = true;
        <InMemoryTodoRepository as hexser::ports::Repository<Todo>>::save(&mut repo, todo)?;
        println!("Todo marked as completed");
    }

    // Clean up by deleting where id matches
    let _removed = <InMemoryTodoRepository as hexser::ports::repository::QueryRepository<Todo>>::delete_where(
        &mut repo,
        &TodoFilter::ById(String::from("1")),
    )?;
    println!("Todo deleted");

    println!("\n✅ Example completed successfully!");

    Ok(())
}

// Domain: Entity
#[derive(Clone)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

impl hexser::domain::Entity for Todo {
    type Id = String;
}

// Adapter: In-memory repository
struct InMemoryTodoRepository {
    todos: Vec<Todo>,
}

impl InMemoryTodoRepository {
    fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }
}

impl hexser::adapters::Adapter for InMemoryTodoRepository {}

impl hexser::ports::Repository<Todo> for InMemoryTodoRepository {
    fn save(&mut self, todo: Todo) -> hexser::HexResult<()> {
        if let Some(existing) = self.todos.iter_mut().find(|t| t.id == todo.id) {
            *existing = todo;
        } else {
            self.todos.push(todo);
        }
        Ok(())
    }
}

#[derive(Clone)]
enum TodoFilter { All, ById(String) }
#[derive(Clone, Copy)]
enum TodoSortKey { Id }

impl hexser::ports::repository::QueryRepository<Todo> for InMemoryTodoRepository {
    type Filter = TodoFilter;
    type SortKey = TodoSortKey;

    fn find_one(&self, filter: &TodoFilter) -> hexser::HexResult<Option<Todo>> {
        let found = match filter {
            TodoFilter::All => self.todos.first().cloned(),
            TodoFilter::ById(id) => self.todos.iter().find(|t| &t.id == id).cloned(),
        };
        Ok(found)
    }

    fn find(&self, filter: &TodoFilter, _opts: hexser::ports::repository::FindOptions<TodoSortKey>) -> hexser::HexResult<Vec<Todo>> {
        let items: Vec<Todo> = match filter {
            TodoFilter::All => self.todos.clone(),
            TodoFilter::ById(id) => self.todos.iter().filter(|t| &t.id == id).cloned().collect(),
        };
        Ok(items)
    }

    fn delete_where(&mut self, filter: &TodoFilter) -> hexser::HexResult<u64> {
        let before = self.todos.len();
        match filter {
            TodoFilter::All => self.todos.clear(),
            TodoFilter::ById(id) => self.todos.retain(|t| &t.id != id),
        }
        Ok((before.saturating_sub(self.todos.len())) as u64)
    }
}
