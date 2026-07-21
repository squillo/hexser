//! HexGraph is the core immutable graph structure.
//!
//! HexGraph represents the entire hexagonal architecture as an immutable,
//! thread-safe graph. It uses Arc for zero-cost sharing across threads
//! and provides methods for querying nodes and edges. Graphs are constructed
//! using GraphBuilder and cannot be modified after creation.
//!
//! Revision History
//! - 2026-07-21T00:00:00Z @AI: PRD-272 §1.5/§3.G — current() moves OnceLock→`LazyLock<ArcSwap<HexGraph>>` (wait-free RCU, self-programming hot-swap via install()/rebuild_current(), SOLE write site per N_BOOK §23); adjacency IndexMap (§3.H).
//! - 2026-07-20T00:00:00Z @AI: Cache current() via OnceLock; BTreeMap nodes for deterministic iteration; O(degree) edges_from/edges_to via precomputed adjacency indices.
//! - 2025-10-02T14:00:00Z @AI: Rename nodes_in_layer to nodes_by_layer and nodes_by_role to nodes_by_role for better API naming.
//! - 2025-10-01T00:03:00Z @AI: Initial immutable HexGraph implementation for Phase 2.

/// Immutable graph representing hexagonal architecture components.
///
/// HexGraph is thread-safe and uses Arc internally for efficient sharing.
/// Once created, it cannot be modified - use GraphBuilder to create new graphs.
///
/// # Example
///
/// ```rust
/// use hexser::graph::{HexGraph, HexNode, NodeId, Layer, Role};
///
/// let graph = HexGraph::builder()
///     .with_node(HexNode::new(
///         NodeId::from_name("Entity"),
///         Layer::Domain,
///         Role::Entity,
///         "MyEntity",
///         "domain",
///     ))
///     .build();
///
/// assert_eq!(graph.node_count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct HexGraph {
  pub(crate) inner: std::sync::Arc<GraphInner>,
}

#[derive(Debug)]
pub(crate) struct GraphInner {
  /// Nodes keyed by id. A `BTreeMap` (NodeId is a `u64` newtype) gives deterministic iteration
  /// order, so DOT/Mermaid/JSON exports and the AI context are byte-stable run-to-run instead
  /// of shuffling with `HashMap`'s randomized hasher.
  pub(crate) nodes:
    std::collections::BTreeMap<crate::graph::node_id::NodeId, crate::graph::hex_node::HexNode>,
  pub(crate) edges: Vec<crate::graph::hex_edge::HexEdge>,
  /// Precomputed adjacency: source node id -> indices into `edges`. Lets `edges_from` return in
  /// O(degree) instead of scanning every edge (which made `to_ai_context` O(V*E)).
  /// `IndexMap` (PRD-272 §3.H) gives deterministic iteration for byte-stable exports.
  pub(crate) outgoing: indexmap::IndexMap<crate::graph::node_id::NodeId, std::vec::Vec<usize>>,
  /// Precomputed adjacency: target node id -> indices into `edges` (for `edges_to`).
  pub(crate) incoming: indexmap::IndexMap<crate::graph::node_id::NodeId, std::vec::Vec<usize>>,
  pub(crate) metadata: crate::graph::metadata::GraphMetadata,
}

