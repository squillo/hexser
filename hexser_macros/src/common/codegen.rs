//! Shared code generation for registration-based derive macros.
//!
//! Five derives (HexDomain, HexPort, HexAdapter, HexDirective, HexQuery) all emit the same
//! `Registrable` impl plus an `inventory::submit!` of a `ComponentEntry`. Centralizing that
//! codegen here removes the copy-paste that previously let the derives drift apart — most
//! notably HexDirective emitting a bare `inventory::submit!` (unresolvable in downstream
//! crates) and HexQuery omitting the submission entirely. All generated paths are fully
//! qualified (`::hexser::`, `::std::`) for hygiene.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Add #[hex(role = "...")] override parsing so component roles are not hardcoded.
//! - 2026-07-20T00:00:00Z @AI: Extract shared Registrable+inventory codegen; fully-qualified paths; skip inventory submission for generic types.

/// Parse an optional `#[hex(role = "VariantName")]` override from a derive input's attributes.
///
/// Returns the tokens `::hexser::graph::Role::VariantName` when present, or `None` when no
/// `role` override is supplied (the derive then applies its default). An unknown variant name
/// produces a normal compile error at the `Role::` path, pointing at the user's attribute.
pub fn role_override(attrs: &[syn::Attribute]) -> Option<proc_macro2::TokenStream> {
  let mut found = None;
  for attr in attrs {
    if !attr.path().is_ident("hex") {
      continue;
    }
    // Ignore parse errors here: a malformed `#[hex(...)]` simply yields no override rather
    // than aborting the derive, so the default role still applies.
    let _ = attr.parse_nested_meta(|meta| {
      if meta.path.is_ident("role") {
        let value = meta.value()?;
        let lit: syn::LitStr = value.parse()?;
        let ident = syn::Ident::new(&lit.value(), lit.span());
        found = Some(quote::quote!(::hexser::graph::Role::#ident));
      }
      Ok(())
    });
  }
  found
}

/// Generate the `Registrable` implementation and (for non-generic types) the inventory
/// submission for a component derive.
///
/// `layer` and `role` are token streams naming the `::hexser::graph::Layer` and
/// `::hexser::graph::Role` variants for this component kind.
///
/// # Generics
///
/// The `Registrable` impl is emitted for generic types, but the `inventory::submit!` is
/// only emitted for non-generic types: the submission expands at item scope where a type's
/// generic parameters are not in scope, and `ComponentEntry::new::<T>()` requires a single
/// concrete type. Registering a generic component is therefore skipped rather than producing
/// an inscrutable `cannot find type T in this scope` error at the derive site.
pub fn registrable_and_submit(
  input: &syn::DeriveInput,
  layer: proc_macro2::TokenStream,
  role: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let name = &input.ident;
  let is_generic = !input.generics.params.is_empty();

  // `Registrable: 'static`, so a generic impl needs `Self: 'static`. Add that bound (only
  // when the type is generic) so generic components compile; their concrete instantiations
  // must be 'static, which is already true for owned data.
  let generics = {
    let mut g = input.generics.clone();
    if is_generic {
      let (_, ty_generics, _) = input.generics.split_for_impl();
      let self_ty: syn::Type = syn::parse_quote!(#name #ty_generics);
      g.make_where_clause()
        .predicates
        .push(syn::parse_quote!(#self_ty: 'static));
    }
    g
  };
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  let registrable = quote::quote! {
    impl #impl_generics ::hexser::registry::Registrable for #name #ty_generics #where_clause {
      fn node_info() -> ::hexser::registry::NodeInfo {
        ::hexser::registry::NodeInfo {
          layer: #layer,
          role: #role,
          type_name: ::std::any::type_name::<Self>(),
          module_path: ::std::module_path!(),
        }
      }

      fn dependencies() -> ::std::vec::Vec<::hexser::graph::NodeId> {
        ::std::vec::Vec::new()
      }
    }
  };

  let submission = if input.generics.params.is_empty() {
    quote::quote! {
      ::hexser::inventory::submit! {
        ::hexser::registry::ComponentEntry::new::<#name>()
      }
    }
  } else {
    proc_macro2::TokenStream::new()
  };

  quote::quote! {
    #registrable
    #submission
  }
}
