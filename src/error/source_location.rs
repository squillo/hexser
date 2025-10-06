//! Source code location information for error tracking.
//!
//! Provides file, line, and column information for pinpointing
//! the exact origin of an error. Used throughout the error system
//! to enhance traceability and debugging.
//!
//! Revision History
//! - 2025-10-06T02:00:00Z @AI: Fix merge conflict duplicates.
//! - 2025-10-06T00:00:00Z @AI: Initial SourceLocation struct for Phase 1.

/// Source code location for error tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Source file path
    pub file: String,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

impl SourceLocation {
    /// Create new source location
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: file.into(),
            line,
            column,
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_creation() {
        let loc = SourceLocation::new("test.rs", 42, 13);
        assert_eq!(loc.file, "test.rs");
        assert_eq!(loc.line, 42);
        assert_eq!(loc.column, 13);
    }

    #[test]
    fn test_source_location_display() {
        let loc = SourceLocation::new("src/main.rs", 100, 5);
        assert_eq!(format!("{}", loc), "src/main.rs:100:5");
    }
}