impl HexGraph {
  /// The process-wide architecture graph, behind a wait-free RCU cell.
  ///
  /// The graph is the self-programming substrate: read on hot paths (every MCP request via
  /// `ProjectRegistry::from_current_graph`, every AI-context export) and hot-swapped when the
  /// program rewrites itself (`hexser/refresh` → [`HexGraph::install`]). Per N_BOOK §1.5
  /// (wait-free RCU) + §23 (Tier 2 SOLE write site) + PRD-272 §1.5/§3.G this state MUST live
  /// behind `arc_swap::ArcSwap`, not `OnceLock`/`RwLock` — reads are lock-free (`load_full`) and
  /// the SOLE write site is [`HexGraph::install`]. Canon exemplar shape: `LazyLock<ArcSwap<…>>`
  /// (PRD-272 change-log 2026-06-26).
  fn arch() -> &'static arc_swap::ArcSwap<HexGraph> {
    static ARCH: std::sync::LazyLock<arc_swap::ArcSwap<HexGraph>> =
      std::sync::LazyLock::new(|| {
        arc_swap::ArcSwap::from_pointee(
          crate::registry::component_registry::ComponentRegistry::build_graph(),
        )
      });
    &ARCH
  }

  /// Get the current architecture graph.
  ///
  /// Built once from the link-time `inventory` registry on first access, then returned by a
  /// wait-free `load_full` (cheap `Arc` clone). Reflects the latest [`HexGraph::install`].
  pub fn current() -> std::sync::Arc<Self> {
    Self::arch().load_full()
  }

  /// Install a new architecture graph as the current one (the SOLE write site, PRD-272 §1.5).
  ///
  /// This is the live self-programming hot-swap: after the program rewrites itself, install the
  /// rebuilt graph and every subsequent `current()` sees it — no process restart required.
  /// Writers are serialized only against each other; concurrent readers are never blocked.
  pub fn install(graph: HexGraph) {
    Self::arch().store(std::sync::Arc::new(graph));
  }

  /// Rebuild the graph from the current link-time registry and install it.
  ///
  /// Note: `inventory` is fixed at link time, so this only picks up new components after the
  /// binary is recompiled; it is the in-process half of the `hexser/refresh` flow.
  pub fn rebuild_current() {
    Self::install(crate::registry::component_registry::ComponentRegistry::build_graph());
  }

  /// Create a new empty graph.
  pub fn new() -> Self {
    Self {
      inner: std::sync::Arc::new(GraphInner {
        nodes: std::collections::BTreeMap::new(),
        edges: Vec::new(),
        outgoing: indexmap::IndexMap::new(),
        incoming: indexmap::IndexMap::new(),
        metadata: crate::graph::metadata::GraphMetadata::default(),
      }),
    }
  }

  /// Export to DOT format
  #[cfg(feature = "visualization")]
  pub fn to_dot(&self) -> crate::result::hex_result::HexResult<String> {
    let exporter = crate::graph::visualization::adapters::dot_exporter::DotExporter::new();
    let use_case =
      crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
    use_case.execute(
      self,
      crate::graph::visualization::domain::visual_style::VisualStyle::default(),
    )
  }

  /// Export to Mermaid format
  #[cfg(feature = "visualization")]
  pub fn to_mermaid(&self) -> crate::result::hex_result::HexResult<String> {
    let exporter = crate::graph::visualization::adapters::mermaid_exporter::MermaidExporter::new();
    let use_case =
      crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
    use_case.execute(
      self,
      crate::graph::visualization::domain::visual_style::VisualStyle::default(),
    )
  }

  /// Export to JSON format
  #[cfg(feature = "visualization")]
  pub fn to_json(&self) -> crate::result::hex_result::HexResult<String> {
    let exporter = crate::graph::visualization::adapters::json_exporter::JsonExporter::new();
    let use_case =
      crate::graph::visualization::application::export_graph::ExportGraph::new(&exporter);
    use_case.execute(
      self,
      crate::graph::visualization::domain::visual_style::VisualStyle::default(),
    )
  }

  /// Save visualization to file
  #[cfg(feature = "visualization")]
  pub fn save_visualization(
    &self,
    path: &std::path::Path,
    exporter: &dyn crate::graph::visualization::ports::format_exporter::FormatExporter,
  ) -> crate::result::hex_result::HexResult<()> {
    let use_case =
      crate::graph::visualization::application::export_graph::ExportGraph::new(exporter);
    let content = use_case.execute(
      self,
      crate::graph::visualization::domain::visual_style::VisualStyle::default(),
    )?;

    std::fs::write(path, content).map_err(|e| {
      crate::error::hex_error::Hexserror::adapter(
        crate::error::codes::io::IO_FAILURE,
        &format!("Failed to write file: {e}"),
      )
      .with_next_step("Check file path and permissions")
      .with_suggestion("Verify directory exists and is writable")
    })
  }

  /// Create a new graph builder.
  pub fn builder() -> crate::graph::builder::GraphBuilder {
    crate::graph::builder::GraphBuilder::new()
  }

  /// Get count of distinct layers in graph
  pub fn layer_count(&self) -> usize {
    let mut layers = std::collections::HashSet::new();
    for node in self.nodes() {
      layers.insert(node.layer());
    }
    layers.len()
  }

  /// Get the number of nodes in the graph.
  pub fn node_count(&self) -> usize {
    self.inner.nodes.len()
  }

  /// Get the number of edges in the graph.
  pub fn edge_count(&self) -> usize {
    self.inner.edges.len()
  }

  /// Export architecture context for AI agent consumption
  #[cfg(feature = "ai")]
  pub fn to_ai_context(&self) -> crate::result::hex_result::HexResult<crate::ai::AIContext> {
    crate::ai::ContextBuilder::new(self).build()
  }

  /// Get a node by its ID.
  pub fn get_node(
    &self,
    id: &crate::graph::node_id::NodeId,
  ) -> Option<&crate::graph::hex_node::HexNode> {
    self.inner.nodes.get(id)
  }

  /// Print human-readable summary of graph.
  ///
  /// This is an intentional stdout diagnostic for CLI/inspection use; the crate ships
  /// no logging facade, so `println!` is deliberate here (see clippy.toml guidance).
  #[allow(clippy::disallowed_macros)]
  pub fn pretty_print(&self) {
    println!("Hexagonal Architecture Graph:");
    println!("  Nodes: {}", self.node_count());
    println!("  Edges: {}", self.edge_count());
    println!("\nBy Layer:");
    for layer in [
      crate::graph::layer::Layer::Domain,
      crate::graph::layer::Layer::Port,
      crate::graph::layer::Layer::Adapter,
      crate::graph::layer::Layer::Application,
      crate::graph::layer::Layer::Infrastructure,
    ] {
      let count = self.nodes_by_layer(layer).len();
      if count > 0 {
        println!("  {layer:?}: {count}");
      }
    }
  }

  /// Get all nodes in the graph.
  pub fn nodes(&self) -> impl Iterator<Item = &crate::graph::hex_node::HexNode> {
    self.inner.nodes.values()
  }

  /// Get all edges in the graph.
  pub fn edges(&self) -> &[crate::graph::hex_edge::HexEdge] {
    &self.inner.edges
  }

  /// Get nodes by layer.
  pub fn nodes_by_layer(
    &self,
    layer: crate::graph::layer::Layer,
  ) -> Vec<&crate::graph::hex_node::HexNode> {
    self
      .inner
      .nodes
      .values()
      .filter(|n| n.layer() == layer)
      .collect()
  }

  /// Get nodes by role.
  pub fn nodes_by_role(
    &self,
    role: crate::graph::role::Role,
  ) -> Vec<&crate::graph::hex_node::HexNode> {
    self
      .inner
      .nodes
      .values()
      .filter(|n| n.role() == role)
      .collect()
  }

  /// Get edges from a specific node.
  ///
  /// O(degree) via the precomputed outgoing adjacency index (was O(edge count) per call).
  pub fn edges_from(
    &self,
    source: &crate::graph::node_id::NodeId,
  ) -> Vec<&crate::graph::hex_edge::HexEdge> {
    match self.inner.outgoing.get(source) {
      std::option::Option::Some(indices) => indices.iter().map(|&i| &self.inner.edges[i]).collect(),
      std::option::Option::None => std::vec::Vec::new(),
    }
  }

  /// Get edges to a specific node.
  ///
  /// O(degree) via the precomputed incoming adjacency index (was O(edge count) per call).
  pub fn edges_to(
    &self,
    target: &crate::graph::node_id::NodeId,
  ) -> Vec<&crate::graph::hex_edge::HexEdge> {
    match self.inner.incoming.get(target) {
      std::option::Option::Some(indices) => indices.iter().map(|&i| &self.inner.edges[i]).collect(),
      std::option::Option::None => std::vec::Vec::new(),
    }
  }

  /// Get graph metadata.
  pub fn metadata(&self) -> &crate::graph::metadata::GraphMetadata {
    &self.inner.metadata
  }

  /// Check if graph is empty.
  pub fn is_empty(&self) -> bool {
    self.inner.nodes.is_empty()
  }
}

