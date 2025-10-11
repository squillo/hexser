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
- [Feature Flags](#feature-flags)
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
- [MCP Server (Model Context Protocol)](#-mcp-server-model-context-protocol)
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
hexser = "0.4.6"
```

Your First Hexagonal Application

```rust
use hexser::prelude::*;

// 1. Define your domain entity
#[derive(Entity)]
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

---

## Feature Flags

Hexser provides granular feature flags to enable only the functionality you need. This keeps compile times fast and binary sizes small, especially for WASM targets.

### Available Features

#### `default = ["macros", "static-di"]`
Enabled by default. Includes procedural macros and zero-cost static dependency injection.

```toml
[dependencies]
hexser = "0.4.6"  # Uses default features
```

#### `macros`
Enables procedural macros for deriving hexagonal architecture traits.

**Provides:**
- `#[derive(HexEntity)]` - Implement HexEntity trait for domain entities
- `#[derive(HexValueItem)]` - Implement HexValueItem trait with default validation (override validate() for custom logic)
- `#[derive(HexAggregate)]` - Mark aggregate roots
- `#[derive(HexPort)]` - Mark port traits
- `#[derive(HexAdapter)]` - Mark adapter implementations
- `#[derive(HexRepository)]` - Mark repository ports
- `#[derive(HexDirective)]` - Mark command/directive types
- `#[derive(HexQuery)]` - Mark query types

**Dependencies:** `hexser_macros`

```toml
[dependencies]
hexser = { version = "0.4.6", default-features = false, features = ["macros"] }
```

#### `static-di`
Zero-cost, WASM-friendly static dependency injection. No runtime overhead, no dynamic dispatch.

**Provides:**
- `StaticContainer` for compile-time dependency resolution
- Type-safe service registration without `dyn`
- Full WASM compatibility

**Dependencies:** None (zero-cost abstraction)

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["static-di"] }
```

**Example:**
```rust
use hexser::prelude::*;

let container = StaticContainer::new()
    .with_service(MyRepository::new())
    .with_service(MyService::new());

let service = container.get::<MyService>();
```

#### `ai`
Enables AI context export functionality for exposing architecture metadata to AI agents.

**Provides:**
- `AIContext` type with architecture metadata
- `AgentPack` for packaging context
- JSON serialization of graph data
- CLI tools: `hex-ai-export`, `hex-ai-pack`
- **Method-level documentation**: ComponentInfo now includes a `methods` field capturing method signatures, parameters, return types, and documentation (currently empty, ready for future extraction via rustdoc JSON)

**Dependencies:** `chrono`, `serde`, `serde_json`

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["ai"] }
```

**Usage:**
```bash
# Export architecture context to JSON
cargo run --bin hex-ai-export > context.json

# Create agent pack
cargo run --bin hex-ai-pack --output agent-pack.json
```

#### `mcp`
Model Context Protocol server implementation for serving architecture data via JSON-RPC.

**Provides:**
- MCP server over stdio transport
- Resources: `hexser://context`, `hexser://pack`
- JSON-RPC 2.0 interface
- CLI tool: `hex-mcp-server`

**Dependencies:** Requires `ai` feature, plus `serde`, `serde_json`

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["mcp"] }
```

**Usage:**
```bash
# Start MCP server (communicates via stdin/stdout)
cargo run --bin hex-mcp-server
```

#### `async`
Enables async/await support for ports and adapters.

**Provides:**
- `AsyncRepository` trait
- `AsyncDirective` trait
- `AsyncQuery` trait
- Tokio runtime integration

**Dependencies:** `tokio`, `async-trait`

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["async"] }
```

**Example:**
```rust
#[async_trait::async_trait]
impl AsyncRepository<User> for AsyncUserRepo {
    async fn find_by_id(&self, id: &String) -> HexResult<Option<User>> {
        // async implementation
    }
}
```

#### `visualization`
Enables graph visualization and export capabilities.

**Provides:**
- Graph serialization to JSON
- DOT format export for Graphviz
- Architecture diagram generation

**Dependencies:** `serde`, `serde_json`

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["visualization"] }
```

#### `container`
Dynamic dependency injection container with async support. **Not enabled by default** to maintain WASM compatibility.

**Provides:**
- `DynContainer` with runtime service resolution
- Async service factories
- Dynamic dispatch with `dyn` traits

**Dependencies:** `tokio`, `async-trait`

**Note:** Use `static-di` instead if you need WASM compatibility or want zero runtime overhead.

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["container"] }
```

