# hex Examples

## Getting Started
1. `simple_todo.rs` - Your first hexagonal app
2. `tutorial_01_entity.rs` - Domain entities
3. `tutorial_02_ports.rs` - Ports and adapters

## Patterns
- `cqrs_pattern.rs` - Command/Query separation
- **CloudEvents v1.0** - Standards-compliant event publishing (see [../docs/events.md](../docs/events.md))
  - Transport-agnostic domain events with CloudEvents envelope
  - EventPublisher/EventSubscriber ports for hexagonal architecture
  - InMemoryEventBus reference implementation
  - CQRS integration: Directives emit events, Queries consume projections

## Advanced
- `graph_example.rs` - Querying your architecture
- `architecture_visualization.rs` - Export and visualization

## Complete Application Example
- `realworld_api/` - Full RealWorld API implementation
  - Complete hexagonal architecture with DPAI layers (Domain, Ports, Application, Adapters)
  - CQRS pattern with Directives (commands) and Queries (read operations)
  - Repository pattern with domain-owned Filter and SortKey enums
  - In-memory adapters with thread-safe Arc<Mutex<>> for concurrency
  - **Web adapter with axum REST API** - Complete HTTP server with JWT authentication
  - **Automatic Architecture Visualization** - Built-in Mermaid diagram generation via `./regenerate_diagram.sh`
  - Rich error handling with Hexserror and actionable error messages
  - Zero use statements - all types fully qualified for clarity
  - **Two running modes:**
    - Demonstration mode: `cargo run --example realworld_api` (shows architecture patterns)
    - Web server mode: `cd hexser/examples/realworld_api && cargo run --bin web_server` (HTTP API on port 3000)
  - **API Endpoints:** Users, Articles, Comments, Profiles, Tags with full CRUD operations
  - All 89+ tests pass, demonstrating comprehensive test coverage

## Context7 Examples (Production Patterns)
These examples demonstrate production-ready patterns for Context7 evaluation:

- `weather_adapter.rs` - REST API adapter with reqwest (Question 7)
  - Demonstrates external API integration with proper error handling
  - Shows JSON mapping to domain models
  - Run with: `cargo run --example weather_adapter`

- `auth_integration.rs` - User authentication integration (Question 4)
  - Shows how to integrate authentication Potions
  - Demonstrates database and session management adapters
  - Run with: `cargo run --example auth_integration`

- `transactional_order.rs` - Atomic multi-repository operations (Question 8)
  - Demonstrates transactional directives across multiple repositories
  - Shows atomic stock decrement, order creation, and event publishing
  - Run with: `cargo run --example transactional_order`

- `composite_profile.rs` - Multi-source data composition (Question 9)
  - Shows fetching from SQL and NoSQL sources
  - Demonstrates graceful degradation and caching strategies
  - Run with: `cargo run --example composite_profile`

## Running Examples
```bash
# Interactive mode
cargo run --example simple_todo

# See output
cargo run --example architecture_visualization

# Context7 examples
cargo run --example weather_adapter --features rest-adapter
cargo run --example auth_integration
cargo run --example transactional_order
cargo run --example composite_profile
```
