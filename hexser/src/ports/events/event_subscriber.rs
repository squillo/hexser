//! EventSubscriber port trait for consuming CloudEvents-wrapped domain events.
//!
//! This module defines the EventSubscriber port trait for transport-agnostic
//! event consumption. Implementations can target different transports (HTTP webhooks,
//! Kafka consumers, AMQP subscribers) while maintaining a consistent interface
//! for consuming CloudEvents v1.0 envelopes with back-pressure support.
//!
//! Revision History
//! - 2025-10-09T15:12:00Z @AI: Fix doc test to use trait import for subscribe/poll methods.
//! - 2025-10-09T14:51:00Z @AI: Initial EventSubscriber port trait definition.

/// Port trait for consuming CloudEvents-wrapped domain events from a transport.
///
/// EventSubscriber defines the interface for consuming events without coupling
/// to specific transport implementations. It supports both push-based (subscribe
/// with handler) and pull-based (poll) consumption models for back-pressure control.
///
/// # Type Parameter
///
/// - `T`: The domain event type contained in the CloudEvents envelope
///
/// # Methods
///
/// - `subscribe`: Registers a handler for events on a specific topic (push model)
/// - `poll`: Retrieves the next available event if any (pull model for back-pressure)
///
/// # Examples
///
/// ```rust
/// use hexser::ports::events::EventSubscriber;
///
/// struct MySubscriber;
///
/// impl hexser::ports::events::EventSubscriber<std::string::String> for MySubscriber {
///     fn subscribe(
///         &mut self,
///         topic: &str,
///         handler: std::boxed::Box<dyn Fn(hexser::ports::events::CloudEventsEnvelope<std::string::String>) -> hexser::HexResult<()>>,
///     ) -> hexser::HexResult<()> {
///         // Subscribe logic here
///         std::result::Result::Ok(())
///     }
///
///     fn poll(&mut self) -> hexser::HexResult<std::option::Option<hexser::ports::events::CloudEventsEnvelope<std::string::String>>> {
///         // Poll logic here
///         std::result::Result::Ok(std::option::Option::None)
///     }
/// }
///
/// // Usage with subscribe (push model)
/// let mut subscriber = MySubscriber;
/// subscriber.subscribe(
///     "user.events",
///     std::boxed::Box::new(|envelope| {
///         // Handle event
///         std::result::Result::Ok(())
///     }),
/// ).unwrap();
///
/// // Usage with poll (pull model for back-pressure)
/// if let std::option::Option::Some(envelope) = subscriber.poll().unwrap() {
///     // Process envelope
/// }
/// ```
pub trait EventSubscriber<T> {
  /// Subscribes to events on a topic with a handler function.
  ///
  /// This method registers a handler that will be invoked when events are received
  /// on the specified topic. The handler is a closure that receives the CloudEvents
  /// envelope and returns a result.
  ///
  /// # Arguments
  ///
  /// * `topic` - The topic/channel/subject to subscribe to
  /// * `handler` - Closure invoked for each received event
  ///
  /// # Returns
  ///
  /// * `Ok(())` if subscription was successful
  /// * `Err(crate::Hexserror)` if subscription failed
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Invalid topic names
  /// - Connection failures
  /// - Authorization/permission issues
  /// - Resource exhaustion (too many subscriptions)
  ///
  /// # Examples
  ///
  /// ```rust
  /// struct UserCreated {
  ///     user_id: std::string::String,
  /// }
  ///
  /// impl hexser::domain::DomainEvent for UserCreated {
  ///     fn event_type(&self) -> &str { "com.example.user.created" }
  ///     fn aggregate_id(&self) -> std::string::String { self.user_id.clone() }
  /// }
  ///
  /// // Subscribe to user events
  /// // subscriber.subscribe(
  /// //     "user.events",
  /// //     std::boxed::Box::new(|envelope| {
  /// //         if let std::option::Option::Some(event) = envelope.data {
  /// //             // Process UserCreated event
  /// //         }
  /// //         std::result::Result::Ok(())
  /// //     }),
  /// // )?;
  /// ```
  fn subscribe(
    &mut self,
    topic: &str,
    handler: std::boxed::Box<dyn Fn(super::CloudEventsEnvelope<T>) -> crate::HexResult<()>>,
  ) -> crate::HexResult<()>;