#### `full`
Enables all features: `ai`, `mcp`, `async`, `macros`, `visualization`, `container`, and `static-di`.

**Use for:** Development, full-featured applications, or when you need all capabilities.

```toml
[dependencies]
hexser = { version = "0.4.6", features = ["full"] }
```

### Binary Targets

Hexser includes three command-line tools that require specific features:

#### `hex-ai-export`
Exports architecture context as JSON for AI consumption.

**Required feature:** `ai`

```bash
cargo run --bin hex-ai-export --features ai > context.json
```

#### `hex-ai-pack`
Creates a complete agent pack with architecture metadata.

**Required feature:** `ai`

```bash
cargo run --bin hex-ai-pack --features ai --output pack.json
```

#### `hex-mcp-server`
Runs an MCP (Model Context Protocol) server over stdio.

**Required feature:** `mcp`

```bash
cargo run --bin hex-mcp-server --features mcp
```

### Feature Combinations

#### Minimal (no default features)
```toml
[dependencies]
hexser = { version = "0.4.6", default-features = false }
```

#### WASM-optimized
```toml
[dependencies]
hexser = { version = "0.4.6", default-features = false, features = ["macros", "static-di"] }
```

#### AI-enabled with async
```toml
[dependencies]
hexser = { version = "0.4.6", features = ["ai", "async", "visualization"] }
```

#### Full development setup
```toml
[dependencies]
hexser = { version = "0.4.6", features = ["full"] }
```

---

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

#[derive(Entity)]
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
#[derive(Clone, PartialEq, Eq)]
struct Email(String);

