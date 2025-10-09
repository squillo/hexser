//! EventPublisher port trait for publishing CloudEvents-wrapped domain events.
//!
//! This module defines the EventPublisher port trait for transport-agnostic
//! event publishing. Implementations can target different transports (HTTP, Kafka, AMQP)
//! while maintaining a consistent interface for publishing CloudEvents v1.0 envelopes.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Initial EventPublisher port trait definition.

/// Port trait for publishing CloudEvents-wrapped domain events to a transport.
///
/// EventPublisher defines the interface for publishing events without coupling
/// to specific transport implementations. This enables hexagonal architecture
/// with adapters for different transports (InMemoryEventBus, HTTP, Kafka, etc.).
///
/// # Type Parameter
///
/// - `T`: The domain event type contained in the CloudEvents envelope
///
/// # Methods
///
/// - `publish`: Publishes a single event envelope
/// - `publish_batch`: Publishes multiple event envelopes in a batch
///
/// # Examples
///
/// ```rust
/// use hexser::ports::events::EventPublisher;
///
/// struct MyPublisher;
///
/// impl hexser::ports::events::EventPublisher<std::string::String> for MyPublisher {
///     fn publish(
///         &self,
///         envelope: &hexser::ports::events::CloudEventsEnvelope<std::string::String>,
///     ) -> hexser::HexResult<()> {
///         // Publish logic here
///         std::result::Result::Ok(())
///     }
///
///     fn publish_batch(
///         &self,
///         envelopes: &[hexser::ports::events::CloudEventsEnvelope<std::string::String>],
///     ) -> hexser::HexResult<()> {
///         for envelope in envelopes {
///             self.publish(envelope)?;
///         }
///         std::result::Result::Ok(())
///     }
/// }
///
/// // Usage
/// let publisher = MyPublisher;
/// let envelope = hexser::ports::events::CloudEventsEnvelope::new(
///     std::string::String::from("evt-001"),
///     std::string::String::from("/services/test"),
///     std::string::String::from("com.example.test.event"),
/// );
///
/// publisher.publish(&envelope).unwrap();
/// ```
pub trait EventPublisher<T> {
  /// Publishes a single CloudEvents envelope to the transport.
  ///
  /// This method sends a CloudEvents-wrapped domain event to the underlying
  /// transport mechanism. The implementation handles transport-specific details
  /// such as serialization, connection management, and error handling.
  ///
  /// # Arguments
  ///
  /// * `envelope` - The CloudEvents envelope to publish
  ///
  /// # Returns
  ///
  /// * `Ok(())` if the event was successfully published
  /// * `Err(crate::Hexserror)` if publication failed (network error, validation error, etc.)
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Network connectivity issues
  /// - Serialization failures
  /// - Transport-specific errors (e.g., Kafka partition unavailable)
  /// - Validation failures (invalid CloudEvents attributes)
  ///
  /// # Examples
  ///
  /// ```rust
  /// struct TestEvent {
  ///     id: std::string::String,
  /// }
  ///
  /// impl hexser::domain::DomainEvent for TestEvent {
  ///     fn event_type(&self) -> &str { "com.test.event" }
  ///     fn aggregate_id(&self) -> std::string::String { self.id.clone() }
  /// }
  ///
  /// // Create and publish event
  /// let event = TestEvent {
  ///     id: std::string::String::from("test-123"),
  /// };
  ///
  /// let envelope = hexser::ports::events::CloudEventsEnvelope::from_domain_event(
  ///     std::string::String::from("evt-001"),
  ///     std::string::String::from("/services/test"),
  ///     event,
  /// );
  ///
  /// // publisher.publish(&envelope)?;
  /// ```
  fn publish(&self, envelope: &super::CloudEventsEnvelope<T>) -> crate::HexResult<()>;

