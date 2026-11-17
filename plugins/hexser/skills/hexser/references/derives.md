# hexser derive macros

The derive macros ship with the `macros` feature (on by default) and are re-exported from the prelude:

```rust
use hexser::prelude::*;
```

This brings `HexActor`, `HexAdapter`, `HexAggregate`, `HexDirective`, `HexDomain`, `HexEntity`, `HexPort`, `HexQuery`, `HexRefusal`, `HexRepository`, `HexValueItem`, and `HexVocabulary` into scope. They target structs and enums only — deriving on a `union` is a compile error. `HexVocabulary` and `HexRefusal` target enums only.

Three families of derive exist:

- **Registration derives** (`HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`, `HexQuery`, `HexActor`) emit `impl Registrable for T` *and* `inventory::submit!(ComponentEntry::new::<T>())`, so the type shows up in the process-wide `HexGraph`. **All six honour `#[hex(role = "...")]` and `#[hex(depends_on(..))]`**; only the default role differs.
- **Trait derives** (`HexEntity`, `HexValueItem`, `HexAggregate`) emit a domain trait impl and do **no** registration.
- **Vocabulary derives** (`HexVocabulary`, `HexRefusal`, new in 0.7) name the variants of a closed enum and do **no** registration.

`HexRepository` is a special case: a pure marker that emits nothing.

## `#[hex(...)]` keys (read strictly in 0.7)

0.7 has one reader for every derive. An unknown key, a malformed value, or a single-valued key (`role`, `port_trait`, `code`) written twice is a compile error. So is a key the derive does not read; the error names the derive that does. In 0.6 each derive read only `role` and dropped the rest in silence.

| Key | Where | Read by |
| --- | --- | --- |
| `role = "Variant"` | the type | the six registration derives |
| `depends_on(Type, ..)` | the type | the six registration derives; lands one `Depends` edge per type |
| `port_trait = Trait` | the type | `HexPort` |
| `implements(PortNode, ..)` | the type | `HexAdapter` |
| `code = "..."` | an enum variant | `HexVocabulary`, `HexRefusal` |
| `next_step = "..."`, `suggestion = "..."` (repeatable) | an enum variant | `HexRefusal` |

```rust,compile_fail
#[derive(hexser::HexDomain)]
#[hex(rol = "Entity")] // ERROR: unknown key
struct Typo;
```

Each type named in `depends_on(..)` must itself be registered:

```rust
struct RepoPortNode;
hexser::hex_register_port!(RepoPortNode, hexser::Role::Repository);

#[derive(hexser::HexDirective)]
#[hex(depends_on(RepoPortNode))]
struct RenameDirective;
// hexser::templates::dependency_edge_exists::<RenameDirective, RepoPortNode>(&graph) == true
```

## Registrable and inventory (what registration means)

Every registration derive produces:

```rust
impl Registrable for T {
    fn node_info() -> NodeInfo { /* layer, role, type_name, module_path */ }
    fn dependencies() -> Vec<NodeId> { /* ... */ }
}
// plus: inventory::submit!(ComponentEntry::new::<T>())
```

`NodeInfo` carries `{ layer, role, type_name, module_path }`. The inventory submission is what makes `HexGraph::current()` see the component at link time.

### Generics: a registration derive on a generic type is a COMPILE ERROR

`inventory` submits one entry per component and `type_name::<Self>()` on a generic names a monomorphization chosen by a consumer crate, so there is no single honest node for `Cache<T>`. The derive says so rather than skipping the submission:

```rust
#[derive(HexAdapter)]
struct Cache<T> { items: Vec<T> } // ERROR: cannot register a generic type
```

⚠ **Until 2026-09-04 this compiled and registered nothing** — the type implemented `Registrable`, answered `node_info()` correctly, and was absent from every graph query, with no error and no warning. If you inherit a codebase built against that behaviour, `#[derive(...)]` on a generic marks a component the graph never had.

Two remedies, both named by the error:

```rust
// 1. A non-generic MARKER struct stands for the component in the graph.
#[derive(HexAdapter)]
struct CacheComponent;

struct Cache<T> { items: Vec<T> }   // the real type, underived

// 2. Or hand-write Registrable when you need node_info() on the generic itself.
//    A hand-written impl submits nothing, so it is still not in the graph.
impl<T: 'static> hexser::registry::Registrable for Cache<T> { /* ... */ }
```

Lifetime and const parameters count: they are not in scope at the item scope where the submission expands either.

