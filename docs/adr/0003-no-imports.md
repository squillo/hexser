# ADR 0003: No Imports, Fully Qualified Paths

## Status

Accepted

## Date

2025-10-01

## Context

Needed to establish import standards that would:
- Make origins of all identifiers immediately visible
- Eliminate ambiguity about symbol sources
- Support "grep-ability" and tooling
- Reduce cognitive load in understanding dependencies

## Decision

Forbid `use` statements and require fully qualified paths for all non-prelude items.

## Rationale

Explicit fully qualified paths provide:
1. **Immediate clarity** - No need to scroll to top for imports
2. **No ambiguity** - Always clear where identifiers come from
3. **Better tooling** - Static analysis simpler
4. **Grep-friendly** - Can search for `crate::module::Item` directly
5. **Reduced cognitive load** - No mental mapping from alias to source

Exception for std prelude items (Vec, String, Option, Result) as these are universally known.

## Consequences

### Positive

- Zero ambiguity about identifier origins
- No need to maintain import lists
- Better code search and navigation
- Simpler refactoring (no import updates needed)
- Self-documenting code

### Negative

- More verbose code
- Longer lines in some cases
- Less familiar to Rust developers

### Neutral

- Different from standard Rust conventions
- Type inference reduces verbosity impact

## Alternatives Considered

### Option 1: Standard Rust Imports

- **Description:** Use `use` statements as idiomatic Rust does
- **Pros:** Familiar, concise, standard practice
- **Cons:** Loses clarity of origin, requires import maintenance
- **Why rejected:** Conflicts with explicit origin goal

### Option 2: Grouped Imports Only

- **Description:** Allow imports but group by crate
- **Pros:** Compromise between verbosity and clarity
- **Cons:** Still requires scrolling to see origins
- **Why rejected:** Half-measure doesn't achieve goal

### Option 3: Prelude Pattern

- **Description:** Create crate prelude with common items
- **Pros:** Reduces verbosity for common items
- **Cons:** Hidden dependencies, less explicit
- **Why rejected:** We do provide prelude, but as opt-in

## Implementation Notes

Rules:
1. No `use` statements except in tests (`super::`)
2. Write fully qualified paths: `std::collections::HashMap`
3. Prelude items don't need qualification: `Vec`, `String`
4. Use type aliases when paths become unwieldy

Exceptions:
- Test modules can use `super::` for item under test
- Procedural macros may need paths for codegen

Benefits compound with one-item-per-file: each file is short enough that verbosity matters less.

## References

- hex/.aiassistant/rules/SYS_PROMPT.md
- hex/src/prelude.rs (opt-in convenience)
- Rust API Guidelines (divergence documented)

---

**Author:** hex team
**Reviewers:** Community
**Last Updated:** 2025-10-02
