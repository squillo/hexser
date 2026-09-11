# hexser graph, introspection and AI

How to inspect the architecture graph hexser builds from registered components, render it, and export it for AI agents. All types below are from the `graph`, `showcase`, and `ai` modules of hexser 0.5. MCP is out of scope here — see mcp.md.

Most graph types are in the prelude: `use hexser::prelude::*;` gives you `HexGraph`, `HexNode`, `HexEdge`, `NodeId`, `Layer`, `Role`, `Relationship`, `GraphBuilder`, `GraphMetadata`, plus the showcase traits `Describable`, `PrettyPrint`, `ArcGraphExt`, `Inspectable`. `Visualizable` is the one exception — it is NOT re-exported (full path below).

## The process-wide graph: `HexGraph::current()`

`HexGraph` is immutable and `Arc`-backed, so clones are cheap. The process-wide graph is built once from the link-time inventory registry (whatever your `#[derive(HexDomain/HexPort/HexAdapter/...)]` types submitted) and cached behind a wait-free `LazyLock<ArcSwap<HexGraph>>` (new in 0.5).

```rust
use hexser::prelude::*;

fn main() -> HexResult<()> {
    // Cheap Arc clone of the shared graph, lazily built on first call.
    let graph: std::sync::Arc<HexGraph> = HexGraph::current();
    graph.pretty_print(); // layer summary to stdout
    Ok(())
}
```

- `HexGraph::current() -> Arc<HexGraph>` — the shared graph (lazy, wait-free reads).
- `HexGraph::install(graph)` — hot-swap the current graph with your own.
- `HexGraph::rebuild_current()` — rebuild from the inventory registry and install the result.

Because `current()` sits behind `ArcSwap`, readers never block and never see a torn value; `install`/`rebuild_current` swap the `Arc` atomically. Note that inventory is fixed at link time, so `rebuild_current()` only reflects components compiled into the current binary.

## Querying the graph

`HexGraph` exposes read-only accessors (all take `&self`):

```rust
use hexser::prelude::*;

let graph = HexGraph::current();

// Counts and emptiness
let _ = graph.node_count();
let _ = graph.edge_count();
let _ = graph.layer_count();
let _ = graph.is_empty();

// Whole-graph iteration
for node in graph.nodes() { let _ = node; }
for edge in graph.edges() { let _ = edge; }

// Look up a single node
if let Some(node) = graph.get_node(&NodeId::of::<MyType>()) {
    let _ = node;
}

// Filter by layer / role
let domain_nodes = graph.nodes_by_layer(Layer::Domain);
let repos = graph.nodes_by_role(Role::Repository);

// Edges incident to a node
let id = NodeId::of::<MyType>();
let outgoing = graph.edges_from(&id);
let incoming = graph.edges_to(&id);

// Metadata
let _meta: &GraphMetadata = graph.metadata();
```

Also available: `HexGraph::new()` (empty graph) and `HexGraph::builder()` (see below).

`pretty_print(&self)` prints a per-layer summary to stdout — handy for a quick sanity check that your components registered.

## Building a graph by hand

Normally the graph is derived from registrations, but you can construct one directly with `GraphBuilder`:

```rust
use hexser::prelude::*;

let node = HexNode::new(
    NodeId::of::<MyType>(),
    Layer::Domain,
    Role::Entity,
    "MyType",                 // type_name
    "my_crate::domain",       // module_path
);

let graph = HexGraph::builder()
    .with_description("hand-built graph")
    .with_node(node)
    .build();
```

`GraphBuilder` methods: `with_node` / `add_node` / `with_nodes` / `with_edge` / `add_edge` / `with_description` / `build`.

`NodeId` constructors: `NodeId::of::<T>()`, `NodeId::from_name(&str)`, `NodeId::from_type_name(...)`, and `as_u64()` to read the underlying id.

## The layer / role / relationship enums

```rust
use hexser::prelude::*;

// Layer — which architectural layer a node belongs to
let _ = Layer::Domain;   // also: Port, Adapter, Application, Infrastructure, Unknown

// Role — 17 variants, the component's job
let _ = Role::Entity;
// full set: Entity, ValueObject, Aggregate, DomainEvent, DomainService,
// InputPort, OutputPort, Repository, UseCase, Query, Adapter, Mapper,
// Directive, DirectiveHandler, QueryHandler, Config, Unknown

// Relationship — edge kind between two nodes
let _ = Relationship::Implements;
// full set: Implements, Depends, Transforms, Aggregates, Invokes,
// Produces, Consumes, Validates, Configures, Unknown
```

`Layer` and `Role` both offer `as_str` / `Display`.

## Showcase traits (introspection helpers)

These live in `hexser::showcase`. `Describable`, `PrettyPrint`, `ArcGraphExt`, and `Inspectable` are in the prelude; `Visualizable` is not.

- `Describable` — `describe()`, `short_name()`, `category()`. Implemented for `HexNode`, `HexGraph`, and `Arc<HexGraph>`.
- `PrettyPrint` — `pretty_print()` on `Arc<HexGraph>`.
- `ArcGraphExt` — `nodes_by_layer(...)` / `nodes_by_role(...)` directly on `Arc<HexGraph>` (so you can query the result of `HexGraph::current()` without deref gymnastics).
- `Inspectable` — `layer_info()`, `dependencies()`, `dependents()`.

