//! Procedural macros for the hex crate.
//!
//! This crate provides derive macros that enable zero-boilerplate hexagonal architecture
//! by automatically implementing registration traits and generating metadata for
//! compile-time graph construction. Also includes error construction macros with
//! automatic source location capture.
//!
//! # Derive Macros
//!
//! - `#[derive(HexDomain)]` - Mark domain layer types
//! - `#[derive(HexPort)]` - Mark port traits
//! - `#[derive(HexAdapter)]` - Mark adapter implementations
//! - `#[derive(Entity)]` - Implement Entity trait
//! - `#[derive(Repository)]` - Mark repository ports
//!
//! # Error Macros
//!
//! - `hex_domain_error!(code, message)` - Create domain error with source location
//! - `hex_port_error!(code, message)` - Create port error with source location
//! - `hex_adapter_error!(code, message)` - Create adapter error with source location
//!
//! # Example
//!
//! ```rust,ignore
//! use hexer::prelude::*;
//!
//! #[derive(HexDomain, Entity)]
//! struct User {
//!     id: String,
//!     email: String,
//! }
//!
//! let err = hex_domain_error!("E_HEX_001", "Invalid state");
//! ```
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Add error construction macros.
//! - 2025-10-02T00:00:00Z @AI: Initial Phase 3 proc macro crate.

mod common;
mod derive;
mod registration;
mod error;

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

#[proc_macro_derive(HexAggregate, attributes(hex))]
pub fn derive_hex_aggregate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::derive::aggregate::derive(input)
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

#[proc_macro]
pub fn hex_domain_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::error::hex_error_macro::hex_domain_error_impl(input)
}

#[proc_macro]
pub fn hex_port_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::error::hex_error_macro::hex_port_error_impl(input)
}

#[proc_macro]
pub fn hex_adapter_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::error::hex_error_macro::hex_adapter_error_impl(input)
}

