//! Visualization Tutorial
//!
//! Learn to visualize your hexagonal architecture using hex's
//! built-in export capabilities.
//!
//! Run with: cargo run --example visualization_tutorial

fn main() -> hex::HexResult<()> {
    use hex::showcase::visualizable::Visualizable;
    println!("Architecture Visualization Tutorial\n");
    println!("{}", "=".repeat(60));

    // Build sample architecture
    let graph = hex::graph::builder::GraphBuilder::new()
        .with_description("E-commerce System")
        .add_node(hex::graph::hex_node::HexNode::new(
            hex::graph::node_id::NodeId::from_name("Product"),
            hex::graph::layer::Layer::Domain,
            hex::graph::role::Role::Entity,
            "Product",
            "domain::product",
        ))
        .add_node(hex::graph::hex_node::HexNode::new(
            hex::graph::node_id::NodeId::from_name("ProductRepository"),
            hex::graph::layer::Layer::Port,
            hex::graph::role::Role::Repository,
            "ProductRepository",
            "ports::product_repository",
        ))
        .add_node(hex::graph::hex_node::HexNode::new(
            hex::graph::node_id::NodeId::from_name("PostgresProductRepository"),
            hex::graph::layer::Layer::Adapter,
            hex::graph::role::Role::Adapter,
            "PostgresProductRepository",
            "adapters::postgres_product_repository",
        ))
        .add_edge(hex::graph::hex_edge::HexEdge::new(
            hex::graph::node_id::NodeId::from_name("ProductRepository"),
            hex::graph::node_id::NodeId::from_name("Product"),
            hex::graph::relationship::Relationship::Depends,
        ))
        .add_edge(hex::graph::hex_edge::HexEdge::new(
            hex::graph::node_id::NodeId::from_name("PostgresProductRepository"),
            hex::graph::node_id::NodeId::from_name("ProductRepository"),
            hex::graph::relationship::Relationship::Implements,
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