```rust
use hexser::prelude::*;

let graph = HexGraph::current(); // Arc<HexGraph>
let _ = graph.describe();                       // Describable
graph.pretty_print();                            // PrettyPrint on Arc<HexGraph>
let _ = graph.nodes_by_layer(Layer::Domain);     // ArcGraphExt on Arc<HexGraph>
```

`Visualizable` (full path `hexser::showcase::visualizable::Visualizable`) provides:
- `to_dot()` and `to_mermaid()` — return `HexResult<String>`; error if the `visualization` feature is off.
- `to_ascii_art() -> String` — always available, no feature required.

```rust
use hexser::prelude::*;
use hexser::showcase::visualizable::Visualizable; // not in prelude

let graph = HexGraph::current();
println!("{}", graph.to_ascii_art()); // always works
```

## Visualization exports (feature `visualization`)

Enable `visualization` for structured exports. These are inherent methods on `HexGraph`:

```rust
// Cargo.toml: hexser = { version = "0.6", features = ["visualization"] }
use hexser::prelude::*;

let graph = HexGraph::current();

let dot: String = graph.to_dot()?;      // HexResult<String>
let mmd: String = graph.to_mermaid()?;  // HexResult<String>
let json: String = graph.to_json()?;    // HexResult<String>

// Write a visualization to a path via a FormatExporter.
graph.save_visualization(path, exporter)?; // save_visualization(path, &dyn FormatExporter) -> HexResult<...>
```

The `visualization` feature pulls in `serde` + `serde_json`. Without it, `to_dot` / `to_mermaid` / `to_json` are unavailable and `Visualizable::to_dot` / `to_mermaid` return an error (only `to_ascii_art` still works).

## AI context (feature `ai`)

Enable `ai` (implies `serde` + `serde_json`) to turn the graph into an agent-readable context. Types live in `hexser::ai`; `AIContext` and `ContextBuilder` are in the prelude.

`AIContext` (`hexser::ai::AIContext`) fields: `architecture`, `version`, `components: Vec<ComponentInfo>`, `relationships`, `constraints`, `suggestions`, `metadata`. Related types: `ComponentInfo` (which now carries `methods: Vec<MethodInfo>`), `MethodInfo`, `ParameterInfo`, `RelationshipInfo`, `ConstraintSet`, `Suggestion`, `SuggestionType`, `Priority`, `ContextMetadata`.

```rust
// Cargo.toml: hexser = { version = "0.6", features = ["ai"] }
use hexser::prelude::*;

let graph = HexGraph::current();

// Two ways to build an AIContext:
let ctx: AIContext = graph.to_ai_context()?;             // inherent on HexGraph (feature ai)
let ctx: AIContext = ContextBuilder::new(&graph).build()?; // explicit builder

let json: String = ctx.to_json()?; // HexResult<String> in 0.5 (was Result<String, String>)
```

Note `AIContext::to_json` returns `HexResult<String>` in 0.5.

## Agent packs (feature `ai`)

`AgentPack` bundles the AI context with docs and guidelines for shipping to an agent. It is not in the prelude — reference it by path.

`AgentPack` (`hexser::ai::agent_pack::AgentPack`) fields: `schema_version`, `crate_name`, `crate_version`, `ai_context`, `guidelines`, `docs`.

```rust
use hexser::prelude::*;
use hexser::ai::agent_pack::AgentPack;

let graph = HexGraph::current();

// Quick path: sensible defaults straight from the graph.
let pack = AgentPack::from_graph_with_defaults(&graph)?; // HexResult<Self>

// Full control via the builder (new in 0.5).
let pack = AgentPack::builder()
    .with_docs(docs)             // DocBundle
    .with_guidelines(guidelines) // GuidelinesSnapshot
    .build(&graph)?;             // HexResult<AgentPack>

let json: String = pack.to_json()?; // HexResult<String> in 0.5
```

## AI export binaries (feature `ai`)

hexser ships two `ai`-gated bins that print JSON to stdout:

- `hex-ai-export` — prints the `AIContext` JSON.
- `hex-ai-pack` — prints the `AgentPack` JSON.

```bash
cargo run -p hexser --features ai --bin hex-ai-export > context.json
cargo run -p hexser --features ai --bin hex-ai-pack   > pack.json
```

Both require the `ai` feature. For the MCP server that serves these over JSON-RPC, see mcp.md.

## Feature-flag summary

- `visualization` (= `serde` + `serde_json`): `HexGraph::to_dot` / `to_mermaid` / `to_json` / `save_visualization`, and working `Visualizable::to_dot` / `to_mermaid`. `to_ascii_art` and all query methods need no feature.
- `ai` (= `serde` + `serde_json`): `HexGraph::to_ai_context`, `AIContext`, `ContextBuilder`, `AgentPack`, and the `hex-ai-export` / `hex-ai-pack` bins.
- Graph construction/querying (`HexGraph::current` / `install` / `rebuild_current`, counts, `nodes_by_layer` / `nodes_by_role`, `edges_from` / `edges_to`, `pretty_print`, `GraphBuilder`) works on default features.
