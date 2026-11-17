---
description: Migrate a crate to a newer hexser (0.6 → 0.7, or the older 0.4 → 0.5)
argument-hint: "[path (optional)] [0.7|0.5 (optional)]"
allowed-tools: Read, Edit, Write, Bash(cargo *), Grep, Glob
---

Migrate a Rust crate to a newer hexser. There are two paths: 0.6 → 0.7, and the older
0.4 → 0.5.

Work carefully, file-by-file. Read each file before editing it. Never run git.

## Choose the path

1. Read the arguments. An argument of `0.7` or `0.5` names the target. Any other argument is
   the crate path. In the steps below, `$1` and `$ARGUMENTS` mean that path, not the version.
   With no path, work on the current crate.
2. With no target given, read the `hexser` pin in each `Cargo.toml` and pick a path:
   - `0.6` or `0.6.x`: take the 0.6 → 0.7 path.
   - `0.4.x`: take the older 0.4 → 0.5 path. It ends on a `0.6` pin. Then take the
     0.6 → 0.7 path.
   - `0.5.x`: apply the 0.6 note in Step 1 of the older path. Then take the 0.6 → 0.7 path.
   - `0.7`: the pin is already moved. Run steps 2 to 9 of the 0.6 → 0.7 path as checks.

## The 0.6 → 0.7 path

