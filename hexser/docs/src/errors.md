# Errors

Hexser provides a rich error model with layered categorization, source locations, and actionable messages.

- Port errors communicate boundary failures (e.g., repository not found, adapter misconfiguration)
- Layer errors indicate violations of architectural boundaries
- Not found, validation, and rich error types provide precise semantics

For in-depth guidance, see the in-repo guide:

- Error Guide: ../../src/error/ERROR_GUIDE.md

Tips:
- Prefer domain-specific errors at the ports/application layer
- Attach lower-level error sources (I/O, network) without leaking them into the domain
- Provide user-facing context via helper methods on error types
