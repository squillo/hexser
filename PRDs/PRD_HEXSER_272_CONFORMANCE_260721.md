---
prd_id: HEXSER-272-CONFORMANCE
status: in-progress (dispatched 2026-07-21)
parent_canon: PRD-272 (/Users/scott/@squillo/n-rust/nlang_x/PRDs/PRD_272_CROSS_REPO_CROSS_CRATE_COMMUNICATION_CANON.md)
companion: PRDs/IMPL_PLAN_260720.md, PRDs/PERF_MEMORY_AUDIT_260720.md
scope: hexser, hexser_macros, hexser_potions
audience: ORCH + dispatched Sonnet sub-agents
n_book: /Users/scott/@squillo/n/N Language Spec/1.0.0/N_BOOK
---

# PRD — hexser PRD-272 Conformance + remaining hardening

> Every crate boundary IS a current or future Snapp boundary. Cross-crate communication IS
> cross-Snapp communication. — PRD-272 §preamble (USER directive 2026-06-13)

`hexser` is the reference implementation of the **Port abstraction** that PRD-272 §1.4 mandates
for every cross-Snapp call site. It is therefore held to the canon it enables. This PRD brings
the three hexser crates into PRD-272 conformance and finishes the two hardening items deferred
from `IMPL_PLAN_260720.md`, treating the whole workspace as the backend↔frontend bifurcation of
the Actor-based Squillo OS "Self-Programming Application."

The architecture graph (`HexGraph`) is the self-programming substrate: it is **read on hot paths**
(every MCP request via `ProjectRegistry::from_current_graph`, every AI-context export) and
**hot-swapped** when the program rewrites itself (`hexser/refresh`). Per N_BOOK §1.5 (wait-free
RCU) + §23 (Tier 2 SOLE write site) this is exactly the state that MUST live behind
`arc_swap::ArcSwap`, not `OnceLock`/`RwLock`. This is the "lock centralization for game-like
graphic/actor" the USER directed.

## Orchestration protocol (BINDING — read before any CLAIM)

Per PRD-272 change-log precedent (2026-06-26 / 06-27 / 07-10):

- **Sub-agents NEVER run `git` or `cargo` (build/test/check/clippy/fmt/update).** The ORCHESTRATOR
  compiles, tests, and commits at the very end. A 57-process target-lock pileup is the reason.
- **Sub-agents NEVER `branch`, `stash`, `checkout`, `revert`, or `reset`.** They **fix-forward**:
  edit files in place. If a prior agent left a partial edit, correct it forward — never revert.
- Agents edit only the files their CLAIM names. No two live CLAIMs touch the same file.
- Every agent runs the **PRD-272 §5 checklist** on its changed files and reports the result in its
  completion message (it verifies by reading code, not by running tools).
