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
//! exactly the kind of trap this crate exists to catch. The wasip1 binary in `main.rs` does the
//! reporting; the cdylib entry point returns a number.
//!
//! Revision History
//! - 2026-09-11T00:00:00Z @AI: Initial probe covering graph construction, metadata clock, static DI, errors, AI timestamps and visualization export.

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
  check_static_di();
  check_errors();
  check_ai_timestamp();
  check_visualization();
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
    graph
      .nodes()
      .any(|n| n.type_name().contains("ProbeEntity")),
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
      created_at, 0,
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
fn check_ai_timestamp() {
  let ts = hexser::ai::timestamp::now_rfc3339();

  std::assert_eq!(ts.len(), 20, "expected YYYY-MM-DDTHH:MM:SSZ, got {ts}");
  std::assert!(ts.ends_with('Z'), "expected a UTC stamp, got {ts}");

  if CLOCKLESS {
    std::assert_eq!(
      ts, "1970-01-01T00:00:00Z",
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
fn check_visualization() {
  let dot = hexser::HexGraph::current()
    .to_dot()
    .expect("DOT export failed under wasm");

  std::assert!(
    dot.contains("digraph"),
    "DOT export produced no graph body under wasm: {dot}"
  );
}

/// Entry point for `wasm32-unknown-unknown`, which has no `_start` to run.
///
/// Invoked as `wasmtime run --invoke probe`. A failed check panics, which wasm lowers to an
/// `unreachable` trap and wasmtime reports as a non-zero exit.
///
/// # Safety
///
/// Takes no arguments and returns a plain `u32`, so there is no pointer or lifetime contract
/// for a caller to violate; `no_mangle` is required only so the export keeps the name `probe`.
#[unsafe(no_mangle)]
pub extern "C" fn probe() -> u32 {
  run_checks()
}