impl HexValueItem for Email {
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
struct OrderPlaced {
  order_id: OrderId,
  customer_id: CustomerId,
  timestamp: u64,
}

impl DomainEvent for OrderPlaced {
  // Manual implementation - no derive macro available
}
```

Domain Services - Operations spanning multiple entities:
```rust
struct PricingService;

impl DomainService for PricingService {
  // Manual implementation - no derive macro available
}

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
  fn save(&mut self, order: Order) -> HexResult<()> {
      // SQL insert/update implementation
      todo!()
  }
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
  fn save(&mut self, user: User) -> HexResult<()> {
    self.users.insert(user.id.clone(), user);
    Ok(())
  }
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

#### Security: Controlling Source Location in Serialized Errors

When using the `serde` feature to serialize errors (e.g., for API responses), source location information (file paths, line numbers, column numbers) can expose internal code structure to clients. **hexser is secure by default** and excludes this sensitive information from serialization unless explicitly enabled.

**Environment Variable: `HEXSER_INCLUDE_SOURCE_LOCATION`**

Control whether source location is included in serialized errors:

```bash
# Production (default, secure) - source location excluded
# No environment variable needed

# Development/Debug - include source location
export HEXSER_INCLUDE_SOURCE_LOCATION=1
# or
export HEXSER_INCLUDE_SOURCE_LOCATION=true
```

**Example:**

```rust
use hexser::prelude::*;

fn api_handler() -> Result<String, Box<dyn std::error::Error>> {
    let err = hexser::hex_domain_error!(
        hexser::error::codes::domain::INVARIANT_VIOLATION,
        "Order must have items"
    );
    
    // Serialize for API response
    let json = serde_json::to_string(&err)?;
    
    // In production (env var not set):
    // {"code":"E_HEX_001","message":"Order must have items",...}
    // Source location is excluded for security
    
    // In development (HEXSER_INCLUDE_SOURCE_LOCATION=1):
    // {"code":"E_HEX_001","message":"Order must have items",
    //  "location":{"file":"src/api.rs","line":42,"column":10},...}
    
    Ok(json)
}
```

**Production Best Practice:**
- Never set `HEXSER_INCLUDE_SOURCE_LOCATION` in production environments
- Source location is still captured and available via `Display` formatting for logs
- Only serialization (JSON/API responses) is affected by this setting

**Affected Error Types:**
- `DomainError`, `PortError`, `AdapterError` (LayerError-based types)
- `ValidationError`
- `NotFoundError`
- `ConflictError`
- All errors with `location` fields


Part 6: Real-World Example - TODO Application
Let's build a complete TODO application using hexagonal architecture.
Domain Layer:

```rust
use hexser::prelude::*;

#[derive(Clone, Entity)]
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
  fn save(&mut self, todo: Todo) -> HexResult<()> {
      let mut todos = self.todos.lock().unwrap();
      if let Some(existing) = todos.iter_mut().find(|t| t.id == todo.id) {
          *existing = todo;
      } else {
          todos.push(todo);
      }
      Ok(())
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
│   ├── HexEntity        - Identity-based objects
│   ├── HexValueItem     - Value-based objects
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
hexser_potions = { path = "../hexser_potions", version = "0.4.6" }
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

#[derive(Entity, Clone, Debug)]
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
    fn save(&mut self, user: User) -> HexResult<()> { if let Some(i)=self.users.iter().position(|u| u.id==user.id){self.users[i]=user;} else { self.users.push(user);} Ok(()) }
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

### 📋 AIContext Structure

The exported `AIContext` JSON includes detailed component information:

**ComponentInfo fields:**
- `type_name`: Fully qualified type name
- `layer`: Architectural layer (Domain, Port, Adapter, Application)
- `role`: Component role (Entity, Repository, Directive, Query, etc.)
- `module_path`: Module path where component is defined
- `purpose`: Optional description of component purpose
- `dependencies`: List of component dependencies
- **`methods`**: List of public methods with detailed information (**NEW**)

**MethodInfo structure** (available in `methods` array):
- `name`: Method name
- `signature`: Full method signature
- `documentation`: Doc comment for the method
- `parameters`: Array of parameter details (name, type, description)
- `return_type`: Method return type
- `is_public`: Visibility flag
- `is_async`: Async flag

**Current Status:** The `methods` field is included in the JSON schema and ready for use. Currently populated as an empty array; future enhancement will extract method information via rustdoc JSON output or source code parsing to provide complete API documentation to AI models.

**Example ComponentInfo with methods:**
```json
{
  "type_name": "UserRepository",
  "layer": "Port",
  "role": "Repository",
  "module_path": "ports::user_repository",
  "purpose": "Manages user persistence",
  "methods": [
    {
      "name": "find_by_id",
      "signature": "fn find_by_id(&self, id: &str) -> HexResult<Option<User>>",
      "documentation": "Finds a user by their ID",
      "parameters": [
        {
          "name": "id",
          "param_type": "&str",
          "description": "User identifier"
        }
      ],
      "return_type": "HexResult<Option<User>>",
      "is_public": true,
      "is_async": false
    }
  ],
  "dependencies": []
}
```

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

---

## 🔌 MCP Server (Model Context Protocol)

Hexser includes a built-in MCP (Model Context Protocol) server that exposes your project's architecture to AI assistants via a standardized JSON-RPC interface. This enables AI tools like Claude Desktop, Cline, and other MCP-compatible clients to query your architecture in real-time.

**🆕 New to MCP?** Check out the [Beginner's Walkthrough for IntelliJ + Junie](docs/MCP_BEGINNER_WALKTHROUGH.md) for step-by-step setup instructions.

Requirements:
- Enable the `mcp` feature (automatically includes `ai`, `serde`, and `serde_json`).

### Running the MCP Server

```sh
# Run the MCP server (stdio transport)
cargo run -p hexser --features mcp --bin hex-mcp-server

# The server reads JSON-RPC requests from stdin and writes responses to stdout
```

### Available MCP Resources

The MCP server supports **multi-project mode**, exposing resources for multiple projects simultaneously:

**Resource URI Format:**
- **New (multi-project):** `hexser://{project}/context` and `hexser://{project}/pack`
- **Legacy (backward compatible):** `hexser://context` and `hexser://pack` (assumes project name "hexser")

**Resource Types:**

1. **`hexser://{project}/context`** - Machine-readable architecture context (AIContext JSON)
   - Current component graph for the specified project
   - Layer relationships
   - Architectural constraints
   - Validation rules

2. **`hexser://{project}/pack`** - Comprehensive Agent Pack (all-in-one JSON)
   - AIContext (architecture)
   - Guidelines snapshot (coding rules)
   - Embedded documentation (README, ERROR_GUIDE, etc.)

**Example Resources:**
- `hexser://hexser/context` - Architecture context for the hexser project
- `hexser://myapp/pack` - Full agent pack for myapp project
- `hexser://context` - Legacy format, maps to `hexser://hexser/context`

### Integration with AI Assistants

Configure your AI assistant to use the MCP server:

**Claude Desktop (config.json):**
```json
{
  "mcpServers": {
    "hexser": {
      "command": "cargo",
      "args": ["run", "-p", "hexser", "--features", "mcp", "--bin", "hex-mcp-server"],
      "cwd": "/path/to/your/hexser/project"
    }
  }
}
```

**Cline / Other MCP Clients:**
Follow the client-specific configuration to add the above command as an MCP server endpoint.

### What the MCP Server Does

- Accepts JSON-RPC 2.0 requests via stdin
- Implements the Model Context Protocol specification
- Provides `initialize`, `resources/list`, `resources/read`, and `hexser/refresh` methods
- Serves architecture data from multiple projects via `ProjectRegistry`
- Enables AI assistants to understand your project structure in real-time

### Refreshing Architecture After Code Changes

When AI agents modify project code (adding new components, changing architecture), the MCP server needs to be updated to reflect these changes. Hexser uses Rust's `inventory` crate which populates a static registry at **compile time**, so changes require recompilation.

**The `hexser/refresh` Method:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "hexser/refresh",
  "params": {
    "project": "myproject"
  }
}
```

**Workflow:**

1. AI agent makes code changes (adds `#[derive(HexEntity)]` to new struct, etc.)
2. AI agent calls `hexser/refresh` with project name
3. MCP server triggers `cargo build -p {project} --features macros`
4. Server returns compilation result:
   - **Success**: Returns `{"status": "restart_required", "compiled": true, ...}` with message that MCP server must be restarted
   - **Error**: Returns `{"status": "error", "compiled": false, "error": "..."}` with compilation errors

