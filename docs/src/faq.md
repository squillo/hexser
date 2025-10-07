# FAQ

- What is Hexser?
  - A crate that encodes Hexagonal Architecture patterns with minimal boilerplate and adds graph-based introspection.

- Do I have to use all features?
  - No. You can start by deriving Entity and adding HexPort/HexAdapter as needed. Graph/visualization features are optional.

- Which derive macros exist?
  - HexDomain, HexPort, HexAdapter, HexAggregate, Entity, HexRepository, HexDirective, HexQuery.

- How do I visualize the architecture graph?
  - Enable the appropriate graph features and use the graph API to export DOT/HTML. See the graph module and Architecture chapter for pointers.

- How are errors handled?
  - Use the high-level error types from the crate’s error module. See the Errors chapter and ../../src/error/ERROR_GUIDE.md.

- Where are step-by-step guides?
  - See Tutorials and the example projects in the repository.
