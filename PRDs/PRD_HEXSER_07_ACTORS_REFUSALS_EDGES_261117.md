---
prd_id: HEXSER-0.7
status: BUILT and gated green on the branch (§11 gate record). APPROVED by Scott's cards (Q1 plan and landing; Q2 node keys). Q3 was answered with an audit order; the audit is §12. Landing per D12; the publish has its own card.
card: HEXSER-0.7 (stc work, workspace proj-88d79f7d8225b85a; posts 241850, 242039, 242296)
checkout: /Users/scott/@squillo/n-rust/nlang_hex, branch hardening/pr0-signal-260720 at 3c2f628 (Scott's card, 2026-11-17)
scope: hexser, hexser_macros, hexser_potions, wasm-e2e/, scripts/wasm-e2e.sh, plugins/hexser
canon: SQUILLO_BOOK (/Users/scott/@squillo/squillo_os/squillo_os/Squillo Spec/1.0.0/SQUILLO_BOOK), N_BOOK (/Users/scott/@squillo/n/N Language Spec/1.0.0/N_BOOK)
---

# PRD: hexser 0.7. One door for a single writer, closed vocabularies, edges the compiler proves

## 0. The request, and Scott's words

Scott, verbatim, his spelling kept:
- On the Deck lane's card (PRD-886 F26, :743): "fix foward and get hexser .7 landed NOW". Then: "New hexser lane".
- On this lane's checkout card: "In it's branch!!! FIX FORAD AND GET THIS ALL FIXED AND MERGED IN!"
- **Q1, the plan and its landing:** "Approve; land and merge on green (Recommended)".
- **Q2, node keys:** "Report it, keys unchanged (Recommended)".
- **Q3, the legacy doors, answered with an order and not an option:** "Audit our work and plan for completeness and correctness to the PRDs, Issue, and Canon. We are focusing on being DRY and maximizing Security, Reliability, Maintainability, and SPEED. We are always preparing for SOC II TYPE 2, HIPA, ISO 27001, EDUCAUSE / HECVAT, etc verifications (and putting evidence in our system map). This is a good opportunity to use `stc simplify` to find places we can be reductive instead of additive.We want to make sure we don't lose value or the mission of the project! We are trying to provide the ultimate data contracts for Rust apps taht scale to huge sizes using our ehxagonala architecture!"

The Deck lane's evidence is PRD-886 F17 (:734): "Wait for hexser 0.7". hexser 0.6.1's handler takes `&self` and returns `HexResult<()>`. Deck's actor is one `&mut` writer that returns its event, and Deck's refusals are closed typed enums. "No lock is added to fit it." F26 turns that wait into this work.

## 1. Summary

After the audit (§12), 0.7 is:

1. **`hexser::application::Actor`**: the one door a single writer implements. It takes `&mut self`, one command, and an associated per-call context, and returns an associated outcome. No lock, no forced `Result`, no `Hexserror`. `#[derive(HexActor)]` registers its node.
2. **`hexser::domain::Vocabulary` and `#[derive(HexVocabulary)]`**: any closed enum that crosses a boundary (a command, an event, a refusal) gets stable codes in declaration order, `code()`, and a total `from_code` for unit enums. This is the Rust half of PRD-886 T68.
3. **`hexser::error::Refusal` and `#[derive(HexRefusal)]`**: a vocabulary that can carry static guidance. `Hexserror::refused` builds the existing `Domain` variant, and `Hexserror::code()` reads the code back. No `Hexserror` variant is added.
4. **A port node that cannot outlive its trait.** The existing `#[derive(HexPort)]` gets one key, `#[hex(port_trait = path::Trait)]`. No new macro kind.
5. **Edges on the derives.** `#[hex(implements(PortNode))]` on `HexAdapter` gives an `Implements` edge, but only if the adapter implements that port's trait. `#[hex(depends_on(..))]` on every registration derive gives the template's `Depends` edge. One reading function per relationship. Templates get the same keys.
6. **The graph says what is wrong as typed values.** `HexGraph::findings()` returns a closed `GraphFinding` for each of these:
   - two registrations sharing a short name (F-A, keys unchanged per Q2);
   - an adapter with no proved port;
   - an edge to a node that is not registered.
7. **Retirement, by deprecation (SB 15/203).** `DirectiveHandler`, `UseCase` and `InputPort` route to `Actor`. `ports::Query` routes to `QueryHandler`. Removal is a later version's card.
8. **No `unsafe` in hexser.**
   - The 28 test-only `unsafe` env writes go: 13 in src/error/env_control.rs and 15 in tests/env_controlled_serde_test.rs.
   - `#![forbid(unsafe_code)]` goes on all three crates.
   - The source-location switch is read once per process.
9. **Also in 0.7:**
   - wasm32 executed for every new door, macros only;
   - a 0.6 to 0.7 migration guide and updated plugin skills;
   - staged compliance evidence rows for the system map (§13);
   - 0.6.0, 0.6.1 and 0.7.0 merged to main.

## 2. Canon, read first by this lane

| Rule | Cite | What it decides here |
|---|---|---|
| A port shaped against one implementation is a rename. Enumerate every implementation and take the type they share. | SB 15/183 :24-31 | §4 is that enumeration, and D1 is its result |
| Two incompatibly shaped implementations force an associated context and an associated output. Refuse `Box<dyn Any>`. No lock in the port. | SB 15/183 :55-68 | D1 has `type Context<'a>` and `type Outcome`. No erasure, no lock |
| An adapter belongs where the port's other adapters live | SB 15/191 :30 | hexser ships traits and derives, never an editor's adapter |
| A decision is a value with one home. An unknown arm is a value, never a guess. No `impl Default`. | SB 15/146 :316-326, :380-411 | D2's `from_code` returns `Option`. D6 findings are values, not strings |
| The one writer is the one gate | SB 15/209 :18-30 | D1 takes `&mut self` |
| A registry of declared intent is not evidence of the store | SB 15/380 :64-73 | D5: `Implements` only where the impl compiles. D6 flags an adapter whose port is only declared |
| Assert by name. Ship a must-fail arm. A graph reading is scoped to one binary. | SB 15/413 :96-104 | Every graph row in §8 names its nodes and carries a must-fail arm |
| No lock across `.await`. An actor owns its state. The wait-free read. | SB 15/10 :20, :28, :30 | D1 adds no lock and no `Sync` bound. Reads stay the lane's (D8) |
| Two tools for one job: route or retire. Unreach before you delete. | SB 15/203 :21-31 | D4 extends `HexPort` and adds no second port macro. D7 deprecates four duplicate doors |
| Commands are proposals, events are facts, the actor decides. Commit before the next command (`&mut`). | N_BOOK 3/6/6 :19-22, :46-52, :169, :296 | D1 consumes the command by value and returns what the actor decided |
| One serialized write site plus wait-free reads | N_BOOK 3/6/6 §6.6.3 :838-844 | No lock in hexser's door |
| Commands never determine events | N_BOOK 3/24/1 :23-25 | D1 does not force one event per command (Tables appends 0..N) |
| `<ed>_core`: Directives and typed errors, no gpui, a permanent wasm32 cell. `editor_host_ports` takes hexser `macros` only. | PRD-847 :170, :172, :798 | D10 executes wasm32 on macros only. D2 and D3 make the errors typed |
| Edges, port nodes tied to traits, and closed refusals | PRD-886 F12 :729, F17 :734, F26 :743; §8.3.7 B H2, H3, H7; T67, T68, T69 | D4 and D5 meet T67. D2 meets T68's Rust half. D3 meets T69 |

## 3. hexser 0.6.1, measured at 3c2f628

- **Doors for a writer, all `&self -> HexResult`.**
  - `DirectiveHandler::handle(&self, D) -> HexResult<()>` (hexser/src/application/directive_handler.rs:54).
  - `UseCase::execute` (ports/use_case.rs:38) and `InputPort::execute` (ports/input_port.rs:38).
- **Read doors.** There are two for one job, with the same shape: `QueryHandler<Q, R>::handle(&self, Q) -> HexResult<R>` (application/query_handler.rs:49) and `ports::Query<P, R>::query(&self, P) -> HexResult<R>` (ports/query.rs:39).
- **`Hexserror`** has six boxed variants (error/hex_error.rs:47-60). Nothing in hexser is `#[non_exhaustive]` (0 uses).
  - Each payload carries code, message, next_steps, suggestions, location, more_info_url and a boxed source (error/layer_error.rs:33-55).
  - There is no `Hexserror::code()`.
- **The graph.**
  - The derives emit `dependencies() = []` (hexser_macros/src/common/codegen.rs:107-109).
  - `depends_on` exists only on the templates (templates/mod.rs:164-187).
  - `Relationship::Implements` has no producer outside tests.
  - A port node is a marker struct, and nothing ties it to its trait.
  - `ComponentEntry` has all-`pub` fields and is not `#[non_exhaustive]` (registry/component_entry.rs:9-12), so a new field there would break semver. Nothing in the family builds one by literal (rg, 0 sites).
- **F-A: two key spellings.**
  - A derive keys its node by the full path (codegen.rs:102), and a template by the short name (templates/mod.rs:168-170).
  - `GraphBuilder::build` records a collision only when the type names differ (graph/builder.rs:104-113). Two template nodes with one short name therefore merge in silence.
  - The record it does make is a `String` warning (builder.rs:127-130; metadata.rs:70-82).
  - The tunnel census (ledger 241901) found 0 live pairs.
- **F-B: main is behind.** 0.6.0 and 0.6.1 are only on `hardening/pr0-signal-260720`, 5 commits ahead of `origin/main`.
- **F-C: `#[derive(HexDirective)]` writes `validate()` as `Ok(())`** (hexser_macros/src/derive/directive.rs:32-34).
- **F-D: `unsafe` in hexser, measured by this audit.**
  - 13 `unsafe { std::env::set_var / remove_var }` blocks in the unit tests of src/error/env_control.rs:70-170.
  - 15 in tests/env_controlled_serde_test.rs.
  - All of them are test-only. The crate cannot carry `#![forbid(unsafe_code)]` while they stand.
  - The agent pack tells agents "no `unsafe` code in this crate" (src/ai/agent_pack.rs:38, :139). The tests contradict it.
  - `should_include_source_location` reads the process environment on every serialized error (env_control.rs:29-33).
- **wasm.** `scripts/wasm-e2e.sh` builds both wasm targets with `--no-default-features --features "macros,$f"` × 8 sets (:43-50) and executes a probe under wasmtime. The probe's export is `#[unsafe(no_mangle)]` (wasm-e2e/src/lib.rs:185), the one place a wasm host can call in.
- **Tests and gates.**
  - 280 `#[test]` in hexser/src.
  - A trybuild harness (tests/trybuild_ui.rs:32-34).
  - deny.toml at the root.
  - cargo-audit and cargo-deny are installed on this box.

## 4. Every implementation that exists (the SB 15/183 enumeration)

Measured 2026-11-17, read-only, from working trees, many of them uncommitted. The lanes' own answers: Taxa 241877, tunnel 241901, Recon 241911.

| Writer | Receiver, input, output | Refusal | Event or fact | Read path | hexser pin; wasm32 |
|---|---|---|---|---|---|
| Deck `DeckActor` | `handle(&mut self, DeckChange) -> Result<DeckEvent, ChangeRefusal>` (deck/interpreter/deck_actor/src/lib.rs:216) | closed, no Display (lib.rs:75-80; deck_contracts/src/lib.rs:141-150) | returned AND emitted through `DeckEventSink::emit(&mut self, &DeckEvent)` (lib.rs:143-145, :210) | `ArcSwap<Deck>` (lib.rs:158) | `"0.6.1"` default (interpreter/Cargo.toml:33); real `cargo check` cell (deck_actor/tests/wasm32.rs:13-36) |
| Deck `EditorActor` | `handle(&mut self, Ask) -> Arc<EditorView>`, infallible (deck_actor/src/editor.rs:173) | a closed `Notice`. "never a sentence: the words for it belong to the host" (editor.rs:15-16, :49-58) | inner event discarded (:206) | `ArcSwap<EditorView>` (:72) | as above |
| Tables `Actor` | `handle(&mut self, Command)`, returns `()` (table-interpreter/table_editor_host/src/actor.rs:205-209) | closed, `Copy` (table_editor_core/src/session.rs:640-659; table_document/src/document.rs:156-178); words `const fn -> &'static str` (words.rs:32-88) | 0..N `Fact { seq, outcome }` via `EventSinkPort::append(&mut self, &Fact)` (ports.rs:140-156); refusals are facts | `RwLock<Arc<View>>` until arc-swap is admitted (actor.rs:51-54, :100) | `"0.6.1"` default (Cargo.toml:52); manifest scan only (table_interpreter_gates/tests/core_is_pure.rs:5-10) |
| Canvas (Nimble) | `select(&mut self, &str) -> Result<(), OpenRefusal>` (canvas-editor/crates/canvas_editor_core/src/lib.rs:170) | `NimbleRefusal`: `Copy`, 67 members, a hand-written `ALL`/`code`/`from_code`/`next_step`/`suggestion` (nimble-gpui/crates/nimble_gpui_runtime/src/refusal.rs:503, :579-621); into `Hexserror` (:1266-1270); read back by string parse with a `_ =>` arm (canvas_editor_core/src/lib.rs:82-89) | none | by ownership | `"0.6"` macros (canvas-editor/Cargo.toml:62); no cell |
| Flo | `apply(&mut StepGraphEditorState, Intent, Context<'_>) -> Outcome` (flo/crates/flo_editor_core/src/session.rs:668-672); `step(&mut State, &HostView, &Ask) -> Answer` (flo_editor_host/src/lib.rs:724) | `Outcome.refused: Option<Refusal>`, closed (session.rs:580-617); words at the edge (flo_editor_host/src/words.rs:68) | none; a `u64` revision | `Arc<parking_lot::RwLock<State>>` owned by the view (flo_editor_view/src/ui/editor_view.rs:102-104) | `"0.6"` macros. **Matches `Hexserror` exhaustively** (nimble_ui.rs:242-251) |
| Formula | 13 directive structs, pure: `apply(&self, &ExpressionDraft[, ports]) -> Result<ExpressionDraft, EditRefusal>` (formula/interpreter/formula_editor_core/src/directive.rs:291-732) | `EditRefusal`: `Copy`, 30, no words by design (refusal.rs:7-10); order pinned by hand (:193-207) | none | by value | `"0.6.1"` default. Rejects `DirectiveHandler`/`QueryHandler` because `Hexserror` is text (query.rs:81-84) |
| Strataform | pure `decide(&Editor, Command) -> Result<Vec<Event>, Refusal>` (strataform/crates/strataform_editor_core/src/session.rs:392); `apply(Editor, &Event) -> Editor` (:556); driver `frame(&mut self, Vec<ComponentEvent>) -> Vec<Request>` (strataform_editor_view/src/ui/driver.rs:321) | `#[non_exhaustive]`, 11, Display words (session.rs:235-306) | returned (:345) | by value; no lock, walker-enforced (strataform_editor_standalone/tests/no_lock_on_the_frame.rs:57) | `"0.6.0"` no-default `macros`, `static-di` (Cargo.toml:200) |
| Recon editor | none yet; plan is Deck's `reduce(doc, session, intent) -> Step` (PRD-891 :239; 241911) | closed, "never a string" | in the Step, appended by the host | one ArcSwap snapshot | wants both wasm targets |
| Raconteur editor | none yet. PRD-895 :766: "Handlers wait for hexser 0.7 ... no lock is added" | `KeyRefusal`, closed by comment (raconteur_editor_core/src/key.rs:48) | none yet | none | `"=0.6.1"` (Cargo.toml:21) |
| OS `ActorBody` | `validate(&self, &Command)`; `commit(&mut self, Command) -> Result<Vec<Event>, ActorError>`; 5 impls (squillo_os/crates/squillo_os_kernel_core/src/lifecycle/body.rs:79-126) | 3 String variants | returned | n/a | OS `"0.6.1"` + ai, visualization, mcp (Cargo.toml:5591) |
| OS `TuiReducer` | `reduce(&mut self, Msg) -> (Vec<Observation>, Vec<Effect>)`; 6 impls (crates/squillo_tui_kit/src/reducer.rs:72, :102) | n/a | returned | n/a | as above |
| OS `CommandContract` | `execute_command(&mut self, &str, String) -> HexResult<CommandResult>`; 4 impls (crates/squillo_plugins/src/api/slot_traits.rs:461) | `Hexserror` | n/a | n/a | as above |

**Users of the 0.6.1 surface.** The counts are rg over the OS tree and every lane tree above, excluding `target*/`, `PRDs/`, `vendor/` and comment lines; the positive control is `dependency_edge_exists`.
- `DirectiveHandler`: 3 impls in squillo_vault_seal (src/application/device_handlers.rs:66, :81, :97), called at 3 sites in squillo_headless (src/vault_device_boot.rs:712, :722, :739).
- 0 for each of `QueryHandler`, `UseCase`, `InputPort` and `ports::Query`.
- `Hexserror`: 273 code lines in 22 OS crates.
- Templates: 321 call lines in 29 OS crates, of which 134 calls carry `depends_on`.
- `dependency_edge_exists`: 158 in 18 crates.
- `NodeId::from_type_name(info.type_name)`: 51 sites, the family's way of finding a node.
- Paths named by the family, top down: `hexser::graph` 1,288, `registry` 346, `templates` 286, `error` 134, `prelude` 119; `application` 11, `adapters` 7, `static_di` 4, `domain` 4; 0 for `ports`, `ai`, `container`, `infrastructure`, `showcase`. What `prelude::*` brings into use is not measured.

### What they share: the crossing type

1. **One exclusive writer.** Every writer is `&mut self`, `&mut State`, or value in and value out. None needs a lock to write.
2. **One command, consumed by value.** Each is a closed enum, except formula's per-directive structs.
3. **The output is not shared.** `Result<Event, Refusal>`, `Arc<View>`, `()`, `Outcome { refused }`, `Result<Vec<Event>, _>`, `(Vec<_>, Vec<_>)`. Hence an associated output.
4. **Per-call context is not shared.** Ports may be held or passed per call. Hence an associated context: the same forcing as SB 15/183 :49-57.
5. **Every refusal, command and event is a closed enum whose identity is its variant's name.**
   - Nimble's `ALL`, formula's order cell and Deck's T68 each list those names by hand.
   - Words are said at the edge in Deck, Tables, Flo, formula and Recon's plan. Strataform and Raconteur use Display, and Nimble keeps static guidance in the type.
6. **No editor lane implements `DirectiveHandler` or `QueryHandler`.**

## 5. What 0.7 must offer, each against its evidence

| Requirement | Asked by | Design |
|---|---|---|
| A handler that fits a single-writer actor and returns its event or typed outcome | brief; Deck 231868 (b); F17 | D1 |
| Refusals as closed typed values that still carry guidance, without becoming strings | brief; Deck 231868 (c); T69; formula query.rs:81-84; Nimble's `_ =>` | D2, D3 |
| Declared commands, refusals and events equal the Rust enums in order | PRD-886 T68 | D2 |
| Edges on the derives: an adapter names its port | brief; Flo 229010; Deck 231868 (a); H2; T67 | D5 |
| Edges from directives and queries to the aggregate and the driven ports | formula 240776 item 3 | D5 (`depends_on` on every derive) |
| An adapter with no port is refused | Flo 229010; Deck 231868 (a) | D6 (`AdapterWithoutProvedPort`) |
| A named node that is not registered is an error, not a silent missing edge | formula 240776 item 3 | D6 (`EdgeToUnregisteredNode`), plus the existing node check in both readers |
| A port node that cannot outlive its trait | brief; H3; T67 | D4 |
| A wait-free read that cannot fail | Deck 231868 (d) | D8: stays the lane's `ArcSwap::load`. hexser adds no read door |
| No lock added to fit | brief; F17; SB 15/10; N_BOOK §6.6.3 | D1 |
| wasm32 for a core crate | brief; PRD-847 :170; Taxa 241877; Recon 241911 | D10 |
| A 0.6 to 0.7 migration guide | brief | D11 |
| DRY, Security, Reliability, Maintainability, Speed; evidence in the system map | Scott's Q3 order | §12, D9, §13 |

## 6. Design

The house style holds throughout: fully qualified paths and no `use`, a Revision History row on every altered file, a `/// why:` line on every test, no `unsafe`, and no `.unwrap()` in `src/`.

### D1. `Actor`: the single writer's door (hexser/src/application/actor.rs)

```rust
pub trait Actor {
  /// The proposal (N_BOOK 3/6/6 :19-22), consumed by value.
  type Command;
  /// What the writer borrows for one call and does not own; `()` where it holds its ports.
  type Context<'a>;
  /// What the actor decided: an event, facts, a view, a refusal, or a pair of them.
  type Outcome;
  fn handle(&mut self, command: Self::Command, context: Self::Context<'_>) -> Self::Outcome;
}
```

The measured writers fit it without changing a body:

| Writer | Associated types |
|---|---|
| `DeckActor` | `Command = DeckChange`, `Context<'a> = ()`, `Outcome = Result<DeckEvent, ChangeRefusal>` |
| `EditorActor` | `Outcome = Arc<EditorView>` |
| Tables | `Outcome = ()` |
| Flo | `impl Actor for StepGraphEditorState` with `Context<'a> = flo_editor_core::session::Context<'a>`. The view keeps its own lock |
| `TuiReducer` | `Outcome = (Vec<Observation>, Vec<Effect>)` |

What the trait leaves out:
- no `Send`, `Sync` or `'static` bound;
- no forced `Result`;
- no `Hexserror`;
- no event sink.

Pure deciders stay functions (D8).

**`#[derive(HexActor)]`** registers `Layer::Application`, `Role::DirectiveHandler`. Both exist, so no enum grows. It goes through the shared `registrable_and_submit` codegen (codegen.rs:71) and takes `#[hex(depends_on(..))]`. Generic actors register a non-generic marker (CHANGELOG.md:95).

### D2. `Vocabulary`: a closed set that crosses a boundary (hexser/src/domain/vocabulary.rs)

```rust
pub trait Vocabulary: ::core::fmt::Debug + 'static {
  /// Every code, in declaration order: the list a declaration is compared against (T68).
  const CODES: &'static [&'static str];
  /// This value's stable code: the variant's name unless `#[hex(code = "...")]` says otherwise.
  fn code(&self) -> &'static str;
}
```

**`#[derive(HexVocabulary)]`** works on any enum: commands, events, refusals, data-carrying or not. `#[non_exhaustive]` is accepted, because `CODES` is complete inside the defining crate.
- On an all-unit enum it also writes an inherent `pub fn from_code(&str) -> Option<Self>`. That function is total, and an unknown code is `None` (SB 15/146).
- A struct or a union is a compile error that names the remedy.
- It writes no `Display` and no `Default`.
- `#[hex(code = "...")]` lets a wire code outlive a variant rename, so Nimble's codes survive. It also keeps a contract stable at scale.

