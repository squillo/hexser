#!/usr/bin/env bash
#
# End-to-end WASM verification for hexser.
#
# Two gates, in order of strength:
#   1. COMPILE  — every feature combination still builds for both wasm targets.
#   2. EXECUTE  — the wasm-e2e probe actually RUNS under wasmtime on both targets.
#
# Gate 2 is the one that matters. hexser passed gate 1 for five releases while
# `HexGraph::current()` trapped on first touch under wasm32-unknown-unknown, because std's
# `SystemTime::now()` is a hard panic on that target. A build-only check cannot see that.
#
# Usage: scripts/wasm-e2e.sh
#
# Revision History
# - 2026-09-11T00:00:00Z @AI: Initial harness — compile matrix plus wasmtime execution on wasm32-unknown-unknown and wasm32-wasip1.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGETS=(wasm32-unknown-unknown wasm32-wasip1)

# Feature combinations a consumer can select. "" is the crate default (macros + static-di),
# which is what the README recommends for wasm; the rest must at least keep compiling so a
# consumer who enables them gets a build error from their own code, never from hexser.
FEATURE_SETS=("" "serde" "ai" "visualization" "mcp" "async" "container" "full")

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

# wasm32-wasip1 is a command module: it has a real _start, real stdout, and a real clock, so
# the probe runs like a normal binary and reports what it saw.
( cd "$E2E" && cargo build --quiet --target wasm32-wasip1 --bin hexser_wasm_e2e ) \
  || fail "probe does not build for wasm32-wasip1"
wasip1_out="$(wasmtime run "$E2E/target/wasm32-wasip1/debug/hexser_wasm_e2e.wasm")" \
  || fail "probe TRAPPED under wasm32-wasip1 (output above)"
printf '  wasm32-wasip1           %s\n' "$wasip1_out"
grep -q 'clockless=false' <<<"$wasip1_out" \
  || fail "wasm32-wasip1 has a real clock; the probe reported it as clockless: $wasip1_out"

# wasm32-unknown-unknown is a reactor module: no _start and no stdout, so the probe is a
# cdylib export invoked directly. It returns the graph node count; a failed check panics,
# which wasm lowers to an `unreachable` trap and wasmtime surfaces as a non-zero exit.
( cd "$E2E" && cargo build --quiet --target wasm32-unknown-unknown --lib ) \
  || fail "probe does not build for wasm32-unknown-unknown"
unknown_out="$(wasmtime run --invoke probe \
  "$E2E/target/wasm32-unknown-unknown/debug/hexser_wasm_e2e.wasm" 2>/dev/null)" \
  || fail "probe TRAPPED under wasm32-unknown-unknown — rerun without 2>/dev/null for the backtrace"
printf '  wasm32-unknown-unknown  probe() -> %s node(s)\n' "$unknown_out"

# A trap is not the only failure mode: link-time `inventory` registration relying on
# life-before-main could silently produce an empty graph, which would exit 0 with 0 nodes.
[ "$unknown_out" -ge 1 ] 2>/dev/null \
  || fail "inventory registration produced no graph nodes under wasm32-unknown-unknown (got '$unknown_out')"

printf '\n\033[32mPASS\033[0m  hexser builds AND runs on wasm32-unknown-unknown and wasm32-wasip1.\n'
