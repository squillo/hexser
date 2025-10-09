//! CLI tool to export AI context for the current HexGraph.
//!
//! This binary prints a machine-readable JSON representation of the
//! project's architecture to stdout. It requires the `ai` feature.
//! External tools and AI assistants can consume this JSON to propose
//! compliant changes and validate against project constraints.
//!
//! Revision History
//! - 2025-10-06T17:59:00Z @AI: Introduce `hex-ai-export` binary (feature-gated) that prints AIContext as JSON.
//! - 2025-10-06T18:09:00Z @AI: Fix unresolved paths by using `hexser::` crate paths and align return type to HexResult; map JSON errors to Hexserror.

fn main() -> hexser::HexResult<()> {
  // Build the current architecture graph from the component registry.
  let graph_arc = hexser::HexGraph::current();

  // Build the AI context using the ContextBuilder and serialize to JSON.
  let builder = hexser::ai::ContextBuilder::new(std::sync::Arc::as_ref(&graph_arc));
  let context = builder.build()?;
  let json = match context.to_json() {
    std::result::Result::Ok(s) => s,
    std::result::Result::Err(e) => {
      return std::result::Result::Err(hexser::Hexserror::adapter("E_AI_SERIALIZE", &e));
    }
  };

  // Print to stdout for downstream tooling.
  std::println!("{}", json);
  std::result::Result::Ok(())
}