This one derive retires three hand-written lists: Nimble's 67-member `ALL`/`code`/`from_code`, formula's order cell, and Deck's T68 list.

### D3. `Refusal`: a vocabulary with optional static guidance (hexser/src/error/refusal.rs)

```rust
pub trait Refusal: crate::domain::vocabulary::Vocabulary {
  fn next_steps(&self) -> &'static [&'static str] { &[] }
  fn suggestions(&self) -> &'static [&'static str] { &[] }
}
```

- **`#[derive(HexRefusal)]`** is `HexVocabulary`'s expansion plus `impl Refusal`. Two optional per-variant keys fill the guidance: `#[hex(next_step = "...")]` and `#[hex(suggestion = "...")]`. One codegen function serves both derives.
- **At the edge**, both additive:
  - `Hexserror::refused(&R, message: &str) -> Hexserror` builds the existing `Domain` variant with `code = r.code()` and the static guidance copied. The edge adds its words through the existing builders (hex_error.rs:123-161).
  - `Hexserror::code(&self) -> &str` lets `R::from_code(err.code())` recover the closed value or `None`.
- **No `Hexserror` variant is added**, so Flo's exhaustive match and the 22 OS crates keep compiling.
- **A refusal's code is a static, closed identifier that carries no caller data**, so it can be logged as is (§13 E2).

