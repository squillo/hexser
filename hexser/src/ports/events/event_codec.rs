//! EventCodec port trait for serializing and deserializing CloudEvents envelopes.
//!
//! This module defines the EventCodec port trait for format-agnostic
//! serialization and deserialization of CloudEvents v1.0 envelopes.
//! Implementations can support different formats (JSON, Avro, Protobuf)
//! while maintaining a consistent encoding/decoding interface.
//!
//! Revision History
//! - 2025-10-09T14:51:00Z @AI: Initial EventCodec port trait definition.

/// Port trait for encoding and decoding CloudEvents envelopes.
///
/// EventCodec defines the interface for serializing CloudEvents-wrapped domain
/// events to bytes and deserializing bytes back to envelopes. This enables
/// support for multiple CloudEvents event formats (JSON, Avro, Protobuf) without
/// coupling the domain or ports to specific serialization libraries.
///
/// # Type Parameter
///
/// - `T`: The domain event type contained in the CloudEvents envelope
///
/// # Methods
///
/// - `encode`: Serializes a CloudEvents envelope to bytes
/// - `decode`: Deserializes bytes to a CloudEvents envelope
///
/// # Examples
///
/// ```rust
/// use hexser::ports::events::EventCodec;
///
/// struct MyCodec;
///
/// impl hexser::ports::events::EventCodec<std::string::String> for MyCodec {
///     fn encode(
///         &self,
///         envelope: &hexser::ports::events::CloudEventsEnvelope<std::string::String>,
///     ) -> hexser::HexResult<std::vec::Vec<u8>> {
///         // Encoding logic here
///         std::result::Result::Ok(std::vec::Vec::new())
///     }
///
///     fn decode(
///         &self,
///         bytes: &[u8],
///     ) -> hexser::HexResult<hexser::ports::events::CloudEventsEnvelope<std::string::String>> {
///         // Decoding logic here
///         let envelope = hexser::ports::events::CloudEventsEnvelope::<std::string::String>::new(
///             std::string::String::from("evt-001"),
///             std::string::String::from("/test"),
///             std::string::String::from("test.event"),
///         );
///         std::result::Result::Ok(envelope)
///     }
/// }
///
/// // Usage
/// let codec = MyCodec;
/// let envelope = hexser::ports::events::CloudEventsEnvelope::new(
///     std::string::String::from("evt-001"),
///     std::string::String::from("/services/test"),
///     std::string::String::from("com.example.test.event"),
/// );
///
/// let bytes = codec.encode(&envelope).unwrap();
/// let decoded = codec.decode(&bytes).unwrap();
/// ```
pub trait EventCodec<T> {
  /// Encodes a CloudEvents envelope to bytes.
  ///
  /// This method serializes the CloudEvents envelope to a byte representation
  /// according to the codec's format (e.g., JSON, Avro, Protobuf). The encoding
  /// must preserve all CloudEvents v1.0 attributes (required, optional, extensions).
  ///
  /// # Arguments
  ///
  /// * `envelope` - The CloudEvents envelope to encode
  ///
  /// # Returns
  ///
  /// * `Ok(Vec<u8>)` - The serialized bytes
  /// * `Err(crate::Hexserror)` - If encoding fails
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Serialization failures (invalid data structure)
  /// - Type conversion issues
  /// - Format-specific validation failures
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
  ///     std::string::String::from("/services/user"),
  ///     event,
  /// );
  ///
  /// // let bytes = codec.encode(&envelope)?;
  /// // Bytes can be sent over HTTP, Kafka, AMQP, etc.
  /// ```
  fn encode(&self, envelope: &super::CloudEventsEnvelope<T>)
  -> crate::HexResult<std::vec::Vec<u8>>;

  /// Decodes bytes to a CloudEvents envelope.
  ///
  /// This method deserializes bytes to a CloudEvents envelope according to
  /// the codec's format. The decoding must reconstruct all CloudEvents v1.0
  /// attributes and validate the envelope structure.
  ///
  /// # Arguments
  ///
  /// * `bytes` - The serialized bytes to decode
  ///
  /// # Returns
  ///
  /// * `Ok(CloudEventsEnvelope<T>)` - The deserialized envelope
  /// * `Err(crate::Hexserror)` - If decoding fails
  ///
  /// # Errors
  ///
  /// Implementations should return errors for:
  /// - Deserialization failures (malformed data)
  /// - Missing required CloudEvents attributes
  /// - Type conversion issues
  /// - Format-specific validation failures
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Receive bytes from transport (HTTP, Kafka, etc.)
  /// let bytes: std::vec::Vec<u8> = std::vec::Vec::new(); // From transport
  ///
  /// // let envelope = codec.decode(&bytes)?;
  /// // envelope.validate()?;
  /// //
  /// // if let std::option::Option::Some(event) = envelope.data {
  /// //     // Process event
  /// // }
  /// ```
  fn decode(&self, bytes: &[u8]) -> crate::HexResult<super::CloudEventsEnvelope<T>>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestEvent {
    id: std::string::String,
    value: std::string::String,
  }

