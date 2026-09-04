//! Integration tests asserting that registration derives actually populate the architecture
//! graph with the correct layer and role.
//!
//! These types are declared at module scope so their `inventory::submit!` expansions register
//! them globally; the tests then query `HexGraph::current()` (built by iterating the inventory
//! registry) and assert each component is present. This directly guards the PR-1 fixes:
//! HexDirective previously emitted a bare `inventory::submit!` that broke downstream builds,
//! and HexQuery emitted no submission at all so query components were invisible to the graph.
//!
//! It also guards the three 2026-09-04 fixes, all of which share one shape — a fact the code
//! had already computed, discarded in silence:
//!
//! 1. `HexDomain` declared `attributes(hex)` and never read it, so `#[hex(role = "…")]`
//!    compiled clean and registered `Role::Entity`.
//! 2. A registration derive on a GENERIC type dropped the submission, leaving a type that
//!    implemented `Registrable`, answered `node_info()`, and was in no graph. That is now a
//!    compile error (`tests/ui/fail/generic_domain.rs`); the two remedies its message names
//!    are pinned below, because an error naming a remedy that does not work is worse than no
//!    error.
//! 3. `hexser::hex_register_component!` (and its `hex_register_domain!` … wrappers) emitted
//!    the `Registrable` impl and NO submission, for every type — hexser's own advertised
//!    non-derive registration path put nothing in the graph.
//!
//! Revision History
//! - 2026-09-04T00:00:00Z @AI: Add the HexDomain #[hex(role)] falsifier, the HexQuery role
//!   override, the template-macro graph-presence falsifier, and replace the generic-derive
//!   case (now a compile error) with the two remedies its message names.
//! - 2026-07-20T00:00:00Z @AI: Add graph-registration coverage for HexDomain/HexPort/HexAdapter/HexDirective/HexQuery and #[hex(role)] override.

#![cfg(feature = "macros")]

use hexser::prelude::*;

#[derive(HexDomain)]
struct RegTestDomainEntity {
  _name: String,
}

// A domain layer is not made only of entities. Before 2026-09-04 this registered as
// `Role::Entity` and said nothing about it.
#[derive(HexDomain)]
#[hex(role = "ValueObject")]
struct RegTestDomainValueObject {
  _value: String,
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

// HexQuery is the registration-only Application-layer derive (no trait impl of its own), so
// with the role override it is the vehicle for an Application component that is neither a
// directive nor a query.
#[derive(HexQuery)]
#[hex(role = "UseCase")]
struct RegTestApplicationUseCase;

// ── The generic arm ─────────────────────────────────────────────────────────────────────
// `#[derive(HexDomain)] struct RegTestGeneric<T>` is now a COMPILE ERROR — see
// `tests/ui/fail/generic_domain.rs`. The error names two remedies, and both are pinned here.

/// Remedy 1: a non-generic marker struct stands for the component in the graph.
#[derive(HexDomain)]
#[hex(role = "ValueObject")]
struct RegTestGenericComponent;

/// The real generic type, deliberately underived.
struct RegTestGeneric<T> {
  _inner: T,
}

/// Remedy 2: hand-write `Registrable` when a caller needs `node_info()` for a concrete
/// instantiation. A hand-written impl submits nothing, so this type is correctly absent from
/// the graph — implementing `Registrable` and being in the graph are two different facts.
impl<T: 'static> hexser::registry::Registrable for RegTestGeneric<T> {
  fn node_info() -> hexser::registry::NodeInfo {
    hexser::registry::NodeInfo {
      layer: hexser::graph::Layer::Domain,
      role: hexser::graph::Role::Entity,
      type_name: std::any::type_name::<Self>(),
      module_path: module_path!(),
    }
  }

  fn dependencies() -> Vec<hexser::graph::NodeId> {
    Vec::new()
  }
}

// ── The non-derive registration path ────────────────────────────────────────────────────
// hexser's own `hex_register_*` macros are the no-macros/explicit door into the graph. They
// emitted the `Registrable` impl and no submission until 2026-09-04.
struct RegTestTemplateRegistered;

hexser::hex_register_domain!(
  RegTestTemplateRegistered,
  hexser::graph::Role::DomainService
);

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

/// why: the `#[hex(role = "ValueObject")]` override must be parsed and applied by HexDomain,
/// not ignored. The derive declared `attributes(hex)` and never called `role_override`, so a
/// value object marked ValueObject compiled clean and registered as an Entity — a graph that
/// lies is worse than no graph. Flip: drop `role_override` from `hex_domain.rs` and this
/// returns `Role::Entity`.
#[test]
fn test_domain_role_override_is_applied() {
  assert_eq!(
    find_role("RegTestDomainValueObject"),
    Some(hexser::graph::Role::ValueObject)
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

/// why: HexQuery must honour `#[hex(role = "…")]` like HexPort/HexAdapter do, so an
/// Application-layer component that is neither a directive nor a query can be registered
/// with a derive instead of a hand-rolled macro. Flip: drop `role_override` from `query.rs`
/// and this returns `Role::Query`.
#[test]
fn test_query_role_override_is_applied() {
  assert_eq!(
    find_role("RegTestApplicationUseCase"),
    Some(hexser::graph::Role::UseCase)
  );
}

/// why: the generic derive's compile error names a marker struct as the remedy, so the marker
/// must actually land in the graph — an error pointing at a remedy that does not work is a
/// second defect, not a fix.
#[test]
fn test_generic_marker_component_is_registered() {
  assert_eq!(
    find_role("RegTestGenericComponent"),
    Some(hexser::graph::Role::ValueObject)
  );
}

/// why: the other remedy the error names — hand-writing `Registrable` on the generic — must
/// still compile and answer node_info() for a concrete instantiation.
#[test]
fn test_generic_hand_written_registrable_reports_layer() {
  let info = <RegTestGeneric<u32> as hexser::registry::Registrable>::node_info();
  assert_eq!(info.layer, hexser::graph::Layer::Domain);
  assert_eq!(info.role, hexser::graph::Role::Entity);
}

/// why: a hand-written `Registrable` submits nothing, so the generic must NOT be in the graph.
/// This is the fact the generic derive used to produce in silence, now asserted deliberately:
/// implementing `Registrable` and being in the graph are two different columns.
#[test]
fn test_hand_written_registrable_is_not_in_the_graph() {
  assert_eq!(find_role("RegTestGeneric<u32>"), None);
}

/// why: `hexser::hex_register_domain!` is the crate's advertised non-derive registration door.
/// It emitted the `Registrable` impl and NO `inventory::submit!`, for every type — so a
/// consumer following hexser's own template got a component that answered `node_info()` and
/// was in no graph. Flip: delete the `inventory::submit!` from `hex_register_component!` and
/// this returns `None` while `node_info()` keeps answering correctly.
#[test]
fn test_template_macro_registration_reaches_the_graph() {
  assert_eq!(
    find_role("RegTestTemplateRegistered"),
    Some(hexser::graph::Role::DomainService)
  );
}
