//! The four doors deprecated in 0.7 still compile and still work, under
//! `#[allow(deprecated)]` (PRD HEXSER-0.7 D7, row R28).
//!
//! SB 15/203: route, then unreach, then delete. 0.7 routes and warns; nothing is removed, so a
//! crate that moves its pin to 0.7 keeps building and sees one warning per use, naming the
//! route. Removal is a later version's card. This cell is what turns red if a door is deleted
//! early.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 R28 — initial cell.

#![allow(deprecated)]

struct Rename {
  name: std::string::String,
}

impl hexser::application::Directive for Rename {
  fn validate(&self) -> hexser::HexResult<()> {
    if self.name.is_empty() {
      return std::result::Result::Err(hexser::Hexserror::validation("a name is required"));
    }
    std::result::Result::Ok(())
  }
}

struct Doors {
  seen: std::cell::Cell<usize>,
}

impl hexser::application::DirectiveHandler<Rename> for Doors {
  fn handle(&self, directive: Rename) -> hexser::HexResult<()> {
    hexser::application::Directive::validate(&directive)?;
    self.seen.set(self.seen.get() + directive.name.len());
    std::result::Result::Ok(())
  }
}

impl hexser::ports::UseCase<u32, u32> for Doors {
  fn execute(&self, input: u32) -> hexser::HexResult<u32> {
    std::result::Result::Ok(input + 1)
  }
}

impl hexser::ports::InputPort<u32, u32> for Doors {
  fn execute(&self, input: u32) -> hexser::HexResult<u32> {
    std::result::Result::Ok(input * 2)
  }
}

impl hexser::ports::Query<u32, std::string::String> for Doors {
  fn query(&self, params: u32) -> hexser::HexResult<std::string::String> {
    std::result::Result::Ok(params.to_string())
  }
}

/// why: R28. Each deprecated door compiles and answers as it did in 0.6.1, so a crate that
/// moves to 0.7 is warned, not broken. Deleting any of the four turns this red.
#[test]
fn the_deprecated_doors_still_compile_under_allow_deprecated() {
  let doors = Doors {
    seen: std::cell::Cell::new(0),
  };
  std::assert!(
    hexser::application::DirectiveHandler::handle(
      &doors,
      Rename {
        name: std::string::String::from("Q3")
      }
    )
    .is_ok()
  );
  std::assert!(
    hexser::application::DirectiveHandler::handle(
      &doors,
      Rename {
        name: std::string::String::new()
      }
    )
    .is_err()
  );
  std::assert_eq!(doors.seen.get(), 2);
  std::assert_eq!(
    hexser::ports::UseCase::execute(&doors, 1).ok(),
    std::option::Option::Some(2)
  );
  std::assert_eq!(
    hexser::ports::InputPort::execute(&doors, 2).ok(),
    std::option::Option::Some(4)
  );
  std::assert_eq!(
    hexser::ports::Query::query(&doors, 7).ok(),
    std::option::Option::Some(std::string::String::from("7"))
  );
}
