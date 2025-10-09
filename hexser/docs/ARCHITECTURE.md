# Hexser Internal Architecture

This document describes the internal architecture of the `hexser` crate, designed for contributors and maintainers.

## Overview

The `hexser` crate implements hexagonal architecture patterns using Rust's type system. The crate itself follows hexagonal architecture principles, demonstrating "dogfooding" of its own patterns. This document explains how the crate is structured internally, how the zero-boilerplate experience is achieved, and how various subsystems work together.

## Design Principles

### 1. Zero Boilerplate Through Compile-Time Codegen
Hexser achieves its zero-boilerplate DX through procedural macros that generate:
- Trait implementations (HexEntity, HexValueItem, Aggregate)
- Component registration code via the `inventory` crate
- Graph node metadata for architectural analysis

### 2. Inventory-Based Registration
Components self-register at compile time using the distributed static collection pattern:
- Each `#[derive(HexDomain)]`, `#[derive(HexAdapter)]`, etc. generates an `inventory::submit!` block
- The `ComponentRegistry` collects all submissions at runtime via `inventory::iter`
- No central registration file needed—fully distributed and automatic

### 3. Type-Safe Layer Boundaries
Rust's type system enforces hexagonal architecture:
- Domain layer has no dependencies (only std types)
- Ports define trait boundaries
- Adapters implement ports with concrete technology
- Generic types ensure compile-time correctness

### 4. Rich, Actionable Errors
Error handling follows a layered approach:
- Structured error types with codes, context, and guidance
- Source location capture via macros (`hex_domain_error!`, etc.)
- Builder pattern for adding next steps, suggestions, and links
- Conversion from infrastructure errors (IO, DB) with `.map_err()`

## Module Structure

### Core Modules (src/)

```
hexser/
├── domain/               # Pure business logic traits
│   ├── entity.rs         # HexEntity trait (identity)
│   ├── value_object.rs   # HexValueItem trait (validation)
│   ├── aggregate.rs      # Aggregate trait (invariants)
│   ├── domain_event.rs   # DomainEvent trait
│   ├── domain_service.rs # DomainService trait
│   └── mcp/              # MCP domain types (feature-gated)
│
├── ports/                # Interface definitions
│   ├── repository.rs     # Repository<T> and QueryRepository<T>
│   ├── query.rs          # Query<Params, Result> trait
│   ├── use_case.rs       # UseCase trait
│   ├── input_port.rs     # InputPort marker
│   ├── output_port.rs    # OutputPort marker
│   └── mcp_server.rs     # McpServer trait (feature-gated)
│
├── adapters/             # Port implementation markers
│   ├── adapter.rs        # Adapter marker trait
│   ├── mapper.rs         # Mapper<From, To> trait
│   └── mcp_stdio.rs      # MCP stdio adapter (feature-gated)
│
├── application/          # Use case orchestration
│   ├── directive.rs      # Directive trait (commands)
│   ├── directive_handler.rs
│   ├── query_handler.rs  # QueryHandler trait
│   └── use_case.rs
│
├── graph/                # Architecture graph
│   ├── hex_graph.rs      # Main graph structure
│   ├── hex_node.rs       # Node representation
│   ├── hex_edge.rs       # Edge/relationship
│   ├── layer.rs          # Layer enum (Domain, Ports, Adapters, etc.)
│   ├── role.rs           # Role enum (Entity, Repository, etc.)
│   ├── node_id.rs        # NodeId type
│   └── graph_builder.rs  # Graph construction logic
│
├── registry/             # Component registration system
│   ├── component_registry.rs  # Global registry using inventory
│   ├── component_entry.rs     # Registration entry point
│   ├── registrable.rs         # Registrable trait
│   ├── node_info.rs           # Component metadata
│   └── inventory_integration.rs
│
├── error/                # Error handling subsystem
│   ├── hex_error.rs      # Main HexError type
│   ├── codes.rs          # Error code registry
│   ├── validation_error.rs
│   ├── domain_error.rs
│   ├── port_error.rs
│   ├── adapter_error.rs
│   └── source_location.rs # Macro-captured location
│
├── result/               # Result types
│   └── hex_result.rs     # HexResult<T> alias
│
├── ai/                   # AI context export (feature: ai)
│   ├── context.rs        # AIContext type
│   └── context_builder.rs
│
├── container/            # DI container (feature: container)
│   ├── container.rs      # Dynamic container with tokio
│   └── provider.rs
│
├── static_di/            # Static DI (feature: static-di)
│   ├── static_container.rs  # Zero-cost container
│   └── static_builder.rs
│
├── infrastructure/       # External concerns
│   └── config.rs         # Config trait
│
├── showcase/             # Introspection utilities
│   ├── describable.rs    # Describable trait
│   ├── inspectable.rs    # Inspectable trait
│   └── pretty_print.rs   # PrettyPrint trait
│
├── templates/            # Code generation helpers
│   └── mod.rs            # Macros for common patterns
│
└── bin/                  # CLI binaries
    ├── hex_ai_export.rs  # Export AIContext JSON
    ├── hex_ai_pack.rs    # Create agent pack
    └── hex_mcp_server.rs # MCP server over stdio
```

