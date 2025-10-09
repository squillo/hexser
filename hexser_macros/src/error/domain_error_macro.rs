//! Domain error construction macro with automatic source location.
//!
//! Provides `hex_domain_error!` macro for creating domain errors with
//! automatic file, line, and column capture. Supports optional next steps
//! and suggestions through a fluent builder-like syntax.
//!
//! Revision History
//! - 2025-10-06T03:00:00Z @AI: Initial domain error macro implementation.

/// Generate domain error macro implementation
pub fn generate_domain_error_macro() -> proc_macro::TokenStream {
  quote::quote! {
        #[macro_export]
        macro_rules! hex_domain_error {
            ($code:expr, $message:expr) => {
                $crate::error::domain_error::DomainError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?]) => {
                $crate::error::domain_error::DomainError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_next_step($step))*
            };
            ($code:expr, $message:expr, suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::domain_error::DomainError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_suggestion($suggestion))*
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?], suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::domain_error::DomainError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_next_step($step))*
                    $(.with_suggestion($suggestion))*
            };
        }
    }.into()
}