### D4. A port node that cannot outlive its trait: one key on the existing `HexPort`

```rust
#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = crate::ports::EventSinkPort)]
pub struct EventSinkPortNode;
```

Besides today's registration, the derive emits one hidden, bounded, inherent function:

```rust
impl EventSinkPortNode {
  #[doc(hidden)]
  pub const fn __hexser_port_trait<T: ?Sized + crate::ports::EventSinkPort>() {}
}
```

Consequences:
- Delete or rename the trait, and the port node does not compile (T67).
- The key works for every trait shape: generic (`port_trait = path::Trait<X>`), with associated types, and not dyn-compatible. Nothing is injected into the user's trait.
- Existing markers move mechanically, one key each. Taxa, the tunnel and Recon asked for exactly that.
- Templates get parity: `hex_register_port!(Node, Role::OutputPort, port_trait: path::Trait)`.
- A marker without the key keeps compiling, as 0.6 does. It just cannot be named by `implements` (D5).

This replaces the first plan's `#[hexser::port]` attribute macro (§12 A1).

### D5. Edges on the derives, and one reading function per relationship

- **`#[hex(implements(PortNode, ..))]` on `HexAdapter`.** For each port node the derive emits two things:
  - `const _: () = PortNode::__hexser_port_trait::<TheAdapter>();`, with the adapter's own type name, because `Self` is not in scope in a free item. An adapter that does not implement the port's trait fails to compile. So does an adapter naming a marker that never declared its trait.
  - An `Implements` edge from the adapter's node to the port node. This is the first producer of `Relationship::Implements`, and it is reserved for proved edges (SB 15/380).
