//! Implementation of #[derive(HexRepository)] macro.
//!
//! `HexRepository` is a *semantic marker*: it documents, at the derive site, that a port is a
//! repository. It intentionally emits no trait impl or registration of its own — pair it with
//! `#[derive(HexPort)]` (whose default role is `Repository`) to register the component. Emitting
//! registration here too would produce a conflicting duplicate `Registrable` impl.
//!
//! The derive still validates its target so that applying it to an unsupported item (e.g. a
//! union) yields a clear error rather than passing silently.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Validate target and document the marker semantics explicitly (was a silent no-op comment).
//! - 2025-10-02T12:00:00Z @AI: Remove Registrable impl to avoid conflict with HexPort.
//! - 2025-10-02T00:00:00Z @AI: Initial Repository derive implementation.

/// Derive HexRepository marker for a port
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  if let Err(e) = crate::common::validation::validate_struct_or_enum(&input) {
    return e.to_compile_error().into();
  }

  // Semantic marker only: registration is performed by HexPort to avoid a duplicate
  // `Registrable` impl. See the module docs.
  proc_macro::TokenStream::new()
}
