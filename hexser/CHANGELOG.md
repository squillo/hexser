## [Unreleased]

### Registration honesty (2026-09-04)

Three paths could report a component as registered while it was absent from
`HexGraph::current()`. All three now either do the registration or say they cannot. **Two are
behaviour changes for existing code**:

- `#[derive(HexDomain)]` now honours `#[hex(role = "...")]`. It declared `attributes(hex)` and
  never read it, so `#[hex(role = "ValueObject")]` compiled clean and registered
  `Role::Entity`. `#[derive(HexDirective)]` and `#[derive(HexQuery)]` now accept the same
  attribute, so all five registration derives honour it and only the default differs.
  ⚠ A graph built before this change reports the DEFAULT role for every overridden type.
- ⚠ **BREAKING:** a registration derive on a GENERIC type is now a compile error naming the
  marker-struct remedy. It used to emit the `Registrable` impl and drop the
  `inventory::submit!`, so the type answered `node_info()` and was in no graph, with no error
  and no warning. `type_name::<Self>()` on a generic names a monomorphization chosen by a
  consumer crate, so there is no single honest node to submit — the omission was right and
  the silence was the defect. Code that derived on a generic must move the derive to a
  non-generic marker struct or hand-write `impl Registrable`.
- ⚠ **BEHAVIOUR CHANGE:** `hex_register_component!` (and `hex_register_domain!`,
  `hex_register_port!`, `hex_register_adapter!`, `hex_register_application!`,
  `hex_register_infrastructure!`) now emit the `inventory::submit!` they are named for. They
  emitted only the `Registrable` impl, for every type — so hexser had NO non-derive door into
  the graph while advertising one. Types registered through them now appear in
  `HexGraph::current()`; any assertion pinning an exact `node_count()` will see more nodes.

---

## [0.5.0] - 2026-07-21

### Hardening pass (2026-07)

Correctness / reliability:
- Fixed a stack overflow when the `visualization` feature is off (the default): `Visualizable`
  for `HexGraph` recursed into itself.
- MCP server now handles id-less JSON-RPC notifications per spec (no spurious error response),
  returns the correct -32700/-32600 error codes, and runs `hexser/refresh` builds with a
  timeout and bounded stderr.
- `#[derive(HexDirective)]` no longer emits an unresolved `inventory::submit!`; `#[derive(HexQuery)]`
  now registers in the graph; derives error clearly on misuse. (The claim that "derives work on
  generic types" was true only of the `Registrable` impl — the submission was dropped in silence.
  See Unreleased.)
- The DI `Container` no longer holds locks across user provider code (deadlock fix; singletons
  use `OnceCell`).
- `InMemoryEventBus` routes by event type to all handlers for a topic (was last-subscription-wins),
  uses a `VecDeque`, and bounds push-mode queue growth.
- Rich-error guidance (`with_next_step`/`with_suggestion`) is retained on all error variants and
  `Hexserror::with_source` is now available; `QueryRepository::delete_where`'s default errors
  instead of silently returning `Ok(0)`.

API / docs:
- Re-exported the v0.4 read API (`QueryRepository`, `FindOptions`, `Sort`, `Direction`) from the
  crate root and prelude.
- `AIContext::to_json` / `AgentPack::to_json` now return `HexResult<String>`.
- Added an explicit `serde` feature; rewrote the crate docs and README Quick Start against the
  real API (now guarded by a compiled test).

Performance / footprint:
- `HexGraph::current()` is cached; graph edge queries use a precomputed adjacency index; node
  iteration is deterministic (`BTreeMap`).
- Removed the `chrono` dependency (std-only RFC3339 timestamp) and trimmed `syn`/`tokio`
  features, reducing downstream compile time and binary size.

Tooling:
- Pinned the toolchain to stable, fixed crate metadata (repository URLs, MSRV), and reworked CI
  (feature matrix, doctests, realworld example, cargo-deny, push-on-main).