### Macro Crate (hexser_macros/)

```
hexser_macros/
├── derive/               # Derive macro implementations
│   ├── entity.rs         # #[derive(HexEntity)]
│   ├── hex_value_item.rs # #[derive(HexValueItem)]
│   ├── aggregate.rs      # #[derive(HexAggregate)]
│   ├── hex_domain.rs     # #[derive(HexDomain)]
│   ├── hex_port.rs       # #[derive(HexPort)]
│   ├── hex_adapter.rs    # #[derive(HexAdapter)]
│   ├── repository.rs     # #[derive(HexRepository)]
│   ├── directive.rs      # #[derive(HexDirective)]
│   └── query.rs          # #[derive(HexQuery)]
│
├── error/                # Error construction macros
│   └── hex_error_macro.rs # hex_domain_error!, hex_port_error!, etc.
│
├── common/               # Shared utilities
│   └── validation.rs     # Input validation helpers
│
└── registration/         # Registration code generation
    └── mod.rs            # Helpers for inventory submission
```

## Key Subsystems

### 1. Component Registration System

**How it works:**

1. User writes: `#[derive(HexDomain)] struct User { id: String }`
2. Macro expands to:
   ```rust
   impl Registrable for User {
       fn node_info() -> NodeInfo { /* metadata */ }
       fn dependencies() -> Vec<NodeId> { vec![] }
   }
   
   inventory::submit! {
       ComponentEntry::new::<User>()
   }
   ```
3. At runtime, `ComponentRegistry::build_graph()` calls `inventory::iter()` to collect all submissions
4. Graph is constructed from collected components

**Key files:**
- `hexser/src/registry/component_registry.rs` - Global registry
- `hexser/src/registry/component_entry.rs` - Entry type
- `hexser_macros/src/derive/hex_domain.rs` - Example derive implementation

### 2. Graph Construction

**Flow:**

1. User calls `HexGraph::current()` or `ComponentRegistry::build_graph()`
2. Registry iterates over all `ComponentEntry` instances
3. For each entry:
   - Extracts `NodeInfo` (layer, role, type name, module path)
   - Creates `HexNode` with unique `NodeId`
   - Analyzes dependencies to create `HexEdge` instances
4. Returns populated `HexGraph` with nodes and edges

**Graph types:**
- `HexNode` - Component in the architecture (struct, trait, etc.)
- `HexEdge` - Relationship between components (depends_on, implements, etc.)
- `Layer` - Architectural layer (Domain, Ports, Adapters, Application, Infrastructure)
- `Role` - Component role (Entity, Repository, Adapter, UseCase, etc.)

**Key files:**
- `hexser/src/graph/hex_graph.rs` - Main graph
- `hexser/src/graph/graph_builder.rs` - Construction logic
- `hexser/src/registry/component_registry.rs` - Data source

### 3. Derive Macro System

**Architecture:**

Each derive macro follows a standard pattern:

1. Parse input using `syn::parse_macro_input!`
2. Validate input (struct vs enum, field requirements)
3. Extract metadata (field names, types, attributes)
4. Generate trait implementation using `quote!`
5. Generate inventory submission (for registration)
6. Return `TokenStream` for compiler

**Example: HexEntity derive**

```rust
// User code
#[derive(HexEntity)]
struct User {
    id: String,
    email: String,
}

// Expands to
impl hexser::domain::HexEntity for User {
    type Id = String;  // Detected from 'id' field
}
```

