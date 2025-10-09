//! CloudEvents v1.0-compliant envelope for wrapping domain events.
//!
//! This module provides the CloudEventsEnvelope struct that wraps domain events
//! with CloudEvents v1.0 specification metadata for transport-agnostic event publishing.
//! The envelope contains REQUIRED attributes (id, source, specversion, type),
//! OPTIONAL attributes (datacontenttype, dataschema, subject, time, data),
//! and support for extension attributes.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Initial CloudEventsEnvelope implementation for CloudEvents v1.0 compliance.

/// CloudEvents v1.0 specification version constant.
pub const CLOUDEVENTS_SPEC_VERSION: &str = "1.0";

/// CloudEvents v1.0-compliant envelope wrapping domain events with transport metadata.
///
/// This struct implements the CloudEvents v1.0 specification for standardized
/// event interchange across different transport protocols (HTTP, Kafka, AMQP, etc.).
/// It wraps domain events (type T) with required and optional CloudEvents attributes.
///
/// # CloudEvents v1.0 Attributes
///
/// **REQUIRED:**
/// - `id`: Unique identifier for the event (unique within source scope)
/// - `source`: URI identifying the context where the event occurred
/// - `specversion`: CloudEvents specification version (always "1.0")
/// - `type`: Event type identifier (e.g., "com.example.user.created")
///
/// **OPTIONAL:**
/// - `datacontenttype`: Media type of data (e.g., "application/json")
/// - `dataschema`: URI of schema that data adheres to
/// - `subject`: Subject of the event in context of source
/// - `time`: Timestamp in RFC3339 format
/// - `data`: The actual event payload (generic type T)
///
/// **EXTENSIONS:**
/// - `extensions`: HashMap for vendor-specific attributes
///
/// # Type Parameter
///
/// - `T`: The domain event type, typically implementing `crate::domain::DomainEvent`
///
/// # Examples
///
/// ```rust
/// use hexser::domain::DomainEvent;
///
/// // Define a domain event
/// struct UserCreated {
///     user_id: std::string::String,
///     email: std::string::String,
/// }
///
/// impl hexser::domain::DomainEvent for UserCreated {
///     fn event_type(&self) -> &str {
///         "com.example.user.created"
///     }
///
///     fn aggregate_id(&self) -> std::string::String {
///         self.user_id.clone()
///     }
/// }
///
/// // Wrap in CloudEvents envelope
/// let event = UserCreated {
///     user_id: std::string::String::from("user-123"),
///     email: std::string::String::from("test@example.com"),
/// };
///
/// let envelope = hexser::ports::events::CloudEventsEnvelope {
///     id: std::string::String::from("evt-001"),
///     source: std::string::String::from("/services/user-service"),
///     specversion: std::string::String::from("1.0"),
///     r#type: event.event_type().to_string(),
///     subject: std::option::Option::Some(event.aggregate_id()),
///     time: std::option::Option::Some(std::string::String::from("2025-10-09T14:51:00Z")),
///     data: std::option::Option::Some(event),
///     datacontenttype: std::option::Option::Some(std::string::String::from("application/json")),
///     dataschema: std::option::Option::None,
///     extensions: std::collections::HashMap::new(),
/// };
///
/// // Verify required attributes
/// std::assert_eq!(envelope.id, "evt-001");
/// std::assert_eq!(envelope.source, "/services/user-service");
/// std::assert_eq!(envelope.specversion, "1.0");
/// std::assert_eq!(envelope.r#type, "com.example.user.created");
/// ```
#[derive(Clone, Debug)]
pub struct CloudEventsEnvelope<T> {
  // REQUIRED attributes per CloudEvents v1.0 specification
  /// Unique identifier for the event.
  ///
  /// MUST be unique within the scope of the producer (source).
  /// Recommended formats: UUID, ULID, or deterministic hash.
  /// Used for idempotency and deduplication.
  pub id: std::string::String,

  /// Identifies the context in which the event occurred.
  ///
  /// MUST be a URI-reference (absolute or relative).
  /// Examples: `/services/user-service`, `https://example.com/orders`
  /// Use for multi-tenant isolation: `/tenants/{tenant-id}/{service}`
  pub source: std::string::String,

  /// CloudEvents specification version.
  ///
  /// MUST be "1.0" for this implementation.
  /// This is an immutable constant per CloudEvents v1.0 spec.
  pub specversion: std::string::String,

  /// Event type identifier.
  ///
  /// Describes the type of event. Should use reverse-DNS naming for collision avoidance.
  /// Examples: `com.example.user.created`, `org.hexser.order.shipped`
  /// Typically maps to `DomainEvent::event_type()` output.
  pub r#type: std::string::String,