**Important:** After successful compilation, you must **manually restart the MCP server** to load the updated architecture graph. The inventory static cache is cleared and repopulated during the restart.

**Example Response (Success):**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "status": "restart_required",
    "compiled": true,
    "components_added": 0,
    "components_removed": 0,
    "error": "Compilation successful. Server restart required to load new graph."
  }
}
```

**Example Response (Compilation Error):**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "status": "error",
    "compiled": false,
    "components_added": 0,
    "components_removed": 0,
    "error": "error[E0425]: cannot find value `foo` in this scope..."
  }
}
```

**Best Practices:**
- Call `hexser/refresh` after making architectural changes
- Check the `compiled` field to verify build success
- Restart your MCP client or server process after successful refresh
- Handle compilation errors gracefully in your AI workflow

### Multi-Project Configuration

The MCP server supports serving multiple projects simultaneously using `ProjectRegistry`:

**Default Behavior (Single Project):**
By default, `McpStdioServer::new()` creates a registry with the current `HexGraph` as a single project named "hexser". This provides backward compatibility with existing configurations.

**Custom Multi-Project Setup:**
Create a custom binary to register multiple projects:

```rust
use hexser::domain::mcp::{ProjectConfig, ProjectRegistry};
use hexser::adapters::mcp_stdio::McpStdioServer;
use hexser::graph::HexGraph;

fn main() -> hexser::HexResult<()> {
    let mut registry = ProjectRegistry::new();
    
    // Register project 1
    let graph1 = HexGraph::current(); // or load from specific crate
    registry.register(ProjectConfig::new(
        String::from("myapp"),
        std::path::PathBuf::from("/path/to/myapp"),
        graph1,
    ));
    
    // Register project 2
    let graph2 = HexGraph::current(); // load from another crate
    registry.register(ProjectConfig::new(
        String::from("backend"),
        std::path::PathBuf::from("/path/to/backend"),
        graph2,
    ));
    
    let server = McpStdioServer::with_registry(registry);
    server.run()
}
```

**Environment-Based Configuration (Future):**
Future versions may support configuration via environment variables or config files for dynamic project discovery.

**Available APIs:**
- `McpStdioServer::new()` - Single project mode (backward compatible)
- `McpStdioServer::with_registry(registry)` - Multi-project mode
- `McpStdioServer::with_graph(graph)` - Deprecated, use `with_registry` instead

Notes:
- The `hex-mcp-server` binary is only built when the `mcp` feature is enabled.
- The server uses stdio transport (line-delimited JSON-RPC messages).
- Each project in the registry gets its own `hexser://{project}/context` and `hexser://{project}/pack` resources.
- For production use, consider wrapping in a process manager or systemd service.