**Key files:**
- `hexser_macros/src/derive/*.rs` - All derive implementations
- `hexser_macros/src/common/validation.rs` - Shared validation logic

### 4. Error Handling System

**Architecture:**

Errors are layered and structured:

```rust
HexError {
    layer: Layer,            // Which layer produced this?
    code: String,            // Error code (e.g., "E_HEX_001")
    message: String,         // Human-readable message
    next_steps: Vec<String>, // Actionable guidance
    suggestions: Vec<String>,// Quick fixes
    more_info: Option<String>, // Link to docs
    source: Option<Box<dyn Error>>, // Underlying cause
    location: Option<SourceLocation>, // File:line from macro
}
```

**Error construction macros:**

```rust
// Captures file!(), line!(), column!() automatically
hex_domain_error!(codes::domain::INVARIANT_EMPTY, "Order has no items")
    .with_next_step("Add at least one item")
    .with_suggestion("order.add_item(item)")
```

**Error propagation:**

```rust
// Adapter layer
fn fetch() -> HexResult<Data> {
    std::fs::read("file").map_err(|io_err|
        hex_adapter_error!(codes::adapter::IO_FAILURE, "Read failed")
            .with_source(io_err)
    )
}

// Port layer wraps adapter errors
fn get_data() -> HexResult<Data> {
    fetch().map_err(|e|
        hex_port_error!(codes::port::COMMUNICATION_FAILURE, "Repository failed")
            .with_source(e)
    )
}
```

**Key files:**
- `hexser/src/error/hex_error.rs` - Main error type
- `hexser/src/error/codes.rs` - Centralized error codes
- `hexser_macros/src/error/hex_error_macro.rs` - Error construction macros

### 5. Repository Query System

**v0.4 Design:**

Repositories now use domain-owned Filter and SortKey types:

```rust
// Domain defines what queries are possible
enum UserFilter {
    ById(String),
    ByEmail(String),
    Active,
    All,
}

enum UserSortKey {
    Email,
    CreatedAt,
}

// Repository implements queries using domain types
impl QueryRepository<User> for UserRepo {
    type Filter = UserFilter;
    type SortKey = UserSortKey;
    
    fn find_one(&self, filter: &UserFilter) -> HexResult<Option<User>>;
    fn find(&self, filter: &UserFilter, opts: FindOptions<UserSortKey>) -> HexResult<Vec<User>>;
}
```

**Benefits:**
- Domain controls what queries exist (no leaky abstractions)
- Type-safe query construction
- Storage-agnostic (SQL, NoSQL, in-memory all use same interface)
- Easy to extend with new filter types

**Key files:**
- `hexser/src/ports/repository.rs` - Repository and QueryRepository traits
- Examples in `hexser/examples/*.rs` and `hexser_potions/`

### 6. Feature Flag Architecture

**Strategy:**

Hexser uses granular features to minimize dependencies and support WASM:

- `default = ["macros", "static-di"]` - Core experience
- `macros` - Derive macros (pulls in `hexser_macros`)
- `static-di` - Zero-cost DI (no dependencies)
- `container` - Dynamic DI with tokio (not WASM-friendly)
- `ai` - Context export with serde/chrono
- `mcp` - MCP server (requires `ai`)
- `async` - Async trait variants
- `visualization` - Graph export
- `full` - All features

**WASM compatibility:**
- Core crate + `macros` + `static-di` = fully WASM-compatible
- `container` feature uses tokio → not WASM-friendly
- Always prefer `static-di` for WASM targets

**Implementation:**

```rust
// Conditional compilation
#[cfg(feature = "macros")]
pub use hexser_macros::HexEntity;

#[cfg(feature = "ai")]
pub mod ai;

// Cargo.toml
[features]
default = ["macros", "static-di"]
ai = ["chrono", "serde", "serde_json"]
```

**Key files:**
- `hexser/Cargo.toml` - Feature definitions
- `hexser/src/lib.rs` - Conditional re-exports

## Testing Strategy

### Unit Tests

