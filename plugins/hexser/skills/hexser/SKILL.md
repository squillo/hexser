---
name: hexser
description: >-
  Build, review, and migrate Rust code that uses hexser — a zero-boilerplate
  Hexagonal Architecture (Ports & Adapters) framework. Use this whenever the
  user writes or edits Rust that depends on the `hexser` crate: modeling domain
  entities/value objects/aggregates, defining Repository/Query ports,
  implementing adapters, writing CQRS Directives and Queries, constructing rich
  Hexserror values, introspecting or visualizing the architecture graph
  (HexGraph, DOT/Mermaid), wiring the hexser MCP server, or upgrading hexser
  (e.g. 0.4 → 0.5). Also use it when a Cargo.toml lists `hexser`, or the user
  mentions hexser, HexEntity, HexDomain/HexPort/HexAdapter, or hexagonal
  architecture in Rust.
license: MIT OR Apache-2.0
---

# hexser — Hexagonal Architecture for Rust

`hexser` (crates.io/crates/hexser, docs.rs/hexser) is a pragmatic, zero-boilerplate
take on Hexagonal Architecture for Rust: you write your **Domain**, describe the
world through **Ports**, and plug in **Adapters**. Derive macros register every
component into a compile-time **architecture graph** you can introspect, visualize,
and hand to AI agents. This skill makes you write hexser code that compiles and
follows the framework's grain on the first try.

**Always target hexser 0.5.x.** Add `hexser = { version = "0.5", features = ["macros"] }`
(the `macros` feature is on by default). Import the prelude: `use hexser::prelude::*;`.

## Mental model

```
Domain  ── entities, value objects, aggregates, domain events, domain services
  ▲
Ports   ── traits describing what the app needs: Repository, QueryRepository,
  │         InputPort/OutputPort, UseCase, Query, custom port traits
Adapters ─ concrete impls of ports (DB, HTTP, in-memory, …)
Application ─ CQRS: Directive (command) + DirectiveHandler, QueryHandler
```

Deriving `HexDomain`/`HexPort`/`HexAdapter`/`HexDirective`/`HexQuery` registers the
type into the process-wide graph (`HexGraph::current()`) via the `inventory` crate —
zero runtime cost, discovered at link time.

## Golden rules (these prevent ~all first-try mistakes)

1. **`#[derive(HexEntity)]` REQUIRES a struct field literally named `id`.** The macro
   takes `type Id` from that field's type. No `id` field (or a non-struct) is a
   **compile error** in 0.5 — there is no silent `Id = String` default anymore. For a
   type that isn't really an entity, don't derive `HexEntity`. To choose a different
   identity, hand-write `impl hexser::HexEntity for T { type Id = …; }`.
2. **`Repository<T>` is save-only** (`fn save(&mut self, T) -> HexResult<()>`). Reads,
   counts, and deletes live on **`QueryRepository<T>`** (associated `Filter` + `SortKey`;
   `find_one`, `find`, `exists`, `count`, `delete_where`). A working repository adapter
   implements **both**.
3. **Override `delete_where` if you support deletion.** Its 0.5 default returns
   `Err(E_HEX_103 / port::NOT_IMPLEMENTED)`, not `Ok(0)` — so an unimplemented delete
   can't masquerade as success.
4. **`Hexserror` variants are boxed** in 0.5 (`Adapter(Box<AdapterError>)`, …). Prefer
   the constructors — `Hexserror::adapter(code, msg)`, `::domain`, `::port`,
   `::validation`, `::validation_field(msg, field)`, `::not_found(resource, id)`,
   `::conflict(msg)` — which box for you. If you construct a variant directly from a
   layer error, wrap it: `Hexserror::Adapter(std::boxed::Box::new(err))`. Matching
   (`Hexserror::Adapter(inner)`) needs no change.
5. **Add guidance to errors.** `.with_next_step("…")` and `.with_suggestion("…")` work
   on every variant; `.with_source(err)` chains a cause on Domain/Port/Adapter. This
   is hexser's whole point — errors that tell you what to do next.
6. **Prefer the prelude and constructor helpers** over deep paths. `use hexser::prelude::*;`
   brings the traits, types, and derives you need.
7. **Custom ports extend base ports:** `trait UserRepository: Repository<User> { … }`.
8. **`HexResult<T> = Result<T, Hexserror>`.** Application/port methods return `HexResult`.

## Minimal correct shape (0.5)

```rust
use hexser::prelude::*;

#[derive(HexDomain, HexEntity, Clone, Debug)]
struct User { id: String, email: String }   // `id` field is mandatory for HexEntity

trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

#[derive(HexAdapter, Default)]
struct InMemoryUsers { users: Vec<User> }

impl Repository<User> for InMemoryUsers {
    fn save(&mut self, u: User) -> HexResult<()> { self.users.push(u); Ok(()) }
}
// … plus `impl QueryRepository<User>` (Filter/SortKey, find_one/find/delete_where)
// … plus `impl UserRepository`. See references/patterns.md for the full, compiling file.
```

## How to use this skill

Consult the reference file that matches the task (they are the fact-checked, current
API — trust them over memory, and over the stale workspace-root README):

- **`references/patterns.md`** — the canonical Domain → Port → Adapter → use-case build,
  with a full compiling end-to-end example. Start here to build anything.
- **`references/api.md`** — condensed, accurate API surface: prelude exports, every core
  trait signature, `FindOptions`/`Sort`/`Direction`, features table.
- **`references/derives.md`** — what each derive (`HexDomain`, `HexPort`, `HexAdapter`,
  `HexRepository`, `HexDirective`, `HexQuery`, `HexEntity`, `HexValueItem`, `HexAggregate`)
  generates, its `#[hex(role = "…")]` options, and the generics/`id` gotchas.
- **`references/errors.md`** — `Hexserror`, layer errors, error codes, constructor helpers,
  and rich-error builders, with real construction examples.
- **`references/graph-and-ai.md`** — `HexGraph::current()`, `pretty_print`, DOT/Mermaid/JSON
  export, `Describable`/`Inspectable`/`Visualizable`, and `AIContext`/`AgentPack`.
- **`references/mcp.md`** — run/wire the hexser MCP server so Claude can read the live
  architecture graph; what resources & methods it exposes.
- **`references/migration-0.4-to-0.5.md`** — every 0.4 → 0.5 breaking change and its
  fix (boxed errors, `HexEntity` needs `id`, `to_json` → `HexResult`, `delete_where`).

Companion slash commands: `/hexser:new`, `/hexser:add-adapter`, `/hexser:add-usecase`,
`/hexser:review`, `/hexser:graph`, `/hexser:migrate`, `/hexser:mcp-setup`.

## When writing hexser code

- Reach for `use hexser::prelude::*;` first; only use qualified paths for items not in
  the prelude (e.g. `hexser::ports::repository::QueryRepository` when disambiguating).
- Name domain types by role; derive the matching macro so they register in the graph.
- Return `HexResult` and attach `.with_next_step`/`.with_suggestion` to errors.
- After building, suggest `HexGraph::current().pretty_print()` (or `/hexser:graph`) so
  the user can see the architecture the code just described.
- Verify with `cargo check`; if entities fail to compile, first check the `id`-field rule.
