//! The hexser surface a WASM consumer actually touches, exercised inside a real wasm runtime.
//!
//! This crate exists because `cargo build --target wasm32-unknown-unknown` is not a WASM test.
//! hexser compiled clean for that target for five releases while `HexGraph::current()` trapped
//! the module on first touch: std routes `SystemTime::now()` through its `unsupported`
//! platform layer there, where it is a hard `panic!`. Only executing the module catches that
//! class of defect, so every check below runs under `wasmtime`.
//!
//! `run_checks` must never print. `println!` panics when it cannot write, and stdout is one of
//! the std surfaces `wasm32-unknown-unknown` does not have — printing here would re-introduce
//! exactly the kind of trap this crate exists to catch. The binary in `main.rs` is the one
//! entry point on both targets: it reports on wasip1, and on `wasm32-unknown-unknown` it is
//! invoked as the module's exported `main`, where a failed check traps.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 (house rule: no `unsafe`) — drop the `#[unsafe(no_mangle)]` `probe` export; a `wasm32-unknown-unknown` bin already exports `main`, which runs the same checks. The probe is now `#![forbid(unsafe_code)]`.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D10 — the static-DI, AI-timestamp and visualization checks run under the default `surface` feature only; every other check, the 0.7 doors included, also runs macros-only.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D10 (R29) — execute the 0.7 doors under wasmtime: an `Actor` of the Deck and Tables shapes, a `HexVocabulary` round trip, a `HexRefusal` through `Hexserror` and back, a `port_trait` port node, an `implements` edge read back, and an empty `findings()`.
//! - 2026-09-11T00:00:00Z @AI: Initial probe covering graph construction, metadata clock, static DI, errors, AI timestamps and visualization export.

#![forbid(unsafe_code)]

/// A domain entity registered through `inventory` at link time.
///
/// Its presence in `HexGraph::current()` is the assertion that matters: `inventory` relies on
/// life-before-main linker sections, which wasm has no native equivalent for, so registration
/// working under wasm is a per-target fact rather than something the host build can vouch for.
#[derive(hexser::HexDomain, hexser::HexEntity)]
pub struct ProbeEntity {
  /// Identity field — `HexEntity` derives `Id` from it.
  pub id: std::string::String,
}

/// True when built with the default `surface` feature; false for the macros-only probe.
pub const SURFACE: bool = std::cfg!(feature = "surface");

/// True when the target has no std clock, and hexser timestamps are therefore pinned to 0.
///
/// Kept as one named constant so each check reads the same rule the crate under test applies.
pub const CLOCKLESS: bool = std::cfg!(all(target_arch = "wasm32", target_os = "unknown"));

/// Run every check, panicking (trapping the wasm module) on the first failure.
///
/// Returns the number of nodes in the architecture graph, so a caller that only sees an exit
/// code and a return value can still tell "registration produced a graph" from "registration
/// silently produced nothing".
pub fn run_checks() -> u32 {
  let node_count = check_graph_registration();
  check_metadata_clock();
  check_errors();
  #[cfg(feature = "surface")]
  {
    check_static_di();
    check_ai_timestamp();
    check_visualization();
  }
  check_actor();
  check_vocabulary_and_refusal();
  check_proved_edges();
  node_count
}

/// `HexGraph::current()` builds from the link-time registry without trapping, and the derived
/// entity is actually in it.
///
/// This is the exact call that used to trap: `current()` forces a `LazyLock` that runs
/// `ComponentRegistry::build_graph()`, which stamps `GraphMetadata` with the clock.
fn check_graph_registration() -> u32 {
  let graph = hexser::HexGraph::current();
  let count = graph.node_count();

  std::assert!(
    count >= 1,
    "inventory registration produced no nodes under wasm; ProbeEntity should be present"
  );
  std::assert!(
    graph.nodes().any(|n| n.type_name().contains("ProbeEntity")),
    "the derived ProbeEntity is missing from HexGraph::current() under wasm"
  );

  count as u32
}

/// Graph metadata carries the target's honest answer about the clock, in both directions.
///
/// A guard that returned 0 everywhere would also pass a "does not trap" check, so the native
/// and WASI side is asserted here too: 0 on a clockless target, a real present-day timestamp
/// anywhere with a clock.
fn check_metadata_clock() {
  let created_at = hexser::HexGraph::current().metadata().created_at;

  if CLOCKLESS {
    std::assert_eq!(
      created_at,
      0,
      "a clockless target must report created_at = 0, not a fabricated time"
    );
  } else {
    std::assert!(
      created_at > 1_700_000_000,
      "a target with a clock must stamp a real present-day created_at, got {created_at}"
    );
  }

  // An explicitly constructed graph walks the same stamping path; both used to trap.
  let _ = hexser::HexGraph::new();
  let _ = hexser::graph::GraphMetadata::new("probe");
}

