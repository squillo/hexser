# Core Concepts

Hexer embraces Hexagonal Architecture and encodes it in Rust types and traits. Here are the key ideas:

## Layers

- Domain: Pure business logic — entities, value objects, aggregates
- Ports: Traits describing interactions (repositories, services, use cases)
- Adapters: Implementations of ports (DB, HTTP, CLI, in-memory)
- Application: Orchestrates use cases, commands/queries
- Infrastructure: External systems and frameworks

Dependencies point inward. Domain depends on nothing; everything else depends on domain.

## Graph and Registration

Hexer can register components and build an internal graph of your architecture for analysis and visualization. Each registered type provides NodeInfo including its layer, dependencies, and intent. The graph can be validated for common architectural mistakes.

See also: ../graph-overview (future section) and the Architecture chapter.

## Entities and Aggregates

- Entity: Has an identity and invariants. Derive with `#[derive(Entity)]`.
- Aggregate: A consistency boundary that enforces invariants across entities. Derive with `#[derive(HexAggregate)]`.

## Ports and Repositories

- HexPort: Marks a trait as a port
- HexRepository: Special derive for repository-style ports

## Directives and Queries (CQRS)

- HexDirective: Command-style operations that change state
- HexQuery: Read-only operations

These derive macros help organize application-level intent.

## Error Handling

Hex uses a rich error model with layers, sources, and user-facing context. Start with high-level errors in application/ports, and attach sources (like I/O) as needed. See the Errors chapter for details.
