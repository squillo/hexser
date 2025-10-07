//! Aggregated AI Agent Pack for comprehensive project context.
//!
//! Provides a single, stable JSON artifact combining:
//! - Machine-readable architecture (AIContext)
//! - Enforceable guidelines snapshot (rules AIs must follow)
//! - Curated documentation bundle (key docs embedded inline)
//!
//! This allows external agents and tools to consume one payload and
//! immediately operate with full knowledge of architecture, rules,
//! and references. The pack is deterministic where possible and
//! resilient to missing optional docs.
//!
//! Revision History
//! - 2025-10-06T18:14:00Z @AI: Introduce AgentPack aggregator with defaults and JSON serialization.

#[cfg(feature = "ai")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AgentPack {
    /// Schema version for Agent Pack JSON to ensure interop stability
    pub schema_version: String,
    /// Crate name for which this pack was generated
    pub crate_name: String,
    /// Crate version
    pub crate_version: String,
    /// Embedded machine-readable architecture context
    pub ai_context: super::ai_context::AIContext,
    /// Snapshot of rules that agents must observe
    pub guidelines: GuidelinesSnapshot,
    /// Embedded docs content for quick reference by agents
    pub docs: DocBundle,
}

#[cfg(feature = "ai")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GuidelinesSnapshot {
    /// Safety mandate: no `unsafe` code in this crate
    pub unsafe_forbidden: bool,
    /// Clarity mandate: `use` statements are forbidden in generated code
    pub use_statements_forbidden: bool,
    /// Function length soft maximum (lines of code)
    pub function_length_max: u32,
    /// All public items should have docs and colocated tests
    pub testing_mandate: bool,
    /// Error guideline identifiers expected in examples and code
    pub error_guidelines: Vec<String>,
    /// Revision history preamble required for file changes
    pub revision_history_required: bool,
}

#[cfg(feature = "ai")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DocBundle {
    /// Embedded documentation entries
    pub entries: Vec<DocEntry>,
}

#[cfg(feature = "ai")]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DocEntry {
    /// Absolute file path on the generating machine (for traceability)
    pub path: String,
    /// Best-effort title derived from file name or first non-empty line
    pub title: String,
    /// Full file content (UTF-8). May be truncated by caller in the future.
    pub content: String,
    /// Byte length of the content for quick size checks
    pub bytes: usize,
}

#[cfg(feature = "ai")]
impl AgentPack {
    /// Build an AgentPack from the current graph with default doc sources.
    ///
    /// This method:
    /// - Builds AIContext via ContextBuilder
    /// - Loads key docs if present (silently skips missing files)
    /// - Attaches a guidelines snapshot aligned with this crate
    pub fn from_graph_with_defaults(
        graph: &crate::graph::hex_graph::HexGraph,
    ) -> crate::result::hex_result::HexResult<Self> {
        let context = super::context_builder::ContextBuilder::new(graph).build()?;

        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let default_paths = vec![
            manifest_dir.join("README.md"),
            manifest_dir.join("src").join("error").join("ERROR_GUIDE.md"),
            manifest_dir.join("..").join(".junie").join("guidelines.md"),
            manifest_dir
                .join("..")
                .join(".aiassistant")
                .join("rules")
                .join("SYS_PROMPT.md"),
        ];

        let docs = Self::load_docs(default_paths);
        let guidelines = Self::default_guidelines();

        let pack = AgentPack {
            schema_version: String::from("1.0.0"),
            crate_name: String::from(env!("CARGO_PKG_NAME")),
            crate_version: String::from(env!("CARGO_PKG_VERSION")),
            ai_context: context,
            guidelines,
            docs,
        };
        Result::Ok(pack)
    }

    /// Serialize this AgentPack to JSON.
    /// Returns Ok(String) or Err(String) with an explanatory message.
    pub fn to_json(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            std::result::Result::Ok(s) => std::result::Result::Ok(s),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Serialization error: {}",
                e
            )),
        }
    }

    fn default_guidelines() -> GuidelinesSnapshot {
        GuidelinesSnapshot {
            unsafe_forbidden: true,
            use_statements_forbidden: true,
            function_length_max: 50,
            testing_mandate: true,
            error_guidelines: vec![
                String::from("C-QUESTION-MARK"),
                String::from("C-GOOD-ERR"),
                String::from("C-CTOR"),
                String::from("C-STRUCT-PRIVATE"),
            ],
            revision_history_required: true,
        }
    }

    fn load_docs(paths: Vec<std::path::PathBuf>) -> DocBundle {
        let mut entries: Vec<DocEntry> = Vec::new();
        for p in paths {
            if p.exists() {
                let content_result = std::fs::read_to_string(&p);
                match content_result {
                    std::result::Result::Ok(content) => {
                        let title = Self::derive_title(&content, &p);
                        let entry = DocEntry {
                            path: p.to_string_lossy().to_string(),
                            title,
                            bytes: content.as_bytes().len(),
                            content,
                        };
                        entries.push(entry);
                    }
                    std::result::Result::Err(_e) => {
                        // Silently skip unreadable files to keep this resilient
                    }
                }
            }
        }
        DocBundle { entries }
    }

    fn derive_title(content: &str, path: &std::path::Path) -> String {
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return String::from(trimmed);
            }
        }
        match path.file_name() {
            Some(os) => os.to_string_lossy().to_string(),
            None => String::from("document"),
        }
    }
}

#[cfg(all(test, feature = "ai"))]
mod tests_agent_pack {
    #[test]
    fn test_build_and_serialize_agent_pack() {
        let graph = crate::graph::builder::GraphBuilder::new().build();
        let pack = super::AgentPack::from_graph_with_defaults(&graph).unwrap();
        let json = pack.to_json().unwrap();
        assert!(json.contains("\"schema_version\""));
        assert!(json.contains("\"crate_name\""));
        assert!(json.contains("\"ai_context\""));
    }
}
