//! DirectiveHandler trait for executing directives.
//!
//! Directive handlers receive directives and execute the corresponding business
//! operations. They orchestrate domain objects and ports to fulfill the directive's
//! intent, ensuring validation and business rules are enforced.
//! Handlers may produce events as side effects of directive execution.
//!
//! Revision History
//! - 2026-11-17T18:11:31Z @AI: HEXSER-0.7 — clippy --all-targets clean: the test directive is a unit struct; its `value` field was never read.
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 Q4 — the deprecation note names its removal boundary, 0.8.0 (N_BOOK 3/16/5; deletion waits for a family census reading zero uses).
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D7 — `#[deprecated]`, routed to `Actor` (the same job through `&self`; SB 15/203: route, then unreach, then delete). Unit tests keep exercising it under `#[allow(deprecated)]`.
//! - 2025-10-01T00:01:00Z @AI: Renamed from CommandHandler to DirectiveHandler.
//! - 2025-10-01T00:00:00Z @AI: Initial CommandHandler trait definition for command execution.

/// Trait for handlers that execute directives.
///
/// Directive handlers contain the logic to execute directives, coordinating
/// domain objects and infrastructure to modify system state.
///
/// # Type Parameters
///
/// * `D` - The directive type this handler processes (must implement `Directive`)
///
/// # Example
///
/// ```rust
/// use hexser::application::{Directive, DirectiveHandler};
/// use hexser::HexResult;
///
/// struct CreateUserDirective {
///     email: String,
/// }
///
/// impl Directive for CreateUserDirective {
///     fn validate(&self) -> HexResult<()> {
///         Ok(())
///     }
/// }
///
/// struct CreateUserHandler;
///
/// impl DirectiveHandler<CreateUserDirective> for CreateUserHandler {
///     fn handle(&self, directive: CreateUserDirective) -> HexResult<()> {
///         // Execute the directive
///         Ok(())
///     }
/// }
/// ```
#[deprecated(
  since = "0.7.0",
  note = "use `hexser::application::Actor`: one writer through `&mut self`, with its own command, context and outcome. See MIGRATING_0.6_TO_0.7.md. Scheduled for removal in 0.8.0."
)]
pub trait DirectiveHandler<D>
where
  D: crate::application::directive::Directive,
{
  /// Handle the execution of a directive.
  ///
  /// Returns `Ok(())` if the directive was successfully executed, or an error
  /// describing what went wrong.
  fn handle(&self, directive: D) -> crate::result::hex_result::HexResult<()>;
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
  use super::*;

  struct TestDirective;

  impl crate::application::directive::Directive for TestDirective {
    fn validate(&self) -> crate::result::hex_result::HexResult<()> {
      Result::Ok(())
    }
  }

  struct TestHandler;

  impl DirectiveHandler<TestDirective> for TestHandler {
    fn handle(&self, _directive: TestDirective) -> crate::result::hex_result::HexResult<()> {
      Result::Ok(())
    }
  }

  #[test]
  fn test_directive_handler_execution() {
    let handler = TestHandler;
    let directive = TestDirective;
    assert!(handler.handle(directive).is_ok());
  }
}
