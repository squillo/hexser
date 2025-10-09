//! Implementation of #[derive(HexRepository)] macro.
//!
//! Marks a port as a Repository with appropriate metadata.
//!
//! Revision History
//! - 2025-10-02T12:00:00Z @AI: Remove Registrable impl to avoid conflict with HexPort.
//! - 2025-10-02T00:00:00Z @AI: Initial Repository derive implementation.

/// Derive Repository marker for a port
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let _input = syn::parse_macro_input!(input as syn::DeriveInput);

  // HexRepository is a marker - registration is handled by HexPort
  // No trait implementations to avoid conflicts
  let expanded = quote::quote! {
      // Marker only - HexPort handles registration
  };

  proc_macro::TokenStream::from(expanded)
}
