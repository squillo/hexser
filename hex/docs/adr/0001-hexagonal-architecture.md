# ADR 0001: Hexagonal Architecture Pattern Choice

## Status

Accepted

## Date

2025-10-01

## Context

Needed to choose an architectural pattern for the hex crate that would:
- Enforce separation of concerns
- Enable testability without infrastructure
- Support multiple adapters for same port
- Make dependencies explicit
- Align with Rust's type system strengths

## Decision

Implement hexagonal architecture (also known as Ports and Adapters pattern) as the foundational pattern for the hex crate, and use the crate itself as a demonstration of the pattern.

## Rationale

Hexagonal architecture provides clear boundaries between:
1. **Domain** - Pure business logic with zero dependencies
2. **Ports** - Interfaces defining what the application needs
3. **Adapters** - Concrete implementations using specific technologies
4. **Application** - Orchestration between domain and ports
5. **Infrastructure** - Technology-specific setup

This pattern excels in Rust because:
- Traits naturally model ports
- Structs naturally model adapters
- Type system enforces layer boundaries
- Zero-cost abstractions make it performant

## Consequences

### Positive

- Clear separation enables testing without infrastructure
- Easy to swap implementations (e.g., mock vs production databases)
- Dependencies point inward, keeping domain pure
- Self-documenting architecture through type relationships
- Extensible without modifying existing code

### Negative

- More initial structure required than simpler patterns
- Learning curve for developers unfamiliar with pattern
- Requires discipline to maintain boundaries

### Neutral

- More files and types than monolithic approach
- Verbosity traded for clarity

## Alternatives Considered

### Option 1: Layered Architecture

- **Description:** Traditional N-tier architecture with presentation, business, data layers
- **Pros:** Familiar to most developers, simple to understand
- **Cons:** Dependencies flow downward, making domain depend on infrastructure
- **Why rejected:** Inverts dependencies, making testing harder

### Option 2: Clean Architecture

- **Description:** Similar to hexagonal but with more layers (entities, use cases, interface adapters, frameworks)
- **Pros:** Even more separation, very detailed
- **Cons:** More complexity, harder to map to Rust idioms
- **Why rejected:** Overengineered for Rust's capabilities

### Option 3: Onion Architecture

- **Description:** Variant of hexagonal with domain at center, layers surrounding
- **Pros:** Visual clarity, similar benefits to hexagonal
- **Cons:** Essentially same as hexagonal with different visualization
- **Why rejected:** No significant advantage over hexagonal

## Implementation Notes

The hex crate itself follows hexagonal architecture:
- `domain/` contains pure traits (Entity, ValueObject, etc.)
- `ports/` contains interfaces (Repository, UseCase, etc.)
- `adapters/` contains marker traits for implementations
- `application/` contains orchestration (Directive handlers, etc.)
- `graph/` feature follows same pattern internally

See ARCHITECTURE.md for detailed module structure.

## References

- [Hexagonal Architecture by Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- [Domain-Driven Design by Eric Evans](https://domainlanguage.com/ddd/)
- hex/docs/ARCHITECTURE.md
- hex/README.md

---

**Author:** hex team
**Reviewers:** Community
**Last Updated:** 2025-10-02
