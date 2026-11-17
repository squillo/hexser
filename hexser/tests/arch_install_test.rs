//! The process-wide architecture graph's one write site, `HexGraph::install`, exercised in a
//! test process of its own (PRD-272 §1.5; every RCU surface gets a 16-thread cell).
//!
//! This cell installs synthetic graphs. It used to sit among the library's unit tests, which
//! read the registered graph through `HexGraph::current()` on parallel threads, so an install
//! could land between a reader's load and its assertion and the reader saw a graph without its
//! nodes: 37 of 300 local runs failed on 2026-11-17, and two pushes to main failed on CI. An
//! integration test is its own process, so nothing else reads the cell while this runs, and no
//! lock between tests is needed.
//!
//! Revision History
//! - 2026-11-17T19:16:44Z @AI: HEXSER-0.7 — moved here from `src/graph/hex_graph.rs`'s unit tests (with the `serial(hexser_arch)` lock it no longer needs) so no library reader shares a process with an install.

fn node(name: &str) -> hexser::graph::HexNode {
  hexser::graph::HexNode::new(
    hexser::graph::NodeId::from_name(name),
    hexser::graph::Layer::Domain,
    hexser::graph::Role::Entity,
    name,
    "arch_install_test",
  )
}

/// why: the two halves of the RCU contract, in order, in one process nothing else touches.
///
/// First, two back-to-back `current()` loads with no install between them return the same
/// allocation, so request handlers never re-walk `inventory`. Then 16 threads hammer
/// `current()` while one thread installs 64 graphs of increasing size: no reader panics or
/// sees a torn graph, and the final `current()` is the last install. Reading `current()` from a
/// `Mutex` or rebuilding it per call turns the first half red; a store that is not atomic turns
/// the second half red.
#[test]
fn current_is_cached_and_install_is_wait_free_under_sixteen_readers() {
  let first = hexser::HexGraph::current();
  let second = hexser::HexGraph::current();
  std::assert!(
    std::sync::Arc::ptr_eq(&first, &second),
    "current() must return the same graph Arc when nothing was installed between loads"
  );

  let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
  let mut readers = std::vec::Vec::new();
  for _ in 0..16 {
    let stop = std::sync::Arc::clone(&stop);
    readers.push(std::thread::spawn(move || {
      while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        let graph = hexser::HexGraph::current();
        let _ = graph.node_count();
        let _ = graph.edge_count();
      }
    }));
  }

  for size in 1..=64u32 {
    let mut builder = hexser::HexGraph::builder();
    for n in 0..size {
      builder = builder.with_node(node(&std::format!("N{n}")));
    }
    hexser::HexGraph::install(builder.build());
  }

  stop.store(true, std::sync::atomic::Ordering::Relaxed);
  for reader in readers {
    reader.join().expect("a reader thread must not panic");
  }

  std::assert_eq!(
    hexser::HexGraph::current().node_count(),
    64,
    "the final current() must reflect the last install"
  );
}