  /// Polls for the next available event with back-pressure control.
  ///
  /// This method retrieves the next event from the transport if available,
  /// enabling pull-based consumption for back-pressure management. Returns
  /// None if no events are currently available.
  ///
  /// # Returns
  ///
  /// * `Ok(Some(envelope))` if an event is available
  /// * `Ok(None)` if no events are currently available
  /// * `Err(crate::Hexserror)` if polling failed
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Connection failures
  /// - Deserialization failures
  /// - Transport-specific errors
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// // Pull-based consumption with back-pressure
  /// loop {
  ///     // match subscriber.poll() {
  ///     //     std::result::Result::Ok(std::option::Option::Some(envelope)) => {
  ///     //         // Process event
  ///     //         envelope.validate().unwrap();
  ///     //     }
  ///     //     std::result::Result::Ok(std::option::Option::None) => {
  ///     //         // No events available, can apply back-pressure
  ///     //         break;
  ///     //     }
  ///     //     std::result::Result::Err(e) => {
  ///     //         // Handle error
  ///     //         break;
  ///     //     }
  ///     // }
  /// }
  /// ```
  fn poll(&mut self) -> crate::HexResult<std::option::Option<super::CloudEventsEnvelope<T>>>;
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

  struct MockSubscriber {
    events: std::cell::RefCell<std::vec::Vec<super::super::CloudEventsEnvelope<TestEvent>>>,
    handlers: std::cell::RefCell<
      std::collections::HashMap<
        std::string::String,
        std::boxed::Box<
          dyn Fn(super::super::CloudEventsEnvelope<TestEvent>) -> crate::HexResult<()>,
        >,
      >,
    >,
  }

  impl MockSubscriber {
    fn new() -> Self {
      Self {
        events: std::cell::RefCell::new(std::vec::Vec::new()),
        handlers: std::cell::RefCell::new(std::collections::HashMap::new()),
      }
    }

    fn add_event(&self, envelope: super::super::CloudEventsEnvelope<TestEvent>) {
      self.events.borrow_mut().push(envelope);
    }

    fn event_count(&self) -> usize {
      self.events.borrow().len()
    }
  }

  impl EventSubscriber<TestEvent> for MockSubscriber {
    fn subscribe(
      &mut self,
      topic: &str,
      handler: std::boxed::Box<
        dyn Fn(super::super::CloudEventsEnvelope<TestEvent>) -> crate::HexResult<()>,
      >,
    ) -> crate::HexResult<()> {
      self
        .handlers
        .borrow_mut()
        .insert(std::string::String::from(topic), handler);
      std::result::Result::Ok(())
    }

    fn poll(
      &mut self,
    ) -> crate::HexResult<std::option::Option<super::super::CloudEventsEnvelope<TestEvent>>> {
      let mut events = self.events.borrow_mut();
      if events.is_empty() {
        std::result::Result::Ok(std::option::Option::None)
      } else {
        std::result::Result::Ok(std::option::Option::Some(events.remove(0)))
      }
    }
  }

  #[test]
  fn test_subscribe_registers_handler() {
    let mut subscriber = MockSubscriber::new();

    let result = subscriber.subscribe(
      "test.topic",
      std::boxed::Box::new(|_envelope| std::result::Result::Ok(())),
    );

    std::assert!(result.is_ok());
    std::assert_eq!(subscriber.handlers.borrow().len(), 1);
    std::assert!(subscriber.handlers.borrow().contains_key("test.topic"));
  }

  #[test]
  fn test_poll_returns_none_when_empty() {
    let mut subscriber = MockSubscriber::new();

    let result = subscriber.poll();

    std::assert!(result.is_ok());
    std::assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_poll_returns_event_when_available() {
    let mut subscriber = MockSubscriber::new();

    let event = TestEvent {
      id: std::string::String::from("test-123"),
    };

    let envelope = super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    subscriber.add_event(envelope);

    let result = subscriber.poll();
    std::assert!(result.is_ok());

    let polled_envelope = result.unwrap();
    std::assert!(polled_envelope.is_some());
    std::assert_eq!(polled_envelope.unwrap().id, "evt-001");
  }

  #[test]
  fn test_poll_removes_event_from_queue() {
    let mut subscriber = MockSubscriber::new();

    let event1 = TestEvent {
      id: std::string::String::from("test-1"),
    };
    let event2 = TestEvent {
      id: std::string::String::from("test-2"),
    };

    subscriber.add_event(super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event1,
    ));

    subscriber.add_event(super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-002"),
      std::string::String::from("/test/source"),
      event2,
    ));

    std::assert_eq!(subscriber.event_count(), 2);

    let _ = subscriber.poll();
    std::assert_eq!(subscriber.event_count(), 1);

    let _ = subscriber.poll();
    std::assert_eq!(subscriber.event_count(), 0);

    let result = subscriber.poll();
    std::assert!(result.unwrap().is_none());
  }

  #[test]
  fn test_multiple_topic_subscriptions() {
    let mut subscriber = MockSubscriber::new();

    let result1 = subscriber.subscribe(
      "topic1",
      std::boxed::Box::new(|_envelope| std::result::Result::Ok(())),
    );

    let result2 = subscriber.subscribe(
      "topic2",
      std::boxed::Box::new(|_envelope| std::result::Result::Ok(())),
    );

    std::assert!(result1.is_ok());
    std::assert!(result2.is_ok());
    std::assert_eq!(subscriber.handlers.borrow().len(), 2);
  }
}
