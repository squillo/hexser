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
//! - 2026-09-04T00:00:00Z @AI: A registration derive on a GENERIC type now refuses with a
//!   compile error naming the marker-struct remedy, instead of silently discarding the
//!   inventory submission (the type implemented `Registrable`, answered `node_info()`, and was
//!   never in the graph). `registrable_and_submit` takes the derive's name for that message.
//! - 2026-07-20T00:00:00Z @AI: Add #[hex(role = "...")] override parsing so component roles are not hardcoded.
//! - 2026-07-20T00:00:00Z @AI: Extract shared Registrable+inventory codegen; fully-qualified paths; skip inventory submission for generic types.

/// Parse an optional `#[hex(role = "VariantName")]` override from a derive input's attributes.
///
/// Returns the tokens `::hexser::graph::Role::VariantName` when present, or `None` when no
/// `role` override is supplied (the derive then applies its default). An unknown variant name
/// produces a normal compile error at the `Role::` path, pointing at the user's attribute.
///
/// Every registration derive calls this, so `#[hex(role = "...")]` means the same thing on all
/// five of them; only the default differs. A derive that declares `attributes(hex)` and does
/// NOT call this would accept the attribute and register the wrong role in silence.
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

/// Generate the `Registrable` implementation and the inventory submission for a component
/// derive, or a compile error when the target is generic.
///
/// `derive_name` is the derive's user-facing name (e.g. `"HexDomain"`); it appears in the
/// generic-target error so the message names the derive the user actually wrote.
/// `layer` and `role` are token streams naming the `::hexser::graph::Layer` and
/// `::hexser::graph::Role` variants for this component kind.
///
/// # Generics
///
/// A generic target is REFUSED, not silently skipped. The submission expands at item scope,
/// where a type's parameters are not in scope, and `ComponentEntry::new::<T>()` needs one
/// concrete type — `::std::any::type_name::<Self>()` on a generic names a monomorphization
/// chosen by a consumer crate, so there is no single honest node for the type. Answering that
/// by dropping the submission (the previous behavior) left the type implementing `Registrable`
/// and answering `node_info()` while being absent from the graph, with no error and no
/// warning: a computed fact discarded in silence. The derive now says so, and names the
/// remedy — a non-generic marker struct — which also matches the polarity of hand
/// registration, where `ComponentEntry::new::<T>()` on a generic is already a compile error.
///
/// The `Registrable` impl is still emitted alongside the error so a consumer sees ONE error
/// about the real problem rather than a cascade of "trait not implemented" errors at every
/// use site.
pub fn registrable_and_submit(
  input: &syn::DeriveInput,
  derive_name: &str,
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

  let submission = if is_generic {
    let message = format!(
      "`#[derive({derive_name})]` cannot register a generic type in the architecture graph, \
       and will not skip it in silence: `inventory` submits one entry per component and \
       `type_name::<Self>()` on a generic names a monomorphization chosen by a consumer crate, \
       so there is no single honest node for this type (a lifetime or const parameter is \
       likewise not in scope where the submission expands). Derive on a NON-GENERIC marker \
       struct that stands for the component instead, e.g. `#[derive({derive_name})] struct \
       {name}Component;`, and leave the generic type underived; or hand-write `impl \
       hexser::registry::Registrable` on the generic and submit the instantiation you mean. \
       This previously compiled: the type implemented `Registrable`, answered `node_info()`, \
       and was never in the graph."
    );
    syn::Error::new_spanned(input, message).to_compile_error()
  } else {
    quote::quote! {
      ::hexser::inventory::submit! {
        ::hexser::registry::ComponentEntry::new::<#name>()
      }
    }
  };

  quote::quote! {
    #registrable
    #submission
  }
}
