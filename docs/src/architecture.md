# Architecture

This chapter provides a high-level view of how Hexser organizes your application and how the internal analysis graph works.

- Read the full internal architecture guide: ../ARCHITECTURE.md
- See the decision record for choosing Hexagonal Architecture: ../adr/0001-hexagonal-architecture.md

## Layers and Dependencies

- Domain: Pure types and invariants (no dependencies)
- Ports: Traits expressing required capabilities
- Adapters: Implement those ports using concrete tech
- Application: Orchestrates directives/queries
- Infrastructure: Integrations and setup

All dependencies point inward toward the domain.

## Graph Visualization and Validation

Hexser can analyze registered components to build a dependency graph and validate layer boundaries. With the appropriate features enabled, you can export the graph to DOT/HTML for visualization. See the graph module in the crate for details.

## Migrating Between Versions

See the crate-level MIGRATION_GUIDE.md for version-specific changes:

- ../../MIGRATION_GUIDE.md
