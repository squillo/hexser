---
task_id: hexser-core-hardening-260720
status: planning
author: @AI (audit) / Scott Wyatt
created: 2026-07-20
scope: hexser, hexser_macros, hexser_potions (published crates)
---

# Task: Core codebase hardening pass — performance, usability, reliability, maintainability

## Executive Summary

A multi-agent audit of the `hexser` workspace (v0.4.7, edition 2024) produced
**66 adversarially-verified findings** — 4 high, 34 medium, 28 low — across seven
dimensions. Every finding was re-checked by an independent skeptic against the actual
source before landing here; 3 candidate findings were refuted and dropped (listed at the
bottom).

The headline: **four defects break downstream users out of the box** — a default-feature
stack overflow, a non-compiling crates.io Quick Start, a derive macro that fails in any
consumer crate, and an MCP server that violates protocol on every real client session.
Underneath that, the published docs describe a substantial amount of API that does not
exist, and CI is almost certainly red today (93 clippy warnings + 10 rustdoc warnings
under `-D warnings`) yet passes because `cargo clippy` is shadowed by a stub locally and
the toolchain/feature matrix is mis-covered.

### Build / test ground truth (nightly 1.96.0, warm cache)

| Check | Result |
|---|---|
| `cargo check --workspace --all-features` | Compiles; warnings only |
| `cargo check` **without** `visualization` (incl. default) | 2 `unconditional_recursion` warnings → runtime stack overflow (see H1) |
| `cargo clippy --workspace --all-features` | **93 warnings** (`result_large_err` ×58, `disallowed_macros` ×8, `new_without_default` ×3, `module_inception` ×2, dead_code ×11, …) |
| `cargo test --workspace --all-features` | 306 pass / **3 fail (flaky)** / 4 ignored — env-var race, passes with `--test-threads=1` |
| `cargo doc --all-features` | Builds; **10 rustdoc warnings** (7 unclosed-HTML, 3 empty-code-block) |
| CI (`pr-test.yml`) | PR-only; installs **stable** but repo pins **nightly** (override wins); `--all-targets` **skips all 64 doctests**; never builds `--no-default-features` or single features |

> ⚠️ Local `cargo clippy` is shadowed by a stub that prints `cargo clippy: ok` and does
> nothing. Real lint results require `rustup run nightly cargo clippy`. This is why the
> 93 warnings have gone unnoticed. Verify/repair the local shim as step 0.

## Plan

Phases are ordered by user impact, not effort. Within a phase, items are roughly
dependency-ordered. Effort: S = <1h, M = ~1 day, L = multi-day/design.

### Phase 0 — Make the signal trustworthy (do first; unblocks everything)
- [ ] 0.1 (S) Fix/replace the local `cargo clippy` shim so lints actually run; re-baseline.
- [ ] 0.2 (S) Add `resolver = "3"` to the root `[workspace]` (silences the resolver-1 warning; avoids silent feature-unification differences). — `Cargo.toml`
- [ ] 0.3 (M) Drive the 93 clippy warnings to zero (or `#[allow]` with justification):
  - `result_large_err` ×58 — `Hexserror` Err payload ≥176 B. Decide the semver-safe fix: box the large variant(s) inside `Hexserror`, or return `Box<Hexserror>`. This touches the public error surface — settle it early (see R5) since many other fixes return `Hexserror`.
  - `disallowed_macros` ×8 — `println!`/`eprintln!` in `graph/hex_graph.rs`, `showcase/describable.rs`, and the two AI bins. Route through a log macro or `#[allow(disallowed_macros)]` on the CLI/diagnostic sites per `clippy.toml`'s own guidance.
  - Remaining: `new_without_default` ×3 (exporters), `module_inception` ×2, `single_char_add_str` ×2, `into_iter_on_ref` ×2, `clone_on_copy`, `redundant_closure`, `needless_as_bytes`, `question_mark`, `type_complexity`, + 11 `dead_code` in `hexser_macros` (see M6).
