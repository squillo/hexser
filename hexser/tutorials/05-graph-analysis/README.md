# Tutorial 05: Graph Analysis (15 minutes)

## Goal
Master runtime introspection using the HexGraph to understand, verify, and communicate your architecture.

## Key Concepts
- HexGraph provides runtime architecture introspection (components and links)
- Query the graph by layer and simple predicates to answer design questions
- Detect smells early (missing layers, unexpected coupling)
- Pretty-print the graph to the console for quick visualization

## Prerequisites
- Completed Tutorials 01–04
- Rust 2024 toolchain installed

---

## 1) The Power of Introspection
The HexGraph is built incrementally as your application starts. Each component (domain types, ports, adapters, application items) registers itself. You can then query the current graph at any point to:
- Count components per layer
- Verify that all intended layers are present
- Quickly spot unexpected coupling when counts change over time

> Tip: The graph is most useful when you run a program that exercises multiple parts of your code (e.g., an example demonstrating several layers).

---

## 2) Quickstart: Build a Graph
Use an existing example that touches multiple layers. Run one of the examples that already prints the graph:

```bash
# From the repository root
cargo run --example tutorial_03_adapters
```

You should see a section like:
```
All three layers registered:
<pretty graph output>
```

Another good option is the application example:
```bash
cargo run --example tutorial_04_application
```
This exercises the Application layer as well.

---

## 3) Basic Queries in Code
You can query the graph directly from your code. Here is a minimal snippet that you can paste into any example (e.g., at the end of main):

```rust
fn print_arch_summary() {
    let graph = hexser::HexGraph::current();

    // Pretty print the entire graph
    graph.pretty_print();

    // Count components by layer
    let domain_count = graph.nodes_by_layer(hexser::Layer::Domain).len();
    let ports_count = graph.nodes_by_layer(hexser::Layer::Port).len();
    let adapters_count = graph.nodes_by_layer(hexser::Layer::Adapter).len();
    let app_count = graph.nodes_by_layer(hexser::Layer::Application).len();

    println!("\nArchitecture summary:");
    println!("  Domain: {}", domain_count);
    println!("  Ports: {}", ports_count);
    println!("  Adapters: {}", adapters_count);
    println!("  Application: {}", app_count);
}
```

> The methods shown above are used in several examples in the repo. They are safe, require no special setup, and work at runtime.

---

## 4) Answer Real Questions with the Graph
Here are common questions you can answer quickly:

- Do we have any components registered in every layer?
  - Check that each `nodes_by_layer(Layer::X).len()` is greater than 0.
- Did the number of Adapters unexpectedly increase after a refactor?
  - Compare counts between runs or print them in CI.
- Are we only using the read side (queries) in a sample?
  - Expect higher counts in Adapters/Ports and lower in Application.

Even these simple checks provide feedback loops that guide healthy architecture growth.

---

## 5) Suggested Workflow for Teams
- Add a small runtime check in your binaries that prints an architecture summary.
- Track counts over time (paste into PRs or CI logs).
- When counts change unexpectedly, inspect the graph output and your recent code changes.

---

## 6) Where to Look in This Repository
- examples/tutorial_03_adapters.rs – Builds domain, ports, and adapters; prints the graph
- examples/tutorial_04_application.rs – Adds the application layer and prints a summary
- examples/simple_todo.rs – Minimal flow using repositories and queries

These are great places to experiment with additional graph prints and summaries.

---

## 7) What’s Next?
- Extend your examples to register more ports and adapters; watch the counts change.
- Pair this with CQRS (Tutorial 04) to confirm that read/write paths register as intended.
- Consider exporting counts to metrics in production for long-term architectural visibility.
