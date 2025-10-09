//! Format exporter port trait.
//!
//! Interface for exporting visual graphs to different formats.
//!
//! Revision History
//! - 2025-10-02T16:00:00Z @AI: Initial FormatExporter port.

/// Port trait for format exporters
pub trait FormatExporter {
  /// Export visual graph to string
  fn export(
    &self,
    visual_graph: &crate::graph::visualization::domain::visual_graph::VisualGraph,
  ) -> crate::result::hex_result::HexResult<String>;

  /// Get format name
  fn format_name(&self) -> &str;

  /// Get file extension
  fn file_extension(&self) -> &str;
}
