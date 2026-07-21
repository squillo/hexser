//! Implementation of #[derive(HexDomain)] macro.
//!
//! Automatically implements Registrable trait and generates
//! inventory submission for domain layer types.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Use shared registrable_and_submit codegen (fully-qualified paths, generics-safe).
//! - 2025-10-02T00:00:00Z @AI: Initial HexDomain derive implementation.

/// Derive HexDomain for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  let expanded = crate::common::codegen::registrable_and_submit(
    &input,
    quote::quote!(::hexser::graph::Layer::Domain),
    quote::quote!(::hexser::graph::Role::Entity),
  );

  proc_macro::TokenStream::from(expanded)
}
