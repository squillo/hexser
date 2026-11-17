# Migrating from hexser 0.6 to 0.7

0.7 adds and deprecates. It deletes nothing. A crate that builds on 0.6 builds on 0.7, with
three exceptions:

- an `#[hex(...)]` key hexser never read is now a compile error (see "`#[hex(...)]` is read strictly");
- uses of four duplicate traits and three duplicate names now print deprecation warnings (see "Four doors are deprecated");
- `HEXSER_INCLUDE_SOURCE_LOCATION` is read once per process (see "The location switch is read once").

Every family pin today is `0.6` or `0.6.x`, so `cargo update` does not move anyone to 0.7.
Each owner moves its own pin:

```toml
hexser = { version = "0.7", default-features = false, features = ["macros"] }
```

Every block in this guide is compiled and run as a doctest of the `hexser` crate. Each "before"
block compiles against 0.7 too; that is the point of a deprecation. A block that reads the
architecture graph is its own crate, so no other example's registrations appear in it.

What 0.7 offers, at a glance:

| You want | 0.7 gives you | Section |
|---|---|---|
| one writer that returns its event, view or typed outcome | `Actor`, `#[derive(HexActor)]` | 1 |
| a closed list of command, event or refusal names | `Vocabulary`, `#[derive(HexVocabulary)]` | 2 |
| a refusal with guidance that stays a value | `Refusal`, `#[derive(HexRefusal)]`, `Hexserror::refused`, `Hexserror::code` | 3 |
| a port node that cannot outlive its trait | `#[hex(port_trait = Trait)]`, `port_trait:` | 4 |
| an adapter that proves which port it serves | `#[hex(implements(..))]`, `implements: [..]`, `ImplementsEntry`, `implements_edge_exists` | 5 |
| edges on a derive | `#[hex(depends_on(..))]` | 6 |
| a graph that says what is wrong with it | `HexGraph::findings()`, `GraphFinding` | 7 |

## 1. `Actor` in place of `DirectiveHandler`

An actor owns its state and is the one place that state changes. `&mut self` already says "one
writer, one command at a time", so the door needs no lock. The command comes in by value. The
context and the outcome are associated types, so an actor can return an event, a view, a
`Result` of its own refusal, or `()`. The trait imposes no `Result`, no `Hexserror`, and no
`Send` or `Sync` bound.

Before (0.6, `&self`, so state changes need a `Cell` or a lock, and the outcome is fixed):

```rust
#![allow(deprecated)]

struct Rename { name: String }

impl hexser::application::Directive for Rename {
  fn validate(&self) -> hexser::HexResult<()> { Ok(()) }
}

struct Deck { name: std::cell::RefCell<String> }

impl hexser::application::DirectiveHandler<Rename> for Deck {
  fn handle(&self, directive: Rename) -> hexser::HexResult<()> {
    *self.name.borrow_mut() = directive.name;
    Ok(())
  }
}

fn main() {
  let deck = Deck { name: std::cell::RefCell::new(String::new()) };
  hexser::application::DirectiveHandler::handle(&deck, Rename { name: "Q3".into() }).unwrap();
  assert_eq!(*deck.name.borrow(), "Q3");
}
```

After (0.7, `&mut self`, and the outcome is the event or the refusal):

```rust
#[derive(Debug, PartialEq)]
enum Renamed { To(String) }

#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum RenameRefusal {
  #[hex(next_step = "Type a name")]
  EmptyName,
}

struct Deck { name: String }

impl hexser::application::Actor for Deck {
  type Command = String;
  type Context<'a> = ();
  type Outcome = Result<Renamed, RenameRefusal>;

  fn handle(&mut self, name: String, _context: ()) -> Self::Outcome {
    if name.is_empty() {
      return Err(RenameRefusal::EmptyName);
    }
    self.name = name.clone();
    Ok(Renamed::To(name))
  }
}

/// The actor's node in the architecture graph: Application layer, role `DirectiveHandler`.
#[derive(hexser::HexActor)]
struct DeckActorNode;

fn main() {
  let mut deck = Deck { name: String::new() };
  assert_eq!(
    hexser::application::Actor::handle(&mut deck, "Q3".into(), ()),
    Ok(Renamed::To("Q3".into()))
  );
  assert_eq!(
    hexser::application::Actor::handle(&mut deck, String::new(), ()),
    Err(RenameRefusal::EmptyName)
  );
  assert_eq!(deck.name, "Q3");
}
```

A context that borrows per call is a generic associated type, for example
`type Context<'a> = &'a Clock;`.

`#[derive(HexActor)]` reads `role` and `depends_on(..)`, and refuses a generic type. A call to
`Actor::handle` through a shared reference does not compile: the single writer holds `&mut`.

## 2. `HexVocabulary` in place of a hand-written name list

A command, an event and a refusal are each a closed enum, and each value's identity is its
variant's name. Before 0.7 the name list was written by hand next to the enum, and the two
drifted.

Before:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ask { Open, Close }

impl Ask {
  const NAMES: &'static [&'static str] = &["Open", "Close"];
  fn name(&self) -> &'static str {
    match self { Ask::Open => "Open", Ask::Close => "Close" }
  }
}

