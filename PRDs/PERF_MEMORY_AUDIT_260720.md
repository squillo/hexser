---
task_id: hexser-perf-memory-260720
status: planning
author: @AI (audit)
created: 2026-07-20
companion: PRDs/IMPL_PLAN_260720.md
scope: hexser, hexser_macros, hexser_potions
---

# Performance & Memory-Footprint Audit — Findings + Plan

A dedicated 6-dimension perf/memory audit (allocations, struct layout, graph/registry
algorithms, AI/MCP per-request cost, binary/dependency footprint, clone/Arc/caching) with
adversarial verification. The finders raised ~42 candidate findings; independent verifiers
re-checked each against the actual code **and against realistic scale**.

## Headline conclusion

**hexser's runtime memory footprint and per-call cost are already appropriate for what it
is.** Nearly every allocation and struct-layout finding was **refuted on magnitude**, not on
facts. The recurring, correct verifier reasoning:

- The architecture graph is a **build-once** structure of **tens-to-hundreds of nodes**, shared
  behind `Arc<GraphInner>` (so `graph.clone()` is a refcount bump, not a deep copy).
- The AI-context / MCP / visualization paths are **cold**: agent-initiated, one-shot, on a
  **local single-client stdio dev tool** — no hot loop, no high QPS, no many-instance
  amplification.
- Empty `HashMap`/`Vec` fields **don't allocate**; the "always-empty metadata map" and
  "empty next_steps Vecs" cost struct *footprint* only (a few KB once), not churn.
- Most proposed fixes (Cow/`&'static str`/enum-instead-of-String on public structs) are
  **semver-breaking** to serde-serialized public DTOs, trading API stability for an
  unmeasurable win.

So chasing per-node `String` allocations, `format!("{:?}")` on enums, `to_string()` on
`NodeId`, double-serialization in MCP, etc. would add complexity and/or break the API for **no
measurable benefit at this scale**. We are explicitly **not** doing those.

The genuine runtime levers were already identified in the hardening plan and are scheduled:
`HexGraph::current()` caching (`OnceLock`) and the adjacency index for `edges_from/edges_to`
(**PR-7**) — those remove *repeated* work, which is the only thing that matters here.

## What IS worth doing — dependency & compile-time weight

For a **published library**, the footprint that actually costs users is **what they compile
and ship**. This is where the audit found real, verified wins. They reduce downstream compile
time and binary size with **no runtime or API impact**.

### F1 (confirmed, medium) — Drop or trim `chrono`
`chrono` is used in **exactly one place**: `generated_at: chrono::Utc::now().to_rfc3339()`
(`ai/context_builder.rs:60`). It's declared with **default features** (root `Cargo.toml:20`),
so the `ai` feature (and `mcp`, which inherits it) pulls `chrono` + `num-traits` +
`iana-time-zone` into every downstream build that enables AI/MCP — for one metadata timestamp.
- **Fix (preferred):** format RFC3339 from `std::time::SystemTime` with a small
  days-to-civil routine (~20 lines, no deps) and **remove `chrono` entirely**.
- **Fix (minimal):** `chrono = { version = "0.4", default-features = false, features = ["now", "std", "alloc"] }`
  — keeps `Utc::now()` + `to_rfc3339()` but sheds `iana-time-zone` and the wasm tree.
- Non-breaking (internal string only). Note: the timestamp also defeats output caching, so
  removing/pinning it helps a future MCP response cache.

