# hexser build patterns

Start here to build anything with hexser. This is the canonical flow: **Domain → Port → Adapter → use-case**. Model your data as domain types, declare capabilities as ports, implement those ports in adapters, then orchestrate them from a use-case (a free function or a CQRS handler). The derive macros register each piece into a process-wide graph you can introspect at the end.

All examples prefer `use hexser::prelude::*;`. Use a qualified path only for items not conveniently in the prelude (e.g. `hexser::ports::repository::QueryRepository`). For error construction see errors.md; for graph/visualization see the graph section at the end.

## The layering

**1. Domain** — plain structs/enums that model your problem. Derive `HexDomain` to register them into the graph under `Layer::Domain`, and `HexEntity` to get an identity type. `#[derive(HexEntity)]` requires a struct field literally named `id` and takes `type Id` from it (no `id` field is a compile error). Aggregates additionally implement `Aggregate` (`check_invariants`), value items implement `HexValueItem` (`validate`). The domain owns everything about the data — including the `Filter` and `SortKey` types used to query it.

**2. Port** — a trait that names a capability the domain needs, with no implementation. `Repository<T>` is save-only (`fn save(&mut self, entity: T) -> HexResult<()>`). `QueryRepository<T>` is the read/delete side (`find_one`, `find`, `exists`, `count`, `delete_where`). Define a custom port as a trait with a supertrait bound on `Repository<T>`. Derive `HexPort` to register it under `Layer::Port` (default `Role::Repository`); pair with `HexRepository` as a pure marker.

**3. Adapter** — a concrete type that implements the port(s). Derive `HexAdapter` to register it under `Layer::Adapter` and auto-emit `impl Adapter for T {}`. One adapter typically implements **both** `Repository` (writes) and `QueryRepository` (reads/delete), plus your custom port trait.

**4. Use-case / application** — orchestration. Either a free function that takes `&mut R` bounded by your port traits, or a CQRS pair: a `Directive` (command) with a hand-written `DirectiveHandler`, and a `Query` with a hand-written `QueryHandler`. Directives derive `HexDirective` (registers under `Layer::Application`, `Role::Directive`, and auto-implements `Directive::validate` returning `Ok(())`).

## Full end-to-end example (compiles against 0.5)

```rust
use hexser::prelude::*;

#[derive(HexDomain, HexEntity, Clone, Debug)]
struct User { id: String, email: String }

trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}

#[derive(Clone)]
enum UserFilter { All, ById(String), ByEmail(String) }
#[derive(Clone, Copy)]
enum UserSortKey { Id, Email }

#[derive(HexAdapter, Default)]
struct InMemoryUserRepository { users: Vec<User> }

impl Repository<User> for InMemoryUserRepository {
    fn save(&mut self, entity: User) -> HexResult<()> {
        if let Some(existing) = self.users.iter_mut().find(|u| u.id == entity.id) {
            *existing = entity;
        } else { self.users.push(entity); }
        Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;
    fn find_one(&self, filter: &UserFilter) -> HexResult<Option<User>> {
        Ok(match filter {
            UserFilter::All => self.users.first().cloned(),
            UserFilter::ById(id) => self.users.iter().find(|u| &u.id == id).cloned(),
            UserFilter::ByEmail(e) => self.users.iter().find(|u| &u.email == e).cloned(),
        })
    }
    fn find(&self, filter: &UserFilter, _opts: FindOptions<UserSortKey>) -> HexResult<Vec<User>> {
        Ok(match filter {
            UserFilter::All => self.users.clone(),
            UserFilter::ById(id) => self.users.iter().filter(|u| &u.id == id).cloned().collect(),
            UserFilter::ByEmail(e) => self.users.iter().filter(|u| &u.email == e).cloned().collect(),
        })
    }
    fn delete_where(&mut self, filter: &UserFilter) -> HexResult<u64> {
        let before = self.users.len();
        match filter {
            UserFilter::All => self.users.clear(),
            UserFilter::ById(id) => self.users.retain(|u| &u.id != id),
            UserFilter::ByEmail(e) => self.users.retain(|u| &u.email != e),
        }
        Ok((before - self.users.len()) as u64)
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>> {
        Ok(self.users.iter().find(|u| u.email == email).cloned())
    }
}

#[derive(HexDirective)]
struct SignUpUser { email: String }

fn execute_signup<R>(repo: &mut R, cmd: SignUpUser) -> HexResult<User>
where R: UserRepository + hexser::ports::repository::QueryRepository<User, Filter = UserFilter> {
    cmd.validate()?;
    if repo.find_by_email(&cmd.email)?.is_some() {
        return Err(Hexserror::conflict("Email already registered")
            .with_next_step("Use a different email"));
    }
    let id = format!("user-{}", repo.count(&UserFilter::All)? + 1);
    let user = User { id, email: cmd.email };
    repo.save(user.clone())?;
    Ok(user)
}

fn main() -> HexResult<()> {
    let mut repo = InMemoryUserRepository::default();
    let user = execute_signup(&mut repo, SignUpUser { email: "a@b.com".into() })?;
    println!("created {} ({})", user.id, user.email);
    let graph = HexGraph::current();
    graph.pretty_print();
    Ok(())
}
```

