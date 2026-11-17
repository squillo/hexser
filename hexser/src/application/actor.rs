//! The single writer's door (PRD HEXSER-0.7 D1).
//!
//! An actor owns its state and is the one place that state changes (SB 15/209: the one writer
//! is the one gate; N_BOOK 3/6/6: commands are proposals, the actor decides the facts, and it
//! commits before the next command). Rust says "one writer, one command at a time" with
//! `&mut self`, so the door needs no lock (SB 15/10).
//!
//! The family's writers share that receiver and a command consumed by value, and nothing else
//! (PRD §4): Deck returns `Result<Event, Refusal>`, its editor returns a view, Tables returns
//! `()` and appends facts, Flo borrows a per-call context. So the context and the outcome are
//! associated types (SB 15/183 :55-68), and the trait forces no `Result`, no `Hexserror`, no
//! event sink, and no `Send`/`Sync` bound: the host decides those.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D1 — initial `Actor` trait.

/// The single writer's door: one command in, by value; what the actor decided out.
///
/// ```rust
/// struct Counter { total: u64 }
///
/// impl hexser::application::Actor for Counter {
///   type Command = u64;
///   type Context<'a> = ();
///   type Outcome = Result<u64, &'static str>;
///
///   fn handle(&mut self, add: u64, _context: ()) -> Self::Outcome {
///     self.total = self.total.checked_add(add).ok_or("overflow")?;
///     Ok(self.total)
///   }
/// }
///
/// let mut counter = Counter { total: 0 };
/// assert_eq!(hexser::application::Actor::handle(&mut counter, 2, ()), Ok(2));
/// ```
pub trait Actor {
  /// The proposal, consumed by value (N_BOOK 3/6/6 :19-22).
  type Command;
  /// What the writer borrows for one call and does not own; `()` when it holds its ports.
  type Context<'a>;
  /// What the actor decided: an event, facts, a view, a refusal, or a pair of them.
  type Outcome;

  /// Decide one command. `&mut self` makes this the one write site: no second command can
  /// begin until this one has committed.
  fn handle(&mut self, command: Self::Command, context: Self::Context<'_>) -> Self::Outcome;
}
