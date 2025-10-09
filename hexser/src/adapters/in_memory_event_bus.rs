//! InMemoryEventBus adapter for CloudEvents publishing and subscription.
//!
//! This module provides a simple in-memory event bus adapter that implements
//! both EventPublisher and EventSubscriber ports. It is intended for testing,
//! development, and as a reference implementation. Events are stored in memory
//! and delivered synchronously without persistence.
//!
//! Revision History
//! - 2025-10-09T15:08:00Z @AI: Fix doc test to use trait imports for subscribe/publish methods.
//! - 2025-10-09T14:51:00Z @AI: Initial InMemoryEventBus adapter implementation.

/// Simple in-memory event bus for testing and development.
///
/// InMemoryEventBus provides a synchronous, non-persistent event bus that
/// implements both EventPublisher and EventSubscriber ports. Events are
/// stored in a queue and delivered via polling or handler invocation.
///
/// # Characteristics
///
/// - **Synchronous**: Events are delivered immediately on publish
/// - **In-memory**: No persistence, events lost on drop
/// - **Topic-based**: Events routed by topic to registered handlers
/// - **Single-threaded**: No concurrency support (uses RefCell)
/// - **Testing-focused**: Designed for unit and integration tests
///
/// # Type Parameter
///
/// - `T`: The domain event type contained in CloudEvents envelopes
///
/// # Examples
///
/// ```rust
/// use hexser::ports::events::EventPublisher;
/// use hexser::ports::events::EventSubscriber;
///
/// #[derive(Clone)]
/// struct TestEvent {
///     id: std::string::String,
/// }
///
/// impl hexser::domain::DomainEvent for TestEvent {
///     fn event_type(&self) -> &str { "com.test.event" }
///     fn aggregate_id(&self) -> std::string::String { self.id.clone() }
/// }
///
/// // Create event bus
/// let mut bus: hexser::adapters::InMemoryEventBus<TestEvent> =
///     hexser::adapters::InMemoryEventBus::new();
///
/// // Subscribe to events (requires EventSubscriber trait in scope)
/// bus.subscribe(
///     "test.events",
///     std::boxed::Box::new(|_envelope| {
///         // Handle event
///         std::result::Result::Ok(())
///     }),
/// ).unwrap();
///
/// // Publish event (requires EventPublisher trait in scope)
/// let event = TestEvent {
///     id: std::string::String::from("test-123"),
/// };
///
/// let envelope = hexser::ports::events::CloudEventsEnvelope::from_domain_event(
///     std::string::String::from("evt-001"),
///     std::string::String::from("/test/source"),
///     event,
/// );
///
/// bus.publish(&envelope).unwrap();
/// ```
pub struct InMemoryEventBus<T> {
  queue: std::cell::RefCell<std::vec::Vec<crate::ports::events::CloudEventsEnvelope<T>>>,
  handlers: std::cell::RefCell<
    std::collections::HashMap<
      std::string::String,
      std::boxed::Box<dyn Fn(crate::ports::events::CloudEventsEnvelope<T>) -> crate::HexResult<()>>,
    >,
  >,
  topic: std::string::String,
}

impl<T> InMemoryEventBus<T> {
  /// Creates a new InMemoryEventBus with default topic.
  ///
  /// The default topic is "default.events". Events published without
  /// a specific topic will use this default.
  ///
  /// # Returns
  ///
  /// A new InMemoryEventBus instance with empty queue and no handlers.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let bus: hexser::adapters::InMemoryEventBus<std::string::String> =
  ///     hexser::adapters::InMemoryEventBus::new();
  /// ```
  pub fn new() -> Self {
    Self {
      queue: std::cell::RefCell::new(std::vec::Vec::new()),
      handlers: std::cell::RefCell::new(std::collections::HashMap::new()),
      topic: std::string::String::from("default.events"),
    }
  }

  /// Creates a new InMemoryEventBus with a specific topic.
  ///
  /// # Arguments
  ///
  /// * `topic` - The default topic for this event bus
  ///
  /// # Returns
  ///
  /// A new InMemoryEventBus instance configured for the specified topic.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let bus: hexser::adapters::InMemoryEventBus<std::string::String> =
  ///     hexser::adapters::InMemoryEventBus::with_topic(
  ///         std::string::String::from("user.events")
  ///     );
  /// ```
  pub fn with_topic(topic: std::string::String) -> Self {
    Self {
      queue: std::cell::RefCell::new(std::vec::Vec::new()),
      handlers: std::cell::RefCell::new(std::collections::HashMap::new()),
      topic,
    }
  }

  /// Returns the number of events currently in the queue.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let bus: hexser::adapters::InMemoryEventBus<std::string::String> =
  ///     hexser::adapters::InMemoryEventBus::new();
  /// std::assert_eq!(bus.queue_size(), 0);
  /// ```
  pub fn queue_size(&self) -> usize {
    self.queue.borrow().len()
  }

  /// Clears all events from the queue.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let mut bus: hexser::adapters::InMemoryEventBus<std::string::String> =
  ///     hexser::adapters::InMemoryEventBus::new();
  /// bus.clear();
  /// std::assert_eq!(bus.queue_size(), 0);
  /// ```
  pub fn clear(&mut self) {
    self.queue.borrow_mut().clear();
  }
}

impl<T> Default for InMemoryEventBus<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> crate::adapters::Adapter for InMemoryEventBus<T> {}

