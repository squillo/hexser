---
description: Migrate a crate from hexser 0.4 to 0.5
argument-hint: "[path (optional)]"
allowed-tools: Read, Edit, Write, Bash(cargo *), Grep, Glob
---

Migrate a Rust crate from hexser 0.4 to hexser 0.5. Follow
`skills/hexser/references/migration-0.4-to-0.5.md` step by step, applying it to
`$ARGUMENTS` when a path is given, or to the current crate when `$ARGUMENTS` is empty.

Work carefully, file-by-file. Read each file before editing it. Never run git.

## Step 0 — locate the crate

- If `$1` is provided, treat it as the crate/workspace path and confine edits to it.
- Otherwise operate on the current crate. Use Glob to find `Cargo.toml` and `**/*.rs`,
  and Grep to find the change sites listed below. Read `skills/hexser/references/migration-0.4-to-0.5.md`
  first so the transforms below stay consistent with the reference.

## Step 1 — bump the dependency

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

## Step 2 — box direct `Hexserror` variant constructions

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

## Step 3 — apply the trait renames

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

## Step 4 — fix id-less `#[derive(HexEntity)]`

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

## Step 5 — update `to_json` call sites

`AIContext::to_json(&self)` and `AgentPack::to_json(&self)` now return `HexResult<String>`
(they previously returned `Result<String, String>`). Update call sites to treat the error
as `Hexserror` — usually just propagate with `?` inside a function returning `HexResult<_>`.

```rust
let json = ai_context.to_json()?; // HexResult<String>
let pack_json = agent_pack.to_json()?; // HexResult<String>
```

These APIs live behind the `ai` feature; only touch them if the crate uses `ai`.

## Step 6 — override `delete_where` where needed

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

## Step 7 — check and report

Run `cargo check` (add `--features ai` / `--features visualization` if the crate uses those
APIs). Fix any newly-surfaced errors that map to the steps above, then re-check.

Report the outcome concisely:

- files changed and which transform was applied to each,
- whether `cargo check` passes,
- any remaining compiler errors that don't fit the steps above (quote the error and file:line
  and suggest a fix), grouped so the user can act on them.

Do not run git.