---

## 🌦 REST Adapter Example: WeatherPort

Hexser includes a complete example of a REST-based adapter using `reqwest::blocking` and `serde_json`. This adapter connects to an external weather API and maps JSON responses to domain models with robust error handling.

### Domain Model

```rust
// Domain: Forecast value object (in hexser::domain::forecast)
pub struct Forecast {
    city: String,
    temperature_c: f64,
    condition: String,
    observed_at_iso: Option<String>,
}
```

### Port Definition

```rust
// Port: WeatherPort trait (in hexser::ports::weather_port)
pub trait WeatherPort {
    fn get_forecast(&self, city: &str) -> HexResult<Forecast>;
}
```

### Adapter Implementation

```rust
// Adapter: RestWeatherAdapter (self-contained in examples/weather_adapter.rs)
pub struct RestWeatherAdapter {
    api_base_url: String,
    client: reqwest::blocking::Client,
}

impl RestWeatherAdapter {
    pub fn new(api_base_url: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
        Self { api_base_url, client }
    }
}

impl WeatherPort for RestWeatherAdapter {
    fn get_forecast(&self, city: &str) -> HexResult<Forecast> {
        let url = format!("{}?city={}", self.api_base_url, city);
        
        // HTTP call with error mapping (API_FAILURE)
        let response = self.client.get(&url)
            .send()
            .map_err(|e| {
                Hexserror::adapter(
                    codes::adapter::API_FAILURE,
                    "Failed to connect to weather API"
                )
                .with_source(e)
                .with_next_steps(&["Verify API endpoint", "Check network"])
            })?;
        
        // Deserialize JSON with error mapping (MAPPING_FAILURE)
        let api_response: ApiWeatherResponse = serde_json::from_str(&response.text()?)
            .map_err(|e| {
                Hexserror::adapter(
                    codes::adapter::MAPPING_FAILURE,
                    "Failed to parse JSON response"
                )
                .with_source(e)
            })?;
        
        // Map to domain model
        Forecast::new(
            api_response.city,
            api_response.temp_c,
            api_response.condition,
            api_response.observed_at,
        )
    }
}
```

This complete example is available at `examples/weather_adapter.rs`. Run with:
```bash
cargo run --example weather_adapter
```

---

## 🔐 Integrating User Authentication Potions

When integrating pre-built authentication patterns from `hexser_potions`, you must connect the Potion's defined Ports to your own concrete adapters for databases and session management.

### Step 1: Define Your Ports

```rust
// Port for user persistence
trait UserRepository: Repository<User> {
    fn find_by_username(&self, username: &str) -> HexResult<Option<User>>;
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

// Port for session management (new for question 4)
trait SessionPort {
    fn create_session(&self, user_id: &str, ttl_secs: u64) -> HexResult<String>;
    fn validate_session(&self, token: &str) -> HexResult<Option<String>>;
    fn revoke_session(&self, token: &str) -> HexResult<()>;
}
```

### Step 2: Implement Database Adapter

```rust
// Concrete PostgreSQL adapter
struct PostgresUserRepository {
    pool: sqlx::PgPool,
}

impl Repository<User> for PostgresUserRepository {
    fn save(&mut self, user: User) -> HexResult<()> {
        // Execute INSERT/UPDATE via sqlx
        sqlx::query!("INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            user.id, user.username, user.email, user.password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Failed to save user")
                .with_source(e))?;
        Ok(())
    }
}

impl UserRepository for PostgresUserRepository {
    fn find_by_username(&self, username: &str) -> HexResult<Option<User>> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_READ_FAILURE, "Query failed")
                .with_source(e))
    }
}
```

### Step 3: Implement Session Adapter (Redis or In-Memory)

