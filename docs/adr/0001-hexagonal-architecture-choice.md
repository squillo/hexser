# ADR 0001: Hexagonal Architecture Choice

## Status

Accepted

## Context

Need an architecture pattern that:
- Enforces clear separation of concerns
- Makes business logic independent of external dependencies
- Enables easy testing through mocking
- Supports extensibility without modifying core domain
- Provides clear boundaries between layers

Traditional layered architecture often leads to tight coupling between presentation, business logic, and data access layers. This makes testing difficult and evolution costly.

## Decision

Implement pure hexagonal architecture (also known as Ports and Adapters) with strict layer boundaries:

1. **Domain Layer**: Pure business logic with zero external dependencies
2. **Ports Layer**: Interfaces defining what the application needs
3. **Adapters Layer**: Concrete implementations using specific technologies
4. **Application Layer**: Orchestration of domain logic and port interactions

Dependencies point inward: Domain ← Ports ← Adapters. Domain layer has no knowledge of infrastructure concerns.

## Consequences

### Positive

- Clear separation between business logic and infrastructure
- Domain layer is pure and testable without mocks
- Easy to mock ports for testing application layer
- Extensible: add new adapters without changing domain
- Technology agnostic: swap implementations easily
- Forces thinking about domain model first
- Makes architectural intent explicit in code structure

### Negative

- Requires discipline to maintain boundaries
- More initial setup and boilerplate
- Learning curve for developers unfamiliar with pattern
- Can feel over-engineered for simple applications
- More interfaces and indirection

### Neutral

- Changes development workflow to domain-first approach
- Requires explicit mapping between layers
- More files and modules in codebase

## Alternatives Considered

1. **Layered Architecture**
   - Traditional N-tier approach
   - Rejected: Often leads to tight coupling and testing difficulties

2. **Clean Architecture**
   - Similar to hexagonal but with more layers
   - Rejected: Added complexity without clear benefit for this use case

3. **Onion Architecture**
   - Similar to hexagonal with emphasis on dependency inversion
   - Rejected: Hexagonal architecture is simpler and more widely understood

4. **No Explicit Architecture**
   - Let structure emerge organically
   - Rejected: Leads to inconsistent patterns and maintainability issues

## References

- Alistair Cockburn: [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- Robert C. Martin: Clean Architecture
- [docs/ARCHITECTURE.md](../ARCHITECTURE.md)

---

**Date:** 2025-10-02
**Author(s):** hex team
**Reviewers:** N/A