- [ ] 0.4 (S) Fix the 10 rustdoc warnings (backtick/escape `<T>`-style generics in doc comments; remove 3 empty code blocks). — `error/env_control.rs`, `error/layer_error.rs`, `ports/events/mod.rs`, `showcase/describable.rs`, `ports/events/event_router.rs`
- [ ] 0.5 (S) Serialize the env-mutating tests (shared `Mutex` or `serial_test` dev-dep) so `cargo test` is deterministic in parallel. — `error/env_control.rs`, `tests/env_controlled_serde_test.rs` (R8, R14)

### Phase 1 — Downstream-breaking correctness (highest external impact)
- [ ] 1.1 (S) **H1** Fix `Visualizable for HexGraph` infinite recursion when `visualization` is off (the default). Gate the impl behind `#[cfg(feature = "visualization")]` or return a "enable the visualization feature" `Err`. The crate's own `visualization_tutorial` example aborts (exit 134) as documented. — `showcase/visualizable.rs:21`
- [ ] 1.2 (S) **H3 / M2** `HexDirective` derive emits bare `inventory::submit!`; change to `hexser::inventory::submit!` (ideally `::hexser::…`) to match the other derives. Breaks every consumer that doesn't depend on `inventory` directly. — `hexser_macros/src/derive/directive.rs:38`
- [ ] 1.3 (S) **M13** `HexQuery` derive implements `Registrable` but never submits to inventory → query components silently absent from the graph. Add the `inventory::submit!` block. — `hexser_macros/src/derive/query.rs:15`
- [ ] 1.4 (M) **M12** Derives fail on generic types with inscrutable `E0425/E0310` errors (inventory submit references type params at item scope). Detect generics and either skip registration (documented) or emit a spanned `syn::Error`. — `hexser_macros/src/derive/{hex_domain,hex_port,hex_adapter,directive}.rs`
- [ ] 1.5 (M) **H4 / R1** MCP stdio server can't handle JSON-RPC notifications: `JsonRpcRequest.id` is required, so id-less `notifications/initialized` (mandatory in the MCP handshake) fails to deserialize and the server replies `-32700` with `id:null` — a protocol violation on every real session. Make `id: Option<Value>` (or a `RequestId` enum), process notifications with no response, use `-32600` for structurally-invalid-but-parseable JSON. — `domain/mcp/json_rpc.rs:21`, `adapters/mcp_stdio.rs:142`
- [ ] 1.6 (M) **H2** Rewrite `hexser/README.md` Quick Start against the real API: `#[derive(Entity)]`→`HexEntity`, remove `#[derive(HexPort)]` on a *trait* (illegal), and delete/implement the fictional `HexQueryHandler`/`HexDirectiveHandler`/`HexConfig` derives. Wire README code into doctests (`#[doc = include_str!("../README.md")]`) so drift becomes a build failure. — `hexser/README.md:74`
- [ ] 1.7 (M) **M5 / M11** The three error proc-macros (`hex_domain_error!`/`hex_port_error!`/`hex_adapter_error!`) are unusable: not re-exported from `hexser`, the documented comma syntax can't parse (parsed as one `syn::Expr`), and the generated `.with_location(..)` method doesn't exist. **Decide: fix-and-ship or delete.** If fixing: parse `Punctuated<Expr, Comma>`, generate a real call, re-export from root+prelude, add tests. — `hexser_macros/src/error/hex_error_macro.rs`, `hexser/src/lib.rs`, `error/mod.rs`

