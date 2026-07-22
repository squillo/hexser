---
description: Implement an adapter for an existing hexser port
argument-hint: "[PortTrait] [AdapterName]"
allowed-tools: Read, Write, Edit, Bash(cargo *), Grep, Glob
---

Implement a concrete adapter named `$2` for the existing hexser port trait `$1`.

If `$1` (the port trait) or `$2` (the adapter name) is missing, do not guess blindly:
- Use Grep/Glob to list candidate port traits (search for `trait .*: *Repository`, `#[derive(HexPort` and `#[hex(role = "InputPort")]`) and ask the user which port to implement and what to name the adapter.
- Only proceed once both a port trait and an adapter name are known.

Follow these steps precisely. Prefer `use hexser::prelude::*;` and keep every hexser API name exactly as written here.

## 1. Locate the port trait

- Grep the codebase for the trait definition `trait $1` (also try `pub trait $1`). Read the file it lives in.
- Determine the trait's supertrait bound and entity type. In hexser a custom port is a trait with a supertrait bound, e.g. `pub trait UserRepository: Repository<User> { fn find_by_email(&self, email: &str) -> HexResult<Option<User>>; }`.
- Classify the port:
  - Repository port — its supertrait is `Repository<T>` (from the prelude). Handle per section 3.
  - Non-repository port — e.g. it declares `InputPort<Input, Output>`, `OutputPort<Request, Response>`, `UseCase<Input, Output>`, or `Query<Params, Result>` methods, or is a plain custom trait. Handle per section 4.
- Read the entity type `T` (e.g. `User`) so you implement the right generic parameter. `T` must implement `HexEntity`.

## 2. Create the adapter struct

Create the struct `$2` with `#[derive(HexAdapter)]`. The derive emits `impl Adapter for $2 {}` and registers the type at `Layer::Adapter`, `Role::Adapter` — so do NOT hand-write `impl Adapter`. Give it whatever backing state the adapter needs (an in-memory `Vec<T>`, a client/pool handle, etc.). `#[derive(Default)]` is convenient for in-memory adapters.

```rust
use hexser::prelude::*;

#[derive(HexAdapter, Default)]
struct $2 {
    // backing store or connection handle, e.g.:
    items: std::vec::Vec<T>,
}
```

(If the port is a mapper rather than a data adapter, override the role with `#[hex(role = "Mapper")]`.)

## 3. If it is a Repository port

Implement BOTH traits. `Repository<T>` is SAVE-ONLY; all reads live on `QueryRepository<T>`. See skills/hexser/references/patterns.md for the full pattern.

`Repository<T>` (from the prelude) — one required method:

```rust
impl Repository<T> for $2 {
    fn save(&mut self, entity: T) -> HexResult<()> {
        // upsert by id; return Ok(()) on success
        Ok(())
    }
}
```

`QueryRepository<T>` — declare `Filter` and `SortKey`, implement `find_one` and `find`. `exists` (default via `find_one`) and `count` (default via `find`) come for free. The `QueryRepository` trait is re-exported by the prelude; the fully-qualified path `hexser::ports::repository::QueryRepository` also works if you need to disambiguate.

Define concrete `Filter` and `SortKey` types (usually small enums) first:

```rust
#[derive(Clone)]
enum TFilter { All, ById(String) }

#[derive(Clone, Copy)]
enum TSortKey { Id }

impl QueryRepository<T> for $2 {
    type Filter = TFilter;
    type SortKey = TSortKey;

    fn find_one(&self, filter: &Self::Filter) -> HexResult<Option<T>> {
        Ok(match filter {
            TFilter::All => self.items.first().cloned(),
            TFilter::ById(id) => self.items.iter().find(|e| /* e.id == *id */ true).cloned(),
        })
    }

    fn find(&self, filter: &Self::Filter, options: FindOptions<Self::SortKey>) -> HexResult<Vec<T>> {
        // Apply the filter, then honor options.sort (Vec<Sort<Self::SortKey>>,
        // each Sort { key, direction: Direction::Asc | Direction::Desc }),
        // options.limit (Option<u32>) and options.offset (Option<u64>).
        let _ = options;
        Ok(self.items.clone())
    }
}
```

`FindOptions<K>` has fields `sort: Option<Vec<Sort<K>>>`, `limit: Option<u32>`, `offset: Option<u64>` and implements `Default`. `Sort<K>` is `{ key: K, direction: Direction }`; `Direction` is `enum { Asc, Desc }`.

Override `delete_where` ONLY if this adapter supports deletion. Its 0.5 default returns `Err` with code `E_HEX_103` (`NOT_IMPLEMENTED`), so leaving it unimplemented is a valid "deletion not supported" signal. When you do support deletion, return the number of rows removed:

```rust
    fn delete_where(&mut self, filter: &Self::Filter) -> HexResult<u64> {
        let before = self.items.len();
        match filter {
            TFilter::All => self.items.clear(),
            TFilter::ById(id) => self.items.retain(|e| /* e.id != *id */ true),
        }
        Ok((before - self.items.len()) as u64)
    }
```

Finally, implement the named custom port trait `$1` itself, wiring up its extra methods (e.g. `find_by_email`):

```rust
impl $1 for $2 {
    // implement each method declared on the $1 trait
}
```

## 4. If it is NOT a Repository port

Implement the port trait `$1` directly against `$2`, matching each method signature exactly (e.g. `InputPort::execute(&self, input) -> HexResult<Output>`, `OutputPort::send(&self, request) -> HexResult<Response>`, `UseCase::execute`, or `Query::query`). Read the trait to copy signatures verbatim.

## 5. Use rich Hexserror values

Return meaningful errors, not bare strings. Use `Hexserror` constructors and chain next steps / suggestions. See skills/hexser/references/errors.md.

```rust
// resource lookups
return Err(Hexserror::not_found("T", &id)
    .with_next_step("Verify the id exists before loading"));

// uniqueness / write conflicts
return Err(Hexserror::conflict("Entity already exists")
    .with_next_step("Load and update instead of inserting"));

// adapter/infrastructure failures — use an adapter code (E_HEX_200..E_HEX_205)
return Err(Hexserror::adapter("E_HEX_203", "Failed to write record")
    .with_suggestion("Check the connection and retry"));
```

`with_next_step`, `with_next_steps`, `with_suggestion`, and `with_suggestions` are available on every `Hexserror` variant. `HexResult<T>` is `Result<T, Hexserror>`.

## 6. Verify

After writing the code, run `cargo check` and fix any errors before reporting done. If the crate gates macros behind a non-default feature set, check with the appropriate `cargo check --features ...` (the `macros` feature that provides `#[derive(HexAdapter)]` is on by default).

Report: the port you implemented, the adapter type you created, which traits you implemented (`Repository` / `QueryRepository` / the custom port), whether deletion is supported, and the `cargo check` result.
