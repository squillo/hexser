# SNAPP_CONTRACT — hexser_potions

- **Snapp tier:** BE (backend example/preset library — "ready-to-mix" copy-paste patterns
  demonstrating idiomatic hexagonal-architecture component shapes; not itself a deployed
  service, but authored and structured as a backend-flavored consumer of `hexser`).

- **PRD-272 status:** conforming as of 2026-07-21 — no exceptions found; this crate has no
  locks, no iterated maps, and no cross-crate wire of its own to bring into conformance.

- **Boundary ports (trait Ports exposed to other crates/Snapps):** none of its own. This crate
  is a pure consumer of `hexser`'s ports (re-exported at the crate root as `pub use hexser as
  core;` in `hexser_potions/src/lib.rs`). Its two modules illustrate — rather than define new —
  Port usage:
  - `auth` (`hexser_potions/src/auth/mod.rs`) — a minimal signup flow built against a
    repository port and a directive.
  - `crud` (`hexser_potions/src/crud/mod.rs`) — a simple in-memory CRUD repository example
    implementing hexser's `Repository`/`QueryRepository` shape.

  `hexser_potions` depends on `hexser` only (per `hexser_potions/Cargo.toml`/`lib.rs`); it does
  not depend on `hexser_macros` directly, nor expose any trait that another crate implements
  against.

- **Wire format at the boundary:** none — no cross-crate message boundary exists in this crate;
  examples are compiled, in-process Rust demonstrating `hexser`'s in-process trait dispatch.

- **Shared-read hot state (ArcSwap surfaces):** none — this crate holds no shared runtime state
  of its own; any hot state it touches (e.g. `HexGraph::current()`, if a potion registers
  components) is `hexser`'s, governed by `hexser/SNAPP_CONTRACT.md`.

- **Locks:** none — no `RwLock`/`Mutex`/`OnceLock`/`ArcSwap` in this crate's own source.

- **Deterministic iteration:** follows `hexser`: no `HashMap`/`HashSet` found in
  `hexser_potions/src/{auth,crud}/mod.rs`; should a future potion need an iterated map, it must
  use `indexmap::IndexMap`/`IndexSet` to match the substrate crate's determinism guarantee
  (§3.H), not `std::collections::HashMap`/`HashSet`.

- **N_BOOK grounding:** §10 (Snapp boundary independence) — `hexser_potions` deliberately draws
  no boundary of its own; it is example code inside the `hexser` Port framework's boundary, so
  its conformance is inherited from, not independent of, `hexser/SNAPP_CONTRACT.md`.

_Authored 2026-07-21 under PRD-272 §1.1 (HEXSER-272-CONFORMANCE)._
