//! Simple TODO application demonstrating hexagonal architecture.
//!
//! This example shows the minimal setup needed to build a hexagonal
//! application with hex. It demonstrates domain entities, ports, adapters,
//! and application layer working together.
//!
//! Run with: `cargo run --example simple_todo`

use hexser::Repository;

// Use hex prelude for convenient imports
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
    repo.save(todo)?;

    // Find all todos
    let all_todos = repo.find_all()?;
    println!("Total todos: {}", all_todos.len());

    // Find specific todo
    if let Some(found) = repo.find_by_id(&String::from("1"))? {
        println!("Found todo: {} (completed: {})", found.title, found.completed);
    }

    // Complete the todo
    if let Some(mut todo) = repo.find_by_id(&String::from("1"))? {
        todo.completed = true;
        repo.save(todo)?;
        println!("Todo marked as completed");
    }

    // Clean up
    repo.delete(&String::from("1"))?;
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
    fn find_by_id(&self, id: &String) -> hexser::HexResult<Option<Todo>> {
        Ok(self.todos.iter().find(|t| &t.id == id).cloned())
    }

    fn save(&mut self, todo: Todo) -> hexser::HexResult<()> {
        if let Some(existing) = self.todos.iter_mut().find(|t| t.id == todo.id) {
            *existing = todo;
        } else {
            self.todos.push(todo);
        }
        Ok(())
    }

    fn delete(&mut self, id: &String) -> hexser::HexResult<()> {
        self.todos.retain(|t| &t.id != id);
        Ok(())
    }

    fn find_all(&self) -> hexser::HexResult<Vec<Todo>> {
        Ok(self.todos.clone())
    }
}
