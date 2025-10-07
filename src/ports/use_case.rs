//! UseCase trait for business operation definitions.
//!
//! Use cases represent specific business operations or user goals within
//! the application. They define what the system can do from a user's perspective
//! and orchestrate domain objects and ports to accomplish their goal.
//! Use cases are similar to InputPort but emphasize the business operation aspect.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial UseCase trait definition as business operation contract.

/// Trait for use cases that represent business operations.
///
/// Use cases encapsulate the application's business logic and orchestrate
/// domain objects to fulfill specific user goals.
///
/// # Type Parameters
///
/// * `Input` - The input data required to execute the use case
/// * `Output` - The output data produced by the use case
///
/// # Example
///
/// ```rust
/// use hexser::ports::UseCase;
/// use hexser::HexResult;
///
/// struct RegisterUserInput {
///     email: String,
///     password: String,
/// }
///
/// struct RegisterUserOutput {
///     user_id: String,
/// }
///
/// trait RegisterUser: UseCase<RegisterUserInput, RegisterUserOutput> {}
/// ```
pub trait UseCase<Input, Output> {
    /// Execute the use case with the given input.
    ///
    /// Returns the output if successful, or an error describing what went wrong.
    fn execute(&self, input: Input) -> crate::result::hex_result::HexResult<Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUseCaseInput {
        value: i32,
    }

    struct TestUseCaseOutput {
        doubled: i32,
    }

    struct TestUseCase;

    impl UseCase<TestUseCaseInput, TestUseCaseOutput> for TestUseCase {
        fn execute(&self, input: TestUseCaseInput) -> crate::result::hex_result::HexResult<TestUseCaseOutput> {
            Result::Ok(TestUseCaseOutput {
                doubled: input.value * 2,
            })
        }
    }

    #[test]
    fn test_use_case_execution() {
        let use_case = TestUseCase;
        let input = TestUseCaseInput { value: 7 };
        let output = use_case.execute(input).unwrap();
        assert_eq!(output.doubled, 14);
    }
}
