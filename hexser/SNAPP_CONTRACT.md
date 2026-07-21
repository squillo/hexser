# SNAPP_CONTRACT — hexser

- **Snapp tier:** XS (cross-shared substrate — `hexser` IS the Port abstraction that PRD-272
  §1.4 mandates for every cross-Snapp call site; every other crate's boundary is expressed in
  terms of the traits this crate defines).

- **PRD-272 status:** conforming as of 2026-07-21, with two exceptions tracked and landing under
  the same PRD (not yet merged at the time this contract was authored):
  - §1.5 / §3.G: `HexGraph::current()` (`graph/hex_graph.rs`) still caches behind
    `std::sync::OnceLock<Arc<HexGraph>>`. CLAIM-B1 replaces it with
    `std::sync::LazyLock<arc_swap::ArcSwap<HexGraph>>` plus `HexGraph::install()` /
    `HexGraph::rebuild_current()` as the sole write site. `container/container.rs`'s
    `tokio::sync::RwLock<HashMap<String, ServiceEntry>>` (feature `container`) is likewise
    slated (CLAIM-B2) to become `arc_swap::ArcSwap<indexmap::IndexMap<String, ServiceEntry>>`
    with CAS-retry writes and wait-free reads; its per-entry `tokio::sync::OnceCell` singleton
    cache is a documented carve-out (per-entry init, not a map-wide lock) and is not affected.
  - §3.H: the graph adjacency indices (`outgoing`/`incoming` in `GraphInner`), and other
    iterated maps (event-bus handler registry, CloudEvents envelope `extensions`) are mid-flight
    HashMap → `indexmap::IndexMap` conversions under the same PRD (CLAIM-A1/A2/A3). Graph
    *nodes* already use `BTreeMap` (deterministic).
  - `arc_swap` and `indexmap` are already core (non-optional) dependencies of this crate
    (`Cargo.toml`), added ahead of the above landing.

- **Boundary ports (trait Ports exposed to other crates/Snapps):**
  - `crate::ports::repository::{Repository, QueryRepository}` — save-side and generic
    filter/sort/paginate read-side persistence ports (`ports/repository.rs`).
  - `crate::ports::events::{EventPublisher, EventSubscriber, EventCodec, EventRouter}` —
    CloudEvents v1.0-compliant transport-agnostic event ports, plus the
    `CloudEventsEnvelope<T>` wire type (`ports/events/mod.rs`, `ports/events/*.rs`).
  - `crate::ports::mcp_server::McpServer` (feature `mcp`) — `initialize`, `list_resources`,
    `read_resource`, `refresh_project`, `handle_request`; the Model Context Protocol JSON-RPC
    boundary adapters implement against (`ports/mcp_server.rs`).
  - `crate::ports::{InputPort, OutputPort, Query, UseCase}` — the remaining Phase-1 port
    traits (input/output boundary and CQRS query shape).
  - Graph introspection surface: `crate::graph::HexGraph` (`HexGraph::current()`,
    `nodes_by_layer`, `nodes_by_role`, `edges_from`/`edges_to`, DOT/Mermaid/JSON export under
    `visualization`) — not a trait Port, but the read surface every other Snapp/tool queries to
    introspect this process's architecture.

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
  export) and hot-swapped when the program rewrites itself (`hexser/refresh`). Landing state
  (CLAIM-B1, this PRD): `current()` backed by
  `std::sync::LazyLock<arc_swap::ArcSwap<HexGraph>>`, a wait-free RCU surface per N_BOOK §1.5,
  read via `.load_full()` (unchanged `Arc<HexGraph>` return type) and hot-swapped by the sole
  write site `HexGraph::install(graph)` (with `HexGraph::rebuild_current()` rebuilding from the
  registry and installing). At the time of authoring, `current()` is still `OnceLock`-cached
  (write-once); this section describes the intended/landing state per this PRD.

- **Locks:** ArcSwap for the graph (landing, see above). The dyn `container` feature (feature
  `container`) uses (landing) `arc_swap::ArcSwap` for its services map plus a per-entry
  `tokio::sync::OnceCell` singleton cache — the latter is a documented carve-out (per-entry
  initialization, not a map-wide lock, never held across provider execution). No hot-path
  `RwLock`/`Mutex`/`OnceLock` outside these two, and the `container` `RwLock` is being removed
  under this PRD. The `RefCell`-based `adapters/in_memory_event_bus.rs` remains the documented
  single-threaded test/dev adapter (exempt as a non-hot-path, non-shared-state, single-threaded
  utility) — see N_BOOK Phase E (actor-grade path) for the multi-threaded successor.

- **Deterministic iteration:** `BTreeMap` for graph nodes (already landed); `indexmap::IndexMap`
  for graph adjacency (`outgoing`/`incoming`), `GraphMetadata::attributes`, the event-bus
  `handlers` registry, and `CloudEventsEnvelope::extensions` (all landing under this PRD's
  CLAIM-A1/A2/A3). No `HashMap`/`HashSet` on iterated (export/serialize/report-order-sensitive)
  paths once landed.

- **N_BOOK grounding:** §1.5 (wait-free RCU — the `HexGraph::current()` ArcSwap surface), §10
  (Snapp boundary independence — every `ports::*` trait is a Port other Snapps/crates code
  against, never a concrete adapter type), §23 (Tier 2 SOLE write site — `HexGraph::install()`
  is the only place the live architecture graph is replaced).

_Authored 2026-07-21 under PRD-272 §1.1 (HEXSER-272-CONFORMANCE)._
