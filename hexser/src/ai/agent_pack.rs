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
//! - 2026-07-21T00:00:00Z @AI: PRD-272 CLAIM-D2 (finding L46): add AgentPackBuilder (with_docs/with_guidelines/build) and AgentPack::builder(), reusing from_graph_with_defaults for baseline assembly; label default_guidelines as hexser's own house-style defaults and point consumers at the builder for overrides.
//! - 2026-07-21T00:00:00Z @AI: to_json now returns `HexResult<String>` (was stringly-typed `Result<String,String>` re-wrapped at every call site).
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
      manifest_dir
        .join("src")
        .join("error")
        .join("ERROR_GUIDE.md"),
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
  ///
  /// Returns the JSON on success, or a `Hexserror` on failure — consistent with the rest of the
  /// crate (was a stringly-typed `Result<String, String>` re-wrapped at every call site).
  pub fn to_json(&self) -> crate::result::hex_result::HexResult<String> {
    serde_json::to_string(self).map_err(|e| {
      crate::error::hex_error::Hexserror::adapter(
        crate::error::codes::adapter::MAPPING_FAILURE,
        &format!("Failed to serialize AgentPack to JSON: {e}"),
      )
    })
  }

  /// Returns a builder for assembling an AgentPack with explicit overrides (e.g.
  /// project-specific guidelines or docs) instead of hexser's built-in defaults.
  ///
  /// See `AgentPackBuilder`. The builder reuses `from_graph_with_defaults` internally, so the
  /// AIContext, schema/crate metadata, and any fields left unset still come from the same
  /// baseline assembly used by the non-builder path.
  pub fn builder() -> AgentPackBuilder {
    AgentPackBuilder::new()
  }

  /// Returns hexser's own house-style guidelines snapshot — the defaults used by
  /// `from_graph_with_defaults` and by `AgentPack::builder()` when no override is supplied.
  /// These describe this crate's internal coding standards (no `unsafe`, no `use` statements,
  /// a function-length ceiling, mandatory revision history, etc.) and are not a claim about
  /// any downstream consumer's own conventions. Downstream projects with different rules
  /// should not treat these as authoritative for their own codebase; supply project-specific
  /// guidelines via `AgentPack::builder().with_guidelines(...)` instead.
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
              bytes: content.len(),
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

/// Builder for assembling an `AgentPack` with explicit overrides in place of hexser's
/// built-in defaults.
///
/// Any field left unset falls back to what `AgentPack::from_graph_with_defaults` would have
/// produced: `with_guidelines` lets a downstream consumer replace hexser's own house-style
/// `GuidelinesSnapshot` with their own project's rules, and `with_docs` lets them replace the
/// embedded `DocBundle` with their own curated docs. Construct via `AgentPack::builder()`.
#[cfg(feature = "ai")]
pub struct AgentPackBuilder {
  /// Doc bundle override; `None` keeps the default docs loaded by `from_graph_with_defaults`.
  docs: std::option::Option<DocBundle>,
  /// Guidelines override; `None` keeps hexser's own house-style default guidelines.
  guidelines: std::option::Option<GuidelinesSnapshot>,
}

#[cfg(feature = "ai")]
impl AgentPackBuilder {
  /// Creates a builder with no overrides set. Prefer `AgentPack::builder()`.
  fn new() -> Self {
    AgentPackBuilder {
      docs: std::option::Option::None,
      guidelines: std::option::Option::None,
    }
  }

  /// Overrides the embedded documentation bundle instead of the default doc set that
  /// `from_graph_with_defaults` loads from this crate's own README/guidelines files.
  pub fn with_docs(mut self, docs: DocBundle) -> Self {
    self.docs = std::option::Option::Some(docs);
    self
  }

  /// Overrides the guidelines snapshot instead of hexser's own house-style defaults. Use
  /// this to attach a downstream project's own rules (e.g. a different function-length
  /// ceiling, or a different set of error-guideline identifiers) to the emitted AgentPack.
  pub fn with_guidelines(mut self, guidelines: GuidelinesSnapshot) -> Self {
    self.guidelines = std::option::Option::Some(guidelines);
    self
  }

  /// Assembles the AgentPack for `graph`, applying any overrides supplied via `with_docs`
  /// and `with_guidelines`.
  ///
  /// This calls `AgentPack::from_graph_with_defaults` for the baseline assembly (AIContext,
  /// schema/crate metadata, and the default docs/guidelines) and then replaces only the
  /// fields that were explicitly overridden, so the graph-to-AIContext logic lives in exactly
  /// one place rather than being duplicated here.
  pub fn build(
    self,
    graph: &crate::graph::hex_graph::HexGraph,
  ) -> crate::result::hex_result::HexResult<AgentPack> {
    let mut pack = AgentPack::from_graph_with_defaults(graph)?;
    if let std::option::Option::Some(docs) = self.docs {
      pack.docs = docs;
    }
    if let std::option::Option::Some(guidelines) = self.guidelines {
      pack.guidelines = guidelines;
    }
    Result::Ok(pack)
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

  /// why: downstream consumers of hexser need a way to attach their own project's
  /// guidelines to an AgentPack instead of silently inheriting hexser's own house-style
  /// defaults (see default_guidelines' doc comment); this proves the builder override
  /// actually reaches the final pack rather than being discarded in favor of the defaults
  /// that `from_graph_with_defaults` would otherwise apply.
  #[test]
  fn test_builder_with_guidelines_overrides_hexser_defaults() {
    let graph = crate::graph::builder::GraphBuilder::new().build();
    let custom_guidelines = super::GuidelinesSnapshot {
      unsafe_forbidden: false,
      use_statements_forbidden: false,
      function_length_max: 999,
      testing_mandate: false,
      error_guidelines: std::vec![std::string::String::from("CUSTOM-RULE")],
      revision_history_required: false,
    };

    let pack = super::AgentPack::builder()
      .with_guidelines(custom_guidelines)
      .build(&graph)
      .unwrap();

    // Overridden guidelines must win over hexser's own house-style defaults.
    std::assert_eq!(pack.guidelines.function_length_max, 999);
    std::assert_eq!(
      pack.guidelines.error_guidelines,
      std::vec![std::string::String::from("CUSTOM-RULE")]
    );
    std::assert!(!pack.guidelines.unsafe_forbidden);
    std::assert!(!pack.guidelines.use_statements_forbidden);
    std::assert!(!pack.guidelines.testing_mandate);
    std::assert!(!pack.guidelines.revision_history_required);

    // Fields not overridden still come from from_graph_with_defaults' baseline assembly.
    std::assert_eq!(pack.crate_name, String::from(env!("CARGO_PKG_NAME")));
  }
}
