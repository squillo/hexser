# hexser — Claude Code plugin

A Claude Code plugin for [**hexser**](https://github.com/squillo/hexser), the
zero-boilerplate Hexagonal Architecture framework for Rust. It teaches Claude the
hexser 0.7 API and grain so it writes code that compiles the first time, and it adds
slash commands for scaffolding, review, visualization, migration, and MCP wiring.

## Install

```
/plugin marketplace add squillo/hexser
/plugin install hexser@hexser
```

Then reload (`/reload-plugins`) or restart Claude Code. That's it — the **skill**
auto-activates whenever you work on Rust code that uses `hexser`.

> The marketplace lives at the root of the hexser repo, so `squillo/hexser` is both
> the crate's home and the plugin marketplace.

## What's inside

### Skill: `hexser`
Auto-invoked when you edit hexser code or a `Cargo.toml` that lists `hexser`. It carries
the "golden rules" that prevent the common mistakes (e.g. `#[derive(HexEntity)]` requires
an `id` field; `Repository` is save-only and reads live on `QueryRepository`; error
variants are boxed; writes go through `Actor`; adapters prove their port with
`implements(..)`) and routes to seven fact-checked reference docs:

| Reference | Covers |
|-----------|--------|
| `patterns.md` | Domain → Port → Adapter → use-case, full compiling example |
| `api.md` | Prelude exports, core trait signatures (incl. `Actor`), deprecations, features table |
| `derives.md` | Every derive macro: what it generates, the `#[hex(...)]` keys it reads, gotchas |
| `errors.md` | `Hexserror`, layer errors, codes, rich-error builders, `Hexserror::refused` |
| `graph-and-ai.md` | `HexGraph`, `findings()`, DOT/Mermaid/JSON, `AIContext`/`AgentPack` |
| `mcp.md` | Running & wiring the hexser MCP server |
| `migration-0.4-to-0.5.md` | Every 0.4 → 0.5 breaking change and its fix |

For 0.6 → 0.7, the source of truth is
[`MIGRATING_0.6_TO_0.7.md`](https://github.com/squillo/hexser/blob/main/hexser/MIGRATING_0.6_TO_0.7.md)
in the hexser repository. `/hexser:migrate` carries its steps.

### Commands

| Command | Does |
|---------|------|
| `/hexser:new` | Scaffold a hexser project or a Domain/Port/Adapter slice |
| `/hexser:add-adapter` | Implement an adapter for an existing port |
| `/hexser:add-usecase` | Add a write-side `Actor` (or a read-side Query + `QueryHandler`) |
| `/hexser:review` | Review code against hexser best practices & layering |
| `/hexser:graph` | Render the architecture graph (pretty-print / DOT / Mermaid) |
| `/hexser:migrate` | Migrate a crate from hexser 0.6 to 0.7 (or the older 0.4 to 0.5) |
| `/hexser:mcp-setup` | Install & wire the hexser MCP server for this project |

## The MCP angle

hexser ships an MCP server (`hex-mcp-server`, behind the `mcp` feature) that exposes your
project's architecture graph and AI context as MCP resources. `/hexser:mcp-setup` installs
it and wires it into the project so Claude can read your live architecture — not just your
source. It's opt-in per project (it isn't auto-started globally, so it never interferes with
non-hexser repos). See `skills/hexser/references/mcp.md`.

## Requirements

- Rust (edition 2024, rustc ≥ 1.85) and `cargo` on PATH.
- `hexser = "0.7"` in the target project. The MCP server additionally needs the `mcp` feature.

## License

MIT OR Apache-2.0, matching hexser.
