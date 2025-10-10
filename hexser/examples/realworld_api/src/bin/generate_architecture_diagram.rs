//! Architecture diagram generator for RealWorld API.
//!
//! Generates a Mermaid diagram of the complete hexagonal architecture
//! by introspecting all registered hexser components. Also generates an
//! AI Agent Pack JSON file containing architecture metadata for AI tools.
//! Outputs to both console and files that can be embedded in documentation.
//!
//! Run with: `cargo run --bin generate_architecture_diagram`
//!
//! Revision History
//! - 2025-10-10T14:42:00Z @AI: Add AI Agent Pack JSON generation alongside Mermaid diagram.
//! - 2025-10-10T11:12:00Z @AI: Initial implementation of architecture diagram generator.

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let _ = realworld_api::domain::user::User::new(
        std::string::String::from("trigger"),
        std::string::String::from("trigger@example.com"),
        std::string::String::from("trigger"),
        std::string::String::from("trigger"),
    );
    println!("=== RealWorld API - Architecture Diagram Generator ===\n");

    println!("🔍 Collecting architecture components...");
    let graph = hexser::graph::hex_graph::HexGraph::current();

    println!("✓ Found {} components\n", graph.node_count());

    println!("📊 Architecture Statistics:");
    println!("  - Domain entities: {}", graph.nodes_by_layer(hexser::graph::layer::Layer::Domain).len());
    println!("  - Ports: {}", graph.nodes_by_layer(hexser::graph::layer::Layer::Port).len());
    println!("  - Adapters: {}", graph.nodes_by_layer(hexser::graph::layer::Layer::Adapter).len());
    println!("  - Application use cases: {}", graph.nodes_by_layer(hexser::graph::layer::Layer::Application).len());
    println!();

    println!("🎨 Generating Mermaid diagram...");

    let exporter = hexser::graph::visualization::adapters::mermaid_exporter::MermaidExporter::new();
    let export_use_case = hexser::graph::visualization::application::export_graph::ExportGraph::new(&exporter);

    let visual_style = hexser::graph::visualization::domain::visual_style::VisualStyle::default();
    let mermaid_output = export_use_case.execute(&graph, visual_style)?;

    println!("✓ Diagram generated\n");

    let output_file = "architecture_diagram.mmd";
    std::fs::write(output_file, &mermaid_output)?;
    println!("✅ Saved Mermaid diagram to: {}", output_file);

    println!("\n🤖 Generating AI Agent Pack JSON...");

    let agent_pack = hexser::ai::AgentPack::from_graph_with_defaults(std::sync::Arc::as_ref(&graph))?;
    let json_output = agent_pack.to_json().map_err(|e| {
        std::format!("Failed to serialize AI pack: {}", e)
    })?;

    let json_file = "architecture_ai_pack.json";
    std::fs::write(json_file, &json_output)?;
    println!("✅ Saved AI pack JSON to: {}", json_file);

    println!("\n📋 To embed Mermaid diagram in README.md, use:\n");
    println!("```mermaid");
    println!("{}", mermaid_output);
    println!("```");

    println!("\n🎉 Architecture visualization generation complete!");
    println!("   - Mermaid diagram: {}", output_file);
    println!("   - AI Agent Pack: {}", json_file);

    std::result::Result::Ok(())
}