- Dispatch in batches of **≤3** concurrent Sonnets (LESSON #25 burst-limit).
- Every altered/new `.rs` file gets a `//! Revision History` entry (SYS_PROMPT §1). Every test
  carries a `/// why:` line (this repo's standard). No `use` statements — fully-qualified paths.

## Canon conformance targets (evidence, 2026-07-21)

- **§3.G lock surfaces (production):** `hex_graph.rs:66` `OnceLock<Arc<HexGraph>>` (current cache);
  `container/container.rs:27` `tokio::sync::RwLock<HashMap<..>>`. (The `RefCell` event bus is the
  documented single-threaded test/dev adapter — exempt, but see Phase E for the actor-grade path.)
- **§3.H non-deterministic iteration:** 16 `HashMap`/`HashSet` sites (graph adjacency + metadata,
  container services, event-bus handlers, envelope extensions). Nodes already use `BTreeMap`
  (deterministic ✓); adjacency + the rest must move to `IndexMap`/`IndexSet`.
- **§1.1 SNAPP_CONTRACT.md:** none of the three crates has one.
- **§1.4 Port abstraction:** hexser IS the reference; its own `ports/` are the exemplar. No fix
  needed beyond documenting BE/FE/XS tiers in the SNAPP_CONTRACTs.
- **§1.3 postcard boundary:** hexser's export artifacts (`AIContext`/`AgentPack`) are JSON for
  external AI agents — JSON is the correct external wire, NOT an internal cross-Snapp message, so
  postcard does not apply here. MCP uses JSON-RPC by protocol mandate. Recorded, no change.

## Dependencies (ORCHESTRATOR adds before dispatch)

- `arc_swap = "1"` (workspace dep) — wait-free RCU per §1.5.
- `indexmap = { version = "2", features = ["serde"] }` (workspace dep) — deterministic iteration
  per §3.H; `serde` feature so envelope/metadata maps still round-trip.

---

## Phase A — §3.H deterministic iteration (IndexMap/IndexSet)

Swap `std::collections::HashMap`/`HashSet` → `indexmap::IndexMap`/`IndexSet` on the iterated
maps. Fully-qualified paths. Keep `#[cfg_attr(feature="serde", ...)]` derives intact (IndexMap
implements serde under its `serde` feature). Preserve public field types where they are `pub`
(these are v0.5 breaking already; note in the file's revision history).

- **CLAIM-A1 [ORCH]** — `graph/hex_graph.rs`, `graph/builder.rs`, `graph/metadata.rs`: the
  adjacency indices `outgoing`/`incoming` and `metadata.attributes` → `IndexMap`. Bundled with
  the ArcSwap refactor (Phase B) because they share `GraphInner`. ORCH-owned (needs compile).
- **CLAIM-A2 [Sonnet]** — `adapters/in_memory_event_bus.rs`: the `handlers` map
  `HashMap<String, Vec<EventHandler<T>>>` → `indexmap::IndexMap<...>`. Update the `EventHandler`
  type alias site's surrounding struct. Do NOT change the `VecDeque` queue or routing logic. Add a
  `/// why:` test asserting two topics still route independently AND that handler iteration for a
  topic is registration-order-deterministic. Revision-history entry.
- **CLAIM-A3 [Sonnet]** — `ports/events/cloud_events_envelope.rs`: the `extensions` field
  `HashMap<String,String>` → `indexmap::IndexMap<String,String>`. Keep the serde derives and the
  CloudEvents field semantics. Add/adjust a `/// why:` test that extensions serialize
  deterministically (insertion order). Revision-history entry.

## Phase B — §1.5 / §3.G lock centralization (ArcSwap RCU) [ORCH — all]

The delicate refactors; ORCH owns them because they must compile and carry the LESSON #16
16-OS-thread regression test that sub-agents cannot run.

- **CLAIM-B1 [ORCH]** — `graph/hex_graph.rs` `current()`:
  `OnceLock<Arc<HexGraph>>` → `std::sync::LazyLock<arc_swap::ArcSwap<HexGraph>>` (the canon's own
  exemplar shape, PRD-272 change-log 2026-06-26 "LazyLock<ArcSwap<IndexMap<…>>> wait-free RCU").
  - `current()` returns `Arc<HexGraph>` via `.load_full()` — **non-breaking** (same return type).
  - Add `HexGraph::install(graph: HexGraph)` (SOLE write site) that `store`s a new `Arc` — the live
    self-programming hot-swap that lets `hexser/refresh` update the graph without a process restart
    (Phase E wires MCP to it).
  - Add `HexGraph::rebuild_current()` that rebuilds from the registry and installs it.
  - Cite N_BOOK §1.5 + §23 + PRD-272 §1.5 in the `//!` preamble.
  - LESSON #16 test: 16 OS threads concurrently `current()`-read while one thread `install()`s;
    assert no torn reads / no panic and the final load reflects the last install.
- **CLAIM-B2 [ORCH]** — `container/container.rs`:
  `tokio::sync::RwLock<HashMap<String,ServiceEntry>>` → `arc_swap::ArcSwap<indexmap::IndexMap<String,ServiceEntry>>`.
  `register`/`register_async` become `rcu`-write (CAS-retry monotone-extend); `resolve`/`contains`/
  `service_count` become wait-free `.load()` reads. Keep the per-entry `tokio::sync::OnceCell`
  singleton cache (that is per-entry init, not a map lock). Preserve all public async signatures.
  16-thread LESSON #16 test: concurrent register+resolve, single-init assertion still holds.

## Phase C — §1.1 SNAPP_CONTRACT.md (one per crate) [Sonnet, parallel]

Each declares the crate's Snapp tier (BE = backend, FE = frontend, XS = cross/shared), its
boundary Port traits, its wire format, and its PRD-272 conformance status. Template in Appendix.

- **CLAIM-C1 [Sonnet]** — `hexser/SNAPP_CONTRACT.md`. Tier: **XS** (shared substrate — the Port
  framework itself). Boundary ports: `Repository`/`QueryRepository`, the `ports::events::*` traits,
  `ports::mcp_server::McpServer`. Wire: JSON (AI/MCP external), in-process traits internal. Note the
  ArcSwap graph as the self-programming substrate.
- **CLAIM-C2 [Sonnet]** — `hexser_macros/SNAPP_CONTRACT.md`. Tier: **XS** (compile-time only, no
  runtime boundary). No wire. Declares that it emits `::hexser::inventory::submit!` registrations.
- **CLAIM-C3 [Sonnet]** — `hexser_potions/SNAPP_CONTRACT.md`. Tier: **BE** examples/presets. Depends
  on hexser only; no cross-crate wire of its own.

## Phase D — deferred hardening [Sonnet, parallel]

- **CLAIM-D1 [Sonnet]** — `hexser/README.md` accuracy + restructure (IMPL_PLAN L48/H2 follow-up):
  finish the Part 5/6 "real-world" cookbook so every code block uses the real API
  (`Hexserror::{domain,port,adapter,validation,not_found,conflict}` + the `.with_next_step(s)`/
  `.with_suggestion(s)`/`.with_source` builders that now exist after PR-6/PR-8); replace remaining
  fictional codes with real `hexser::error::codes::*` constants; change hardcoded `= "0.4.7"`
  dependency snippets to `= "0.4"`; add a short "Canonical examples" section linking the compiled
  `examples/` + tutorials as the source of truth. Do NOT invent APIs — every symbol must exist in
  `hexser/src`. (Agent verifies by reading source, not compiling.)
- **CLAIM-D2 [Sonnet]** — `ai/method_extractor.rs` + `ai/agent_pack.rs` (L45/L46):
  (a) add a `/// why:` test in `method_extractor.rs` that asserts the hardcoded Repository method
  signatures still name-match the real `ports::repository::Repository`/`QueryRepository` methods
  (guard against silent drift into published AI context); (b) add `AgentPack::builder()` returning a
  small builder that lets a consumer supply project docs/guidelines instead of hexser's own
  house-style defaults, and label `default_guidelines()` output in-doc as hexser-specific. Keep the
  existing `from_graph_with_defaults` constructor. Revision-history + `/// why:` tests.

## Phase E — next phases (planned, NOT dispatched this round)

- **E1 — MCP live self-programming refresh:** wire `hexser/refresh` to `HexGraph::rebuild_current()`
  + `install()` so an AI agent rewriting the project sees the new graph **without a restart**
  (removes the `restart_required` result). This is the payoff of CLAIM-B1. Needs the ArcSwap swap to
  land first + a §1.5 wait-free-read integration test.
- **E2 — EventSubscriber `Send + Sync` (M27):** breaking (v0.5); enables the actor-grade
  multi-threaded transport bus (Kafka/AMQP) that the RefCell dev bus cannot host. Pair with an
  ArcSwap handler registry (the actor bus §1.5 surface).
- **E3 — postcard boundary crate:** when hexser gains an internal cross-crate message (e.g. a
  `hexser_wire` crate for BE↔FE graph deltas), it MUST be postcard-serialised with a round-trip
  test per §1.3. Not needed until that boundary is drawn.
- **E4 — `#![deny(missing_docs)]` + README-include doctests** (IMPL_PLAN testing workstream tail).

## §5 checklist (every Sonnet reports this on its changed files)

```
[ ] §1.2  independent .await pairs use tokio::join!/try_join! or document the data dep (N/A if sync).
[ ] §1.3  new cross-crate message types derive Serialize+Deserialize + postcard round-trip (N/A here).
[ ] §1.4  cross-crate calls go through a trait Port; no wide concrete-type pulls.
[ ] §1.5  new shared-read state uses ArcSwap; NO new RwLock/Mutex/OnceLock on read paths.
[ ] §3.F  no .unwrap()/.expect() in src/ (tests OK) — use ? .
[ ] §3.G  no new RwLock/Mutex/OnceLock in changed src/.
[ ] §3.H  no new HashMap/HashSet in changed src/ — IndexMap/IndexSet only.
```
A failed box is not automatically a blocker but MUST be justified in the completion report.

## Appendix — SNAPP_CONTRACT.md template

```markdown
# SNAPP_CONTRACT — <crate>

- **Snapp tier:** BE | FE | XS  (backend / frontend / cross-shared)
- **PRD-272 status:** conforming as of 2026-07-21 (cite exceptions)
- **Boundary ports (trait Ports exposed to other crates/Snapps):** <list traits + module paths>
- **Wire format at the boundary:** <JSON (external) | in-process trait | postcard (internal cross-Snapp)>
- **Shared-read hot state (ArcSwap surfaces):** <list, or "none">
- **Locks:** <ArcSwap-only | documented carve-outs | none>
- **Deterministic iteration:** IndexMap/IndexSet + BTreeMap (no HashMap/HashSet on iterated paths)
- **N_BOOK grounding:** §1.5 (wait-free RCU), §10 (Snapp boundary), §23 (Tier 2 SOLE write site)
```

## Landed status (2026-07-21)

Orchestrator-compiled on stable 1.88 — all suites green, clippy(lib+bins)/fmt clean, both
LESSON #16 16-thread stress tests pass.

- **CLAIM-B1 [ORCH] ✅** — graph `LazyLock<ArcSwap<HexGraph>>`, `current()`/`install()`/
  `rebuild_current()`, adjacency IndexMap; 16-thread read-during-install test.
- **CLAIM-B2 [ORCH] ✅** — container `ArcSwap<IndexMap>`, `insert_service` rcu SOLE write site,
  `load_entry` wait-free read; 16-thread register/resolve test.
- **CLAIM-A2/A3 [Sonnet] ✅** — event-bus handlers + CloudEvents extensions IndexMap (+ determinism
  tests). Read-verified deviation: envelope has no serde derive → test asserts IndexMap Serialize.
- **CLAIM-C1/C2/C3 [Sonnet] ✅** — SNAPP_CONTRACT.md for all three crates.
- **CLAIM-D2 [Sonnet] ✅** — method_extractor drift-guard test + `AgentPack::builder()` + labeled
  default guidelines as hexser-specific.
- **CLAIM-D1 [Sonnet] 🔄** — README cookbook accuracy + version snippets + canonical-examples
  pointer (docs-only; landing).

§3.G: zero production RwLock/Mutex/OnceLock remain in the changed core (ArcSwap + per-entry tokio
OnceCell only). §3.H: zero HashMap/HashSet in the changed core. Deps added: `arc-swap`, `indexmap`.

Phase E (E1 live MCP refresh via `install()`, E2 EventSubscriber Send+Sync, E3 postcard boundary,
E4 deny(missing_docs)+README doctests) remains planned, not dispatched.

## Revision history
- 2026-07-21T00:00:00Z @AI: Author hexser PRD-272 conformance plan (ArcSwap lock-centralization,
  IndexMap determinism, SNAPP_CONTRACTs) + carry the two deferred hardening items; structured as
  ORCH-owned + Sonnet-dispatchable CLAIMs.
- 2026-07-21T01:00:00Z @AI: CLAIM-B1/B2/A2/A3/C/D2 landed + verified; canon change-log updated with
  exemplar line-refs; D1 (README) landing.
