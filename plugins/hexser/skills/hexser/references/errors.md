# hexser errors

Everything fallible in hexser returns `HexResult<T>`, and the error type is the `Hexserror` enum. This reference covers the result alias, the enum and its boxed variants, the constructors and rich builders, the underlying layer error types, the free constructor helpers, the error-code table, and the serde source-location note. Source: `hexser/src/error/`. See patterns.md for how errors flow through repositories and directives.

```rust
use hexser::prelude::*; // brings HexResult and Hexserror into scope
```

## HexResult

```rust
// HexResult<T> = Result<T, Hexserror>
fn find(&self, id: &str) -> HexResult<Option<User>> { /* ... */ }
```

Every trait method in hexser (`save`, `validate`, `check_invariants`, `handle`, `execute`, `map`, ...) returns `HexResult<T>`. Use `?` to propagate.

## The `Hexserror` enum

Defined in `hexser/src/error/hex_error.rs`. In 0.5 **every variant payload is `Box<…>`** (keeps the enum small on the stack):

```rust
pub enum Hexserror {
    Domain(Box<DomainError>),
    Port(Box<PortError>),
    Adapter(Box<AdapterError>),
    Validation(Box<ValidationError>),
    NotFound(Box<NotFoundError>),
    Conflict(Box<ConflictError>),
}
```

It implements `Display` and `std::error::Error` (with source chaining), and derives `Serialize`/`Deserialize` under the `serde` feature.

## Constructors (they box for you)

Prefer these associated functions — they build the inner error and box it into the right variant, so you never touch `Box` yourself:

```rust
use hexser::prelude::*;

Hexserror::domain("E_HEX_002", "Order must have items");     // Domain
Hexserror::port("E_HEX_100", "Upstream unreachable");        // Port
Hexserror::adapter("E_HEX_201", "Stripe returned 500");      // Adapter

Hexserror::validation("Email is not a valid format");        // Validation, code E_HEX_301
Hexserror::validation_field("Name is required", "name");     // Validation, code E_HEX_300
Hexserror::not_found("User", "user-42");                     // NotFound, code E_HEX_400
Hexserror::conflict("Email already registered");             // Conflict, code E_HEX_402
```

## Direct variant construction must box

If you construct a variant directly (for example from a free constructor helper that returns a concrete layer error — see below), wrap the payload in `std::boxed::Box::new(...)`:

```rust
use hexser::prelude::*;

let adapter_err = hexser::error::adapter_error::api_failure("Stripe returned 500");
let err = Hexserror::Adapter(std::boxed::Box::new(adapter_err));
```

Matching is unaffected — the pattern binds the `Box`, so no changes are needed there:

```rust
match err {
    Hexserror::Adapter(inner) => { /* inner: Box<AdapterError> */ }
    Hexserror::NotFound(inner) => { /* inner: Box<NotFoundError> */ }
    _ => {}
}
```

## Rich builders

Chainable builders on `Hexserror`. Guidance-oriented builders apply to **all six variants** (fixed in 0.5); the rest are variant-specific:

| Builder | Applies to |
| --- | --- |
| `with_next_step(step)` / `with_next_steps(steps)` | all six variants |
| `with_suggestion(s)` / `with_suggestions(ss)` | all six variants |
| `with_field(field)` | `Validation` only |
| `with_existing_id(id)` | `Conflict` only |
| `with_source(err)` | chains cause on `Domain` / `Port` / `Adapter`; no-op elsewhere |

`with_source` takes `impl std::error::Error + Send + Sync + 'static` and adds it to the error's source chain.

```rust
use hexser::prelude::*;

let err = Hexserror::validation_field("Name is required", "name")
    .with_next_step("Provide a non-empty name")
    .with_suggestion("user.name = \"Ada\".into()")
    .with_field("name");

let conflict = Hexserror::conflict("Email already registered")
    .with_existing_id("user-42")
    .with_next_step("Sign in instead");

// with_source chains an underlying cause on layer variants
let wrapped = Hexserror::adapter("E_HEX_205", "connection failed")
    .with_source(std::io::Error::new(std::io::ErrorKind::Other, "socket closed"));
```

## Layer error types

