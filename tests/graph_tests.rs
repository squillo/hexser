//! Integration tests for graph system.
//!
//! These tests validate the graph core functionality including node and edge
//! creation, graph building, querying, and thread safety. Tests ensure the
//! immutable graph structure works correctly and provides accurate results.

#[cfg(test)]
mod graph_construction {
    #[test]
    fn test_build_simple_graph() {
        let node1 = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Entity"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "UserEntity",
            "domain::user",
        );

        let node2 = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Repository"),
            hexser::graph::Layer::Port,
            hexser::graph::Role::Repository,
            "UserRepository",
            "ports::user",
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .build();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_build_graph_with_edges() {
        let entity_id = hexser::graph::NodeId::from_name("Entity");
        let repo_id = hexser::graph::NodeId::from_name("Repo");
        let adapter_id = hexser::graph::NodeId::from_name("Adapter");

        let entity = hexser::graph::HexNode::new(
            entity_id.clone(),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "User",
            "domain",
        );

        let repo = hexser::graph::HexNode::new(
            repo_id.clone(),
            hexser::graph::Layer::Port,
            hexser::graph::Role::Repository,
            "UserRepo",
            "ports",
        );

        let adapter = hexser::graph::HexNode::new(
            adapter_id.clone(),
            hexser::graph::Layer::Adapter,
            hexser::graph::Role::Adapter,
            "PgUserRepo",
            "adapters",
        );

        let edge1 = hexser::graph::HexEdge::new(
            repo_id.clone(),
            entity_id.clone(),
            hexser::graph::Relationship::Depends,
        );

        let edge2 = hexser::graph::HexEdge::new(
            adapter_id.clone(),
            repo_id.clone(),
            hexser::graph::Relationship::Implements,
        );

        let graph = hexser::graph::GraphBuilder::new()
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
        let domain_node = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Entity"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let port_node = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Repo"),
            hexser::graph::Layer::Port,
            hexser::graph::Role::Repository,
            "Repo",
            "ports",
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_nodes(vec![domain_node, port_node])
            .build();

        let domain_nodes = graph.nodes_by_layer(hexser::graph::Layer::Domain);
        assert_eq!(domain_nodes.len(), 1);

        let port_nodes = graph.nodes_by_layer(hexser::graph::Layer::Port);
        assert_eq!(port_nodes.len(), 1);
    }

    #[test]
    fn test_query_nodes_by_role() {
        let entity = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("E1"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Entity1",
            "domain",
        );

        let value_obj = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("V1"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::ValueObject,
            "ValueObj1",
            "domain",
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_nodes(vec![entity, value_obj])
            .build();

        let entities = graph.nodes_by_role(hexser::graph::Role::Entity);
        assert_eq!(entities.len(), 1);

        let value_objects = graph.nodes_by_role(hexser::graph::Role::ValueObject);
        assert_eq!(value_objects.len(), 1);
    }

    #[test]
    fn test_query_edges_from_node() {
        let source_id = hexser::graph::NodeId::from_name("Source");
        let target1_id = hexser::graph::NodeId::from_name("Target1");
        let target2_id = hexser::graph::NodeId::from_name("Target2");

        let source = hexser::graph::HexNode::new(
            source_id.clone(),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Source",
            "domain",
        );

        let target1 = hexser::graph::HexNode::new(
            target1_id.clone(),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Target1",
            "domain",
        );

        let target2 = hexser::graph::HexNode::new(
            target2_id.clone(),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Target2",
            "domain",
        );

        let edge1 = hexser::graph::HexEdge::new(
            source_id.clone(),
            target1_id,
            hexser::graph::Relationship::Depends,
        );

        let edge2 = hexser::graph::HexEdge::new(
            source_id.clone(),
            target2_id,
            hexser::graph::Relationship::Depends,
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_nodes(vec![source, target1, target2])
            .with_edges(vec![edge1, edge2])
            .build();

        let edges = graph.edges_from(&source_id);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_query_edges_to_node() {
        let source1_id = hexser::graph::NodeId::from_name("Source1");
        let source2_id = hexser::graph::NodeId::from_name("Source2");
        let target_id = hexser::graph::NodeId::from_name("Target");

        let source1 = hexser::graph::HexNode::new(
            source1_id.clone(),
            hexser::graph::Layer::Adapter,
            hexser::graph::Role::Adapter,
            "Source1",
            "adapters",
        );

        let source2 = hexser::graph::HexNode::new(
            source2_id.clone(),
            hexser::graph::Layer::Adapter,
            hexser::graph::Role::Adapter,
            "Source2",
            "adapters",
        );

        let target = hexser::graph::HexNode::new(
            target_id.clone(),
            hexser::graph::Layer::Port,
            hexser::graph::Role::Repository,
            "Target",
            "ports",
        );

        let edge1 = hexser::graph::HexEdge::new(
            source1_id,
            target_id.clone(),
            hexser::graph::Relationship::Implements,
        );

        let edge2 = hexser::graph::HexEdge::new(
            source2_id,
            target_id.clone(),
            hexser::graph::Relationship::Implements,
        );

        let graph = hexser::graph::GraphBuilder::new()
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
        let node_id = hexser::graph::NodeId::from_name("Node");

        let node = hexser::graph::HexNode::new(
            node_id.clone(),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let builder = hexser::graph::GraphBuilder::new().with_node(node);

        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_missing_nodes() {
        let edge = hexser::graph::HexEdge::new(
            hexser::graph::NodeId::from_name("Missing1"),
            hexser::graph::NodeId::from_name("Missing2"),
            hexser::graph::Relationship::Depends,
        );

        let builder = hexser::graph::GraphBuilder::new().with_edge(edge);

        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validated_build_success() {
        let node = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Valid"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Valid",
            "domain",
        );

        let result = hexser::graph::GraphBuilder::new()
            .with_node(node)
            .build_validated();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validated_build_fails() {
        let edge = hexser::graph::HexEdge::new(
            hexser::graph::NodeId::from_name("Invalid"),
            hexser::graph::NodeId::from_name("Missing"),
            hexser::graph::Relationship::Depends,
        );

        let result = hexser::graph::GraphBuilder::new()
            .with_edge(edge)
            .build_validated();

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod graph_thread_safety {
    #[test]
    fn test_graph_cloning_and_sharing() {
        let node = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("Test"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "Test",
            "domain",
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let graph_clone = graph.clone();

        assert_eq!(graph.node_count(), graph_clone.node_count());
    }

    #[test]
    fn test_graph_send_across_threads() {
        let node = hexser::graph::HexNode::new(
            hexser::graph::NodeId::from_name("ThreadTest"),
            hexser::graph::Layer::Domain,
            hexser::graph::Role::Entity,
            "ThreadTest",
            "domain",
        );

        let graph = hexser::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let handle = std::thread::spawn(move || {
            assert_eq!(graph.node_count(), 1);
        });

        handle.join().unwrap();
    }
}
