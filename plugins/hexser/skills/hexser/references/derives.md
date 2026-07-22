# hexser derive macros

The derive macros ship with the `macros` feature (on by default) and are re-exported from the prelude:

```rust
use hexser::prelude::*;
```

This brings `HexAdapter`, `HexAggregate`, `HexDirective`, `HexDomain`, `HexEntity`, `HexPort`, `HexQuery`, `HexRepository`, and `HexValueItem` into scope. They target structs and enums only — deriving on a `union` is a compile error.

Two families of derive exist:

- **Registration derives** (`HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`, `HexQuery`) emit `impl Registrable for T` *and* `inventory::submit!(ComponentEntry::new::<T>())`, so the type shows up in the process-wide `HexGraph`.
- **Trait derives** (`HexEntity`, `HexValueItem`, `HexAggregate`) emit a domain trait impl and do **no** registration.

`HexRepository` is a special case: a pure marker that emits nothing.

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

### Generics gotcha

For a **generic** type only the `Registrable` impl is emitted — the `inventory::submit!` is skipped, because inventory cannot name a generic instantiation. A generic component therefore still answers `T::node_info()` but will **not** appear in the graph automatically.

```rust
#[derive(HexAdapter)]
struct Cache<T> { items: Vec<T> } // Registrable impl only; no inventory submit
```

## HexDomain

- **Generates:** `impl Registrable` + inventory submission.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Domain`, `Role::Entity`.

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
- **Layer/Role:** `Layer::Application`, `Role::Directive`.

The generated `validate` always returns `Ok(())`. Hand-write your own `impl Directive` (instead of deriving) when a directive needs real validation, and pair with a hand-written `impl DirectiveHandler<D>` to execute it (see patterns.md).

```rust
use hexser::prelude::*;

#[derive(HexDirective)]
struct CreateTodoDirective { title: String }
```

## HexQuery

- **Generates:** `impl Registrable` + inventory submission only — **no** trait impl.
- **Requires:** nothing beyond struct/enum.
- **Layer/Role:** `Layer::Application`, `Role::Query`.

Unlike `HexDirective`, it emits no `validate`/`query` method. Provide behavior with a hand-written `impl QueryHandler<Q, R>`.

```rust
use hexser::prelude::*;

#[derive(HexQuery)]
struct ListActiveTodosQuery;
```

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

See errors.md for building rich `Hexserror` values inside hand-written `validate`/`check_invariants`, and patterns.md for wiring directives, queries, and repositories end to end.