  /// Publishes multiple CloudEvents envelopes in a batch operation.
  ///
  /// Batch publishing can improve throughput by reducing overhead for multiple events.
  /// Implementations may optimize batch operations using transport-specific features
  /// (e.g., Kafka batch producer, HTTP/2 multiplexing).
  ///
  /// # Arguments
  ///
  /// * `envelopes` - Slice of CloudEvents envelopes to publish
  ///
  /// # Returns
  ///
  /// * `Ok(())` if all events were successfully published
  /// * `Err(crate::Hexserror)` if any event publication failed
  ///
  /// # Errors
  ///
  /// Implementations should define their batch error semantics:
  /// - Fail-fast: Return error on first failure (remaining events not published)
  /// - Best-effort: Attempt all events and return error if any failed
  /// - Transactional: All-or-nothing guarantee (if supported by transport)
  ///
  /// # Examples
  ///
  /// ```rust
  /// let envelopes = vec![
  ///     hexser::ports::events::CloudEventsEnvelope::<std::string::String>::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/test"),
  ///         std::string::String::from("com.example.test.event"),
  ///     ),
  ///     hexser::ports::events::CloudEventsEnvelope::<std::string::String>::new(
  ///         std::string::String::from("evt-002"),
  ///         std::string::String::from("/services/test"),
  ///         std::string::String::from("com.example.test.event"),
  ///     ),
  /// ];
  ///
  /// // publisher.publish_batch(&envelopes)?;
  /// ```
  fn publish_batch(&self, envelopes: &[super::CloudEventsEnvelope<T>]) -> crate::HexResult<()>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestEvent {
    id: std::string::String,
  }

  impl crate::domain::DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
      "com.test.event"
    }

    fn aggregate_id(&self) -> std::string::String {
      self.id.clone()
    }
  }

  struct MockPublisher {
    published_count: std::cell::RefCell<usize>,
  }

  impl MockPublisher {
    fn new() -> Self {
      Self {
        published_count: std::cell::RefCell::new(0),
      }
    }

    fn get_published_count(&self) -> usize {
      *self.published_count.borrow()
    }
  }

  impl EventPublisher<TestEvent> for MockPublisher {
    fn publish(
      &self,
      envelope: &crate::ports::events::cloud_events_envelope::CloudEventsEnvelope<TestEvent>,
    ) -> crate::HexResult<()> {
      envelope.validate()?;
      *self.published_count.borrow_mut() += 1;
      std::result::Result::Ok(())
    }

    fn publish_batch(
      &self,
      envelopes: &[crate::ports::events::cloud_events_envelope::CloudEventsEnvelope<TestEvent>],
    ) -> crate::HexResult<()> {
      for envelope in envelopes {
        self.publish(envelope)?;
      }
      std::result::Result::Ok(())
    }
  }

  #[test]
  fn test_publish_single_event() {
    let publisher = MockPublisher::new();
    let event = TestEvent {
      id: std::string::String::from("test-123"),
    };

    let envelope = super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    let result = publisher.publish(&envelope);
    std::assert!(result.is_ok());
    std::assert_eq!(publisher.get_published_count(), 1);
  }

  #[test]
  fn test_publish_batch_events() {
    let publisher = MockPublisher::new();

    let envelopes = vec![
      super::super::CloudEventsEnvelope::from_domain_event(
        std::string::String::from("evt-001"),
        std::string::String::from("/test/source"),
        TestEvent {
          id: std::string::String::from("test-1"),
        },
      ),
      super::super::CloudEventsEnvelope::from_domain_event(
        std::string::String::from("evt-002"),
        std::string::String::from("/test/source"),
        TestEvent {
          id: std::string::String::from("test-2"),
        },
      ),
      super::super::CloudEventsEnvelope::from_domain_event(
        std::string::String::from("evt-003"),
        std::string::String::from("/test/source"),
        TestEvent {
          id: std::string::String::from("test-3"),
        },
      ),
    ];

    let result = publisher.publish_batch(&envelopes);
    std::assert!(result.is_ok());
    std::assert_eq!(publisher.get_published_count(), 3);
  }

  #[test]
  fn test_publish_validates_envelope() {
    let publisher = MockPublisher::new();

    // Create invalid envelope (empty id)
    let envelope = super::super::CloudEventsEnvelope::new(
      std::string::String::from(""),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let result = publisher.publish(&envelope);
    std::assert!(result.is_err());
    std::assert_eq!(publisher.get_published_count(), 0);
  }
}