impl<T> crate::ports::events::EventPublisher<T> for InMemoryEventBus<T>
where
  T: Clone,
{
  fn publish(
    &self,
    envelope: &crate::ports::events::CloudEventsEnvelope<T>,
  ) -> crate::HexResult<()> {
    // Validate envelope before publishing
    envelope.validate()?;
    envelope.validate_time_format()?;

    // Add to queue
    self.queue.borrow_mut().push(envelope.clone());

    // Invoke handlers for this topic
    let handlers = self.handlers.borrow();
    if let std::option::Option::Some(handler) = handlers.get(&self.topic) {
      handler(envelope.clone())?;
    }

    std::result::Result::Ok(())
  }

  fn publish_batch(
    &self,
    envelopes: &[crate::ports::events::CloudEventsEnvelope<T>],
  ) -> crate::HexResult<()> {
    for envelope in envelopes {
      self.publish(envelope)?;
    }
    std::result::Result::Ok(())
  }
}

impl<T> crate::ports::events::EventSubscriber<T> for InMemoryEventBus<T>
where
  T: Clone,
{
  fn subscribe(
    &mut self,
    topic: &str,
    handler: std::boxed::Box<
      dyn Fn(crate::ports::events::CloudEventsEnvelope<T>) -> crate::HexResult<()>,
    >,
  ) -> crate::HexResult<()> {
    self
      .handlers
      .borrow_mut()
      .insert(std::string::String::from(topic), handler);
    self.topic = std::string::String::from(topic);
    std::result::Result::Ok(())
  }

  fn poll(
    &mut self,
  ) -> crate::HexResult<std::option::Option<crate::ports::events::CloudEventsEnvelope<T>>> {
    let mut queue = self.queue.borrow_mut();
    if queue.is_empty() {
      std::result::Result::Ok(std::option::Option::None)
    } else {
      std::result::Result::Ok(std::option::Option::Some(queue.remove(0)))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ports::events::{EventPublisher, EventSubscriber};

  #[derive(Clone)]
  struct TestEvent {
    id: std::string::String,
    value: std::string::String,
  }

  impl crate::domain::DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
      "com.test.event.created"
    }

    fn aggregate_id(&self) -> std::string::String {
      self.id.clone()
    }
  }

  #[test]
  fn test_new_bus_is_empty() {
    let bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();
    std::assert_eq!(bus.queue_size(), 0);
  }

  #[test]
  fn test_with_topic_sets_topic() {
    let bus: InMemoryEventBus<TestEvent> =
      InMemoryEventBus::with_topic(std::string::String::from("test.events"));
    std::assert_eq!(bus.topic, "test.events");
  }

  #[test]
  fn test_publish_adds_to_queue() {
    let bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let event = TestEvent {
      id: std::string::String::from("test-123"),
      value: std::string::String::from("test value"),
    };

    let envelope = crate::ports::events::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    bus.publish(&envelope).unwrap();
    std::assert_eq!(bus.queue_size(), 1);
  }

  #[test]
  fn test_publish_batch_adds_multiple() {
    let bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let envelopes = vec![
      crate::ports::events::CloudEventsEnvelope::from_domain_event(
        std::string::String::from("evt-001"),
        std::string::String::from("/test/source"),
        TestEvent {
          id: std::string::String::from("test-1"),
          value: std::string::String::from("value-1"),
        },
      ),
      crate::ports::events::CloudEventsEnvelope::from_domain_event(
        std::string::String::from("evt-002"),
        std::string::String::from("/test/source"),
        TestEvent {
          id: std::string::String::from("test-2"),
          value: std::string::String::from("value-2"),
        },
      ),
    ];

    bus.publish_batch(&envelopes).unwrap();
    std::assert_eq!(bus.queue_size(), 2);
  }

  #[test]
  fn test_poll_returns_none_when_empty() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let result = bus.poll().unwrap();
    std::assert!(result.is_none());
  }

  #[test]
  fn test_poll_returns_event_and_removes_from_queue() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let event = TestEvent {
      id: std::string::String::from("test-123"),
      value: std::string::String::from("test value"),
    };

    let envelope = crate::ports::events::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    bus.publish(&envelope).unwrap();
    std::assert_eq!(bus.queue_size(), 1);

    let polled = bus.poll().unwrap();
    std::assert!(polled.is_some());
    std::assert_eq!(bus.queue_size(), 0);
  }

  #[test]
  fn test_subscribe_and_handler_invoked() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let invoked = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let invoked_clone = invoked.clone();

    bus
      .subscribe(
        "default.events",
        std::boxed::Box::new(move |_envelope| {
          invoked_clone.store(true, std::sync::atomic::Ordering::SeqCst);
          std::result::Result::Ok(())
        }),
      )
      .unwrap();

    let event = TestEvent {
      id: std::string::String::from("test-123"),
      value: std::string::String::from("test value"),
    };

    let envelope = crate::ports::events::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    bus.publish(&envelope).unwrap();
    std::assert!(invoked.load(std::sync::atomic::Ordering::SeqCst));
  }

  #[test]
  fn test_clear_empties_queue() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let event = TestEvent {
      id: std::string::String::from("test-123"),
      value: std::string::String::from("test value"),
    };

    let envelope = crate::ports::events::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    bus.publish(&envelope).unwrap();
    std::assert_eq!(bus.queue_size(), 1);

    bus.clear();
    std::assert_eq!(bus.queue_size(), 0);
  }

  #[test]
  fn test_publish_validates_envelope() {
    let bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    // Create invalid envelope (empty id)
    let envelope = crate::ports::events::CloudEventsEnvelope::<TestEvent>::new(
      std::string::String::from(""),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let result = bus.publish(&envelope);
    std::assert!(result.is_err());
    std::assert_eq!(bus.queue_size(), 0);
  }
}
