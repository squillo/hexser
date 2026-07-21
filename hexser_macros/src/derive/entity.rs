//! Implementation of #[derive(HexEntity)] macro.
//!
//! Implements the HexEntity trait, using the type of a field named `id` as the entity's
//! `Id` associated type. A struct without an `id` field (or an enum) is a compile error
//! rather than silently defaulting to `String`, so the entity's identity type is always
//! explicit and correct.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Error on missing `id` field / non-struct instead of silently defaulting Id = String; fully-qualified paths.
//! - 2025-10-09T09:43:00Z @AI: Update to implement HexEntity trait.
//! - 2025-10-02T00:00:00Z @AI: Initial Entity derive implementation.

/// Derive HexEntity for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let name = &input.ident;

  let id_type = match &input.data {
    syn::Data::Struct(data) => data
      .fields
      .iter()
      .find(|f| f.ident.as_ref().map(|i| i == "id").unwrap_or(false))
      .map(|f| f.ty.clone()),
    _ => None,
  };

  let id_type = match id_type {
    Some(ty) => ty,
    None => {
      return syn::Error::new_spanned(
        &input,
        "HexEntity requires a struct with a field named `id`; the entity's `Id` associated \
         type is taken from that field. Add an `id` field, or implement `HexEntity` manually \
         to choose a different identity type.",
      )
      .to_compile_error()
      .into();
    }
  };

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let expanded = quote::quote! {
    impl #impl_generics ::hexser::domain::HexEntity for #name #ty_generics #where_clause {
      type Id = #id_type;
    }
  };

  proc_macro::TokenStream::from(expanded)
}
