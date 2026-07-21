---
prd_id: HEXSER-PHASE-E
status: planned (not dispatched)
parent: PRDs/PRD_HEXSER_272_CONFORMANCE_260721.md
canon: PRD-272 (/Users/scott/@squillo/n-rust/nlang_x/PRDs/PRD_272_CROSS_REPO_CROSS_CRATE_COMMUNICATION_CANON.md)
scope: hexser (+ a future hexser_wire crate)
audience: ORCH + Sonnet sub-agents
created: 2026-07-21
---

# Phase E — hexser next phases (self-programming + actor-grade boundaries)

Builds on the landed PRD-272 conformance (ArcSwap graph + container, IndexMap, SNAPP_CONTRACTs).
Every crate boundary is a Snapp boundary; these phases turn the now-hot-swappable graph into a
live self-programming loop and make the event bus actor-grade. Same orchestration protocol as the
parent PRD: **sub-agents NEVER run git/cargo, NEVER branch/stash/checkout, fix-forward only; the
ORCHESTRATOR compiles + tests + commits at the end; every Sonnet runs the §5 checklist.**

## E1 — Live MCP self-programming refresh (payoff of CLAIM-B1) [ORCH-led]

Today `hexser/refresh` recompiles and returns `restart_required`; the process must restart for the
MCP server to see new components. With `HexGraph::install()` (landed), the in-process graph can be
hot-swapped instead.

- **CLAIM-E1.1** — `adapters/mcp_stdio.rs`: after a successful `run_cargo_build_bounded`, call
  `HexGraph::rebuild_current()` and rebuild the served `ProjectRegistry` from the new graph, then
  return a `RefreshResult` that reports `refreshed` (not `restart_required`). The graph read on the
  next `resources/read` reflects the rebuild with no restart. Note the link-time-inventory caveat
  in the doc (a recompiled *binary* is still required to pick up genuinely new `#[derive]`d types;
  the hot-swap covers the in-process registry rebuild + any programmatic `install`).
- **CLAIM-E1.2** — integration test (`tests/`): drive an `install()` of a modified graph and assert
  a subsequent `read_resource("hexser://.../context")` serializes the NEW graph (wait-free read
  reflects the SOLE-write install). §1.5 wait-free-read verification. LESSON #16 not required (no
  new RCU surface — reuses the graph ArcSwap).

## E2 — Actor-grade event bus (§1.5 + M27) [ORCH-led, breaking → v0.5]

The `RefCell` `InMemoryEventBus` is single-threaded/test-only; the `EventSubscriber` handler type
lacks `Send + Sync`, so no real transport (Kafka/AMQP/actor mailbox) can host it.

- **CLAIM-E2.1** — `ports/events/event_subscriber.rs`: add `+ Send + Sync + 'static` to the boxed
  handler type (breaking; v0.5). Consider `Arc<dyn Fn ...>` and passing `Arc<CloudEventsEnvelope<T>>`
  to cut per-subscriber clones.
- **CLAIM-E2.2** — new `adapters/actor_event_bus.rs`: a `Send + Sync` bus whose handler registry is
  `ArcSwap<IndexMap<String, Vec<Handler>>>` (§1.5/§3.G wait-free dispatch reads, rcu subscribe as
  SOLE write site). LESSON #16 16-thread publish-during-subscribe stress test. Keep the RefCell bus
  as the documented single-threaded dev adapter.
- N_BOOK §1.5 (wait-free RCU) + §22 (distributed execution, per-peer LSN if the bus federates).

## E3 — postcard boundary crate (only when the BE↔FE cut is drawn) [ORCH-led]

When hexser gains an internal cross-crate/cross-Snapp message (e.g. graph deltas streamed from a
backend `hexser` Snapp to a frontend visualizer Snapp), that message MUST live in a `hexser_wire`
crate, `#[derive(Serialize, Deserialize)]`, postcard-serialised, with a round-trip test (§1.3/§2).
Do NOT create until the boundary exists — premature wire types are speculative coupling. The
current AI/MCP JSON exports are an EXTERNAL wire and stay JSON.

- **CLAIM-E3.1 (deferred trigger)** — when the first BE↔FE delta message is needed: create
  `hexser_wire`, define the message + a trait `Port` in the upstream crate, add the postcard
  round-trip test, register it in the SNAPP_CONTRACTs.

## E4 — documentation + test-coverage tail (IMPL_PLAN workstream) [Sonnet, parallel]

- **CLAIM-E4.1** — flip `#![warn(missing_docs)]` → `#![deny(missing_docs)]` on the three crate roots
  once the remaining public-item doc gaps are closed (audit + fill).
- **CLAIM-E4.2** — wire the README code blocks into compiled doctests (`#[doc = include_str!(...)]`
  on a hidden docs module) so README drift becomes a build failure — the durable fix for the H2/L48
  class. Requires D1 (README accuracy) to have landed first.
- **CLAIM-E4.3** — backfill `/// why:` on the ~261 pre-existing `hexser/src` tests (IMPL_PLAN
  testing workstream) and add the CI lint asserting every `#[test]` has one.

## Sequencing

1. E1 first (small, high-value — completes the self-programming loop the USER emphasized).
2. E4.1/E4.3 in parallel (docs/tests, independent, Sonnet-friendly).
3. E2 batched into the v0.5 breaking release (with the other v0.5 items from IMPL_PLAN: the
   `Send+Sync` trait change, `delete_where` already landed, `to_json` already landed).
4. E3 only when the BE↔FE boundary is actually drawn.

## Revision history
- 2026-07-21T00:00:00Z @AI: Author Phase E next-phases plan (live self-programming refresh,
  actor-grade event bus, deferred postcard boundary, docs/test tail) as dispatchable CLAIMs.
