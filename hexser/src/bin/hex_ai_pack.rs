//! CLI tool to export an aggregated AI Agent Pack (architecture + rules + docs).
//!
//! Emits a single JSON document to stdout, suitable for direct consumption by
//! AI assistants and external tools. Requires the `ai` feature.
//!
//! Revision History
//! - 2026-07-20T00:00:00Z @AI: Allow `disallowed_macros` crate-wide in this CLI binary whose sole job is writing JSON to stdout.
//! - 2025-10-06T18:14:00Z @AI: Introduce `hex-ai-pack` binary emitting comprehensive AgentPack JSON.

// This is a CLI binary; writing to stdout is its purpose, so `println!` is intentional.
#![allow(clippy::disallowed_macros)]

fn main() -> hexser::HexResult<()> {
  // Build the current architecture graph from the component registry.
  let graph_arc = hexser::HexGraph::current();

  // Build the aggregated AgentPack and serialize to JSON.
  let pack = hexser::ai::AgentPack::from_graph_with_defaults(std::sync::Arc::as_ref(&graph_arc))?;

  let json = match pack.to_json() {
    std::result::Result::Ok(s) => s,
    std::result::Result::Err(e) => {
      return std::result::Result::Err(hexser::Hexserror::adapter("E_AI_PACK_SERIALIZE", &e));
    }
  };

  std::println!("{}", json);
  std::result::Result::Ok(())
}
