//! CloudEvents v1.0-compliant event ports for hexagonal architecture.
//!
//! This module provides CloudEvents v1.0 specification-compliant ports for
//! transport-agnostic, standardized domain event publishing and consumption.
//! The design integrates seamlessly with hexser's existing DomainEvent trait
//! while providing CloudEvents-standard envelope, ports, and routing.
//!
//! # Architecture
//!
//! - **CloudEventsEnvelope<T>**: CloudEvents v1.0-compliant wrapper struct
//! - **EventPublisher<T>**: Port for publishing events to transports
//! - **EventSubscriber<T>**: Port for consuming events from transports
//! - **EventCodec<T>**: Port for serialization/deserialization
//! - **EventRouter**: Port for topic/subject resolution
//!
//! # CloudEvents v1.0 Compliance
//!
//! All types and traits adhere to the CloudEvents v1.0 specification:
//! - REQUIRED attributes: id, source, specversion, type
//! - OPTIONAL attributes: datacontenttype, dataschema, subject, time, data
//! - Extension attributes support
//! - Transport bindings: HTTP, Kafka, AMQP
//! - JSON format support (application/cloudevents+json)
//!
//! # Integration with DomainEvent
//!
//! The CloudEventsEnvelope wraps domain events implementing the DomainEvent trait:
//! - `event_type()` maps to CloudEvents `type` attribute
//! - `aggregate_id()` maps to CloudEvents `subject` attribute
//! - Domain event becomes the `data` attribute
//!
//! # Examples
//!
//! ```rust
//! // Define a domain event
//! struct UserCreated {
//!     user_id: std::string::String,
//!     email: std::string::String,
//! }
//!
//! impl hexser::domain::DomainEvent for UserCreated {
//!     fn event_type(&self) -> &str {
//!         "com.example.user.created"
//!     }
//!
//!     fn aggregate_id(&self) -> std::string::String {
//!         self.user_id.clone()
//!     }
//! }
//!
//! // Wrap in CloudEvents envelope
//! let event = UserCreated {
//!     user_id: std::string::String::from("user-123"),
//!     email: std::string::String::from("user@example.com"),
//! };
//!
//! let envelope = hexser::ports::events::CloudEventsEnvelope::from_domain_event(
//!     std::string::String::from("evt-001"),
//!     std::string::String::from("/services/user-service"),
//!     event,
//! );
//!
//! // Validate CloudEvents compliance
//! envelope.validate().unwrap();
//! envelope.validate_time_format().unwrap();
//!
//! // Publish using EventPublisher port
//! // publisher.publish(&envelope)?;
//! ```
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Initial events module with CloudEvents v1.0 ports.

pub mod cloud_events_envelope;
pub mod event_codec;
pub mod event_publisher;
pub mod event_router;
pub mod event_subscriber;

// Re-export main types and traits
pub use cloud_events_envelope::{CLOUDEVENTS_SPEC_VERSION, CloudEventsEnvelope};
pub use event_codec::EventCodec;
pub use event_publisher::EventPublisher;
pub use event_router::EventRouter;
pub use event_subscriber::EventSubscriber;
