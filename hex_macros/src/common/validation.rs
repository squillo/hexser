//! Validation utilities for derive macro inputs.
//!
//! Validates that derive macros are applied to appropriate targets
//! (structs, enums, traits) and provides helpful error messages.
//!
//! Revision History
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

/// Validate that input is a trait
pub fn validate_trait(input: &syn::DeriveInput) -> Result<(), syn::Error> {
    Err(syn::Error::new_spanned(
        input,
        "HexPort should be used with traits (this limitation will be addressed)",
    ))
}
