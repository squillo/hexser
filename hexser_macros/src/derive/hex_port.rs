//! Implementation of #[derive(HexPort)] macro.
//!
//! Marks a port type (struct/enum) as a port-layer component and registers it in the
//! architecture graph. The port's `Role` defaults to `Repository` but can be overridden
//! with `#[hex(role = "InputPort")]` (or any `Role` variant).
//!
//! Note: derives apply to structs/enums, not traits. To register a trait-based port, apply
//! this derive to a marker struct representing the port.
//!
//! Revision History
//! - 2026-09-04T00:00:00Z @AI: Pass the derive name to the shared codegen (generic-target compile error names the derive).
//! - 2026-07-20T00:00:00Z @AI: Parse #[hex(role = "...")] override; use shared codegen; validate target; fully-qualified paths.
//! - 2025-10-02T00:00:00Z @AI: Initial HexPort derive implementation.

/// Derive HexPort for a type
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  let role = crate::common::codegen::role_override(&input.attrs)
    .unwrap_or_else(|| quote::quote!(::hexser::graph::Role::Repository));

  let expanded = crate::common::codegen::registrable_and_submit(
    &input,
    "HexPort",
    quote::quote!(::hexser::graph::Layer::Port),
    role,
  );

  proc_macro::TokenStream::from(expanded)
}