### Phase 5: Visualization & Export (Completed)
- Hexagonal architecture for visualization system
- Domain models (VisualGraph, VisualNode, VisualEdge, VisualStyle)
- Port traits (FormatExporter)
- Adapters for multiple formats (DOT, Mermaid, JSON)
- Application layer (ExportGraph use case)
- Convenience methods on HexGraph (to_dot, to_mermaid, to_json)
- Visualizable showcase trait
- Extensible format system (easy to add new formats)
- Comprehensive tests for all exporters

### Phase 4 (Completed)
- Advanced query API with complex filters
- Intent inference engine (pattern detection)
- Architectural validation rules
- Mermaid diagram generation
- Refactoring suggestions

---

## [0.3.0] - 2025-10-02

### Added - Phase 3: Zero-Boilerplate DX

#### Procedural Macros
- `#[derive(HexDomain)]` - Automatic domain layer registration
- `#[derive(HexPort)]` - Port trait registration
- `#[derive(HexAdapter)]` - Adapter implementation registration
- `#[derive(Entity)]` - Automatic Entity trait implementation
- `#[derive(HexRepository)]` - Repository port marker
- `#[derive(HexDirective)]` - Directive (command) registration
- `#[derive(HexQuery)]` - Query registration

#### Registry System
- Compile-time component registration using `inventory` pattern
- Automatic graph construction from registered components
- Zero runtime overhead for registration
- `ComponentRegistry::build_graph()` - Build graph from all registered components
- `Registrable` trait for component metadata

#### DevX Showcase Traits
- `Describable` trait for human-readable component descriptions
- `Inspectable` trait for graph traversal and introspection
- Beautiful terminal output with emojis and formatting
- `pretty_print()` for instant visualization

#### Tutorial Series
- Tutorial 01: Hello Hex (5 minutes) - First component
- Tutorial 02: Adding Ports (10 minutes) - Port interfaces
- Tutorial 03: Implementing Adapters (15 minutes) - Adapters
- Tutorial 04: CQRS Basics (20 minutes) - Directives and queries
- Tutorial 05: Graph Analysis (15 minutes) - Introspection
- Tutorial 06: Production Ready (30 minutes) - Complete app

#### Examples
- `full_showcase.rs` - Comprehensive demonstration of all features
- Tutorial examples for each learning stage

#### Documentation
- ASCII art diagrams throughout documentation
- Visual architecture examples
- Before/after boilerplate comparisons
- Progressive learning path

### Technical Improvements
- Inventory-based compile-time registration
- Proc macro crate structure (`hexser_macros`)
- Attribute parsing infrastructure
- Metadata extraction utilities
- Validation with helpful error messages

### Developer Experience
- 90% reduction in boilerplate code
- Automatic graph construction
- Zero manual registration required
- Compile-time errors for misuse
- Beautiful terminal output
- Clear learning progression

### Breaking Changes
- None - Phase 3 is fully additive

---

## [0.2.0] - 2025-10-01

### Added - Phase 2: Graph Core

#### Graph Structure
- `HexGraph` - Immutable, thread-safe graph using Arc
- `HexNode` - Nodes representing architecture components
- `HexEdge` - Directed edges representing relationships
- `GraphBuilder` - Fluent API for constructing graphs
- `GraphMetadata` - Metadata for graphs, nodes, and edges
- `NodeId` - Type-based unique identification

#### Graph Operations
- Query nodes by layer (Domain, Port, Adapter, etc.)
- Query nodes by role (Entity, Repository, Adapter, etc.)
- Query edges from/to specific nodes
- Validate graph structure before building
- Thread-safe graph cloning and sharing

#### Testing & Examples
- Comprehensive graph integration tests
- Graph construction and validation tests
- Thread safety tests
- `graph_example` demonstrating graph usage

#### Documentation
- Complete documentation for all graph types
- Examples showing graph construction and queries
- Integration with Phase 1 traits and types

### Technical Details
- Zero-cost Arc-based sharing
- Immutable by design
- Lock-free reads
- Validation before construction
- HashMap-based node lookup (O(1))
- Vector-based edge storage

---
