//! Validation utilities for derive macro inputs.
//!
//! Validates that derive macros are applied to appropriate targets
//! (structs, enums) and provides helpful error messages.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Remove dead validate_trait helper (derives never apply to traits).
//! - 2025-10-02T00:00:00Z @AI: Initial validation implementation.

/// Validate that input is a struct or enum
pub fn validate_struct_or_enum(input: &syn::DeriveInput) -> Result<(), syn::Error> {
  match &input.data {
    syn::Data::Struct(_) | syn::Data::Enum(_) => Ok(()),
    syn::Data::Union(_) => Err(syn::Error::new_spanned(
      input,
      "hex derive macros cannot be applied to unions",
    )),
  }
}
