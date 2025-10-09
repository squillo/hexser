# CloudEvents-Compliant Event System for Hexser

## Overview

This document describes hexser's CloudEvents v1.0-compliant event system for transport-agnostic, standardized domain event publishing and consumption. The design integrates seamlessly with hexser's existing `DomainEvent` trait while providing CloudEvents-standard envelope, ports, and adapters for event-driven architectures.

**Revision History**
- 2025-10-09T14:51:00Z @AI: Initial design document for CloudEvents v1.0 compliance.

## Design Principles

1. **Standards Compliance**: Full CloudEvents v1.0 specification compliance
2. **Transport Agnostic**: Events work with HTTP, Kafka, AMQP, and other transports
3. **Separation of Concerns**: Domain events remain pure; transport concerns live in ports/adapters
4. **Backward Compatible**: Existing `DomainEvent` trait unchanged; new functionality is additive
5. **Zero Dependencies**: Native implementation without external CloudEvents crate
6. **Hexagonal Architecture**: Clear boundaries between domain, ports, and adapters

## Architecture

### Layer Responsibilities

**Domain Layer** (`hexser::domain::DomainEvent`)
- Existing trait for domain event semantics
- Provides `event_type()` and `aggregate_id()` methods
- Remains unchanged for backward compatibility

**Ports Layer** (`hexser::ports::events`)
- `CloudEventsEnvelope<T>`: CloudEvents v1.0-compliant wrapper struct
- `EventPublisher`: Port for publishing events
- `EventSubscriber`: Port for consuming events
- `EventCodec`: Port for serialization/deserialization
- `EventRouter`: Port for topic/subject resolution

**Adapters Layer** (`hexser::adapters`)
- `InMemoryEventBus`: Reference implementation for testing
- `JsonEventCodec`: CloudEvents JSON format codec
- Future: HTTP, Kafka, AMQP transport adapters

## CloudEvents v1.0 Specification

### CloudEventsEnvelope<T> Structure

The `CloudEventsEnvelope<T>` struct wraps domain events with CloudEvents v1.0 metadata:

```rust
pub struct CloudEventsEnvelope<T> {
    // REQUIRED attributes (CloudEvents v1.0 spec)
    pub id: std::string::String,
    pub source: std::string::String,
    pub specversion: std::string::String,
    pub r#type: std::string::String,
    
    // OPTIONAL attributes
    pub datacontenttype: std::option::Option<std::string::String>,
    pub dataschema: std::option::Option<std::string::String>,
    pub subject: std::option::Option<std::string::String>,
    pub time: std::option::Option<std::string::String>,
    pub data: std::option::Option<T>,
    
    // Extension attributes
    pub extensions: std::collections::HashMap<std::string::String, std::string::String>,
}
```

### Required Attributes

**id** (String)
- Unique identifier for the event
- MUST be unique within the scope of the producer (source)
- Example: UUID, ULID, or source+timestamp hash
- Constraint: Non-empty string

**source** (String)
- Identifies the context in which the event occurred
- URI-reference format (absolute or relative)
- Examples: `/services/user-service`, `https://example.com/orders`
- Use for multi-tenant isolation: `/tenants/acme/users`

**specversion** (String)
- CloudEvents specification version
- MUST be `"1.0"` for this implementation
- Immutable constant

**type** (String)
- Describes the type of event
- Should be prefixed with reverse-DNS name for collision avoidance
- Examples: `com.example.user.created`, `org.hexser.order.shipped`
- Maps to `DomainEvent::event_type()` output

### Optional Attributes

**datacontenttype** (String)
- Media type of the `data` attribute
- Examples: `application/json`, `application/octet-stream`
- Defaults to `application/json` if data is JSON-serializable

**dataschema** (String)
- URI identifying the schema that `data` adheres to
- Examples: `https://example.com/schemas/user.json`, JSON Schema URI

**subject** (String)
- Describes the subject of the event in the context of the source
- Examples: user ID, order ID, resource path
- Can map to `DomainEvent::aggregate_id()`

**time** (String)
- Timestamp when the event occurred
- MUST use RFC3339 format: `2025-10-09T14:51:00Z`
- If omitted, consumer may use receipt time

**data** (Generic T)
- The actual domain event payload
- Type T typically implements `DomainEvent` trait
- Can be any serializable type

### Extension Attributes

Extension attributes allow vendor-specific or application-specific metadata:
- Stored in `extensions` HashMap
- Keys MUST NOT start with `ce-` (reserved for CloudEvents)
- Examples: `traceparent`, `correlationid`, `tenantid`