Each module contains inline tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_trait() {
        // Test implementation
    }
}
```

**Location:** Same file as code (`src/domain/entity.rs` includes its own tests)

### Integration Tests

Located in `hexser/tests/`:
- `integration_test.rs` - Cross-module integration
- `macro_tests.rs` - Derive macro functionality
- `graph_tests.rs` - Graph construction and validation

### Example-Based Tests

All examples in `hexser/examples/*.rs` serve as:
- Executable documentation
- Smoke tests for API ergonomics
- Real-world usage patterns

**Run with:** `cargo run --example simple_todo`

### Potion Tests

`hexser_potions/` crate contains copy-paste-friendly examples with tests:
- `auth/` - Authentication flow
- `crud/` - CRUD operations
- Each module has `#[cfg(test)] mod tests`

**Run with:** `cargo test -p hexser_potions`

### Doc Tests

All public API items have doc comments with examples:

```rust
/// Example usage
///
/// ```rust
/// use hexser::HexEntity;
/// struct User { id: String }
/// impl HexEntity for User {
///     type Id = String;
/// }
/// ```
```

**Run with:** `cargo test --doc`

## Coding Conventions

### Strict Rules (from guidelines)

1. **NO `use` statements** - All types fully qualified (except std prelude)
   - ✅ `std::collections::HashMap`
   - ❌ `use std::collections::HashMap;`

2. **One item per file** - Each file contains one primary struct/trait/fn
   - Exception: `impl` blocks stay with their type

3. **File-level docs** - Every file starts with `//!` documentation
   ```rust
   //! Brief one-line description.
   //!
   //! Detailed explanation (3-5 lines).
   //!
   //! Revision History
   //! - 2025-10-09T14:25:00Z @AI: Description of change.
   ```

4. **In-file tests** - Tests live in the same file as code
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
   }
   ```

5. **No `unsafe`** - Safe Rust only (except FFI)

### Recommended Practices

- Functions ≤50 lines (excluding signature, comments, blank lines)
- Prefer immutable data and iterator chains
- Use builder pattern for complex construction
- Meaningful error messages with actionable guidance
- Doc comments on all public items

## Contributing Workflow

### 1. Setup

```bash
git clone https://github.com/squillo/hexser
cd hexser
cargo build --all-features
cargo test --all-features
```

### 2. Development

- Create feature branch: `git checkout -b feature/your-feature`
- Follow coding conventions above
- Add tests for new functionality
- Update documentation

### 3. Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test --test integration_test
cargo test --test macro_tests

# Test examples
cargo run --example simple_todo

# Test potions
cargo test -p hexser_potions

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-features
```

### 4. Documentation

- Update README.md for user-facing changes
- Update ARCHITECTURE.md (this file) for internal changes
- Add/update doc comments
- Update book docs in `hexser/docs/src/`

### 5. Submit PR

- Ensure all tests pass
- Ensure no new warnings
- Write clear commit messages
- Reference any related issues

## Publishing

See `PUBLISHING.md` in repository root for detailed release process.

**Quick summary:**
1. Publish `hexser_macros` first
2. Wait for crates.io indexing
3. Publish `hexser`
4. Publish `hexser_potions`
5. Tag release: `git tag v0.x.y`

## Architecture Decision Records

See `hexser/docs/adr/` for key architectural decisions:
- `0001-hexagonal-architecture.md` - Why hexagonal architecture
- Additional ADRs as needed

## Future Phases

Hexser development follows a phased approach:

- **Phase 1 (Complete):** Core traits and types
- **Phase 2 (Complete):** Graph introspection
- **Phase 3 (Complete):** Derive macros and registration
- **Phase 4 (Complete):** Advanced analysis and validation (query API, intent inference, validation rules, Mermaid diagrams, refactoring suggestions)
- **Phase 5 (Complete):** Visualization & export (DOT, Mermaid, JSON exporters with hexagonal architecture implementation)
- **Phase 6 (Planned):** Code generation and scaffolding

## Getting Help

- **User Documentation:** See README.md and the book in `docs/`
- **Internal Questions:** This document (ARCHITECTURE.md)
- **Issues:** https://github.com/squillo/hexser/issues
- **Discussions:** GitHub Discussions

## Revision History

- 2025-10-09T14:32:00Z @AI: Correct phase statuses - Phase 4 and Phase 5 are both complete per CHANGELOG.md; add Phase 6 as planned.
- 2025-10-09T14:25:00Z @AI: Complete comprehensive internal architecture documentation for contributors covering all subsystems, conventions, and workflows.
