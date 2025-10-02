//! Graph construction and analysis example.
//!
//! This example demonstrates how to build and analyze a hexagonal architecture
//! graph using hex. It shows node and edge creation, graph construction,
//! and basic queries for analyzing the architecture.
//!
//! Run with: `cargo run --example graph_example`

fn main() -> hex::HexResult<()> {
    println!("=== Hexagonal Architecture Graph Example ===\n");

    // Define node IDs
    let user_entity_id = hex::graph::NodeId::from_name("UserEntity");
    let user_repo_port_id = hex::graph::NodeId::from_name("UserRepositoryPort");
    let pg_adapter_id = hex::graph::NodeId::from_name("PostgresUserRepository");
    let create_directive_id = hex::graph::NodeId::from_name("CreateUserDirective");
    let directive_handler_id = hex::graph::NodeId::from_name("CreateUserHandler");

    // Create nodes
    let user_entity = hex::graph::HexNode::new(
        user_entity_id.clone(),
        hex::graph::Layer::Domain,
        hex::graph::Role::Entity,
        "User",
        "domain::user",
    );

    let user_repo_port = hex::graph::HexNode::new(
        user_repo_port_id.clone(),
        hex::graph::Layer::Port,
        hex::graph::Role::Repository,
        "UserRepository",
        "ports::user_repository",
    );

    let pg_adapter = hex::graph::HexNode::new(
        pg_adapter_id.clone(),
        hex::graph::Layer::Adapter,
        hex::graph::Role::Adapter,
        "PostgresUserRepository",
        "adapters::postgres::user_repository",
    );

    let create_directive = hex::graph::HexNode::new(
        create_directive_id.clone(),
        hex::graph::Layer::Application,
        hex::graph::Role::Directive,
        "CreateUserDirective",
        "application::directives::create_user",
    );

    let directive_handler = hex::graph::HexNode::new(
        directive_handler_id.clone(),
        hex::graph::Layer::Application,
        hex::graph::Role::DirectiveHandler,
        "CreateUserHandler",
        "application::handlers::create_user",
    );

    // Create edges
    let repo_depends_on_entity = hex::graph::HexEdge::new(
        user_repo_port_id.clone(),
        user_entity_id.clone(),
        hex::graph::Relationship::Depends,
    );

    let adapter_implements_port = hex::graph::HexEdge::new(
        pg_adapter_id.clone(),
        user_repo_port_id.clone(),
        hex::graph::Relationship::Implements,
    );

    let handler_invokes_directive = hex::graph::HexEdge::new(
        directive_handler_id.clone(),
        create_directive_id.clone(),
        hex::graph::Relationship::Invokes,
    );

    let handler_depends_on_repo = hex::graph::HexEdge::new(
        directive_handler_id.clone(),
        user_repo_port_id.clone(),
        hex::graph::Relationship::Depends,
    );

    // Build graph
    println!("Building hexagonal architecture graph...");
    let graph = hex::graph::GraphBuilder::new()
        .with_description("User Management Architecture")
        .with_nodes(vec![
            user_entity,
            user_repo_port,
            pg_adapter,
            create_directive,
            directive_handler,
        ])
        .with_edges(vec![
            repo_depends_on_entity,
            adapter_implements_port,
            handler_invokes_directive,
            handler_depends_on_repo,
        ])
        .build();

    println!("✓ Graph constructed successfully\n");

    // Analyze graph
    println!("=== Graph Statistics ===");
    println!("Total nodes: {}", graph.node_count());
    println!("Total edges: {}", graph.edge_count());
    println!();

    // Query by layer
    println!("=== Nodes by Layer ===");
    for layer in &[
        hex::graph::Layer::Domain,
        hex::graph::Layer::Port,
        hex::graph::Layer::Adapter,
        hex::graph::Layer::Application,
    ] {
        let nodes = graph.nodes_by_layer(*layer);
        println!("{}: {} node(s)", layer, nodes.len());
        for node in nodes {
            println!("  - {}", node.type_name());
        }
    }
    println!();

    // Query by role
    println!("=== Nodes by Role ===");
    let roles = vec![
        hex::graph::Role::Entity,
        hex::graph::Role::Repository,
        hex::graph::Role::Adapter,
        hex::graph::Role::Directive,
        hex::graph::Role::DirectiveHandler,
    ];

    for role in roles {
        let nodes = graph.nodes_by_role(role);
        if !nodes.is_empty() {
            println!("{}: {} node(s)", role, nodes.len());
            for node in nodes {
                println!("  - {}", node.type_name());
            }
        }
    }
    println!();

    // Analyze relationships
    println!("=== Relationship Analysis ===");
    println!("Adapter implementations:");
    let adapters = graph.nodes_by_role(hex::graph::Role::Adapter);
    for adapter in adapters {
        let implementations = graph.edges_from(adapter.id());
        for edge in implementations {
            if edge.relationship() == hex::graph::Relationship::Implements {
                if let Some(port) = graph.get_node(edge.target()) {
                    println!("  {} implements {}", adapter.type_name(), port.type_name());
                }
            }
        }
    }
    println!();

    println!("Handler dependencies:");
    let handlers = graph.nodes_by_role(hex::graph::Role::DirectiveHandler);
    for handler in handlers {
        let deps = graph.edges_from(handler.id());
        println!("  {}:", handler.type_name());
        for edge in deps {
            if let Some(dep) = graph.get_node(edge.target()) {
                println!("    {} {}", edge.relationship(), dep.type_name());
            }
        }
    }

    println!("\n✅ Graph analysis complete!");
    println!("\nThis graph can be used for:");
    println!("  - Architectural validation");
    println!("  - Dependency analysis");
    println!("  - Visualization generation");
    println!("  - Intent inference (Phase 7)");

    Ok(())
}
