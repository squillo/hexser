//! DomainEvent trait for significant occurrences in the domain.
//!
//! Domain events represent something that happened in the domain that domain
//! experts care about. They are immutable facts about the past and are used
//! for event sourcing, event-driven architectures, and communication between
//! bounded contexts. Events capture the intent and meaning behind state changes.
//!
//! Revision History
//! - 2025-10-01T00:00:00Z @AI: Initial DomainEvent trait definition with metadata.

/// Trait for domain events representing significant occurrences.
///
/// Domain events are immutable records of something that happened in the domain.
/// They capture the intent behind changes and enable event-driven architecture.
///
/// # Example
///
/// ```rust
/// use hexser::domain::DomainEvent;
///
/// struct UserRegistered {
///     user_id: String,
///     email: String,
///     timestamp: u64,
/// }
///
/// impl DomainEvent for UserRegistered {
///     fn event_type(&self) -> &str {
///         "UserRegistered"
///     }
///
///     fn aggregate_id(&self) -> String {
///         self.user_id.clone()
///     }
/// }
/// ```
pub trait DomainEvent {
    /// Returns the type name of this event.
    fn event_type(&self) -> &str;

    /// Returns the identifier of the aggregate that produced this event.
    fn aggregate_id(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUserCreated {
        user_id: String,
        email: String,
    }

    impl DomainEvent for TestUserCreated {
        fn event_type(&self) -> &str {
            "UserCreated"
        }

        fn aggregate_id(&self) -> String {
            self.user_id.clone()
        }
    }

    #[test]
    fn test_domain_event_type() {
        let event = TestUserCreated {
            user_id: String::from("123"),
            email: String::from("test@example.com"),
        };
        assert_eq!(event.event_type(), "UserCreated");
    }

    #[test]
    fn test_domain_event_aggregate_id() {
        let event = TestUserCreated {
            user_id: String::from("123"),
            email: String::from("test@example.com"),
        };
        assert_eq!(event.aggregate_id(), "123");
    }
}
