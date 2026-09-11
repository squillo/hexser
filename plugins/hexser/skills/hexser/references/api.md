# hexser API reference (0.5)

Condensed, accurate surface for hexser `0.6.0` (edition 2024, rust-version 1.85). Prefer `use hexser::prelude::*;`; use qualified paths only for items not in the prelude. For error constructors and codes see errors.md; for derives and registration see derives.md.

## 1. Prelude

`use hexser::prelude::*;` re-exports (defined inline in `hexser/src/lib.rs:135-174`):

- **Errors/result:** `HexResult`, `Hexserror`
- **Domain traits:** `Aggregate`, `DomainEvent`, `DomainService`, `HexEntity`, `HexValueItem`
- **Port traits/types:** `Direction`, `FindOptions`, `InputPort`, `OutputPort`, `Query`, `QueryRepository`, `Repository`, `Sort`, `UseCase`
- **Adapter traits:** `Adapter`, `Mapper`
- **Application traits:** `Application`, `Directive`, `DirectiveHandler`, `QueryHandler`
- **Infrastructure:** `Config`
- **Graph types:** `GraphBuilder`, `GraphMetadata`, `HexEdge`, `HexGraph`, `HexNode`, `Layer`, `NodeId`, `Relationship`, `Role`
- **Showcase:** `ArcGraphExt`, `Describable`, `Inspectable`, `PrettyPrint`
- **Registry:** `ComponentEntry`, `ComponentRegistry`, `NodeInfo`, `Registrable`
- **Derive macros** (feature `macros`, default on): `HexAdapter`, `HexAggregate`, `HexDirective`, `HexDomain`, `HexEntity`, `HexPort`, `HexQuery`, `HexRepository`, `HexValueItem`
- **Feature-gated:** `AIContext`, `ContextBuilder` (feature `ai`); `Container`, `Provider`, `Scope` (feature `container`); `StaticBuilder`, `StaticContainer` (feature `static-di`, default on)

**Not in the prelude** — at crate root, use qualified: `hexser::HexEntity`, `hexser::HexResult`, `hexser::Hexserror`, `hexser::inventory`, `hexser::error_codes`, and all derives (qualified). `Visualizable` is **not** re-exported — full path is `hexser::showcase::visualizable::Visualizable`.

## 2. Domain traits

`hexser/src/domain/`. In 0.5, `Entity`→`HexEntity` and `ValueObject`→`HexValueItem`; the old `Entity` alias was removed.

```rust
// HexEntity (domain/entity.rs:32) — associated identity type, no methods.
pub trait HexEntity {
    type Id;
}
// Manual impl:
impl HexEntity for User { type Id = String; }

// Aggregate (domain/aggregate.rs:48)
pub trait Aggregate: HexEntity {
    fn check_invariants(&self) -> HexResult<()>;
}

// HexValueItem (domain/value_object.rs:36)
pub trait HexValueItem {
    fn validate(&self) -> HexResult<()>;
}

// DomainEvent (domain/domain_event.rs:37)
pub trait DomainEvent {
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> String;
}

// DomainService (domain/domain_service.rs:39) — empty marker.
pub trait DomainService {}
```

`#[derive(HexEntity)]` requires a struct field literally named `id` and takes `type Id` from it. No `id` field (or a non-struct) is a compile error — there is no silent `String` default (`hexser_macros/src/derive/entity.rs:19-40`).

## 3. Ports

`hexser/src/ports/`. A custom port is a trait with a supertrait bound, e.g. `pub trait UserRepository: Repository<User> { fn find_by_email(&self, email: &str) -> HexResult<Option<User>>; }`.

```rust
// Repository<T> (ports/repository.rs:56) — SAVE-ONLY.
pub trait Repository<T> where T: HexEntity {
    fn save(&mut self, entity: T) -> HexResult<()>;
}

// QueryRepository<T> (ports/repository.rs:65)
pub trait QueryRepository<T> where T: HexEntity {
    type Filter;
    type SortKey;
    fn find_one(&self, filter: &Self::Filter) -> HexResult<Option<T>>;
    fn find(&self, filter: &Self::Filter, options: FindOptions<Self::SortKey>) -> HexResult<Vec<T>>;
    fn exists(&self, filter: &Self::Filter) -> HexResult<bool>;   // default via find_one
    fn count(&self, filter: &Self::Filter) -> HexResult<u64>;     // default via find
    fn delete_where(&mut self, filter: &Self::Filter) -> HexResult<u64>;
    // 0.5 DEFAULT for delete_where returns Err(port NOT_IMPLEMENTED = E_HEX_103)
    // (was Ok(0)). Override to support deletion.
}
```

```rust
// FindOptions<K> (repository.rs:17) — impl Default.
pub struct FindOptions<K> {
    pub sort: Option<Vec<Sort<K>>>,
    pub limit: Option<u32>,
    pub offset: Option<u64>,
}

// Sort<K> (repository.rs:40)
pub struct Sort<K> {
    pub key: K,
    pub direction: Direction,
}

// Direction (repository.rs:34) — Copy.
pub enum Direction { Asc, Desc }
```