- **The edge rides its own `inventory` collection,** `ImplementsEntry { adapter, port }`. `ComponentEntry` and `Registrable` are left alone, so there is no semver break (§3). `build_graph` reads both collections.
- **`#[hex(depends_on(T, ..))]` on every registration derive** (`HexDirective`, `HexQuery` and `HexActor` included) gives one `Depends` edge per type. It uses the template's `dependency_id` (templates/mod.rs:97-99), so there is one vocabulary.
- **Template parity:** `implements: [PortNode]` on `hex_register_adapter!` and `hex_register_component!`. The 134 existing `depends_on:` calls are untouched.
- **Reading:** `hexser::templates::implements_edge_exists::<A, P>(&graph) -> bool`. It is a function, not a macro, and it shares one private `edge_exists(graph, a, p, relationship)` with `dependency_edge_exists` (templates/mod.rs:111-122, behaviour unchanged).

### D6. `HexGraph::findings()`: what is wrong, as closed values (hexser/src/graph/finding.rs)

```rust
pub enum GraphFinding {
  /// F-A: two registrations, one short name, two modules; keys unchanged (Q2).
  ShortNameCollision { id: NodeId, kept_type_name: String, kept_module_path: String, dropped_type_name: String, dropped_module_path: String },
  /// An Adapter-layer node (not Role::Mapper) with no Implements edge: its port, if any, is only declared.
  AdapterWithoutProvedPort { adapter: NodeId },
  /// An edge whose source or target is not a registered node.
  EdgeToUnregisteredNode { source: NodeId, target: NodeId, relationship: Relationship },
}
```