```rust
// Redis-based session adapter
struct RedisSessionAdapter {
    client: redis::Client,
}

impl SessionPort for RedisSessionAdapter {
    fn create_session(&self, user_id: &str, ttl_secs: u64) -> HexResult<String> {
        let token = uuid::Uuid::new_v4().to_string();
        let mut conn = self.client.get_connection()
            .map_err(|e| Hexserror::adapter(codes::adapter::CONNECTION_FAILURE, "Redis unavailable")
                .with_source(e))?;
        
        redis::cmd("SETEX")
            .arg(format!("session:{}", token))
            .arg(ttl_secs)
            .arg(user_id)
            .query(&mut conn)
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Session write failed")
                .with_source(e))?;
        
        Ok(token)
    }
    
    fn validate_session(&self, token: &str) -> HexResult<Option<String>> {
        let mut conn = self.client.get_connection()?;
        let user_id: Option<String> = redis::cmd("GET")
            .arg(format!("session:{}", token))
            .query(&mut conn)
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_READ_FAILURE, "Session read failed")
                .with_source(e))?;
        Ok(user_id)
    }
    
    fn revoke_session(&self, token: &str) -> HexResult<()> {
        let mut conn = self.client.get_connection()?;
        redis::cmd("DEL")
            .arg(format!("session:{}", token))
            .query(&mut conn)
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Session delete failed")
                .with_source(e))?;
        Ok(())
    }
}
```

### Step 4: Wire Adapters to Application

```rust
// Application context with wired adapters
struct AppContext {
    user_repo: Box<dyn UserRepository>,
    session_port: Box<dyn SessionPort>,
}

impl AppContext {
    fn new_production(db_pool: sqlx::PgPool, redis_client: redis::Client) -> Self {
        Self {
            user_repo: Box::new(PostgresUserRepository { pool: db_pool }),
            session_port: Box::new(RedisSessionAdapter { client: redis_client }),
        }
    }
}
```

---

## 🔄 Transactional Directives: ProcessOrder Example

When a directive involves multiple repository operations that must succeed or fail atomically (e.g., decrementing stock and creating an order), use a database transaction and pass it explicitly to each repository call.

### Port Definitions

```rust
// Ports accepting a transaction context
trait ProductRepository {
    fn decrement_stock(&self, tx: &mut PgTransaction, product_id: &str, qty: u32) -> HexResult<()>;
}

trait OrderRepository {
    fn create_order(&self, tx: &mut PgTransaction, order: Order) -> HexResult<()>;
}

trait EventBus {
    fn publish(&self, event: OrderCreated) -> HexResult<()>;
}
```

### Directive Handler with Transaction

```rust
struct ProcessOrderHandler {
    product_repo: Box<dyn ProductRepository>,
    order_repo: Box<dyn OrderRepository>,
    event_bus: Box<dyn EventBus>,
    db_pool: sqlx::PgPool,
}

impl ProcessOrderHandler {
    async fn handle(&self, directive: ProcessOrderDirective) -> HexResult<()> {
        // Begin transaction
        let mut tx = self.db_pool.begin()
            .await
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_CONNECTION_FAILURE, "Failed to begin transaction")
                .with_source(e))?;
        
        // 1) Decrement stock for each product (atomic within tx)
        for item in &directive.items {
            self.product_repo.decrement_stock(&mut tx, &item.product_id, item.quantity)
                .await
                .map_err(|e| {
                    // Rollback is automatic on error via Drop
                    Hexserror::domain(codes::domain::INVARIANT_VIOLATION, "Insufficient stock")
                        .with_source(e)
                })?;
        }
        
        // 2) Create order record (atomic within tx)
        let order = Order::new(directive.customer_id, directive.items)?;
        self.order_repo.create_order(&mut tx, order.clone())
            .await
            .map_err(|e| {
                Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Failed to create order")
                    .with_source(e)
            })?;
        
        // Commit transaction (all-or-nothing)
        tx.commit()
            .await
            .map_err(|e| Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Transaction commit failed")
                .with_source(e))?;
        
        // 3) Dispatch event (after commit)
        let event = OrderCreated { order_id: order.id.clone(), timestamp: now() };
        self.event_bus.publish(event)?;
        
        Ok(())
    }
}
```

### Adapter Implementation (PostgreSQL)

```rust
struct PostgresProductRepository;

impl ProductRepository for PostgresProductRepository {
    async fn decrement_stock(&self, tx: &mut PgTransaction<'_>, product_id: &str, qty: u32) -> HexResult<()> {
        let rows_affected = sqlx::query!(
            "UPDATE products SET stock = stock - $1 WHERE id = $2 AND stock >= $1",
            qty as i32, product_id
        )
        .execute(tx)
        .await
        .map_err(|e| Hexserror::adapter(codes::adapter::DB_WRITE_FAILURE, "Stock update failed")
            .with_source(e))?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(Hexserror::domain(codes::domain::INVARIANT_VIOLATION, "Insufficient stock or product not found"));
        }
        Ok(())
    }
}
```

