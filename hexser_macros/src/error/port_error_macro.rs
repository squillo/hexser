//! Port error construction macro with automatic source location.
//!
//! Provides `hex_port_error!` macro for creating port errors with
//! automatic file, line, and column capture. Supports optional next steps
//! and suggestions through a fluent builder-like syntax.
//!
//! Revision History
//! - 2025-10-06T03:00:00Z @AI: Initial port error macro implementation.

/// Generate port error macro implementation
pub fn generate_port_error_macro() -> proc_macro::TokenStream {
    quote::quote! {
        #[macro_export]
        macro_rules! hex_port_error {
            ($code:expr, $message:expr) => {
                $crate::error::port_error::PortError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?]) => {
                $crate::error::port_error::PortError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_next_step($step))*
            };
            ($code:expr, $message:expr, suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::port_error::PortError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
                    $(.with_suggestion($suggestion))*
            };
            ($code:expr, $message:expr, next_steps: [$($step:expr),* $(,)?], suggestions: [$($suggestion:expr),* $(,)?]) => {
                $crate::error::port_error::PortError::new($code, $message)
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