## HexDomain

- **Generates:** `impl Registrable` + inventory submission.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Domain`, default `Role::Entity`.
- **Override:** `#[hex(role = "ValueObject")]` (or any `Role` variant).

⚠ **Until 2026-09-04 HexDomain declared `attributes(hex)` and IGNORED it** — a value object marked `#[hex(role = "ValueObject")]` compiled clean and registered as `Role::Entity`. A domain layer is not made only of entities, so set the role deliberately:

```rust
#[derive(HexDomain)]
#[hex(role = "ValueObject")]
struct Email(String);
```

Typically stacked with `HexEntity` on a domain aggregate root or entity so the type both registers and gets its `HexEntity` impl.

```rust
use hexser::prelude::*;

#[derive(HexDomain, HexEntity, Clone, Debug)]
struct User { id: String, email: String }
```

## HexPort

- **Generates:** `impl Registrable` + inventory submission.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Port`, default `Role::Repository`.
- **Override:** `#[hex(role = "...")]` with any `Role` variant.

Valid `Role` variants: `Entity`, `ValueObject`, `Aggregate`, `DomainEvent`, `DomainService`, `InputPort`, `OutputPort`, `Repository`, `UseCase`, `Query`, `Adapter`, `Mapper`, `Directive`, `DirectiveHandler`, `QueryHandler`, `Config`, `Unknown`.

```rust
use hexser::prelude::*;

#[derive(HexPort)]
#[hex(role = "InputPort")]
struct GreetPort;
```

**`port_trait` (0.7).** A port marker names the trait it stands for. Delete or rename the trait and the marker does not compile. Adapters then prove they serve the port with `implements(..)`. A generic trait or a trait with associated types is accepted.

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }

#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = SinkPort)]
struct SinkPortNode;
```

## HexAdapter

- **Generates:** `impl Registrable` + inventory submission **and** `impl Adapter for T {}`.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Adapter`, default `Role::Adapter`.
- **Override:** `#[hex(role = "Mapper")]` (any `Role` variant).

```rust
use hexser::prelude::*;

#[derive(HexAdapter, Default)]
struct InMemoryUserRepository { users: Vec<User> }

// A mapper adapter overrides its role:
#[derive(HexAdapter)]
#[hex(role = "Mapper")]
struct UserDtoMapper;
```

**`implements(..)` (0.7).** An adapter proves the port it serves. It compiles only if the port marker has `port_trait` and the adapter implements that trait. It lands an `Implements` edge, carried by `hexser::registry::ImplementsEntry`. Use it in place of a `depends_on` onto a port marker: that edge is declared, not proved, and `HexGraph::findings()` reports the adapter as `AdapterWithoutProvedPort`.

```rust
#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink { facts: Vec<u32> }

impl SinkPort for MemorySink {
  fn append(&mut self, fact: u32) { self.facts.push(fact); }
}
// hexser::templates::implements_edge_exists::<MemorySink, SinkPortNode>(&graph) == true
```

## HexRepository

- **Generates:** nothing. It is a **pure marker** — it validates the target and emits no impl and no registration.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** none of its own.

Because it registers nothing, pair it with `HexPort` (whose default role is already `Repository`) when you want the repository to appear in the graph.

```rust
use hexser::prelude::*;

#[derive(HexPort, HexRepository)]
struct UserRepositoryPort; // HexPort registers it as Layer::Port / Role::Repository
```

## HexDirective

- **Generates:** `impl Directive for T { fn validate(&self) -> HexResult<()> { Ok(()) } }` **and** `impl Registrable` + inventory submission.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Application`, default `Role::Directive`.
- **Override:** `#[hex(role = "...")]` (any `Role` variant).

The generated `validate` always returns `Ok(())`. Hand-write your own `impl Directive` (instead of deriving) when a directive needs real validation. To execute a write in 0.7, implement `hexser::application::Actor` (see HexActor below). `DirectiveHandler` is deprecated in 0.7, routed to `Actor`.

```rust
use hexser::prelude::*;

#[derive(HexDirective)]
struct CreateTodoDirective { title: String }
```

## HexQuery

- **Generates:** `impl Registrable` + inventory submission only — **no** trait impl.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Application`, default `Role::Query`.
- **Override:** `#[hex(role = "...")]` (any `Role` variant). Because HexQuery registers WITHOUT emitting a trait impl, it is the vehicle for an Application-layer component that is neither a directive nor a query:

