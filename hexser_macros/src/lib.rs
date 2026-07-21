//! Procedural macros for the hex crate.
//!
//! This crate provides derive macros that enable zero-boilerplate hexagonal architecture
//! by automatically implementing registration traits and generating metadata for
//! compile-time graph construction.
//!
//! # Derive Macros
//!
//! - `#[derive(HexDomain)]` - Register a domain-layer type (role defaults to `Entity`)
//! - `#[derive(HexPort)]` - Register a port-layer type (role defaults to `Repository`;
//!   override with `#[hex(role = "InputPort")]`)
//! - `#[derive(HexAdapter)]` - Register an adapter and mark it as an `Adapter`
//! - `#[derive(HexEntity)]` - Implement `HexEntity`, taking `Id` from the `id` field
//! - `#[derive(HexValueItem)]` - Implement `HexValueItem` with default validation
//! - `#[derive(HexAggregate)]` - Implement `Aggregate` with default invariant check
//! - `#[derive(HexDirective)]` - Implement `Directive` and register (role `Directive`)
//! - `#[derive(HexQuery)]` - Register a query type (role `Query`)
//! - `#[derive(HexRepository)]` - Semantic marker for repository ports (pair with `HexPort`)
//!
//! # Example
//!
//! ```rust,ignore
//! use hexser::prelude::*;
//!
//! #[derive(HexDomain, HexEntity)]
//! struct User {
//!     id: String,
//!     email: String,
//! }
//! ```
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Remove unexported/unused/non-functional error macros and their dead codegen; refresh docs to match the shipped derive set.
//! - 2025-10-09T14:14:00Z @AI: Remove Entity derive, expose only HexEntity for clarity.
//! - 2025-10-06T02:00:00Z @AI: Add error construction macros.
//! - 2025-10-02T00:00:00Z @AI: Initial Phase 3 proc macro crate.

mod common;
mod derive;

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

#[proc_macro_derive(HexEntity)]
pub fn derive_hex_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::entity::derive(input)
}

#[proc_macro_derive(HexValueItem)]
pub fn derive_hex_value_item(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::hex_value_item::derive(input)
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
