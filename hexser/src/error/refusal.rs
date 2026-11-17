//! A refusal: a closed vocabulary that may carry static guidance (PRD HEXSER-0.7 D3).
//!
//! A refusal stays a closed typed value. It crosses into [`Hexserror`](crate::Hexserror) only
//! at an edge, through [`Hexserror::refused`](crate::Hexserror::refused), which builds the
//! EXISTING `Domain` variant (no new variant, so code that matches `Hexserror` exhaustively
//! keeps compiling) and copies the code and any static guidance. The code reads back to the
//! closed value with the vocabulary's `from_code`. Its code is a static identifier that carries
//! no caller data, so it can be logged as is; the words for it are said at the edge.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D3 — initial `Refusal` trait.

/// A refusal. Derive it with `#[derive(hexser::HexRefusal)]`; guidance is optional, written per
/// variant as `#[hex(next_step = "...", suggestion = "...")]`.
pub trait Refusal: crate::domain::vocabulary::Vocabulary {
  /// Static next steps this refusal carries in its type. Empty unless the type keeps them.
  fn next_steps(&self) -> &'static [&'static str] {
    &[]
  }

  /// Static suggestions this refusal carries in its type. Empty unless the type keeps them.
  fn suggestions(&self) -> &'static [&'static str] {
    &[]
  }
}
