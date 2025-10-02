//! Procedural macros for the hex crate.
//!
//! This crate provides derive macros that enable zero-boilerplate hexagonal architecture
//! by automatically implementing registration traits and generating metadata for
//! compile-time graph construction.
//!
//! # Derive Macros
//!
//! - `#[derive(HexDomain)]` - Mark domain layer types
//! - `#[derive(HexPort)]` - Mark port traits
//! - `#[derive(HexAdapter)]` - Mark adapter implementations
//! - `#[derive(Entity)]` - Implement Entity trait
//! - `#[derive(Repository)]` - Mark repository ports
//!
//! # Example
//!
//! ```rust,ignore
//! use hex::prelude::*;
//!
//! #[derive(HexDomain, Entity)]
//! struct User {
//!     id: String,
//!     email: String,
//! }
//! ```
//!
//! Revision History
//! - 2025-10-02T00:00:00Z @AI: Initial Phase 3 proc macro crate.

mod common;
mod derive;
mod registration;

#[proc_macro_derive(HexDomain, attributes(hex))]
pub fn derive_hex_domain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::hex_domain::derive(input)
}

#[proc_macro_derive(HexPort, attributes(hex))]
pub fn derive_hex_port(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::hex_port::derive(input)
}

#[proc_macro_derive(HexAdapter, attributes(hex))]
pub fn derive_hex_adapter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::hex_adapter::derive(input)
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::entity::derive(input)
}

#[proc_macro_derive(HexRepository)]
pub fn derive_repository(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::repository::derive(input)
}

#[proc_macro_derive(HexDirective)]
pub fn derive_directive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::directive::derive(input)
}

#[proc_macro_derive(HexQuery)]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::query::derive(input)
}
