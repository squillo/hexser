//! EventRouter port trait for routing CloudEvents to topics and subjects.
//!
//! This module defines the EventRouter port trait for transport-agnostic
//! routing of CloudEvents based on event types and aggregate IDs.
//! Implementations can define routing strategies (topic conventions, partitioning)
//! without coupling to specific transports.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Initial EventRouter port trait definition.

/// Port trait for routing CloudEvents to topics and subjects.
///
/// EventRouter defines the interface for resolving routing destinations
/// (topics, channels, queues) and subjects (partitioning keys) based on
/// event attributes. This enables flexible routing strategies without
/// coupling the domain or application logic to transport-specific routing.
///
/// # Methods
///
/// - `resolve_topic`: Resolves the topic/channel/queue for an event type
/// - `resolve_subject`: Resolves the subject/partition key for an aggregate ID
///
/// # Examples
///
/// ```rust
/// use hexser::ports::events::EventRouter;
///
/// struct MyRouter;
///
/// impl hexser::ports::events::EventRouter for MyRouter {
///     fn resolve_topic(&self, event_type: &str) -> hexser::HexResult<std::string::String> {
///         // Routing logic here
///         let topic = format!("{}.events", event_type.split('.').next().unwrap_or("default"));
///         std::result::Result::Ok(topic)
///     }
///
///     fn resolve_subject(&self, aggregate_id: &str) -> std::option::Option<std::string::String> {
///         // Subject logic here
///         std::option::Option::Some(std::string::String::from(aggregate_id))
///     }
/// }
///
/// // Usage
/// let router = MyRouter;
/// let topic = router.resolve_topic("com.example.user.created").unwrap();
/// let subject = router.resolve_subject("user-123");
/// ```
pub trait EventRouter {
  /// Resolves the topic/channel/queue name for an event type.
  ///
  /// This method determines the routing destination based on the event type.
  /// Common strategies include:
  /// - Domain-based: Extract domain from reverse-DNS type (com.example → example.events)
  /// - Entity-based: Route by entity type (user.created → user.events)
  /// - Flat: All events to single topic
  /// - Hierarchical: Build topic hierarchy from type segments
  ///
  /// # Arguments
  ///
  /// * `event_type` - The CloudEvents type attribute (e.g., "com.example.user.created")
  ///
  /// # Returns
  ///
  /// * `Ok(String)` - The resolved topic/channel/queue name
  /// * `Err(crate::Hexserror)` - If routing resolution fails
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Invalid event type format
  /// - Unknown/unmapped event types
  /// - Routing policy violations
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Domain-based routing
  /// // router.resolve_topic("com.example.user.created")?;
  /// // Returns: "example.events"
  ///
  /// // Entity-based routing
  /// // router.resolve_topic("com.example.user.created")?;
  /// // Returns: "user.events"
  ///
  /// // Hierarchical routing
  /// // router.resolve_topic("com.example.user.created")?;
  /// // Returns: "com/example/user/events"
  /// ```
  fn resolve_topic(&self, event_type: &str) -> crate::HexResult<std::string::String>;

  /// Resolves the subject/partition key for an aggregate ID.
  ///
  /// This method determines the subject or partitioning key based on the
  /// aggregate ID. The subject can be used for:
  /// - Kafka partitioning: Ensure events for same aggregate go to same partition
  /// - Message ordering: Maintain event order per aggregate
  /// - Filtering: Enable subject-based event filtering
  /// - Multi-tenancy: Incorporate tenant ID into subject
  ///
  /// # Arguments
  ///
  /// * `aggregate_id` - The aggregate identifier from domain event
  ///
  /// # Returns
  ///
  /// * `Some(String)` - The resolved subject/partition key
  /// * `None` - If no subject mapping is needed
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Direct mapping
  /// // router.resolve_subject("user-123");
  /// // Returns: Some("user-123")
  ///
  /// // Prefixed mapping
  /// // router.resolve_subject("user-123");
  /// // Returns: Some("aggregate:user-123")
  ///
  /// // Hash-based partitioning
  /// // router.resolve_subject("user-123");
  /// // Returns: Some("partition-5")
  ///
  /// // No subject
  /// // router.resolve_subject("user-123");
  /// // Returns: None
  /// ```
  fn resolve_subject(&self, aggregate_id: &str) -> std::option::Option<std::string::String>;
}

