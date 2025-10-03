//! Integration tests for graph system.
//!
//! These tests validate the graph core functionality including node and edge
//! creation, graph building, querying, and thread safety. Tests ensure the
//! immutable graph structure works correctly and provides accurate results.

#[cfg(test)]
mod graph_construction {
    #[test]
    fn test_build_simple_graph() {
        let node1 = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Entity"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "UserEntity",
            "domain::user",
        );

        let node2 = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Repository"),
            hexer::graph::Layer::Port,
            hexer::graph::Role::Repository,
            "UserRepository",
            "ports::user",
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .build();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_build_graph_with_edges() {
        let entity_id = hexer::graph::NodeId::from_name("Entity");
        let repo_id = hexer::graph::NodeId::from_name("Repo");
        let adapter_id = hexer::graph::NodeId::from_name("Adapter");

        let entity = hexer::graph::HexNode::new(
            entity_id.clone(),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "User",
            "domain",
        );

        let repo = hexer::graph::HexNode::new(
            repo_id.clone(),
            hexer::graph::Layer::Port,
            hexer::graph::Role::Repository,
            "UserRepo",
            "ports",
        );

        let adapter = hexer::graph::HexNode::new(
            adapter_id.clone(),
            hexer::graph::Layer::Adapter,
            hexer::graph::Role::Adapter,
            "PgUserRepo",
            "adapters",
        );

        let edge1 = hexer::graph::HexEdge::new(
            repo_id.clone(),
            entity_id.clone(),
            hexer::graph::Relationship::Depends,
        );

        let edge2 = hexer::graph::HexEdge::new(
            adapter_id.clone(),
            repo_id.clone(),
            hexer::graph::Relationship::Implements,
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_nodes(vec![entity, repo, adapter])
            .with_edges(vec![edge1, edge2])
            .build();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }
}

#[cfg(test)]
mod graph_queries {
    #[test]
    fn test_query_nodes_by_layer() {
        let domain_node = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Entity"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let port_node = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Repo"),
            hexer::graph::Layer::Port,
            hexer::graph::Role::Repository,
            "Repo",
            "ports",
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_nodes(vec![domain_node, port_node])
            .build();

        let domain_nodes = graph.nodes_by_layer(hexer::graph::Layer::Domain);
        assert_eq!(domain_nodes.len(), 1);

        let port_nodes = graph.nodes_by_layer(hexer::graph::Layer::Port);
        assert_eq!(port_nodes.len(), 1);
    }

    #[test]
    fn test_query_nodes_by_role() {
        let entity = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("E1"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Entity1",
            "domain",
        );

        let value_obj = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("V1"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::ValueObject,
            "ValueObj1",
            "domain",
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_nodes(vec![entity, value_obj])
            .build();

        let entities = graph.nodes_by_role(hexer::graph::Role::Entity);
        assert_eq!(entities.len(), 1);

        let value_objects = graph.nodes_by_role(hexer::graph::Role::ValueObject);
        assert_eq!(value_objects.len(), 1);
    }

    #[test]
    fn test_query_edges_from_node() {
        let source_id = hexer::graph::NodeId::from_name("Source");
        let target1_id = hexer::graph::NodeId::from_name("Target1");
        let target2_id = hexer::graph::NodeId::from_name("Target2");

        let source = hexer::graph::HexNode::new(
            source_id.clone(),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Source",
            "domain",
        );

        let target1 = hexer::graph::HexNode::new(
            target1_id.clone(),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Target1",
            "domain",
        );

        let target2 = hexer::graph::HexNode::new(
            target2_id.clone(),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Target2",
            "domain",
        );

        let edge1 = hexer::graph::HexEdge::new(
            source_id.clone(),
            target1_id,
            hexer::graph::Relationship::Depends,
        );

        let edge2 = hexer::graph::HexEdge::new(
            source_id.clone(),
            target2_id,
            hexer::graph::Relationship::Depends,
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_nodes(vec![source, target1, target2])
            .with_edges(vec![edge1, edge2])
            .build();

        let edges = graph.edges_from(&source_id);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_query_edges_to_node() {
        let source1_id = hexer::graph::NodeId::from_name("Source1");
        let source2_id = hexer::graph::NodeId::from_name("Source2");
        let target_id = hexer::graph::NodeId::from_name("Target");

        let source1 = hexer::graph::HexNode::new(
            source1_id.clone(),
            hexer::graph::Layer::Adapter,
            hexer::graph::Role::Adapter,
            "Source1",
            "adapters",
        );

        let source2 = hexer::graph::HexNode::new(
            source2_id.clone(),
            hexer::graph::Layer::Adapter,
            hexer::graph::Role::Adapter,
            "Source2",
            "adapters",
        );

        let target = hexer::graph::HexNode::new(
            target_id.clone(),
            hexer::graph::Layer::Port,
            hexer::graph::Role::Repository,
            "Target",
            "ports",
        );

        let edge1 = hexer::graph::HexEdge::new(
            source1_id,
            target_id.clone(),
            hexer::graph::Relationship::Implements,
        );

        let edge2 = hexer::graph::HexEdge::new(
            source2_id,
            target_id.clone(),
            hexer::graph::Relationship::Implements,
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_nodes(vec![source1, source2, target])
            .with_edges(vec![edge1, edge2])
            .build();

        let edges = graph.edges_to(&target_id);
        assert_eq!(edges.len(), 2);
    }
}

#[cfg(test)]
mod graph_validation {
    #[test]
    fn test_validation_success() {
        let node_id = hexer::graph::NodeId::from_name("Node");

        let node = hexer::graph::HexNode::new(
            node_id.clone(),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let builder = hexer::graph::GraphBuilder::new().with_node(node);

        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_missing_nodes() {
        let edge = hexer::graph::HexEdge::new(
            hexer::graph::NodeId::from_name("Missing1"),
            hexer::graph::NodeId::from_name("Missing2"),
            hexer::graph::Relationship::Depends,
        );

        let builder = hexer::graph::GraphBuilder::new().with_edge(edge);

        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validated_build_success() {
        let node = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Valid"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Valid",
            "domain",
        );

        let result = hexer::graph::GraphBuilder::new()
            .with_node(node)
            .build_validated();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validated_build_fails() {
        let edge = hexer::graph::HexEdge::new(
            hexer::graph::NodeId::from_name("Invalid"),
            hexer::graph::NodeId::from_name("Missing"),
            hexer::graph::Relationship::Depends,
        );

        let result = hexer::graph::GraphBuilder::new()
            .with_edge(edge)
            .build_validated();

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod graph_thread_safety {
    #[test]
    fn test_graph_cloning_and_sharing() {
        let node = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("Test"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "Test",
            "domain",
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let graph_clone = graph.clone();

        assert_eq!(graph.node_count(), graph_clone.node_count());
    }

    #[test]
    fn test_graph_send_across_threads() {
        let node = hexer::graph::HexNode::new(
            hexer::graph::NodeId::from_name("ThreadTest"),
            hexer::graph::Layer::Domain,
            hexer::graph::Role::Entity,
            "ThreadTest",
            "domain",
        );

        let graph = hexer::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let handle = std::thread::spawn(move || {
            assert_eq!(graph.node_count(), 1);
        });

        handle.join().unwrap();
    }
}