The source of truth is the hexser guide `MIGRATING_0.6_TO_0.7.md`, at
`hexser/MIGRATING_0.6_TO_0.7.md` in the hexser repository
(<https://github.com/squillo/hexser/blob/main/hexser/MIGRATING_0.6_TO_0.7.md>). It has the full
before and after for each step below. Every example in it is compiled as a doctest. Copy API
shapes from it or from this command. Do not invent API.

0.7 adds and deprecates. It deletes nothing. A crate that builds on 0.6 builds on 0.7, with
three exceptions:

- an `#[hex(...)]` key hexser never read is now a compile error (Step 6);
- uses of four duplicate traits print deprecation warnings (Step 2);
- `HEXSER_INCLUDE_SOURCE_LOCATION` is read once per process (Step 7).

Locate the crate first. Use Glob to find `Cargo.toml` and `**/*.rs`, and Grep for the change
sites named in each step.

### Step 1: bump the pin

`cargo update` does not move a `0.6` pin to 0.7. Edit each `Cargo.toml` that depends on
hexser. Change the version only. Keep the existing feature list and any
`default-features = false`.

```toml
hexser = { version = "0.7", features = ["macros"] }
```

Every 0.7 door runs on `wasm32-unknown-unknown` and `wasm32-wasip1` with the `macros` feature
only. A wasm crate can pin:

```toml
hexser = { version = "0.7", default-features = false, features = ["macros"] }
```

Then run `cargo check`. Its deprecation warnings and `#[hex(...)]` errors are the worklist for
steps 2 and 6.

### Step 2: move off the four deprecated traits

Each still compiles and works. Each use prints a warning that names its route. None is removed
in 0.7.

| Deprecated | Route to |
|---|---|
| `hexser::application::DirectiveHandler` | `hexser::application::Actor` |
| `hexser::ports::UseCase` | `hexser::application::Actor` |
| `hexser::ports::InputPort` | `hexser::application::Actor` |
| `hexser::ports::Query` | `hexser::application::QueryHandler` (the same shape) |

Grep for `DirectiveHandler`, `UseCase`, `InputPort`, `Query<` and `fn query(`. They may arrive
through `use hexser::prelude::*;`. `HexQuery`, `QueryHandler`, `QueryRepository` and
`OutputPort` are not deprecated. Leave them.

A handler or a use case becomes an `Actor`:

```rust
// before (0.6)
impl hexser::application::DirectiveHandler<Rename> for Deck {
  fn handle(&self, directive: Rename) -> hexser::HexResult<()> { /* ... */ }
}

// after (0.7)
impl hexser::application::Actor for Deck {
  type Command = Rename;
  type Context<'a> = ();
  type Outcome = hexser::HexResult<()>;

  fn handle(&mut self, command: Rename, _context: ()) -> Self::Outcome { /* ... */ }
}
```

- `handle` takes `&mut self`. That is the single-writer guarantee, so the actor needs no lock.
  Remove a `RefCell` or `Mutex` that existed only to write through `&self`. Callers now hold
  `&mut`.
- For `UseCase<I, O>` or `InputPort<I, O>`, set `type Command = I` and
  `type Outcome = hexser::HexResult<O>`. Move the body of `execute` into `handle`.
- `type Outcome = hexser::HexResult<..>` is the mechanical move. The trait imposes no `Result`
  and no `Hexserror`. The outcome can be an event, a view, `Result<Event, Refusal>`, or `()`.
  Step 4 shows a refusal outcome.
- A context that borrows per call is a generic associated type, for example
  `type Context<'a> = &'a Clock;`.
- To put the actor in the graph, add `#[derive(hexser::HexActor)]`. It registers an
  Application node with role `DirectiveHandler`. It refuses a generic type, so a generic actor
  gets a non-generic marker struct instead.

A `Query` becomes a `QueryHandler`. Rename `query` to `handle`:

```rust
// before (0.6)
impl hexser::ports::Query<FindByEmail, Option<UserView>> for UserReader {
  fn query(&self, params: FindByEmail) -> hexser::HexResult<Option<UserView>> { /* ... */ }
}

// after (0.7)
impl hexser::application::QueryHandler<FindByEmail, Option<UserView>> for UserReader {
  fn handle(&self, params: FindByEmail) -> hexser::HexResult<Option<UserView>> { /* ... */ }
}
```

Update supertrait bounds and call sites the same way. To keep a 0.6 call site quiet while you
move it, put `#[allow(deprecated)]` on the item. Remove it when the move is done.

### Step 3: replace hand-written name lists with `HexVocabulary`

A command, an event and a refusal are each a closed enum. Each value's identity is its
variant's name. Grep for name lists kept by hand next to an enum: `const NAMES`, `const ALL`,
`fn name(&self) -> &'static str`, `fn as_str(&self)`, and `match` arms that map strings to
variants.

```rust
// before
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ask { Open, Close }

impl Ask {
  const NAMES: &'static [&'static str] = &["Open", "Close"];
  fn name(&self) -> &'static str {
    match self { Ask::Open => "Open", Ask::Close => "Close" }
  }
}

// after
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexVocabulary)]
enum Ask {
  Open,
  #[hex(code = "ASK_CLOSE")]
  Close,
}
// <Ask as hexser::domain::Vocabulary>::CODES == &["Open", "ASK_CLOSE"]
// hexser::domain::Vocabulary::code(&Ask::Close) == "ASK_CLOSE"
// Ask::from_code("ASK_CLOSE") == Some(Ask::Close); Ask::from_code("Close") == None
```

- Delete the hand-written list and its methods. Point callers at `CODES`, `code()` and
  `from_code`.
- `CODES` lists the codes in declaration order. If the old list had another order, check its
  readers.
- Give a variant `#[hex(code = "...")]` when its old string differs from its name. The wire
  code then stays stable.
- `from_code` is generated only when every variant is a unit variant. An unknown code reads
  back as `None`, never as a guessed variant.
- Two variants with the same code are a compile error.
- The derive writes no `Display` and no `Default`.
- For a refusal enum, derive `HexRefusal` instead (Step 4). It implements the vocabulary too,
  so do not derive both.

### Step 4: turn string refusals into `HexRefusal`

Grep for `Hexserror::domain(` called with a literal code, and the `.with_next_step(` and
`.with_suggestion(` chained onto it.

```rust
// before: the code and the guidance are strings at the call site
let err = hexser::Hexserror::domain("EmptyName", "The deck needs a name")
  .with_next_step("Type a name");

// after
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum EditRefusal {
  #[hex(next_step = "Type a name", suggestion = "A name is 1 to 64 characters")]
  EmptyName,
  #[hex(code = "E_NAME_TOO_LONG")]
  NameTooLong,
}

let err = hexser::Hexserror::refused(&EditRefusal::EmptyName, "The deck needs a name");
assert!(matches!(err, hexser::Hexserror::Domain(_)));
assert_eq!(err.code(), "EmptyName");
assert_eq!(EditRefusal::from_code(err.code()), Some(EditRefusal::EmptyName));
```

- Name each variant after its old code, or keep the old string with `#[hex(code = "...")]`.
- Move static guidance into `#[hex(next_step = "...")]` and `#[hex(suggestion = "...")]`.
  Each may be written more than once on a variant.
- Inside the crate, return the refusal as a value: `Result<Event, EditRefusal>`, or an actor's
  outcome.
- At the edge, cross with `hexser::Hexserror::refused(&refusal, "message")`. It builds the
  existing `Domain` variant and copies the code and the guidance. `Hexserror` keeps its six
  variants, so an exhaustive match still compiles.
- `Hexserror::code()` answers for all six variants. Read a refusal back with
  `EditRefusal::from_code(err.code())`, not by matching on error text.

### Step 5: give port markers their trait, and let adapters prove it

A port marker without `port_trait` can outlive its trait. An adapter's `depends_on` onto a
port marker is declared, not proved.

Grep for `HexPort` and `hex_register_port!`. Name the trait each marker stands for:

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }
pub trait RowsPort<T> { fn rows(&self) -> Vec<T>; }

