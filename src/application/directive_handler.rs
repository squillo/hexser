//! DirectiveHandler trait for executing directives.
//!
//! Directive handlers receive directives and execute the corresponding business
//! operations. They orchestrate domain objects and ports to fulfill the directive's
//! intent, ensuring validation and business rules are enforced.
//! Handlers may produce events as side effects of directive execution.
//!
//! Revision History
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
mod tests {
    use super::*;

    struct TestDirective {
        value: i32,
    }

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
        let directive = TestDirective { value: 5 };
        assert!(handler.handle(directive).is_ok());
    }
}
