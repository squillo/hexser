//! The probe's one entry point, on every target.
//!
//! On `wasm32-wasip1` the module is a command with a real `_start` and stdout, and the report
//! below is what the script reads. On `wasm32-unknown-unknown` there is no `_start` and no
//! stdout: the linker exports this `main` as the module's `main`, the script invokes it with
//! `wasmtime run --invoke main <module> 0 0`, the report goes nowhere, and a failed check
//! panics, which wasm lowers to an `unreachable` trap and wasmtime reports as a non-zero exit.
//! Also runs natively (`cargo run`), so a developer can confirm the checks hold on the host
//! before blaming the wasm runtime.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 — the one entry point on both wasm targets; the `unsafe` cdylib export is gone.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D10 — report whether the wider `surface` ran.
//! - 2026-09-11T00:00:00Z @AI: Initial WASI/native reporting entry point for the probe.

#![forbid(unsafe_code)]

fn main() {
  let node_count = hexser_wasm_e2e::run_checks();

  std::println!(
    "hexser wasm e2e: OK (nodes={node_count}, clockless={}, surface={})",
    hexser_wasm_e2e::CLOCKLESS,
    hexser_wasm_e2e::SURFACE
  );
}