- `GraphFinding` implements `Vocabulary` by hand: hexser cannot use its own derives, since lib.rs has no `extern crate self as hexser`.
- `GraphBuilder::build` records `ShortNameCollision` when the module paths differ, not only the names (builder.rs:104-113).
- The existing `metadata.warnings()` strings are now rendered from the typed finding, so 0.6 readers still see them.
- The other two findings are computed on demand, in O(nodes + edges).
- Nothing panics, and `build_graph` refuses nothing. A lane's graph cell asserts `findings()` is empty, or names the ones it accepts. That is the "check that refuses an adapter with no port" (229010, 231868 (a)) without breaking any graph that compiles today.

### D7. Retiring duplicate doors (SB 15/203: route, then unreach, then delete)

- **`#[deprecated]` in 0.7, each with a note naming its route:**
  - `DirectiveHandler`, `application::UseCase` and `InputPort` route to `Actor`;
  - `ports::Query` routes to `QueryHandler`, which has the same shape and 0 impls.
- **This does not break semver.** Removal is a later version's card.
- **Measured reach:** squillo_vault_seal's 3 impls and squillo_headless's 3 calls, only when the OS moves its pin. Under `clippy -D warnings` that is one `#[allow(deprecated)]` or a move to `Actor`.
- F-C stays, and its doc says plainly that the derived `validate()` checks nothing.

### D8. Declined, so it is not re-proposed

- **A pure `Decide` or `Reduce` trait.** Nothing is generic over those functions.
- **A hexser read door or `Published<T>`.** A wait-free read is `ArcSwap::load` (Deck), and Tables' arc-swap row is security's call (239934 item 4). This answers 231868 (d).
- **A `Hexserror::Refused` variant.**
- **A second port macro.** See D4.
- **Sealing the new traits.**
- **Re-keying template nodes (Q2).**

### D9. Security: no `unsafe`, one read of the switch

- **The switch.** `env_control` splits into a pure `fn includes_location(value: Option<&str>) -> bool` and a process-wide `std::sync::OnceLock<bool>` read of `HEXSER_INCLUDE_SOURCE_LOCATION`.
  - The default stays off.
  - This is a behaviour change: a program that changes the variable while it runs is no longer seen. Since edition 2024, changing the environment at run time takes `unsafe`. It goes in the CHANGELOG.
- **The tests.**
  - The unit tests test the pure function.
  - The serde integration tests that need the switch on re-run their own test binary with the variable set. They use `std::process::Command::new(std::env::current_exe())` with `.env(..)`, so the environment is fixed at process start, with no `unsafe` and no `serial_test` lock.
- **The gate.** `#![forbid(unsafe_code)]` goes in hexser, hexser_macros and hexser_potions.
  - The wasm-e2e probe keeps its one `#[unsafe(no_mangle)]` export (§3). It is a test harness and is never published. That is said plainly in §11.
- **The release gates** gain `cargo audit` and `cargo deny check` (deny.toml exists). Their results go into the landing's gate record.

### D10. wasm32

`wasm-e2e/` grows a probe that executes under wasmtime, on `wasm32-unknown-unknown` and `wasm32-wasip1`, with `--no-default-features --features macros`:
- an `Actor` of each D1 shape;
- a `HexVocabulary` round trip;
- a `HexRefusal` round trip through `Hexserror`;
- a `port_trait` node;
- an `Implements` edge read back;
- `findings()`.

Reverting any one of them turns the job red. There are no new dependencies.
- As built: the probe pulls hexser with `macros` only and puts the wider surface (static-di, serde, ai, visualization) behind its default `surface` feature. `scripts/wasm-e2e.sh` runs it twice per target, surface and macros-only, and the compile matrix gains the macros-only set.
- Must-fail evidence: flipping the `findings()` assertion in the macros-only probe traps on both targets (wasmtime exit 134), then restored.

### D11. Migration guide and skills

- `hexser/MIGRATING_0.6_TO_0.7.md`, at the root of the `hexser` package, linked from both READMEs and the CHANGELOG. It sits in the package, not the workspace root, so it ships in the .crate and its examples compile as doctests (`#[cfg(doctest)]`). Each change is shown before and after:
  - `Actor` in place of `DirectiveHandler`;
  - `HexVocabulary` in place of a hand-written name list;
  - `HexRefusal` and `Hexserror::refused`;
  - `port_trait` on a marker;
  - `implements(..)` in place of a template `depends_on` onto a port marker;
  - `findings()` in a graph cell;
  - the deprecations, and the switch's read-once change.
- plugins/hexser:
  - `migrate.md` gains 0.6 to 0.7;
  - `add-usecase.md` teaches `Actor`;
  - `review.md` checks closed vocabularies, words at the edge, `port_trait`, proved edges, and `findings()`;
  - `SKILL.md` matches;
  - the agent pack's "no `unsafe`" claim becomes true (F-D).

### D12. Versions and landing

- `hexser`, `hexser_macros` and `hexser_potions` go to 0.7.0, and the workspace rows move with them (Cargo.toml:40-41 still say 0.6.0 today).
- Every family pin is `0.6`/`0.6.x`, so none picks up 0.7 on `cargo update`. Each owner moves its own pin.
- Acts, as Q1 ruled:
  1. commits on `hardening/pr0-signal-260720` (no trailer);
  2. push;
  3. a PR to `main` carrying 0.6.0, 0.6.1 and 0.7.0;
  4. merge on green;
  5. tag `v0.7.0`.
- **The crates.io publish is not covered by Q1 and has its own card:** `hexser_macros`, then `hexser`, then `hexser_potions`.

## 7. Phases

Red first, then green. Each `stc run -- cargo` takes a BUILD-LOCK-TURNS turn, one at a time, then clippy `--all-targets -- -D warnings`.

| Phase | Work | Rows |
|---|---|---|
| P0 | F-D: forbid `unsafe`, pure switch reader, subprocess serde tests | R01-R03 |
| P1 | F-A as a typed finding, then `findings()` | R04-R08 |
| P2 | `Actor` + `HexActor` | R09-R12 |
| P3 | `Vocabulary` + `HexVocabulary`, `Refusal` + `HexRefusal`, `Hexserror::refused` and `code` | R13-R20 |
| P4 | `port_trait`, `implements(..)`, `depends_on(..)` on the derives, template keys, readers | R21-R27 |
| P5 | Deprecations | R28 |
| P6 | wasm-e2e probe | R29 |
| P7 | Migration guide, CHANGELOG, README, plugin skills, versions | R30 |
| P8 | Gates, then the landing acts (D12) | none |

