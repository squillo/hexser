//! A closed set of named values that crosses a boundary: a command, an event, a refusal
//! (PRD HEXSER-0.7 D2).
//!
//! Every editor in the family names its commands, events and refusals with a closed enum whose
//! identity is the variant's name, and three of them kept that list by hand (Nimble's `ALL`,
//! formula's order cell, Deck's T68 list). `#[derive(HexVocabulary)]` writes it once: the codes
//! in declaration order, the code of a value, and, for an all-unit enum, an inherent
//! `from_code(&str) -> Option<Self>` that answers `None` for an unknown code (SB 15/146: an
//! unknown arm is a value, never a guess). `#[hex(code = "...")]` on a variant keeps a wire code
//! stable across a rename. The words for a value are said at the edge, not here.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D2 — initial `Vocabulary` trait.

/// A closed set of named values. Derive it with `#[derive(hexser::HexVocabulary)]`.
///
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, hexser::HexVocabulary)]
/// enum Ask { Open, Close }
///
/// assert_eq!(<Ask as hexser::domain::Vocabulary>::CODES, &["Open", "Close"]);
/// assert_eq!(hexser::domain::Vocabulary::code(&Ask::Close), "Close");
/// assert_eq!(Ask::from_code("Close"), Some(Ask::Close));
/// assert_eq!(Ask::from_code("close"), None);
/// ```
pub trait Vocabulary: core::fmt::Debug + 'static {
  /// Every code, in declaration order: the list a declaration is compared against.
  const CODES: &'static [&'static str];

  /// This value's stable code: its variant's name unless `#[hex(code = "...")]` says otherwise.
  fn code(&self) -> &'static str;
}