  impl crate::domain::DomainEvent for TestEvent {
    fn event_type(&self) -> &str {
      "com.test.event"
    }

    fn aggregate_id(&self) -> std::string::String {
      self.id.clone()
    }
  }

  // Simple mock codec that uses a basic string format
  struct MockCodec;

  impl EventCodec<TestEvent> for MockCodec {
    fn encode(
      &self,
      envelope: &super::super::CloudEventsEnvelope<TestEvent>,
    ) -> crate::HexResult<std::vec::Vec<u8>> {
      envelope.validate()?;

      // Simple format: id|source|type|subject
      let mut parts = std::vec::Vec::new();
      parts.push(envelope.id.clone());
      parts.push(envelope.source.clone());
      parts.push(envelope.r#type.clone());

      if let std::option::Option::Some(ref subject) = envelope.subject {
        parts.push(subject.clone());
      } else {
        parts.push(std::string::String::from(""));
      }

      let encoded = parts.join("|");
      std::result::Result::Ok(encoded.into_bytes())
    }

    fn decode(
      &self,
      bytes: &[u8],
    ) -> crate::HexResult<super::super::CloudEventsEnvelope<TestEvent>> {
      let string = std::string::String::from_utf8(bytes.to_vec())
        .map_err(|_| crate::Hexserror::validation("Invalid UTF-8"))?;

      let parts: std::vec::Vec<&str> = string.split('|').collect();

      if parts.len() < 4 {
        return std::result::Result::Err(crate::Hexserror::validation("Invalid format"));
      }

      let mut envelope = super::super::CloudEventsEnvelope::new(
        std::string::String::from(parts[0]),
        std::string::String::from(parts[1]),
        std::string::String::from(parts[2]),
      );

      if !parts[3].is_empty() {
        envelope.subject = std::option::Option::Some(std::string::String::from(parts[3]));
      }

      std::result::Result::Ok(envelope)
    }
  }

  #[test]
  fn test_encode_creates_bytes() {
    let codec = MockCodec;
    let event = TestEvent {
      id: std::string::String::from("test-123"),
      value: std::string::String::from("test value"),
    };

    let envelope = super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      event,
    );

    let result = codec.encode(&envelope);
    std::assert!(result.is_ok());

    let bytes = result.unwrap();
    std::assert!(!bytes.is_empty());
  }

  #[test]
  fn test_decode_reconstructs_envelope() {
    let codec = MockCodec;

    let original = super::super::CloudEventsEnvelope::<TestEvent>::new(
      std::string::String::from("evt-001"),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let bytes = codec.encode(&original).unwrap();
    let decoded = codec.decode(&bytes).unwrap();

    std::assert_eq!(decoded.id, original.id);
    std::assert_eq!(decoded.source, original.source);
    std::assert_eq!(decoded.r#type, original.r#type);
  }

  #[test]
  fn test_encode_decode_roundtrip() {
    let codec = MockCodec;
    let event = TestEvent {
      id: std::string::String::from("test-456"),
      value: std::string::String::from("roundtrip test"),
    };

    let original = super::super::CloudEventsEnvelope::from_domain_event(
      std::string::String::from("evt-roundtrip"),
      std::string::String::from("/roundtrip/source"),
      event,
    );

    let bytes = codec.encode(&original).unwrap();
    let decoded = codec.decode(&bytes).unwrap();

    std::assert_eq!(decoded.id, original.id);
    std::assert_eq!(decoded.source, original.source);
    std::assert_eq!(decoded.r#type, original.r#type);
    std::assert_eq!(decoded.subject, original.subject);
  }

  #[test]
  fn test_encode_validates_envelope() {
    let codec = MockCodec;

    // Create invalid envelope (empty id)
    let envelope = super::super::CloudEventsEnvelope::<TestEvent>::new(
      std::string::String::from(""),
      std::string::String::from("/test/source"),
      std::string::String::from("com.test.event"),
    );

    let result = codec.encode(&envelope);
    std::assert!(result.is_err());
  }

  #[test]
  fn test_decode_rejects_invalid_format() {
    let codec = MockCodec;

    let invalid_bytes = b"invalid";
    let result = codec.decode(invalid_bytes);

    std::assert!(result.is_err());
  }

  #[test]
  fn test_decode_rejects_invalid_utf8() {
    let codec = MockCodec;

    let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
    let result = codec.decode(&invalid_bytes);

    std::assert!(result.is_err());
  }
}