The inner payloads are the layer error types in `hexser/src/error/layer_error.rs`:

- `DomainError`, `PortError`, `AdapterError` are `LayerError<L>` aliases and implement the `RichError` trait: `with_next_step`/`with_next_steps`, `with_suggestion`/`with_suggestions`, `with_location`, `with_more_info`, `with_source`.
- `NotFoundError`, `ValidationError`, `ConflictError` are standalone types (0.5 added next-steps/suggestions plus their builders).

You normally interact with these only through the `Hexserror` constructors above; construct them directly only when using the free helpers.

## Free constructor helpers

These free functions return a **concrete layer error** (not a `Hexserror`). Box them into the matching variant to get a `Hexserror` (see "Direct variant construction must box"):

```rust
// hexser::error::adapter_error
hexser::error::adapter_error::connection_failed(/* ... */);
hexser::error::adapter_error::api_failure(/* ... */);
hexser::error::adapter_error::mapping_failure(/* ... */);
hexser::error::adapter_error::io_failure(/* ... */);

// hexser::error::domain_error
hexser::error::domain_error::invariant_violation(/* ... */);
hexser::error::domain_error::invalid_state_transition(/* ... */);
hexser::error::domain_error::invariant_empty(/* ... */);

// hexser::error::port_error
hexser::error::port_error::communication_failure(/* ... */);
hexser::error::port_error::port_not_found(/* ... */);
hexser::error::port_error::port_timeout(/* ... */);
```

There are no free `not_found` or `validation_field` helpers — those exist only as `Hexserror` associated functions.

## Error codes

Codes live in `hexser/src/error/codes.rs`, re-exported as `hexser::error_codes`. Every code has the form `E_HEX_<n>`:

| Code | Constant | Meaning |
| --- | --- | --- |
| `E_HEX_001` | `INVARIANT_EMPTY` | domain: required value empty |
| `E_HEX_002` | `INVARIANT_VIOLATION` | domain: invariant violated |
| `E_HEX_003` | `INVALID_STATE_TRANSITION` | domain: illegal state change |
| `E_HEX_100` | `COMMUNICATION_FAILURE` | port: communication failed |
| `E_HEX_101` | `PORT_NOT_FOUND` | port: port not found |
| `E_HEX_102` | `PORT_TIMEOUT` | port: timed out |
| `E_HEX_103` | `NOT_IMPLEMENTED` | port: operation not implemented (default `delete_where`) |
| `E_HEX_200` | `DB_CONNECTION_FAILURE` | adapter: DB connection failed |
| `E_HEX_201` | `API_FAILURE` | adapter: external API failed |
| `E_HEX_202` | `MAPPING_FAILURE` | adapter: mapping failed |
| `E_HEX_203` | `DB_WRITE_FAILURE` | adapter: DB write failed |
| `E_HEX_204` | `DB_READ_FAILURE` | adapter: DB read failed |
| `E_HEX_205` | `CONNECTION_FAILURE` | adapter: generic connection failed |
| `E_HEX_300` | `REQUIRED_FIELD` | validation: required field missing |
| `E_HEX_301` | `INVALID_FORMAT` | validation: invalid format |
| `E_HEX_302` | `OUT_OF_RANGE` | validation: value out of range |
| `E_HEX_400` | `NOT_FOUND` | resource: not found |
| `E_HEX_401` | `ALREADY_EXISTS` | resource: already exists |
| `E_HEX_402` | `CONFLICT` | resource: conflict |
| `E_HEX_500` | `FILE_NOT_FOUND` | io: file not found |
| `E_HEX_501` | `PERMISSION_DENIED` | io: permission denied |
| `E_HEX_502` | `IO_FAILURE` | io: I/O failure |

## Real construction example

Taken from `hexser/src/domain/aggregate.rs` — a domain invariant failure with actionable guidance:

```rust
use hexser::prelude::*;

Hexserror::domain("E_HEX_001", "Order must have items")
    .with_next_step("Add at least one item")
    .with_suggestion("order.add_item(item)")
```

## Serde source-location note

When errors are serialized (feature `serde`), source locations are **hidden by default**. Set the environment variable `HEXSER_INCLUDE_SOURCE_LOCATION=1` to include them in the serialized output.
