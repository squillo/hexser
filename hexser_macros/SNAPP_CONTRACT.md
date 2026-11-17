# SNAPP_CONTRACT — hexser_macros

- **Snapp tier:** XS (compile-time only — a `proc-macro` crate; it has no runtime presence, no
  process boundary, and therefore no Snapp-to-Snapp wire of its own. It exists purely to emit
  code that participates in `hexser`'s XS substrate at the *consuming* crate's compile time.)

- **PRD-272 status:** conforming as of 2026-07-21 — n/a for most of the checklist because there
  is no runtime state, no lock, and no wire (see below); nothing to fix.

- **Boundary ports (trait Ports exposed to other crates/Snapps):** none at runtime — this crate
  exposes twelve `#[proc_macro_derive(...)]` entry points (`HexDomain`, `HexPort`,
  `HexAdapter`, `HexAggregate`, `HexEntity`, `HexValueItem`, `HexRepository`, `HexDirective`,
  `HexQuery`, and in 0.7 `HexActor`, `HexVocabulary`, `HexRefusal`; `hexser_macros/src/lib.rs`) that run inside the *downstream* crate's compiler invocation. It
  does not itself implement or call any `hexser::ports::*` trait. What it emits, for each
  non-generic derive target, is:
  - an `impl ::hexser::registry::Registrable for #name { ... }` (component metadata: layer,
    role, type name, description), and
  - an `::hexser::inventory::submit! { ::hexser::registry::ComponentEntry { ... } }`
    registration (skipped for generic types — see `common/codegen.rs`), plus, for
    `HexDirective` specifically, a submission built directly against `::hexser::inventory` to
    avoid the bare-`inventory::submit!` resolution failure that surfaces in downstream crates
    (`hexser_macros/src/common/codegen.rs`, `hexser_macros/src/derive/directive.rs`);
  - from 0.7, `Depends` edges for `#[hex(depends_on(..))]`; for `#[hex(implements(..))]`, a
    `const` proof that the adapter implements the port's trait plus a
    `::hexser::registry::ImplementsEntry` submission; and for `#[hex(port_trait = ..)]`, the
    port node's hidden `__hexser_port_trait` bound;
  - for `HexVocabulary`/`HexRefusal`, `impl ::hexser::domain::Vocabulary` (and
    `::hexser::error::Refusal`) plus an inherent `from_code` when every variant is a unit
    variant. No registration: a vocabulary is a value type, not a graph node.

  Every derive that reads `#[hex(...)]` reads it through one strict reader (`common/attrs.rs`):
  an unknown key, a malformed value, a duplicate single-valued key, or a key the derive does
  not read is a compile error. `HexAggregate` declares the attribute but reads no key, so on a
  type that derives only `HexAggregate` a `#[hex(...)]` is still ignored. The crate is
  `#![forbid(unsafe_code)]`.

  These registrations are consumed by `hexser` at link time via the `inventory` crate's
  distributed-slice mechanism and read back by `hexser::registry::ComponentRegistry::build_graph`
  when `HexGraph::current()` is first built — i.e. this crate is the codegen half of the
  registration pipeline that feeds hexser's `HexGraph` self-programming substrate; it is not a
  Port consumer or provider itself.

- **Wire format at the boundary:** none — no runtime process boundary exists for a proc-macro
  crate. Its "output" is generated Rust source (`proc_macro::TokenStream`), consumed entirely at
  compile time by `rustc`.

- **Shared-read hot state (ArcSwap surfaces):** none — no runtime state at all.

- **Locks:** none — no runtime state to guard.

- **Deterministic iteration:** n/a — no runtime collections/maps exist in this crate; codegen
  operates on `syn`/`proc_macro2` ASTs, not on iterated hexser Snapp state.

- **N_BOOK grounding:** §10 (Snapp boundary independence) — this crate produces exactly the
  Snapp-registration glue (`Registrable` impls + `inventory::submit!`) that lets downstream
  crates register into hexser's architecture graph without hand-written wiring, while itself
  remaining outside any runtime Snapp boundary.

_Authored 2026-07-21 under PRD-272 §1.1 (HEXSER-272-CONFORMANCE). Updated 2026-11-17 for 0.7
(PRD HEXSER-0.7): three derives, the edge and proof codegen, and the strict reader._
