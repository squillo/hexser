//! Template framework for quickly scaffolding hexser components.
//!
//! This module provides lightweight helpers and macros that REGISTER your components
//! without relying on derive macros: each one emits the `Registrable` impl AND the
//! `inventory::submit!` that puts the type in `HexGraph::current()`. It complements
//! proc-macro derives by offering simple, explicit building blocks you can use in any
//! context (including no-macros builds).
//!
//! ⚠ BOTH HALVES ARE THE REGISTRATION. `HexGraph::current()` is built by iterating the
//! `inventory` registry, so a `Registrable` impl on its own yields a type that answers
//! `node_info()` correctly and is in NO graph. These macros emitted only the impl until
//! 2026-09-04: the crate had no non-derive door into the graph at all, while naming these
//! `hex_register_*`.
//!
//! Registration is an ITEM-scope act — the submission is a static discovered at link time —
//! so invoke these macros at module scope, next to the type, not inside a function body.
//!
//! # Quick examples
//!
//! ```rust
//! use hexser::prelude::*;
//!
//! struct MyEntity { id: u64 }
//!
//! // Register a domain Entity using a template macro (module scope, next to the type).
//! hexser::hex_register_domain!(MyEntity, Role::Entity);
//!
//! fn main() {
//!   // The type answers node_info() AND is in the process-wide graph.
//!   let info = <MyEntity as Registrable>::node_info();
//!   assert_eq!(info.layer, Layer::Domain);
//!   assert_eq!(info.role, Role::Entity);
//!   assert!(
//!     hexser::HexGraph::current()
//!       .nodes()
//!       .any(|n| n.type_name().ends_with("MyEntity"))
//!   );
//! }
//! ```
//!
//! ```rust
//! use hexser::prelude::*;
//!
//! struct PgUserRepo;
//!
//! // Register as an Adapter implementing a Repository.
//! hexser::hex_register_adapter!(PgUserRepo, Role::Adapter);
//!
//! fn main() {
//!   assert_eq!(
//!     <PgUserRepo as Registrable>::node_info().layer,
//!     Layer::Adapter
//!   );
//! }
//! ```
//!
//! These helpers are intended as templates: copy, adapt, and extend as needed.
//!
//! Revision History
//! - 2026-11-03T00:00:00Z @AI: `depends_on: [...]` on `hex_register_component!` and its five layer shorthands, plus `dependency_id::<D>()`: a registration can now carry its edges (adapter -> port node). The dependency id is derived from `D::node_info().type_name`, exactly as `ComponentRegistry::build_graph` keys nodes, because `NodeId::of` hashes the full path while template nodes are keyed by the short name, so an edge built with it dangles silently. The three-argument forms are unchanged. Graph-reading cells pin the edge and its absence.
//! - 2026-09-11T00:00:00Z @AI: Allow needless_doctest_main — the explicit `fn main` is what keeps the macro invocations at module scope, which is the whole subject of these examples.
//! - 2026-09-04T00:00:00Z @AI: The hex_register_* macros now emit the inventory submission they
//!   are named for. They implemented `Registrable` and never submitted, so every type
//!   "registered" through hexser's own explicit path answered node_info() correctly and was
//!   absent from every graph query — the same end state as the silently-omitted generic
//!   submission, reached through the door the crate advertises.

// The `fn main` in the examples above is load-bearing, not boilerplate: rustdoc only skips its
// implicit wrapper when the snippet declares `main` itself, and skipping it is the only way to
// invoke `hex_register_*!` at MODULE scope — which is exactly the constraint these examples
// exist to demonstrate (registration is an item-scope act, see the preamble). Removing the
// `fn main` to satisfy the lint would move the macro into a function body and teach the misuse.
#![allow(clippy::needless_doctest_main)]

/// Split a fully-qualified Rust type path into (module_path, type_name).
///
/// For example: "my_crate::module::Type" -> ("my_crate::module", "Type").
/// If the input has no module qualifiers, module_path is an empty string.
pub fn split_type_name(full: &'static str) -> (&'static str, &'static str) {
  match full.rfind("::") {
    Some(idx) => (&full[..idx], &full[idx + 2..]),
    None => ("", full),
  }
}

