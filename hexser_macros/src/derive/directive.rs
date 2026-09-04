//! Implementation of #[derive(HexDirective)] macro.
//!
//! Implements the Directive trait for command/intent types and registers them in the
//! architecture graph. The role defaults to `Directive` and can be overridden with
//! `#[hex(role = "DirectiveHandler")]` (or any `Role` variant), matching HexPort/HexAdapter.
//!
//! Revision History
//! - 2026-09-04T00:00:00Z @AI: Honour #[hex(role = "...")] (all five registration derives now do);
//!   pass the derive name to the shared codegen.
//! - 2026-07-20T00:00:00Z @AI: Fix bare `inventory::submit!` (unresolvable downstream) by using shared codegen with ::hexser::inventory; generics-safe.
//! - 2025-10-02T12:00:00Z @AI: Fix to implement validate method and add inventory submission.
//! - 2025-10-02T00:00:00Z @AI: Initial Directive derive implementation.

/// Derive Directive for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let name = &input.ident;
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let role = crate::common::codegen::role_override(&input.attrs)
    .unwrap_or_else(|| quote::quote!(::hexser::graph::Role::Directive));

  let registration = crate::common::codegen::registrable_and_submit(
    &input,
    "HexDirective",
    quote::quote!(::hexser::graph::Layer::Application),
    role,
  );

  let expanded = quote::quote! {
    impl #impl_generics ::hexser::application::Directive for #name #ty_generics #where_clause {
      fn validate(&self) -> ::hexser::HexResult<()> {
        ::std::result::Result::Ok(())
      }
    }

    #registration
  };

  proc_macro::TokenStream::from(expanded)
}