```rust
// InputPort<Input, Output> (ports/input_port.rs:38)
pub trait InputPort<Input, Output> {
    fn execute(&self, input: Input) -> HexResult<Output>;
}

// OutputPort<Request, Response> (ports/output_port.rs:39)
pub trait OutputPort<Request, Response> {
    fn send(&self, request: Request) -> HexResult<Response>;
}

// UseCase<Input, Output> (ports/use_case.rs:38)
pub trait UseCase<Input, Output> {
    fn execute(&self, input: Input) -> HexResult<Output>;
}

// Query<Params, Result> (ports/query.rs:39) — READ-side Query lives in ports.
pub trait Query<Params, Result> {
    fn query(&self, params: Params) -> HexResult<Result>;
}
```

Feature-gated extras: CloudEvents ports (`ports/events/`): `CloudEventsEnvelope`, `EventPublisher`, `EventSubscriber`, `EventCodec`, `EventRouter`, `CLOUDEVENTS_SPEC_VERSION`. MCP `McpServer` port (feature `mcp`, `ports/mcp_server.rs`).

## 4. Application / CQRS

`hexser/src/application/`. Pattern (`examples/tutorial_04`): `#[derive(HexDirective)]` on the command struct plus a hand-written `impl DirectiveHandler<...>`; `#[derive(HexQuery)]` on the query struct plus a hand-written `impl QueryHandler<...>`.

```rust
// Directive (command) (application/directive.rs:38) — renamed from Command.
pub trait Directive {
    fn validate(&self) -> HexResult<()>;
}

// DirectiveHandler<D> (application/directive_handler.rs:46)
pub trait DirectiveHandler<D> where D: Directive {
    fn handle(&self, directive: D) -> HexResult<()>;
}

// QueryHandler<Q, R> (application/query_handler.rs:45)
pub trait QueryHandler<Q, R> {
    fn handle(&self, query: Q) -> HexResult<R>;
}

// Application (application/application.rs:86)
pub trait Application {
    fn name(&self) -> &str;                          // required
    fn initialize(&mut self) -> HexResult<()>;       // default no-op
    fn run(&mut self) -> HexResult<()>;              // default no-op
    fn shutdown(&mut self) -> HexResult<()>;         // default no-op
    fn execute(&mut self) -> HexResult<()>;          // provided: initialize -> run -> shutdown
}
```

## 5. Adapters

`hexser/src/adapters/`.

```rust
// Adapter (adapters/adapter.rs:40) — empty marker.
// #[derive(HexAdapter)] impls this + registers.
pub trait Adapter {}

// Mapper<From, To> (adapters/mapper.rs:57)
pub trait Mapper<From, To> {
    fn map(&self, from: From) -> HexResult<To>;
}
```

Provided adapters: `InMemoryEventBus` (always compiled), `McpStdioServer` (feature `mcp`). There is **no** generic `InMemoryRepository` — write an in-memory repo per entity. `#[derive(HexAdapter)]` defaults to `Role::Adapter`; override with `#[hex(role = "Mapper")]`.

## 6. HexResult

```rust
pub type HexResult<T> = Result<T, Hexserror>;
```

`Hexserror` (`error/hex_error.rs:47`) is an enum where every variant payload is `Box<…>`: `Domain(Box<DomainError>) | Port(Box<PortError>) | Adapter(Box<AdapterError>) | Validation(Box<ValidationError>) | NotFound(Box<NotFoundError>) | Conflict(Box<ConflictError>)`. It implements `Display` + `std::error::Error` (source chaining), and derives `Serialize`/`Deserialize` under feature `serde`. See errors.md for constructors, builders, and error codes.

## 7. Features

From `hexser/Cargo.toml`.

| Feature | Enables / implies | Notes |
| --- | --- | --- |
| `default` | `["macros", "static-di"]` | WASM-friendly (no tokio) |
| `macros` | `["hexser_macros"]` | derive macros |
| `serde` | `["dep:serde"]` | Serialize/Deserialize for rich errors |
| `ai` | `["serde", "serde_json"]` | `AIContext`, `AgentPack`, `ContextBuilder` |
| `mcp` | `["ai", "serde", "serde_json"]` | MCP server (implies `ai`) |
| `async` | `["tokio", "async-trait"]` | |
| `visualization` | `["serde", "serde_json"]` | DOT / Mermaid / JSON export |
| `container` | `["tokio", "async-trait"]` | dynamic (dyn) DI container |
| `static-di` | `[]` | zero-cost, WASM-friendly static DI (default) |
| `full` | `["ai","mcp","async","macros","visualization","container","static-di"]` | |

Bins: `hex-ai-export` and `hex-ai-pack` need `ai`; `hex-mcp-server` needs `mcp`. Default features are WASM-friendly — keep `container`/`async` off for wasm.

WASM: `wasm32-unknown-unknown` and `wasm32-wasip1` are verified by *running* hexser under wasmtime in CI (`scripts/wasm-e2e.sh`), not just building it. `inventory` registration works on both. `wasm32-unknown-unknown` has no std clock, so `GraphMetadata::created_at` is `0` and `ai` timestamps are the epoch; WASI reports real times. `save_visualization` (`std::fs`) errors under wasm — use `to_dot()`/`to_mermaid()`/`to_json()`.

DI: `static-di` (default) gives `StaticContainer<T>` (`get`/`get_mut`/`into_inner`/`map`), `StaticBuilder`, and the `hex_static!{…}` macro (`src/static_di.rs`). `container` gives the runtime `Container`, `Provider`, `Scope`, `ContainerError`, `AsyncProvider` (`src/container/mod.rs`).
