---
description: Render the hexser architecture graph
argument-hint: "[pretty|dot|mermaid|json]"
allowed-tools: Read, Write, Edit, Bash(cargo *)
---

Render the hexser architecture graph for the current crate.

Requested format: `$1` (default: `pretty` when no argument is given).

Valid formats: `pretty`, `dot`, `mermaid`, `json`. If `$1` is anything else, tell the user the valid options and stop.

## Background you must know first

`HexGraph::current()` returns an `Arc<HexGraph>` built once, at link time, from the inventory registry that the derive macros (`HexDomain`, `HexPort`, `HexAdapter`, `HexDirective`, `HexQuery`, ...) populate via `inventory::submit!`.

CRITICAL: the graph is only populated by components that are actually LINKED into the running process. A library crate that is merely compiled does not run any linking of its own; you need an executable — a `bin`, an `example`, or a `test` — that pulls the component modules in (so their `inventory::submit!` entries survive the link). If the graph comes back empty, the registered components were not linked. You can check with `graph.is_empty()` / `graph.node_count()` before printing.

For fuller detail on the graph, showcase traits, and AI export, read `skills/hexser/references/graph-and-ai.md`.

## Format-by-format instructions

### pretty (default — no extra feature)

`pretty_print(&self)` prints a layer summary to stdout. It needs no cargo feature beyond the defaults.

```rust
use hexser::prelude::*;

fn main() {
    let graph = HexGraph::current();
    if graph.is_empty() {
        eprintln!("graph is empty — no components linked into this binary");
    }
    graph.pretty_print();
}
```

### dot / mermaid / json (require the `visualization` feature)

These methods are inherent on `HexGraph` and each returns `HexResult<String>`:

- `to_dot() -> HexResult<String>`
- `to_mermaid() -> HexResult<String>`
- `to_json() -> HexResult<String>`

They are only compiled when the `visualization` feature is on (`visualization = ["serde", "serde_json"]`). Add it to the crate's `Cargo.toml`:

```toml
hexser = { version = "0.6", features = ["macros", "visualization"] }
```

Then, for whichever format was requested:

```rust
use hexser::prelude::*;

fn main() -> HexResult<()> {
    let graph = HexGraph::current();
    let out = graph.to_dot()?; // or .to_mermaid()? / .to_json()?
    println!("{out}");
    Ok(())
}
```

## What to actually do

1. Parse `$1`. Default to `pretty`. Reject unknown values.
2. Read the crate's `Cargo.toml` to see whether `hexser` is a dependency and which features are enabled (you need `visualization` for `dot`/`mermaid`/`json`). Read before editing.
3. Look for an existing executable that already links the components and calls the graph (search `src/bin/`, `examples/`, or a `main` that uses `HexGraph::current()`). If one exists, run it with the right features via `cargo run`.
4. If none exists, OFFER to add one — pick the least intrusive of:
   - a small `examples/hex_graph.rs` (run with `cargo run --example hex_graph --features visualization`),
   - a tiny `src/bin/hex_graph.rs`,
   - or a `#[test]` that prints the requested format (run with `cargo test -- --nocapture`).
   Make sure the chosen executable references the modules that hold the registered components, otherwise the graph will be empty. For `dot`/`mermaid`/`json`, also add the `visualization` feature (edit `Cargo.toml`) before running.
5. For AIContext JSON specifically, you can instead run the existing `hex-ai-export` bin (feature `ai`) from the hexser crate:

   ```bash
   cargo run -p hexser --features ai --bin hex-ai-export > context.json
   ```

   Note this reflects the components linked into that binary; to export YOUR crate's context, add a small `ai`-gated bin/example/test in your crate instead.
6. After any `Cargo.toml` or source edit, run `cargo check` (with the relevant `--features`) to confirm it compiles, then run the chosen command and show the output.

Do not run `git`. Do not claim any MCP server is running — MCP is opt-in via `/hexser:mcp-setup`.