/// The graph id of a REGISTERED dependency `D`, derived exactly the way
/// [`ComponentRegistry::build_graph`](crate::registry::component_registry::ComponentRegistry::build_graph)
/// derives every node's id: from `D`'s own `NodeInfo::type_name`.
///
/// ⚠ Not `NodeId::of::<D>()`. That hashes the FULL type path, while a node registered through
/// these templates is keyed by its SHORT name (`split_type_name`), so an edge built with
/// `NodeId::of` points at an id no node has. `build_graph` does not validate edges, so that
/// edge dangles silently: the graph reports an edge count and no query from the port finds the
/// adapter. Requiring `D: Registrable` also makes an edge to an unregistered port a compile
/// error instead of a dangling id.
pub fn dependency_id<D: crate::registry::Registrable>() -> crate::graph::NodeId {
  crate::graph::NodeId::from_type_name(D::node_info().type_name)
}

/// Core macro that REGISTERS a type: it implements `Registrable` with the given layer and
/// role AND submits the component to the inventory registry, so the type is in
/// `HexGraph::current()`.
///
/// ⚠ Both halves are the registration, and this macro emitted only the first until
/// 2026-09-04 — the type answered `node_info()` correctly and no graph query could find it.
/// That is the same silent end state as a derive dropping its submission, reached through
/// the macro the crate itself names `register`.
///
/// The type must be CONCRETE: it is used as `ComponentEntry::new::<T>()` at item scope, where
/// a generic parameter is not in scope, so a generic target is a compile error — the same
/// polarity `#[derive(HexDomain)]` and its siblings now have. Register a non-generic marker
/// struct that stands for the component instead.
///
/// # Edges: `depends_on: [...]`
///
/// The three-argument form registers a NODE with no edges (`dependencies()` is empty), so a
/// graph built only from it can say an adapter exists and never which port it plugs into.
/// The `depends_on:` form also declares the component's edges, one `Depends` edge per listed
/// type. Each listed type must itself be registered (a `Registrable`), typically a
/// zero-sized `<Port>Node` marker registered with `hex_register_port!`; its id is taken
/// through [`dependency_id`](crate::templates::dependency_id), so the edge lands on the real node.
///
/// ```rust
/// struct UserRepositoryPortNode;
/// struct PgUserRepo;
///
/// hexser::hex_register_port!(UserRepositoryPortNode, hexser::Role::Repository);
/// hexser::hex_register_adapter!(PgUserRepo, hexser::Role::Adapter, depends_on: [UserRepositoryPortNode]);
///
/// fn main() {
///   let graph = hexser::HexGraph::current();
///   let port = hexser::templates::dependency_id::<UserRepositoryPortNode>();
///   let adapter = hexser::templates::dependency_id::<PgUserRepo>();
///   assert!(graph.edges_to(&port).iter().any(|e| *e.source() == adapter));
/// }
/// ```
///
/// Invoke at MODULE scope, next to the type: the submission is a link-time static.
#[macro_export]
macro_rules! hex_register_component {
  (@register $t:ty, $layer:expr, $role:expr, [$($dep:ty),*]) => {
    impl $crate::registry::Registrable for $t {
      fn node_info() -> $crate::registry::NodeInfo {
        let full = ::std::any::type_name::<Self>();
        let (module_path, type_name) = $crate::templates::split_type_name(full);
        $crate::registry::NodeInfo::new($layer, $role, type_name, module_path)
      }
      fn dependencies() -> ::std::vec::Vec<$crate::graph::NodeId> {
        ::std::vec![$($crate::templates::dependency_id::<$dep>()),*]
      }
    }

    $crate::inventory::submit! {
      $crate::registry::ComponentEntry::new::<$t>()
    }
  };
  ($t:ty, $layer:expr, $role:expr) => {
    $crate::hex_register_component!(@register $t, $layer, $role, []);
  };
  ($t:ty, $layer:expr, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!(@register $t, $layer, $role, [$($dep),+]);
  };
}

/// Convenience macro for Domain-layer components. Takes the same optional `depends_on: [...]`
/// as [`hex_register_component!`](crate::hex_register_component).
#[macro_export]
macro_rules! hex_register_domain {
  ($t:ty, $role:expr) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Domain, $role);
  };
  ($t:ty, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Domain, $role, depends_on: [$($dep),+]);
  };
}

/// Convenience macro for Port-layer components. Takes the same optional `depends_on: [...]`
/// as [`hex_register_component!`](crate::hex_register_component).
#[macro_export]
macro_rules! hex_register_port {
  ($t:ty, $role:expr) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Port, $role);
  };
  ($t:ty, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Port, $role, depends_on: [$($dep),+]);
  };
}

