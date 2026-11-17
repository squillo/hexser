//! Best-effort wall-clock reads that never panic, on every target hexser compiles for.
//!
//! `std::time::SystemTime::now()` is not a total function. On `wasm32-unknown-unknown` std
//! routes time through its `unsupported` platform layer, where `now()` is a hard
//! `panic!("time not implemented on this platform")`. hexser stamps `GraphMetadata::created_at`
//! on the universal graph-construction path, so that trap fired on the first touch of
//! `HexGraph::current()` in a browser — in code that had compiled clean, which is why a
//! build-only check never caught it.
//!
//! Every hexser timestamp is informational metadata, never a correctness input, so the honest
//! answer on a clockless target is "unknown" (0) rather than a trap that takes the module down.
//! WASI (`wasm32-wasip1`) has a real clock and takes the normal path, as does every native
//! target.
//!
//! Reading a real browser clock would mean `js-sys`/`wasm-bindgen`, which would cost the
//! dependency-free WASM story documented in the README for one metadata field. Callers that
//! need a true browser timestamp should stamp it themselves at the edge.
//!
//! Revision History
//! - 2026-09-11T00:00:00Z @AI: Initial clockless-target guard — single site for the `SystemTime::now()` panic on wasm32-unknown-unknown, shared by graph metadata and the AI RFC3339 formatter.

/// Seconds since the Unix epoch, best-effort.
///
/// Returns 0 rather than panicking in both degenerate cases: a clock set before the epoch, and
/// a target with no clock at all.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) fn unix_secs() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map(|d| d.as_secs())
    .unwrap_or(0)
}

/// Seconds since the Unix epoch on a target whose std has no clock: always 0.
///
/// `wasm32-unknown-unknown` (the browser target) has no host clock behind std, and its
/// `SystemTime::now()` panics rather than returning an error, so the call is compiled out
/// entirely instead of guarded at runtime.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn unix_secs() -> u64 {
  0
}

#[cfg(test)]
mod tests {
  /// why: the whole point of this module is that the call is total — it must return on every
  /// target rather than trapping. On a target with a clock the value must also be a plausible
  /// present-day timestamp (> 2023-11-14), proving the guard did not swallow a real clock.
  #[test]
  fn test_unix_secs_is_total_and_plausible() {
    let secs = super::unix_secs();

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    std::assert_eq!(secs, 0, "a clockless target must report 0, not trap");

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    std::assert!(
      secs > 1_700_000_000,
      "a target with a clock must report a real present-day timestamp, got {secs}"
    );
  }
}
