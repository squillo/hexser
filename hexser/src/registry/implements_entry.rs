//! A proved `Implements` claim, submitted at link time beside the component it belongs to
//! (PRD HEXSER-0.7 D5).
//!
//! `#[derive(HexAdapter)]` with `#[hex(implements(PortNode))]`, and the template form
//! `implements: [PortNode]`, emit a compile-time proof that the adapter implements the port
//! node's trait, then submit one of these. `ComponentRegistry::build_graph` turns each into an
//! `Implements` edge. It is a collection of its own so that `ComponentEntry` and `Registrable`,
//! whose shapes downstream code can name, stay unchanged.
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D5 — initial `ImplementsEntry` collection.

/// One proved claim: adapter `A` implements the trait of port node `P`.
pub struct ImplementsEntry {
  adapter_fn: fn() -> crate::graph::node_id::NodeId,
  port_fn: fn() -> crate::graph::node_id::NodeId,
}

impl ImplementsEntry {
  /// The claim that `A` serves port node `P`. Both ids are taken the way `build_graph` keys
  /// every node, through [`dependency_id`](crate::templates::dependency_id).
  pub const fn new<A, P>() -> Self
  where
    A: crate::registry::registrable::Registrable,
    P: crate::registry::registrable::Registrable,
  {
    Self {
      adapter_fn: crate::templates::dependency_id::<A>,
      port_fn: crate::templates::dependency_id::<P>,
    }
  }

  /// The adapter's node id.
  pub fn adapter(&self) -> crate::graph::node_id::NodeId {
    (self.adapter_fn)()
  }

  /// The port node's id.
  pub fn port(&self) -> crate::graph::node_id::NodeId {
    (self.port_fn)()
  }
}

inventory::collect!(ImplementsEntry);