## Integration with Existing DomainEvent Trait

The CloudEvents system integrates with the existing `DomainEvent` trait through the envelope pattern:

```rust
// Existing domain event (unchanged)
struct UserCreated {
    user_id: std::string::String,
    email: std::string::String,
    timestamp: u64,
}

impl hexser::domain::DomainEvent for UserCreated {
    fn event_type(&self) -> &str {
        "com.example.user.created"
    }
    
    fn aggregate_id(&self) -> std::string::String {
        self.user_id.clone()
    }
}

// Wrap in CloudEvents envelope for transport
let event = UserCreated {
    user_id: std::string::String::from("user-123"),
    email: std::string::String::from("user@example.com"),
    timestamp: 1696867860,
};

let envelope = hexser::ports::events::CloudEventsEnvelope {
    id: std::string::String::from("evt-001"),
    source: std::string::String::from("/services/user-service"),
    specversion: std::string::String::from("1.0"),
    r#type: event.event_type().to_string(),
    subject: std::option::Option::Some(event.aggregate_id()),
    time: std::option::Option::Some(std::string::String::from("2025-10-09T14:51:00Z")),
    data: std::option::Option::Some(event),
    datacontenttype: std::option::Option::Some(std::string::String::from("application/json")),
    dataschema: std::option::Option::None,
    extensions: std::collections::HashMap::new(),
};
```

## Port Definitions

### EventPublisher

Publishes CloudEvents-wrapped domain events to a transport:

```rust
pub trait EventPublisher<T> {
    fn publish(
        &self,
        envelope: &hexser::ports::events::CloudEventsEnvelope<T>,
    ) -> hexser::HexResult<()>;
    
    fn publish_batch(
        &self,
        envelopes: &[hexser::ports::events::CloudEventsEnvelope<T>],
    ) -> hexser::HexResult<()>;
}
```

### EventSubscriber

Consumes CloudEvents from a transport with back-pressure support:

```rust
pub trait EventSubscriber<T> {
    fn subscribe(
        &mut self,
        topic: &str,
        handler: Box<dyn Fn(hexser::ports::events::CloudEventsEnvelope<T>) -> hexser::HexResult<()>>,
    ) -> hexser::HexResult<()>;
    
    fn poll(&mut self) -> hexser::HexResult<std::option::Option<hexser::ports::events::CloudEventsEnvelope<T>>>;
}
```

### EventCodec

Serializes and deserializes CloudEvents envelopes:

```rust
pub trait EventCodec<T> {
    fn encode(
        &self,
        envelope: &hexser::ports::events::CloudEventsEnvelope<T>,
    ) -> hexser::HexResult<std::vec::Vec<u8>>;
    
    fn decode(
        &self,
        bytes: &[u8],
    ) -> hexser::HexResult<hexser::ports::events::CloudEventsEnvelope<T>>;
}
```

### EventRouter

Resolves topics and subjects for routing events:

```rust
pub trait EventRouter {
    fn resolve_topic(&self, event_type: &str) -> hexser::HexResult<std::string::String>;
    
    fn resolve_subject(&self, aggregate_id: &str) -> std::option::Option<std::string::String>;
}
```

## Transport Bindings

### HTTP Binary Content Mode

In binary mode, CloudEvents attributes are mapped to HTTP headers with `ce-` prefix:

**Request:**
```
POST /events HTTP/1.1
Host: example.com
Content-Type: application/json
ce-specversion: 1.0
ce-type: com.example.user.created
ce-source: /services/user-service
ce-id: evt-001
ce-time: 2025-10-09T14:51:00Z
ce-subject: user-123

{"user_id":"user-123","email":"user@example.com","timestamp":1696867860}
```

**Mapping:**
- `data` → HTTP body
- `datacontenttype` → `Content-Type` header
- All other attributes → `ce-{attribute}` headers
- Extension attributes → `ce-{extension-name}` headers

### HTTP Structured Content Mode

In structured mode, the entire CloudEvent is in the HTTP body:

**Request:**
```
POST /events HTTP/1.1
Host: example.com
Content-Type: application/cloudevents+json

{
  "specversion": "1.0",
  "type": "com.example.user.created",
  "source": "/services/user-service",
  "id": "evt-001",
  "time": "2025-10-09T14:51:00Z",
  "subject": "user-123",
  "datacontenttype": "application/json",
  "data": {
    "user_id": "user-123",
    "email": "user@example.com",
    "timestamp": 1696867860
  }
}
```

