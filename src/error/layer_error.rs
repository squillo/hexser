//! Generic layer error implementation.
//!
//! Provides LayerError<L> generic struct that implements RichError trait.
//! Uses phantom marker types to distinguish between different architectural layers
//! while sharing implementation. Eliminates code duplication across layer-specific errors.
//!
//! Revision History
//! - 2025-10-06T01:00:00Z @AI: Initial LayerError generic for Phase 1 refactor.

/// Layer marker types for type-safe layer distinction
pub mod layer_markers {
    /// Domain layer marker
    #[derive(Debug)]
    pub struct DomainLayer;

    /// Port layer marker
    #[derive(Debug)]
    pub struct PortLayer;

    /// Adapter layer marker
    #[derive(Debug)]
    pub struct AdapterLayer;
}

/// Generic layer error with rich context and error chaining
#[derive(Debug)]
pub struct LayerError<L> {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Actionable next steps for resolution
    pub next_steps: Vec<String>,
    /// Concrete suggestions for fixing the error
    pub suggestions: Vec<String>,
    /// Optional source code location
    pub location: Option<crate::error::source_location::SourceLocation>,
    /// Optional link to documentation
    pub more_info_url: Option<String>,
    /// Underlying error cause
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
    /// Layer marker (zero-sized)
    pub layer: std::marker::PhantomData<L>,
}

impl<L> LayerError<L> {
    /// Create new layer error with code and message
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            next_steps: Vec::new(),
            suggestions: Vec::new(),
            location: None,
            more_info_url: None,
            source: None,
            layer: std::marker::PhantomData,
        }
    }
}

impl<L: std::fmt::Debug> crate::error::rich_error::RichError for LayerError<L> {
    fn code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn next_steps(&self) -> &[String] {
        &self.next_steps
    }

    fn suggestions(&self) -> &[String] {
        &self.suggestions
    }

    fn location(&self) -> Option<&crate::error::source_location::SourceLocation> {
        self.location.as_ref()
    }

    fn more_info_url(&self) -> Option<&str> {
        self.more_info_url.as_deref()
    }

    fn with_next_step(mut self, step: impl Into<String>) -> Self {
        self.next_steps.push(step.into());
        self
    }

    fn with_next_steps(mut self, steps: &[&str]) -> Self {
        self.next_steps.extend(steps.iter().map(|s| String::from(*s)));
        self
    }

    fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    fn with_suggestions(mut self, suggestions: &[&str]) -> Self {
        self.suggestions.extend(suggestions.iter().map(|s| String::from(*s)));
        self
    }

    fn with_location(mut self, location: crate::error::source_location::SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    fn with_more_info(mut self, url: impl Into<String>) -> Self {
        self.more_info_url = Some(url.into());
        self
    }

    fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl<L: std::fmt::Debug> std::fmt::Display for LayerError<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error [{}]: {}", self.code, self.message)?;

        if !self.next_steps.is_empty() {
            write!(f, "\nNext Steps:")?;
            if self.next_steps.len() == 1 {
                write!(f, " {}", self.next_steps[0])?;
            } else {
                for step in &self.next_steps {
                    write!(f, "\n  - {}", step)?;
                }
            }
        }

        if !self.suggestions.is_empty() {
            write!(f, "\nSuggestions:")?;
            if self.suggestions.len() == 1 {
                write!(f, " {}", self.suggestions[0])?;
            } else {
                for suggestion in &self.suggestions {
                    write!(f, "\n  - {}", suggestion)?;
                }
            }
        }

        if let Some(ref url) = self.more_info_url {
            write!(f, "\nMore: {}", url)?;
        }

        if let Some(ref location) = self.location {
            write!(f, "\nSource: {}", location)?;
        }

        Ok(())
    }
}

impl<L: std::fmt::Debug> std::error::Error for LayerError<L> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

#[cfg(test)]
mod tests {
  use std::error::Error;
  use crate::error::RichError;
  use super::*;
    use super::layer_markers::*;

    #[test]
    fn test_domain_error_creation() {
        let err = LayerError::<DomainLayer>::new("E_HEX_001", "Test error");
        assert_eq!(err.code, "E_HEX_001");
        assert_eq!(err.message, "Test error");
    }

    #[test]
    fn test_port_error_builder() {
        let err = LayerError::<PortLayer>::new("E_HEX_100", "Port failure")
            .with_next_step("Check connection")
            .with_suggestion("Retry operation");

        assert_eq!(err.next_steps.len(), 1);
        assert_eq!(err.suggestions.len(), 1);
    }

    #[test]
    fn test_adapter_error_with_source() {
        let inner = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let err = LayerError::<AdapterLayer>::new("E_HEX_200", "Adapter failed")
            .with_source(inner);

        assert!(err.source().is_some());
    }

    #[test]
    fn test_error_display() {
        let err = LayerError::<DomainLayer>::new("E_HEX_001", "Test")
            .with_next_step("Fix it");

        let display = format!("{}", err);
        assert!(display.contains("E_HEX_001"));
        assert!(display.contains("Next Steps"));
    }
}