/// Convenience macro for Adapter-layer components. Takes the same optional `depends_on: [...]`
/// as [`hex_register_component!`](crate::hex_register_component): name the port node(s) the adapter implements, so the graph
/// carries the adapter → port edge.
#[macro_export]
macro_rules! hex_register_adapter {
  ($t:ty, $role:expr) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Adapter, $role);
  };
  ($t:ty, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Adapter, $role, depends_on: [$($dep),+]);
  };
}

/// Convenience macro for Application-layer components. Takes the same optional
/// `depends_on: [...]` as [`hex_register_component!`](crate::hex_register_component).
#[macro_export]
macro_rules! hex_register_application {
  ($t:ty, $role:expr) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Application, $role);
  };
  ($t:ty, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Application, $role, depends_on: [$($dep),+]);
  };
}

/// Convenience macro for Infrastructure-layer components. Takes the same optional
/// `depends_on: [...]` as [`hex_register_component!`](crate::hex_register_component).
#[macro_export]
macro_rules! hex_register_infrastructure {
  ($t:ty, $role:expr) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Infrastructure, $role);
  };
  ($t:ty, $role:expr, depends_on: [$($dep:ty),+ $(,)?]) => {
    $crate::hex_register_component!($t, $crate::graph::Layer::Infrastructure, $role, depends_on: [$($dep),+]);
  };
}

#[cfg(test)]
mod tests {
  use crate::prelude::*;

  struct TDomain;
  struct TAdapter;

  // Use the macros to implement Registrable
  hex_register_domain!(TDomain, Role::Entity);
  hex_register_adapter!(TAdapter, Role::Adapter);

  #[test]
  fn test_domain_template_macro() {
    let info = <TDomain as Registrable>::node_info();
    assert_eq!(info.layer, Layer::Domain);
    assert_eq!(info.role, Role::Entity);
    assert_eq!(info.type_name, "TDomain");
  }

  #[test]
  fn test_adapter_template_macro() {
    let info = <TAdapter as Registrable>::node_info();
    assert_eq!(info.layer, Layer::Adapter);
    assert_eq!(info.role, Role::Adapter);
    assert_eq!(info.type_name, "TAdapter");
  }

  #[test]
  fn test_split_type_name() {
    let (m, n) = super::split_type_name("foo::bar::Baz");
    assert_eq!(m, "foo::bar");
    assert_eq!(n, "Baz");
    let (m2, n2) = super::split_type_name("Baz");
    assert_eq!(m2, "");
    assert_eq!(n2, "Baz");
  }

  // Edge cells for `depends_on:`. The names are deliberately distinctive: the graph is
  // process-wide, and template-registered nodes are keyed by their SHORT type name.
  struct TemplatesEdgeCellPortNode;
  struct TemplatesEdgeCellWiredAdapter;
  struct TemplatesEdgeCellBareAdapter;

  hex_register_port!(TemplatesEdgeCellPortNode, Role::OutputPort);
  hex_register_adapter!(TemplatesEdgeCellWiredAdapter, Role::Adapter, depends_on: [TemplatesEdgeCellPortNode]);
  hex_register_adapter!(TemplatesEdgeCellBareAdapter, Role::Adapter);

  /// WHY: `depends_on:` exists so the graph carries the adapter -> port edge; a node-presence
  /// cell says nothing about wiring. MUTATION: drop `depends_on:` from the wired adapter, or
  /// derive the dependency id with `NodeId::of` instead of `dependency_id` => red (the edge
  /// then targets an id no node has).
  #[test]
  fn an_adapter_registered_with_depends_on_has_an_edge_onto_its_real_port_node() {
    let graph = crate::HexGraph::current();
    let port = super::dependency_id::<TemplatesEdgeCellPortNode>();
    let adapter = super::dependency_id::<TemplatesEdgeCellWiredAdapter>();
    std::assert!(
      graph.nodes().any(|n| *n.id() == port),
      "the port node the edge targets is a node in the graph"
    );
    std::assert!(
      graph.edges_to(&port).iter().any(|e| *e.source() == adapter),
      "the adapter -> port edge lands on the registered port node"
    );
  }

  /// WHY: the three-argument form keeps its behaviour (a node with no edges).
  /// MUTATION: make the bare arm emit a dependency => red.
  #[test]
  fn an_adapter_registered_without_depends_on_has_no_edges() {
    std::assert!(<TemplatesEdgeCellBareAdapter as Registrable>::dependencies().is_empty());
    let graph = crate::HexGraph::current();
    std::assert!(
      graph
        .edges_from(&super::dependency_id::<TemplatesEdgeCellBareAdapter>())
        .is_empty()
    );
  }
}
