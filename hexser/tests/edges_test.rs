//! Edges on the derives and the templates, port nodes tied to their traits, and the graph's
//! typed findings, read through a real `inventory`-built graph (PRD HEXSER-0.7 D4, D5, D6;
//! rows R06, R24-R27).
//!
//! Revision History
//! - 2026-11-17T16:31:10Z @AI: HEXSER-0.7 R06, R24-R27 — initial cells, written red.

/// A port with no generics.
pub trait SinkPort {
  fn append(&mut self, fact: u32);
}

/// A generic port.
pub trait GenericPort<T> {
  fn put(&self, item: T) -> usize;
}

/// A port with an associated type, as Tables' `UiSnappPort::Root`.
pub trait AssocPort {
  type Item;
  fn get(&self) -> Self::Item;
}

/// A port a template registers.
pub trait TemplatePort {
  fn ping(&self) -> u8;
}

#[derive(hexser::HexPort)]
#[hex(role = "OutputPort", port_trait = SinkPort)]
struct SinkPortNode;

#[derive(hexser::HexPort)]
#[hex(port_trait = GenericPort<u32>)]
struct GenericPortNode;

#[derive(hexser::HexPort)]
#[hex(port_trait = AssocPort, role = "OutputPort")]
struct AssocPortNode;

#[derive(hexser::HexAdapter)]
#[hex(implements(SinkPortNode))]
struct MemorySink {
  facts: std::vec::Vec<u32>,
}

impl SinkPort for MemorySink {
  fn append(&mut self, fact: u32) {
    self.facts.push(fact);
  }
}

#[derive(hexser::HexAdapter)]
#[hex(implements(GenericPortNode, AssocPortNode))]
struct BothPorts;

impl GenericPort<u32> for BothPorts {
  fn put(&self, item: u32) -> usize {
    item as usize
  }
}

impl AssocPort for BothPorts {
  type Item = u8;
  fn get(&self) -> u8 {
    7
  }
}

/// An adapter that names no port: the graph must say so.
#[derive(hexser::HexAdapter)]
struct Unported;

/// A mapper is an adapter-layer node that serves no port: never a finding.
#[derive(hexser::HexAdapter)]
#[hex(role = "Mapper")]
struct RowMapper;

/// A port marker a template registers, and one with its trait.
struct TemplatePortNode;
hexser::hex_register_port!(TemplatePortNode, hexser::Role::Repository);

struct TemplateTraitPortNode;
hexser::hex_register_port!(TemplateTraitPortNode, hexser::Role::OutputPort, port_trait: TemplatePort);

struct TemplateAdapter;
impl TemplatePort for TemplateAdapter {
  fn ping(&self) -> u8 {
    1
  }
}
hexser::hex_register_adapter!(TemplateAdapter, hexser::Role::Adapter, implements: [TemplateTraitPortNode]);

/// The template way of saying a dependency, for comparison with the derive way.
struct TemplateDirective;
hexser::hex_register_application!(TemplateDirective, hexser::Role::Directive, depends_on: [TemplatePortNode]);

/// The derive way: `depends_on` on HexDirective and on HexActor.
#[derive(hexser::HexDirective)]
#[hex(depends_on(TemplatePortNode, SinkPortNode))]
struct RenameDirective;

#[derive(hexser::HexActor)]
#[hex(depends_on(SinkPortNode), role = "DirectiveHandler")]
struct EditorActorNode;

fn graph() -> hexser::graph::HexGraph {
  hexser::registry::ComponentRegistry::build_graph()
}

/// why: R25. `implements(..)` lands one `Implements` edge from the adapter's node onto the
/// port node, and the reader finds it. The must-fail arms: an adapter that named no port, and
/// a port the adapter did not name, both read false.
#[test]
fn an_implements_edge_lands_on_the_port_node_and_reads_back() {
  let graph = graph();
  std::assert!(hexser::templates::implements_edge_exists::<
    MemorySink,
    SinkPortNode,
  >(&graph));
  std::assert!(hexser::templates::implements_edge_exists::<
    BothPorts,
    GenericPortNode,
  >(&graph));
  std::assert!(hexser::templates::implements_edge_exists::<
    BothPorts,
    AssocPortNode,
  >(&graph));
  std::assert!(hexser::templates::implements_edge_exists::<
    TemplateAdapter,
    TemplateTraitPortNode,
  >(&graph));

  std::assert!(!hexser::templates::implements_edge_exists::<
    Unported,
    SinkPortNode,
  >(&graph));
  std::assert!(!hexser::templates::implements_edge_exists::<
    MemorySink,
    AssocPortNode,
  >(&graph));

  let sink = hexser::templates::dependency_id::<MemorySink>();
  let port = hexser::templates::dependency_id::<SinkPortNode>();
  let implements: std::vec::Vec<_> = graph
    .edges_from(&sink)
    .into_iter()
    .filter(|edge| edge.relationship() == hexser::graph::Relationship::Implements)
    .collect();
  std::assert_eq!(implements.len(), 1);
  std::assert_eq!(*implements[0].target(), port);
}

