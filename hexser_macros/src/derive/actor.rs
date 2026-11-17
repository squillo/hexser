//! Implementation of `#[derive(HexActor)]` (PRD HEXSER-0.7 D1).
//!
//! Registers the node that stands for a single-writer actor: Application layer, with the
//! existing `DirectiveHandler` role (no new `Role` variant), overridable with
//! `#[hex(role = "...")]`, and `#[hex(depends_on(..))]` for the ports the actor holds. It emits
//! no `Actor` impl: the actor's command, context and outcome are the actor's own (D1). A generic
//! actor (`DeckActor<S>`) registers a non-generic marker, as every registration derive requires.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D1 — initial derive, through `codegen::registration`.

/// Derive HexActor for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  let expanded = crate::common::codegen::registration(
    &input,
    "HexActor",
    quote::quote!(::hexser::graph::Layer::Application),
    quote::quote!(::hexser::graph::Role::DirectiveHandler),
    &[
      crate::common::attrs::Key::Role,
      crate::common::attrs::Key::DependsOn,
    ],
  );

  proc_macro::TokenStream::from(expanded)
}
