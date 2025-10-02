## [Unreleased]

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
- Proc macro crate structure (`hex_macros`)
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