```rust
#[derive(HexQuery)]
#[hex(role = "UseCase")]
struct SignupUseCaseComponent;
```

Unlike `HexDirective`, it emits no `validate`/`query` method. Provide behavior with a hand-written `impl QueryHandler<Q, R>`.

```rust
use hexser::prelude::*;

#[derive(HexQuery)]
struct ListActiveTodosQuery;
```

## HexActor (0.7)

- **Generates:** `impl Registrable` + inventory submission only. It emits **no** `Actor` impl: the actor's command, context and outcome are its own.
- **Requires:** a non-generic struct or enum. A generic type is a compile error, so a generic actor registers a non-generic marker.
- **Layer/Role:** `Layer::Application`, default `Role::DirectiveHandler`.
- **Reads:** `#[hex(role = "...")]` and `#[hex(depends_on(..))]`.

```rust
struct Deck { name: String }

impl hexser::application::Actor for Deck {
  type Command = String;
  type Context<'a> = ();
  type Outcome = ();

  fn handle(&mut self, name: String, _context: ()) -> Self::Outcome { self.name = name; }
}

/// The actor's node in the architecture graph.
#[derive(hexser::HexActor)]
struct DeckActorNode;
```

See api.md for the `Actor` trait.

## HexEntity

- **Generates:** `impl HexEntity for T { type Id = /* type of the `id` field */; }`.
- **Requires:** a struct field literally named `id`. The associated `Id` type is taken from that field's type.
- **Layer/Role:** none — this derive does **no** registration.

### Gotcha: the `id` field is mandatory

If the type has no field named `id` (or is not a struct), it is a **compile error** — there is no silent `String` default.

```rust
use hexser::prelude::*;

#[derive(HexEntity)]
struct Account { id: u64, balance: i64 } // type Id = u64
```

To use an identity type that is not a plain field, hand-write the impl instead of deriving:

```rust
use hexser::prelude::*;

struct UserId(String);
struct User { user_id: UserId, email: String }

impl HexEntity for User {
    type Id = UserId;
}
```

## HexValueItem

- **Generates:** `impl HexValueItem for T { fn validate(&self) -> HexResult<()> { Ok(()) } }`.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** none — no registration.

The generated `validate` always returns `Ok(())`; hand-write `impl HexValueItem` when the value item needs real validation.

```rust
use hexser::prelude::*;

#[derive(HexValueItem)]
struct Email(String);
```

## HexAggregate

- **Generates:** `impl Aggregate for T { fn check_invariants(&self) -> HexResult<()> { Ok(()) } }`.
- **Requires:** nothing beyond struct/enum. `Aggregate: HexEntity`, so the type also needs a `HexEntity` impl (derive `HexEntity` or hand-write it).
- **Layer/Role:** none — no registration.

The generated `check_invariants` always returns `Ok(())`; hand-write `impl Aggregate` when the aggregate enforces real invariants.

```rust
use hexser::prelude::*;

#[derive(HexEntity, HexAggregate)]
struct Order { id: String, items: Vec<String> }
```

## HexVocabulary (0.7)

- **Generates:** `impl hexser::domain::Vocabulary` (`const CODES`, `fn code(&self)`), and, when every variant is a unit variant, an inherent `fn from_code(code: &str) -> Option<Self>`.
- **Requires:** an enum. A struct or union is a compile error.
- **Layer/Role:** none — no registration.
- **Reads:** `#[hex(code = "...")]` on a variant. It replaces the variant's name as its code, everywhere the code appears.

```rust
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

- `CODES` lists the codes in declaration order.
- An unknown code reads back as `None`, never as a guessed variant.
- Data-carrying variants still have codes. `from_code` is then not generated.
- `#[non_exhaustive]` enums are accepted.
- Two variants with the same code are a compile error.
- It writes no `Display` and no `Default`.

## HexRefusal (0.7)

