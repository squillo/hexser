---
name: hexser
description: >-
  Build, review, and migrate Rust code that uses hexser — a zero-boilerplate
  Hexagonal Architecture (Ports & Adapters) framework. Use this whenever the
  user writes or edits Rust that depends on the `hexser` crate: modeling domain
  entities/value objects/aggregates, defining Repository ports and port
  markers, implementing adapters that prove their port, writing single-writer
  Actors and QueryHandlers, closed vocabularies and refusals
  (HexVocabulary/HexRefusal), constructing rich Hexserror values,
  introspecting or visualizing the architecture graph (HexGraph, findings,
  DOT/Mermaid), wiring the hexser MCP server, or upgrading hexser (0.6 → 0.7,
  or 0.4 → 0.5). Also use it when a Cargo.toml lists `hexser`, or the user
  mentions hexser, HexEntity, HexDomain/HexPort/HexAdapter, HexActor, or
  hexagonal architecture in Rust.
license: MIT OR Apache-2.0
---

# hexser — Hexagonal Architecture for Rust

`hexser` (crates.io/crates/hexser, docs.rs/hexser) is a pragmatic, zero-boilerplate
take on Hexagonal Architecture for Rust: you write your **Domain**, describe the
world through **Ports**, and plug in **Adapters**. Derive macros register every
component into a compile-time **architecture graph** you can introspect, visualize,
and hand to AI agents. This skill makes you write hexser code that compiles and
follows the framework's grain on the first try.

**Always target hexser 0.7.x.** Add `hexser = { version = "0.7", features = ["macros"] }`
(the `macros` feature is on by default). Import the prelude: `use hexser::prelude::*;`.
`Actor`, `Vocabulary` and `Refusal` are at the crate root but NOT in the prelude. Name them
by path: `hexser::application::Actor`, `hexser::domain::Vocabulary`, `hexser::error::Refusal`.
Their derives (`HexActor`, `HexVocabulary`, `HexRefusal`) are in the prelude. Every 0.7 door
runs on `wasm32-unknown-unknown` and `wasm32-wasip1` with `features = ["macros"]` only.