fn main() {
  assert_eq!(Ask::NAMES, &["Open", "Close"]);
  assert_eq!(Ask::Close.name(), "Close");
}
```

After:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexVocabulary)]
enum Ask {
  Open,
  #[hex(code = "ASK_CLOSE")]
  Close,
}

fn main() {
  assert_eq!(<Ask as hexser::domain::Vocabulary>::CODES, &["Open", "ASK_CLOSE"]);
  assert_eq!(hexser::domain::Vocabulary::code(&Ask::Close), "ASK_CLOSE");
  assert_eq!(Ask::from_code("ASK_CLOSE"), Some(Ask::Close));
  assert_eq!(Ask::from_code("Close"), None);
}
```

- `CODES` lists the codes in declaration order.
- `#[hex(code = "...")]` replaces a variant's name as its code, everywhere the code appears.
- `from_code` is generated only when every variant is a unit variant. An unknown code reads back
  as `None`, never as a guessed variant.
- Data-carrying variants still have codes.
- `#[non_exhaustive]` enums are accepted.
- Two variants with the same code are a compile error.

## 3. `HexRefusal` and `Hexserror::refused`

A refusal is a closed vocabulary whose variants carry static guidance. It stays a value inside
your crate. At the edge it crosses `Hexserror` as the existing `Domain` variant, so
`Hexserror` keeps its six variants and stays exhaustively matchable. Its code reads back to the
value it came from.

Before (the code and the guidance are strings at the call site):

```rust
fn main() {
  let err = hexser::Hexserror::domain("EmptyName", "The deck needs a name")
    .with_next_step("Type a name");
  assert!(matches!(err, hexser::Hexserror::Domain(_)));
}
```

After:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum EditRefusal {
  #[hex(next_step = "Type a name", suggestion = "A name is 1 to 64 characters")]
  EmptyName,
  #[hex(code = "E_NAME_TOO_LONG")]
  NameTooLong,
}

fn main() {
  assert_eq!(hexser::error::Refusal::next_steps(&EditRefusal::EmptyName), &["Type a name"]);

  let err = hexser::Hexserror::refused(&EditRefusal::EmptyName, "The deck needs a name");
  assert!(matches!(err, hexser::Hexserror::Domain(_)));
  assert_eq!(err.code(), "EmptyName");
  assert_eq!(EditRefusal::from_code(err.code()), Some(EditRefusal::EmptyName));
}
```

`#[hex(next_step = "...")]` and `#[hex(suggestion = "...")]` may each be written more than once
on a variant. `Hexserror::code()` answers for all six variants.

## 4. `port_trait` on a port marker

A port node is a marker type in the graph. Before 0.7 nothing tied the marker to the trait it
stands for, so a marker could outlive its trait. `port_trait` names the trait. An adapter that
claims the port (section 5) is then checked against that trait at compile time.

Before:

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }

#[derive(hexser::HexPort)]
struct SinkPortNode;

fn main() {}
```

After, with the derive or with the template:

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }
pub trait RowsPort<T> { fn rows(&self) -> Vec<T>; }

#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = SinkPort)]
struct SinkPortNode;

struct RowsPortNode;
hexser::hex_register_port!(RowsPortNode, hexser::Role::Repository, port_trait: RowsPort<u32>);

fn main() {}
```

A generic trait or a trait with associated types is accepted.

## 5. `implements(..)` in place of a `depends_on` onto a port marker

In 0.6.1 the only edge a template could carry was `depends_on:`. An adapter that served a port
said so with a `Depends` edge. That edge was declared, not proved. `implements` lands an
`Implements` edge. It compiles only if the adapter implements the port's `port_trait`.

Before (0.6.1):

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }

struct SinkPortNode;
hexser::hex_register_port!(SinkPortNode, hexser::Role::OutputPort);

struct MemorySink;
hexser::hex_register_adapter!(MemorySink, hexser::Role::Adapter, depends_on: [SinkPortNode]);

fn main() {}
```

After, with the derive or with the template:

```rust,standalone_crate
pub trait SinkPort { fn append(&mut self, fact: u32); }

#[derive(hexser::HexPort)]
#[hex(port_trait = SinkPort)]
struct SinkPortNode;

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink { facts: Vec<u32> }

impl SinkPort for MemorySink {
  fn append(&mut self, fact: u32) { self.facts.push(fact); }
}

struct FileSink;
impl SinkPort for FileSink {
  fn append(&mut self, _fact: u32) {}
}
hexser::hex_register_adapter!(FileSink, hexser::Role::Adapter, implements: [SinkPortNode]);