/// why: R24. `port_trait` takes a generic trait and a trait with an associated type; the proof
/// holds for an adapter that implements them (this file compiles), and their nodes are ports.
#[test]
fn port_trait_accepts_a_generic_trait_and_one_with_an_associated_type() {
  let graph = graph();
  for id in [
    hexser::templates::dependency_id::<GenericPortNode>(),
    hexser::templates::dependency_id::<AssocPortNode>(),
    hexser::templates::dependency_id::<SinkPortNode>(),
  ] {
    let node = graph.get_node(&id).expect("the port node is registered");
    std::assert_eq!(node.layer(), hexser::graph::Layer::Port);
  }
  let assoc = graph
    .get_node(&hexser::templates::dependency_id::<AssocPortNode>())
    .expect("registered");
  std::assert_eq!(assoc.role(), hexser::graph::Role::OutputPort);
  std::assert_eq!(BothPorts.put(3), 3);
  std::assert_eq!(BothPorts.get(), 7);
}

/// why: R26. `depends_on` on a derive says the same thing as `depends_on:` on a template: a
/// `Depends` edge onto the same port node. A `Depends` edge never reads as `Implements`, and an
/// `Implements` edge never reads as `Depends` (SB 15/380: declared is not proved).
#[test]
fn depends_on_on_a_derive_equals_depends_on_on_a_template() {
  let graph = graph();
  std::assert!(hexser::templates::dependency_edge_exists::<
    TemplateDirective,
    TemplatePortNode,
  >(&graph));
  std::assert!(hexser::templates::dependency_edge_exists::<
    RenameDirective,
    TemplatePortNode,
  >(&graph));
  std::assert!(hexser::templates::dependency_edge_exists::<
    RenameDirective,
    SinkPortNode,
  >(&graph));
  std::assert!(hexser::templates::dependency_edge_exists::<
    EditorActorNode,
    SinkPortNode,
  >(&graph));

  let template_target: std::vec::Vec<_> = graph
    .edges_from(&hexser::templates::dependency_id::<TemplateDirective>())
    .into_iter()
    .map(|edge| *edge.target())
    .collect();
  let derive_targets: std::vec::Vec<_> = graph
    .edges_from(&hexser::templates::dependency_id::<RenameDirective>())
    .into_iter()
    .map(|edge| *edge.target())
    .collect();
  std::assert!(derive_targets.contains(&template_target[0]));

  std::assert!(!hexser::templates::implements_edge_exists::<
    RenameDirective,
    TemplatePortNode,
  >(&graph));
  std::assert!(!hexser::templates::dependency_edge_exists::<
    MemorySink,
    SinkPortNode,
  >(&graph));
}

/// why: R27. The reader is true only when the port's node is itself in the graph; an
/// `Implements` edge onto an absent node is not a proved port. Built by hand so the port node
/// can be left out; the control adds it back.
#[test]
fn implements_edge_exists_is_false_when_the_ports_node_is_absent() {
  let adapter = hexser::templates::dependency_id::<MemorySink>();
  let port = hexser::templates::dependency_id::<SinkPortNode>();
  let adapter_node = hexser::graph::HexNode::new(
    adapter,
    hexser::graph::Layer::Adapter,
    hexser::graph::Role::Adapter,
    "MemorySink",
    "edges_test",
  );
  let port_node = hexser::graph::HexNode::new(
    port,
    hexser::graph::Layer::Port,
    hexser::graph::Role::OutputPort,
    "SinkPortNode",
    "edges_test",
  );
  let edge = hexser::graph::HexEdge::new(adapter, port, hexser::graph::Relationship::Implements);

  let without_port = hexser::graph::GraphBuilder::new()
    .with_node(adapter_node.clone())
    .with_edges(std::vec![edge.clone()])
    .build();
  std::assert!(!hexser::templates::implements_edge_exists::<
    MemorySink,
    SinkPortNode,
  >(&without_port));

  let with_port = hexser::graph::GraphBuilder::new()
    .with_node(adapter_node)
    .with_node(port_node)
    .with_edges(std::vec![edge])
    .build();
  std::assert!(hexser::templates::implements_edge_exists::<
    MemorySink,
    SinkPortNode,
  >(&with_port));
}

/// why: R06 through the real graph. An adapter that named no port is a typed finding, a proved
/// adapter and a mapper are not. Counting a `Depends` edge as proof, or dropping the Mapper
/// exemption, turns this red.
#[test]
fn the_graph_names_the_adapter_without_a_proved_port() {
  let findings = graph().findings();
  let unported = hexser::templates::dependency_id::<Unported>();
  std::assert!(
    findings.contains(&hexser::graph::GraphFinding::AdapterWithoutProvedPort { adapter: unported })
  );
  for proved in [
    hexser::templates::dependency_id::<MemorySink>(),
    hexser::templates::dependency_id::<BothPorts>(),
    hexser::templates::dependency_id::<TemplateAdapter>(),
    hexser::templates::dependency_id::<RowMapper>(),
  ] {
    std::assert!(
      !findings
        .contains(&hexser::graph::GraphFinding::AdapterWithoutProvedPort { adapter: proved })
    );
  }
  let mut sink = MemorySink {
    facts: std::vec::Vec::new(),
  };
  SinkPort::append(&mut sink, 4);
  std::assert_eq!(sink.facts, std::vec![4]);
  std::assert_eq!(TemplatePort::ping(&TemplateAdapter), 1);
}
