//! InMemoryEventBus adapter for CloudEvents publishing and subscription.
//!
//! This module provides a simple in-memory event bus adapter that implements
//! both EventPublisher and EventSubscriber ports. It is intended for testing,
//! development, and as a reference implementation. Events are stored in memory
//! and delivered synchronously without persistence.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Route by envelope type to all handlers for that topic (fix last-subscription-wins misrouting); VecDeque queue with O(1) poll; enqueue only undelivered events to bound push-mode growth.
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
/// // Subscribe to events (requires EventSubscriber trait in scope). The topic is the
/// // CloudEvents `type`, i.e. the event's `event_type()` — here "com.test.event".
/// bus.subscribe(
///     "com.test.event",
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
/// Boxed synchronous handler invoked with each delivered event on a topic.
type EventHandler<T> =
  std::boxed::Box<dyn Fn(crate::ports::events::CloudEventsEnvelope<T>) -> crate::HexResult<()>>;

pub struct InMemoryEventBus<T> {
  /// Events published with no matching handler, awaiting `poll()`. A `VecDeque` gives O(1)
  /// FIFO dequeue (vs. the previous `Vec::remove(0)` which was O(n) per poll).
  queue:
    std::cell::RefCell<std::collections::VecDeque<crate::ports::events::CloudEventsEnvelope<T>>>,
  /// Handlers keyed by topic. Each topic may have several handlers, and events are routed by
  /// the envelope's CloudEvents `type`, so subscribing to a second topic no longer starves the
  /// first (the previous single `self.topic` routing delivered only to the last subscription).
  handlers: std::cell::RefCell<
    std::collections::HashMap<std::string::String, std::vec::Vec<EventHandler<T>>>,
  >,
  /// Legacy default-topic label retained for API compatibility (`with_topic`); routing is by
  /// envelope type, not this field.
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
      queue: std::cell::RefCell::new(std::collections::VecDeque::new()),
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
      queue: std::cell::RefCell::new(std::collections::VecDeque::new()),
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

    // Route by the envelope's CloudEvents `type`, falling back to this bus's default topic when
    // the envelope carries no type. Invoke every handler subscribed to the resolved topic.
    // (`self.handlers.borrow()` here is a shared borrow; a handler that re-enters `publish`
    // takes another shared borrow, which is fine — only `subscribe`, which takes `&mut self`,
    // borrows mutably.)
    let route = if envelope.r#type.is_empty() {
      self.topic.as_str()
    } else {
      envelope.r#type.as_str()
    };
    let delivered = {
      let handlers = self.handlers.borrow();
      match handlers.get(route) {
        std::option::Option::Some(topic_handlers) if !topic_handlers.is_empty() => {
          for handler in topic_handlers {
            handler(envelope.clone())?;
          }
          true
        }
        _ => false,
      }
    };

    // Queue only events that no handler consumed, so a subscribe+publish (push-mode) user does
    // not accumulate an unbounded backlog they never drain; poll-mode users (no handlers) still
    // get every event queued.
    if !delivered {
      self.queue.borrow_mut().push_back(envelope.clone());
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
    // Append to this topic's handler list (multiple handlers per topic are supported). Do NOT
    // mutate a bus-wide "current topic" — that was the source of the last-subscription-wins bug.
    self
      .handlers
      .borrow_mut()
      .entry(std::string::String::from(topic))
      .or_default()
      .push(handler);
    std::result::Result::Ok(())
  }

  fn poll(
    &mut self,
  ) -> crate::HexResult<std::option::Option<crate::ports::events::CloudEventsEnvelope<T>>> {
    std::result::Result::Ok(self.queue.borrow_mut().pop_front())
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

  /// why: a handler subscribed to an event's type must fire when a matching envelope is
  /// published (the base delivery contract). The topic is the CloudEvents `type`.
  #[test]
  fn test_subscribe_and_handler_invoked() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let invoked = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let invoked_clone = invoked.clone();

    bus
      .subscribe(
        "com.test.event.created",
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

  /// Build an envelope and stamp it with an explicit CloudEvents `type` (topic).
  fn envelope_typed(id: &str, ty: &str) -> crate::ports::events::CloudEventsEnvelope<TestEvent> {
    let mut env = crate::ports::events::CloudEventsEnvelope::from_domain_event(
      std::string::String::from(id),
      std::string::String::from("/test/source"),
      TestEvent {
        id: std::string::String::from(id),
        value: std::string::String::from("v"),
      },
    );
    env.r#type = std::string::String::from(ty);
    env
  }

  /// why: subscribing to a second topic must not starve the first — the exact
  /// last-subscription-wins misrouting bug (M28/M33). Each topic's handler must fire only for
  /// its own event type.
  #[test]
  fn test_multiple_topics_route_independently() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();

    let a = std::rc::Rc::new(std::cell::Cell::new(0));
    let b = std::rc::Rc::new(std::cell::Cell::new(0));
    let a_c = a.clone();
    let b_c = b.clone();

    bus
      .subscribe(
        "topic.a",
        std::boxed::Box::new(move |_e| {
          a_c.set(a_c.get() + 1);
          std::result::Result::Ok(())
        }),
      )
      .unwrap();
    bus
      .subscribe(
        "topic.b",
        std::boxed::Box::new(move |_e| {
          b_c.set(b_c.get() + 1);
          std::result::Result::Ok(())
        }),
      )
      .unwrap();

    bus.publish(&envelope_typed("1", "topic.a")).unwrap();
    bus.publish(&envelope_typed("2", "topic.b")).unwrap();
    bus.publish(&envelope_typed("3", "topic.a")).unwrap();

    assert_eq!(a.get(), 2, "topic.a handler must fire for its two events");
    assert_eq!(b.get(), 1, "topic.b handler must fire for its one event");
    // All events were delivered to a handler, so nothing should be queued.
    assert_eq!(bus.queue_size(), 0);
  }

  /// why: multiple handlers on the same topic must all be invoked (a second subscribe to a
  /// topic previously replaced the first via HashMap::insert).
  #[test]
  fn test_multiple_handlers_same_topic_all_fire() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();
    let count = std::rc::Rc::new(std::cell::Cell::new(0));
    for _ in 0..3 {
      let c = count.clone();
      bus
        .subscribe(
          "topic.x",
          std::boxed::Box::new(move |_e| {
            c.set(c.get() + 1);
            std::result::Result::Ok(())
          }),
        )
        .unwrap();
    }
    bus.publish(&envelope_typed("1", "topic.x")).unwrap();
    assert_eq!(count.get(), 3, "all three handlers on topic.x must fire");
  }

  /// why: an event with no subscribed handler must be queued for poll-mode consumers, and an
  /// event that a handler consumed must NOT accumulate in the queue (bounds push-mode growth).
  #[test]
  fn test_undelivered_events_queue_delivered_events_do_not() {
    let mut bus: InMemoryEventBus<TestEvent> = InMemoryEventBus::new();
    bus
      .subscribe(
        "topic.handled",
        std::boxed::Box::new(|_e| std::result::Result::Ok(())),
      )
      .unwrap();

    bus.publish(&envelope_typed("1", "topic.handled")).unwrap();
    bus
      .publish(&envelope_typed("2", "topic.unhandled"))
      .unwrap();

    assert_eq!(bus.queue_size(), 1, "only the unhandled event is queued");
    let polled = bus.poll().unwrap().expect("one queued event");
    assert_eq!(polled.r#type, "topic.unhandled");
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
