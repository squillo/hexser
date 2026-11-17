//! Procedural macros for the hex crate.
//!
//! This crate provides derive macros that enable zero-boilerplate hexagonal architecture
//! by automatically implementing registration traits and generating metadata for
//! compile-time graph construction.
//!
//! # Derive Macros
//!
//! - `#[derive(HexDomain)]` - Register a domain-layer type (role defaults to `Entity`;
//!   override with `#[hex(role = "ValueObject")]`)
//! - `#[derive(HexPort)]` - Register a port-layer type (role defaults to `Repository`;
//!   override with `#[hex(role = "InputPort")]`)
//! - `#[derive(HexAdapter)]` - Register an adapter and mark it as an `Adapter`
//! - `#[derive(HexEntity)]` - Implement `HexEntity`, taking `Id` from the `id` field
//! - `#[derive(HexValueItem)]` - Implement `HexValueItem` with default validation
//! - `#[derive(HexAggregate)]` - Implement `Aggregate` with default invariant check
//! - `#[derive(HexDirective)]` - Implement `Directive` and register (role `Directive`;
//!   override with `#[hex(role = "...")]`)
//! - `#[derive(HexQuery)]` - Register a query type (role `Query`; override with
//!   `#[hex(role = "UseCase")]` for a registration-only Application-layer marker)
//! - `#[derive(HexRepository)]` - Semantic marker for repository ports (pair with `HexPort`)
//! - `#[derive(HexActor)]` - Register a single-writer actor's node (Application,
//!   `DirectiveHandler`)
//! - `#[derive(HexVocabulary)]` - Name a closed enum's variants (codes, `from_code`)
//! - `#[derive(HexRefusal)]` - A closed refusal with optional static guidance
//!
//! Every registration derive reads `#[hex(role = "...")]` and `#[hex(depends_on(..))]`;
//! `HexPort` also reads `#[hex(port_trait = path::Trait)]` and `HexAdapter` reads
//! `#[hex(implements(PortNode, ..))]`. An unknown key, or a key the derive does not read, is a
//! compile error.
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
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 — `HexActor`, `HexVocabulary`, `HexRefusal`; every registration derive now reads `#[hex(...)]` strictly, with `depends_on(..)`, and `port_trait`/`implements(..)` on HexPort/HexAdapter.
//! - 2026-11-17T16:20:28Z @AI: HEXSER-0.7 D9 — `#![forbid(unsafe_code)]` at the crate root, pinned by `forbid_unsafe_tests`.
//! - 2026-09-04T00:00:00Z @AI: All five registration derives honour `#[hex(role)]` (HexDomain
//!   declared the attribute and ignored it; HexDirective/HexQuery did not declare it at all), and
//!   a registration derive on a generic type is now a compile error instead of a silent omission.
//! - 2026-07-20T00:00:00Z @AI: Remove unexported/unused/non-functional error macros and their dead codegen; refresh docs to match the shipped derive set.
//! - 2025-10-09T14:14:00Z @AI: Remove Entity derive, expose only HexEntity for clarity.
//! - 2025-10-06T02:00:00Z @AI: Add error construction macros.
//! - 2025-10-02T00:00:00Z @AI: Initial Phase 3 proc macro crate.

#![forbid(unsafe_code)]

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

#[proc_macro_derive(HexDirective, attributes(hex))]
pub fn derive_directive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::directive::derive(input)
}

#[proc_macro_derive(HexQuery, attributes(hex))]
pub fn derive_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::query::derive(input)
}

/// Register a single-writer actor's node: Application layer, role `DirectiveHandler` (PRD
/// HEXSER-0.7 D1). Reads `#[hex(role = "...")]` and `#[hex(depends_on(..))]`.
#[proc_macro_derive(HexActor, attributes(hex))]
pub fn derive_actor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::actor::derive(input)
}

/// Name a closed enum's variants: `::hexser::domain::Vocabulary` plus, on an all-unit enum,
/// `from_code` (PRD HEXSER-0.7 D2). Reads `#[hex(code = "...")]` on a variant.
#[proc_macro_derive(HexVocabulary, attributes(hex))]
pub fn derive_vocabulary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::vocabulary::derive_vocabulary(input)
}

/// A closed refusal: `HexVocabulary` plus `::hexser::error::Refusal`, with optional static
/// guidance per variant (PRD HEXSER-0.7 D3). Reads `#[hex(code, next_step, suggestion)]` on a
/// variant.
#[proc_macro_derive(HexRefusal, attributes(hex))]
pub fn derive_refusal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  crate::derive::vocabulary::derive_refusal(input)
}

#[cfg(test)]
mod forbid_unsafe_tests {
  /// why: the crate promises no `unsafe` (house rule; ISO 27001 A.8.28 evidence row E1 in
  /// PRD HEXSER-0.7 §13). The compiler enforces it only while this attribute stands at the
  /// crate root, so the attribute itself is pinned: deleting it turns this red.
  #[test]
  fn this_crate_forbids_unsafe_code() {
    let source = include_str!("lib.rs");
    std::assert!(
      source
        .lines()
        .any(|line| line.trim() == "#![forbid(unsafe_code)]"),
      "src/lib.rs must carry `#![forbid(unsafe_code)]` at the crate root"
    );
  }
}