#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = SinkPort)]
struct SinkPortNode;

struct RowsPortNode;
hexser::hex_register_port!(RowsPortNode, hexser::Role::Repository, port_trait: RowsPort<u32>);
```

A generic trait or a trait with associated types is accepted. The template takes
`port_trait:` or `depends_on:`, not both.

Grep for `HexAdapter` and `hex_register_adapter!`. Where an adapter names the port it serves
with `depends_on`, replace that with `implements`:

```rust
// before (0.6.1)
hexser::hex_register_adapter!(FileSink, hexser::Role::Adapter, depends_on: [SinkPortNode]);

// after (0.7), template or derive
hexser::hex_register_adapter!(FileSink, hexser::Role::Adapter, implements: [SinkPortNode]);

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink { facts: Vec<u32> }

impl SinkPort for MemorySink {
  fn append(&mut self, fact: u32) { self.facts.push(fact); }
}
```

- `implements` compiles only if the port marker has `port_trait` and the adapter implements
  that trait. If it does not compile, report it. Do not write a stub impl to make it compile.
- Keep `depends_on` for edges that are not the port served. `hex_register_adapter!` and
  `hex_register_component!` take `depends_on: [..], implements: [..]` together.
- Update wiring cells. `hexser::templates::dependency_edge_exists::<A, P>(&graph)` becomes
  `hexser::templates::implements_edge_exists::<A, P>(&graph)`. A `Depends` edge never reads as
  `Implements`, and an `Implements` edge never reads as `Depends`.

### Step 6: fix `#[hex(...)]` keys that now error

In 0.6 every derive read only `role` and ignored the rest. 0.7 has one strict reader for every
derive. Run `cargo check`. Each of these is now a compile error:

- an unknown key, such as `#[hex(rol = "Entity")]`. The error lists the keys hexser reads;
- a malformed value;
- a single-valued key (`role`, `port_trait`, `code`) written twice;
- a key the derive does not read. The error names the derive that does.

| Key | Read by |
|---|---|
| `role = "..."`, `depends_on(..)` | `HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`, `HexQuery`, `HexActor` |
| `port_trait = Trait` | `HexPort` |
| `implements(..)` | `HexAdapter` |
| `code = "..."`, on a variant | `HexVocabulary`, `HexRefusal` |
| `next_step = "..."`, `suggestion = "..."`, on a variant | `HexRefusal` |

