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
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D2/D4/D5 — `registration` is the one entry for every registration derive: it reads `#[hex(...)]` through the strict `attrs` reader (an unknown key or a key the derive does not read is a compile error, where `role_override` ignored it), emits `depends_on` as `Depends` edges with the template's `dependency_id`, emits `implements` as a compile-time proof plus an `ImplementsEntry`, and emits `port_trait` as the hidden bounded `__hexser_port_trait`. `role_override` is retired into it.
//! - 2026-09-04T00:00:00Z @AI: A registration derive on a GENERIC type now refuses with a
//!   compile error naming the marker-struct remedy, instead of silently discarding the
//!   inventory submission (the type implemented `Registrable`, answered `node_info()`, and was
//!   never in the graph). `registrable_and_submit` takes the derive's name for that message.
//! - 2026-07-20T00:00:00Z @AI: Add #[hex(role = "...")] override parsing so component roles are not hardcoded.
//! - 2026-07-20T00:00:00Z @AI: Extract shared Registrable+inventory codegen; fully-qualified paths; skip inventory submission for generic types.

/// The one entry for every registration derive: read `#[hex(...)]` strictly, refuse any key
/// this derive does not read (`owns`), take the role (or `default_role`), and emit the
/// registration with its edges. Returns the compile error instead when the attribute is wrong.
pub fn registration(
  input: &syn::DeriveInput,
  derive_name: &str,
  layer: proc_macro2::TokenStream,
  default_role: proc_macro2::TokenStream,
  owns: &[crate::common::attrs::Key],
) -> proc_macro2::TokenStream {
  let attrs = match crate::common::attrs::parse(&input.attrs)
    .and_then(|attrs| attrs.admit(derive_name, owns, &[]).map(|()| attrs))
  {
    Ok(attrs) => attrs,
    Err(error) => return error.to_compile_error(),
  };
  let role = attrs.role_tokens().unwrap_or(default_role);
  registrable_and_submit(input, derive_name, layer, role, &attrs)
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
  attrs: &crate::common::attrs::HexAttrs,
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

  // `depends_on(..)`: one `Depends` edge per listed type, through the same `dependency_id`
  // the templates use, so a derive and a template say the same edge onto the same node.
  let depends_on = &attrs.depends_on;

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
        ::std::vec![#( ::hexser::templates::dependency_id::<#depends_on>() ),*]
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
    // `implements(PortNode, ..)`: the proof first (this compiles only if `#name` implements
    // the trait the port node named with `port_trait`), then one proved `Implements` edge.
    let claims = attrs.implements.iter().map(|port| {
      let proof = quote::quote_spanned! {syn::spanned::Spanned::span(port)=>
        const _: () = <#port>::__hexser_port_trait::<#name>();
      };
      quote::quote! {
        #proof
        ::hexser::inventory::submit! {
          ::hexser::registry::ImplementsEntry::new::<#name, #port>()
        }
      }
    });
    // `port_trait = Trait`: the port node carries its trait as a bound, so the node does not
    // compile without the trait, and `implements` has something to prove an adapter against.
    let port_trait = attrs.port_trait.as_ref().map(|bound| {
      quote::quote! {
        impl #name {
          /// The port node's trait, as a bound: `implements` proves an adapter against it.
          #[doc(hidden)]
          pub const fn __hexser_port_trait<__HexserImplementor: ?::core::marker::Sized + #bound>() {}
        }
      }
    });
    quote::quote! {
      ::hexser::inventory::submit! {
        ::hexser::registry::ComponentEntry::new::<#name>()
      }
      #( #claims )*
      #port_trait
    }
  };

  quote::quote! {
    #registrable
    #submission
  }
}
