//! Implementation of #[derive(HexDomain)] macro.
//!
//! Registers a domain-layer type in the architecture graph (`Registrable` impl + inventory
//! submission). The role defaults to `Entity` and can be overridden with
//! `#[hex(role = "ValueObject")]` (or any `Role` variant) — a domain layer is not made only
//! of entities, and a value object registered as an `Entity` is a graph that lies.
//!
//! Revision History
//! - 2026-09-04T00:00:00Z @AI: Honour #[hex(role = "...")]. The derive declared attributes(hex)
//!   and never read it, so `#[hex(role = "ValueObject")]` compiled clean and registered
//!   Role::Entity; hex_port.rs / hex_adapter.rs already did this correctly.
//! - 2026-07-20T00:00:00Z @AI: Use shared registrable_and_submit codegen (fully-qualified paths, generics-safe).
//! - 2025-10-02T00:00:00Z @AI: Initial HexDomain derive implementation.

/// Derive HexDomain for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  let role = crate::common::codegen::role_override(&input.attrs)
    .unwrap_or_else(|| quote::quote!(::hexser::graph::Role::Entity));

  let expanded = crate::common::codegen::registrable_and_submit(
    &input,
    "HexDomain",
    quote::quote!(::hexser::graph::Layer::Domain),
    role,
  );

  proc_macro::TokenStream::from(expanded)
}
