//! Implementation of #[derive(HexAdapter)] macro.
//!
//! Implements Registrable, marks the type as an `Adapter`, and registers it in the
//! architecture graph. The role defaults to `Adapter` and can be overridden with
//! `#[hex(role = "Mapper")]`.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Parse #[hex(role = "...")] override; use shared codegen; fully-qualified paths; generics-safe.
//! - 2025-10-02T00:00:00Z @AI: Initial HexAdapter derive implementation.

/// Derive HexAdapter for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  let name = &input.ident;
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let role = crate::common::codegen::role_override(&input.attrs)
    .unwrap_or_else(|| quote::quote!(::hexser::graph::Role::Adapter));

  let registration = crate::common::codegen::registrable_and_submit(
    &input,
    quote::quote!(::hexser::graph::Layer::Adapter),
    role,
  );

  let expanded = quote::quote! {
    #registration

    impl #impl_generics ::hexser::adapters::Adapter for #name #ty_generics #where_clause {}
  };

  proc_macro::TokenStream::from(expanded)
}