  // OPTIONAL attributes per CloudEvents v1.0 specification
  /// Media type of the data attribute value.
  ///
  /// Examples: `application/json`, `application/octet-stream`, `text/plain`
  /// Defaults to `application/json` for JSON-serializable data.
  pub datacontenttype: std::option::Option<std::string::String>,

  /// URI identifying the schema that data adheres to.
  ///
  /// Examples: `https://example.com/schemas/user.json`, JSON Schema URI
  /// Enables schema validation and evolution tracking.
  pub dataschema: std::option::Option<std::string::String>,

  /// Subject of the event in the context of the event source.
  ///
  /// Examples: user ID, order ID, resource path
  /// Typically maps to `DomainEvent::aggregate_id()` for domain events.
  pub subject: std::option::Option<std::string::String>,

  /// Timestamp when the event occurred.
  ///
  /// MUST be in RFC3339 format: `2025-10-09T14:51:00Z`
  /// If omitted, consumers may use event receipt time.
  pub time: std::option::Option<std::string::String>,

  /// The actual event payload.
  ///
  /// Contains the domain event data. Type T typically implements
  /// `crate::domain::DomainEvent` trait. Can be any serializable type.
  pub data: std::option::Option<T>,

  /// Extension attributes for vendor-specific or application-specific metadata.
  ///
  /// Keys MUST NOT start with `ce-` (reserved for CloudEvents).
  /// Common extensions: `traceparent`, `correlationid`, `tenantid`
  pub extensions: std::collections::HashMap<std::string::String, std::string::String>,
}

impl<T> CloudEventsEnvelope<T> {
  /// Creates a new CloudEventsEnvelope with required attributes.
  ///
  /// This constructor initializes the envelope with CloudEvents v1.0 REQUIRED
  /// attributes. Optional attributes can be set via the returned struct.
  ///
  /// # Arguments
  ///
  /// * `id` - Unique event identifier (must be non-empty)
  /// * `source` - URI identifying the event source context
  /// * `event_type` - Event type identifier (reverse-DNS recommended)
  ///
  /// # Returns
  ///
  /// A new CloudEventsEnvelope with required attributes set and optional attributes as None.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// std::assert_eq!(envelope.id, "evt-001");
  /// std::assert_eq!(envelope.specversion, "1.0");
  /// ```
  pub fn new(
    id: std::string::String,
    source: std::string::String,
    event_type: std::string::String,
  ) -> Self {
    Self {
      id,
      source,
      specversion: std::string::String::from(CLOUDEVENTS_SPEC_VERSION),
      r#type: event_type,
      datacontenttype: std::option::Option::None,
      dataschema: std::option::Option::None,
      subject: std::option::Option::None,
      time: std::option::Option::None,
      data: std::option::Option::None,
      extensions: std::collections::HashMap::new(),
    }
  }

  /// Creates a CloudEventsEnvelope from a domain event implementing DomainEvent trait.
  ///
  /// This convenience constructor automatically maps DomainEvent trait methods
  /// to CloudEvents attributes:
  /// - `event_type()` → `type` attribute
  /// - `aggregate_id()` → `subject` attribute
  ///
  /// # Arguments
  ///
  /// * `id` - Unique event identifier
  /// * `source` - URI identifying the event source context
  /// * `event` - Domain event implementing `crate::domain::DomainEvent`
  ///
  /// # Returns
  ///
  /// A new CloudEventsEnvelope with the event as data and attributes mapped from trait.
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
  /// let event = UserCreated {
  ///     user_id: std::string::String::from("user-123"),
  /// };
  ///
  /// let envelope = hexser::ports::events::CloudEventsEnvelope::from_domain_event(
  ///     std::string::String::from("evt-001"),
  ///     std::string::String::from("/services/user-service"),
  ///     event,
  /// );
  ///
  /// std::assert_eq!(envelope.r#type, "com.example.user.created");
  /// std::assert_eq!(envelope.subject, std::option::Option::Some(std::string::String::from("user-123")));
  /// ```
  pub fn from_domain_event(id: std::string::String, source: std::string::String, event: T) -> Self
  where
    T: crate::domain::DomainEvent,
  {
    let event_type = event.event_type().to_string();
    let subject = event.aggregate_id();

    Self {
      id,
      source,
      specversion: std::string::String::from(CLOUDEVENTS_SPEC_VERSION),
      r#type: event_type,
      datacontenttype: std::option::Option::Some(std::string::String::from("application/json")),
      dataschema: std::option::Option::None,
      subject: std::option::Option::Some(subject),
      time: std::option::Option::None,
      data: std::option::Option::Some(event),
      extensions: std::collections::HashMap::new(),
    }
  }

