//! Macro implementations for error construction with source location.
//!
//! Provides `hex_domain_error!`, `hex_port_error!`, and `hex_adapter_error!` macros
//! that automatically capture source location using file!(), line!(), and column!().
//! Significantly reduces boilerplate in error creation.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Initial error macro implementations.

/// Generate hex_domain_error macro
pub fn hex_domain_error_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);

    let expanded = match input {
        syn::Expr::Tuple(tuple) if tuple.elems.len() == 2 => {
            let code = &tuple.elems[0];
            let message = &tuple.elems[1];

            quote::quote! {
                hexser::error::hex_error::Hexserror::domain(#code, #message)
                    .with_location(hexser::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            }
        }
        _ => {
            return syn::Error::new_spanned(
                input,
                "Expected (code, message) tuple"
            ).to_compile_error().into();
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Generate hex_port_error macro
pub fn hex_port_error_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);

    let expanded = match input {
        syn::Expr::Tuple(tuple) if tuple.elems.len() == 2 => {
            let code = &tuple.elems[0];
            let message = &tuple.elems[1];

            quote::quote! {
                hexser::error::hex_error::Hexserror::port(#code, #message)
                    .with_location(hexser::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            }
        }
        _ => {
            return syn::Error::new_spanned(
                input,
                "Expected (code, message) tuple"
            ).to_compile_error().into();
        }
    };

    proc_macro::TokenStream::from(expanded)
}

/// Generate hex_adapter_error macro
pub fn hex_adapter_error_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);

    let expanded = match input {
        syn::Expr::Tuple(tuple) if tuple.elems.len() == 2 => {
            let code = &tuple.elems[0];
            let message = &tuple.elems[1];

            quote::quote! {
                hexser::error::hex_error::Hexserror::adapter(#code, #message)
                    .with_location(hexser::error::source_location::SourceLocation::new(
                        file!(),
                        line!(),
                        column!()
                    ))
            }
        }
        _ => {
            return syn::Error::new_spanned(
                input,
                "Expected (code, message) tuple"
            ).to_compile_error().into();
        }
    };

    proc_macro::TokenStream::from(expanded)
}