## 8. Rows

[U] is a unit or integration test, [UI] a trybuild compile-fail, [E] wasm execution. Each row names the mutant that must turn it red.

| Row | Name | Mutant that must turn it red |
|---|---|---|
| R01 | [U] `the_location_switch_is_on_only_for_1_or_true_in_any_case`, on the pure reader | accept any non-empty value |
| R02 | [U] `a_serialized_error_carries_its_location_only_when_the_process_starts_with_the_switch_on` (re-exec with `.env`) | ignore the switch |
| R03 | [U] `this_crate_forbids_unsafe_code`, one per crate: each reads its own lib.rs with `include_str!` and asserts the attribute (a published crate cannot `include_str!` a sibling's source). Also, the 28 blocks fail to compile under it at 3c2f628 | remove a forbid line |
| R04 | [U] `two_template_nodes_with_one_short_name_in_two_modules_are_a_typed_finding` | revert the module-path compare. Red at 3c2f628 |
| R05 | [U] `one_type_registered_twice_is_not_a_collision` (the control) | report every duplicate id |
| R06 | [U] `an_adapter_with_no_implements_edge_is_a_finding_and_a_mapper_is_not` | count `Depends` as proof; drop the Mapper exemption |
| R07 | [U] `an_edge_to_an_unregistered_node_is_a_finding` | skip edge endpoints |
| R08 | [U] `the_0_6_warning_text_is_rendered_from_the_typed_collision` | drop the rendering |
| R09 | [U] `an_actor_of_each_measured_shape_implements_one_trait`: Deck, Tables and Flo shapes | fix `Outcome` to a `Result` |
| R10 | [UI] `an_actor_cannot_be_handled_through_a_shared_reference` | `&self` receiver |
| R11 | [U] `a_hex_actor_node_is_application_layer_directive_handler_by_name`, with a must-fail arm | default role `UseCase` |
| R12 | [UI] `hex_actor_on_a_generic_type_names_the_marker_remedy` | drop the generic check |
| R13 | [U] `codes_are_the_variant_names_in_declaration_order` (commands, events, refusals; data-carrying arms included) | sort the codes |
| R14 | [U] `from_code_is_total_and_an_unknown_code_is_none` | a `_ =>` arm returning a variant |
| R15 | [U] `a_code_override_wins_and_is_listed_in_codes` | ignore `#[hex(code)]` |
| R16 | [U] `a_non_exhaustive_enum_derives_its_vocabulary` | refuse `#[non_exhaustive]` |
| R17 | [UI] `hex_vocabulary_on_a_struct_names_the_remedy` | accept structs |
| R18 | [U] `a_refusal_carries_its_static_guidance` | drop `#[hex(next_step)]` |
| R19 | [U] `a_refused_hexserror_is_the_domain_variant_and_its_code_reads_back_to_the_refusal` | use the `Adapter` variant; drop the guidance copy |
| R20 | [U] `hexserror_still_has_exactly_six_variants` (an exhaustive match, no wildcard, as Flo's) | add a variant |
| R21 | [UI] `a_port_node_naming_a_trait_that_does_not_exist_does_not_compile` (T67) | none needed; compile-fail |
| R22 | [UI] `an_adapter_that_does_not_implement_its_ports_trait_does_not_compile` | drop the const proof |
| R23 | [UI] `an_adapter_naming_a_port_node_without_port_trait_does_not_compile` | emit the edge without the proof |
| R24 | [U] `port_trait_accepts_a_generic_trait_and_one_with_an_associated_type` | parse `port_trait` as a bare ident |
| R25 | [U] `an_implements_edge_lands_on_the_port_node_and_reads_back`, with a must-fail arm for a non-implementing pair | edge onto the adapter itself |
| R26 | [U] `depends_on_on_a_derive_equals_depends_on_on_a_template`, and never reads as `Implements` | build the id with `NodeId::of`; read any relationship |
| R27 | [U] `implements_edge_exists_is_false_when_the_ports_node_is_absent` | drop the node check |
| R28 | [U] `the_deprecated_doors_still_compile_under_allow_deprecated` (compile-pass cell) | delete a door |
| R29 | [E] wasm-e2e executes R09, R13, R19, R21's pass case, R25 and R06 on both wasm targets, macros only | revert any one door |
| R30 | [U] `the_migration_guide_names_every_new_public_item` (a doc walk, per section, with each named item checked against its source), plus its must-fail arm `a_guide_missing_a_section_or_its_body_fails_the_walk`. The guide's 13 examples run as doctests | delete a section |
| R31 | [UI] `hex_unknown_key` and `hex_key_on_wrong_derive`: an unknown `#[hex]` key, and a key the derive does not read, are compile errors naming the keys and the reader (A14) | swallow the parse error again |

The gates for green:
1. `stc run -- cargo test --workspace --all-features`;
2. `cargo test -p hexser --no-default-features --features macros`;
3. clippy `--all-targets --all-features -D warnings`;
4. `scripts/wasm-e2e.sh`;
5. `cargo audit`;
6. `cargo deny check`;
7. as built, CI's own gates too: `cargo fmt --check`, `cargo test --workspace` (default features), the `cargo check` feature matrix, `--examples`, `cargo doc` with `RUSTDOCFLAGS=-D warnings`, and the realworld_api crate's `cargo test`.

Gate 3 as built: CI's clippy gate is `--lib --bins -D warnings` (pr-test.yml:44), and that is the binding one. `--all-targets` carries 559 errors that predate 0.7 (examples, old test files, src test modules; 463 of them `disallowed_macros`). It is 559 after 0.7 as well. No new 0.7 file carries a hit. The touched files that do (two migrated examples, `directive_handler.rs`, `ports/query.rs`, `integration_test.rs`) carry only the older kinds: dead fields in old tests and examples, and `println!` in examples.

The 280 existing unit tests and every integration test stay green, except the env tests R01 and R02 replace.

## 9. Decisions

| # | Question | Ruled |
|---|---|---|
| Q1 | Plan and landing | Scott: "Approve; land and merge on green (Recommended)". Publish keeps its own card |
| Q2 | Template node identity (F-A) | Scott: "Report it, keys unchanged (Recommended)". D6 makes it a typed finding |
| Q3 | The `&self` writer doors | Scott ordered the audit (§0). The audit's reading: SB 15/203 decides it, so the doors are deprecated in 0.7 and `ports::Query` is added to the set (D7). Deprecation is not a semver break, so it is not a breaking change for Scott's card. Removal is |

Still Scott's, each a card when it comes:
- the crates.io publish;
- removing the deprecated doors (0.8);
- any re-key of template nodes;
- merging or closing the 7 open dependabot PRs (§11).

## 10. Risks, said plainly

- **The GAT context.** Deck and Tables would call `handle(cmd, ())` through the trait. Their inherent `handle` stays.
- **The const proof (D5)** costs one monomorphized empty `const fn` per claim and nothing at run time. A port node with `port_trait` gains one hidden inherent function. An implementor could call it, but it does nothing.
- **Findings noise.** Every adapter registered today through a template `depends_on` onto a port marker reads as `AdapterWithoutProvedPort`. That is correct (SB 15/380). It forces nobody, because `findings()` is read only by the cells that choose to.
- **Link time.** A port node registers in the crate that declares its marker. A binary that links none of that crate has no node (SB 15/413 :82-90).
- **The read-once switch (D9)** is a behaviour change for any program that flips the variable at run time.
- **Deprecation under `-D warnings`** reaches squillo_vault_seal and squillo_headless, only on their pin move.
- **Seen, not hexser's:**
  - Strataform's recorder holds a `tokio::sync::Mutex` across reduce, append and fold (strataform_runtime/src/apply_event_recorder.rs:157, :228).
  - The OS's three local `&mut self` doors (§4) are their owners' to route.
  - Nimble's `DataPort::fetch_epoch` is required, and the canvas and Flo `Feed`s lack it (posted, 242296).

## 11. Not measured, and not done

- **Not measured:**
  - what `hexser::prelude::*` brings into use across the family;
  - the 29 pass-through forwarders the simplify census flags (§12), which this audit did not read one by one;
  - Security, Reliability and Speed on the simplify instrument, which reports them NOT-MEASURED for this root. This audit measured them by hand instead (§12);
  - generated code, and trees outside §4.
- **Seen, not acted on:**
  - Seven dependabot PRs have been open on squillo/hexser since 2026-07-21 to 2026-09-15 (#2, #3, #5, #6, #7, #13, #14). syn 3 (#6) is a major bump for the macro crate, and reqwest (#5) and criterion (#7) are dev-dependencies only.
  - 0.7 takes none of them. `cargo audit` and `cargo deny` at the gate decide whether one is a security fix, and merging or closing any of them is an outward act with its own card.
  - squillo/hexser has no open issues.
- **Kept, said plainly:** the wasm-e2e probe's `#[unsafe(no_mangle)]` export (D9).
- **Gate record (2026-11-17, `stc run`, one cargo at a time):**
  - `fmt --check`, CI clippy (`--lib --bins -D warnings`), the 10-cell `cargo check` feature matrix, release bins, `--examples`, `cargo doc` with `-D warnings`: all exit 0;
  - `cargo test --workspace`: 345 passed, 0 failed, 2 ignored;
  - `cargo test --workspace --all-features`: 456 passed, 0 failed, 3 ignored (doctests included; the guide's 13 among them);
  - `cargo test -p hexser --no-default-features --features macros`: 336 passed, 0 failed;
  - realworld_api `cargo test`: 172 passed, 0 failed;
  - `scripts/wasm-e2e.sh`: 18 compile cells, and the probe executed on both targets, surface and macros-only;
  - `cargo audit` and `cargo deny check`: clean after A15, no waivers.
- **Not done, said plainly:**
  - The README's long tutorial still shows `DirectiveHandler` in three blocks, each under a 0.7 note; the canonical examples moved (A16).
  - `HexAggregate` declares `#[hex]` and reads no key, so on a type that derives only it, a `#[hex(...)]` is still ignored.
  - Cargo manifests, the CHANGELOG and the READMEs carry no Revision History block, as before 0.7; none was invented.
  - The system map rows (§13) are staged, not written; they wait for the merge.
  - The crates.io publish waits for its own card.
  - `hex_register_port!` takes `port_trait:` or `depends_on:`, not both; no arm combines them.
  - A generic adapter is not registered by the derive (0.6 rule), so it carries no `implements` edge; the docs say so and offer no workaround.
- **plugins/hexser (D11):** every command and reference moved to 0.7, the manifests to 0.7.0. The scaffold (`/hexser:new`) now emits a `port_trait` node, an `implements` proof and a `findings()` cell, so a fresh slice has no finding. The plugin docs' complete examples (SKILL.md's minimal shape, add-usecase's actor, patterns.md's end-to-end program and actor, the new.md scaffold) were compiled and run verbatim in a throwaway test target, then deleted; the rest are marked sketches. The 0.4 → 0.5 reference keeps its `0.6` landing pin by design.

## 12. The audit (Scott's Q3 order)

### What the audit changed

| # | Finding | Axis | Change |
|---|---|---|---|
| A1 | The first plan added a `#[hexser::port]` attribute beside the marker `HexPort`: two tools for one job (SB 15/203). It also injected a hidden item into user traits and needed three spike rows for dyn-compatibility and re-exports | DRY, Reliability, Maintainability | D4: one key on the existing derive. One macro kind fewer, no item injected, three risk rows gone, and every trait shape covered |
| A2 | T68 needs commands and events listed in order, not only refusals. The first plan's `CODES` lived on `Refusal` only | Completeness (PRD-886 T68) | D2: one `Vocabulary` for all three. `Refusal` extends it |
| A3 | Flo 229010 and Deck 231868 (a) asked for "a check that refuses an adapter with no port". The first plan had none | Completeness (lane asks) | D6 `AdapterWithoutProvedPort` |
| A4 | Formula 240776 asked that a named node which is not registered be an error, not a silent missing edge | Completeness, Reliability | D6 `EdgeToUnregisteredNode` |
| A5 | The F-A collision record is a `String` warning, against the rule that a decision is a value (SB 15/146) | Reliability, data contracts | D6 `ShortNameCollision`, with the string rendered from it |
| A6 | Deck 231868 (d), the wait-free read, had no answer in the plan | Completeness | D8 answers it: `ArcSwap::load`, no hexser door |
| A7 | Two read doors with one shape: `QueryHandler` and `ports::Query`, 0 impls each | DRY | D7 adds `ports::Query` to the deprecations |
| A8 | 28 test-only `unsafe` blocks, against the house rule and the crate's own agent pack. No `forbid(unsafe_code)` | Security (ISO A.8.28) | D9 |
| A9 | The location switch reads the environment on every serialized error | Speed | D9: read once (`OnceLock`) |
| A10 | `Implements` would have needed a new `ComponentEntry` field, a semver break: its fields are all `pub` | Reliability (semver) | D5: its own `inventory` collection |
| A11 | Two readers would duplicate the edge walk | DRY | D5: one private `edge_exists` |
| A12 | The gates had no dependency audit, though deny.toml and both tools exist | Security (supply chain) | D9 and §8: `cargo audit` and `cargo deny check` |
| A13 | Four PRD counts drifted against live trees: templates 291 → 321 lines, `depends_on` calls 125 → 134 | Correctness | §4 re-measured, positive control 158 |
| A14 | Found while building: each derive's `role_override` discarded its parse error, so a malformed `#[hex]` dropped every key after it and the derive applied its default in silence. With 0.7's new keys, that silence would hide a lost `implements` | Reliability, Security (A.8.28) | One strict reader for every derive (`hexser_macros/src/common/attrs.rs`), R31. The census found only `role` written across the family (536 sites, never with a comma), so it breaks no known crate |
| A15 | Found while gating: `cargo audit` and `cargo deny` failed at 3c2f628's lock: `tokio` 1.47.1 (RUSTSEC-2026-0386, unsound) and `pin-utils` (RUSTSEC-2026-0441, unmaintained, via `futures-util` 0.3.31 and `hyper` 1.7.0) | Security (supply chain) | Lock-only moves: `tokio` 1.54.1 (MSRV 1.85), `futures-util` 0.3.34, `hyper` 1.12.0. `pin-utils` leaves the tree; no waiver. No manifest changed |
| A16 | Found while documenting: the two canonical examples taught the deprecated `DirectiveHandler`, and each said its `&self` handler could not write | Mission, Maintainability | `examples/cqrs_pattern.rs` and `tutorial_04_application.rs` now write through an `Actor` with a `HexRefusal`, and both run |
| A17 | Found while documenting: hexser's SNAPP_CONTRACT still called the ArcSwap and IndexMap moves "not yet merged", four months after they landed | Correctness (PRD-272 §1.1) | Both contracts re-read against the code and updated, with the read-once `OnceLock<bool>` declared as a carve-out |

### The `stc simplify` reading

Scan 34750d39 covered 211 of 211 files under nlang_hex.
- **DRY, T0 floor:** 59‰ redundant tokens over 1,216 production functions.
- **DRY, fused T0+T1 near-miss mass:** 279‰, reported apart and not summed.
- **The production clone classes are small and shallow.** Two classes have several production members:
  - an accessor shape (`aggregate_id` and the like) across ports/events/*.rs and adapters/in_memory_event_bus.rs, at 1-10‰;
  - a 99-member, mostly examples-and-benches class at 4‰.
  - None touches 0.7's surface, and collapsing them would add an abstraction to save a few lines each. Not taken.
- **Maintainability:** 2 layout-convention findings, cohesion half not run.
- **Security, Reliability and Speed:** NOT-MEASURED by the instrument, because the graph store has no nodes under this root. A1-A13 are the hand measurement.
- **Not read:** the 29 forwarders (§11).
- The scan wrote `.squillo/simplify/` in the checkout. That is not source and is never committed.

### Value and mission kept

- **Every 0.6.1 surface still compiles:**
  - derives, templates, `depends_on` and `dependency_edge_exists`;
  - the marker `HexPort`;
  - `Hexserror`'s six variants and builders;
  - `QueryHandler`, `static_di`, `ai`, `mcp` and `container`.
- The deprecated four warn and route to their replacements.
- **The data-contract mission gains:**
  - every command, event and refusal has a stable, ordered, code-addressable vocabulary, which survives renames through `#[hex(code)]`;
  - port claims are proved by the compiler;
  - graph defects come back as typed values a cell can assert at any scale (O(nodes + edges));
  - nothing new requires a lock, a thread, or a dependency, so wasm32 is unchanged.

## 13. Compliance evidence staged for the system map

The system map is /Users/scott/@squillo/squillo-system-map/src/map/table.n.md, with 148 controls rows, written by Tirith ORCH f5693f2e with the keeper's certificate.
- **These rows are written only after the merge, and only by that path.** A row for a control the code does not yet carry is never written as its evidence (the tunnel lane's rule, 239472).
- **No new ids.** Every framework and control id below already occurs in the file: A.8.28 ×11, A.8.15 ×26, CC8.1 ×10, 164.312(b) ×21, and the HECVAT-4 themes.

| Row | framework, control_id | claim | evidence_query | maps_to |
|---|---|---|---|---|
| E1 | ISO27001-2022+A1-2024, A.8.28 | hexser, hexser_macros and hexser_potions forbid `unsafe` code at the crate root, and an adapter's claim to serve a port compiles only when the adapter implements that port's trait | the three `#![forbid(unsafe_code)]` lines; R03; R21-R23 (compile-fail) | intersects SOC2-TSC-2017:CC8.1; HECVAT-4:change management theme |
| E2 | HIPAA-45CFR164-2013, 164.312(b) | a refusal crosses hexser as a closed, static code that carries no caller data, and serialized errors omit source locations unless the process starts with the switch on | R13-R15, R19; R01-R02 | intersects ISO27001-2022+A1-2024:A.8.15; HECVAT-4:logging theme |
| E3 | SOC2-TSC-2017, CC8.1 | an architecture graph reports, as closed typed findings, an adapter with no proved port, an edge to an unregistered node, and two registrations sharing one short name | R04-R08, R25-R27; each lane's graph cell that asserts `findings()` | intersects ISO27001-2022+A1-2024:A.8.28; HECVAT-4:documentation and assurance theme |

## Revision History

- 2026-11-17T17:07:54Z @AI (the hexser lane, claude_code:2423a90f): built P0-P7 and ran the gates (§11).
  - D6's collision fields, R03's name, D10 as built, and the guide's location (D11) now match the code.
  - R31 and audit rows A14-A17 were added; the gate list gained CI's own gates.
  - The PRD now says plainly what is still not done.
- 2026-11-17T16:15:38Z @AI (the hexser lane, claude_code:2423a90f): Scott's three card answers recorded verbatim (§0, §9). The audit he ordered is §12.
  - It was run with `stc simplify` (scan 34750d39) plus hand measurement of the axes the instrument could not see.
  - D4 is now a key on `HexPort`, not an attribute macro.
  - D2 `Vocabulary` was added, and `Refusal` extends it.
  - D6 `findings()` was added for three lane asks and F-A.
  - D7 is decided by canon, with `ports::Query` added.
  - D9 was added: no `unsafe`, the read-once switch, the audit and deny gates.
  - §13 stages the evidence rows.
  - Rows were renumbered R01-R30. Counts were re-measured.
- 2026-11-17T15:56:37Z @AI (the hexser lane, claude_code:2423a90f): initial plan.
  - Canon was read first, by the lane: SB 15/183, /191, /146, /209, /380, /413, /10, /203; N_BOOK 3/6/6 and 3/24/1; PRD-847 :166-175; PRD-886 F12/F17/F26, §8.3.7, T67-T69.
  - hexser 0.6.1 was measured at 3c2f628.
  - The writers were enumerated across Deck, Tables, canvas (Nimble), Flo, formula, Strataform, Recon, Raconteur and the OS. Read-only measurement agents gathered this; the lanes answered on the ledger (241877, 241901, 241911).