### F2 (flagged, unverified — verify then apply) — Trim `syn` features
Workspace pins `syn = { version = "2.0", features = ["full", "extra-traits"] }`
(`Cargo.toml:10`). After PR-1 removed the error macros, `hexser_macros` only parses
`DeriveInput` and uses `parse_quote!` for types/where-predicates — it almost certainly needs
neither **`full`** (full-expression/item grammar) nor **`extra-traits`** (Debug/Eq/Hash on syn
types). `syn` is one of the heaviest compile-time deps; dropping these features measurably
speeds every build of the macro crate (and thus every downstream build using `macros`, which
is on by default).
- **Fix:** drop to `features = ["derive", "parsing", "printing", "clone-impls", "proc-macro"]`
  (roughly syn's defaults) — or just remove the explicit `full`/`extra-traits`.
- **Must compile-test** `hexser_macros` + the derive tests before committing.

### F3 (flagged, unverified) — `reqwest` dev-dependency weight
`reqwest = { ..., features = ["blocking", "json"] }` (dev-dep) pulls hyper + tokio + a TLS
stack, compiled for the workspace's own tests/examples. It does **not** affect downstream users
(dev-dep), but it slows the project's own `cargo test`. Check whether the single example that
uses it (`weather_adapter`) needs blocking+json+TLS, or can use a lighter client / be
feature-gated / moved so it's not built by default.

### F4 (adjacent, low) — `Arc<HexGraph>` double indirection
`HexGraph::current()` returns `Arc<HexGraph>` although `HexGraph` is already `Arc<GraphInner>`
inside — a redundant second `Arc`. This is **already noted under PR-7** (the `current()` cache
work touches the same signature); fold the double-Arc removal into that change rather than a
separate pass. (Semver-visible: `current()` return type — batch with the v0.5 API work.)

## Explicitly NOT doing (refuted micro-optimizations)

Recording these so they aren't "rediscovered" and re-litigated. All are factually real but
**negligible at hexser's scale**, and most require breaking public DTOs:

- Per-node `String` for `type_name`/`module_path` → `&'static`/`Cow` (breaking; ~2 tiny allocs
  ×N-nodes, once).
- `method_extractor` / `ContextBuilder` rebuilding constant string tables per build (breaking
  DTOs; sub-ms, cold path).
- `format!("{:?}")` / `NodeId::to_string()` per node/edge (cold, dwarfed by JSON serialize).
- `SourceLocation.file` / `specversion` / `datacontenttype` heap-copying constants (cold error
  path / already-large envelope; several are semantically required for wire serialization).
- Always-empty `metadata: HashMap` on `HexNode`/`HexEdge`, `extensions` on envelope (footprint
  only, empty maps don't allocate; boxing them breaks public fields).
- `LayerError` embedding empty `Vec`s + `Option<SourceLocation>` (cold path; boxing is breaking
  and the Hexserror payload is already boxed at the enum level).
- `HashMap<NodeId,_>` SipHashing a u64, `layer_count` HashSet, no `with_capacity`,
  `nodes_by_layer` scans, `pretty_print` 5 scans, DOT/Mermaid `push_str(&format!)` (all
  build-once or diagnostic; ns–µs).
- MCP double/triple serialization + `to_writer` streaming (cold, interactive; `to_string`
  already hands off its buffer without an extra copy).
- `inventory` always-on (tiny crate; startup registration is sub-ms; gating it is breaking for
  a niche).

**If the scale assumptions change** (graphs of 10k+ nodes, or MCP served at high QPS behind a
network transport), revisit the allocation-hot and per-request-cost findings — several become
worthwhile at 100×–1000× the current scale. At today's scale they are not.

## Plan (PR-10 — footprint; independent of PR-6..9)

- [ ] F1: remove/trim `chrono` (prefer full removal via std formatter). Add a `/// why` test
      asserting the generated timestamp parses as RFC3339 and round-trips.
- [ ] F2: trim `syn` features; compile-test `hexser_macros` + derive/trybuild tests.
- [ ] F3: assess and lighten the `reqwest` dev-dep / gate its example.
- [ ] F4: fold `Arc<HexGraph>` double-indirection into PR-7's `current()` change (not here).
- [ ] Add `benches/` (criterion) for `HexGraph::current()` + `to_ai_context()` so the PR-7
      caching/adjacency wins are measured, not assumed — and so any future footprint claim is
      backed by numbers rather than re-audited by hand.

## Current Step
- **Action:** Audit complete and synthesized. No source changed by this document.
- **Details:** The high-value, low-risk work is F1–F3 (dependency weight). F4 belongs with PR-7.
  The runtime micro-opts are intentionally declined with rationale above.

## Blockers
- None. F2/F3 need a compile check before landing (trivial).

---
_Note: the audit's own ground-truth agent (release-build sizing, `cargo tree`) was cut off by a
session limit, so absolute binary/dep numbers weren't captured. Re-run `cargo tree -p hexser
--all-features` and `cargo build --release` sizing when quantifying F1–F3 wins._

_Revision history_
- 2026-07-20T00:00:00Z @AI: Synthesize the verified perf/memory audit; conclusion = runtime footprint already right at scale, real wins are dependency/compile weight (chrono/syn/reqwest).
