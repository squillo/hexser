//! InputPort trait for application entry points.
//!
//! Input ports define the ways external actors can interact with the application.
//! They represent use cases from the perspective of the caller and define
//! what operations are available. Input ports are typically implemented by
//! application services that orchestrate domain logic.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 Q4 — the deprecation note names its removal boundary, 0.8.0 (N_BOOK 3/16/5; deletion waits for a family census reading zero uses).
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D7 — `#[deprecated]`, routed to `Actor` (the same job through `&self`; SB 15/203: route, then unreach, then delete). Unit tests keep exercising it under `#[allow(deprecated)]`.
//! - 2025-10-01T00:00:00Z @AI: Initial InputPort trait definition with generic input/output.

/// Trait for input ports that define application entry points.
///
/// Input ports represent operations that external actors can perform.
/// They accept input and produce output, handling the use case execution.
///
/// # Type Parameters
///
/// * `Input` - The type of input data for this operation
/// * `Output` - The type of output data from this operation
///
/// # Example
///
/// ```rust
/// use hexser::ports::InputPort;
/// use hexser::HexResult;
///
/// struct CreateUserInput {
///     email: String,
///     name: String,
/// }
///
/// struct CreateUserOutput {
///     user_id: String,
/// }
///
/// trait CreateUserPort: InputPort<CreateUserInput, CreateUserOutput> {}
/// ```
#[deprecated(
  since = "0.7.0",
  note = "use `hexser::application::Actor`: one writer through `&mut self`, with its own command, context and outcome. See MIGRATING_0.6_TO_0.7.md. Scheduled for removal in 0.8.0."
)]
pub trait InputPort<Input, Output> {
  /// Execute the operation with the given input.
  ///
  /// Returns the output if successful, or an error if the operation fails.
  fn execute(&self, input: Input) -> crate::result::hex_result::HexResult<Output>;
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
  use super::*;

  struct TestInput {
    value: i32,
  }

  struct TestOutput {
    result: i32,
  }

  struct TestPort;

  impl InputPort<TestInput, TestOutput> for TestPort {
    fn execute(&self, input: TestInput) -> crate::result::hex_result::HexResult<TestOutput> {
      Result::Ok(TestOutput {
        result: input.value * 2,
      })
    }
  }

  #[test]
  fn test_input_port_execution() {
    let port = TestPort;
    let input = TestInput { value: 5 };
    let output = port.execute(input).unwrap();
    assert_eq!(output.result, 10);
  }
}
