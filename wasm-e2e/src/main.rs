//! Entry point for `wasm32-wasip1`, where the module is a command with a real `_start`.
//!
//! Runs the same checks as the cdylib export and reports them on stdout, which WASI has and
//! `wasm32-unknown-unknown` does not. Also runs natively (`cargo run`), so a developer can
//! confirm the checks hold on the host before blaming the wasm runtime.
//!
//! Revision History
//! - 2026-09-11T00:00:00Z @AI: Initial WASI/native reporting entry point for the probe.

fn main() {
  let node_count = hexser_wasm_e2e::run_checks();

  std::println!(
    "hexser wasm e2e: OK (nodes={node_count}, clockless={})",
    hexser_wasm_e2e::CLOCKLESS
  );
}
