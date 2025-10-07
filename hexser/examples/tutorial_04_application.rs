//! Tutorial 04: Application Layer
//!
//! Demonstrates the application layer that orchestrates domain logic
//! and coordinates port interactions through directives and queries.
//!
//! Run with: cargo run --example tutorial_04_application
//!
//! Revision History
//! - 2025-10-07T11:55:00Z @AI: Align with v0.4 Repository by removing id-centric methods; no behavior change otherwise.

use hexser::prelude::*;

// Domain Layer
#[derive(HexDomain, Entity, Clone)]
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

struct CreateTodoHandler {
    next_id: u32,
}

impl CreateTodoHandler {
    fn new() -> Self {
        Self { next_id: 1 }
    }
}

impl DirectiveHandler<CreateTodoDirective> for CreateTodoHandler {
    fn handle(&self, directive: CreateTodoDirective) -> HexResult<()> {
        if directive.title.trim().is_empty() {
            return Err(Hexserror::validation("Title cannot be empty")
                .with_field("title"));
        }

        println!("Creating todo: {}", directive.title);
        println!("  Assigned ID: {}", self.next_id);
        println!("  Validation passed");

        Ok(())
    }
}

#[derive(HexQuery)]
struct ListActiveTodosQuery;

struct ListActiveTodosHandler<'a> {
    repository: &'a InMemoryTodoRepository,
}

impl<'a> QueryHandler<ListActiveTodosQuery, Vec<Todo>>
    for ListActiveTodosHandler<'a>
{
    fn handle(&self, _query: ListActiveTodosQuery) -> HexResult<Vec<Todo>> {
        self.repository.find_active()
    }
}

fn main() {
    println!("Tutorial 04: Application Layer\n");
    println!("{}", "=".repeat(50));

    let handler = CreateTodoHandler::new();

    let directive = CreateTodoDirective {
        title: String::from("Complete tutorial series"),
    };

    println!("\nExecuting directive:");
    handler.handle(directive).unwrap();

    let repo = InMemoryTodoRepository::new();
    let query_handler = ListActiveTodosHandler { repository: &repo };

    println!("\nExecuting query:");
    let active = query_handler.handle(ListActiveTodosQuery).unwrap();
    println!("  Found {} active todos", active.len());

    let graph = HexGraph::current();

    println!("\nComplete hexagonal architecture:");
    graph.pretty_print();

    println!("\nArchitecture summary:");
    println!("  Domain: {} components",
        graph.nodes_by_layer(Layer::Domain).len());
    println!("  Ports: {} components",
        graph.nodes_by_layer(Layer::Port).len());
    println!("  Adapters: {} components",
        graph.nodes_by_layer(Layer::Adapter).len());
    println!("  Application: {} components",
        graph.nodes_by_layer(Layer::Application).len());

    println!("\nKey concepts:");
    println!("  Directives handle write operations (CQRS)");
    println!("  Queries handle read operations");
    println!("  Application layer orchestrates domain and ports");
    println!("  Business logic stays in domain layer");

    println!("\nYou've built a complete hexagonal application!");
    println!("{}", "=".repeat(50));

    println!("\nNext steps:");
    println!("  Check architecture_visualization.rs for advanced features");
    println!("  Review cqrs_pattern.rs for CQRS best practices");
    println!("  Explore validation_example.rs for architecture checks");
}