/// The documented WASM dependency-injection path builds and yields its values.
#[cfg(feature = "surface")]
fn check_static_di() {
  #[derive(Clone)]
  struct Repo(u8);
  #[derive(Clone)]
  struct Service {
    repo: Repo,
  }

  let app = hexser::hex_static!({
    let repo = Repo(7);
    let service = Service { repo: repo.clone() };
    (repo, service)
  });
  let (repo, service) = app.into_inner();

  std::assert_eq!(repo.0, 7, "static DI lost the constructed value");
  std::assert_eq!(service.repo.0, 7, "static DI lost the injected dependency");
}

/// Rich errors build and render without touching a std surface wasm lacks.
///
/// `Hexserror` captures source locations and consults the `HEXSER_INCLUDE_SOURCE_LOCATION`
/// environment variable; env reads are unsupported (not fatal) under wasm, and this pins that
/// they stay non-fatal.
fn check_errors() {
  let err = hexser::Hexserror::validation("probe rejected the input")
    .with_next_step("supply a non-empty value")
    .with_suggestion("see the port's contract");
  let rendered = std::format!("{err}");

  std::assert!(
    rendered.contains("probe rejected the input"),
    "error rendering lost its message under wasm: {rendered}"
  );

  let not_found = hexser::Hexserror::not_found("ProbeEntity", "probe-1");
  std::assert!(
    std::format!("{not_found}").contains("probe-1"),
    "not_found rendering lost its id under wasm"
  );
}

/// The `ai` feature's RFC3339 stamp is well formed, and honest about a missing clock.
///
/// This is the second site that read the clock directly, and the one `ai`/`mcp` builds hit.
#[cfg(feature = "surface")]
fn check_ai_timestamp() {
  let ts = hexser::ai::timestamp::now_rfc3339();

  std::assert_eq!(ts.len(), 20, "expected YYYY-MM-DDTHH:MM:SSZ, got {ts}");
  std::assert!(ts.ends_with('Z'), "expected a UTC stamp, got {ts}");

  if CLOCKLESS {
    std::assert_eq!(
      ts,
      "1970-01-01T00:00:00Z",
      "a clockless target must report the epoch, not a fabricated time"
    );
  } else {
    std::assert!(
      ts.starts_with("20"),
      "a target with a clock must report a 21st-century year, got {ts}"
    );
  }
}

/// The `visualization` feature exports the live graph in-memory.
///
/// Only the in-memory export is exercised: `save_visualization` writes through `std::fs`, which
/// correctly returns an error rather than panicking under wasm and is not a WASM-supported path.
#[cfg(feature = "surface")]
fn check_visualization() {
  let dot = hexser::HexGraph::current()
    .to_dot()
    .expect("DOT export failed under wasm");

  std::assert!(
    dot.contains("digraph"),
    "DOT export produced no graph body under wasm: {dot}"
  );
}

/// A port the probe's adapter proves it serves (HEXSER-0.7 D4, D5).
pub trait ProbeSinkPort {
  /// Record one fact.
  fn append(&mut self, fact: u32);
}

/// The port node, tied to its trait.
#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = ProbeSinkPort)]
pub struct ProbeSinkPortNode;

/// The adapter, with a compile-time proof that it implements the port's trait.
#[derive(hexser::HexAdapter)]
#[hex(implements(ProbeSinkPortNode))]
pub struct ProbeSink {
  /// What was appended.
  pub facts: std::vec::Vec<u32>,
}

impl ProbeSinkPort for ProbeSink {
  fn append(&mut self, fact: u32) {
    self.facts.push(fact);
  }
}

/// A single-writer actor's node, depending on the port it writes through.
#[derive(hexser::HexActor)]
#[hex(depends_on(ProbeSinkPortNode))]
pub struct ProbeActorNode;

/// A closed refusal with static guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
pub enum ProbeRefusal {
  /// Nothing to record.
  #[hex(next_step = "send a non-zero fact")]
  Zero,
  /// Recorded already.
  Duplicate,
}