Fix the spelling, or move the key to the derive that reads it. In 0.6 a typo dropped the key,
and the derive applied its default role in silence. After the fix, a node's role can change.
Check graph assertions that relied on the default.

### Step 7: set `HEXSER_INCLUDE_SOURCE_LOCATION` before the process starts

The variable decides whether a serialized error carries its source file and line. It is off
unless set to `1` or `true` (in any case). 0.7 reads it the first time it is needed and keeps
that answer for the life of the process.

Grep for `HEXSER_INCLUDE_SOURCE_LOCATION`, `set_var` and `remove_var`. A change made while the
program runs is no longer seen. Since edition 2024 it also takes `unsafe`. Remove it, and set
the variable in the environment that starts the process:

```bash
HEXSER_INCLUDE_SOURCE_LOCATION=1 cargo test
```

### Step 8: add a graph cell asserting `findings()` is empty

`HexGraph::findings()` returns everything wrong with a graph as values. Add a test:

```rust
/// why: the architecture holds no collision, no unproved adapter, and no dangling edge.
#[test]
fn the_architecture_graph_has_no_findings() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  assert_eq!(graph.findings(), Vec::<hexser::GraphFinding>::new());
}
```

If it fails, fix each finding:

- `AdapterWithoutProvedPort { adapter }`: an Adapter-layer node, other than a `Mapper`, with no
  `Implements` edge. Add `implements(..)` (Step 5). If the type is a mapper, give it
  `#[hex(role = "Mapper")]`.
- `EdgeToUnregisteredNode { source, target, relationship }`: an edge whose end is not a node.
  Register the missing node, or remove the edge.
- `ShortNameCollision { id, kept_type_name, kept_module_path, dropped_type_name, dropped_module_path }`:
  two registrations hashed to one node id, for example two template nodes with one short name
  in two modules. 0.6 merged them in silence. Rename one.

A cell may also name the findings it accepts, instead of asserting none.

### Step 9: check and report

Run `cargo check` and `cargo test`, with the features the crate uses. Fix newly surfaced
errors that map to the steps above, then re-check.

Report the outcome concisely:

- files changed, and which step was applied to each;
- whether `cargo check` and `cargo test` pass, and whether any deprecation warning remains;
- each `#[allow(deprecated)]` left in place, with file:line;
- each adapter that could not prove its port, and each finding the graph cell still reports;
- any remaining compiler error that does not fit the steps (quote it with file:line, and
  suggest a fix).

Do not run git.

## The older path: 0.4 → 0.5

Migrate a Rust crate from hexser 0.4 to hexser 0.5. Follow
`skills/hexser/references/migration-0.4-to-0.5.md` step by step, applying it to
`$ARGUMENTS` when a path is given, or to the current crate when `$ARGUMENTS` is empty.

### Step 0 — locate the crate

- If `$1` is provided, treat it as the crate/workspace path and confine edits to it.
- Otherwise operate on the current crate. Use Glob to find `Cargo.toml` and `**/*.rs`,
  and Grep to find the change sites listed below. Read `skills/hexser/references/migration-0.4-to-0.5.md`
  first so the transforms below stay consistent with the reference.

### Step 1 — bump the dependency

In each `Cargo.toml` that depends on hexser, set the version to `"0.6"`. `macros` is a
default feature, so keep the existing feature list; add `ai`/`visualization` only if the
crate already used those APIs.

```toml
hexser = { version = "0.6", features = ["macros"] }
```

This guide covers the 0.4 → 0.5 API changes. 0.6 adds exactly one further break on top of
them: a registration derive (`HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`,
`HexQuery`) on a GENERIC type is now a compile error — move the derive to a non-generic
marker struct, or hand-write `impl Registrable`. It also changes behaviour in two ways that
need no edits but can move assertions: `#[hex(role = "...")]` is now honoured, and the
`hex_register_*!` macros now actually submit to the graph, so `node_count()` rises. See
CHANGELOG 0.6.0.