Upgrading from 0.6? Follow `MIGRATING_0.6_TO_0.7.md` in the hexser repository
(<https://github.com/squillo/hexser/blob/main/hexser/MIGRATING_0.6_TO_0.7.md>), or run
`/hexser:migrate`. 0.7 adds and deprecates. It deletes nothing.

## Mental model

```
Domain  ── entities, value objects, aggregates, domain events, domain services,
  ▲         closed vocabularies (HexVocabulary) and refusals (HexRefusal)
Ports   ── traits describing what the app needs: Repository, QueryRepository,
  │         OutputPort, custom port traits. A port MARKER names its trait (port_trait).
Adapters ─ concrete impls of ports (DB, HTTP, in-memory, …). Each proves the port it
            serves with implements(..).
Application ─ Actor (the single writer: &mut self, command in, outcome out),
              QueryHandler (reads)
```

Deriving `HexDomain`/`HexPort`/`HexAdapter`/`HexDirective`/`HexQuery`/`HexActor` registers
the type into the process-wide graph (`HexGraph::current()`) via the `inventory` crate —
zero runtime cost, discovered at link time. For a type you cannot annotate (or a build
without the `macros` feature), `hexser::hex_register_domain!` and its siblings do the same.
Edges ride on the registration: `#[hex(depends_on(..))]` lands a `Depends` edge, and
`#[hex(implements(..))]` on an adapter lands a proved `Implements` edge.
`HexGraph::findings()` says what is wrong with the graph.

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
   brings the traits, types, and derives you need. The exceptions are `Actor`, `Vocabulary`,
   `Refusal` and `GraphFinding`: name those by path.
7. **Custom ports extend base ports:** `trait UserRepository: Repository<User> { … }`.
8. **`HexResult<T> = Result<T, Hexserror>`.** Port and query methods return `HexResult`.
   An actor's outcome is its own (rule 11).
9. **Set the role deliberately: `#[hex(role = "…")]` on any of the six registration
   derives.** Defaults are `Entity`/`Repository`/`Adapter`/`Directive`/`Query`, and
   `DirectiveHandler` for `HexActor`. Before
   2026-09-04 `HexDomain` declared the attribute and IGNORED it, so a value object marked
   `ValueObject` registered as `Role::Entity` and compiled clean — if you are reading a graph
   built by an older hexser, its domain roles are all `Entity`.
10. **A registration derive on a GENERIC type is a compile error.** `inventory` cannot name a
    single honest node for `Foo<T>`. Derive on a non-generic `FooComponent` marker struct, or
    hand-write `impl Registrable`. Before 2026-09-04 the derive dropped the submission in
    silence: the type implemented `Registrable`, answered `node_info()`, and was in no graph.
    ⭐ **Implementing `Registrable` and being IN the graph are two different facts** — the
    `inventory::submit!` is the second one, and it is what `HexGraph::current()` walks.
11. **Writes go through `Actor`.** `fn handle(&mut self, command, context) -> Self::Outcome`,
    with associated `Command`, `Context<'a>` and `Outcome`. `&mut self` is the single
    writer, so no lock and no `RefCell`. The outcome is the actor's own: an event, a view,
    `Result<Event, Refusal>`, or `()`. `#[derive(HexActor)]` registers its node; it emits
    no `Actor` impl. Reads go through `QueryHandler`.
12. **Do not use the four deprecated doors.** `DirectiveHandler`, `ports::UseCase` and
    `ports::InputPort` route to `Actor`. `ports::Query` routes to `QueryHandler`. They still
    compile in 0.7 and warn.
13. **Closed sets are closed enums.** Derive `HexVocabulary` for a command, event or name
    list (`CODES`, `code()`, `from_code`). Derive `HexRefusal` for a refusal, with
    `#[hex(next_step = "…", suggestion = "…")]` per variant. Do not keep a name list by
    hand. Do not write a refusal as a string code.
14. **Words at the edge only.** A refusal stays a typed value inside the crate. It crosses
    into `Hexserror` only at the edge, through `Hexserror::refused(&refusal, message)`,
    which builds the existing `Domain` variant. `err.code()` and `R::from_code` read it back.
15. **Port markers name their trait; adapters prove it.** `#[hex(port_trait = Trait)]` on a
    `HexPort` marker. `#[hex(implements(PortNode))]` on the `HexAdapter`. That compiles
    only if the adapter implements the trait. Do not name the served port with `depends_on`:
    that edge is declared, not proved. A `Mapper` serves no port and needs none.
16. **`#[hex(...)]` is strict.** An unknown key, a malformed value, a duplicate
    single-valued key, or a key the derive does not read is a compile error.
17. **Keep a graph cell.** One test asserts
    `assert_eq!(graph.findings(), Vec::<hexser::GraphFinding>::new())`.
18. **No `unsafe`.** All three hexser crates are `#![forbid(unsafe_code)]`.
    `HEXSER_INCLUDE_SOURCE_LOCATION` is read once per process, so set it before the process
    starts. Never write it at run time.

## Minimal correct shape (0.7)

```rust
use hexser::prelude::*;

#[derive(HexDomain, HexEntity, Clone, Debug)]
struct User { id: String, email: String }   // `id` field is mandatory for HexEntity

// The port trait, and its marker node in the graph.
pub trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

#[derive(HexPort)]
#[hex(port_trait = UserRepository)]
struct UserRepositoryPortNode;

// The adapter proves it implements the port's trait.
#[derive(HexAdapter, Default)]
#[hex(implements(UserRepositoryPortNode))]
struct InMemoryUsers { users: Vec<User> }

impl Repository<User> for InMemoryUsers {
    fn save(&mut self, u: User) -> HexResult<()> { self.users.push(u); Ok(()) }
}
impl UserRepository for InMemoryUsers {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| u.email == email).cloned())
    }
}
// … plus `impl QueryRepository<User>` (Filter/SortKey, find_one/find/delete_where).
// See references/patterns.md for the full repository.

// A closed refusal with static guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, HexRefusal)]
enum ChangeEmailRefusal {
    #[hex(next_step = "Type an email address")]
    EmptyEmail,
}

// The single writer: it owns its state and decides one command at a time.
struct Profile { user: User }

impl hexser::application::Actor for Profile {
    type Command = String;
    type Context<'a> = ();
    type Outcome = Result<String, ChangeEmailRefusal>;

    fn handle(&mut self, email: String, _context: ()) -> Self::Outcome {
        if email.is_empty() {
            return Err(ChangeEmailRefusal::EmptyEmail);
        }
        self.user.email = email.clone();
        Ok(email)
    }
}

// The actor's node in the graph: Application layer, role `DirectiveHandler`.
#[derive(HexActor)]
struct ProfileActorNode;

// At the edge, the refusal crosses into `Hexserror` with its code and guidance.
fn change_email(profile: &mut Profile, email: String) -> HexResult<String> {
    hexser::application::Actor::handle(profile, email, ())
        .map_err(|refusal| Hexserror::refused(&refusal, "The email was not changed"))
}

/// why: no collision, no unproved adapter, and no dangling edge.
#[test]
fn the_architecture_graph_has_no_findings() {
    let graph = hexser::registry::ComponentRegistry::build_graph();
    assert_eq!(graph.findings(), Vec::<hexser::GraphFinding>::new());
}
```

## How to use this skill

Consult the reference file that matches the task (they are the fact-checked, current
API — trust them over memory, and over the stale workspace-root README):

- **`references/patterns.md`** — the canonical Domain → Port → Adapter → use-case build,
  with a full compiling end-to-end example. Start here to build anything. Its write side
  still shows the 0.6 `DirectiveHandler`, which is deprecated. Write an `Actor` instead
  (rule 11), and add `port_trait`/`implements(..)` (rule 15).
- **`references/api.md`** — condensed, accurate API surface: prelude exports, every core
  trait signature (including `Actor`, `Vocabulary`, `Refusal`), the deprecated doors,
  `FindOptions`/`Sort`/`Direction`, features table.
- **`references/derives.md`** — what each derive (`HexDomain`, `HexPort`, `HexAdapter`,
  `HexRepository`, `HexDirective`, `HexQuery`, `HexActor`, `HexEntity`, `HexValueItem`,
  `HexAggregate`, `HexVocabulary`, `HexRefusal`) generates, the `#[hex(...)]` keys each
  reads, and the generics/`id` gotchas.
- **`references/errors.md`** — `Hexserror`, layer errors, error codes, constructor helpers,
  rich-error builders, and refusals (`Hexserror::refused`, `Hexserror::code`).
- **`references/graph-and-ai.md`** — `HexGraph::current()`, `findings()`/`GraphFinding`,
  `implements_edge_exists`, `pretty_print`, DOT/Mermaid/JSON export,
  `Describable`/`Inspectable`/`Visualizable`, and `AIContext`/`AgentPack`.
- **`references/mcp.md`** — run/wire the hexser MCP server so Claude can read the live
  architecture graph; what resources & methods it exposes.
- **`references/migration-0.4-to-0.5.md`** — every 0.4 → 0.5 breaking change and its
  fix (boxed errors, `HexEntity` needs `id`, `to_json` → `HexResult`, `delete_where`).
- **`MIGRATING_0.6_TO_0.7.md`** in the hexser repository — the 0.6 → 0.7 guide. Every
  example in it is compiled as a doctest. `/hexser:migrate` carries its steps.

Companion slash commands: `/hexser:new`, `/hexser:add-adapter`, `/hexser:add-usecase`,
`/hexser:review`, `/hexser:graph`, `/hexser:migrate`, `/hexser:mcp-setup`.

## When writing hexser code

- Reach for `use hexser::prelude::*;` first; only use qualified paths for items not in
  the prelude (e.g. `hexser::ports::repository::QueryRepository` when disambiguating).
- Name domain types by role; derive the matching macro so they register in the graph.
- Give each port marker its `port_trait`, and each adapter its `implements(..)`.
- Write the write side as an `Actor`, and the read side as a `QueryHandler`.
- Keep refusals as `HexRefusal` values; cross into `Hexserror` with `refused` at the edge.
- Return `HexResult` and attach `.with_next_step`/`.with_suggestion` to errors.
- After building, suggest `HexGraph::current().pretty_print()` (or `/hexser:graph`) so
  the user can see the architecture the code just described.
- Verify with `cargo check`; if entities fail to compile, first check the `id`-field rule.