**Mapping:**
- `Content-Type` → `application/cloudevents+json`
- Entire envelope serialized as JSON in body

### Kafka Binary Content Mode

In Kafka binary mode, CloudEvents attributes are mapped to message headers with `ce_` prefix:

**Message:**
```
Headers:
  ce_specversion: 1.0
  ce_type: com.example.user.created
  ce_source: /services/user-service
  ce_id: evt-001
  ce_time: 2025-10-09T14:51:00Z
  ce_subject: user-123
  content-type: application/json

Value:
  {"user_id":"user-123","email":"user@example.com","timestamp":1696867860}
```

**Mapping:**
- `data` → Kafka message value
- `datacontenttype` → `content-type` header
- All other attributes → `ce_{attribute}` headers (UTF-8 encoded)
- Extension attributes → `ce_{extension-name}` headers

### Kafka Structured Content Mode

In Kafka structured mode, the entire CloudEvent is in the message value:

**Message:**
```
Headers:
  content-type: application/cloudevents+json

Value:
  {
    "specversion": "1.0",
    "type": "com.example.user.created",
    "source": "/services/user-service",
    "id": "evt-001",
    "time": "2025-10-09T14:51:00Z",
    "subject": "user-123",
    "datacontenttype": "application/json",
    "data": {
      "user_id": "user-123",
      "email": "user@example.com",
      "timestamp": 1696867860
    }
  }
```

### AMQP Bindings (Future)

AMQP bindings use `cloudEvents_` prefix for message application-properties:
- Binary mode: `data` in body, attributes as `cloudEvents_{attribute}` properties
- Structured mode: entire event in body with `content-type: application/cloudevents+json`

## CloudEvents JSON Format

The mandatory JSON format for CloudEvents uses media type `application/cloudevents+json`:

### JSON Encoding Rules

1. **Required attributes**: Always present in JSON object
2. **Optional attributes**: Included only if non-None
3. **Extension attributes**: Included as top-level keys (not nested)
4. **Data encoding**:
   - JSON data: Direct inclusion in `data` field
   - Binary data: Base64-encoded in `data_base64` field
   - Other types: String representation in `data` field

### Example JSON Representation

```json
{
  "specversion": "1.0",
  "type": "com.example.user.created",
  "source": "/services/user-service",
  "id": "evt-001",
  "time": "2025-10-09T14:51:00Z",
  "subject": "user-123",
  "datacontenttype": "application/json",
  "dataschema": "https://example.com/schemas/user.json",
  "data": {
    "user_id": "user-123",
    "email": "user@example.com",
    "timestamp": 1696867860
  },
  "traceparent": "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"
}
```

## CQRS Integration

### Emitting Events from Directives (Write Side)

Directives (commands) emit domain events wrapped in CloudEvents envelopes:

```rust
// Directive handler emits event after state change
struct CreateUserDirective {
    email: std::string::String,
}

impl hexser::DirectiveHandler for CreateUserDirective {
    type Output = std::string::String;
    
    fn handle(&self, context: &mut Context) -> hexser::HexResult<Self::Output> {
        // 1. Execute domain logic
        let user_id = context.user_repository.create_user(&self.email)?;
        
        // 2. Create domain event
        let event = UserCreated {
            user_id: user_id.clone(),
            email: self.email.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        // 3. Wrap in CloudEvents envelope
        let envelope = hexser::ports::events::CloudEventsEnvelope {
            id: uuid::Uuid::new_v4().to_string(),
            source: std::string::String::from("/services/user-service"),
            specversion: std::string::String::from("1.0"),
            r#type: event.event_type().to_string(),
            subject: std::option::Option::Some(event.aggregate_id()),
            time: std::option::Option::Some(format_rfc3339(std::time::SystemTime::now())),
            data: std::option::Option::Some(event),
            datacontenttype: std::option::Option::Some(std::string::String::from("application/json")),
            dataschema: std::option::Option::None,
            extensions: std::collections::HashMap::new(),
        };
        
        // 4. Publish event
        context.event_publisher.publish(&envelope)?;
        
        std::result::Result::Ok(user_id)
    }
}
```

### Consuming Events for Queries (Read Side)

Query models subscribe to events and update projections:

```rust
// Query model subscriber updates read model
struct UserQueryProjection {
    users: std::collections::HashMap<std::string::String, UserView>,
}

impl UserQueryProjection {
    fn handle_user_created(
        &mut self,
        envelope: hexser::ports::events::CloudEventsEnvelope<UserCreated>,
    ) -> hexser::HexResult<()> {
        if let std::option::Option::Some(event) = envelope.data {
            let view = UserView {
                id: event.user_id.clone(),
                email: event.email.clone(),
                created_at: envelope.time.unwrap_or_default(),
            };
            self.users.insert(event.user_id, view);
        }
        std::result::Result::Ok(())
    }
}

// Subscribe to events
let mut subscriber = InMemoryEventBus::new();
subscriber.subscribe(
    "user.events",
    std::boxed::Box::new(|envelope| {
        projection.handle_user_created(envelope)
    }),
)?;
```

## Security and Reliability

### Idempotency

Use the `id` attribute for idempotency:
- Generate deterministic IDs: `{source}:{aggregate_id}:{sequence}`
- Consumer tracks processed event IDs to prevent duplicate processing
- Store ID + checksum for verification

### Retry Policy

Recommended retry strategy:
1. Exponential backoff: 1s, 2s, 4s, 8s, 16s
2. Maximum retry count: 5 attempts
3. Dead letter queue after max retries
4. Use `id` for deduplication across retries

### Message Size Limits

- Recommended max event size: 1MB
- For larger payloads:
  - Store data externally (S3, blob storage)
  - Include reference URI in `dataschema` or extension attribute
  - Set `data` to metadata/summary only

### Signing and Encryption

CloudEvents supports signing and encryption via extensions:
- **Signing**: Use `signature` extension with JWS format
- **Encryption**: Use structured mode with encrypted `data` field
- Implement at codec layer or transport adapter level

### Multi-Tenant Isolation

Use `source` URI for tenant isolation:
- Format: `/tenants/{tenant-id}/{service}`
- Example: `/tenants/acme/user-service`
- Router can enforce tenant-specific routing rules

## Reference Implementations

### InMemoryEventBus

Simple in-memory event bus for testing:
- Implements `EventPublisher<T>` and `EventSubscriber<T>`
- Synchronous delivery (no concurrency)
- Topic-based routing
- No persistence (events lost on restart)

### JsonEventCodec

CloudEvents JSON format codec:
- Implements `EventCodec<T>` where T: serde::Serialize + serde::Deserialize
- Media type: `application/cloudevents+json`
- Handles all CloudEvents v1.0 attributes
- Supports extension attributes

## Testing Strategy

### Unit Tests

1. **CloudEventsEnvelope validation**:
   - Required attributes must be non-empty
   - `specversion` must be "1.0"
   - `time` must be valid RFC3339 format
   - Extension keys must not start with "ce-"

2. **Codec round-trip**:
   - Encode envelope to JSON
   - Decode JSON back to envelope
   - Assert equality (data preservation)

3. **Attribute mapping**:
   - `type` maps to `DomainEvent::event_type()`
   - `subject` maps to `DomainEvent::aggregate_id()`

### Integration Tests

1. **Publish/Subscribe flow**:
   - Publisher emits CloudEvents envelope
   - Subscriber receives and processes envelope
   - Verify event data integrity

2. **Transport binding simulation**:
   - Encode to HTTP headers/body
   - Decode from HTTP headers/body
   - Verify CloudEvents compliance

### Doc Tests

All port traits and structs include doc tests demonstrating usage without `use` statements.

## Future Enhancements

### Phase 2: HTTP Transport Adapters
- `HttpEventPublisher` (binary and structured modes)
- `HttpEventSubscriber` (webhook receiver)

### Phase 3: Kafka Transport Adapters
- `KafkaEventPublisher` (binary and structured modes)
- `KafkaEventSubscriber` (consumer group support)

### Phase 4: Advanced Features
- Event sourcing integration
- CQRS event store adapter
- CloudEvents discovery API
- Batch processing optimizations
- Distributed tracing integration (OpenTelemetry)

## References

- [CloudEvents v1.0 Specification](https://github.com/cloudevents/spec/blob/v1.0/spec.md)
- [CloudEvents HTTP Protocol Binding](https://github.com/cloudevents/spec/blob/v1.0/http-protocol-binding.md)
- [CloudEvents Kafka Protocol Binding](https://github.com/cloudevents/spec/blob/v1.0/kafka-protocol-binding.md)
- [CloudEvents JSON Event Format](https://github.com/cloudevents/spec/blob/v1.0/json-format.md)
- [RFC3339 Date and Time Format](https://tools.ietf.org/html/rfc3339)