### Step 2 — box direct `Hexserror` variant constructions

In 0.5 every `Hexserror` variant wraps its payload in a `Box`
(`Domain(Box<DomainError>)`, `Port(Box<PortError>)`, `Adapter(Box<AdapterError>)`,
`Validation(Box<ValidationError>)`, `NotFound(Box<NotFoundError>)`,
`Conflict(Box<ConflictError>)`). Wrap the payload of any **direct** variant construction
in `std::boxed::Box::new(...)`.

```rust
// before
return Err(Hexserror::Adapter(adapter_err));
// after
return Err(Hexserror::Adapter(std::boxed::Box::new(adapter_err)));
```

Leave match arms alone — a pattern like `Hexserror::Adapter(inner) => ...` binds the `Box`
and needs no change. The constructor helpers (`Hexserror::domain`, `::port`, `::adapter`,
`::validation`, `::validation_field`, `::not_found`, `::conflict`) box for you, so call
sites that use those are already correct.

### Step 3 — apply the trait renames

Rename `Entity` → `HexEntity` and `ValueObject` → `HexValueItem` everywhere they appear:
trait impls, trait bounds, derive attributes, and imports. The old `Entity` alias was
removed in 0.5.

```rust
// before
impl Entity for User { /* ... */ }
impl ValueObject for Email { /* ... */ }
// after
impl HexEntity for User { type Id = String; }
impl HexValueItem for Email { /* fn validate(&self) -> HexResult<()> */ }
```

Prefer `use hexser::prelude::*;` — both `HexEntity` and `HexValueItem` are re-exported there.

### Step 4 — fix id-less `#[derive(HexEntity)]`

`#[derive(HexEntity)]` now requires a struct field literally named `id` and derives
`type Id` from it; a missing `id` field is a compile error (no silent `String` default).

- **Real entity** with identity: ensure an `id` field exists, or hand-write the impl.

  ```rust
  #[derive(HexEntity)]
  struct User { id: String, email: String }

  // or, when identity isn't a plain `id` field:
  impl HexEntity for User { type Id = String; }
  ```

- **DTO / value object** with no identity: remove `#[derive(HexEntity)]`. Use
  `#[derive(HexValueItem)]` if it is a value object.

### Step 5 — update `to_json` call sites

`AIContext::to_json(&self)` and `AgentPack::to_json(&self)` now return `HexResult<String>`
(they previously returned `Result<String, String>`). Update call sites to treat the error
as `Hexserror` — usually just propagate with `?` inside a function returning `HexResult<_>`.

```rust
let json = ai_context.to_json()?; // HexResult<String>
let pack_json = agent_pack.to_json()?; // HexResult<String>
```

These APIs live behind the `ai` feature; only touch them if the crate uses `ai`.

### Step 6 — override `delete_where` where needed

`QueryRepository::delete_where` now has a default that returns
`Err(E_HEX_103 NOT_IMPLEMENTED)` (it used to return `Ok(0)`). Any repository that relied on
the old default must override it to actually delete and return the count removed.

```rust
impl hexser::ports::repository::QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;
    // ... find_one / find ...
    fn delete_where(&mut self, filter: &Self::Filter) -> HexResult<u64> {
        let before = self.users.len();
        // remove matching entities ...
        Ok((before - self.users.len()) as u64)
    }
}
```

### Step 7 — check and report

Run `cargo check` (add `--features ai` / `--features visualization` if the crate uses those
APIs). Fix any newly-surfaced errors that map to the steps above, then re-check.

Report the outcome concisely:

- files changed and which transform was applied to each,
- whether `cargo check` passes,
- any remaining compiler errors that don't fit the steps above (quote the error and file:line
  and suggest a fix), grouped so the user can act on them.

Do not run git.
