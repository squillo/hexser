//! Metadata extraction utilities for hex components.
//!
//! Extracts type information, module paths, and other metadata
//! needed for graph node construction.
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial metadata extraction implementation.

/// Extract metadata from a type for node registration
pub fn extract_type_metadata(input: &syn::DeriveInput) -> TypeMetadata {
  TypeMetadata {
    type_name: input.ident.to_string(),
    module_path: String::from("unknown"),
  }
}

/// Metadata about a type for registration
pub struct TypeMetadata {
  pub type_name: String,
  pub module_path: String,
}
