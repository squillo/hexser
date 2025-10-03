//! Graph construction and analysis example.
//!
//! This example demonstrates how to build and analyze a hexagonal architecture
//! graph using hex. It shows node and edge creation, graph construction,
//! and basic queries for analyzing the architecture.
//!
//! Run with: `cargo run --example graph_example`

fn main() -> hexer::HexResult<()> {
    println!("=== Hexagonal Architecture Graph Example ===\n");

    // Define node IDs
    let user_entity_id = hexer::graph::NodeId::from_name("UserEntity");
    let user_repo_port_id = hexer::graph::NodeId::from_name("UserRepositoryPort");
    let pg_adapter_id = hexer::graph::NodeId::from_name("PostgresUserRepository");
    let create_directive_id = hexer::graph::NodeId::from_name("CreateUserDirective");
    let directive_handler_id = hexer::graph::NodeId::from_name("CreateUserHandler");

    // Create nodes
    let user_entity = hexer::graph::HexNode::new(
        user_entity_id.clone(),
        hexer::graph::Layer::Domain,
        hexer::graph::Role::Entity,
        "User",
        "domain::user",
    );

    let user_repo_port = hexer::graph::HexNode::new(
        user_repo_port_id.clone(),
        hexer::graph::Layer::Port,
        hexer::graph::Role::Repository,
        "UserRepository",
        "ports::user_repository",
    );

    let pg_adapter = hexer::graph::HexNode::new(
        pg_adapter_id.clone(),
        hexer::graph::Layer::Adapter,
        hexer::graph::Role::Adapter,
        "PostgresUserRepository",
        "adapters::postgres::user_repository",
    );

    let create_directive = hexer::graph::HexNode::new(
        create_directive_id.clone(),
        hexer::graph::Layer::Application,
        hexer::graph::Role::Directive,
        "CreateUserDirective",
        "application::directives::create_user",
    );

    let directive_handler = hexer::graph::HexNode::new(
        directive_handler_id.clone(),
        hexer::graph::Layer::Application,
        hexer::graph::Role::DirectiveHandler,
        "CreateUserHandler",
        "application::handlers::create_user",
    );

    // Create edges
    let repo_depends_on_entity = hexer::graph::HexEdge::new(
        user_repo_port_id.clone(),
        user_entity_id.clone(),
        hexer::graph::Relationship::Depends,
    );

    let adapter_implements_port = hexer::graph::HexEdge::new(
        pg_adapter_id.clone(),
        user_repo_port_id.clone(),
        hexer::graph::Relationship::Implements,
    );

    let handler_invokes_directive = hexer::graph::HexEdge::new(
        directive_handler_id.clone(),
        create_directive_id.clone(),
        hexer::graph::Relationship::Invokes,
    );

    let handler_depends_on_repo = hexer::graph::HexEdge::new(
        directive_handler_id.clone(),
        user_repo_port_id.clone(),
        hexer::graph::Relationship::Depends,
    );

    // Build graph
    println!("Building hexagonal architecture graph...");
    let graph = hexer::graph::GraphBuilder::new()
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
        hexer::graph::Layer::Domain,
        hexer::graph::Layer::Port,
        hexer::graph::Layer::Adapter,
        hexer::graph::Layer::Application,
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
        hexer::graph::Role::Entity,
        hexer::graph::Role::Repository,
        hexer::graph::Role::Adapter,
        hexer::graph::Role::Directive,
        hexer::graph::Role::DirectiveHandler,
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
    let adapters = graph.nodes_by_role(hexer::graph::Role::Adapter);
    for adapter in adapters {
        let implementations = graph.edges_from(adapter.id());
        for edge in implementations {
            if edge.relationship() == hexer::graph::Relationship::Implements {
                if let Some(port) = graph.get_node(edge.target()) {
                    println!("  {} implements {}", adapter.type_name(), port.type_name());
                }
            }
        }
    }
    println!();

    println!("Handler dependencies:");
    let handlers = graph.nodes_by_role(hexer::graph::Role::DirectiveHandler);
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