#[cfg(test)]
mod tests {
  use super::*;

  // Simple domain-based router
  struct DomainRouter;

  impl EventRouter for DomainRouter {
    fn resolve_topic(&self, event_type: &str) -> crate::HexResult<std::string::String> {
      // Extract domain from reverse-DNS format (com.example.user.created -> example)
      let parts: std::vec::Vec<&str> = event_type.split('.').collect();

      if parts.len() < 2 {
        return std::result::Result::Err(crate::Hexserror::validation(
          "Event type must be in reverse-DNS format",
        ));
      }

      let domain = parts[1];
      let topic = format!("{}.events", domain);
      std::result::Result::Ok(topic)
    }

    fn resolve_subject(&self, aggregate_id: &str) -> std::option::Option<std::string::String> {
      std::option::Option::Some(std::string::String::from(aggregate_id))
    }
  }

  // Entity-based router
  struct EntityRouter;

  impl EventRouter for EntityRouter {
    fn resolve_topic(&self, event_type: &str) -> crate::HexResult<std::string::String> {
      // Extract entity from type (com.example.user.created -> user)
      let parts: std::vec::Vec<&str> = event_type.split('.').collect();

      if parts.len() < 3 {
        return std::result::Result::Err(crate::Hexserror::validation(
          "Event type must have entity segment",
        ));
      }

      let entity = parts[2];
      let topic = format!("{}.events", entity);
      std::result::Result::Ok(topic)
    }

    fn resolve_subject(&self, aggregate_id: &str) -> std::option::Option<std::string::String> {
      let subject = format!("aggregate:{}", aggregate_id);
      std::option::Option::Some(subject)
    }
  }

  // Flat router (all events to single topic)
  struct FlatRouter;

  impl EventRouter for FlatRouter {
    fn resolve_topic(&self, _event_type: &str) -> crate::HexResult<std::string::String> {
      std::result::Result::Ok(std::string::String::from("all.events"))
    }

    fn resolve_subject(&self, _aggregate_id: &str) -> std::option::Option<std::string::String> {
      std::option::Option::None
    }
  }

  #[test]
  fn test_domain_router_resolves_topic() {
    let router = DomainRouter;

    let topic = router.resolve_topic("com.example.user.created").unwrap();
    std::assert_eq!(topic, "example.events");
  }

  #[test]
  fn test_domain_router_resolves_subject() {
    let router = DomainRouter;

    let subject = router.resolve_subject("user-123");
    std::assert_eq!(
      subject,
      std::option::Option::Some(std::string::String::from("user-123"))
    );
  }

  #[test]
  fn test_domain_router_rejects_invalid_format() {
    let router = DomainRouter;

    let result = router.resolve_topic("invalid");
    std::assert!(result.is_err());
  }

  #[test]
  fn test_entity_router_resolves_topic() {
    let router = EntityRouter;

    let topic = router.resolve_topic("com.example.user.created").unwrap();
    std::assert_eq!(topic, "user.events");
  }

  #[test]
  fn test_entity_router_resolves_subject_with_prefix() {
    let router = EntityRouter;

    let subject = router.resolve_subject("user-123");
    std::assert_eq!(
      subject,
      std::option::Option::Some(std::string::String::from("aggregate:user-123"))
    );
  }

  #[test]
  fn test_entity_router_rejects_invalid_format() {
    let router = EntityRouter;

    let result = router.resolve_topic("com.example");
    std::assert!(result.is_err());
  }

  #[test]
  fn test_flat_router_uses_single_topic() {
    let router = FlatRouter;

    let topic1 = router.resolve_topic("com.example.user.created").unwrap();
    let topic2 = router.resolve_topic("com.example.order.shipped").unwrap();

    std::assert_eq!(topic1, "all.events");
    std::assert_eq!(topic2, "all.events");
  }

  #[test]
  fn test_flat_router_returns_no_subject() {
    let router = FlatRouter;

    let subject = router.resolve_subject("user-123");
    std::assert_eq!(subject, std::option::Option::None);
  }

  #[test]
  fn test_multiple_event_types_routing() {
    let router = DomainRouter;

    let topic1 = router.resolve_topic("com.acme.user.created").unwrap();
    let topic2 = router.resolve_topic("com.widgets.order.shipped").unwrap();

    std::assert_eq!(topic1, "acme.events");
    std::assert_eq!(topic2, "widgets.events");
  }
}
