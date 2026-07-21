# hexser_macros — Procedural macros for the hexser crate

[![Crates.io](https://img.shields.io/crates/v/hexser_macros.svg)](https://crates.io/crates/hexser_macros)
[![Documentation](https://docs.rs/hexser_macros/badge.svg)](https://docs.rs/hexser_macros)
[![License](https://img.shields.io/crates/l/hexser_macros.svg)](https://github.com/squillo/hexser)

A small companion utility crate for the main [hexser](https://crates.io/crates/hexser) library.

This crate provides the procedural macros that power hexser’s zero‑boilerplate Hexagonal Architecture experience (Ports & Adapters). Most users should depend on `hexser` directly and let it enable/use these macros. Depend on `hexser_macros` directly only if you have an advanced use case and know you specifically need the raw macros.

---

## Relationship to hexser (recommended usage)

In typical projects you do not need to add `hexser_macros` explicitly. The main crate re‑exports and uses these macros for you.

```toml
[dependencies]
hexser = "0.4.7"  # macros enabled by default via the `macros` feature
```

Advanced users who know they only need the macros can depend directly, but this is uncommon:

```toml
[dependencies]
hexser_macros = "0.4.7"
```

> Note: The derive and attribute macros are designed to work with the types/traits defined in `hexser`. Using them standalone usually requires `hexser` in your dependency tree anyway.

---

## Version compatibility

- Keep `hexser_macros` and `hexser` on the same minor/patch version (e.g., `0.3.x`).
- When upgrading `hexser`, upgrade `hexser_macros` to the matching version.

---

## Links

- Main crate (recommended): https://crates.io/crates/hexser
- Docs: https://docs.rs/hexser_macros
- Repository: https://github.com/squillo/hexser
- Publishing guide: ./../../PUBLISHING.md

---

## License

Licensed under either of

- Apache License, Version 2.0, or
- MIT license

at your option.

---

## Derive macros

All are re-exported from `hexser` (and its prelude) under the `macros` feature, so use
`hexser::HexDomain` etc. rather than depending on this crate directly.

| Derive | Effect | Registered role |
|--------|--------|-----------------|
| `HexDomain` | `Registrable` + graph registration | `Entity` (override: `#[hex(role = "…")]`) |
| `HexPort` | `Registrable` + graph registration | `Repository` (override: `#[hex(role = "InputPort")]`) |
| `HexAdapter` | `Registrable` + `Adapter` marker + registration | `Adapter` (override via `#[hex(role)]`) |
| `HexDirective` | `Directive` impl + registration | `Directive` |
| `HexQuery` | `Registrable` + registration | `Query` |
| `HexEntity` | `HexEntity` impl (`Id` from the `id` field) | — |
| `HexValueItem` | `HexValueItem` impl (default validation) | — |
| `HexAggregate` | `Aggregate` impl (default invariant check) | — |
| `HexRepository` | Semantic marker (pair with `HexPort`) | — |

Notes:
- Derives apply to structs/enums, not traits.
- Generic types compile (the `Registrable` impl is generated with a `Self: 'static` bound),
  but they are not auto-registered in the inventory graph — register a concrete alias instead.
- `HexEntity` on a struct without an `id` field is a compile error (choose the `Id` explicitly).

## Knowledge Graph (high level)

- hexser_macros
  - modules:
    - `derive` — the nine derive implementations above
    - `common::codegen` — shared `Registrable` + `inventory::submit!` code generation and
      `#[hex(role = "…")]` parsing
    - `common::validation` — target validation (struct/enum only)
  - provides: derive macros used by `hexser`
  - consumed by: `hexser` (compile time)
  - typical user dependency: `hexser` (not this crate directly)
  - tests: `hexser/tests/derive_registration_test.rs` (runtime graph registration),
    `hexser/tests/macro_tests.rs` (derive usage), `hexser/tests/trybuild_ui.rs` (compile-fail)
