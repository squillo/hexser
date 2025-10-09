//! Implementation of #[derive(HexValueItem)] macro.
//!
//! Automatically implements the HexValueItem trait with a default validation
//! that returns Ok(()). Override the validate method in your impl block if
//! custom validation logic is needed.
//!
//! Revision History
//! - 2025-10-09T11:03:00Z @AI: Initial HexValueItem derive implementation.

/// Derive HexValueItem for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let name = &input.ident;
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let expanded = quote::quote! {
      impl #impl_generics hexser::domain::HexValueItem for #name #ty_generics #where_clause {
          fn validate(&self) -> hexser::result::hex_result::HexResult<()> {
              std::result::Result::Ok(())
          }
      }
  };

  proc_macro::TokenStream::from(expanded)
}