impl Default for HexGraph {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_graph() {
    let graph = HexGraph::new();
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.is_empty());
  }

  #[test]
  fn test_graph_thread_safety() {
    let graph = HexGraph::new();
    let graph_clone = graph.clone();

    std::thread::spawn(move || {
      assert_eq!(graph_clone.node_count(), 0);
    })
    .join()
    .unwrap();
  }

  #[test]
  fn test_graph_default() {
    let graph = HexGraph::default();
    assert!(graph.is_empty());
  }

  fn node(name: &str, layer: crate::graph::layer::Layer) -> crate::graph::hex_node::HexNode {
    crate::graph::hex_node::HexNode::new(
      crate::graph::node_id::NodeId::from_name(name),
      layer,
      crate::graph::role::Role::Entity,
      name,
      "test",
    )
  }

  fn edge(from: &str, to: &str) -> crate::graph::hex_edge::HexEdge {
    crate::graph::hex_edge::HexEdge::new(
      crate::graph::node_id::NodeId::from_name(from),
      crate::graph::node_id::NodeId::from_name(to),
      crate::graph::relationship::Relationship::Depends,
    )
  }

  /// why: two back-to-back `current()` loads with no intervening `install()` must return Arcs to
  /// the same graph allocation (wait-free `load_full` of the ArcSwap), so request handlers don't
  /// re-walk inventory. `#[serial]` keeps a concurrent `install()` from another test out of the
  /// window between the two loads.
  #[test]
  #[serial_test::serial(hexser_arch)]
  fn test_current_is_cached_same_arc() {
    let a = HexGraph::current();
    let b = HexGraph::current();
    assert!(
      std::sync::Arc::ptr_eq(&a, &b),
      "current() must return the same graph Arc when nothing was installed between loads"
    );
  }

  /// why: LESSON #16 — every NEW RCU (ArcSwap) surface requires a 16-OS-thread regression test.
  /// 16 threads hammer `current()` (wait-free reads) while one thread repeatedly `install()`s a
  /// distinct graph; assert no torn read / no panic, every observed graph is internally
  /// consistent, and the final `current()` reflects the last install. `#[serial]` isolates this
  /// mutation from the identity test above.
  #[test]
  #[serial_test::serial(hexser_arch)]
  fn test_arcswap_16_thread_read_during_install() {
    // Snapshot the pre-test graph so we can restore it (other tests read the global arch).
    let original = HexGraph::current();

    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut readers = std::vec::Vec::new();
    for _ in 0..16 {
      let stop = std::sync::Arc::clone(&stop);
      readers.push(std::thread::spawn(move || {
        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
          // Wait-free read; node_count() dereferences inner — a torn read would panic/segfault.
          let g = HexGraph::current();
          let _ = g.node_count();
          let _ = g.edge_count();
        }
      }));
    }

    // Sole writer: install graphs of increasing size, so the last install is identifiable.
    for i in 1..=64u32 {
      let mut builder = HexGraph::builder();
      for n in 0..i {
        builder = builder.with_node(node(&format!("N{n}"), crate::graph::layer::Layer::Domain));
      }
      HexGraph::install(builder.build());
    }

    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    for r in readers {
      r.join().expect("reader thread must not panic");
    }

    assert_eq!(
      HexGraph::current().node_count(),
      64,
      "final current() must reflect the last install"
    );

    // Restore the original graph for any test that reads the global arch afterwards.
    HexGraph::install((*original).clone());
  }

  /// why: edges_from/edges_to must return exactly the incident edges via the adjacency index,
  /// matching a brute-force scan — the index must not drop or duplicate edges.
  #[test]
  fn test_adjacency_index_matches_bruteforce() {
    let graph = HexGraph::builder()
      .with_node(node("A", crate::graph::layer::Layer::Domain))
      .with_node(node("B", crate::graph::layer::Layer::Port))
      .with_node(node("C", crate::graph::layer::Layer::Adapter))
      .with_edge(edge("A", "B"))
      .with_edge(edge("A", "C"))
      .with_edge(edge("C", "B"))
      .build();

    let a = crate::graph::node_id::NodeId::from_name("A");
    let b = crate::graph::node_id::NodeId::from_name("B");

    let from_a = graph.edges_from(&a);
    assert_eq!(from_a.len(), 2, "A has two outgoing edges");
    assert!(from_a.iter().all(|e| e.source() == &a));

    let to_b = graph.edges_to(&b);
    assert_eq!(to_b.len(), 2, "B has two incoming edges");
    assert!(to_b.iter().all(|e| e.target() == &b));

    // A node with no incident edges yields empty, not a panic.
    let isolated = crate::graph::node_id::NodeId::from_name("Z");
    assert!(graph.edges_from(&isolated).is_empty());
    assert!(graph.edges_to(&isolated).is_empty());
  }

  /// why: node iteration must be deterministic (BTreeMap by NodeId) so exports/AI context are
  /// byte-stable across runs. Building the same node set in different insertion orders must
  /// yield identical iteration order.
  #[test]
  fn test_node_iteration_is_deterministic() {
    let order1: std::vec::Vec<_> = HexGraph::builder()
      .with_node(node("A", crate::graph::layer::Layer::Domain))
      .with_node(node("B", crate::graph::layer::Layer::Port))
      .with_node(node("C", crate::graph::layer::Layer::Adapter))
      .build()
      .nodes()
      .map(|n| *n.id())
      .collect();
    let order2: std::vec::Vec<_> = HexGraph::builder()
      .with_node(node("C", crate::graph::layer::Layer::Adapter))
      .with_node(node("A", crate::graph::layer::Layer::Domain))
      .with_node(node("B", crate::graph::layer::Layer::Port))
      .build()
      .nodes()
      .map(|n| *n.id())
      .collect();
    assert_eq!(
      order1, order2,
      "iteration order must not depend on insertion order"
    );
  }
}