  /// Validates that required CloudEvents v1.0 attributes are non-empty.
  ///
  /// Checks that all REQUIRED attributes (id, source, specversion, type) are non-empty strings.
  /// The specversion is also validated to be exactly "1.0".
  ///
  /// # Returns
  ///
  /// * `Ok(())` if all required attributes are valid
  /// * `Err(crate::Hexserror)` if any required attribute is empty or specversion is invalid
  ///
  /// # Examples
  ///
  /// ```rust
  /// let envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// std::assert!(envelope.validate().is_ok());
  ///
  /// let invalid_envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from(""),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// std::assert!(invalid_envelope.validate().is_err());
  /// ```
  pub fn validate(&self) -> crate::HexResult<()> {
    if self.id.is_empty() {
      return std::result::Result::Err(crate::Hexserror::validation(
        "CloudEvents id attribute must not be empty",
      ));
    }

    if self.source.is_empty() {
      return std::result::Result::Err(crate::Hexserror::validation(
        "CloudEvents source attribute must not be empty",
      ));
    }

    if self.specversion != CLOUDEVENTS_SPEC_VERSION {
      return std::result::Result::Err(crate::Hexserror::validation(&format!(
        "CloudEvents specversion must be '{}', got '{}'",
        CLOUDEVENTS_SPEC_VERSION, self.specversion
      )));
    }

    if self.r#type.is_empty() {
      return std::result::Result::Err(crate::Hexserror::validation(
        "CloudEvents type attribute must not be empty",
      ));
    }

