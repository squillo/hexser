//! Derives the Aggregate trait for domain entities.
//!
//! Generates implementation for marking entities as aggregates
//! with consistency boundary enforcement through invariant checking.
//! Provides default no-op implementation that can be overridden.
//!
//! `HexAggregate` declares `#[hex]` but reads no key of its own. It goes through the one strict
//! reader anyway (PRD HEXSER-0.7 §12 A14): the type-level keys a sibling derive reads (`role`,
//! `depends_on`, `implements`, `port_trait`) are left to that derive, and any other key is a
//! compile error instead of being dropped in silence.
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 A14 — read `#[hex]` through the strict reader: no key owned, sibling type-level keys tolerated, anything else refused.
//! - 2026-07-20T00:00:00Z @AI: Fully-qualify generated paths (::hexser::, ::std::) for hygiene.
//! - 2025-10-02T21:00:00Z @AI: Initial HexAggregate derive macro implementation.

/// Derives Aggregate trait implementation
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  if let Err(error) = crate::common::attrs::parse(&input.attrs).and_then(|attrs| {
    attrs.admit(
      "HexAggregate",
      &[],
      &[
        crate::common::attrs::Key::Role,
        crate::common::attrs::Key::DependsOn,
        crate::common::attrs::Key::Implements,
        crate::common::attrs::Key::PortTrait,
      ],
    )
  }) {
    return error.to_compile_error().into();
  }
  let name = &input.ident;

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let expanded = quote::quote! {
    impl #impl_generics ::hexser::domain::Aggregate for #name #ty_generics #where_clause {
      fn check_invariants(&self) -> ::hexser::HexResult<()> {
        ::std::result::Result::Ok(())
      }
    }
  };

  proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
  use super::*;

  /// why: sanity check that the derive emits non-empty tokens. Ignored because
  /// `proc_macro::TokenStream` only functions inside the compiler; real expansion coverage
  /// lives in the trybuild suite (tests/), which invokes rustc.
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
