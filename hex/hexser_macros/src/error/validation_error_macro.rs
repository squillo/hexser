//! Validation error construction macro with automatic source location.
//!
//! Provides `hex_validation_error!` macro for creating validation errors with
//! automatic file, line, and column capture. Supports optional field names.
//!
//! Revision History
//! - 2025-10-06T03:00:00Z @AI: Initial validation error macro implementation.

/// Generate validation error macro implementation
pub fn generate_validation_error_macro() -> proc_macro::TokenStream {
    quote::quote! {
        #[macro_export]
        macro_rules! hex_validation_error {
            ($code:expr, $message:expr) => {
                $crate::error::validation_error::ValidationError::new($code, $message)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            };
            ($code:expr, $message:expr, field: $field:expr) => {
                $crate::error::validation_error::ValidationError::new($code, $message)
                    .with_field($field)
                    .with_location($crate::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            };
        }
    }.into()
}