### Phase 2 — Reliability & concurrency hardening
- [ ] 2.1 (M) **R7 / M25 / R2 / M32** DI container holds the services read-lock and singleton write-lock across arbitrary user provider code (`provide()` / `provide_async().await`). Enables real deadlocks (nested resolution behind a queued `register`; self-resolving singleton; `register`-from-provider) and executor stalls. Clone the `Arc` factory + copy `Scope`, drop the map guard before invoking, use per-entry `tokio::sync::OnceCell`. Document that providers must not `register`. — `container/container.rs:103,111,211,219`
- [ ] 2.2 (S) **M26** Cached singleton resolution takes an exclusive `write()` on every call (the hot path serializes). Add a `read()` fast-path with double-checked init, or `OnceCell`. — `container/container.rs:111`
- [ ] 2.3 (M) **M28 / M33** `InMemoryEventBus::subscribe` sets a single `self.topic`; `publish` only fires `handlers.get(&self.topic)` → only the last-subscribed topic ever delivers, earlier handlers silently dead, contradicting the "routed by topic" doc. Route per-publish (resolve topic from the envelope / store `Vec` of handlers per topic); drop the `self.topic` mutation. — `adapters/in_memory_event_bus.rs:186,219`
- [ ] 2.4 (S) **M27** `EventSubscriber` handler is `Box<dyn Fn(..) -> HexResult<()>>` with no `Send`/`Sync` — no real transport adapter (Kafka/AMQP/HTTP, all named in the docs) can move it to a task. Add `+ Send + Sync + 'static` (and consider `&`/`Arc<Envelope>` to cut per-subscriber clones) in the next breaking release, before adoption makes it costlier. — `ports/events/event_subscriber.rs:116`
- [ ] 2.5 (M) **M30 / M19** `hexser/refresh` runs `cargo build` synchronously with no timeout on the single-threaded request loop → server freezes indefinitely; `.output()` buffers unbounded stderr. Also the logic is duplicated inline (the `refresh_project` impl is dead: `&self` vs `&mut self`) and the two copies already diverge. Run with a timeout + capped stderr; make `refresh_project(&self)` and delegate. — `adapters/mcp_stdio.rs:292,498`
- [ ] 2.6 (M) **M6 / R31** `Hexserror::with_next_step`/`with_suggestion`/`with_field`/`with_existing_id` silently drop input on `Validation`/`NotFound`/`Conflict` variants (no fields to hold it). The crate's own `container.rs:75,180` and the README's `not_found(..).with_next_step(..)` hit this. Add storage to those variant structs, or make the mismatched builders unrepresentable; fix the README either way. — `error/hex_error.rs:106`
- [ ] 2.7 (S) **R58** `validate_time_format` accepts garbage (`"2025-13-99T99:99:99Z"`) despite claiming RFC3339 validation; used as a publish gate. Parse properly (chrono, already a dep under `ai`) or re-document as a heuristic. — `ports/events/cloud_events_envelope.rs:354`
- [ ] 2.8 (S) **R59** `GraphMetadata::current_timestamp` `.unwrap()`s `duration_since(UNIX_EPOCH)` on a near-universal path; use `.map(..).unwrap_or(0)`. Contradicts the project's own no-unwrap guideline. — `graph/metadata.rs:55`
- [ ] 2.9 (S) **R57** MCP `run()` reads unbounded stdin lines (OOM on a giant/never-terminated frame) and any single read error kills the process. Cap with `BufRead::take` + `read_until`, reject oversized frames with `-32600`, tolerate transient errors. — `adapters/mcp_stdio.rs:127`
- [ ] 2.10 (S) **P54 / P55** `InMemoryEventBus` queue grows unbounded in push-mode and `poll` uses `Vec::remove(0)` (O(n²) drain). Use `VecDeque::pop_front`; only enqueue when unconsumed or document the drain requirement / add bounded capacity. — `adapters/in_memory_event_bus.rs:182,230`

