//! Derives the Aggregate trait for domain entities.
//!
//! Generates implementation for marking entities as aggregates
//! with consistency boundary enforcement through invariant checking.
//! Provides default no-op implementation that can be overridden.
//!
//! Revision History
//! - 2025-10-02T21:00:00Z @AI: Initial HexAggregate derive macro implementation.

/// Derives Aggregate trait implementation
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let expanded = quote::quote! {
      impl #impl_generics hexser::domain::Aggregate for #name #ty_generics #where_clause {
          fn check_invariants(&self) -> hexser::HexResult<()> {
              Ok(())
          }
      }
  };

  proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn test_derive_compiles() {
    let input = quote::quote! {
        struct TestAggregate {
            id: String,
            value: i32,
        }
    };

    let result = derive(input.into());
    assert!(!result.is_empty());
  }
}