**Key Points:**
- Pass `&mut PgTransaction` (or equivalent) to all repository methods within the transaction.
- Rollback is automatic via Rust's `Drop` trait if any error occurs before `commit()`.
- Publish events only after successful commit to ensure consistency.

---

## 🔗 Composite Adapters: ProfileRepository Example

When data must be fetched from multiple sources (e.g., SQL for core profile, NoSQL for preferences), implement a composite adapter that queries both, handles failures gracefully, and optionally caches results.

### Port Definition

```rust
trait ProfileRepository {
    fn find_by_id(&self, user_id: &str) -> HexResult<Profile>;
}
```

### Composite Adapter Implementation

```rust
struct CompositeProfileRepository {
    sql_db: sqlx::PgPool,
    nosql_client: mongodb::Client,
    cache: std::sync::Arc<std::sync::Mutex<lru::LruCache<String, Profile>>>,
}

impl CompositeProfileRepository {
    fn new(sql_db: sqlx::PgPool, nosql_client: mongodb::Client, cache_size: usize) -> Self {
        Self {
            sql_db,
            nosql_client,
            cache: std::sync::Arc::new(std::sync::Mutex::new(lru::LruCache::new(cache_size))),
        }
    }
}

impl ProfileRepository for CompositeProfileRepository {
    async fn find_by_id(&self, user_id: &str) -> HexResult<Profile> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(user_id) {
                return Ok(cached.clone());
            }
        }
        
        // 1) Fetch core profile from SQL (primary source, must succeed)
        let core_profile: SqlProfileRow = sqlx::query_as!(
            SqlProfileRow,
            "SELECT id, username, email, created_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(&self.sql_db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Hexserror::not_found("Profile", user_id),
            _ => Hexserror::adapter(codes::adapter::DB_READ_FAILURE, "SQL query failed")
                .with_source(e)
                .with_next_step("Check database connectivity"),
        })?;
        
        // 2) Enrich with preferences from NoSQL (optional, degrade gracefully)
        let collection = self.nosql_client.database("app").collection::<bson::Document>("user_prefs");
        let prefs_result = collection.find_one(bson::doc! { "user_id": user_id }, None).await;
        
        let preferences = match prefs_result {
            Ok(Some(doc)) => {
                // Parse preferences from document
                Preferences::from_bson(&doc).unwrap_or_default()
            }
            Ok(None) => {
                // User has no preferences document yet; use defaults
                Preferences::default()
            }
            Err(e) => {
                // NoSQL source failed; log warning and use defaults (degrade gracefully)
                eprintln!("Warning: Failed to fetch preferences for {}: {}", user_id, e);
                Preferences::default()
            }
        };
        
        // 3) Combine into domain model
        let profile = Profile {
            id: core_profile.id,
            username: core_profile.username,
            email: core_profile.email,
            created_at: core_profile.created_at,
            preferences,
        };
        
        // 4) Cache result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(user_id.to_string(), profile.clone());
        }
        
        Ok(profile)
    }
}
```

### Handling Data Inconsistencies

- **Primary Source Failure:** Return `Hexserror::Adapter` or `Hexserror::NotFound` with actionable guidance.
- **Secondary Source Failure:** Degrade gracefully by logging a warning and using defaults (e.g., `Preferences::default()`).
- **Caching Strategy:** Use an LRU cache with TTL to reduce load; invalidate on writes.
- **Stale Data:** Implement cache invalidation hooks or TTL-based expiry for eventually-consistent NoSQL data.

### Caching Strategies

1. **Read-Through Cache:** Check cache before querying databases (shown above).
2. **Write-Through Cache:** Invalidate or update cache on writes.
3. **TTL-Based Expiry:** Use a cache with time-to-live for each entry.

```rust
// Example: TTL-based cache wrapper
struct TtlCache<K, V> {
    cache: lru::LruCache<K, (V, std::time::Instant)>,
    ttl: std::time::Duration,
}

impl<K: std::hash::Hash + Eq, V: Clone> TtlCache<K, V> {
    fn get(&mut self, key: &K) -> Option<V> {
        if let Some((value, inserted_at)) = self.cache.get(key) {
            if inserted_at.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }
}
```

---
