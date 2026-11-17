//! What is wrong with an architecture graph, as closed values (PRD HEXSER-0.7 D6).
//!
//! A graph never refuses to build: every graph that compiled under 0.6 still builds. What it
//! now does is SAY what is wrong, as a [`GraphFinding`] a cell can assert, instead of a string
//! warning or a silence. Three findings exist:
//!
//! - [`GraphFinding::ShortNameCollision`]: two registrations in two modules share one short
//!   name, so they hash to one node id and one of them is kept (F-A). The keys are unchanged
//!   (Scott's card, Q2); the merge is now loud.
//! - [`GraphFinding::AdapterWithoutProvedPort`]: an Adapter-layer node (not a `Mapper`) with no
//!   `Implements` edge. Its port, if it has one, is only declared (SB 15/380). Asked for by Flo
//!   (ledger 229010) and Deck (231868 a).
//! - [`GraphFinding::EdgeToUnregisteredNode`]: an edge whose source or target is not a node of
//!   this graph. Asked for by the formula lane (240776 item 3).
//!
//! Revision History
//! - 2026-11-17T18:04:19Z @AI: HEXSER-0.7 (SB 15/203) — call the canonical `with_node`/`with_edge`/`NodeId::from_name`; their aliases are deprecated.
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 D6 — initial findings and their cells (R04-R08).

/// Something wrong with a graph, as a value. Read with [`HexGraph::findings`](crate::graph::HexGraph::findings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphFinding {
  /// Two registrations hashed to one node id. `kept` is the one the graph holds (the later
  /// registration); `dropped` is the one merged into it.
  ShortNameCollision {
    /// The shared node id.
    id: crate::graph::node_id::NodeId,
    /// The kept registration's type name, as registered.
    kept_type_name: std::string::String,
    /// The kept registration's module path.
    kept_module_path: std::string::String,
    /// The dropped registration's type name, as registered.
    dropped_type_name: std::string::String,
    /// The dropped registration's module path.
    dropped_module_path: std::string::String,
  },
  /// An Adapter-layer node, other than a `Mapper`, with no `Implements` edge.
  AdapterWithoutProvedPort {
    /// The adapter's node id.
    adapter: crate::graph::node_id::NodeId,
  },
  /// An edge whose source or target is not a node of this graph.
  EdgeToUnregisteredNode {
    /// The edge's source id.
    source: crate::graph::node_id::NodeId,
    /// The edge's target id.
    target: crate::graph::node_id::NodeId,
    /// The edge's relationship.
    relationship: crate::graph::relationship::Relationship,
  },
}

#[cfg(test)]
mod tests {
  fn node(
    name: &str,
    module_path: &str,
    layer: crate::graph::layer::Layer,
    role: crate::graph::role::Role,
  ) -> crate::graph::hex_node::HexNode {
    crate::graph::hex_node::HexNode::new(
      crate::graph::node_id::NodeId::from_name(name),
      layer,
      role,
      name,
      module_path,
    )
  }

  fn adapter(name: &str) -> crate::graph::hex_node::HexNode {
    node(
      name,
      "adapters",
      crate::graph::layer::Layer::Adapter,
      crate::graph::role::Role::Adapter,
    )
  }

  fn port(name: &str) -> crate::graph::hex_node::HexNode {
    node(
      name,
      "ports",
      crate::graph::layer::Layer::Port,
      crate::graph::role::Role::OutputPort,
    )
  }

  fn edge(
    from: &str,
    to: &str,
    relationship: crate::graph::relationship::Relationship,
  ) -> crate::graph::hex_edge::HexEdge {
    crate::graph::hex_edge::HexEdge::new(
      crate::graph::node_id::NodeId::from_name(from),
      crate::graph::node_id::NodeId::from_name(to),
      relationship,
    )
  }