Cargo: `hexser = { version = "0.6", features = ["macros"] }` (macros default on). Add `visualization` and/or `ai` for graph JSON / AI export.

## Defining a custom port (supertrait on `Repository<T>`)

A custom port is a trait that names domain-specific capabilities on top of the generic `Repository<T>` save contract. Bound it with a supertrait so any implementor must also be a `Repository<User>`:

```rust
use hexser::prelude::*;

trait UserRepository: Repository<User> {
    fn find_by_email(&self, email: &str) -> HexResult<Option<User>>;
}
```

Use-case code then bounds its repo generically on the custom port (and, when it also needs reads, on `QueryRepository`) rather than on any concrete adapter — see `execute_signup` above. To register the port into the graph, apply `#[derive(HexPort, HexRepository)]` to a port marker type (`HexPort` defaults to `Role::Repository`; `HexRepository` is a pure validation marker that emits no code). See derives.md for the derive-vs-trait breakdown.

## Domain-owned `Filter` / `SortKey` types

The domain — not the adapter — owns how its entities are queried. Define `Filter` and `SortKey` as domain enums, then bind them as the associated types on `QueryRepository`:

```rust
#[derive(Clone)]
enum UserFilter { All, ById(String), ByEmail(String) }
#[derive(Clone, Copy)]
enum UserSortKey { Id, Email }
```

`find` receives a `FindOptions<Self::SortKey>` with fields `sort: Option<Vec<Sort<K>>>`, `limit: Option<u32>`, `offset: Option<u64>` (`FindOptions` implements `Default`). Each `Sort<K>` is `{ key: K, direction: Direction }` where `Direction` is `enum { Asc, Desc }` (Copy). Keeping these types in the domain means every adapter speaks the same query vocabulary.

## Implementing BOTH `Repository` (save) and `QueryRepository` (reads/delete) on one adapter

`Repository<T>` is save-only; `QueryRepository<T>` carries the reads and deletion. A single adapter implements both, plus your custom port. Note the split of responsibilities:

- `Repository<T>`: `fn save(&mut self, entity: T) -> HexResult<()>` (upsert semantics are yours to define).
- `QueryRepository<T>`: `type Filter`, `type SortKey`, `find_one`, `find`, and the defaulted `exists` (via `find_one`), `count` (via `find`), and `delete_where`.
- Override `delete_where` to support deletion — its 0.5 default returns `Err(port::NOT_IMPLEMENTED = E_HEX_103)`, not `Ok(0)`.

```rust
#[derive(HexAdapter, Default)]
struct InMemoryUserRepository { users: Vec<User> }

impl Repository<User> for InMemoryUserRepository {
    fn save(&mut self, entity: User) -> HexResult<()> {
        if let Some(existing) = self.users.iter_mut().find(|u| u.id == entity.id) {
            *existing = entity;
        } else { self.users.push(entity); }
        Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;
    fn find_one(&self, filter: &UserFilter) -> HexResult<Option<User>> { /* ... */ }
    fn find(&self, filter: &UserFilter, _opts: FindOptions<UserSortKey>) -> HexResult<Vec<User>> { /* ... */ }
    fn delete_where(&mut self, filter: &UserFilter) -> HexResult<u64> { /* ... */ }
}
```

See the full example above for complete method bodies. There is no generic `InMemoryRepository` in hexser — write an in-memory (or database-backed) repository per entity.

## A CQRS Directive plus hand-written `DirectiveHandler`

CQRS splits writes (directives) from reads (queries). A `Directive` is a command that can `validate()` itself; a `DirectiveHandler<D>` executes it. Derive `HexDirective` to register the command and auto-implement `Directive::validate` (returns `Ok(())`), then hand-write the handler:

```rust
use hexser::prelude::*;

#[derive(HexDirective)]
struct CreateTodo { title: String }

struct CreateTodoHandler;

impl DirectiveHandler<CreateTodo> for CreateTodoHandler {
    fn handle(&self, directive: CreateTodo) -> HexResult<()> {
        directive.validate()?;
        // persist via a Repository, publish a DomainEvent, etc.
        Ok(())
    }
}
```

`DirectiveHandler<D> where D: Directive` requires `fn handle(&self, directive: D) -> HexResult<()>`. The read side mirrors this: derive `HexQuery` on a query struct (registers under `Layer::Application`, `Role::Query`; no trait impl emitted) and hand-write a `QueryHandler<Q, R>` whose `fn handle(&self, query: Q) -> HexResult<R>` returns the projection. When a handler needs to mutate a repository from `&self`, wrap the repo in interior mutability (e.g. `RefCell`) or hold an owning collaborator — `save` takes `&mut self`. For simpler flows, a plain use-case free function bounded on the port traits (like `execute_signup`) is equally valid.

## After you build

Run `cargo check` to confirm the ports, adapters, and handlers all line up. Then, in `main` (or a test), inspect the registered architecture:

```rust
let graph = HexGraph::current();
graph.pretty_print();
```

`HexGraph::current()` returns the process-wide `Arc<HexGraph>` built once from the link-time registry; `pretty_print()` writes a layer summary to stdout so you can confirm every domain type, port, adapter, and directive registered where you expect. Enable the `visualization` feature for `to_dot()` / `to_mermaid()` / `to_json()`, or `ai` for `to_ai_context()`.