fn main() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  assert!(hexser::templates::implements_edge_exists::<MemorySink, SinkPortNode>(&graph));
  assert!(hexser::templates::implements_edge_exists::<FileSink, SinkPortNode>(&graph));
  assert!(!hexser::templates::dependency_edge_exists::<MemorySink, SinkPortNode>(&graph));
}
```

An adapter that names a port marker without `port_trait` does not compile. Neither does an
adapter that does not implement the port's trait.

- `implements_edge_exists::<A, P>(&graph)` is true only when the edge exists and the port's
  node is in the graph.
- A `Depends` edge never reads as `Implements`, and an `Implements` edge never reads as `Depends`.
- The edge is carried by `hexser::registry::ImplementsEntry`, which is collected through
  `inventory` like every registration.
- `hex_register_component!` takes `implements: [..]`, and `depends_on: [..], implements: [..]`.

## 6. `depends_on(..)` on every registration derive

0.6.1 put `depends_on: [..]` on the templates only. 0.7 puts it on `HexDomain`, `HexPort`,
`HexAdapter`, `HexDirective`, `HexQuery` and `HexActor`. It lands the same `Depends` edge, on
the same node.

```rust,standalone_crate
struct RepoPortNode;
hexser::hex_register_port!(RepoPortNode, hexser::Role::Repository);

#[derive(hexser::HexDirective)]
#[hex(depends_on(RepoPortNode))]
struct RenameDirective;

fn main() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  assert!(hexser::templates::dependency_edge_exists::<RenameDirective, RepoPortNode>(&graph));
}
```

## 7. `findings()` in a graph cell

0.6 recorded a node-id collision as a warning string and said nothing else. 0.7 returns
everything wrong with a graph as values:

```rust,standalone_crate
pub trait SinkPort { fn append(&mut self, fact: u32); }

#[derive(hexser::HexPort)]
#[hex(port_trait = SinkPort)]
struct SinkPortNode;

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink;
impl SinkPort for MemorySink { fn append(&mut self, _fact: u32) {} }

/// why: the architecture holds no collision, no unproved adapter, and no dangling edge.
fn main() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  assert_eq!(graph.findings(), Vec::<hexser::GraphFinding>::new());
}
```

`hexser::graph::GraphFinding` has three variants:

- `ShortNameCollision { id, kept_type_name, kept_module_path, dropped_type_name, dropped_module_path }`:
  two registrations hashed to one node id. Two template nodes with one short name in two
  modules are now a finding. In 0.6 they were merged in silence. The 0.6 warning string is
  still written, and it now names both module paths.
- `AdapterWithoutProvedPort { adapter }`: an Adapter-layer node, other than a `Mapper`, with no
  `Implements` edge.
- `EdgeToUnregisteredNode { source, target, relationship }`: an edge whose end is not a node.

## 8. Four doors are deprecated

Each still compiles and works, and each use prints a warning naming its route. None is
removed in 0.7. Each is scheduled for removal in 0.8.0, and the deletion waits until no
family crate still uses it.

| Deprecated | Route to |
|---|---|
| `application::DirectiveHandler` | `application::Actor` |
| `ports::UseCase` | `application::Actor` |
| `ports::InputPort` | `application::Actor` |
| `ports::Query` | `application::QueryHandler` (the same shape) |

Three names were two names for one job, and the alias is deprecated on the same schedule:

| Deprecated | Route to |
|---|---|
| `GraphBuilder::add_node` | `GraphBuilder::with_node` (the same call) |
| `GraphBuilder::add_edge` | `GraphBuilder::with_edge` (the same call) |
| `NodeId::from_type_name` | `NodeId::from_name` (the same hash) |

To keep a 0.6 call site quiet while you move it, put `#[allow(deprecated)]` on the item.

## 9. `#[hex(...)]` is read strictly

In 0.6 every derive read only `role`, and it ignored the rest. A parse error was discarded with
it, so a key written after a typo was dropped too, and the derive applied its default in
silence. 0.7 has one reader for every derive:

- an unknown key is a compile error that lists the keys hexser reads;
- a malformed value is a compile error;
- a single-valued key (`role`, `port_trait`, `code`) written twice is a compile error;
- a key the derive does not read is a compile error naming the derive that does. For example,
  `implements` is read by `HexAdapter`, and `port_trait` by `HexPort`.

```rust,compile_fail
#[derive(hexser::HexDomain)]
#[hex(rol = "Entity")]
struct Typo;

fn main() {}
```

Every family crate, read at the time of writing, writes only `#[hex(role = "...")]`, which
reads as before.

## 10. The location switch is read once

`HEXSER_INCLUDE_SOURCE_LOCATION` decides whether a serialized error carries its source file and
line. It is off unless set to `1` or `true` (in any case). 0.7 reads it the first time it is
needed and keeps that answer for the life of the process. A program that changes the variable
while it runs is no longer seen. Since edition 2024, changing the environment at run time takes
`unsafe`, and hexser now forbids `unsafe` in all three crates. Set the variable before the
process starts.

## 11. Nothing else moves

- `Hexserror` keeps exactly its six variants.
- `Actor`, `Vocabulary` and `Refusal` are re-exported at the crate root, not in the prelude, so
  a glob import of the prelude cannot collide with a type of yours with the same name. The
  derives `HexActor`, `HexVocabulary` and `HexRefusal` are in both.
- `hexser`, `hexser_macros` and `hexser_potions` are each `#![forbid(unsafe_code)]`.
- Every 0.7 door runs on `wasm32-unknown-unknown` and `wasm32-wasip1` with `features = ["macros"]`
  only. The repository's `scripts/wasm-e2e.sh` executes them under wasmtime.
