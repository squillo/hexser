//! Implementation of #[derive(HexQuery)] macro.
//!
//! Registers a type in the architecture graph at the Application layer, and emits no trait
//! impl of its own. The role defaults to `Query` and can be overridden with
//! `#[hex(role = "UseCase")]` (or any `Role` variant), matching HexPort/HexAdapter — this is
//! the registration-only vehicle for an Application-layer component that is neither a
//! directive nor a query (a use case marker, a directive handler marker).
//!
//! Revision History
//! - 2026-09-04T00:00:00Z @AI: Honour #[hex(role = "...")] (all five registration derives now do);
//!   pass the derive name to the shared codegen.
//! - 2026-07-20T00:00:00Z @AI: Add the missing inventory submission (query components were absent from the graph); use shared codegen.
//! - 2025-10-02T00:00:00Z @AI: Initial Query derive implementation.

/// Derive Query for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let role = crate::common::codegen::role_override(&input.attrs)
    .unwrap_or_else(|| quote::quote!(::hexser::graph::Role::Query));

  let expanded = crate::common::codegen::registrable_and_submit(
    &input,
    "HexQuery",
    quote::quote!(::hexser::graph::Layer::Application),
    role,
  );

  proc_macro::TokenStream::from(expanded)
}
