//! Closed vocabularies (`#[derive(HexVocabulary)]`), refusals (`#[derive(HexRefusal)]`), and a
//! refusal's round trip through `Hexserror` (PRD HEXSER-0.7 D2, D3; rows R13-R16, R18-R20).
//!
//! A command, an event and a refusal are each a closed enum whose identity is its variant's
//! name (PRD §4 item 5). These cells pin the codes, their order, the total read-back, the
//! static guidance, and that a refusal crosses `Hexserror` without a new variant.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 R13-R16, R18-R20 — initial cells, written red.

/// A command vocabulary: unit variants only, so it reads back from a code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexVocabulary)]
enum Ask {
  Open,
  Close,
  Rename,
}

/// An event vocabulary: data-carrying variants still have codes.
#[derive(Debug, Clone, PartialEq, Eq, hexser::HexVocabulary)]
enum Event {
  Opened { id: u64 },
  Renamed(std::string::String),
  Closed,
}

/// A refusal vocabulary with static guidance on one variant and a code override on another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexRefusal)]
enum EditRefusal {
  #[hex(next_step = "Type a name", suggestion = "A name is 1 to 64 characters")]
  EmptyName,
  #[hex(code = "E_NAME_TOO_LONG")]
  NameTooLong,
  Closed,
}

/// A vocabulary its owner may grow: `#[non_exhaustive]` is accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, hexser::HexVocabulary)]
#[non_exhaustive]
enum Growing {
  First,
  Second,
}

/// why: R13. The codes are the variant names in declaration order, for commands, events (with
/// data) and refusals alike: the list a declaration is compared against (PRD-886 T68). A derive
/// that sorted them, or skipped data-carrying arms, turns this red.
#[test]
fn codes_are_the_variant_names_in_declaration_order() {
  std::assert_eq!(
    <Ask as hexser::domain::Vocabulary>::CODES,
    &["Open", "Close", "Rename"]
  );
  std::assert_eq!(
    <Event as hexser::domain::Vocabulary>::CODES,
    &["Opened", "Renamed", "Closed"]
  );
  std::assert_eq!(
    <EditRefusal as hexser::domain::Vocabulary>::CODES,
    &["EmptyName", "E_NAME_TOO_LONG", "Closed"]
  );
  std::assert_eq!(hexser::domain::Vocabulary::code(&Ask::Close), "Close");
  std::assert_eq!(
    hexser::domain::Vocabulary::code(&Event::Opened { id: 7 }),
    "Opened"
  );
  std::assert_eq!(
    hexser::domain::Vocabulary::code(&Event::Renamed(std::string::String::from("x"))),
    "Renamed"
  );
  std::assert_eq!(hexser::domain::Vocabulary::code(&Event::Closed), "Closed");
}

/// why: R14. `from_code` is total: every listed code reads back to the value that answers it,
/// and anything else is `None`, never a guessed variant (SB 15/146). A `_ =>` arm returning a
/// variant turns the unknown-code half red.
#[test]
fn from_code_is_total_and_an_unknown_code_is_none() {
  for code in <Ask as hexser::domain::Vocabulary>::CODES {
    let value = Ask::from_code(code).expect("every listed code reads back");
    std::assert_eq!(hexser::domain::Vocabulary::code(&value), *code);
  }
  std::assert_eq!(
    Ask::from_code("Close"),
    std::option::Option::Some(Ask::Close)
  );
  for unknown in ["", "close", "CLOSE", "Closed", "Open ", "Shut"] {
    std::assert_eq!(
      Ask::from_code(unknown),
      std::option::Option::None,
      "{unknown:?}"
    );
  }
}

/// why: R15. `#[hex(code = "...")]` replaces the variant's name as its code everywhere: in
/// `code()`, in `CODES`, and in `from_code`. A derive that ignored it keeps the name.
#[test]
fn a_code_override_wins_and_is_listed_in_codes() {
  std::assert_eq!(
    hexser::domain::Vocabulary::code(&EditRefusal::NameTooLong),
    "E_NAME_TOO_LONG"
  );
  std::assert_eq!(
    EditRefusal::from_code("E_NAME_TOO_LONG"),
    std::option::Option::Some(EditRefusal::NameTooLong)
  );
  std::assert_eq!(
    EditRefusal::from_code("NameTooLong"),
    std::option::Option::None
  );
}