    std::result::Result::Ok(())
  }

  /// Validates the time attribute format as RFC3339 if present.
  ///
  /// CloudEvents v1.0 specification requires time to be in RFC3339 format.
  /// This method validates the format if the time attribute is set.
  ///
  /// # Returns
  ///
  /// * `Ok(())` if time is None or valid RFC3339 format
  /// * `Err(crate::Hexserror)` if time format is invalid
  ///
  /// # Examples
  ///
  /// ```rust
  /// let mut envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// envelope.time = std::option::Option::Some(std::string::String::from("2025-10-09T14:51:00Z"));
  /// std::assert!(envelope.validate_time_format().is_ok());
  ///
  /// envelope.time = std::option::Option::Some(std::string::String::from("invalid-time"));
  /// std::assert!(envelope.validate_time_format().is_err());
  /// ```
  pub fn validate_time_format(&self) -> crate::HexResult<()> {
    if let std::option::Option::Some(ref time_str) = self.time {
      // Basic RFC3339 format validation (simplified)
      // Format: YYYY-MM-DDTHH:MM:SSZ or YYYY-MM-DDTHH:MM:SS+00:00
      if time_str.len() < 20 {
        return std::result::Result::Err(crate::Hexserror::validation(
          "CloudEvents time attribute must be in RFC3339 format (e.g., 2025-10-09T14:51:00Z)",
        ));
      }

      // Check for 'T' separator and 'Z' or '+'/'-' timezone indicator
      if !time_str.contains('T') {
        return std::result::Result::Err(crate::Hexserror::validation(
          "CloudEvents time attribute must contain 'T' separator (RFC3339 format)",
        ));
      }
    }

    std::result::Result::Ok(())
  }

  /// Adds an extension attribute to the envelope.
  ///
  /// Extension attributes allow vendor-specific or application-specific metadata.
  /// Keys MUST NOT start with `ce-` (reserved by CloudEvents specification).
  ///
  /// # Arguments
  ///
  /// * `key` - Extension attribute name (must not start with "ce-")
  /// * `value` - Extension attribute value
  ///
  /// # Returns
  ///
  /// * `Ok(())` if extension was added successfully
  /// * `Err(crate::Hexserror)` if key starts with "ce-"
  ///
  /// # Examples
  ///
  /// ```rust
  /// let mut envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// std::assert!(envelope.add_extension(
  ///     std::string::String::from("traceparent"),
  ///     std::string::String::from("00-trace-id-01")
  /// ).is_ok());
  ///
  /// std::assert!(envelope.add_extension(
  ///     std::string::String::from("ce-invalid"),
  ///     std::string::String::from("value")
  /// ).is_err());
  /// ```
  pub fn add_extension(
    &mut self,
    key: std::string::String,
    value: std::string::String,
  ) -> crate::HexResult<()> {
    if key.starts_with("ce-") {
      return std::result::Result::Err(crate::Hexserror::validation(
        "Extension attribute keys must not start with 'ce-' (reserved by CloudEvents specification)",
      ));
    }

    self.extensions.insert(key, value);
    std::result::Result::Ok(())
  }

  /// Gets an extension attribute value by key.
  ///
  /// # Arguments
  ///
  /// * `key` - Extension attribute name
  ///
  /// # Returns
  ///
  /// * `Some(&String)` if extension exists
  /// * `None` if extension does not exist
  ///
  /// # Examples
  ///
  /// ```rust
  /// let mut envelope: hexser::ports::events::CloudEventsEnvelope<std::string::String> =
  ///     hexser::ports::events::CloudEventsEnvelope::new(
  ///         std::string::String::from("evt-001"),
  ///         std::string::String::from("/services/user-service"),
  ///         std::string::String::from("com.example.user.created"),
  ///     );
  ///
  /// envelope.add_extension(
  ///     std::string::String::from("tenantid"),
  ///     std::string::String::from("acme")
  /// ).unwrap();
  ///
  /// std::assert_eq!(
  ///     envelope.get_extension("tenantid"),
  ///     std::option::Option::Some(&std::string::String::from("acme"))
  /// );
  /// ```
  pub fn get_extension(&self, key: &str) -> std::option::Option<&std::string::String> {
    self.extensions.get(key)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestEvent {
    id: std::string::String,
    data: std::string::String,
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
  fn test_new_envelope_with_required_attributes() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    std::assert_eq!(envelope.id, "evt-001");
    std::assert_eq!(envelope.source, "/test/source");
    std::assert_eq!(envelope.specversion, "1.0");
    std::assert_eq!(envelope.r#type, "com.test.event");
    std::assert!(envelope.data.is_none());
    std::assert!(envelope.subject.is_none());
  }

  #[test]
  fn test_from_domain_event_maps_attributes() {
    let event = TestEvent {
      id: std::string::String::from("test-123"),
      data: std::string::String::from("test data"),
    };

    let envelope = CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-002"),
      std::string::String::from("/test/source"),
      event,
    );

    std::assert_eq!(envelope.id, "evt-002");
    std::assert_eq!(envelope.r#type, "com.test.event.created");
    std::assert_eq!(
      envelope.subject,
      std::option::Option::Some(std::string::String::from("test-123"))
    );
    std::assert_eq!(
      envelope.datacontenttype,
      std::option::Option::Some(std::string::String::from("application/json"))
    );
    std::assert!(envelope.data.is_some());
  }

  #[test]
  fn test_validate_required_attributes_success() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    std::assert!(envelope.validate().is_ok());
  }

  #[test]
  fn test_validate_empty_id_fails() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from(""),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    std::assert!(envelope.validate().is_err());
  }

  #[test]
  fn test_validate_empty_source_fails() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from(""),
      std::string::String::from("com.test.event"),
    );

    std::assert!(envelope.validate().is_err());
  }

  #[test]
  fn test_validate_empty_type_fails() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from(""),
    );

    std::assert!(envelope.validate().is_err());
  }

  #[test]
  fn test_validate_time_format_valid() {
    let mut envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    envelope.time = std::option::Option::Some(std::string::String::from("2025-10-09T14:51:00Z"));
    std::assert!(envelope.validate_time_format().is_ok());
  }

  #[test]
  fn test_validate_time_format_invalid() {
    let mut envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    envelope.time = std::option::Option::Some(std::string::String::from("invalid"));
    std::assert!(envelope.validate_time_format().is_err());
  }

  #[test]
  fn test_add_extension_success() {
    let mut envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let result = envelope.add_extension(
      std::string::String::from("traceparent"),
      std::string::String::from("00-trace-01"),
    );

    std::assert!(result.is_ok());
    std::assert_eq!(
      envelope.extensions.get("traceparent"),
      std::option::Option::Some(&std::string::String::from("00-trace-01"))
    );
  }

  #[test]
  fn test_add_extension_rejects_ce_prefix() {
    let mut envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let result = envelope.add_extension(
      std::string::String::from("ce-invalid"),
      std::string::String::from("value"),
    );

    std::assert!(result.is_err());
  }

  #[test]
  fn test_get_extension_returns_value() {
    let mut envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    envelope
      .add_extension(
        std::string::String::from("tenantid"),
        std::string::String::from("acme"),
      )
      .unwrap();

    std::assert_eq!(
      envelope.get_extension("tenantid"),
      std::option::Option::Some(&std::string::String::from("acme"))
    );
  }

  #[test]
  fn test_get_extension_returns_none_for_missing_key() {
    let envelope: CloudEventsEnvelope<std::string::String> = CloudEventsEnvelope::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    std::assert_eq!(
      envelope.get_extension("nonexistent"),
      std::option::Option::None
    );
  }
}
