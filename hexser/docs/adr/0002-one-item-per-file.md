# ADR 0002: One Item Per File Standard

## Status

Accepted

## Date

2025-10-01

## Context

Needed to establish code organization standards that would:
- Maximize modularity and composability
- Make code origins immediately clear
- Support cognitive load reduction
- Enable precise dependency tracking
- Facilitate tooling and analysis

## Decision

Enforce strict "one logical item per file" rule where each source file contains exactly one primary item (struct, enum, function, trait, etc.) with all its implementations.

## Rationale

This extreme modularity provides:
1. **Clarity of origin** - Each item has exactly one canonical location
2. **Reduced cognitive load** - Each file is small and focused
3. **Better git diffs** - Changes isolated to specific items
4. **Easier testing** - In-file tests colocated with implementation
5. **Tooling support** - Simpler static analysis

Aligns with Rust's module system and ownership principles.

## Consequences

### Positive

- Crystal clear where every item is defined
- Tests always colocated with code
- Tiny files reduce cognitive load
- Git history cleaner (one item = one file)
- Easier to reason about dependencies
- Natural single responsibility enforcement

### Negative

- More files to navigate
- Potentially more imports in user code
- IDE project trees larger

### Neutral

- Requires consistent naming convention
- File count proportional to API surface

## Alternatives Considered

### Option 1: Grouped Items

- **Description:** Group related items in single file (e.g., all repository traits in repositories.rs)
- **Pros:** Fewer files, easier browsing
- **Cons:** Harder to find specific items, larger files
- **Why rejected:** Loses clarity of origin, increases cognitive load

### Option 2: Module-Level Organization

- **Description:** Use modules (mod.rs) to contain related items
- **Pros:** Standard Rust practice, familiar
- **Cons:** Items spread across files, harder to locate
- **Why rejected:** Conflicts with "one logical location" goal

### Option 3: Feature-Based Grouping

- **Description:** Group by feature rather than item type
- **Pros:** Business-aligned organization
- **Cons:** Still allows multiple items per file
- **Why rejected:** Doesn't address core modularity goal

## Implementation Notes

Rules:
1. Each .rs file contains one primary item
2. File name matches item name in snake_case
3. Implementations stay with their types
4. Tests use `#[cfg(test)] mod tests` inline
5. Module files (mod.rs) only contain `mod` declarations

Exceptions:
- Module files for organization
- Test modules for in-file tests
- Associated types/constants stay with parent

## References

- hex/.aiassistant/rules/SYS_PROMPT.md
- Rust API Guidelines
- hex/CONTRIBUTING.md

---

**Author:** hex team
**Reviewers:** Community
**Last Updated:** 2025-10-02
