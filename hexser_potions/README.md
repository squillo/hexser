# hexser_potions — Potions for the hexser crate

[![Crates.io](https://img.shields.io/crates/v/hexser_potions.svg)](https://crates.io/crates/hexser_potions)
[![Documentation](https://docs.rs/hexser_potions/badge.svg)](https://docs.rs/hexser_potions)
[![License](https://img.shields.io/crates/l/hexser_potions.svg)](https://github.com/squillo/hexser)

A small companion utility crate to the main [hexser](https://crates.io/crates/hexser) library.

“Potions” are ready‑to‑mix patterns, examples, and small helpers that demonstrate effective use of hexagonal architecture with `hexser`. This crate is optional and aimed at learning, scaffolding, and testing. Use it when you want copy‑friendly examples and patterns that pair naturally with `hexser`.

---

## Relationship to hexser (recommended usage)

You will almost always depend on `hexser` directly for your application code and add `hexser_potions` as a development aid:

```toml
[dependencies]
hexser = "0.4.1"

[dev-dependencies]
hexser_potions = "0.4.1"
```

Nothing in `hexser_potions` is required for production use of `hexser`; it simply accelerates discovery, experimentation, and testing.

---

## Version compatibility

- Keep `hexser_potions` on the same minor/patch version as `hexser` (e.g., `0.3.x`).
- When upgrading `hexser`, upgrade `hexser_potions` to the matching version.

---

## Links

- Main crate: https://crates.io/crates/hexser
- Docs: https://docs.rs/hexser_potions
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

- hexser_potions
  - depends on: `hexser`
  - provides: examples, patterns, and test/learning helpers
  - typical user dependency: `hexser` (prod), `hexser_potions` (dev/learning)
