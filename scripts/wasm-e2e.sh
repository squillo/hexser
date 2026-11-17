#!/usr/bin/env bash
#
# End-to-end WASM verification for hexser.
#
# Two gates, in order of strength:
#   1. COMPILE  — every feature combination still builds for both wasm targets.
#   2. EXECUTE  — the wasm-e2e probe actually RUNS under wasmtime on both targets, with its
#                default surface and macros-only.
#
# Gate 2 is the one that matters. hexser passed gate 1 for five releases while
# `HexGraph::current()` trapped on first touch under wasm32-unknown-unknown, because std's
# `SystemTime::now()` is a hard panic on that target. A build-only check cannot see that.
#
# Usage: scripts/wasm-e2e.sh
#
# Revision History
# - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 — wasm32-unknown-unknown invokes the binary's exported `main` (no `#[unsafe(no_mangle)]` export); the non-empty-graph guard is the probe's own assertion.
# - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D10 — execute the probe twice per target (surface, macros-only); add the macros-only set (`--no-default-features --features macros`), the pin every editor core takes; the probe now also executes the 0.7 doors (wasm-e2e/src/lib.rs).
# - 2026-09-11T00:00:00Z @AI: Initial harness — compile matrix plus wasmtime execution on wasm32-unknown-unknown and wasm32-wasip1.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGETS=(wasm32-unknown-unknown wasm32-wasip1)

# Feature combinations a consumer can select. "" is the crate default (macros + static-di),
# which is what the README recommends for wasm; the rest must at least keep compiling so a
# consumer who enables them gets a build error from their own code, never from hexser.
# "macros" alone is the pin PRD-847 gives editor_host_ports (`features = ["macros"]`, no defaults).
FEATURE_SETS=("" "macros" "serde" "ai" "visualization" "mcp" "async" "container" "full")

fail() { printf '\n\033[31mFAIL\033[0m  %s\n' "$*" >&2; exit 1; }
step() { printf '\n\033[1m==> %s\033[0m\n' "$*"; }

# --- Prerequisites -----------------------------------------------------------------------

command -v wasmtime >/dev/null 2>&1 \
  || fail "wasmtime not found. Install it (https://wasmtime.dev) — the execution gate needs a wasm runtime."

for target in "${TARGETS[@]}"; do
  rustup target list --installed | grep -qx "$target" \
    || fail "rust target $target not installed. Run: rustup target add $target"
done

# --- Gate 1: compile matrix --------------------------------------------------------------

step "Compile matrix (${#TARGETS[@]} targets x ${#FEATURE_SETS[@]} feature sets)"
for target in "${TARGETS[@]}"; do
  for features in "${FEATURE_SETS[@]}"; do
    label="${features:-default}"
    if [ -z "$features" ]; then
      args=(--lib)
    else
      args=(--lib --no-default-features --features "macros,$features")
    fi
    ( cd "$ROOT" && cargo build --quiet -p hexser --target "$target" "${args[@]}" ) \
      || fail "hexser does not build for $target with features [$label]"
    printf '  ok  %-24s %s\n' "$label" "$target"
  done
done

# --- Gate 2: execute under wasmtime ------------------------------------------------------

step "Execute probe under wasmtime"
E2E="$ROOT/wasm-e2e"

# Each target runs the probe twice: with the default `surface` feature (static-di, serde, ai,
# visualization), and macros-only (`--no-default-features`), the pin an editor core takes. The
# 0.7 doors (Actor, vocabularies, refusals, port_trait, Implements edges, findings) run in both.
PROBE_SETS=("surface" "macros-only")
probe_args() { if [ "$1" = "macros-only" ]; then printf '%s' "--no-default-features"; fi; }

for set in "${PROBE_SETS[@]}"; do
  extra="$(probe_args "$set")"
  want_surface=true
  [ "$set" = "macros-only" ] && want_surface=false

  # wasm32-wasip1 is a command module: it has a real _start, real stdout, and a real clock, so
  # the probe runs like a normal binary and reports what it saw.
  ( cd "$E2E" && cargo build --quiet --target wasm32-wasip1 --bin hexser_wasm_e2e $extra ) \
    || fail "probe [$set] does not build for wasm32-wasip1"
  wasip1_out="$(wasmtime run "$E2E/target/wasm32-wasip1/debug/hexser_wasm_e2e.wasm")" \
    || fail "probe [$set] TRAPPED under wasm32-wasip1 (output above)"
  printf '  %-12s wasm32-wasip1           %s\n' "$set" "$wasip1_out"
  grep -q 'clockless=false' <<<"$wasip1_out" \
    || fail "wasm32-wasip1 has a real clock; the probe reported it as clockless: $wasip1_out"
  grep -q "surface=$want_surface" <<<"$wasip1_out" \
    || fail "probe [$set] ran with the wrong feature set: $wasip1_out"

  # wasm32-unknown-unknown has no _start and no stdout. The same binary is built, and the
  # linker exports its `main` as the module's `main(argc, argv)`, invoked directly with no
  # `unsafe` export. A failed check panics, which wasm lowers to an `unreachable` trap and
  # wasmtime surfaces as a non-zero exit. An empty graph is one of those checks:
  # `check_graph_registration` asserts at least one node and the derived ProbeEntity, so
  # link-time `inventory` registration that silently produced nothing traps here.
  ( cd "$E2E" && cargo build --quiet --target wasm32-unknown-unknown --bin hexser_wasm_e2e $extra ) \
    || fail "probe [$set] does not build for wasm32-unknown-unknown"
  unknown_out="$(wasmtime run --invoke main \
    "$E2E/target/wasm32-unknown-unknown/debug/hexser_wasm_e2e.wasm" 0 0 2>/dev/null)" \
    || fail "probe [$set] TRAPPED under wasm32-unknown-unknown — rerun without 2>/dev/null for the backtrace"
  [ "$unknown_out" = "0" ] \
    || fail "probe [$set] main() returned '$unknown_out' under wasm32-unknown-unknown, not 0"
  printf '  %-12s wasm32-unknown-unknown  main() -> 0, every check passed\n' "$set"
done

printf '\n\033[32mPASS\033[0m  hexser builds AND runs on wasm32-unknown-unknown and wasm32-wasip1.\n'
