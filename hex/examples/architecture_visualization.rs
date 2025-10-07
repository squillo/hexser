//! Architecture Visualization Example
//!
//! Demonstrates how hex automatically tracks and visualizes your architecture.
//!
//! Run with: cargo run --example architecture_visualization

use hexser::prelude::*;

// Domain Layer - Core business logic
#[derive(HexDomain, Entity)]
struct Customer {
    id: String,
    name: String,
    email: String,
}

#[derive(HexDomain, Entity)]
struct Order {
    id: String,
    customer_id: String,
    total: f64,
}

#[derive(HexDomain, Entity)]
struct Product {
    id: String,
    name: String,
    price: f64,
}

// Port Layer - Interfaces
#[derive(HexPort, HexRepository)]
struct CustomerRepository;

#[derive(HexPort, HexRepository)]
struct OrderRepository;

#[derive(HexPort, HexRepository)]
struct ProductRepository;

// Adapter Layer - Implementations
#[derive(HexAdapter)]
struct PostgresCustomerRepository {
    connection_string: String,
}

#[derive(HexAdapter)]
struct PostgresOrderRepository {
    connection_string: String,
}

#[derive(HexAdapter)]
struct RedisProductCache {
    redis_url: String,
}

// Application Layer - Use cases
#[derive(HexDirective)]
struct CreateOrderDirective {
    customer_id: String,
    products: Vec<String>,
}

#[derive(HexQuery)]
struct GetCustomerOrdersQuery {
    customer_id: String,
}

fn main() {
    println!("🏗️  hex Architecture Visualization\n");
    println!("{}", "═".repeat(70));

    let graph = HexGraph::current();

    // Overview
    println!("\n📊 ARCHITECTURE OVERVIEW");
    println!("{}", "─".repeat(70));
    graph.pretty_print();

    // Layer-by-layer breakdown
    println!("\n🏛️  LAYER-BY-LAYER BREAKDOWN");
    println!("{}", "─".repeat(70));

    visualize_layer(&graph, Layer::Domain, "📦 Domain Layer (Business Logic)");
    visualize_layer(&graph, Layer::Port, "🔌 Port Layer (Interfaces)");
    visualize_layer(&graph, Layer::Adapter, "🔧 Adapter Layer (Implementations)");
    visualize_layer(&graph, Layer::Application, "⚡ Application Layer (Use Cases)");

    // ASCII Architecture Diagram
    println!("\n🎨 ASCII ARCHITECTURE DIAGRAM");
    println!("{}", "─".repeat(70));
    print_ascii_architecture(&graph);

    // Metrics
    println!("\n📈 ARCHITECTURE METRICS");
    println!("{}", "─".repeat(70));
    println!("Total Components: {}", graph.node_count());
    println!("Domain Entities: {}",
        graph.nodes_by_role(Role::Entity).len());
    println!("Repository Ports: {}",
        graph.nodes_by_layer(Layer::Port).len());
    println!("Adapter Implementations: {}",
        graph.nodes_by_layer(Layer::Adapter).len());
    println!("Directives: {}",
        graph.nodes_by_role(Role::Directive).len());
    println!("Queries: {}",
        graph.nodes_by_role(Role::Query).len());

    // Health Check
    println!("\n✅ ARCHITECTURE HEALTH CHECK");
    println!("{}", "─".repeat(70));
    health_check(&graph);

    println!("{}", "═".repeat(70));
    println!("🎉 Your architecture is automatically documented and visualized!");
    println!("{}", "═".repeat(70));
}

fn visualize_layer(graph: &HexGraph, layer: Layer, title: &str) {
    println!("\n{}", title);
    let nodes: Vec<_> = graph.nodes_by_layer(layer).into_iter().collect();

    if nodes.is_empty() {
        println!("  (no components)");
        return;
    }

    for node in nodes {
        println!("  • {} ({:?})", node.short_name(), node.role());
    }
}

fn print_ascii_architecture(graph: &HexGraph) {
    let domain_count = graph.nodes_by_layer(Layer::Domain).len();
    let port_count = graph.nodes_by_layer(Layer::Port).len();
    let adapter_count = graph.nodes_by_layer(Layer::Adapter).len();
    let app_count = graph.nodes_by_layer(Layer::Application).len();

    println!(r#"
        ┌────────────────────────────────────┐
        │      Application Layer ({:2})        │
        │   [Directives, Queries]            │
        └───────────┬────────────────────────┘
                    │
        ┌───────────▼────────────────────────┐
        │         Port Layer ({:2})            │
        │    [Repository Interfaces]         │
        └───────────┬────────────────────────┘
                    │
            ┌───────┼───────┐
            │       │       │
        ┌───▼──┐ ┌──▼──┐ ┌──▼────┐
        │Domain│ │Adapt│ │Infrast│
        │ ({:2}) │ │ ({:2})│ │  (0)  │
        └──────┘ └─────┘ └───────┘
    "#, app_count, port_count, domain_count, adapter_count);
}

fn health_check(graph: &HexGraph) {
    let domain_count = graph.nodes_by_layer(Layer::Domain).len();
    let port_count = graph.nodes_by_layer(Layer::Port).len();
    let adapter_count = graph.nodes_by_layer(Layer::Adapter).len();

    if domain_count > 0 {
        println!("✅ Domain layer present ({} entities)", domain_count);
    } else {
        println!("⚠️  No domain entities found");
    }

    if port_count > 0 {
        println!("✅ Port layer present ({} ports)", port_count);
    } else {
        println!("⚠️  No ports defined");
    }

    if adapter_count > 0 {
        println!("✅ Adapter layer present ({} adapters)", adapter_count);
    } else {
        println!("⚠️  No adapters implemented");
    }

    if port_count > 0 && adapter_count == 0 {
        println!("💡 Tip: Ports without adapters need implementations!");
    }

    if domain_count == 0 && port_count == 0 && adapter_count == 0 {
        println!("📚 Getting started: Check out the tutorials!");
    }
}
