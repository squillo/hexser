---
task_id: cloudevents_alignment_v0
status: in-progress
---

# Task: Define and plan CloudEvents-aligned, transport-agnostic eventing for hexser

## Plan
- [x] 1. Establish problem statement and goals for CloudEvents alignment (agnostic, generic, SRP).
- [x] 2. Define target deliverables and acceptance criteria.
  - [x] A. TASK_PLAN.md as Source of Record.
  - [ ] B. Design doc (docs/events.md) describing Event model, ports, adapters, and mappings.
  - [ ] C. Minimal reference adapter (in-memory EventBus) and example usage.
  - [ ] D. Tests validating contracts and examples compile/run.
  - [ ] E. CQRS alignment notes: show how Directives (writes) emit events and Queries (reads) project them.
- [ ] 3. Domain model and interfaces (Ports) for events.
  - [ ] A. Define a generic EventEnvelope<T> type reflecting CloudEvents core attributes without dependency lock-in.
  - [ ] B. Define EventPublisher and EventSubscriber ports with back-pressure friendly, pull-based APIs.
  - [ ] C. Define EventCodec port (serialize/deserialize) to support multiple bindings (JSON, Avro, etc.).
  - [ ] D. Define EventRouter trait for topic/subject resolution decoupled from transport.
  - [ ] E. Define DirectiveEventBridge guidance: mapping directive outcomes to EventEnvelope and idempotency keys.
- [ ] 4. Mapping to CloudEvents spec (design only in this iteration).
  - [ ] A. Provide lossless mapping rules between EventEnvelope and CloudEvents v1.0 (core attributes, extensions).
  - [ ] B. Specify transport bindings mapping (HTTP binary/structured) and Kafka header conventions.
  - [ ] C. Document constraints: time format (RFC3339), id uniqueness, datacontenttype, dataschema handling.
- [ ] 5. Security, reliability, and performance considerations.
  - [ ] A. Validate data size limits and streaming data option.
  - [ ] B. Define retry policy and idempotency key guidance.
  - [ ] C. Outline signing/encryption hooks at codec or transport layer.
  - [ ] D. Clarify multi-tenant source/subject conventions.
- [ ] 6. Minimal reference implementation (v0.1, optional in this iteration).
  - [ ] A. InMemoryEventBus adapter implementing EventPublisher/EventSubscriber.
  - [ ] B. JsonEventCodec implementing EventCodec for serde-json.
  - [ ] C. Example wiring in examples/ (emit event from directive; handle with subscriber).
  - [ ] D. Integrate with existing cqrs_pattern example to emit a UserCreated event and project to a read model.
- [ ] 7. Backward/forward compatibility plan.
  - [ ] A. Non-breaking addition under hexser::ports::events module.
  - [ ] B. Deprecation path (if any) for existing ad-hoc events in repo.
  - [ ] C. Versioning strategy (feature flag events).
- [ ] 8. Testing strategy.
  - [ ] A. Unit tests for mapping rules and codec round-trip.
  - [ ] B. Integration test: publish/subscribe flow using InMemoryEventBus.
  - [ ] C. Doc tests in ports to show usage without use statements.
- [ ] 9. Documentation and tutorials.
  - [ ] A. docs/events.md with end-to-end guidance and examples.
  - [ ] B. Update tutorials index to point to events doc and add a short “emit on write” snippet.
  - [ ] C. Update Tutorial 04 (CQRS Basics) README with an event emission snippet aligned with QueryRepository.
- [ ] 10. Execution plan for implementation PRs.
  - [ ] A. PR1: Add ports and docs (no adapters).
  - [ ] B. PR2: Add InMemoryEventBus + JsonEventCodec + tests.
  - [ ] C. PR3: Example integration and tutorial note.
- [ ] 11. Repository analysis for CQRS alignment.
  - [ ] A. Review examples/tutorial_04_application.rs for directive/query flow and identify emission points.
  - [ ] B. Review examples/cqrs_pattern.rs for read/write separation and define projection example.
  - [ ] C. Review hexser_potions flows to propose optional event emission.
- [ ] 12. Next actions for this session.
  - [x] A. Create TASK_PLAN.md with this plan and mark status in-progress.
  - [ ] B. Draft docs/events.md skeleton with headings (design only).
  - [ ] C. Prepare minimal API skeleton types and traits (no behavior) if time allows.
  - [ ] D. Analyze existing CQRS examples and annotate where events would be emitted and consumed.

## Current Step
- Action: Draft design doc and API skeletons; analyze current CQRS examples to align event emission/consumption points.
- Details: Create docs/events.md with CloudEvents mapping rules and outline how directives emit events and queries consume projections; prepare port trait skeletons if time allows.

## Blockers
- None identified. Need preference on first transport target (HTTP or Kafka) and decision on external CloudEvents crate vs. spec-only mapping for phase 1.