- **Generates:** everything `HexVocabulary` generates, plus `impl hexser::error::Refusal` with the static guidance. Do not derive both on one enum.
- **Requires:** an enum.
- **Layer/Role:** none — no registration.
- **Reads:** `#[hex(code = "...")]`, `#[hex(next_step = "...")]` and `#[hex(suggestion = "...")]` on a variant. `next_step` and `suggestion` may each be written more than once.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum EditRefusal {
  #[hex(next_step = "Type a name", suggestion = "A name is 1 to 64 characters")]
  EmptyName,
  #[hex(code = "E_NAME_TOO_LONG")]
  NameTooLong,
}
// hexser::error::Refusal::next_steps(&EditRefusal::EmptyName) == &["Type a name"]
```

A refusal stays a value inside the crate. At the edge it crosses into `Hexserror` through `Hexserror::refused` (see errors.md).

See errors.md for building rich `Hexserror` values inside hand-written `validate`/`check_invariants`, and patterns.md for wiring directives, queries, and repositories end to end.

## Registering without a derive: `hex_register_*`

For a type you cannot annotate, or a build without the `macros` feature, hexser exports
`hex_register_component!` plus per-layer wrappers (`hex_register_domain!`,
`hex_register_port!`, `hex_register_adapter!`, `hex_register_application!`,
`hex_register_infrastructure!`). Each emits the `Registrable` impl **and** the inventory
submission, so the type lands in `HexGraph::current()`:

```rust
struct PgUserRepo;

hexser::hex_register_adapter!(PgUserRepo, hexser::graph::Role::Adapter);
```

**Edges: `depends_on: [...]`.** The two- and three-argument forms register a NODE with no
edges, so the graph can say an adapter exists but never which port it plugs into. To carry
the adapter → port edge, register a zero-sized `<Port>Node` marker for the port and name it:

```rust
struct UserRepositoryPortNode;
struct PgUserRepo;

hexser::hex_register_port!(UserRepositoryPortNode, hexser::graph::Role::Repository);
hexser::hex_register_adapter!(PgUserRepo, hexser::graph::Role::Adapter, depends_on: [UserRepositoryPortNode]);
```

To ask "is this adapter wired to this port?", call
`hexser::templates::dependency_edge_exists::<PgUserRepo, UserRepositoryPortNode>(&graph)`.
It is true only for a `Depends` edge in that direction whose port is itself a node in the
graph, so a dangling edge never reads as wired. Write a wiring cell with it; do not write a
private edge walk. To list the edges, use
`graph.edges_to(&hexser::templates::dependency_id::<UserRepositoryPortNode>())`.
⚠ Build the id with `dependency_id`, **never `NodeId::of`**. Template-registered nodes are
keyed by their short type name, while `NodeId::of` hashes the full path, so an id built with
it names no node, and `build_graph` does not validate edges.

**0.7: `port_trait:` and `implements: [...]`.** The `depends_on:` edge above is declared, not
proved, and `HexGraph::findings()` reports such an adapter as `AdapterWithoutProvedPort`. In
0.7 the port marker names its trait, and the adapter names the port it serves with
`implements:`. That compiles only if the adapter implements the trait. It lands an
`Implements` edge.

```rust
pub trait SinkPort { fn append(&mut self, fact: u32); }

struct SinkPortNode;
hexser::hex_register_port!(SinkPortNode, hexser::Role::OutputPort, port_trait: SinkPort);

struct FileSink;
impl SinkPort for FileSink { fn append(&mut self, _fact: u32) {} }
hexser::hex_register_adapter!(FileSink, hexser::Role::Adapter, implements: [SinkPortNode]);
```

- Read it with `hexser::templates::implements_edge_exists::<FileSink, SinkPortNode>(&graph)`.
  It is true only when the edge exists and the port's node is in the graph. A `Depends` edge
  never reads as `Implements`, and an `Implements` edge never reads as `Depends`.
- `hex_register_adapter!` and `hex_register_component!` take `implements: [..]`, and
  `depends_on: [..], implements: [..]`.
- `hex_register_port!` takes `port_trait:` or `depends_on:`, not both.

⚠ **Until 2026-09-04 these emitted the `Registrable` impl and NO submission — for every
type, not only generics.** A component "registered" through hexser's own advertised
non-derive door answered `node_info()` correctly and was in no graph. If you inherit code
built against that behaviour, every `hex_register_*` call is a component the graph never had.

Two properties differ from the derives, deliberately:

- The macro is invoked at the **call site**, not at the type's definition, and it derives
  `module_path` by splitting `type_name::<Self>()` — so the node reports the type's true
  module even when the macro is invoked from an adoption-seam file. A derive uses
  `module_path!()`, which reports the module the derive is *written* in.
- Consequently its `type_name` is the **short** name (`"PgUserRepo"`), where a derive's is the
  full path (`"my_crate::adapters::PgUserRepo"`). Match on the suffix, not on equality, if a
  graph query has to accept both.

The type must be concrete — `ComponentEntry::new::<T>()` on a generic is a compile error, the
same polarity as the derives.