### Phase 3 — Published API & documentation contract
- [ ] 3.1 (S) **M7** Export the v0.4 flagship read API — `QueryRepository`, `FindOptions`, `Sort`, `Direction` — from `ports/mod.rs`, the crate root, and the prelude (currently only reachable via `hexser::ports::repository::…`; the README's own quick start doesn't compile). — `ports/mod.rs:27`, `lib.rs`
- [ ] 3.2 (S) **M10** Rewrite the crate-level rustdoc header (docs.rs front page): it lists nonexistent `graph`/`analysis` features, claims a "zero dependencies" default, and still says "Phase 1 … future phases will add …" on a 0.4.7 crate that ships all of it. — `lib.rs:45`
- [ ] 3.3 (S) **M4** `async` feature enables tokio/async-trait but gates **no** code; README documents fictional `AsyncRepository`/`AsyncDirective`/`AsyncQuery`. Either implement the traits or rewrite the docs to say it's a dependency toggle (and consider deprecating). — `hexser/README.md:249`, `Cargo.toml:36`
- [ ] 3.4 (M) **M8 / M9** README error/DI cookbook references non-existent items: `codes::adapter::{IO_FAILURE,DB_WRITE_FAILURE,DB_READ_FAILURE,CONNECTION_FAILURE}`, `Hexserror::infrastructure/application`, `DynContainer`, builder-style `StaticContainer::new().with_service(..)`. Audit README Parts 5–6 against `error/codes.rs`, `hex_error.rs`, `static_di.rs`, `container/mod.rs`. — `hexser/README.md:190,1129`
- [ ] 3.5 (S) **L38** Root workspace README quick start uses the removed `Entity` trait and non-prelude `QueryRepository`; update to `HexEntity` + prelude (after 3.1). — `README.md:56`
- [ ] 3.6 (S) **L40** Declare an explicit `serde = ["dep:serde"]` feature and document it + `HEXSER_INCLUDE_SOURCE_LOCATION` in the feature lists (today serialization is gated on an *implicit* optional-dep feature nobody can discover). Note: converting all deps to `dep:` is semver-sensitive — do `serde` deliberately. — `hexser/README.md`, `lib.rs`, `Cargo.toml:16`
- [ ] 3.7 (S) **L39** `QueryRepository::delete_where` default returns `Ok(0)` (silent no-op indistinguishable from "0 matched"). Make it required, or default to an `Err(port(..))`. Good candidate for the v0.5 breaking train. — `ports/repository.rs:95`
- [ ] 3.8 (M) **L48** `hexser/README.md` is 2,458 lines with internal drift (line 228 advertises pre-0.4.7 flat MCP URIs the server no longer lists). Fix line 228; consider making the mdbook canonical and shrinking the README to an overview; use `hexser = "0.4"` in snippets to stop editing 15 version strings each release. — `hexser/README.md:228`

### Phase 4 — Maintainability & dead code
- [ ] 4.1 (M) **M18 / M23** 820 lines of orphaned, never-compiled source ship in the crates.io tarball (`graph/{query,analysis,validation,intent}.rs` — no `mod` decl, won't compile, call a non-existent `HexGraph::query()`), while `lib.rs`/README advertise "intent inference" and "architectural validation" as done. **Decide per file: revive (wire in, fix API, test) or delete**, then correct the docs. Note `validation.rs::is_valid_layer_dependency` is the real impl needed by 4.3. — `graph/mod.rs:12`
- [ ] 4.2 (M) **M16 / M15** Macro metadata is hardcoded: every `HexPort` gets `Role::Repository`, every `HexDomain` gets `Role::Entity`; the declared `#[hex(...)]` attribute is parsed nowhere; `HexEntity` silently defaults `type Id = String` when no `id` field exists (incl. enums). Parse the attribute and derive real roles; error on missing `id`. — `hexser_macros/src/derive/{hex_port,entity}.rs`
- [ ] 4.3 (M) **M20** `ContextBuilder::validate_relationship` is a stub returning `true`, so every AI-context edge reports `is_valid: true` — the opposite of the crate's pitch, shipped over MCP and `hex-ai-export`. Wire in the real layer-rule table (from 4.1's `validation.rs`) and test that an invalid edge is flagged. — `ai/context_builder.rs:217`
- [ ] 4.4 (M) **M17 / L43 / L44 / L42 / L41** `hexser_macros` hygiene: ~40% dead code (stub codegen, unused validators, 4 unused generators — the 11 `dead_code` warnings); `HexRepository` derive expands to a comment (no-op); `Registrable` codegen duplicated across 5 derives (root cause of 1.2/1.3); generated paths use `hexser::`/`std::` not `::hexser::`/`::std::` (hygiene); `darling` declared but unused (drop it). Consolidate into one shared codegen helper, delete dead code, fully-qualify paths. — `hexser_macros/src/**`
- [ ] 4.5 (S) **L53** `HexGraph::get_dependencies`/`get_dependents` are trapped inside `#[cfg(test)] mod tests` (with a nested duplicate `mod tests`) — the only neighbor-traversal helpers never ship. Move the `impl` block out to `hex_graph.rs`. — `showcase/inspectable.rs:106`
- [ ] 4.6 (S) **L47 / L49** MCP/AI code mints ad-hoc `E_MCP_*`/`E_AI_*` string literals (one retyped in two places) that bypass `error/codes.rs`; `to_json` returns `Result<String,String>` re-wrapped at 4+ call sites. Add the codes as documented consts; return `HexResult<String>`. — `adapters/mcp_stdio.rs:132`, `ai/ai_context.rs:381`, `ai/agent_pack.rs:114`
- [ ] 4.7 (M) **L45 / L46** `method_extractor.rs` hand-transcribes `Repository` signatures (silent drift into AI output on the next change; `query_trait_methods` returns empty); `AgentPack` defaults bake `CARGO_MANIFEST_DIR` machine paths and hexser's *own* house-style guidelines into downstream users' packs. Add a drift test now (long-term: rustdoc-JSON); add an `AgentPack::builder()` and label default guidelines as hexser-specific. — `ai/method_extractor.rs:25`, `ai/agent_pack.rs:83`