  /// why: R04. Two template registrations of `FeedNode` in two modules hash to one id, since
  /// templates key by the short name. 0.6 merged them in silence (builder.rs compared names
  /// only); the merge must now be a typed finding naming both modules. Keys stay unchanged, so
  /// the graph still holds one node.
  #[test]
  fn two_template_nodes_with_one_short_name_in_two_modules_are_a_typed_finding() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .with_node(node(
        "FeedNode",
        "deck_view",
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::OutputPort,
      ))
      .with_node(node(
        "FeedNode",
        "flo_view",
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::OutputPort,
      ))
      .build();
    std::assert_eq!(graph.node_count(), 1);
    std::assert_eq!(
      graph.findings(),
      std::vec![super::GraphFinding::ShortNameCollision {
        id: crate::graph::node_id::NodeId::from_name("FeedNode"),
        kept_type_name: std::string::String::from("FeedNode"),
        kept_module_path: std::string::String::from("flo_view"),
        dropped_type_name: std::string::String::from("FeedNode"),
        dropped_module_path: std::string::String::from("deck_view"),
      }]
    );
  }

  /// why: R05, the control. One registration seen twice (same name, same module) is not a
  /// collision; a builder that reported every repeated id would turn this red.
  #[test]
  fn one_type_registered_twice_is_not_a_collision() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .with_node(port("SinkPortNode"))
      .with_node(port("SinkPortNode"))
      .build();
    std::assert_eq!(graph.node_count(), 1);
    std::assert!(graph.findings().is_empty());
    std::assert!(graph.metadata().warnings().is_empty());
  }

  /// why: R06. An adapter with no `Implements` edge is a finding even when a `Depends` edge
  /// declares a port (declared is not proved, SB 15/380); a proved adapter and a `Mapper` are
  /// not findings.
  #[test]
  fn an_adapter_with_no_implements_edge_is_a_finding_and_a_mapper_is_not() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .with_node(port("SinkPortNode"))
      .with_node(adapter("Proved"))
      .with_node(adapter("DeclaredOnly"))
      .with_node(adapter("Bare"))
      .with_node(node(
        "RowMapper",
        "adapters",
        crate::graph::layer::Layer::Adapter,
        crate::graph::role::Role::Mapper,
      ))
      .with_edges(std::vec![
        edge(
          "Proved",
          "SinkPortNode",
          crate::graph::relationship::Relationship::Implements
        ),
        edge(
          "DeclaredOnly",
          "SinkPortNode",
          crate::graph::relationship::Relationship::Depends
        ),
      ])
      .build();
    let findings = graph.findings();
    let unproved = |name: &str| super::GraphFinding::AdapterWithoutProvedPort {
      adapter: crate::graph::node_id::NodeId::from_name(name),
    };
    std::assert!(findings.contains(&unproved("DeclaredOnly")));
    std::assert!(findings.contains(&unproved("Bare")));
    std::assert!(!findings.contains(&unproved("Proved")));
    std::assert!(!findings.contains(&unproved("RowMapper")));
    std::assert_eq!(findings.len(), 2);
  }

  /// why: R07. An edge whose target (or source) is not registered is a finding, not a silent
  /// missing edge; an edge between registered nodes is not.
  #[test]
  fn an_edge_to_an_unregistered_node_is_a_finding() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .with_node(port("SinkPortNode"))
      .with_node(adapter("Proved"))
      .with_edges(std::vec![
        edge(
          "Proved",
          "SinkPortNode",
          crate::graph::relationship::Relationship::Implements
        ),
        edge(
          "Proved",
          "GonePortNode",
          crate::graph::relationship::Relationship::Depends
        ),
        edge(
          "Ghost",
          "SinkPortNode",
          crate::graph::relationship::Relationship::Depends
        ),
      ])
      .build();
    std::assert_eq!(
      graph.findings(),
      std::vec![
        super::GraphFinding::EdgeToUnregisteredNode {
          source: crate::graph::node_id::NodeId::from_name("Proved"),
          target: crate::graph::node_id::NodeId::from_name("GonePortNode"),
          relationship: crate::graph::relationship::Relationship::Depends,
        },
        super::GraphFinding::EdgeToUnregisteredNode {
          source: crate::graph::node_id::NodeId::from_name("Ghost"),
          target: crate::graph::node_id::NodeId::from_name("SinkPortNode"),
          relationship: crate::graph::relationship::Relationship::Depends,
        },
      ]
    );
  }

  /// why: R08. The 0.6 string warning is still written for a collision, rendered from the
  /// typed finding, so a 0.6 reader of `metadata().warnings()` still sees it and both modules
  /// are named in it.
  #[test]
  fn the_0_6_warning_text_is_rendered_from_the_typed_collision() {
    let graph = crate::graph::builder::GraphBuilder::new()
      .with_node(port("FeedNode"))
      .with_node(node(
        "FeedNode",
        "flo_view",
        crate::graph::layer::Layer::Port,
        crate::graph::role::Role::OutputPort,
      ))
      .build();
    let warnings = graph.metadata().warnings();
    std::assert_eq!(warnings.len(), 1);
    std::assert!(warnings[0].contains("ports::FeedNode"), "{}", warnings[0]);
    std::assert!(
      warnings[0].contains("flo_view::FeedNode"),
      "{}",
      warnings[0]
    );
  }
}
