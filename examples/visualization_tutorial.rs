//! Visualization Tutorial
//!
//! Learn to visualize your hexagonal architecture using hex's
//! built-in export capabilities.
//!
//! Run with: cargo run --example visualization_tutorial

fn main() -> hexer::HexResult<()> {
    use hexer::showcase::visualizable::Visualizable;
    println!("Architecture Visualization Tutorial\n");
    println!("{}", "=".repeat(60));

    // Build sample architecture
    let graph = hexer::graph::builder::GraphBuilder::new()
        .with_description("E-commerce System")
        .add_node(hexer::graph::hex_node::HexNode::new(
            hexer::graph::node_id::NodeId::from_name("Product"),
            hexer::graph::layer::Layer::Domain,
            hexer::graph::role::Role::Entity,
            "Product",
            "domain::product",
        ))
        .add_node(hexer::graph::hex_node::HexNode::new(
            hexer::graph::node_id::NodeId::from_name("ProductRepository"),
            hexer::graph::layer::Layer::Port,
            hexer::graph::role::Role::Repository,
            "ProductRepository",
            "ports::product_repository",
        ))
        .add_node(hexer::graph::hex_node::HexNode::new(
            hexer::graph::node_id::NodeId::from_name("PostgresProductRepository"),
            hexer::graph::layer::Layer::Adapter,
            hexer::graph::role::Role::Adapter,
            "PostgresProductRepository",
            "adapters::postgres_product_repository",
        ))
        .add_edge(hexer::graph::hex_edge::HexEdge::new(
            hexer::graph::node_id::NodeId::from_name("ProductRepository"),
            hexer::graph::node_id::NodeId::from_name("Product"),
            hexer::graph::relationship::Relationship::Depends,
        ))
        .add_edge(hexer::graph::hex_edge::HexEdge::new(
            hexer::graph::node_id::NodeId::from_name("PostgresProductRepository"),
            hexer::graph::node_id::NodeId::from_name("ProductRepository"),
            hexer::graph::relationship::Relationship::Implements,
        ))
        .build();

    println!("\n1. Basic Graph Info");
    println!("{}", "-".repeat(60));
    graph.pretty_print();

    println!("\n2. Export to DOT (GraphViz)");
    println!("{}", "-".repeat(60));
    let dot = graph.to_dot()?;
    println!("Generated DOT format ({} bytes)", dot.len());
    println!("\nTo render:");
    println!("  echo '{}' | dot -Tpng > architecture.png", dot.lines().next().unwrap());

    println!("\n3. Export to Mermaid");
    println!("{}", "-".repeat(60));
    let mermaid = graph.to_mermaid()?;
    println!("Generated Mermaid format ({} bytes)", mermaid.len());
    println!("\nTo use:");
    println!("  Copy to markdown:");
    println!("  ```mermaid");
    println!("  {}", mermaid.lines().nth(0).unwrap());
    println!("  ...");
    println!("  ```");

    #[cfg(feature = "visualization")]
    {
        println!("\n4. Export to JSON (D3.js)");
        println!("{}", "-".repeat(60));
        let json = graph.to_json()?;
        println!("Generated JSON format ({} bytes)", json.len());
        println!("\nTo use:");
        println!("  Load in D3.js force graph visualization");
    }

    println!("\n{}", "=".repeat(60));
    println!("Tutorial Complete!");
    println!("\nNext Steps:");
    println!("  1. Save exports to files with std::fs::write");
    println!("  2. Integrate with documentation tools");
    println!("  3. Add to CI/CD for automated diagrams");
    println!("{}", "=".repeat(60));

    Ok(())
}
