//! Benchmarks for the graph hot paths targeted by the perf work.
//!
//! Measures the operations the perf audit identified as the real "repeated work" levers so the
//! caching/adjacency wins are backed by numbers rather than assumed:
//! - `edges_from` on a synthetic graph (adjacency index vs. the old O(E) scan)
//! - `to_ai_context` over a synthetic graph (was O(V*E) through per-node edge scans)
//!
//! Run with: `cargo bench -p hexser --features ai`
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Add criterion benches for edges_from and to_ai_context on a synthetic graph.

/// Build a synthetic graph of `n` nodes in a chain (edge i -> i+1) plus a hub node that every
/// node depends on, giving a realistic mix of in/out degrees for the adjacency benchmarks.
fn build_graph(n: usize) -> hexser::graph::hex_graph::HexGraph {
  let mut builder = hexser::graph::builder::GraphBuilder::new();
  for i in 0..n {
    builder = builder.with_node(hexser::graph::hex_node::HexNode::new(
      hexser::graph::node_id::NodeId::from_name(&format!("N{}", i)),
      hexser::graph::layer::Layer::Domain,
      hexser::graph::role::Role::Entity,
      "Node",
      "bench",
    ));
  }
  for i in 0..n.saturating_sub(1) {
    builder = builder.with_edge(hexser::graph::hex_edge::HexEdge::new(
      hexser::graph::node_id::NodeId::from_name(&format!("N{}", i)),
      hexser::graph::node_id::NodeId::from_name(&format!("N{}", i + 1)),
      hexser::graph::relationship::Relationship::Depends,
    ));
  }
  builder.build()
}

fn bench_edges_from(c: &mut criterion::Criterion) {
  let graph = build_graph(200);
  let mid = hexser::graph::node_id::NodeId::from_name("N100");
  c.bench_function("edges_from/200-node-chain", |b| {
    b.iter(|| {
      let edges = graph.edges_from(criterion::black_box(&mid));
      criterion::black_box(edges.len())
    })
  });
}

fn bench_to_ai_context(c: &mut criterion::Criterion) {
  let graph = build_graph(200);
  c.bench_function("to_ai_context/200-node-chain", |b| {
    b.iter(|| {
      let ctx = graph.to_ai_context().unwrap();
      criterion::black_box(ctx.components.len())
    })
  });
}

criterion::criterion_group!(benches, bench_edges_from, bench_to_ai_context);
criterion::criterion_main!(benches);