### Phase 5 — Performance
- [ ] 5.1 (M) **P24** `GraphInner.nodes` is a `HashMap` → nondeterministic iteration → DOT/Mermaid/JSON exports and AI context differ run-to-run (spurious diffs, broken content-hash caching). Switch to `BTreeMap<NodeId, HexNode>` (add `Ord` to the u64 `NodeId`) or sort at export boundaries. — `graph/hex_graph.rs:41`
- [ ] 5.2 (S) **P50** `HexGraph::current()` rebuilds the whole graph from `inventory` on every call (link-time-fixed data) and returns a redundant `Arc<HexGraph>` (already `Arc` inside). Cache in a `OnceLock`; keep `Arc<Self>` for API compat. — `graph/hex_graph.rs:49`
- [ ] 5.3 (M) **P51** No adjacency index: `edges_from`/`edges_to` are O(E) scan+alloc per call, making `ContextBuilder::build_components` O(V·E). Precompute `HashMap<NodeId, Vec<usize>>` in `GraphBuilder::build`; keep the `Vec<&HexEdge>` return signature (build from the index) to stay semver-safe. — `graph/hex_graph.rs:219`
- [ ] 5.4 (S) **P52** `GraphBuilder::build` discards `HashMap::insert`'s return → a `NodeId` (weak hand-rolled djb2) collision silently drops a node. Flag `Some(prev)` with a different `type_name` (or a duplicate-id check in `build_validated` via the existing `E_HEX_GRAPH` family). — `graph/builder.rs:96`
- [ ] 5.5 (S) **P29** Trim tokio: the workspace pins `["sync","macros","rt-multi-thread","time"]` but library code uses only `tokio::sync::RwLock`; the runtime features are test-only (and `rt-multi-thread` won't build on wasm32, contradicting the "WASM-friendly" comment). Move `["macros","rt-multi-thread","time"]` to hexser's `[dev-dependencies]` (workspace inheritance allows additive features). — `Cargo.toml:19`

### Phase 6 — Tooling, CI & release hygiene
- [ ] 6.1 (S) **M34** Fix the broken `repository` URL (`github.com/squillo/hexser/hex`) in all three crates; add `homepage`/`documentation`/`rust-version` (MSRV). — `hexser/Cargo.toml:10`, `hexser_macros`, `hexser_potions`
- [ ] 6.2 (S) **M22 / M35** Resolve the toolchain split-brain: `rust-toolchain.toml` pins floating `nightly`, CI runs `@stable`, and `.rustfmt.toml` uses the nightly-only `imports_granularity` (silently ignored by stable CI). Edition 2024 + no unstable features → pin **stable** (e.g. `1.88`) or a **dated** nightly, and make CI read the toolchain file. Gate/remove nightly-only rustfmt options. — `rust-toolchain.toml:2`, `.github/workflows/pr-test.yml`
- [ ] 6.3 (M) **M36 / L61** Rework CI (`pr-test.yml`): add a `push` trigger on `main` (currently PR-only — main is never tested); add `--no-default-features` and per-feature builds (would have caught H1); run doctests (`--all-targets` skips all 64); once Phase 0 lands, `-D warnings` will actually gate. Consider `cargo-hack` for the feature matrix.
- [ ] 6.4 (S) **M37 / M21** `examples/realworld_api` is an isolated `[workspace]` no CI job compiles — the library's most realistic consumer breaks silently. Add a CI job (`working-directory: hexser/examples/realworld_api`, `cargo check`/`test`) or fold it in with `publish = false`. (It currently fails to build — axum lock drift.) Regenerate + diff `architecture_diagram.mmd` via the existing script. — `hexser/examples/realworld_api/Cargo.toml:1`
- [ ] 6.5 (M) **M14** `hexser_macros` has zero expansion tests (no `tests/`, the one unit test is `#[ignore]`d). Add `trybuild` pass/fail cases — especially a downstream-style crate that does *not* depend on `inventory` (locks in 1.2), a generic-type case (1.4), and the error macros (1.7).
- [ ] 6.6 (S) **L60** (covered by 0.2) `resolver = "3"`.
- [ ] 6.7 (S) **L62 / L64 / L65** Release hygiene: `hexser/CHANGELOG.md` is 4 minor versions stale; `PUBLISHING.md` has stale versions + nonexistent crate paths; a stale git-tracked `hexser/Cargo.lock` pins 0.3.0 (a lib shouldn't track a member-level lock). Update/remove.
- [ ] 6.8 (M) **L63** Add supply-chain/dependency automation: `cargo-deny` (or `cargo audit`) in CI, a `dependabot.yml`, and consider `cargo-release` + a `CHANGELOG` discipline. Add a small benchmark harness (`benches/`) if 5.x perf work needs regression guards.

## Current Step

- **Action:** Plan authored and awaiting prioritization/approval. No source changed.
- **Details:** All 66 findings verified. Recommend executing **Phase 0 → Phase 1** first
  (trustworthy signal, then the four downstream-breaking defects) as an initial PR, since
  they are highest-impact and mostly small. The one cross-cutting decision to make before
  writing much code is **R5 / `result_large_err`** (how to shrink the `Hexserror` Err
  payload) because most fixes thread `Hexserror` through their return types — settling the
  representation once avoids churn.

## Open Decisions (need a human call)

1. **`Hexserror` size** — box the large variant(s) vs. `Box<Hexserror>` return alias. Semver-visible; affects 58 sites.
2. **Error proc-macros & orphaned graph modules** — fix-and-ship or delete? (1.7, 4.1). Both are advertised in docs, so "delete" also means "walk back the marketing."
3. **`async` feature** — implement the async port traits or downgrade to a documented dependency toggle? (3.3)
4. **Version target** — batch the breaking items (2.4 `Send+Sync`, 3.7 `delete_where`, 4.6 `to_json`, R5) into a single **v0.5**, or dribble them out? Pre-1.0 minors are conventionally breaking.
5. **Toolchain** — commit to stable (preferred; nothing needs nightly) or a pinned dated nightly? (6.2)

## Blockers

- None. Execution is gated only on the Open Decisions above; Phase 0 can start immediately.

## Refuted findings (checked, intentionally NOT actioned)

- *MCP `read_resource` rebuilds AIContext per request* — real but negligible (serial local
  stdio tool, ms-scale, and caching would freeze live doc content).
- *Query surface returns owned `Vec`s* — `nodes()` already returns `impl Iterator`; changing
  the rest is a semver break for unmeasurable gain on tiny graphs.
- *`InMemoryEventBus::publish` clones twice* — both clones are forced by the (test-focused)
  port signatures; the actionable part is the port design, already tracked as 2.4.

---

_Prior completed work (`TASK_PLAN.md`): `mcp-concurrent-access-support` — MCP concurrent
access during active development. Left intact; this document is a separate initiative._