/// why: R16. A `#[non_exhaustive]` enum still has a complete vocabulary inside its own crate
/// (Strataform's `Refusal` is one), so the derive must accept it.
#[test]
fn a_non_exhaustive_enum_derives_its_vocabulary() {
  std::assert_eq!(
    <Growing as hexser::domain::Vocabulary>::CODES,
    &["First", "Second"]
  );
  std::assert_eq!(
    Growing::from_code("Second"),
    std::option::Option::Some(Growing::Second)
  );
}

/// why: R18. Static guidance written on a variant is the refusal's, and a variant without any
/// has none. Dropping `#[hex(next_step)]` or `#[hex(suggestion)]` turns this red.
#[test]
fn a_refusal_carries_its_static_guidance() {
  std::assert_eq!(
    hexser::error::Refusal::next_steps(&EditRefusal::EmptyName),
    &["Type a name"]
  );
  std::assert_eq!(
    hexser::error::Refusal::suggestions(&EditRefusal::EmptyName),
    &["A name is 1 to 64 characters"]
  );
  std::assert!(hexser::error::Refusal::next_steps(&EditRefusal::Closed).is_empty());
  std::assert!(hexser::error::Refusal::suggestions(&EditRefusal::NameTooLong).is_empty());
}

/// why: R19. A refusal crosses `Hexserror` as the EXISTING `Domain` variant, carrying its code
/// and static guidance, and its code reads back to the closed value. The must-fail arm: another
/// refusal's code does not read back as this one.
#[test]
fn a_refused_hexserror_is_the_domain_variant_and_its_code_reads_back_to_the_refusal() {
  let err = hexser::Hexserror::refused(&EditRefusal::EmptyName, "The deck needs a name");
  let domain = match &err {
    hexser::Hexserror::Domain(domain) => domain,
    other => std::panic!("a refusal must be the Domain variant, got {other:?}"),
  };
  std::assert_eq!(hexser::error::RichError::code(domain.as_ref()), "EmptyName");
  std::assert_eq!(
    hexser::error::RichError::message(domain.as_ref()),
    "The deck needs a name"
  );
  std::assert_eq!(
    hexser::error::RichError::next_steps(domain.as_ref()),
    &[std::string::String::from("Type a name")]
  );
  std::assert_eq!(
    hexser::error::RichError::suggestions(domain.as_ref()),
    &[std::string::String::from("A name is 1 to 64 characters")]
  );

  std::assert_eq!(err.code(), "EmptyName");
  std::assert_eq!(
    EditRefusal::from_code(err.code()),
    std::option::Option::Some(EditRefusal::EmptyName)
  );

  let other = hexser::Hexserror::refused(&EditRefusal::NameTooLong, "Too long");
  std::assert_eq!(
    EditRefusal::from_code(other.code()),
    std::option::Option::Some(EditRefusal::NameTooLong)
  );
  std::assert_ne!(
    EditRefusal::from_code(other.code()),
    std::option::Option::Some(EditRefusal::EmptyName)
  );
}

/// why: R20. `Hexserror` keeps exactly its six variants and stays exhaustively matchable from
/// another crate, as Flo matches it (nimble_ui.rs:242-251). Adding a variant, or marking the
/// enum `#[non_exhaustive]`, makes this match fail to compile.
#[test]
fn hexserror_still_has_exactly_six_variants() {
  fn arm(err: &hexser::Hexserror) -> u8 {
    match err {
      hexser::Hexserror::Domain(_) => 0,
      hexser::Hexserror::Port(_) => 1,
      hexser::Hexserror::Adapter(_) => 2,
      hexser::Hexserror::Validation(_) => 3,
      hexser::Hexserror::NotFound(_) => 4,
      hexser::Hexserror::Conflict(_) => 5,
    }
  }
  std::assert_eq!(arm(&hexser::Hexserror::domain("D", "d")), 0);
  std::assert_eq!(arm(&hexser::Hexserror::port("P", "p")), 1);
  std::assert_eq!(arm(&hexser::Hexserror::adapter("A", "a")), 2);
  std::assert_eq!(arm(&hexser::Hexserror::validation("v")), 3);
  std::assert_eq!(arm(&hexser::Hexserror::not_found("R", "1")), 4);
  std::assert_eq!(arm(&hexser::Hexserror::conflict("c")), 5);
  std::assert_eq!(hexser::Hexserror::port("P_CODE", "p").code(), "P_CODE");
}
