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
hexser = "0.4.4"  # macros enabled by default via the `macros` feature
```

Advanced users who know they only need the macros can depend directly, but this is uncommon:

```toml
[dependencies]
hexser_macros = "0.4.4"
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

## Knowledge Graph (high level)

- hexser_macros
  - provides: derive/attribute macros used by `hexser`
  - consumed by: `hexser` (compile time)
  - typical user dependency: `hexser` (not this crate directly)
