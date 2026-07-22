# Migrating hexser 0.4 to 0.5

A practical checklist of every breaking change from hexser 0.4.x to 0.5.0 and the exact fix for each. Work top to bottom, then run `cargo check`. For the full 0.5 surface see api.md; for derive details see derives.md.

The `/hexser:migrate` command automates the mechanical parts of this list (the Cargo bump, boxed-error rewrites, and rename passes); review its edits and hand-fix the judgement calls (id-less entities, `delete_where` overrides).

## 1. Bump the version in `Cargo.toml`

```toml
# Before
hexser = { version = "0.4", features = ["macros"] }

# After
hexser = { version = "0.5", features = ["macros"] }
```

`macros` and `static-di` are the default features. Add `ai`, `mcp`, or `visualization` if you use those APIs.

## 2. `Hexserror` variants are now boxed

In 0.5 every `Hexserror` variant carries a `Box<…>` payload: `Domain(Box<DomainError>)`, `Port(Box<PortError>)`, `Adapter(Box<AdapterError>)`, `Validation(Box<ValidationError>)`, `NotFound(Box<NotFoundError>)`, `Conflict(Box<ConflictError>)`.

Direct variant construction must box the payload. Prefer the constructor methods (`Hexserror::domain`, `::port`, `::adapter`, `::validation`, `::validation_field`, `::not_found`, `::conflict`) — they box for you.

```rust
use hexser::prelude::*;

// Before (0.4): direct variant construction took the error by value
let err = Hexserror::Adapter(hexser::error::adapter_error::connection_failed("db", "timeout"));

// After (0.5), option A — box it yourself
let err = Hexserror::Adapter(std::boxed::Box::new(
    hexser::error::adapter_error::connection_failed("db", "timeout"),
));

// After (0.5), option B (preferred) — use the constructor, which boxes for you
let err = Hexserror::adapter("E_HEX_205", "connection failed");
```

Match patterns need no change — the binding captures the `Box`, which derefs transparently:

```rust
match err {
    Hexserror::Adapter(inner) => eprintln!("{inner}"), // unchanged
    _ => {}
}
```

## 3. `#[derive(HexEntity)]` now requires an `id` field

The 0.4 derive silently defaulted `type Id = String`. In 0.5 the derive reads `type Id` from a struct field literally named `id`; a type with no `id` field is a compile error.

For a real entity, add an `id` field:

```rust
use hexser::prelude::*;

// Before (0.4): compiled via the silent Id = String default
#[derive(HexEntity)]
struct User { email: String }

// After (0.5): give it an id field
#[derive(HexEntity)]
struct User { id: String, email: String }
```

Or hand-implement the trait with the identity type you want:

```rust
use hexser::prelude::*;

struct User { email: String }

impl HexEntity for User {
    type Id = String;
}
```

For plain DTOs that are not entities (they were only relying on the old silent `Id = String` default), remove the derive entirely:

```rust
// Before (0.4)
#[derive(HexEntity)]
struct CreateUserRequest { email: String, name: String }

// After (0.5): not an entity — drop the derive
struct CreateUserRequest { email: String, name: String }
```

## 4. `AIContext::to_json` and `AgentPack::to_json` return `HexResult<String>`

Both were `Result<String, String>` in 0.4 and are now `HexResult<String>` (feature `ai`). Update call sites to thread the `Hexserror` with `?` or `match`.

```rust
use hexser::prelude::*;

// Before (0.4)
let json: String = context.to_json().map_err(|s| /* String */ s)?;

// After (0.5): error is a Hexserror, propagate directly
fn dump(context: &AIContext) -> HexResult<String> {
    let json = context.to_json()?; // HexResult<String>
    Ok(json)
}
```

The same applies to `AgentPack::to_json(&self) -> HexResult<String>`.

## 5. `QueryRepository::delete_where` default now errors

The default `delete_where` implementation returned `Ok(0)` in 0.4. In 0.5 it returns `Err` with code `E_HEX_103` (`NOT_IMPLEMENTED`). Override it on any adapter that actually supports deletion.

```rust
use hexser::prelude::*;

// After (0.5): override delete_where so it does real work instead of erroring
impl hexser::ports::repository::QueryRepository<User> for InMemoryUserRepository {
    type Filter = UserFilter;
    type SortKey = UserSortKey;

    fn find_one(&self, filter: &UserFilter) -> HexResult<Option<User>> { /* ... */ }
    fn find(&self, filter: &UserFilter, opts: FindOptions<UserSortKey>) -> HexResult<Vec<User>> { /* ... */ }

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
```

If an adapter genuinely cannot delete, leave the default and expect callers to receive `E_HEX_103`.

## 6. Trait renames: `Entity` -> `HexEntity`, `ValueObject` -> `HexValueItem`

The old `Entity` alias was removed in 0.5 — there is no compatibility shim. Rename both the trait references and the derives.

```rust
use hexser::prelude::*;

// Before (0.4)
impl Entity for User { type Id = String; }
impl ValueObject for Email {
    fn validate(&self) -> HexResult<()> { Ok(()) }
}

// After (0.5)
impl HexEntity for User { type Id = String; }
impl HexValueItem for Email {
    fn validate(&self) -> HexResult<()> { Ok(()) }
}
```

The derive macros `HexEntity` and `HexValueItem` are unchanged in name and are exported from the prelude. See derives.md.

## 7. Rich-error guidance is now retained on all variants

In 0.5 `with_next_step`, `with_next_steps`, `with_suggestion`, and `with_suggestions` are honored on all six `Hexserror` variants (in 0.4 some variants silently dropped them). No API change is required, but guidance you attach to `NotFound`, `Validation`, and `Conflict` errors will now actually appear.

```rust
use hexser::prelude::*;

// Now retained on every variant, including NotFound / Validation / Conflict
let err = Hexserror::conflict("Email already registered")
    .with_next_step("Use a different email")
    .with_suggestion("Try password reset instead");

let missing = Hexserror::not_found("User", "user-42")
    .with_next_step("Verify the id exists before fetching");
```

## Finish: run `cargo check`

```sh
cargo check
```

The two most common breakages are the boxed-error constructions (section 2) and the id-less `HexEntity` derives (section 3) — fix those first and the rest of the errors usually clear. Remember that `/hexser:migrate` automates the mechanical parts of this checklist.
