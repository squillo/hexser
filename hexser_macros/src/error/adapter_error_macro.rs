//! Adapter error construction macro with automatic source location.
//!
//! Provides `hex_adapter_error!` macro for creating adapter errors with
//! automatic file, line, and column capture. Supports optional next steps
//! and suggestions through a fluent builder-like syntax.
//!
//! Revision History
//! - 2025-10-06T03:00:00Z @AI: Initial adapter error macro implementation.

/// Generate adapter error macro implementation
pub fn generate_adapter_error_macro() -> proc_macro::TokenStream {
  quote::quote! {
        #[macro_export]
        macro_rules! hex_adapter_error {
            ($code:expr, $message:expr) => {
                $crate::error::adapter_error::AdapterError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?]) => {
                $crate::error::adapter_error::AdapterError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_next_step($step))*
            };
            ($code:expr, $message:expr, suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::adapter_error::AdapterError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_suggestion($suggestion))*
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?], suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::adapter_error::AdapterError::new($code, $message)
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
