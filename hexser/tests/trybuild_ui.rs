//! Compile-fail (UI) tests for the derive macros.
//!
//! These verify that misusing a derive produces a clear compile error rather than silently
//! doing the wrong thing. Runtime behavior (successful derives, graph registration) is covered
//! by `derive_registration_test.rs` and `macro_tests.rs`; this file covers the failure cases
//! that can only be observed at compile time.
//!
//! ⚠ The `.stderr` files are EXACT compiler output. If a case fails on formatting alone —
//! same message, different caret art or column — regenerate with `TRYBUILD=overwrite cargo
//! test -p hexser --test trybuild_ui` and read the diff before accepting it: a changed
//! MESSAGE is a behaviour change, a changed caret is a toolchain artifact.
//!
//! The expected compiler output lives alongside each `.rs` case as a `.stderr` file. If a
//! future toolchain changes the wording, regenerate with `TRYBUILD=overwrite cargo test`.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 A14 — `aggregate_hex_key.rs`: a stray key on `HexAggregate` is refused.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 cases R10 (an actor through `&`), R12 (HexActor on a generic), R17 (HexVocabulary on a struct), R21 (a port node whose trait does not exist, T67), R22 (an adapter that does not implement its port's trait), R23 (an adapter naming a port node with no `port_trait`), R31 (an unknown `#[hex]` key; a key the derive does not read).
//! - 2026-09-04T00:00:00Z @AI: Second case: a registration derive on a generic type must
//!   error instead of silently dropping the inventory submission.
//! - 2026-07-20T00:00:00Z @AI: Add trybuild UI suite; first case: HexEntity without an `id` field must error.

/// why: each case pins a derive REFUSING bad input. `generic_domain.rs` is the falsifier for
/// the silent-omission defect: `#[derive(HexDomain)]` on a generic used to compile, implement
/// `Registrable`, and never reach the graph. Flip: restore the empty-TokenStream arm in
/// `codegen.rs` and this case compiles, so trybuild fails with "expected test case to fail to
/// compile, but it succeeded".
///
/// why: the HEXSER-0.7 cases pin each door that must not compile: a writer reached through a
/// shared reference, a vocabulary without variants, a port node that outlived its trait, an
/// adapter claiming a port it does not implement or one that cannot be checked, and an
/// attribute key read by nobody, including on `HexAggregate`, which reads no key of its own. Each `.stderr` names the intended error; an unrelated error in
/// its place is a regression the overwrite diff shows.
///
/// why: HexEntity must reject a struct with no `id` field with a clear, actionable error
/// instead of silently defaulting `type Id = String` (M15). A compile-fail test is the only
/// way to assert a derive rejects bad input.
#[cfg(feature = "macros")]
#[test]
fn ui_compile_fail() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/ui/fail/*.rs");
}
