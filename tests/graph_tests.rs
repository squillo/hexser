//! Integration tests for graph system.
//!
//! These tests validate the graph core functionality including node and edge
//! creation, graph building, querying, and thread safety. Tests ensure the
//! immutable graph structure works correctly and provides accurate results.

#[cfg(test)]
mod graph_construction {
    #[test]
    fn test_build_simple_graph() {
        let node1 = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Entity"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "UserEntity",
            "domain::user",
        );

        let node2 = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Repository"),
            hex::graph::Layer::Port,
            hex::graph::Role::Repository,
            "UserRepository",
            "ports::user",
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .build();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_build_graph_with_edges() {
        let entity_id = hex::graph::NodeId::from_name("Entity");
        let repo_id = hex::graph::NodeId::from_name("Repo");
        let adapter_id = hex::graph::NodeId::from_name("Adapter");

        let entity = hex::graph::HexNode::new(
            entity_id.clone(),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "User",
            "domain",
        );

        let repo = hex::graph::HexNode::new(
            repo_id.clone(),
            hex::graph::Layer::Port,
            hex::graph::Role::Repository,
            "UserRepo",
            "ports",
        );

        let adapter = hex::graph::HexNode::new(
            adapter_id.clone(),
            hex::graph::Layer::Adapter,
            hex::graph::Role::Adapter,
            "PgUserRepo",
            "adapters",
        );

        let edge1 = hex::graph::HexEdge::new(
            repo_id.clone(),
            entity_id.clone(),
            hex::graph::Relationship::Depends,
        );

        let edge2 = hex::graph::HexEdge::new(
            adapter_id.clone(),
            repo_id.clone(),
            hex::graph::Relationship::Implements,
        );

        let graph = hex::graph::GraphBuilder::new()
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
        let domain_node = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Entity"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let port_node = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Repo"),
            hex::graph::Layer::Port,
            hex::graph::Role::Repository,
            "Repo",
            "ports",
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_nodes(vec![domain_node, port_node])
            .build();

        let domain_nodes = graph.nodes_by_layer(hex::graph::Layer::Domain);
        assert_eq!(domain_nodes.len(), 1);

        let port_nodes = graph.nodes_by_layer(hex::graph::Layer::Port);
        assert_eq!(port_nodes.len(), 1);
    }

    #[test]
    fn test_query_nodes_by_role() {
        let entity = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("E1"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Entity1",
            "domain",
        );

        let value_obj = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("V1"),
            hex::graph::Layer::Domain,
            hex::graph::Role::ValueObject,
            "ValueObj1",
            "domain",
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_nodes(vec![entity, value_obj])
            .build();

        let entities = graph.nodes_by_role(hex::graph::Role::Entity);
        assert_eq!(entities.len(), 1);

        let value_objects = graph.nodes_by_role(hex::graph::Role::ValueObject);
        assert_eq!(value_objects.len(), 1);
    }

    #[test]
    fn test_query_edges_from_node() {
        let source_id = hex::graph::NodeId::from_name("Source");
        let target1_id = hex::graph::NodeId::from_name("Target1");
        let target2_id = hex::graph::NodeId::from_name("Target2");

        let source = hex::graph::HexNode::new(
            source_id.clone(),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Source",
            "domain",
        );

        let target1 = hex::graph::HexNode::new(
            target1_id.clone(),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Target1",
            "domain",
        );

        let target2 = hex::graph::HexNode::new(
            target2_id.clone(),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Target2",
            "domain",
        );

        let edge1 = hex::graph::HexEdge::new(
            source_id.clone(),
            target1_id,
            hex::graph::Relationship::Depends,
        );

        let edge2 = hex::graph::HexEdge::new(
            source_id.clone(),
            target2_id,
            hex::graph::Relationship::Depends,
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_nodes(vec![source, target1, target2])
            .with_edges(vec![edge1, edge2])
            .build();

        let edges = graph.edges_from(&source_id);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_query_edges_to_node() {
        let source1_id = hex::graph::NodeId::from_name("Source1");
        let source2_id = hex::graph::NodeId::from_name("Source2");
        let target_id = hex::graph::NodeId::from_name("Target");

        let source1 = hex::graph::HexNode::new(
            source1_id.clone(),
            hex::graph::Layer::Adapter,
            hex::graph::Role::Adapter,
            "Source1",
            "adapters",
        );

        let source2 = hex::graph::HexNode::new(
            source2_id.clone(),
            hex::graph::Layer::Adapter,
            hex::graph::Role::Adapter,
            "Source2",
            "adapters",
        );

        let target = hex::graph::HexNode::new(
            target_id.clone(),
            hex::graph::Layer::Port,
            hex::graph::Role::Repository,
            "Target",
            "ports",
        );

        let edge1 = hex::graph::HexEdge::new(
            source1_id,
            target_id.clone(),
            hex::graph::Relationship::Implements,
        );

        let edge2 = hex::graph::HexEdge::new(
            source2_id,
            target_id.clone(),
            hex::graph::Relationship::Implements,
        );

        let graph = hex::graph::GraphBuilder::new()
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
        let node_id = hex::graph::NodeId::from_name("Node");

        let node = hex::graph::HexNode::new(
            node_id.clone(),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Entity",
            "domain",
        );

        let builder = hex::graph::GraphBuilder::new().with_node(node);

        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_missing_nodes() {
        let edge = hex::graph::HexEdge::new(
            hex::graph::NodeId::from_name("Missing1"),
            hex::graph::NodeId::from_name("Missing2"),
            hex::graph::Relationship::Depends,
        );

        let builder = hex::graph::GraphBuilder::new().with_edge(edge);

        assert!(builder.validate().is_err());
    }

    #[test]
    fn test_validated_build_success() {
        let node = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Valid"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Valid",
            "domain",
        );

        let result = hex::graph::GraphBuilder::new()
            .with_node(node)
            .build_validated();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validated_build_fails() {
        let edge = hex::graph::HexEdge::new(
            hex::graph::NodeId::from_name("Invalid"),
            hex::graph::NodeId::from_name("Missing"),
            hex::graph::Relationship::Depends,
        );

        let result = hex::graph::GraphBuilder::new()
            .with_edge(edge)
            .build_validated();

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod graph_thread_safety {
    #[test]
    fn test_graph_cloning_and_sharing() {
        let node = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("Test"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "Test",
            "domain",
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let graph_clone = graph.clone();

        assert_eq!(graph.node_count(), graph_clone.node_count());
    }

    #[test]
    fn test_graph_send_across_threads() {
        let node = hex::graph::HexNode::new(
            hex::graph::NodeId::from_name("ThreadTest"),
            hex::graph::Layer::Domain,
            hex::graph::Role::Entity,
            "ThreadTest",
            "domain",
        );

        let graph = hex::graph::GraphBuilder::new()
            .with_node(node)
            .build();

        let handle = std::thread::spawn(move || {
            assert_eq!(graph.node_count(), 1);
        });

        handle.join().unwrap();
    }
}
