# SNAPP_CONTRACT — hexser

- **Snapp tier:** XS (cross-shared substrate — `hexser` IS the Port abstraction that PRD-272
  §1.4 mandates for every cross-Snapp call site; every other crate's boundary is expressed in
  terms of the traits this crate defines).

- **PRD-272 status:** conforming. The two exceptions this contract first recorded have landed
  (re-read 2026-11-17 for 0.7):
  - §1.5 / §3.G: `HexGraph::current()` reads a `LazyLock<arc_swap::ArcSwap<HexGraph>>`
    (`graph/hex_graph.rs`, `arch()`), and `HexGraph::install()` is the sole write site.
    `container/container.rs` holds its services in an
    `arc_swap::ArcSwap<indexmap::IndexMap<String, ServiceEntry>>` (feature `container`).
  - §3.H: the graph adjacency indices (`outgoing`/`incoming`) and `CloudEventsEnvelope`
    `extensions` are `indexmap::IndexMap`. Graph nodes are a `BTreeMap`.

- **Boundary ports (trait Ports exposed to other crates/Snapps):**
  - `crate::ports::repository::{Repository, QueryRepository}` — save-side and generic
    filter/sort/paginate read-side persistence ports (`ports/repository.rs`).
  - `crate::ports::events::{EventPublisher, EventSubscriber, EventCodec, EventRouter}` —
    CloudEvents v1.0-compliant transport-agnostic event ports, plus the
    `CloudEventsEnvelope<T>` wire type (`ports/events/mod.rs`, `ports/events/*.rs`).
  - `crate::ports::mcp_server::McpServer` (feature `mcp`) — `initialize`, `list_resources`,
    `read_resource`, `refresh_project`, `handle_request`; the Model Context Protocol JSON-RPC
    boundary adapters implement against (`ports/mcp_server.rs`).
  - `crate::ports::OutputPort`, and the deprecated `crate::ports::{InputPort, UseCase}`
    (routed to `Actor`) and `crate::ports::Query` (routed to `QueryHandler`). Deprecated in 0.7,
    not removed.
  - 0.7 doors (PRD HEXSER-0.7): `crate::application::Actor`, the single writer's door
    (`&mut self`, associated `Command`/`Context<'a>`/`Outcome`); `crate::domain::Vocabulary`
    and `crate::error::Refusal`, the closed vocabularies a command, event or refusal enum
    derives; `Hexserror::refused`/`Hexserror::code`, the one way a refusal crosses the edge.
    `Hexserror` keeps its six variants.
  - Graph introspection surface: `crate::graph::HexGraph` (`HexGraph::current()`,
    `nodes_by_layer`, `nodes_by_role`, `edges_from`/`edges_to`, DOT/Mermaid/JSON export under
    `visualization`) — not a trait Port, but the read surface every other Snapp/tool queries to
    introspect this process's architecture. 0.7 adds `HexGraph::findings()` (typed
    `GraphFinding`s), proved `Implements` edges (`registry::ImplementsEntry`,
    `templates::implements_edge_exists`), and `port_trait` on port nodes.

- **Wire format at the boundary:**
  - JSON at the **external** AI/MCP boundary: `AIContext`/`ContextBuilder` and `AgentPack`
    (feature `ai`) serialize via `serde_json` (`ai::ai_context::AIContext::to_json`,
    `ai::agent_pack::AgentPack::to_json`); the MCP server (feature `mcp`) speaks JSON-RPC
    (`domain::mcp::{JsonRpcRequest, JsonRpcResponse}`) per protocol mandate.
  - In-process trait dispatch internally — `Repository`/`QueryRepository`/`EventPublisher`/
    `EventSubscriber`/`McpServer` calls are direct Rust trait calls within a process, not
    serialized messages.
  - Postcard (PRD-272 §1.3, internal cross-Snapp message wire) explicitly does **not** apply
    here: the AIContext/AgentPack export and the MCP JSON-RPC surface are an **external** wire
    consumed by AI agents and MCP clients outside the Rust process/workspace, not an internal
    cross-Snapp message between Rust crates. JSON is the correct choice for both (human/AI
    readability for the former, protocol mandate for the latter).

- **Shared-read hot state (ArcSwap surfaces):** the `HexGraph` architecture graph via
  `HexGraph::current()` is the self-programming substrate — read on every hot path (MCP
  `read_resource`/`list_resources` via `ProjectRegistry::from_current_graph`, every AI-context
  export) and hot-swapped when the program rewrites itself (`hexser/refresh`). Landed
  (CLAIM-B1): `current()` is backed by
  `std::sync::LazyLock<arc_swap::ArcSwap<HexGraph>>`, a wait-free RCU surface per N_BOOK §1.5,
  read via `.load_full()` (unchanged `Arc<HexGraph>` return type) and hot-swapped by the sole
  write site `HexGraph::install(graph)` (with `HexGraph::rebuild_current()` rebuilding from the
  registry and installing).

- **Locks:** ArcSwap for the graph (landed, see above). The dyn `container` feature (feature
  `container`) uses `arc_swap::ArcSwap` for its services map plus a per-entry
  `tokio::sync::OnceCell` singleton cache — the latter is a documented carve-out (per-entry
  initialization, not a map-wide lock, never held across provider execution). No hot-path
  `RwLock`/`Mutex`/`OnceLock` outside these two; the `container` `RwLock` is gone. One further carve-out (0.7): `error/env_control.rs` reads
  `HEXSER_INCLUDE_SOURCE_LOCATION` once into a `std::sync::OnceLock<bool>`. It is process
  configuration fixed at start, never written again and never hot-swapped, so it is not
  shared-read hot state and ArcSwap does not apply. 0.7's `Actor` takes `&mut self` and adds no
  lock. The `RefCell`-based `adapters/in_memory_event_bus.rs` remains the documented
  single-threaded test/dev adapter (exempt as a non-hot-path, non-shared-state, single-threaded
  utility) — see N_BOOK Phase E (actor-grade path) for the multi-threaded successor.

- **Deterministic iteration:** `BTreeMap` for graph nodes; `indexmap::IndexMap` for graph
  adjacency (`outgoing`/`incoming`), `GraphMetadata::attributes`, the event-bus `handlers`
  registry, and `CloudEventsEnvelope::extensions` (all landed, CLAIM-A1/A2/A3; re-read
  2026-11-17). No `HashMap`/`HashSet` on iterated (export/serialize/report-order-sensitive)
  paths. 0.7's `HexGraph::findings()` reports in a fixed order: collisions, then adapters in
  node order, then edges in edge order.

- **N_BOOK grounding:** §1.5 (wait-free RCU — the `HexGraph::current()` ArcSwap surface), §10
  (Snapp boundary independence — every `ports::*` trait is a Port other Snapps/crates code
  against, never a concrete adapter type), §23 (Tier 2 SOLE write site — `HexGraph::install()`
  is the only place the live architecture graph is replaced).

_Authored 2026-07-21 under PRD-272 §1.1 (HEXSER-272-CONFORMANCE). Updated 2026-11-17 for 0.7
(PRD HEXSER-0.7): the landed status, the 0.7 doors, the deprecations, and the switch's
`OnceLock` carve-out._
