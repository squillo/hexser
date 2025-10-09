---
task_id: cloudevents_alignment_v1
status: in-progress
---

# Task: Implement CloudEvents v1.0-compliant DomainEvent model for hexser

## Plan
- [x] 1. Research CloudEvents v1.0 specification using Context7 MCP server.
- [x] 2. Define target deliverables and acceptance criteria.
  - [x] A. TASK_PLAN.md as Source of Record (updated with CloudEvents v1.0 spec).
  - [ ] B. Design doc (docs/events.md) describing DomainEvent model, ports, adapters, and CloudEvents v1.0 mappings.
  - [ ] C. Minimal reference adapter (in-memory EventBus) and example usage.
  - [ ] D. Tests validating CloudEvents v1.0 compliance and examples compile/run.
  - [ ] E. CQRS alignment notes: show how Directives (writes) emit DomainEvents and Queries (reads) project them.
- [ ] 3. Domain model and interfaces (Ports) for CloudEvents v1.0.
  - [ ] A. Define DomainEvent<T> struct with CloudEvents v1.0 REQUIRED attributes: id, source, specversion, type.
  - [ ] B. Define DomainEvent<T> OPTIONAL attributes: datacontenttype, dataschema, subject, time, data.
  - [ ] C. Support extension attributes via generic map or trait for vendor-specific fields.
  - [ ] D. Define EventPublisher and EventSubscriber ports with back-pressure friendly, pull-based APIs.
  - [ ] E. Define EventCodec port (serialize/deserialize) to support CloudEvents event formats (JSON mandatory, Avro/Protobuf optional).
  - [ ] F. Define EventRouter trait for topic/subject resolution decoupled from transport.
  - [ ] G. Define DirectiveEventBridge guidance: mapping directive outcomes to DomainEvent and idempotency keys via id attribute.
- [ ] 4. CloudEvents v1.0 transport bindings (design and implementation).
  - [ ] A. HTTP Binary Mode: data in body, attributes as ce- prefixed headers, datacontenttype maps to Content-Type.
  - [ ] B. HTTP Structured Mode: complete event as JSON with Content-Type: application/cloudevents+json.
  - [ ] C. Kafka Binary Mode: data in message value, attributes as ce_ prefixed UTF-8 headers.
  - [ ] D. Kafka Structured Mode: complete event as JSON in message value.
  - [ ] E. Document AMQP bindings (cloudEvents_ prefix) for future reference.
  - [ ] F. Enforce constraints: time as RFC3339, id uniqueness (source+id), specversion="1.0".
- [ ] 5. CloudEvents JSON format implementation (mandatory).
  - [ ] A. Implement JSON codec with media type application/cloudevents+json.
  - [ ] B. Handle data serialization: JSON data directly, binary as data_base64, other as string.
  - [ ] C. Ensure REQUIRED attributes always present, OPTIONAL attributes conditionally included.
  - [ ] D. Support extension attributes in JSON representation.
- [ ] 6. Security, reliability, and performance considerations.
  - [ ] A. Validate data size limits and document streaming data option for large payloads.
  - [ ] B. Define retry policy and idempotency key guidance using id attribute.
  - [ ] C. Outline signing/encryption hooks at codec or transport layer (CloudEvents security extensions).
  - [ ] D. Clarify multi-tenant source/subject conventions (use source URI for tenant isolation).
- [ ] 7. Minimal reference implementation (v0.1).
  - [ ] A. InMemoryEventBus adapter implementing EventPublisher/EventSubscriber.
  - [ ] B. JsonEventCodec implementing EventCodec for CloudEvents JSON format (application/cloudevents+json).
  - [ ] C. Example wiring in examples/ (emit DomainEvent from directive; handle with subscriber).
  - [ ] D. Integrate with existing cqrs_pattern example to emit UserCreated DomainEvent and project to read model.
- [ ] 8. Backward/forward compatibility plan.
  - [ ] A. Non-breaking addition under hexser::ports::events module.
  - [ ] B. Deprecation path (if any) for existing ad-hoc events in repo.
  - [ ] C. Versioning strategy via feature flag (feature = "events").
  - [ ] D. Ensure forward compatibility by supporting extension attributes.
- [ ] 9. Testing strategy.
  - [ ] A. Unit tests for CloudEvents v1.0 attribute validation and mapping rules.
  - [ ] B. Codec round-trip tests (encode to JSON, decode back to DomainEvent).
  - [ ] C. Integration test: publish/subscribe flow using InMemoryEventBus.
  - [ ] D. Doc tests in ports showing CloudEvents creation without use statements.
  - [ ] E. Validation tests for required attributes and RFC3339 time format.
- [ ] 10. Documentation and tutorials.
  - [ ] A. docs/events.md with CloudEvents v1.0 specification alignment, end-to-end guidance, and examples.
  - [ ] B. Document all REQUIRED and OPTIONAL CloudEvents attributes in DomainEvent struct.
  - [ ] C. Update tutorials index to point to events doc and add "emit on write" snippet.
  - [ ] D. Update Tutorial 04 (CQRS Basics) README with DomainEvent emission example.
  - [ ] E. Include transport binding examples (HTTP binary/structured, Kafka binary/structured).
- [ ] 11. Execution plan for implementation PRs.
  - [ ] A. PR1: Add DomainEvent port definitions and docs/events.md (no adapters).
  - [ ] B. PR2: Add JsonEventCodec + InMemoryEventBus + comprehensive tests.
  - [ ] C. PR3: Add HTTP binary/structured transport adapters.
  - [ ] D. PR4: Add Kafka binary/structured transport adapters.
  - [ ] E. PR5: Example integration in CQRS tutorials and final documentation.
- [ ] 12. Repository analysis for CQRS alignment.
  - [ ] A. Review examples/tutorial_04_application.rs for directive/query flow and identify DomainEvent emission points.
  - [ ] B. Review examples/cqrs_pattern.rs for read/write separation and define projection example.
  - [ ] C. Review hexser_potions flows to propose optional DomainEvent emission.
- [ ] 13. Next actions for this session.
  - [x] A. Update TASK_PLAN.md with CloudEvents v1.0-compliant requirements.
  - [ ] B. Draft docs/events.md skeleton with CloudEvents v1.0 spec alignment.
  - [ ] C. Prepare DomainEvent<T> struct definition with all CloudEvents v1.0 attributes.
  - [ ] D. Analyze existing CQRS examples and annotate where DomainEvents would be emitted and consumed.

## Current Step
- Action: Update TASK_PLAN.md with CloudEvents v1.0 specification requirements; prepare to draft design doc.
- Details: TASK_PLAN now reflects authoritative CloudEvents v1.0 spec from cloudevents/spec. DomainEvent model will have exact required (id, source, specversion, type) and optional (datacontenttype, dataschema, subject, time, data) attributes per spec. Transport bindings updated with precise header conventions (ce- for HTTP, ce_ for Kafka, cloudEvents_ for AMQP). JSON format mandatory with media type application/cloudevents+json.

## Blockers
- None. CloudEvents v1.0 spec is authoritative source. First transport targets: HTTP (both modes) and Kafka (both modes). Phase 1 will be spec-compliant native implementation without external CloudEvents crate dependency to maintain zero-dependency philosophy.
