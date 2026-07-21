//! Integration tests asserting that registration derives actually populate the architecture
//! graph with the correct layer and role.
//!
//! These types are declared at module scope so their `inventory::submit!` expansions register
//! them globally; the tests then query `HexGraph::current()` (built by iterating the inventory
//! registry) and assert each component is present. This directly guards the PR-1 fixes:
//! HexDirective previously emitted a bare `inventory::submit!` that broke downstream builds,
//! and HexQuery emitted no submission at all so query components were invisible to the graph.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Add graph-registration coverage for HexDomain/HexPort/HexAdapter/HexDirective/HexQuery and #[hex(role)] override.

#![cfg(feature = "macros")]

use hexser::prelude::*;

#[derive(HexDomain)]
struct RegTestDomainEntity {
  _name: String,
}

#[derive(HexPort)]
struct RegTestPort;

#[derive(HexPort)]
#[hex(role = "InputPort")]
struct RegTestInputPort;

#[derive(HexAdapter)]
struct RegTestAdapter;

#[derive(HexDirective)]
struct RegTestDirective {
  _payload: String,
}

#[derive(HexQuery)]
struct RegTestQuery {
  _filter: String,
}

// Generic component: previously the derive emitted `ComponentEntry::new::<Wrapper #ty_generics>()`
// at item scope, producing `cannot find type T in this scope`. The shared codegen now skips the
// inventory submission for generic types while still emitting the Registrable impl, so this
// compiles.
#[derive(HexDomain)]
struct RegTestGeneric<T> {
  _inner: T,
}

/// Locate a registered node whose `type_name` ends with the given short name.
fn find_role(short_name: &str) -> Option<hexser::graph::Role> {
  let graph = hexser::HexGraph::current();
  graph
    .nodes()
    .find(|n| n.type_name().ends_with(short_name))
    .map(|n| n.role())
}

/// why: HexDomain must register its type in the graph at the Domain layer with role Entity;
/// guards the shared registration codegen for the domain derive.
#[test]
fn test_domain_type_registered_as_entity() {
  assert_eq!(
    find_role("RegTestDomainEntity"),
    Some(hexser::graph::Role::Entity)
  );
}

/// why: HexPort must register with the default role Repository when no #[hex(role)] is given.
#[test]
fn test_port_type_registered_as_repository_by_default() {
  assert_eq!(
    find_role("RegTestPort"),
    Some(hexser::graph::Role::Repository)
  );
}

/// why: the #[hex(role = "InputPort")] override must be parsed and applied instead of the
/// hardcoded Repository default (M16 — roles were previously not overridable).
#[test]
fn test_port_role_override_is_applied() {
  assert_eq!(
    find_role("RegTestInputPort"),
    Some(hexser::graph::Role::InputPort)
  );
}

/// why: HexAdapter must register at the Adapter layer with role Adapter.
#[test]
fn test_adapter_type_registered_as_adapter() {
  assert_eq!(
    find_role("RegTestAdapter"),
    Some(hexser::graph::Role::Adapter)
  );
}

/// why: HexDirective must register in the graph (role Directive). Previously its bare
/// `inventory::submit!` path meant the submission did not resolve for downstream crates;
/// this asserts the component actually lands in the graph.
#[test]
fn test_directive_type_registered() {
  assert_eq!(
    find_role("RegTestDirective"),
    Some(hexser::graph::Role::Directive)
  );
}

/// why: HexQuery must register in the graph (role Query). Previously HexQuery emitted no
/// inventory submission, so query components were silently absent (M13).
#[test]
fn test_query_type_registered() {
  assert_eq!(find_role("RegTestQuery"), Some(hexser::graph::Role::Query));
}

/// why: a generic type must derive without the old `cannot find type T in this scope` error;
/// the Registrable impl still works for a concrete instantiation (inventory submission is
/// skipped for generics, so it is intentionally not asserted to be in the graph).
#[test]
fn test_generic_type_derives_and_reports_layer() {
  let info = <RegTestGeneric<u32> as hexser::registry::Registrable>::node_info();
  assert_eq!(info.layer, hexser::graph::Layer::Domain);
  assert_eq!(info.role, hexser::graph::Role::Entity);
}
