//! CLI tool to export an aggregated AI Agent Pack (architecture + rules + docs).
//!
//! Emits a single JSON document to stdout, suitable for direct consumption by
//! AI assistants and external tools. Requires the `ai` feature.
//!
//! Revision History
//! - 2025-10-06T18:14:00Z @AI: Introduce `hex-ai-pack` binary emitting comprehensive AgentPack JSON.

fn main() -> hexer::HexResult<()> {
    // Build the current architecture graph from the component registry.
    let graph_arc = hexer::HexGraph::current();

    // Build the aggregated AgentPack and serialize to JSON.
    let pack = match hexer::ai::AgentPack::from_graph_with_defaults(std::sync::Arc::as_ref(&graph_arc)) {
        std::result::Result::Ok(p) => p,
        std::result::Result::Err(e) => {
            return std::result::Result::Err(e)
        }
    };

    let json = match pack.to_json() {
        std::result::Result::Ok(s) => s,
        std::result::Result::Err(e) => {
            return std::result::Result::Err(hexer::HexError::adapter("E_AI_PACK_SERIALIZE", &e))
        }
    };

    std::println!("{}", json);
    std::result::Result::Ok(())
}
