# Hexser - Zero-Boilerplate Hexagonal Architecture

[![Crates.io](https://img.shields.io/crates/v/hexser.svg)](https://crates.io/crates/hexser)
[![Documentation](https://docs.rs/hexser/badge.svg)](https://docs.rs/hexser)
[![License](https://img.shields.io/crates/l/hexser.svg)](https://github.com/squillo/hexser)

**Zero-boilerplate hexagonal architecture with graph-based introspection for Rust.**

The `hexser` crate provides reusable generic types and traits for implementing Hexagonal Architecture (Ports and Adapters pattern) with automatic graph construction, intent inference, and architectural validation. **Write business logic, let `hexser` handle the architecture.**

---

## Table of Contents

- [Why hexser?](#why-hexser)
- [Quick Start](#quick-start)
- [Complete Tutorial](#complete-tutorial)
- [CQRS Pattern with hex](#part-3-cqrs-pattern-with-hex)
- [Testing Your Hexagonal Application](#part-4-testing-your-hexagonal-application)
- [Error Handling](#part-5-error-handling)
- [Real-World Example - TODO Application](#part-6-real-world-example---todo-application)
- [Advanced Patterns](#advanced-patterns)
- [Knowledge Graph](#knowledge-graph)
- [Static (non-dyn) DI — WASM-friendly](#static-non-dyn-di--wasm-friendly)
- [Repository: Filter-based queries (vNext)](#repository-filter-based-queries-vnext)
- [AI Context Export (CLI)](#ai-context-export-cli)
- [Examples & Tutorials](#examples--tutorials)
- [Potions (copy-friendly examples)](#potions-copy-friendly-examples)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Additional Resources](#additional-resources)

> Tip: Press Cmd/Ctrl+F and search for “Part” to jump to tutorials.

## Why hexser?

Traditional hexagonal architecture requires significant boilerplate:
- Manual registration of components
- Explicit dependency wiring
- Repetitive trait implementations
- Complex validation logic

**hexser eliminates all of this.** Through intelligent trait design, compile-time graph construction, and rich error handling, you get:

- [x] **Zero Boilerplate** - Define your types, derive traits, done
- [x] **Type-Safe Architecture** - Compiler enforces layer boundaries
- [x] **Self-Documenting** - Graph visualization shows your architecture
- [x] **Intent Inference** - System understands itself through structure
- [x] **Rich Errors** - Helpful, actionable error messages
- [x] **Zero Runtime Overhead** - Everything happens at compile time
- [x] **AI Completion** - Expose your Rust architecture to AI agents

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
hexser = "0.3.0"
```

Your First Hexagonal Application

```rust
use hexser::prelude::*;

// 1. Define your domain entity
#[derive(HexEntity)]
struct User {
  id: String,
  email: String,
  name: String,
}

// 2. Define a port (interface)
#[derive(HexPort)]
trait UserRepository: Repository<User> {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

// 3. Implement an adapter
#[derive(HexAdapter)]
struct InMemoryUserRepository {
    users: Vec<User>,
}

impl Repository<User> for InMemoryUserRepository {
  fn find_by_id(&self, id: &String) -> HexResult<Option<User>> {
    Ok(self.users.iter().find(|u| &u.id == id).cloned())
  }

  fn save(&mut self, user: User) -> HexResult<()> {
    self.users.push(user);
    Ok(())
  }

  fn delete(&mut self, id: &String) -> HexResult<()> {
    self.users.retain(|u| &u.id != id);
    Ok(())
  }

  fn find_all(&self) -> HexResult<Vec<User>> {
    Ok(self.users.clone())
  }
}

impl UserRepository for InMemoryUserRepository {
  fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
    Ok(self.users.iter().find(|u| u.email == email).cloned())
  }
}

// 4. Use it!
fn main() -> HexResult<()> {
    let mut repo = InMemoryUserRepository { users: Vec::new() };

    let user = User {
      id: "1".to_string(),
      email: "alice@example.com".to_string(),
      name: "Alice".to_string(),
    };

    repo.save(user)?;

    let found = repo.find_by_email("alice@example.com")?;
    println!("Found: {:?}", found.map(|u| u.name));

    Ok(())
}
```

That's it! You've just built a hexagonal architecture application with:
- Clear layer separation
- Type-safe interfaces
- Testable components
- Swappable implementations

## Complete Tutorial
### Part 1: Understanding Hexagonal Architecture
Hexagonal Architecture (also known as Ports and Adapters) structures applications into concentric layers:

```
┌─────────────────────────────────────────────┐
│         Infrastructure Layer                │
│  (Databases, APIs, External Services)       │
│                                             │
│  ┌───────────────────────────────────────┐  │
│  │      Adapters Layer                   │  │
│  │  (Concrete Implementations)           │  │
│  │                                       │  │
│  │  ┌─────────────────────────────────┐  │  │
│  │  │    Ports Layer                  │  │  │
│  │  │  (Interfaces/Contracts)         │  │  │
│  │  │                                 │  │  │
│  │  │  ┌───────────────────────────┐  │  │  │
│  │  │  │   Domain Layer            │  │  │  │
│  │  │  │ (Business Logic)          │  │  │  │
│  │  │  └───────────────────────────┘  │  │  │
│  │  └─────────────────────────────────┘  │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

**Key Principles:**
- Dependency Rule: Dependencies point inward (Domain has no dependencies)
- Port Interfaces: Define what the domain needs (don't dictate how)
- Adapter Implementations: Provide concrete implementations using specific tech
- Testability: Mock adapters for testing without infrastructure


### Part 2: The Five Layers

1. Domain Layer - Your Business Logic
The domain layer contains your core business logic, completely independent of frameworks or infrastructure.
Entities - Things with identity:

```rust
use hexser::prelude::*;

#[derive(HexEntity)]
struct Order {
  id: OrderId,
  customer_id: CustomerId,
  items: Vec<OrderItem>,
  status: OrderStatus,
}

impl Aggregate for Order {
  fn check_invariants(&self) -> HexResult<()> {
    if self.items.is_empty() {
      return Err(hexser::hex_domain_error!(
        hexser::error::codes::domain::INVARIANT_EMPTY,
        "Order must contain at least one item"
      ).with_next_step("Add at least one item"));
    }
    Ok(())
  }
}
```

Value Objects - Things defined by values:

```rust
#[derive(Clone, PartialEq, Eq, HexValueObject)]
struct Email(String);

impl Email {
  fn validate(&self) -> HexResult<()> {
    if !self.0.contains('@') {
      return Err(Hexserror::validation("Email must contain @"));
    }
    Ok(())
  }
}
```

Domain Events - Things that happened:

```rust
#[derive(HexDomainEvent)]
struct OrderPlaced {
  order_id: OrderId,
  customer_id: CustomerId,
  timestamp: u64,
}
```

Domain Services - Operations spanning multiple entities:
```rust
#[derive(HexDomainService)]
struct PricingService;

impl PricingService {
  fn calculate_order_total(&self, order: &Order) -> Money {
    order.items
      .iter()
      .map(|item| item.price * item.quantity)
      .sum()
  }
}
```


2. Ports Layer - Your Interfaces
Ports define the contracts between your domain and the outside world.
Repositories - Persistence abstraction:

```rust
#[derive(HexPort)]
trait OrderRepository: Repository<Order> {
  fn find_by_customer(&self, customer_id: &CustomerId)
      -> HexResult<Vec<Order>>;

  fn find_pending(&self) -> HexResult<Vec<Order>>;
}
```

Use Cases - Business operations:

```rust
#[derive(HexPort)]
trait PlaceOrder: UseCase<PlaceOrderInput, PlaceOrderOutput> {}

struct PlaceOrderInput {
  customer_id: CustomerId,
  items: Vec<OrderItem>,
}

struct PlaceOrderOutput {
  order_id: OrderId,
}
```

Queries - Read operations (CQRS):

```rust
#[derive(HexPort)]
trait OrderHistory: Query<OrderHistoryParams, Vec<OrderView>> {}

struct OrderHistoryParams {
  customer_id: CustomerId,
  from_date: u64,
  to_date: u64,
}

struct OrderView {
  order_id: String,
  total: f64,
  status: String,
}
```


3. Adapters Layer - Your Implementations
   Adapters implement ports using specific technologies.

Database Adapter:

```rust
#[derive(HexAdapter)]
struct PostgresOrderRepository {
  pool: PgPool,
}

impl Repository<Order> for PostgresOrderRepository {
  fn find_by_id(&self, id: &OrderId) -> HexResult<Option<Order>> {
  // SQL query implementation
  todo!()
}

  fn save(&mut self, order: Order) -> HexResult<()> {
      // SQL insert/update implementation
      todo!()
  }

  // ... other methods
}

impl OrderRepository for PostgresOrderRepository {
  fn find_by_customer(&self, customer_id: &CustomerId)
  -> HexResult<Vec<Order>> {
    // Custom query implementation
    todo!()
  }

  fn find_pending(&self) -> HexResult<Vec<Order>> {
      // Custom query implementation
      todo!()
  }
}
```

API Adapter:

```rust
#[derive(HexAdapter)]
struct RestPaymentGateway {
  client: reqwest::Client,
  api_key: String,
}

impl PaymentPort for RestPaymentGateway {
  fn charge(&self, amount: Money, card: &Card) -> HexResult<PaymentResult> {
    // HTTP API call implementation
    todo!()
  }
}
```

Mapper - Data transformation:

```rust
#[derive(HexAdapter)]
struct OrderMapper;

impl Mapper<Order, DbOrderRow> for OrderMapper {
  fn map(&self, order: Order) -> HexResult<DbOrderRow> {
    Ok(DbOrderRow {
      id: order.id.to_string(),
      customer_id: order.customer_id.to_string(),
      items_json: serde_json::to_string(&order.items)?,
      status: order.status.to_string(),
    })
  }
}
```


4. Application Layer - Your Orchestration
The application layer coordinates domain logic and ports.
Directive (Write Operation):

```rust
#[derive(HexDirective)]
struct PlaceOrderDirective {
    customer_id: CustomerId,
    items: Vec<OrderItem>,
}

impl PlaceOrderDirective {
  fn validate(&self) -> HexResult<()> {
    if self.items.is_empty() {
      return Err(Hexserror::validation("Items cannot be empty"));
    }
    Ok(())
  }
}
```

Directive Handler:
```rust
#[derive(HexDirectiveHandler)]
struct PlaceOrderHandler {
  order_repo: Box<dyn OrderRepository>,
  payment_port: Box<dyn PaymentPort>,
}

impl PlaceOrderHandler {
  fn handle(&self, directive: PlaceOrderDirective) -> HexResult<()> {
    // Validate
    directive.validate()?;

    // Create domain object
    let order = Order::new(directive.customer_id, directive.items)?;

    // Check invariants
    order.check_invariants()?;

    // Save
    self.order_repo.save(order)?;

    // Side effects
    self.payment_port.charge(order.total(), &order.payment_method)?;

    Ok(())
  }
}
```

Query Handler:

```rust
#[derive(HexQueryHandler)]
struct OrderHistoryHandler {
  query_repo: Box<dyn OrderQueryRepository>,
}

impl OrderHistoryHandler {
  fn handle(&self, params: OrderHistoryParams) -> HexResult<Vec<OrderView>> {
    self.query_repo.get_order_history(
        &params.customer_id,
        params.from_date,
        params.to_date
    )
  }
}
```


5. Infrastructure Layer - Your Technology
   Infrastructure provides the concrete technology implementations.
```rust
#[derive(HexConfig)]
struct DatabaseConfig {
  connection_string: String,
  pool_size: u32,
}

impl DatabaseConfig {
  fn create_pool(&self) -> PgPool {
    // Create database connection pool
    todo!()
  }
}
```


### Part 3: CQRS Pattern with hex

hexser supports Command Query Responsibility Segregation (CQRS) out of the box.

Write Side (Directives):

```rust
// Directive represents intent to change state
#[derive(HexDirective)]
struct UpdateUserEmail {
  user_id: UserId,
  new_email: Email,
}

impl UpdateUserEmail {
  fn validate(&self) -> HexResult<()> {
    self.new_email.validate()
  }
}

// Handler executes the directive
#[derive(HexDirectiveHandler)]
struct UpdateUserEmailHandler {
  repo: Box<dyn UserRepository>,
}

impl UpdateUserEmailHandler {
  fn handle(&self, directive: UpdateUserEmail) -> HexResult<()> {
    let mut user = self.repo.find_by_id(&directive.user_id)?
      .ok_or_else(|| Hexserror::not_found("User", &directive.user_id))?;

    user.email = directive.new_email;
    self.repo.save(user)?;

    Ok(())
  }
}
```

Read Side (Queries):

```rust
// Query represents read operation
#[derive(HexQuery)]
struct FindUserByEmail {
  email: String,
}

// Handler executes the query
#[derive(HexQueryHandler)]
struct FindUserByEmailHandler {
  query_repo: Box<dyn UserQueryRepository>,
}

impl FindUserByEmailHandler {
  fn handle(&self, query: FindUserByEmail)
  -> HexResult<Option<UserView>> {
    self.query_repo.find_by_email(&query.email)
  }
}
```


### Part 4: Testing Your Hexagonal Application

Hexagonal architecture makes testing trivial - just mock the ports!

Unit Testing Domain Logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_invariants() {
        let order = Order {
          id: OrderId::new(),
          customer_id: CustomerId::new(),
          items: vec![],  // Empty!
          status: OrderStatus::Pending,
        };

        assert!(order.check_invariants().is_err());
    }

    #[test]
    fn test_email_validation() {
        let invalid = Email("notanemail".to_string());
        assert!(invalid.validate().is_err());

        let valid = Email("test@example.com".to_string());
        assert!(valid.validate().is_ok());
    }
}
```

Testing with Mock Adapters:

```rust
#[derive(HexAdapter)]
struct MockUserRepository {
  users: std::collections::HashMap<UserId, User>,
}

impl Repository<User> for MockUserRepository {
  fn find_by_id(&self, id: &UserId) -> HexResult<Option<User>> {
    Ok(self.users.get(id).cloned())
  }

  fn save(&mut self, user: User) -> HexResult<()> {
    self.users.insert(user.id.clone(), user);
    Ok(())
  }

  // ... other methods
}

#[test]
fn test_create_user_handler() {
  let mut repo = MockUserRepository {
    users: std::collections::HashMap::new(),
  };

  let handler = CreateUserHandler {
      repo: Box::new(repo),
  };

  let directive = CreateUserDirective {
      email: "test@example.com".to_string(),
      name: "Test User".to_string(),
  };

  assert!(handler.handle(directive).is_ok());
}
```


### Part 5: Error Handling

hexser provides rich, actionable, code-first errors with automatic source location and layering support. Prefer the new macro-based constructors and error codes over manual struct construction.

Preferred: macro + code + guidance

```rust
fn validate_order(order: &Order) -> HexResult<()> {
  if order.items.is_empty() {
    return Err(
        hexser::hex_domain_error!(
            hexser::error::codes::domain::INVARIANT_EMPTY,
            "Order must contain at least one item"
        )
        .with_next_steps(&["Add at least one item to the order"]) // actionable guidance
        .with_suggestions(&["order.add_item(item)", "order.items.push(item)"]) // quick fixes
        .with_more_info("https://docs.rs/hexser/latest/hexser/error/codes/domain")
    );
  }
  Ok(())
}
```

Display output (example):

```text
E_HEX_001: Order must contain at least one item
at src/domain/order.rs:42:13
Next steps:
- Add at least one item to the order
Suggestions:
- order.add_item(item)
- order.items.push(item)
```

Cookbook

```rust
// Validation errors (field-aware)
return Err(hexser::error::hex_error::Hexserror::validation_field(
    "Title cannot be empty",
    "title",
));

// Not Found errors (resource + id)
return Err(hexser::error::hex_error::Hexserror::not_found("User", "123")
    .with_next_step("Verify the ID and try again"));

// Port errors (communication issues)
let port_err = hexser::hex_port_error!(
    hexser::error::codes::port::PORT_TIMEOUT,
    "User service timed out"
).with_suggestion("Increase timeout or retry later");

// Adapter errors (infra failures) with source error
fn fetch_from_api(url: &str) -> HexResult<String> {
    let resp = std::fs::read_to_string(url)
        .map_err(|ioe| hexser::hex_adapter_error!(
            hexser::error::codes::adapter::IO_FAILURE, // or API_FAILURE in real HTTP
            "Failed to fetch resource"
        ).with_source(ioe))?;
    Ok(resp)
}
```

🔥 Amazing Example: Layered mapping (Adapter → Port → Domain)

```rust
// Adapter layer
fn db_get_user(id: &str) -> HexResult<User> {
    let conn = std::fs::read_to_string("/tmp/mock-db").map_err(|e|
        hexser::hex_adapter_error!(
            hexser::error::codes::adapter::DB_CONNECTION_FAILURE,
            "Database unavailable"
        )
        .with_source(e)
        .with_next_steps(&["Ensure DB is running", "Check connection string"]) 
    )?;
    // ... parse and return User or NotFound
    Err(hexser::error::hex_error::Hexserror::not_found("User", id))
}

// Port layer wraps adapter failure with port context
fn port_get_user(id: &str) -> HexResult<User> {
    db_get_user(id).map_err(|e|
        hexser::hex_port_error!(
            hexser::error::codes::port::COMMUNICATION_FAILURE,
            "UserRepository failed"
        ).with_source(e)
    )
}

// Domain layer consumes rich errors
fn ensure_user_exists(id: &str) -> HexResult<()> {
    let _user = port_get_user(id)?; // `?` preserves full rich error stack
    Ok(())
}
```

Notes
- All hexser errors implement std::error::Error and the RichError trait (code, message, next_steps, suggestions, location, more_info, source).
- Prefer hex_domain_error!, hex_port_error!, hex_adapter_error! and constants from hexser::error::codes::*.
- Use with_source(err) to preserve underlying causes; Display shows a helpful, compact summary.


Part 6: Real-World Example - TODO Application
Let's build a complete TODO application using hexagonal architecture.
Domain Layer:

```rust
use hexser::prelude::*;

#[derive(Clone, HexEntity)]
struct Todo {
  id: TodoId,
  title: String,
  description: String,
  completed: bool,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct TodoId(String);

impl TodoId {
  fn new() -> Self {
    Self(uuid::Uuid::new_v4().to_string())
  }
}
```

Ports Layer:

```rust
#[derive(HexPort)]
trait TodoRepository: Repository<Todo> {
    fn find_active(&self) -> HexResult<Vec<Todo>>;
    fn find_completed(&self) -> HexResult<Vec<Todo>>;
}
```

Adapters Layer:

```rust
#[derive(HexAdapter)]
struct InMemoryTodoRepository {
  todos: std::sync::Mutex<Vec<Todo>>,
}

impl Repository<Todo> for InMemoryTodoRepository {
  fn find_by_id(&self, id: &TodoId) -> HexResult<Option<Todo>> {
    let todos = self.todos.lock().unwrap();
    Ok(todos.iter().find(|t| &t.id == id).cloned())
  }

  fn save(&mut self, todo: Todo) -> HexResult<()> {
      let mut todos = self.todos.lock().unwrap();
      if let Some(existing) = todos.iter_mut().find(|t| t.id == todo.id) {
          *existing = todo;
      } else {
          todos.push(todo);
      }
      Ok(())
  }

  fn delete(&mut self, id: &TodoId) -> HexResult<()> {
      let mut todos = self.todos.lock().unwrap();
      todos.retain(|t| &t.id != id);
      Ok(())
  }

  fn find_all(&self) -> HexResult<Vec<Todo>> {
      let todos = self.todos.lock().unwrap();
      Ok(todos.clone())
  }
}

impl TodoRepository for InMemoryTodoRepository {
  fn find_active(&self) -> HexResult<Vec<Todo>> {
    let todos = self.todos.lock().unwrap();
    Ok(todos.iter().filter(|t| !t.completed).cloned().collect())
  }

  fn find_completed(&self) -> HexResult<Vec<Todo>> {
    let todos = self.todos.lock().unwrap();
    Ok(todos.iter().filter(|t| t.completed).cloned().collect())
  }
}
```

Application Layer:

```rust
#[derive(HexDirective)]
struct CreateTodoDirective {
    title: String,
    description: String,
}

impl CreateTodoDirective {
    fn validate(&self) -> HexResult<()> {
        if self.title.is_empty() {
            return Err(Hexserror::validation_field("Title cannot be empty", "title"));
        }
        Ok(())
    }
}

#[derive(HexDirectiveHandler)]
struct CreateTodoHandler {
    repo: Box<dyn TodoRepository>,
}

impl CreateTodoHandler {
    fn handle(&self, directive: CreateTodoDirective) -> HexResult<()> {
        directive.validate()?;

        let todo = Todo {
            id: TodoId::new(),
            title: directive.title,
            description: directive.description,
            completed: false,
        };

        self.repo.save(todo)?;
        Ok(())
    }
}
```

---

## 🎓 Advanced Patterns
Event Sourcing

```rust
#[derive(HexAggregate)]
struct OrderAggregate {
  id: OrderId,
  uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

impl OrderAggregate {
  fn place_order(&mut self, items: Vec<OrderItem>) -> HexResult<()> {
    // Validate
    if items.is_empty() {
      return Err(hexser::hex_domain_error!(
        hexser::error::codes::domain::INVARIANT_EMPTY,
        "Order must have items"
      ));
    }

    // Create event
    let event = OrderPlaced {
        order_id: self.id.clone(),
        items,
        timestamp: current_timestamp(),
    };

    // Apply event
    self.apply_event(&event);

    // Record event
    self.uncommitted_events.push(Box::new(event));

    Ok(())
  }

  fn apply_event(&mut self, event: &dyn DomainEvent) {
      // Update state based on event
  }
}
```

Dependency Injection

```rust
struct ApplicationContext {
    user_repo: Box<dyn UserRepository>,
    order_repo: Box<dyn OrderRepository>,
    payment_port: Box<dyn PaymentPort>,
}

impl ApplicationContext {
    fn new_production() -> Self {
        Self {
            user_repo: Box::new(PostgresUserRepository::new()),
            order_repo: Box::new(PostgresOrderRepository::new()),
            payment_port: Box::new(StripePaymentGateway::new()),
        }
    }

    fn new_test() -> Self {
        Self {
            user_repo: Box::new(MockUserRepository::new()),
            order_repo: Box::new(MockOrderRepository::new()),
            payment_port: Box::new(MockPaymentGateway::new()),
        }
    }
}
```


## 📊 Knowledge Graph

```
hexser/
├── domain/              [Core Business Logic - No Dependencies]
│   ├── Entity           - Identity-based objects
│   ├── ValueObject      - Value-based objects
│   ├── Aggregate        - Consistency boundaries
│   ├── DomainEvent      - Significant occurrences
│   └── DomainService    - Cross-entity operations
│
├── ports/               [Interface Definitions]
│   ├── Repository       - Persistence abstraction
│   ├── UseCase          - Business operations
│   ├── Query            - Read-only operations (CQRS)
│   ├── InputPort        - Entry points
│   └── OutputPort       - External system interfaces
│
├── adapters/            [Concrete Implementations]
│   ├── Adapter          - Port implementations
│   └── Mapper           - Data transformation
│
├── application/         [Orchestration Layer]
│   ├── Directive        - Write operations (CQRS)
│   ├── DirectiveHandler - Directive execution
│   └── QueryHandler     - Query execution
│
├── infrastructure/      [Technology Layer]
│   └── Config           - Infrastructure setup
│
├── error/               [Rich Error Types]
│   └── Hexserror         - Actionable errors
│
└── graph/               [Introspection - Phase 2+]
    ├── Layer            - Architectural layers
    ├── Role             - Component roles
    ├── Relationship     - Component connections
    └── NodeId           - Unique identification
```

## 💡 Design Philosophy
- "Language of the Language": Use Rust's type system to express architecture
- Zero Boilerplate: Derive everything, configure nothing
- Compile-Time Guarantees: Catch errors before runtime
- Rich Errors: Every error is helpful and actionable
- Self-Documenting: Graph reveals architecture automatically
- Testability First: Mock anything, test everything

## 🤝 Contributing
We welcome contributions! This crate follows strict coding standards:
- One item per file: Each file contains one logical item
- No imports: Fully qualified paths (except std prelude)
- Documentation: Every item has //! and /// docs
- In-file tests: Tests live with the code they test
- No unsafe: Safe Rust only
- Rust 2024: Latest edition

See CONTRIBUTING.md for details.

## 📄 License
Licensed under either of:
- Apache License, Version 2.0 (LICENSE-APACHE)
- MIT license (LICENSE-MIT)

at your option.

## 🙏 Acknowledgments
Inspired by:
- CEQRS by Scott Wyatt
- N Lang by Scott Wyatt
- Domain-Driven Design by Eric Evans
- Hexagonal Architecture by Alistair Cockburn
- Clean Architecture by Robert C. Martin
- Rust's type system and error handling
- The Rust community's commitment to excellence

## 📚 Additional Resources
- Hexagonal Architecture Explained
- Domain-Driven Design
- CQRS Pattern
- Ports and Adapters

## 🎯 Examples & Tutorials
The hex crate includes comprehensive examples and tutorials to help you learn hexagonal architecture.

Running Examples

```bash
cargo run --example simple_todo
```



---

## 🧪 Potions (copy-friendly examples)

Looking for concrete, minimal examples you can paste into your app?
Check out the Potions crate in this workspace:

- Path: ./hexser_potions
- Crate: hexser_potions
- Focus: small, mixable examples (auth signup, CRUD, etc.)

Add to your project via workspace path:

```toml
[dependencies]
hexser_potions = { path = "../hexser_potions", version = "0.3.0" }
```

Then in code:

```rust
use hexser_potions::auth::{SignUpUser, InMemoryUserRepository, execute_signup};
```


---

## ⚙️ Static (non-dyn) DI — WASM-friendly

When you want zero dynamic dispatch and the smallest possible runtime footprint (including on wasm32-unknown-unknown), use the new static DI utilities.

Feature flags:
- Enabled by default: `static-di`
- Opt-in for dyn container (tokio-based): `container`

Static DI provides two simple building blocks:
- `StaticContainer<T>`: owns your fully built object graph
- `hex_static! { ... }` macro: builds the graph from a block without any `dyn`

Example:

```rust,ignore
use hexser::prelude::*;

#[derive(Clone, Debug)]
struct Repo;
#[derive(Clone, Debug)]
struct Service { repo: Repo }

let app = hexser::hex_static!({
    let repo = Repo;
    let service = Service { repo: repo.clone() };
    (repo, service)
});

let (repo, service) = app.into_inner();
```

WASM guidance:
- Default features are WASM-friendly (no tokio). Keep `container` disabled for wasm.
- Use `static-di` (default) and avoid the dyn container for maximum compatibility.



---

## Repository: Filter-based queries (vNext)

We are migrating the repository port away from id-centric methods (find_by_id/find_all) toward a generic, filter-oriented API that better models your domain while staying storage-agnostic. The new QueryRepository trait introduces domain-owned Filter and SortKey types plus FindOptions for sorting and pagination.

Highlights:
- Define small Filter and SortKey enums/structs in your domain
- Use find_one for unique lookups and find for lists with sorting/pagination
- Legacy methods are still available but deprecated; prefer the new API

Example:

```rust
use hexser::prelude::*;
use hexser::ports::repository::{QueryRepository, FindOptions, Sort, Direction};

#[derive(HexEntity, Clone, Debug)]
struct User { id: String, email: String, created_at: u64 }

// Domain-owned query types
#[derive(Clone, Debug)]
enum UserFilter {
    ById(String),
    ByEmail(String),
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UserSortKey { CreatedAt, Email }

#[derive(Default)]
struct InMemoryUserRepository { users: Vec<User> }

impl Repository<User> for InMemoryUserRepository {
    fn find_by_id(&self, id: &String) -> HexResult<Option<User>> { Ok(self.users.iter().find(|u| &u.id == id).cloned()) }
    fn save(&mut self, user: User) -> HexResult<()> { if let Some(i)=self.users.iter().position(|u| u.id==user.id){self.users[i]=user;} else { self.users.push(user);} Ok(()) }
    fn delete(&mut self, id: &String) -> HexResult<()> { self.users.retain(|u| &u.id != id); Ok(()) }
    fn find_all(&self) -> HexResult<Vec<User>> { Ok(self.users.clone()) }
}

impl QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;

    fn find_one(&self, f: &Self::Filter) -> HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| match f { UserFilter::ById(id)=>&u.id==id, UserFilter::ByEmail(e)=>&u.email==e, UserFilter::All=>true }).cloned())
    }

    fn find(&self, f: &Self::Filter, opts: FindOptions<Self::SortKey>) -> HexResult<Vec<User>> {
        let mut items: Vec<_> = self.users.iter().filter(|u| match f { UserFilter::ById(id)=>&u.id==id, UserFilter::ByEmail(e)=>&u.email==e, UserFilter::All=>true }).cloned().collect();
        if let Some(sorts) = opts.sort {
            for s in sorts.into_iter().rev() {
                match (s.key, s.direction) {
                    (UserSortKey::CreatedAt, Direction::Asc) => items.sort_by_key(|u| u.created_at),
                    (UserSortKey::CreatedAt, Direction::Desc) => items.sort_by_key(|u| std::cmp::Reverse(u.created_at)),
                    (UserSortKey::Email, Direction::Asc) => items.sort_by(|a,b| a.email.cmp(&b.email)),
                    (UserSortKey::Email, Direction::Desc) => items.sort_by(|a,b| b.email.cmp(&a.email)),
                }
            }
        }
        let offset = opts.offset.unwrap_or(0) as usize;
        let limit = opts.limit.map(|l| l as usize).unwrap_or_else(|| items.len().saturating_sub(offset));
        let end = offset.saturating_add(limit).min(items.len());
        Ok(items.into_iter().skip(offset).take(end.saturating_sub(offset)).collect())
    }
}

fn main() -> HexResult<()> {
    let repo = InMemoryUserRepository::default();
    // Unique lookup
    let _ = <InMemoryUserRepository as QueryRepository<User>>::find_one(&repo, &UserFilter::ByEmail("alice@ex.com".into()))?;

    // List with pagination
    let opts = FindOptions { sort: Some(vec![Sort { key: UserSortKey::CreatedAt, direction: Direction::Desc }]), limit: Some(25), offset: Some(0) };
    let _page = <InMemoryUserRepository as QueryRepository<User>>::find(&repo, &UserFilter::All, opts)?;
    Ok(())
}
```

Migration tips:
- find_by_id(id) -> find_one(&Filter::ById(id))
- find_all() -> find(&Filter::All, FindOptions::default())
- Add sorting/pagination via FindOptions { sort, limit, offset }

For more details, see MIGRATION_GUIDE.md and docs/core-concepts.md.

### v0.4 QueryRepository Examples (5+)

The following focused examples demonstrate the new query-first API using domain-owned Filter and SortKey types. These snippets avoid deprecated methods and illustrate common tasks.

1) Unique lookup with find_one

```rust
// Given: domain types User, UserFilter::ByEmail(String)
let repo = InMemoryUserRepository::default();
let maybe_user = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>
    ::find_one(&repo, &UserFilter::ByEmail(String::from("alice@example.com")))?;
```

2) Listing with multi-key sorting (Email asc, CreatedAt desc)

```rust
let opts = hexser::ports::repository::FindOptions {
    sort: Some(vec![
        hexser::ports::repository::Sort { key: UserSortKey::Email, direction: hexser::ports::repository::Direction::Asc },
        hexser::ports::repository::Sort { key: UserSortKey::CreatedAt, direction: hexser::ports::repository::Direction::Desc },
    ]),
    limit: None,
    offset: None,
};
let users = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>::find(
    &repo,
    &UserFilter::All,
    opts,
)?;
```

3) Pagination (page size 10, second page)

```rust
let opts = hexser::ports::repository::FindOptions { sort: None, limit: Some(10), offset: Some(10) };
let page = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>::find(&repo, &UserFilter::All, opts)?;
```

4) Existence check

```rust
let exists = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>::exists(
    &repo,
    &UserFilter::ByEmail(String::from("bob@example.com")),
)?;
```

5) Count matching entities

```rust
let total = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>::count(
    &repo,
    &UserFilter::All,
)?;
```

6) Delete by filter (returns removed count)

```rust
let removed = <InMemoryUserRepository as hexser::ports::repository::QueryRepository<User>>::delete_where(
    &mut repo.clone(),
    &UserFilter::ByEmail(String::from("bob@example.com")),
)?;
```

---

## 🤖 AI Context Export (CLI)

Export a machine-readable JSON describing your project's architecture for AI assistants and tooling.

Requirements:
- Enable the `ai` feature (serde/serde_json are included automatically).

Commands:

```sh
# Build and run the exporter (prints JSON to stdout)
cargo run -p hexser --features ai --bin hex-ai-export

# Save to a file
cargo run -p hexser --features ai --bin hex-ai-export --quiet > target/ai-context.json
```

What it does:
- Builds the current `HexGraph` from the component registry
- Generates an `AIContext` via `hexser::ai::ContextBuilder`
- Serializes to JSON with a stable field order

Notes:
- The binary `hex-ai-export` is only built when the `ai` feature is enabled.
- For reproducible diffs, commit `target/ai-context.json` or generate it in CI as an artifact.

### 🧠 AI Agent Pack (All-in-One)

Export a comprehensive, single-file JSON that bundles:
- AIContext (machine-readable architecture)
- Guidelines snapshot (rules enforced for agents)
- Embedded key docs (README, ERROR_GUIDE, and local AI/guideline prompts when present)

Commands:

```sh
# Print Agent Pack JSON to stdout
cargo run -p hexser --features ai --bin hex-ai-pack

# Save to a file
cargo run -p hexser --features ai --bin hex-ai-pack --quiet > target/ai-pack.json
```

Notes:
- Missing optional docs are skipped gracefully. The pack remains valid JSON.
- Use this artifact as the single source of truth for external AIs and tools when proposing changes.
