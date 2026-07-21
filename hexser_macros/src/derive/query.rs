//! Implementation of #[derive(HexQuery)] macro.
//!
//! Implements Registrable for query types and registers them in the architecture graph
//! with `Role::Query`.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Add the missing inventory submission (query components were absent from the graph); use shared codegen.
//! - 2025-10-02T00:00:00Z @AI: Initial Query derive implementation.

/// Derive Query for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let expanded = crate::common::codegen::registrable_and_submit(
    &input,
    quote::quote!(::hexser::graph::Layer::Application),
    quote::quote!(::hexser::graph::Role::Query),
  );

  proc_macro::TokenStream::from(expanded)
}
