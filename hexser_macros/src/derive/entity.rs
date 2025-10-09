//! Implementation of #[derive(Entity)] macro.
//!
//! Automatically implements the Entity trait, detecting the Id type
//! from a field named 'id'.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial Entity derive implementation.

/// Derive Entity for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let name = &input.ident;

  let id_type = match &input.data {
    syn::Data::Struct(data) => data
      .fields
      .iter()
      .find(|f| f.ident.as_ref().map(|i| i == "id").unwrap_or(false))
      .map(|f| &f.ty)
      .cloned(),
    _ => None,
  };

  let id_type = id_type.unwrap_or_else(|| syn::parse_quote!(std::string::String));

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let expanded = quote::quote! {
      impl #impl_generics hexser::domain::Entity for #name #ty_generics #where_clause {
          type Id = #id_type;
      }
  };

  proc_macro::TokenStream::from(expanded)
}