/// A Deck-shaped actor (returns its event or a closed refusal) that writes through its port.
struct ProbeActor {
  sink: ProbeSink,
}

impl hexser::application::Actor for ProbeActor {
  type Command = u32;
  type Context<'a> = ();
  type Outcome = std::result::Result<u32, ProbeRefusal>;

  fn handle(&mut self, fact: u32, _context: ()) -> Self::Outcome {
    if fact == 0 {
      return std::result::Result::Err(ProbeRefusal::Zero);
    }
    if self.sink.facts.contains(&fact) {
      return std::result::Result::Err(ProbeRefusal::Duplicate);
    }
    ProbeSinkPort::append(&mut self.sink, fact);
    std::result::Result::Ok(fact)
  }
}

/// A Tables-shaped actor: returns `()` and appends every outcome as a fact.
struct ProbeFacts {
  facts: std::vec::Vec<std::result::Result<u32, ProbeRefusal>>,
}

impl hexser::application::Actor for ProbeFacts {
  type Command = u32;
  type Context<'a> = ();
  type Outcome = ();

  fn handle(&mut self, fact: u32, _context: ()) {
    let outcome = if fact == 0 {
      std::result::Result::Err(ProbeRefusal::Zero)
    } else {
      std::result::Result::Ok(fact)
    };
    self.facts.push(outcome);
  }
}

/// The single writer's door decides under wasm: both shapes, through the trait.
fn check_actor() {
  let mut actor = ProbeActor {
    sink: ProbeSink {
      facts: std::vec::Vec::new(),
    },
  };
  std::assert_eq!(
    hexser::application::Actor::handle(&mut actor, 3, ()),
    std::result::Result::Ok(3)
  );
  std::assert_eq!(
    hexser::application::Actor::handle(&mut actor, 3, ()),
    std::result::Result::Err(ProbeRefusal::Duplicate)
  );
  std::assert_eq!(
    hexser::application::Actor::handle(&mut actor, 0, ()),
    std::result::Result::Err(ProbeRefusal::Zero)
  );
  std::assert_eq!(actor.sink.facts, std::vec![3]);

  let mut facts = ProbeFacts {
    facts: std::vec::Vec::new(),
  };
  hexser::application::Actor::handle(&mut facts, 0, ());
  hexser::application::Actor::handle(&mut facts, 5, ());
  std::assert_eq!(
    facts.facts,
    std::vec![
      std::result::Result::Err(ProbeRefusal::Zero),
      std::result::Result::Ok(5)
    ]
  );
}

/// A refusal's codes, its read-back, and its trip through `Hexserror` and back under wasm.
fn check_vocabulary_and_refusal() {
  std::assert_eq!(
    <ProbeRefusal as hexser::domain::Vocabulary>::CODES,
    &["Zero", "Duplicate"]
  );
  std::assert_eq!(
    ProbeRefusal::from_code("Duplicate"),
    std::option::Option::Some(ProbeRefusal::Duplicate)
  );
  std::assert_eq!(ProbeRefusal::from_code("zero"), std::option::Option::None);

  let err = hexser::Hexserror::refused(&ProbeRefusal::Zero, "nothing to record");
  std::assert!(std::matches!(err, hexser::Hexserror::Domain(_)));
  std::assert_eq!(
    ProbeRefusal::from_code(err.code()),
    std::option::Option::Some(ProbeRefusal::Zero)
  );
  std::assert!(
    std::format!("{err}").contains("nothing to record"),
    "a refused Hexserror lost its words under wasm"
  );
}

/// The proved `Implements` edge and the actor's `Depends` edge are in a fresh graph, and the
/// graph has nothing to say against itself.
fn check_proved_edges() {
  let graph = hexser::registry::ComponentRegistry::build_graph();
  std::assert!(
    hexser::templates::implements_edge_exists::<ProbeSink, ProbeSinkPortNode>(&graph),
    "the proved Implements edge is missing under wasm"
  );
  std::assert!(
    hexser::templates::dependency_edge_exists::<ProbeActorNode, ProbeSinkPortNode>(&graph),
    "the actor's Depends edge is missing under wasm"
  );
  std::assert!(
    !hexser::templates::implements_edge_exists::<ProbeActorNode, ProbeSinkPortNode>(&graph),
    "a Depends edge read as Implements under wasm"
  );
  std::assert_eq!(graph.findings(), std::vec::Vec::new());
}
